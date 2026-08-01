use crate::event::NativeEventKind;
use crate::platform_host::PlatformHostRevision;

use super::drag_drop::{SelfDrawnDragSession, SelfDrawnDropOperation};
use super::drag_drop_input::source_context;
use super::input::RoutedInput;
use super::interaction::{SelfDrawnEventContext, SelfDrawnInteractionSession};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
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
        let dragging_nodes = if session.dragging_nodes.is_empty() {
            vec![session.source.clone()]
        } else {
            session.dragging_nodes.clone()
        };
        for node in dragging_nodes {
            self.change_state(&node, &mut routed.changes, |state| state.dragging = false);
        }
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
}
