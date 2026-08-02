use std::sync::Arc;

use crate::error::{GuiError, GuiResult};
use crate::platform_host::{
    PlatformHost, PlatformHostCommand, PlatformHostEvent, PlatformHostTransaction,
    PlatformInputEvent, PlatformPresentationAck, PlatformPresentationStatus, PlatformWindowEvent,
    PlatformWindowId,
};

use super::drop_policy::SelfDrawnDropPolicyResolver;
use super::runtime::{
    canonical_scale_factor, SelfDrawnFrameCommit, SelfDrawnFrameCommitStatus,
    SelfDrawnHostEventOutcome, SelfDrawnWindowRuntime,
};
use super::transaction::{
    next_revision, presentation_request, rollback_after_commit_error, rollback_staged_surface,
    validate_ack,
};
use super::{PlatformScenePreparation, PlatformScenePresenter};
use super::{SelfDrawnActionInvocation, SelfDrawnActionPropagation};

impl<H, P> SelfDrawnWindowRuntime<H, P>
where
    H: PlatformHost,
    P: PlatformScenePresenter<H::PresentationTarget>,
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
                presentation_status: None,
                host_commands: 0,
            });
        }
        let revision = next_revision(Some(previous))?;
        let replay = previous.replay(revision);
        let transaction = PlatformHostTransaction {
            revision,
            commands: vec![PlatformHostCommand::Present {
                request: presentation_request(&replay),
            }],
        };
        if let Err(error) = self.host.prepare(transaction) {
            self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
            return Err(error);
        }
        let target = match self.host.presentation_target(replay.window()) {
            Ok(target) => target,
            Err(error) => {
                let rollback = self.host.rollback();
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return rollback_after_commit_error(error, rollback);
            }
        };
        let prepared = match self.presenter.prepare(target, replay.render_frame()) {
            Ok(PlatformScenePreparation::Ready(prepared)) => prepared,
            Ok(PlatformScenePreparation::Deferred(deferral)) => {
                let status = PlatformPresentationStatus::from(deferral);
                let surface_cleanup = (status == PlatformPresentationStatus::SurfaceLost)
                    .then(|| self.presenter.surface_lost(replay.window()));
                let rollback = self.host.rollback();
                rollback_staged_surface(surface_cleanup, rollback)?;
                self.pending_redraw = true;
                return Ok(SelfDrawnFrameCommit {
                    status: SelfDrawnFrameCommitStatus::Deferred,
                    revision: previous.revision(),
                    layout_rebuilt: false,
                    scene_rebuilt: false,
                    presentation_requested: true,
                    presentation_status: Some(status),
                    host_commands: 0,
                });
            }
            Err(error) => {
                let rollback = self.host.rollback();
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return rollback_after_commit_error(error, rollback);
            }
        };
        let ack = match self.host.commit() {
            Ok(ack) => ack,
            Err(error) => {
                self.presenter.discard(prepared);
                let rollback = self.host.rollback();
                self.stats.rejected_frames = self.stats.rejected_frames.saturating_add(1);
                return rollback_after_commit_error(error, rollback);
            }
        };
        validate_ack(&ack, revision, 1, Some(replay.window()))?;
        let presentation_status =
            PlatformPresentationStatus::from(self.presenter.publish(prepared));
        self.committed = Some(replay);
        self.last_presentation_revision = Some(revision);
        self.pending_redraw = matches!(
            presentation_status,
            PlatformPresentationStatus::Dropped | PlatformPresentationStatus::SurfaceLost
        );
        if presentation_status == PlatformPresentationStatus::SurfaceLost {
            self.stats.surface_recoveries = self.stats.surface_recoveries.saturating_add(1);
        }
        self.stats.host_commits = self.stats.host_commits.saturating_add(1);
        self.stats.redraws = self.stats.redraws.saturating_add(1);
        Ok(SelfDrawnFrameCommit {
            status: SelfDrawnFrameCommitStatus::Committed,
            revision,
            layout_rebuilt: false,
            scene_rebuilt: false,
            presentation_requested: true,
            presentation_status: Some(presentation_status),
            host_commands: 1,
        })
    }

    pub fn handle_event(
        &mut self,
        event: PlatformHostEvent,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        self.handle_event_with_optional_drop_policy(event, None)
    }

    /// Routes an event with a synchronous, revision-scoped drop policy
    /// resolver. Missing, stale, malformed, timed-out, or failed answers are
    /// converted to a canceled target without surfacing a partially accepted
    /// drag state.
    pub fn handle_event_with_drop_policy(
        &mut self,
        event: PlatformHostEvent,
        resolver: &mut dyn SelfDrawnDropPolicyResolver,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        self.handle_event_with_optional_drop_policy(event, Some(resolver))
    }

    fn handle_event_with_optional_drop_policy(
        &mut self,
        event: PlatformHostEvent,
        resolver: Option<&mut dyn SelfDrawnDropPolicyResolver>,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        self.ensure_running()?;
        event.validate()?;
        match event {
            PlatformHostEvent::Window { event } => self.handle_window_event(event),
            PlatformHostEvent::Presentation { ack } => self.handle_presentation_ack(ack),
            PlatformHostEvent::Input { event } => self.handle_input_event(event, resolver),
            event @ (PlatformHostEvent::TextInput { .. }
            | PlatformHostEvent::Accessibility { .. }
            | PlatformHostEvent::System { .. }) => Ok(SelfDrawnHostEventOutcome::Forwarded(event)),
        }
    }

    /// Routes an event and synchronously applies every selected action in
    /// target-to-ancestor order. A reducer error restores the staged portable
    /// interaction state and event sequence before returning the error.
    pub fn handle_event_with_reducer<R>(
        &mut self,
        event: PlatformHostEvent,
        mut reducer: R,
    ) -> GuiResult<SelfDrawnHostEventOutcome>
    where
        R: FnMut(&SelfDrawnActionInvocation) -> GuiResult<SelfDrawnActionPropagation>,
    {
        let interaction = self.interaction.clone();
        let stats = self.stats;
        let mut outcome = self.handle_event(event)?;
        let SelfDrawnHostEventOutcome::Input(dispatch) = &mut outcome else {
            return Ok(outcome);
        };
        if let Err(error) = reduce_dispatch(dispatch, &mut reducer) {
            self.interaction = interaction;
            self.stats = stats;
            self.stats.reducer_failures = self.stats.reducer_failures.saturating_add(1);
            return Err(error);
        }
        Ok(outcome)
    }

    /// Combines synchronous drop-policy resolution with transactional action
    /// reduction for a single committed-frame event.
    pub fn handle_event_with_drop_policy_and_reducer<R>(
        &mut self,
        event: PlatformHostEvent,
        resolver: &mut dyn SelfDrawnDropPolicyResolver,
        mut reducer: R,
    ) -> GuiResult<SelfDrawnHostEventOutcome>
    where
        R: FnMut(&SelfDrawnActionInvocation) -> GuiResult<SelfDrawnActionPropagation>,
    {
        let interaction = self.interaction.clone();
        let stats = self.stats;
        let mut outcome = self.handle_event_with_drop_policy(event, resolver)?;
        let SelfDrawnHostEventOutcome::Input(dispatch) = &mut outcome else {
            return Ok(outcome);
        };
        if let Err(error) = reduce_dispatch(dispatch, &mut reducer) {
            self.interaction = interaction;
            self.stats = stats;
            self.stats.reducer_failures = self.stats.reducer_failures.saturating_add(1);
            return Err(error);
        }
        Ok(outcome)
    }

    /// Earliest monotonic host-clock timestamp at which portable interaction
    /// state needs another event-loop callback.
    pub fn next_interaction_deadline_micros(&self) -> Option<u64> {
        self.interaction.next_interaction_deadline_micros()
    }

    /// Advances scheduled portable interaction state without synthesizing an
    /// operating-system input event. Hosts call this at or after the deadline
    /// returned by [`Self::next_interaction_deadline_micros`]. Each call drains
    /// one stable pointer deadline; call again while the next deadline is not
    /// later than the current host timestamp.
    pub fn advance_interaction_time(
        &mut self,
        timestamp_micros: u64,
    ) -> GuiResult<Option<super::SelfDrawnInputDispatch>> {
        self.ensure_running()?;
        let Some(snapshot) = self.committed.as_ref() else {
            return Ok(None);
        };
        let revision = snapshot.revision();
        let tree = Arc::clone(snapshot.interaction_tree());
        let dispatch =
            self.interaction
                .route_interaction_time(timestamp_micros, revision, &tree)?;
        if let Some(dispatch) = &dispatch {
            self.stats.interaction_ticks = self.stats.interaction_ticks.saturating_add(1);
            self.stats.action_invocations = self
                .stats
                .action_invocations
                .saturating_add(dispatch.invocations.len() as u64);
        }
        Ok(dispatch)
    }

    /// Advances a scheduled interaction deadline and applies its ordered
    /// action batch transactionally to the portable interaction session.
    pub fn advance_interaction_time_with_reducer<R>(
        &mut self,
        timestamp_micros: u64,
        mut reducer: R,
    ) -> GuiResult<Option<super::SelfDrawnInputDispatch>>
    where
        R: FnMut(&SelfDrawnActionInvocation) -> GuiResult<SelfDrawnActionPropagation>,
    {
        let interaction = self.interaction.clone();
        let stats = self.stats;
        let Some(mut dispatch) = self.advance_interaction_time(timestamp_micros)? else {
            return Ok(None);
        };
        if let Err(error) = reduce_dispatch(&mut dispatch, &mut reducer) {
            self.interaction = interaction;
            self.stats = stats;
            self.stats.reducer_failures = self.stats.reducer_failures.saturating_add(1);
            return Err(error);
        }
        Ok(Some(dispatch))
    }

    pub fn poll_event(&mut self) -> GuiResult<Option<SelfDrawnHostEventOutcome>> {
        let Some(event) = self.host.poll_event()? else {
            return Ok(None);
        };
        self.handle_event(event).map(Some)
    }

    pub fn poll_event_with_drop_policy(
        &mut self,
        resolver: &mut dyn SelfDrawnDropPolicyResolver,
    ) -> GuiResult<Option<SelfDrawnHostEventOutcome>> {
        let Some(event) = self.host.poll_event()? else {
            return Ok(None);
        };
        self.handle_event_with_drop_policy(event, resolver)
            .map(Some)
    }

    fn handle_input_event(
        &mut self,
        event: PlatformInputEvent,
        resolver: Option<&mut dyn SelfDrawnDropPolicyResolver>,
    ) -> GuiResult<SelfDrawnHostEventOutcome> {
        if input_window(&event) != self.window_spec.id {
            return Ok(SelfDrawnHostEventOutcome::Forwarded(
                PlatformHostEvent::Input { event },
            ));
        }
        let Some(snapshot) = self.committed.as_ref() else {
            return Ok(SelfDrawnHostEventOutcome::Ignored);
        };
        let revision = snapshot.revision();
        let tree = Arc::clone(snapshot.interaction_tree());
        self.last_input_timestamp_micros = Some(input_timestamp_micros(&event));
        let (dispatch, policy_stats) = self
            .interaction
            .route_input(&event, revision, &tree, resolver)?;
        self.stats.input_events = self.stats.input_events.saturating_add(1);
        self.stats.action_invocations = self
            .stats
            .action_invocations
            .saturating_add(dispatch.invocations.len() as u64);
        self.stats.drop_policy_queries = self
            .stats
            .drop_policy_queries
            .saturating_add(policy_stats.queries);
        self.stats.drop_policy_failures = self
            .stats
            .drop_policy_failures
            .saturating_add(policy_stats.failures);
        Ok(SelfDrawnHostEventOutcome::Input(dispatch))
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
            PlatformWindowEvent::CloseRequested { .. } => {
                let Some(snapshot) = self.committed.as_ref() else {
                    return Ok(SelfDrawnHostEventOutcome::Ignored);
                };
                let revision = snapshot.revision();
                let tree = Arc::clone(snapshot.interaction_tree());
                let dispatch = self.interaction.route_window_close(
                    revision,
                    &tree,
                    self.last_input_timestamp_micros.unwrap_or_default(),
                )?;
                self.stats.input_events = self.stats.input_events.saturating_add(1);
                self.stats.action_invocations = self
                    .stats
                    .action_invocations
                    .saturating_add(dispatch.invocations.len() as u64);
                Ok(SelfDrawnHostEventOutcome::Input(dispatch))
            }
            event @ PlatformWindowEvent::FocusChanged { .. } => Ok(
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

fn input_window(event: &PlatformInputEvent) -> PlatformWindowId {
    match event {
        PlatformInputEvent::Pointer { event } => event.window,
        PlatformInputEvent::Key { event } => event.window,
        PlatformInputEvent::Wheel { event } => event.window,
        PlatformInputEvent::ModifiersChanged { window, .. } => *window,
    }
}

fn input_timestamp_micros(event: &PlatformInputEvent) -> u64 {
    match event {
        PlatformInputEvent::Pointer { event } => event.timestamp_micros,
        PlatformInputEvent::Key { event } => event.timestamp_micros,
        PlatformInputEvent::Wheel { event } => event.timestamp_micros,
        PlatformInputEvent::ModifiersChanged {
            timestamp_micros, ..
        } => *timestamp_micros,
    }
}

fn reduce_dispatch<R>(
    dispatch: &mut super::SelfDrawnInputDispatch,
    reducer: &mut R,
) -> GuiResult<()>
where
    R: FnMut(&SelfDrawnActionInvocation) -> GuiResult<SelfDrawnActionPropagation>,
{
    let mut stopped_at = None;
    for invocation in &dispatch.invocations {
        if stopped_at
            .as_ref()
            .is_some_and(|target| invocation.current_target() != target)
        {
            continue;
        }
        match reducer(invocation)? {
            SelfDrawnActionPropagation::Continue => {}
            SelfDrawnActionPropagation::Stop => {
                stopped_at = Some(invocation.current_target().clone());
            }
        }
    }
    dispatch.propagation_stopped_at = stopped_at;
    Ok(())
}
