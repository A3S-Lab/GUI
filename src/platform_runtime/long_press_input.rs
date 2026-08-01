use crate::error::{GuiError, GuiResult};
use crate::event::NativeEventKind;
use crate::platform_host::PlatformHostRevision;

use super::input::RoutedInput;
use super::interaction::{
    ActivePress, LongPressTracking, SelfDrawnEventContext, SelfDrawnInputDispatch,
    SelfDrawnInteractionSession,
};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
    pub(super) fn route_interaction_time(
        &mut self,
        timestamp_micros: u64,
        frame_revision: PlatformHostRevision,
        tree: &SelfDrawnInteractionTree,
    ) -> GuiResult<Option<SelfDrawnInputDispatch>> {
        let due_pointer = self
            .pointers
            .iter()
            .filter_map(|(pointer, interaction)| {
                interaction
                    .active_press
                    .as_ref()
                    .and_then(|press| press.long_press.as_ref())
                    .filter(|tracking| tracking.deadline_micros <= timestamp_micros)
                    .map(|tracking| (tracking.deadline_micros, *pointer))
            })
            .min()
            .map(|(_, pointer)| pointer);
        let Some(pointer_id) = due_pointer else {
            return Ok(None);
        };
        let event_sequence = self
            .event_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("self-drawn input event sequence overflowed"))?;
        let mut routed = RoutedInput::default();
        let Some(mut pointer) = self.pointers.remove(&pointer_id) else {
            return Ok(None);
        };
        let target = pointer.active_press.as_mut().and_then(|active| {
            let target = active.target.clone();
            self.recognize_long_press_if_due(
                active,
                timestamp_micros,
                tree,
                frame_revision,
                event_sequence,
                &mut routed,
            )
            .then_some(target)
        });
        if pointer.hover_target.is_some() || pointer.active_press.is_some() {
            self.pointers.insert(pointer_id, pointer);
        }
        let Some(target) = target else {
            return Ok(None);
        };
        self.event_sequence = event_sequence;
        Ok(Some(SelfDrawnInputDispatch {
            frame_revision,
            event_sequence,
            target: Some(target),
            invocations: routed.invocations,
            interaction_changes: routed.changes,
            propagation_stopped_at: None,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn start_active_long_press(
        &mut self,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(threshold_micros) = active.long_press_threshold_micros else {
            return;
        };
        if active.long_press.is_some() || active.long_press_recognized {
            return;
        }
        let mut context = context.clone();
        context.click_count = active.click_count;
        active.long_press = Some(LongPressTracking {
            deadline_micros: context.timestamp_micros.saturating_add(threshold_micros),
            context: context.clone(),
        });
        self.begin_long_press(&active.target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::LongPressStart,
            context,
            None,
            &mut routed.invocations,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn end_active_long_press(
        &mut self,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        if active.long_press.take().is_none() {
            return false;
        }
        self.end_long_press(&active.target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::LongPressEnd,
            context.clone(),
            None,
            &mut routed.invocations,
        );
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn recognize_long_press_if_due(
        &mut self,
        active: &mut ActivePress,
        timestamp_micros: u64,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(tracking) = active.long_press.as_ref() else {
            return false;
        };
        if tracking.deadline_micros > timestamp_micros || active.long_press_recognized {
            return false;
        }
        let mut context = tracking.context.clone();
        context.timestamp_micros = timestamp_micros;
        active.long_press_recognized = true;
        self.end_active_long_press(
            active,
            tree,
            frame_revision,
            event_sequence,
            &context,
            routed,
        );
        if active.start_emitted {
            self.end_press(&active.target, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &active.target,
                NativeEventKind::PressCancel,
                context.clone(),
                Some("false".to_string()),
                &mut routed.invocations,
            );
            active.start_emitted = false;
        }
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::LongPress,
            context,
            None,
            &mut routed.invocations,
        );
        true
    }
}
