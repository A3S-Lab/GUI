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
                        source_collection: candidate.source.collection,
                        dragging_keys: candidate.source.dragging_keys,
                        dragging_nodes: candidate.source.dragging_nodes,
                        current_target: None,
                        current_collection: None,
                        current_collection_target: None,
                        current_item_indices: Vec::new(),
                        current_operation: SelfDrawnDropOperation::Cancel,
                        last_position: Some(candidate.start_position),
                        drop_activation: None,
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
            let dragging_nodes = if session.dragging_nodes.is_empty() {
                vec![session.source.clone()]
            } else {
                session.dragging_nodes.clone()
            };
            for node in dragging_nodes {
                self.change_state(&node, &mut routed.changes, |state| state.dragging = true);
            }
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
            session.source_collection.as_ref(),
            &session.dragging_keys,
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
                    session.source_collection.as_ref(),
                    &session.dragging_keys,
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
        let exact_target = match (&session.current_target, &next) {
            (None, None) => true,
            (Some(current), Some(next)) => {
                current == &next.id
                    && session.current_collection.as_ref() == next.collection.as_ref()
                    && session.current_collection_target.as_ref() == next.collection_target.as_ref()
            }
            _ => false,
        };
        let equivalent_collection_target = match (
            session.current_collection.as_ref(),
            session.current_collection_target.as_ref(),
            next.as_ref(),
        ) {
            (Some(collection), Some(current), Some(next))
                if next.collection.as_ref() == Some(collection) =>
            {
                next.collection_target.as_ref().is_some_and(|candidate| {
                    tree.collection_targets_equivalent(collection, current, candidate)
                })
            }
            _ => false,
        };
        let same_target = exact_target || equivalent_collection_target;
        if !same_target {
            session.drop_activation = None;
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
            session.current_collection = None;
            session.current_collection_target = None;
            session.current_item_indices.clear();
            session.current_operation = SelfDrawnDropOperation::Cancel;
            if let Some(next) = next {
                self.change_state(&next.id, &mut routed.changes, |state| {
                    state.drop_target = true;
                });
                session.current_target = Some(next.id.clone());
                session.current_collection = next.collection;
                session.current_collection_target = next.collection_target;
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
                self.start_drop_activation(session, tree, context);
            }
        } else if let Some(next) = next {
            if !equivalent_collection_target {
                session.current_collection = next.collection;
                session.current_collection_target = next.collection_target;
            }
            session.current_item_indices = next.item_indices;
            session.current_operation = next.operation;
            self.refresh_drop_activation(session, context);
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
        session.drop_activation = None;
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
        session.current_collection = None;
        session.current_collection_target = None;
        session.current_item_indices.clear();
        session.current_operation = SelfDrawnDropOperation::Cancel;
        operation
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
        if kind == NativeEventKind::Drop {
            if let (Some(collection), Some(collection_target)) = (
                session.current_collection.as_ref(),
                session.current_collection_target.as_ref(),
            ) {
                let actions = tree.collection_drop_actions(
                    collection,
                    collection_target,
                    session.source_collection.as_ref(),
                    &session.dragging_keys,
                );
                if !actions.is_empty() {
                    for action in actions {
                        routed.invocations.push(super::SelfDrawnActionInvocation {
                            frame_revision,
                            event_sequence,
                            node: target.clone(),
                            current_target: (collection != target).then(|| collection.clone()),
                            action: action.to_string(),
                            event: kind,
                            context: drop_context.clone(),
                            value: session.value.clone(),
                        });
                    }
                    return;
                }
            }
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
