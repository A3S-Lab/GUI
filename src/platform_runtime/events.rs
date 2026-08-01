use crate::error::{GuiError, GuiResult};
use crate::platform_host::{
    PlatformHost, PlatformHostCommand, PlatformHostEvent, PlatformHostTransaction,
    PlatformPresentationAck, PlatformPresentationStatus, PlatformWindowEvent, PlatformWindowId,
};

use super::runtime::{
    canonical_scale_factor, next_revision, presentation_request, rollback_after_commit_error,
    validate_ack, SelfDrawnFrameCommit, SelfDrawnFrameCommitStatus, SelfDrawnHostEventOutcome,
    SelfDrawnWindowRuntime,
};
use super::PlatformScenePresenter;

impl<H, P> SelfDrawnWindowRuntime<H, P>
where
    H: PlatformHost,
    P: PlatformScenePresenter,
{
    pub fn redraw(&mut self) -> GuiResult<SelfDrawnFrameCommit> {
        self.ensure_running()?;
        let Some(previous) = self.committed.as_ref() else {
            return Err(GuiError::host(
                "cannot redraw before a self-drawn frame is committed",
            ));
        };
        if self.occluded {
            self.pending_redraw = true;
            return Ok(SelfDrawnFrameCommit {
                status: SelfDrawnFrameCommitStatus::Deferred,
                revision: previous.revision(),
                layout_rebuilt: false,
                scene_rebuilt: false,
                presentation_requested: false,
                host_commands: 0,
            });
        }
        let revision = next_revision(Some(previous))?;
        let replay = previous.replay(revision);
        let prepared = self.presenter.prepare(replay.render_frame())?;
        let transaction = PlatformHostTransaction {
            revision,
            commands: vec![PlatformHostCommand::Present {
                request: presentation_request(&replay),
            }],
        };
        if let Err(error) = self.host.prepare(transaction) {
            self.presenter.discard(prepared);
            self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
            return Err(error);
        }
        let ack = match self.host.commit() {
            Ok(ack) => ack,
            Err(error) => {
                let rollback = self.host.rollback();
                self.presenter.discard(prepared);
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return rollback_after_commit_error(error, rollback);
            }
        };
        validate_ack(&ack, revision, 1)?;
        self.presenter.publish(prepared);
        self.committed = Some(replay);
        self.last_presentation_revision = Some(revision);
        self.pending_redraw = false;
        self.stats.host_commits = self.stats.host_commits.saturating_add(1);
        self.stats.redraws = self.stats.redraws.saturating_add(1);
        Ok(SelfDrawnFrameCommit {
            status: SelfDrawnFrameCommitStatus::Committed,
            revision,
            layout_rebuilt: false,
            scene_rebuilt: false,
            presentation_requested: true,
            host_commands: 1,
        })
    }

    pub fn handle_event(
        &mut self,
        event: PlatformHostEvent,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        self.ensure_running()?;
        event.validate()?;
        match event {
            PlatformHostEvent::Window { event } => self.handle_window_event(event),
            PlatformHostEvent::Presentation { ack } => self.handle_presentation_ack(ack),
            event @ (PlatformHostEvent::Input { .. }
            | PlatformHostEvent::TextInput { .. }
            | PlatformHostEvent::Accessibility { .. }
            | PlatformHostEvent::System { .. }) => Ok(SelfDrawnHostEventOutcome::Forwarded(event)),
        }
    }

    pub fn poll_event(&mut self) -> GuiResult<Option<SelfDrawnHostEventOutcome>> {
        let Some(event) = self.host.poll_event()? else {
            return Ok(None);
        };
        self.handle_event(event).map(Some)
    }

    fn handle_window_event(
        &mut self,
        event: PlatformWindowEvent,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        if window_event_id(&event) != self.window_spec.id {
            return Ok(SelfDrawnHostEventOutcome::Forwarded(
                PlatformHostEvent::Window { event },
            ));
        }
        match event {
            PlatformWindowEvent::Resized { logical_size, .. } => {
                let Some(root) = self
                    .committed
                    .as_ref()
                    .map(|snapshot| snapshot.native_root().clone())
                else {
                    self.window_spec.logical_size = logical_size;
                    return Ok(SelfDrawnHostEventOutcome::StateChanged);
                };
                let mut desired = self.window_spec.clone();
                desired.logical_size = logical_size;
                let commit = self.rebuild(root, desired, self.scale_factor)?;
                Ok(SelfDrawnHostEventOutcome::Frame(commit))
            }
            PlatformWindowEvent::ScaleChanged { scale_factor, .. } => {
                let Some(root) = self
                    .committed
                    .as_ref()
                    .map(|snapshot| snapshot.native_root().clone())
                else {
                    self.scale_factor = canonical_scale_factor(scale_factor)?;
                    return Ok(SelfDrawnHostEventOutcome::StateChanged);
                };
                let commit = self.rebuild(root, self.window_spec.clone(), scale_factor)?;
                Ok(SelfDrawnHostEventOutcome::Frame(commit))
            }
            PlatformWindowEvent::OcclusionChanged { occluded, .. } => {
                let changed = self.occluded != occluded;
                self.occluded = occluded;
                if !occluded && self.pending_redraw {
                    return self.redraw().map(SelfDrawnHostEventOutcome::Frame);
                }
                Ok(if changed {
                    SelfDrawnHostEventOutcome::StateChanged
                } else {
                    SelfDrawnHostEventOutcome::Ignored
                })
            }
            PlatformWindowEvent::RedrawRequested { .. } => {
                self.redraw().map(SelfDrawnHostEventOutcome::Frame)
            }
            PlatformWindowEvent::Closed { .. } => {
                self.closed = true;
                Ok(SelfDrawnHostEventOutcome::StateChanged)
            }
            event @ (PlatformWindowEvent::FocusChanged { .. }
            | PlatformWindowEvent::CloseRequested { .. }) => Ok(
                SelfDrawnHostEventOutcome::Forwarded(PlatformHostEvent::Window { event }),
            ),
        }
    }

    fn handle_presentation_ack(
        &mut self,
        ack: PlatformPresentationAck,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        if ack.window != self.window_spec.id
            || self.last_presentation_revision != Some(ack.revision)
        {
            return Ok(SelfDrawnHostEventOutcome::Ignored);
        }
        match ack.status {
            PlatformPresentationStatus::SurfaceLost => {
                self.presenter.surface_lost(ack.window)?;
                self.stats.surface_recoveries = self.stats.surface_recoveries.saturating_add(1);
                self.pending_redraw = true;
                if self.occluded {
                    Ok(SelfDrawnHostEventOutcome::StateChanged)
                } else {
                    self.redraw().map(SelfDrawnHostEventOutcome::Frame)
                }
            }
            PlatformPresentationStatus::Dropped => {
                self.pending_redraw = true;
                if self.occluded {
                    Ok(SelfDrawnHostEventOutcome::StateChanged)
                } else {
                    self.redraw().map(SelfDrawnHostEventOutcome::Frame)
                }
            }
            PlatformPresentationStatus::Queued | PlatformPresentationStatus::Presented => {
                Ok(SelfDrawnHostEventOutcome::StateChanged)
            }
        }
    }
}

fn window_event_id(event: &PlatformWindowEvent) -> PlatformWindowId {
    match event {
        PlatformWindowEvent::Resized { window, .. }
        | PlatformWindowEvent::ScaleChanged { window, .. }
        | PlatformWindowEvent::FocusChanged { window, .. }
        | PlatformWindowEvent::OcclusionChanged { window, .. }
        | PlatformWindowEvent::RedrawRequested { window }
        | PlatformWindowEvent::CloseRequested { window }
        | PlatformWindowEvent::Closed { window } => *window,
    }
}
