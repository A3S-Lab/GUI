use crate::error::{GuiError, GuiResult};
use crate::geometry::Size;
use crate::layout::LAYOUT_QUANTIZATION;
use crate::native::NativeElement;
use crate::platform_host::{
    PlatformElementId, PlatformHost, PlatformHostCommand, PlatformHostEvent, PlatformHostRevision,
    PlatformHostTransaction, PlatformPresentationStatus, PlatformWindowCommand, PlatformWindowSpec,
};

use super::frame::build_snapshot;
use super::interaction::{SelfDrawnElementInteraction, SelfDrawnInteractionSession};
use super::transaction::{
    next_revision, presentation_request, rollback_after_commit_error,
    rollback_after_staged_surface_error, rollback_staged_surface, validate_ack,
};
use super::{
    PlatformScenePreparation, PlatformScenePresenter, SelfDrawnFrameSnapshot,
    SelfDrawnInputDispatch,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfDrawnFrameCommitStatus {
    Committed,
    Unchanged,
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfDrawnFrameCommit {
    pub status: SelfDrawnFrameCommitStatus,
    pub revision: PlatformHostRevision,
    pub layout_rebuilt: bool,
    pub scene_rebuilt: bool,
    pub presentation_requested: bool,
    pub presentation_status: Option<PlatformPresentationStatus>,
    pub host_commands: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelfDrawnHostEventOutcome {
    Frame(SelfDrawnFrameCommit),
    /// Raw host input routed against the last atomically committed frame.
    Input(SelfDrawnInputDispatch),
    StateChanged,
    Forwarded(PlatformHostEvent),
    Ignored,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelfDrawnRuntimeStats {
    pub layout_builds: u64,
    pub scene_builds: u64,
    pub host_commits: u64,
    pub unchanged_frames: u64,
    pub rejected_frames: u64,
    pub redraws: u64,
    pub surface_recoveries: u64,
    pub input_events: u64,
    pub interaction_ticks: u64,
    pub action_invocations: u64,
    pub reducer_failures: u64,
    pub drop_policy_queries: u64,
    pub drop_policy_failures: u64,
}

/// Shared H1 coordinator for one self-drawn top-level window.
///
/// Candidate Native IR, layout, hit regions, Graphics scene, and accessibility
/// state remain private until the zero-widget host transaction commits. A
/// rejected prepare or commit discards staged pixels and leaves the complete
/// previous snapshot active.
pub struct SelfDrawnWindowRuntime<H, P>
where
    H: PlatformHost,
    P: PlatformScenePresenter<H::PresentationTarget>,
{
    pub(super) presenter: P,
    // Presenter precedes host so automatic field drop releases every owned
    // surface lease before the platform host destroys native windows.
    pub(super) host: H,
    pub(super) window_spec: PlatformWindowSpec,
    pub(super) scale_factor: f64,
    pub(super) committed: Option<SelfDrawnFrameSnapshot>,
    pub(super) interaction: SelfDrawnInteractionSession,
    pub(super) last_presentation_revision: Option<PlatformHostRevision>,
    pub(super) occluded: bool,
    pub(super) pending_redraw: bool,
    pub(super) closed: bool,
    pub(super) shutdown: bool,
    pub(super) last_input_timestamp_micros: Option<u64>,
    pub(super) stats: SelfDrawnRuntimeStats,
}

impl<H, P> std::fmt::Debug for SelfDrawnWindowRuntime<H, P>
where
    H: PlatformHost,
    P: PlatformScenePresenter<H::PresentationTarget>,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SelfDrawnWindowRuntime")
            .field("window", &self.window_spec.id)
            .field("logical_size", &self.window_spec.logical_size)
            .field("scale_factor", &self.scale_factor)
            .field(
                "committed_revision",
                &self
                    .committed
                    .as_ref()
                    .map(SelfDrawnFrameSnapshot::revision),
            )
            .field("occluded", &self.occluded)
            .field("event_sequence", &self.interaction.event_sequence)
            .field("focused_element", &self.interaction.focused)
            .field("pending_redraw", &self.pending_redraw)
            .field("closed", &self.closed)
            .field("shutdown", &self.shutdown)
            .field(
                "last_input_timestamp_micros",
                &self.last_input_timestamp_micros,
            )
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

impl<H, P> SelfDrawnWindowRuntime<H, P>
where
    H: PlatformHost,
    P: PlatformScenePresenter<H::PresentationTarget>,
{
    pub fn new(
        host: H,
        presenter: P,
        window_spec: PlatformWindowSpec,
        scale_factor: f64,
    ) -> GuiResult<Self> {
        let window_spec = canonical_window_spec(window_spec)?;
        let scale_factor = canonical_scale_factor(scale_factor)?;
        Ok(Self {
            presenter,
            host,
            window_spec,
            scale_factor,
            committed: None,
            interaction: SelfDrawnInteractionSession::default(),
            last_presentation_revision: None,
            occluded: false,
            pending_redraw: false,
            closed: false,
            shutdown: false,
            last_input_timestamp_micros: None,
            stats: SelfDrawnRuntimeStats::default(),
        })
    }

    pub fn host(&self) -> &H {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut H {
        &mut self.host
    }

    pub fn presenter(&self) -> &P {
        &self.presenter
    }

    pub fn presenter_mut(&mut self) -> &mut P {
        &mut self.presenter
    }

    pub fn window_spec(&self) -> &PlatformWindowSpec {
        &self.window_spec
    }

    pub fn snapshot(&self) -> Option<&SelfDrawnFrameSnapshot> {
        self.committed.as_ref()
    }

    pub fn stats(&self) -> SelfDrawnRuntimeStats {
        self.stats
    }

    pub fn is_occluded(&self) -> bool {
        self.occluded
    }

    pub fn pending_redraw(&self) -> bool {
        self.pending_redraw
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Stable layout-path identity that currently owns portable focus.
    pub fn focused_element(&self) -> Option<&PlatformElementId> {
        self.interaction.focused.as_ref()
    }

    /// Last committed portable interaction state for a mounted element.
    pub fn element_interaction(
        &self,
        id: &PlatformElementId,
    ) -> Option<&SelfDrawnElementInteraction> {
        self.interaction.element(id)
    }

    /// Last successfully routed input sequence for this window runtime.
    pub fn event_sequence(&self) -> u64 {
        self.interaction.event_sequence
    }

    /// Latest monotonic timestamp received from this host's input clock.
    pub fn last_input_timestamp_micros(&self) -> Option<u64> {
        self.last_input_timestamp_micros
    }

    pub fn render(&mut self, native_root: NativeElement) -> GuiResult<SelfDrawnFrameCommit> {
        self.ensure_running()?;
        if self.closed {
            return Err(GuiError::host("cannot render a closed self-drawn window"));
        }
        if self.committed.as_ref().is_some_and(|committed| {
            committed.native_root() == &native_root
                && committed.logical_size() == self.window_spec.logical_size
                && committed.scale_factor() == self.scale_factor
        }) {
            self.stats.unchanged_frames = self.stats.unchanged_frames.saturating_add(1);
            return Ok(self.unchanged_commit());
        }
        let spec = self.window_spec.clone();
        let scale_factor = self.scale_factor;
        self.rebuild(native_root, spec, scale_factor)
    }

    pub fn shutdown(&mut self) -> GuiResult<()> {
        if self.shutdown {
            return Ok(());
        }
        self.presenter.shutdown()?;
        self.host.shutdown()?;
        self.committed = None;
        self.interaction = SelfDrawnInteractionSession::default();
        self.pending_redraw = false;
        self.last_input_timestamp_micros = None;
        self.shutdown = true;
        Ok(())
    }

    pub(super) fn rebuild(
        &mut self,
        native_root: NativeElement,
        desired_spec: PlatformWindowSpec,
        desired_scale: f64,
    ) -> GuiResult<SelfDrawnFrameCommit> {
        let desired_spec = canonical_window_spec(desired_spec)?;
        let desired_scale = canonical_scale_factor(desired_scale)?;
        let revision = next_revision(self.committed.as_ref())?;
        self.stats.layout_builds = self.stats.layout_builds.saturating_add(1);
        self.stats.scene_builds = self.stats.scene_builds.saturating_add(1);
        let candidate = match build_snapshot(
            revision,
            desired_spec.id,
            native_root,
            desired_spec.logical_size,
            desired_scale,
            self.committed.as_ref(),
        ) {
            Ok(candidate) => candidate,
            Err(error) => {
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return Err(error);
            }
        };
        let first_frame = self.committed.is_none();
        let visual_changed = self
            .committed
            .as_ref()
            .is_none_or(|previous| previous.scene_fingerprint() != candidate.scene_fingerprint());
        let presentation_requested = visual_changed && !self.occluded;
        let mut commands = Vec::with_capacity(3);
        if first_frame {
            commands.push(PlatformHostCommand::Window {
                command: PlatformWindowCommand::Open {
                    spec: desired_spec.clone(),
                },
            });
        }
        commands.push(PlatformHostCommand::Accessibility {
            snapshot: Box::new(candidate.accessibility().clone()),
        });
        if presentation_requested {
            commands.push(PlatformHostCommand::Present {
                request: presentation_request(&candidate),
            });
        }
        let command_count = commands.len();
        let transaction = PlatformHostTransaction { revision, commands };
        if let Err(error) = self.host.prepare(transaction) {
            self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
            return Err(error);
        }
        let prepared = if presentation_requested {
            let target = match self.host.presentation_target(candidate.window()) {
                Ok(target) => target,
                Err(error) => {
                    let rollback = self.host.rollback();
                    self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                    return rollback_after_commit_error(error, rollback);
                }
            };
            match self.presenter.prepare(target, candidate.render_frame()) {
                Ok(PlatformScenePreparation::Ready(prepared)) => Some(prepared),
                Ok(PlatformScenePreparation::Deferred(deferral)) => {
                    let status = PlatformPresentationStatus::from(deferral);
                    let surface_cleanup =
                        if first_frame || status == PlatformPresentationStatus::SurfaceLost {
                            Some(self.presenter.surface_lost(candidate.window()))
                        } else {
                            None
                        };
                    let rollback = self.host.rollback();
                    rollback_staged_surface(surface_cleanup, rollback)?;
                    if status == PlatformPresentationStatus::SurfaceLost {
                        self.stats.surface_recoveries =
                            self.stats.surface_recoveries.saturating_add(1);
                    }
                    self.pending_redraw = true;
                    let retained_revision = self
                        .committed
                        .as_ref()
                        .map(SelfDrawnFrameSnapshot::revision)
                        .unwrap_or(revision);
                    return Ok(SelfDrawnFrameCommit {
                        status: SelfDrawnFrameCommitStatus::Deferred,
                        revision: retained_revision,
                        layout_rebuilt: true,
                        scene_rebuilt: true,
                        presentation_requested: true,
                        presentation_status: Some(status),
                        host_commands: 0,
                    });
                }
                Err(error) => {
                    let surface_cleanup =
                        first_frame.then(|| self.presenter.surface_lost(candidate.window()));
                    let rollback = self.host.rollback();
                    self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                    return rollback_after_staged_surface_error(error, surface_cleanup, rollback);
                }
            }
        } else {
            None
        };
        let ack = match self.host.commit() {
            Ok(ack) => ack,
            Err(error) => {
                if let Some(prepared) = prepared {
                    self.presenter.discard(prepared);
                }
                let surface_cleanup =
                    first_frame.then(|| self.presenter.surface_lost(candidate.window()));
                let rollback = self.host.rollback();
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return rollback_after_staged_surface_error(error, surface_cleanup, rollback);
            }
        };
        validate_ack(
            &ack,
            revision,
            command_count,
            presentation_requested.then_some(candidate.window()),
        )?;
        let presentation_status = if let Some(prepared) = prepared {
            let status = PlatformPresentationStatus::from(self.presenter.publish(prepared));
            self.last_presentation_revision = Some(revision);
            Some(status)
        } else {
            None
        };
        self.window_spec = desired_spec;
        self.scale_factor = desired_scale;
        self.pending_redraw = (visual_changed && self.occluded)
            || matches!(
                presentation_status,
                Some(PlatformPresentationStatus::Dropped | PlatformPresentationStatus::SurfaceLost)
            );
        if presentation_status == Some(PlatformPresentationStatus::SurfaceLost) {
            self.stats.surface_recoveries = self.stats.surface_recoveries.saturating_add(1);
        }
        self.interaction
            .reconcile(candidate.interaction_tree(), revision);
        self.committed = Some(candidate);
        self.stats.host_commits = self.stats.host_commits.saturating_add(1);
        Ok(SelfDrawnFrameCommit {
            status: SelfDrawnFrameCommitStatus::Committed,
            revision,
            layout_rebuilt: true,
            scene_rebuilt: true,
            presentation_requested,
            presentation_status,
            host_commands: command_count,
        })
    }

    fn unchanged_commit(&self) -> SelfDrawnFrameCommit {
        let revision = self
            .committed
            .as_ref()
            .map(SelfDrawnFrameSnapshot::revision)
            .unwrap_or_else(|| PlatformHostRevision::new(1));
        SelfDrawnFrameCommit {
            status: SelfDrawnFrameCommitStatus::Unchanged,
            revision,
            layout_rebuilt: false,
            scene_rebuilt: false,
            presentation_requested: false,
            presentation_status: None,
            host_commands: 0,
        }
    }

    pub(super) fn ensure_running(&self) -> GuiResult<()> {
        if self.shutdown {
            Err(GuiError::host("self-drawn window runtime is shut down"))
        } else {
            Ok(())
        }
    }
}

pub(super) fn canonical_scale_factor(scale_factor: f64) -> GuiResult<f64> {
    let narrowed = scale_factor as f32;
    if !scale_factor.is_finite() || scale_factor <= 0.0 || !narrowed.is_finite() || narrowed <= 0.0
    {
        Err(GuiError::host(
            "self-drawn scale factor must fit in a positive finite 32-bit float",
        ))
    } else {
        Ok(f64::from(narrowed))
    }
}

fn canonical_window_spec(mut spec: PlatformWindowSpec) -> GuiResult<PlatformWindowSpec> {
    spec.validate()?;
    spec.logical_size = canonical_size(spec.logical_size);
    spec.min_size = spec.min_size.map(canonical_size);
    spec.max_size = spec.max_size.map(canonical_size);
    spec.validate()?;
    Ok(spec)
}

fn canonical_size(size: Size) -> Size {
    Size::new(
        canonical_logical_value(size.width),
        canonical_logical_value(size.height),
    )
}

fn canonical_logical_value(value: f64) -> f64 {
    let value = (value / LAYOUT_QUANTIZATION).round() * LAYOUT_QUANTIZATION;
    if value == -0.0 {
        0.0
    } else {
        value
    }
}
