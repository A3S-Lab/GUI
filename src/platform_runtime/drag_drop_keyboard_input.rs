use crate::event::NativeEventKind;
use crate::platform_host::PlatformHostRevision;

use super::drag_drop::{SelfDrawnDragSession, SelfDrawnDropOperation};
use super::drag_drop_input::source_context;
use super::input::RoutedInput;
use super::interaction::{SelfDrawnEventContext, SelfDrawnInteractionSession};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn route_keyboard_drag(
        &mut self,
        key: &str,
        reverse: bool,
        repeat: bool,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &mut SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        if key == "Escape" && !repeat {
            if let Some(pointer_id) = self.active_drag.as_ref().and_then(|drag| drag.pointer) {
                if let Some(mut pointer) = self.pointers.remove(&pointer_id) {
                    if let Some(mut active) = pointer.active_press.take() {
                        context.handled_activation = true;
                        let handled = self.finish_pointer_drag(
                            pointer_id,
                            &mut active,
                            false,
                            tree,
                            frame_revision,
                            event_sequence,
                            context,
                            routed,
                        );
                        if pointer.hover_target.is_some() || pointer.active_press.is_some() {
                            self.pointers.insert(pointer_id, pointer);
                        }
                        if handled {
                            return true;
                        }
                    } else {
                        self.pointers.insert(pointer_id, pointer);
                    }
                }
            }
        }
        if self
            .active_drag
            .as_ref()
            .is_some_and(|session| session.pointer.is_none())
        {
            context.handled_activation = true;
            if !repeat {
                match key {
                    "Tab" => self.cycle_keyboard_drop_target(
                        reverse,
                        tree,
                        frame_revision,
                        event_sequence,
                        context,
                        routed,
                    ),
                    "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | "Home" | "End" => self
                        .move_keyboard_collection_target(
                            key,
                            tree,
                            frame_revision,
                            event_sequence,
                            context,
                            routed,
                        ),
                    "Enter" => self.finish_keyboard_drag(
                        true,
                        tree,
                        frame_revision,
                        event_sequence,
                        context,
                        routed,
                    ),
                    "Escape" => self.finish_keyboard_drag(
                        false,
                        tree,
                        frame_revision,
                        event_sequence,
                        context,
                        routed,
                    ),
                    _ => {}
                }
            }
            return true;
        }
        if key != "Enter" || repeat {
            return false;
        }
        let Some(source_id) = self.focused.clone() else {
            return false;
        };
        let Some(source) = tree.drag_source_for_start(&source_id) else {
            return false;
        };

        context.handled_activation = true;
        let session = SelfDrawnDragSession {
            source: source_id.clone(),
            pointer: None,
            types: source.types,
            value: source.value,
            items: source.items,
            allowed_operations: source.allowed_operations,
            source_collection: source.collection,
            dragging_keys: source.dragging_keys,
            dragging_nodes: source.dragging_nodes,
            current_target: None,
            current_collection: None,
            current_collection_target: None,
            current_item_indices: Vec::new(),
            current_operation: SelfDrawnDropOperation::Cancel,
            last_position: None,
        };
        let dragging_nodes = if session.dragging_nodes.is_empty() {
            vec![source_id.clone()]
        } else {
            session.dragging_nodes.clone()
        };
        for node in dragging_nodes {
            self.change_state(&node, &mut routed.changes, |state| state.dragging = true);
        }
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &source_id,
            NativeEventKind::DragStart,
            source_context(&session, context, SelfDrawnDropOperation::Cancel),
            session.value.clone(),
            &mut routed.invocations,
        );
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &source_id,
            NativeEventKind::KeyDown,
            context.clone(),
            Some(key.to_string()),
            &mut routed.invocations,
        );
        routed.target = Some(source_id);
        self.active_drag = Some(session);
        true
    }

    #[allow(clippy::too_many_arguments)]
    fn cycle_keyboard_drop_target(
        &mut self,
        reverse: bool,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(mut session) = self.active_drag.take() else {
            return;
        };
        let candidates = tree.compatible_drop_targets(
            &session.items,
            &session.types,
            &session.allowed_operations,
            session.source_collection.as_ref(),
            &session.dragging_keys,
        );
        let current = candidates.iter().position(|candidate| {
            if let Some(collection) = session.current_collection.as_ref() {
                candidate.collection.as_ref() == Some(collection)
            } else {
                session.current_target.as_ref().is_some_and(|current| {
                    candidate.collection.is_none() && candidate.id == *current
                })
            }
        });
        let next = if candidates.is_empty() {
            None
        } else {
            let index = match (current, reverse) {
                (Some(0), true) | (None, true) => candidates.len() - 1,
                (Some(index), true) => index - 1,
                (Some(index), false) => (index + 1) % candidates.len(),
                (None, false) => 0,
            };
            Some(candidates[index].clone())
        };
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
        let focused = session
            .current_target
            .clone()
            .filter(|target| tree.is_focusable(target));
        if let Some(focused) = focused {
            self.transition_focus(
                tree,
                frame_revision,
                event_sequence,
                Some(focused.clone()),
                context,
                &mut routed.invocations,
                &mut routed.changes,
            );
            routed.target = Some(focused);
        }
        self.active_drag = Some(session);
    }

    #[allow(clippy::too_many_arguments)]
    fn move_keyboard_collection_target(
        &mut self,
        key: &str,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(mut session) = self.active_drag.take() else {
            return;
        };
        let next = session
            .current_collection
            .as_ref()
            .zip(session.current_collection_target.as_ref())
            .and_then(|(collection, target)| {
                tree.keyboard_collection_target(
                    collection,
                    target,
                    key,
                    &session.items,
                    &session.types,
                    &session.allowed_operations,
                    session.source_collection.as_ref(),
                    &session.dragging_keys,
                )
            });
        if let Some(next) = next {
            self.transition_drop_target(
                &mut session,
                Some(next),
                None,
                true,
                tree,
                frame_revision,
                event_sequence,
                context,
                routed,
            );
            if let Some(focused) = session
                .current_target
                .clone()
                .filter(|target| tree.is_focusable(target))
            {
                self.transition_focus(
                    tree,
                    frame_revision,
                    event_sequence,
                    Some(focused.clone()),
                    context,
                    &mut routed.invocations,
                    &mut routed.changes,
                );
                routed.target = Some(focused);
            }
        }
        self.active_drag = Some(session);
    }

    #[allow(clippy::too_many_arguments)]
    fn finish_keyboard_drag(
        &mut self,
        drop_requested: bool,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(mut session) = self.active_drag.take() else {
            return;
        };
        if drop_requested && session.current_target.is_none() {
            self.active_drag = Some(session);
            return;
        }
        let operation = if drop_requested {
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
        let source = session.source;
        if tree.is_focusable(&source) {
            self.transition_focus(
                tree,
                frame_revision,
                event_sequence,
                Some(source.clone()),
                context,
                &mut routed.invocations,
                &mut routed.changes,
            );
        }
        routed.target = Some(source);
    }
}
