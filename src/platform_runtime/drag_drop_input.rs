use crate::event::NativeEventKind;
use crate::platform_host::{
    PlatformElementId, PlatformHostRevision, PlatformPoint, PlatformPointerId,
};

use super::drag_drop::{SelfDrawnDragSession, SelfDrawnDropOperation, SelfDrawnMatchedDropTarget};
use super::input::RoutedInput;
use super::interaction::{ActivePress, SelfDrawnEventContext, SelfDrawnInteractionSession};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn update_pointer_drag(
        &mut self,
        pointer: PlatformPointerId,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(position) = context.position else {
            return false;
        };
        let (mut session, started) = match self.active_drag.take() {
            Some(session) if session.pointer == Some(pointer) => (session, false),
            Some(session) => {
                self.active_drag = Some(session);
                return false;
            }
            None => {
                if active.long_press_recognized || tree.drag_source(&active.target).is_none() {
                    return false;
                }
                let Some(candidate) = active.drag_candidate.as_ref() else {
                    return false;
                };
                let delta = delta(candidate.start_position, position);
                if is_zero(delta) {
                    return false;
                }
                let candidate = active.drag_candidate.take().expect("candidate was checked");
                (
                    SelfDrawnDragSession {
                        source: active.target.clone(),
                        pointer: Some(pointer),
                        types: candidate.source.types,
                        value: candidate.source.value,
                        items: candidate.source.items,
                        allowed_operations: candidate.source.allowed_operations,
                        current_target: None,
                        current_item_indices: Vec::new(),
                        current_operation: SelfDrawnDropOperation::Cancel,
                        last_position: Some(candidate.start_position),
                    },
                    true,
                )
            }
        };

        let previous = session.last_position.unwrap_or(position);
        let movement = delta(previous, position);
        if is_zero(movement) {
            self.active_drag = Some(session);
            return false;
        }
        session.last_position = Some(position);

        if started {
            self.cancel_press_for_drag(
                active,
                tree,
                frame_revision,
                event_sequence,
                context,
                routed,
            );
            self.change_state(&session.source, &mut routed.changes, |state| {
                state.dragging = true;
            });
            let drag_context = source_context(&session, context, SelfDrawnDropOperation::Cancel);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &session.source,
                NativeEventKind::DragStart,
                drag_context,
                session.value.clone(),
                &mut routed.invocations,
            );
        }

        let mut drag_context = source_context(&session, context, session.current_operation);
        drag_context.delta = Some(movement);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &session.source,
            NativeEventKind::DragMove,
            drag_context,
            session.value.clone(),
            &mut routed.invocations,
        );
        let next = tree.drop_target_at(
            position,
            &session.items,
            &session.types,
            &session.allowed_operations,
        );
        self.transition_drop_target(
            &mut session,
            next,
            None,
            true,
            tree,
            frame_revision,
            event_sequence,
            context,
            routed,
        );
        routed.target = Some(session.source.clone());
        self.active_drag = Some(session);
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn leave_pointer_drag(
        &mut self,
        pointer: PlatformPointerId,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(mut session) = self.take_pointer_drag(pointer) else {
            return false;
        };
        self.transition_drop_target(
            &mut session,
            None,
            None,
            false,
            tree,
            frame_revision,
            event_sequence,
            context,
            routed,
        );
        routed.target = Some(session.source.clone());
        self.active_drag = Some(session);
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn finish_pointer_drag(
        &mut self,
        pointer: PlatformPointerId,
        active: &mut ActivePress,
        drop_requested: bool,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(mut session) = self.take_pointer_drag(pointer) else {
            return false;
        };
        self.end_active_move(
            active,
            tree,
            frame_revision,
            event_sequence,
            context,
            routed,
        );
        active.drag_candidate = None;

        let operation = if drop_requested {
            let next = context.position.and_then(|position| {
                tree.drop_target_at(
                    position,
                    &session.items,
                    &session.types,
                    &session.allowed_operations,
                )
            });
            self.transition_drop_target(
                &mut session,
                next,
                None,
                false,
                tree,
                frame_revision,
                event_sequence,
                context,
                routed,
            );
            self.drop_on_current_target(
                &mut session,
                tree,
                frame_revision,
                event_sequence,
                context,
                routed,
            )
        } else {
            self.transition_drop_target(
                &mut session,
                None,
                Some(SelfDrawnDropOperation::Cancel),
                false,
                tree,
                frame_revision,
                event_sequence,
                context,
                routed,
            );
            SelfDrawnDropOperation::Cancel
        };
        self.end_drag_source(
            &session,
            operation,
            tree,
            frame_revision,
            event_sequence,
            context,
            routed,
        );
        routed.target = Some(session.source);
        true
    }

    #[allow(clippy::too_many_arguments)]
    fn cancel_press_for_drag(
        &mut self,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let mut press_context = context.clone();
        press_context.click_count = active.click_count;
        self.end_active_long_press(
            active,
            tree,
            frame_revision,
            event_sequence,
            &press_context,
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
                press_context,
                Some("false".to_string()),
                &mut routed.invocations,
            );
            active.start_emitted = false;
        }
    }

    fn take_pointer_drag(&mut self, pointer: PlatformPointerId) -> Option<SelfDrawnDragSession> {
        let session = self.active_drag.take()?;
        if session.pointer == Some(pointer) {
            Some(session)
        } else {
            self.active_drag = Some(session);
            None
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn transition_drop_target(
        &mut self,
        session: &mut SelfDrawnDragSession,
        next: Option<SelfDrawnMatchedDropTarget>,
        exit_operation: Option<SelfDrawnDropOperation>,
        emit_move: bool,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let next_id = next.as_ref().map(|target| &target.id);
        if session.current_target.as_ref() != next_id {
            if let Some(previous) = session.current_target.take() {
                self.change_state(&previous, &mut routed.changes, |state| {
                    state.drop_target = false;
                });
                let operation = exit_operation.unwrap_or(session.current_operation);
                self.emit_drop_event(
                    session,
                    &previous,
                    NativeEventKind::DropExit,
                    operation,
                    tree,
                    frame_revision,
                    event_sequence,
                    context,
                    routed,
                );
            }
            session.current_item_indices.clear();
            session.current_operation = SelfDrawnDropOperation::Cancel;
            if let Some(next) = next {
                self.change_state(&next.id, &mut routed.changes, |state| {
                    state.drop_target = true;
                });
                session.current_target = Some(next.id.clone());
                session.current_item_indices = next.item_indices;
                session.current_operation = next.operation;
                self.emit_drop_event(
                    session,
                    &next.id,
                    NativeEventKind::DropEnter,
                    next.operation,
                    tree,
                    frame_revision,
                    event_sequence,
                    context,
                    routed,
                );
            }
        } else if let Some(next) = next {
            session.current_item_indices = next.item_indices;
            session.current_operation = next.operation;
        }

        if emit_move {
            if let Some(target) = session.current_target.clone() {
                self.emit_drop_event(
                    session,
                    &target,
                    NativeEventKind::DropMove,
                    session.current_operation,
                    tree,
                    frame_revision,
                    event_sequence,
                    context,
                    routed,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn drop_on_current_target(
        &mut self,
        session: &mut SelfDrawnDragSession,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> SelfDrawnDropOperation {
        let Some(target) = session.current_target.take() else {
            return SelfDrawnDropOperation::Cancel;
        };
        let operation = session.current_operation;
        self.emit_drop_event(
            session,
            &target,
            NativeEventKind::Drop,
            operation,
            tree,
            frame_revision,
            event_sequence,
            context,
            routed,
        );
        self.change_state(&target, &mut routed.changes, |state| {
            state.drop_target = false;
        });
        session.current_item_indices.clear();
        session.current_operation = SelfDrawnDropOperation::Cancel;
        operation
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn end_drag_source(
        &mut self,
        session: &SelfDrawnDragSession,
        operation: SelfDrawnDropOperation,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        self.change_state(&session.source, &mut routed.changes, |state| {
            state.dragging = false;
        });
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &session.source,
            NativeEventKind::DragEnd,
            source_context(session, context, operation),
            session.value.clone(),
            &mut routed.invocations,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_drop_event(
        &self,
        session: &SelfDrawnDragSession,
        target: &PlatformElementId,
        kind: NativeEventKind,
        operation: SelfDrawnDropOperation,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let mut drop_context = context.clone();
        drop_context.drag = Some(session.target_context(operation));
        if let Some(position) = context.position {
            drop_context.position = tree.local_position(target, position);
        }
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            target,
            kind,
            drop_context,
            session.value.clone(),
            &mut routed.invocations,
        );
    }
}

pub(super) fn source_context(
    session: &SelfDrawnDragSession,
    context: &SelfDrawnEventContext,
    operation: SelfDrawnDropOperation,
) -> SelfDrawnEventContext {
    let mut context = context.clone();
    context.drag = Some(session.context(operation));
    context
}

fn delta(previous: PlatformPoint, next: PlatformPoint) -> PlatformPoint {
    PlatformPoint::new(next.x - previous.x, next.y - previous.y)
}

fn is_zero(point: PlatformPoint) -> bool {
    point.x == 0.0 && point.y == 0.0
}
