use crate::event::{native_key_value, NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::{NativeEventContext, NativeEventPosition, NativeInputModality};

/// Tracks one pointer-driven move interaction independently of a native
/// toolkit. A press alone is not a move: the lifecycle begins with the first
/// non-zero position delta and ends on pointer release or cancellation.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct PointerMoveState {
    active: bool,
    did_move: bool,
    pointer_id: Option<u64>,
    last_position: Option<NativeEventPosition>,
}

impl PointerMoveState {
    pub(crate) fn begin(&mut self, context: NativeEventContext) {
        self.begin_pointer(0, context);
    }

    pub(crate) fn begin_pointer(&mut self, pointer_id: u64, context: NativeEventContext) {
        self.active = context.position.is_some_and(finite_position);
        self.did_move = false;
        self.pointer_id = self.active.then_some(pointer_id);
        self.last_position = self.active.then_some(context.position).flatten();
    }

    pub(crate) fn update(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        self.update_pointer(0, node, context)
    }

    pub(crate) fn update_pointer(
        &mut self,
        pointer_id: u64,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if !self.active || self.pointer_id != Some(pointer_id) {
            return Vec::new();
        }
        let Some(position) = context
            .position
            .filter(|position| finite_position(*position))
        else {
            return Vec::new();
        };
        let Some(previous) = self.last_position.replace(position) else {
            return Vec::new();
        };
        let delta_x = position.x - previous.x;
        let delta_y = position.y - previous.y;
        if delta_x == 0.0 && delta_y == 0.0 {
            return Vec::new();
        }

        let base_context = without_delta(context);
        let move_context = base_context.delta(delta_x, delta_y);
        if self.did_move {
            vec![NativeEvent::new(node, NativeEventKind::Move).context(move_context)]
        } else {
            self.did_move = true;
            vec![
                NativeEvent::new(node, NativeEventKind::MoveStart).context(base_context),
                NativeEvent::new(node, NativeEventKind::Move).context(move_context),
            ]
        }
    }

    pub(crate) fn end(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        self.end_pointer(0, node, context)
    }

    pub(crate) fn end_pointer(
        &mut self,
        pointer_id: u64,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if self.pointer_id != Some(pointer_id) {
            return Vec::new();
        }
        let emit_end = self.active && self.did_move;
        self.active = false;
        self.did_move = false;
        self.pointer_id = None;
        self.last_position = None;
        emit_end
            .then(|| {
                NativeEvent::new(node, NativeEventKind::MoveEnd).context(without_delta(context))
            })
            .into_iter()
            .collect()
    }

    pub(crate) fn cancel(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        let Some(pointer_id) = self.pointer_id else {
            return Vec::new();
        };
        self.end_pointer(pointer_id, node, context)
    }
}

pub(crate) fn keyboard_move_events(
    node: HostNodeId,
    key: &str,
    context: NativeEventContext,
) -> Vec<NativeEvent> {
    let (delta_x, delta_y) = match native_key_value(key).as_str() {
        "ArrowLeft" => (-1.0, 0.0),
        "ArrowRight" => (1.0, 0.0),
        "ArrowUp" => (0.0, -1.0),
        "ArrowDown" => (0.0, 1.0),
        _ => return Vec::new(),
    };
    let context = without_delta(context).modality(NativeInputModality::Keyboard);
    [
        NativeEvent::new(node, NativeEventKind::MoveStart).context(context),
        NativeEvent::new(node, NativeEventKind::Move).context(context.delta(delta_x, delta_y)),
        NativeEvent::new(node, NativeEventKind::MoveEnd).context(context),
    ]
    .into()
}

fn without_delta(mut context: NativeEventContext) -> NativeEventContext {
    context.delta = None;
    context
}

fn finite_position(position: NativeEventPosition) -> bool {
    position.x.is_finite() && position.y.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::NativeKeyModifiers;

    #[test]
    fn pointer_move_starts_only_after_a_non_zero_delta() {
        let node = HostNodeId::new(7);
        let mut state = PointerMoveState::default();
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Touch)
            .position(10.0, 20.0);

        state.begin(context);
        assert!(state.update(node, context).is_empty());
        let events = state.update(node, context.position(13.0, 18.0));

        assert_eq!(
            events.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![NativeEventKind::MoveStart, NativeEventKind::Move]
        );
        assert_eq!(events[0].context.delta, None);
        assert_eq!(
            events[1].context.delta,
            Some(NativeEventPosition::new(3.0, -2.0))
        );
    }

    #[test]
    fn pointer_move_reports_incremental_deltas_and_one_end() {
        let node = HostNodeId::new(8);
        let mut state = PointerMoveState::default();
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Mouse)
            .position(1.0, 1.0);

        state.begin(context);
        state.update(node, context.position(3.0, 4.0));
        let update = state.update(node, context.position(4.5, 2.0));
        assert_eq!(update.len(), 1);
        assert_eq!(update[0].kind, NativeEventKind::Move);
        assert_eq!(
            update[0].context.delta,
            Some(NativeEventPosition::new(1.5, -2.0))
        );

        let end = state.end(node, context.position(4.5, 2.0));
        assert_eq!(end.len(), 1);
        assert_eq!(end[0].kind, NativeEventKind::MoveEnd);
        assert!(state.end(node, context).is_empty());
    }

    #[test]
    fn tap_and_invalid_positions_do_not_emit_move_events() {
        let node = HostNodeId::new(9);
        let mut state = PointerMoveState::default();
        let context = NativeEventContext::new().position(2.0, 3.0);

        state.begin(context);
        assert!(state
            .update(node, context.position(f64::NAN, 4.0))
            .is_empty());
        assert!(state.end(node, context).is_empty());
    }

    #[test]
    fn a_second_pointer_cannot_take_over_an_active_move() {
        let node = HostNodeId::new(11);
        let mut state = PointerMoveState::default();
        let context = NativeEventContext::new().position(2.0, 3.0);

        state.begin_pointer(41, context);
        assert!(state
            .update_pointer(42, node, context.position(5.0, 6.0))
            .is_empty());
        assert!(state
            .end_pointer(42, node, context.position(5.0, 6.0))
            .is_empty());
        assert_eq!(
            state
                .update_pointer(41, node, context.position(4.0, 3.0))
                .len(),
            2
        );
        assert_eq!(state.cancel(node, context).len(), 1);
    }

    #[test]
    fn arrow_key_move_is_a_complete_keyboard_lifecycle() {
        let node = HostNodeId::new(10);
        let context = NativeEventContext::new()
            .modifiers(NativeKeyModifiers::new().shift(true))
            .repeat(true);

        let events = keyboard_move_events(node, "Left", context);
        assert_eq!(
            events.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![
                NativeEventKind::MoveStart,
                NativeEventKind::Move,
                NativeEventKind::MoveEnd
            ]
        );
        assert!(events.iter().all(|event| {
            event.context.modality == NativeInputModality::Keyboard
                && event.context.modifiers.shift
                && event.context.repeat
        }));
        assert_eq!(
            events[1].context.delta,
            Some(NativeEventPosition::new(-1.0, 0.0))
        );
        assert!(keyboard_move_events(node, "Enter", context).is_empty());
    }
}
