use crate::error::{GuiError, GuiResult};
use crate::event::NativeEventKind;
use crate::platform_host::PlatformHostRevision;

use super::drag_drop::{
    SelfDrawnDragSession, SelfDrawnDropActivationTracking, DEFAULT_DROP_ACTIVATE_THRESHOLD_MICROS,
};
use super::drag_drop_collection::SelfDrawnCollectionDropTarget;
use super::interaction::{
    SelfDrawnEventContext, SelfDrawnInputDispatch, SelfDrawnInteractionSession,
};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionTree {
    pub(super) fn supports_drop_activation(&self, session: &SelfDrawnDragSession) -> bool {
        let Some(target) = session.current_target.as_ref() else {
            return false;
        };
        if session.current_collection.is_some()
            && !matches!(
                session.current_collection_target.as_ref(),
                Some(SelfDrawnCollectionDropTarget::Item { .. })
            )
        {
            return false;
        }
        self.ancestors_inclusive(target).into_iter().any(|id| {
            self.node(&id)
                .and_then(|node| node.props.web.events.get("onDropActivate"))
                .is_some_and(|action| !action.trim().is_empty())
        })
    }

    pub(super) fn drop_activation_tracking_is_valid(
        &self,
        session: &SelfDrawnDragSession,
        tracking: &SelfDrawnDropActivationTracking,
    ) -> bool {
        if session.current_target.as_ref() != Some(&tracking.target)
            || session.current_collection.as_ref() != tracking.collection.as_ref()
            || !self.supports_drop_activation(session)
        {
            return false;
        }
        if session.current_collection_target == tracking.collection_target {
            return true;
        }
        match (
            session.current_collection.as_ref(),
            session.current_collection_target.as_ref(),
            tracking.collection_target.as_ref(),
        ) {
            (Some(collection), Some(current), Some(previous)) => {
                self.collection_targets_equivalent(collection, current, previous)
            }
            _ => false,
        }
    }
}

impl SelfDrawnInteractionSession {
    pub(super) fn start_drop_activation(
        &self,
        session: &mut SelfDrawnDragSession,
        tree: &SelfDrawnInteractionTree,
        context: &SelfDrawnEventContext,
    ) {
        let Some(target) = session
            .current_target
            .clone()
            .filter(|_| tree.supports_drop_activation(session))
        else {
            session.drop_activation = None;
            return;
        };
        session.drop_activation = Some(SelfDrawnDropActivationTracking {
            deadline_micros: context
                .timestamp_micros
                .saturating_add(DEFAULT_DROP_ACTIVATE_THRESHOLD_MICROS),
            target,
            collection: session.current_collection.clone(),
            collection_target: session.current_collection_target.clone(),
            context: context.clone(),
        });
    }

    pub(super) fn refresh_drop_activation(
        &self,
        session: &mut SelfDrawnDragSession,
        context: &SelfDrawnEventContext,
    ) {
        if let Some(tracking) = session.drop_activation.as_mut() {
            tracking.context = context.clone();
        }
    }

    pub(super) fn next_drop_activation_deadline_micros(&self) -> Option<u64> {
        self.active_drag
            .as_ref()?
            .drop_activation
            .as_ref()
            .map(|tracking| tracking.deadline_micros)
    }

    pub(super) fn route_drop_activation_time(
        &mut self,
        timestamp_micros: u64,
        frame_revision: PlatformHostRevision,
        tree: &SelfDrawnInteractionTree,
    ) -> GuiResult<Option<SelfDrawnInputDispatch>> {
        let is_due = self
            .active_drag
            .as_ref()
            .and_then(|session| session.drop_activation.as_ref())
            .is_some_and(|tracking| tracking.deadline_micros <= timestamp_micros);
        if !is_due {
            return Ok(None);
        }
        let event_sequence = self
            .event_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("self-drawn input event sequence overflowed"))?;

        let Some(mut session) = self.active_drag.take() else {
            return Ok(None);
        };
        let Some(tracking) = session.drop_activation.take() else {
            self.active_drag = Some(session);
            return Ok(None);
        };
        if !tree.drop_activation_tracking_is_valid(&session, &tracking) {
            self.active_drag = Some(session);
            return Ok(None);
        }
        let target = tracking.target;
        let mut context = tracking.context;
        context.timestamp_micros = timestamp_micros;
        context.drag = Some(session.target_context(session.current_operation));
        if let Some(position) = context.position {
            context.position = tree.local_position(&target, position);
        }
        let mut invocations = Vec::new();
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &target,
            NativeEventKind::DropActivate,
            context,
            session.value.clone(),
            &mut invocations,
        );
        self.active_drag = Some(session);
        self.event_sequence = event_sequence;
        Ok(Some(SelfDrawnInputDispatch {
            frame_revision,
            event_sequence,
            target: Some(target),
            invocations,
            interaction_changes: Vec::new(),
            propagation_stopped_at: None,
        }))
    }
}
