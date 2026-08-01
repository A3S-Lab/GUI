use crate::event::NativeEventKind;
use crate::platform_host::{PlatformElementId, PlatformHostRevision, PlatformPoint};

use super::input::RoutedInput;
use super::interaction::{ActivePress, SelfDrawnEventContext, SelfDrawnInteractionSession};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn update_active_move(
        &mut self,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(movement) = active.movement.as_mut() else {
            return false;
        };
        let Some(position) = context.position else {
            return false;
        };
        let previous = std::mem::replace(&mut movement.last_position, position);
        let delta = PlatformPoint::new(position.x - previous.x, position.y - previous.y);
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }

        let mut base_context = context.clone();
        base_context.click_count = active.click_count;
        base_context.delta = None;
        if !movement.did_move {
            movement.did_move = true;
            self.begin_move(&active.target, &mut routed.changes);
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &active.target,
                NativeEventKind::MoveStart,
                base_context.clone(),
                None,
                &mut routed.invocations,
            );
        }
        let mut move_context = base_context;
        move_context.delta = Some(delta);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::Move,
            move_context,
            None,
            &mut routed.invocations,
        );
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn end_active_move(
        &mut self,
        active: &mut ActivePress,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) -> bool {
        let Some(movement) = active.movement.take() else {
            return false;
        };
        if !movement.did_move {
            return false;
        }
        let mut context = context.clone();
        context.click_count = active.click_count;
        context.delta = None;
        self.end_move(&active.target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            &active.target,
            NativeEventKind::MoveEnd,
            context,
            None,
            &mut routed.invocations,
        );
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn route_keyboard_move(
        &mut self,
        target: &PlatformElementId,
        key: &str,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        context: &mut SelfDrawnEventContext,
        routed: &mut RoutedInput,
    ) {
        let Some(delta) = keyboard_delta(key) else {
            return;
        };
        if !tree.tracks_movement(target) {
            return;
        }
        context.handled_activation = true;
        context.delta = None;
        self.begin_move(target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            target,
            NativeEventKind::MoveStart,
            context.clone(),
            None,
            &mut routed.invocations,
        );
        let mut move_context = context.clone();
        move_context.delta = Some(delta);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            target,
            NativeEventKind::Move,
            move_context,
            None,
            &mut routed.invocations,
        );
        self.end_move(target, &mut routed.changes);
        self.emit(
            tree,
            frame_revision,
            event_sequence,
            target,
            NativeEventKind::MoveEnd,
            context.clone(),
            None,
            &mut routed.invocations,
        );
    }
}

fn keyboard_delta(key: &str) -> Option<PlatformPoint> {
    match key {
        "ArrowLeft" => Some(PlatformPoint::new(-1.0, 0.0)),
        "ArrowRight" => Some(PlatformPoint::new(1.0, 0.0)),
        "ArrowUp" => Some(PlatformPoint::new(0.0, -1.0)),
        "ArrowDown" => Some(PlatformPoint::new(0.0, 1.0)),
        _ => None,
    }
}
