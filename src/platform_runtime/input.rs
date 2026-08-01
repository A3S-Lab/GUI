use crate::error::{GuiError, GuiResult};
use crate::event::NativeEventKind;
use crate::platform_host::{
    PlatformElementId, PlatformHostRevision, PlatformInputEvent, PlatformPointerButton,
    PlatformPointerEvent, PlatformPointerPhase,
};

use super::interaction::{
    ActivePress, LastClick, LongPressTracking, PointerInteraction, RoutedSemanticEvent,
    SelfDrawnEventContext, SelfDrawnInputDispatch, SelfDrawnInteractionChange,
    SelfDrawnInteractionSession,
};
use super::interaction_tree::SelfDrawnInteractionTree;

const MULTI_CLICK_INTERVAL_MICROS: u64 = 500_000;

#[derive(Debug, Default)]
pub(super) struct RoutedInput {
    pub(super) target: Option<PlatformElementId>,
    pub(super) invocations: Vec<super::SelfDrawnActionInvocation>,
    pub(super) changes: Vec<SelfDrawnInteractionChange>,
}

impl SelfDrawnInteractionSession {
    pub(super) fn route_input(
        &mut self,
        event: &PlatformInputEvent,
        frame_revision: PlatformHostRevision,
        tree: &SelfDrawnInteractionTree,
    ) -> GuiResult<SelfDrawnInputDispatch> {
        let event_sequence = self
            .event_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("self-drawn input event sequence overflowed"))?;
        let routed = match event {
            PlatformInputEvent::Pointer { event } => {
                self.route_pointer(event, frame_revision, event_sequence, tree)
            }
            PlatformInputEvent::Key { event } => {
                self.route_key(event, frame_revision, event_sequence, tree)
            }
            PlatformInputEvent::Wheel { event } => {
                self.route_wheel(event, frame_revision, event_sequence, tree)
            }
            PlatformInputEvent::ModifiersChanged { .. } => RoutedInput::default(),
        };
        self.event_sequence = event_sequence;
        Ok(SelfDrawnInputDispatch {
            frame_revision,
            event_sequence,
            target: routed.target,
            invocations: routed.invocations,
            interaction_changes: routed.changes,
            propagation_stopped_at: None,
        })
    }

    fn route_pointer(
        &mut self,
        event: &PlatformPointerEvent,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        tree: &SelfDrawnInteractionTree,
    ) -> RoutedInput {
        let hit_target = tree.hit_test(event.position);
        let mut pointer = self.pointers.remove(&event.pointer).unwrap_or_default();
        let mut routed = RoutedInput {
            target: hit_target.clone(),
            ..RoutedInput::default()
        };
        let mut context = SelfDrawnEventContext::pointer(
            event.device,
            event.pointer,
            event.modality,
            event.modifiers,
            event.position,
            event.button,
            event.pressure,
            event.timestamp_micros,
        );

        match event.phase {
            PlatformPointerPhase::Entered | PlatformPointerPhase::Moved => {
                self.transition_hover(
                    &mut pointer,
                    hit_target.clone(),
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
                self.transition_active_press_boundary(
                    &mut pointer,
                    hit_target.as_ref(),
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
            }
            PlatformPointerPhase::Left => {
                routed.target = pointer
                    .active_press
                    .as_ref()
                    .map(|press| press.target.clone());
                self.transition_hover(
                    &mut pointer,
                    None,
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
                self.transition_active_press_boundary(
                    &mut pointer,
                    None,
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
            }
            PlatformPointerPhase::Pressed => {
                self.transition_hover(
                    &mut pointer,
                    hit_target.clone(),
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
                if event.button == Some(PlatformPointerButton::Primary) {
                    if let Some(active) = pointer.active_press.take() {
                        self.cancel_active_press(
                            active,
                            tree,
                            frame_revision,
                            event_sequence,
                            &context,
                            &mut routed,
                        );
                    }
                    if let Some(target) = hit_target {
                        let click_count =
                            self.next_click_count(event.pointer, &target, event.timestamp_micros);
                        context.click_count = click_count;
                        let focus = tree.focus_target(&target);
                        self.transition_focus(
                            tree,
                            frame_revision,
                            event_sequence,
                            focus,
                            &context,
                            &mut routed.invocations,
                            &mut routed.changes,
                        );
                        let long_press_threshold_micros =
                            tree.long_press_threshold_micros(&target, context.modality);
                        let long_press = long_press_threshold_micros.map(|threshold| {
                            let mut long_press_context = context.clone();
                            long_press_context.click_count = click_count;
                            LongPressTracking {
                                deadline_micros: event.timestamp_micros.saturating_add(threshold),
                                context: long_press_context,
                            }
                        });
                        if long_press.is_some() {
                            self.begin_long_press(&target, &mut routed.changes);
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &target,
                                NativeEventKind::LongPressStart,
                                context.clone(),
                                None,
                                &mut routed.invocations,
                            );
                        }
                        self.begin_press(&target, &mut routed.changes);
                        self.emit(
                            tree,
                            frame_revision,
                            event_sequence,
                            &target,
                            NativeEventKind::PressStart,
                            context.clone(),
                            Some("true".to_string()),
                            &mut routed.invocations,
                        );
                        pointer.active_press = Some(ActivePress {
                            target: target.clone(),
                            over_target: true,
                            start_emitted: true,
                            click_count,
                            long_press_threshold_micros,
                            long_press,
                            long_press_recognized: false,
                        });
                        routed.target = Some(target);
                    }
                }
            }
            PlatformPointerPhase::Released => {
                self.transition_hover(
                    &mut pointer,
                    hit_target.clone(),
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
                if event.button == Some(PlatformPointerButton::Primary) {
                    if let Some(mut active) = pointer.active_press.take() {
                        routed.target = Some(active.target.clone());
                        let over_target = hit_target.as_ref() == Some(&active.target);
                        let recognized_now = over_target
                            && self.recognize_long_press_if_due(
                                &mut active,
                                event.timestamp_micros,
                                tree,
                                frame_revision,
                                event_sequence,
                                &mut routed,
                            );
                        if !recognized_now
                            && !active.long_press_recognized
                            && over_target
                            && !active.start_emitted
                        {
                            context.click_count = active.click_count;
                            self.start_active_long_press(
                                &mut active,
                                tree,
                                frame_revision,
                                event_sequence,
                                &context,
                                &mut routed,
                            );
                            self.begin_press(&active.target, &mut routed.changes);
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &active.target,
                                NativeEventKind::PressStart,
                                context.clone(),
                                Some("true".to_string()),
                                &mut routed.invocations,
                            );
                            active.start_emitted = true;
                        } else if !active.long_press_recognized
                            && !over_target
                            && active.start_emitted
                        {
                            context.click_count = active.click_count;
                            self.end_active_long_press(
                                &mut active,
                                tree,
                                frame_revision,
                                event_sequence,
                                &context,
                                &mut routed,
                            );
                            self.end_press(&active.target, &mut routed.changes);
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &active.target,
                                NativeEventKind::PressEnd,
                                context.clone(),
                                Some("false".to_string()),
                                &mut routed.invocations,
                            );
                            active.start_emitted = false;
                        }
                        if !recognized_now
                            && !active.long_press_recognized
                            && over_target
                            && active.start_emitted
                        {
                            context.click_count = active.click_count;
                            self.end_active_long_press(
                                &mut active,
                                tree,
                                frame_revision,
                                event_sequence,
                                &context,
                                &mut routed,
                            );
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &active.target,
                                NativeEventKind::PressUp,
                                context.clone(),
                                None,
                                &mut routed.invocations,
                            );
                            self.end_press(&active.target, &mut routed.changes);
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &active.target,
                                NativeEventKind::PressEnd,
                                context.clone(),
                                Some("false".to_string()),
                                &mut routed.invocations,
                            );
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &active.target,
                                NativeEventKind::Press,
                                context,
                                None,
                                &mut routed.invocations,
                            );
                            self.last_clicks.insert(
                                event.pointer,
                                LastClick {
                                    target: active.target,
                                    timestamp_micros: event.timestamp_micros,
                                    count: active.click_count,
                                },
                            );
                        }
                    }
                }
            }
            PlatformPointerPhase::Cancelled => {
                routed.target = pointer
                    .active_press
                    .as_ref()
                    .map(|press| press.target.clone());
                self.transition_hover(
                    &mut pointer,
                    None,
                    tree,
                    frame_revision,
                    event_sequence,
                    &context,
                    &mut routed,
                );
                if let Some(active) = pointer.active_press.take() {
                    self.cancel_active_press(
                        active,
                        tree,
                        frame_revision,
                        event_sequence,
                        &context,
                        &mut routed,
                    );
                }
            }
        }
        if pointer.hover_target.is_some() || pointer.active_press.is_some() {
            self.pointers.insert(event.pointer, pointer);
        }
        routed
    }

    #[allow(clippy::too_many_arguments)]
    fn transition_hover(
        &mut self,
        pointer: &mut PointerInteraction,
        next: Option<PlatformElementId>,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let next = context.modality.supports_hover().then_some(next).flatten();
        if pointer.hover_target == next {
            return;
        }
        if let Some(previous) = pointer.hover_target.take() {
            self.end_hover(&previous, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &previous,
                NativeEventKind::HoverEnd,
                context.clone(),
                Some("false".to_string()),
                &mut routed.invocations,
            );
        }
        if let Some(next) = next {
            self.begin_hover(&next, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &next,
                NativeEventKind::HoverStart,
                context.clone(),
                Some("true".to_string()),
                &mut routed.invocations,
            );
            pointer.hover_target = Some(next);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn transition_active_press_boundary(
        &mut self,
        pointer: &mut PointerInteraction,
        hit_target: Option<&PlatformElementId>,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(active) = pointer.active_press.as_mut() else {
            return;
        };
        let over_target = hit_target == Some(&active.target);
        if active.long_press_recognized {
            active.over_target = over_target;
            return;
        }
        if over_target == active.over_target {
            return;
        }
        active.over_target = over_target;
        let mut context = context.clone();
        context.click_count = active.click_count;
        if over_target {
            self.start_active_long_press(
                active,
                tree,
                frame_revision,
                event_sequence,
                &context,
                routed,
            );
            self.begin_press(&active.target, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &active.target,
                NativeEventKind::PressStart,
                context,
                Some("true".to_string()),
                &mut routed.invocations,
            );
            active.start_emitted = true;
        } else if active.start_emitted {
            self.end_active_long_press(
                active,
                tree,
                frame_revision,
                event_sequence,
                &context,
                routed,
            );
            self.end_press(&active.target, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &active.target,
                NativeEventKind::PressEnd,
                context,
                Some("false".to_string()),
                &mut routed.invocations,
            );
            active.start_emitted = false;
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn cancel_active_press(
        &mut self,
        mut active: ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        if active.long_press_recognized {
            return;
        }
        let mut context = context.clone();
        context.click_count = active.click_count;
        self.end_active_long_press(
            &mut active,
            tree,
            frame_revision,
            event_sequence,
            &context,
            routed,
        );
        if !active.start_emitted {
            return;
        }
        self.end_press(&active.target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::PressCancel,
            context,
            Some("false".to_string()),
            &mut routed.invocations,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit(
        &self,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        target: &PlatformElementId,
        kind: NativeEventKind,
        context: SelfDrawnEventContext,
        value: Option<String>,
        invocations: &mut Vec<super::SelfDrawnActionInvocation>,
    ) {
        self.route_event(
            tree,
            frame_revision,
            event_sequence,
            &RoutedSemanticEvent {
                target: target.clone(),
                kind,
                context,
                value,
            },
            true,
            invocations,
        );
    }

    fn next_click_count(
        &self,
        pointer: crate::platform_host::PlatformPointerId,
        target: &PlatformElementId,
        timestamp_micros: u64,
    ) -> u8 {
        let Some(previous) = self.last_clicks.get(&pointer) else {
            return 1;
        };
        if previous.target != *target
            || timestamp_micros.saturating_sub(previous.timestamp_micros)
                > MULTI_CLICK_INTERVAL_MICROS
        {
            return 1;
        }
        if previous.count == 1 {
            2
        } else {
            1
        }
    }
}
