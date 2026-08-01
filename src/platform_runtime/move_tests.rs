use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{
    PlatformElementId, PlatformKeyState, PlatformPointerId, PlatformPointerPhase,
};
use crate::web::WebProps;
use crate::{GuiError, NativeEventKind, PlatformPoint};

use super::tests::{key_event, pointer_event, pointer_event_for, runtime};
use super::{SelfDrawnActionPropagation, SelfDrawnHostEventOutcome, SelfDrawnInputDispatch};

fn move_tree(color: &str) -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("move", NativeRole::View).with_props(
                NativeProps::new().tab_index(Some(0)).web(
                    WebProps::new()
                        .class_name(format!(
                            "absolute left-[10px] top-[10px] h-[30px] w-[50px] {color}"
                        ))
                        .event("onMoveStart", "moveStart")
                        .event("onMove", "move")
                        .event("onMoveEnd", "moveEnd")
                        .on_key_down("keyDown"),
                ),
            ),
        )
}

fn long_press_move_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("move", NativeRole::View).with_props(
                NativeProps::new()
                    .tab_index(Some(0))
                    .metadata("threshold", "100")
                    .web(
                        WebProps::new()
                            .class_name(
                                "absolute left-[10px] top-[10px] h-[30px] w-[50px] bg-black",
                            )
                            .event("onMoveStart", "moveStart")
                            .event("onMove", "move")
                            .event("onMoveEnd", "moveEnd")
                            .event("onLongPressStart", "longStart")
                            .event("onLongPressEnd", "longEnd")
                            .event("onLongPress", "longPress")
                            .event("onPressEnd", "pressEnd"),
                    ),
            ),
        )
}

fn input(outcome: SelfDrawnHostEventOutcome) -> SelfDrawnInputDispatch {
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("raw input should produce an input dispatch");
    };
    dispatch
}

#[test]
fn pointer_move_starts_on_first_delta_stays_captured_and_ends_on_release() {
    let mut runtime = runtime();
    runtime.render(move_tree("bg-black")).unwrap();
    let target = PlatformElementId::new("4:root/4:move").unwrap();

    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    let stationary = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 20.0, 20.0, 20))
            .unwrap(),
    );
    assert!(stationary.invocations.is_empty());
    assert!(!runtime.element_interaction(&target).unwrap().moving);

    let first = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 24.0, 17.0, 30))
            .unwrap(),
    );
    assert_eq!(
        first
            .invocations
            .iter()
            .map(|invocation| (invocation.event, invocation.context.delta))
            .collect::<Vec<_>>(),
        vec![
            (NativeEventKind::MoveStart, None),
            (NativeEventKind::Move, Some(PlatformPoint::new(4.0, -3.0))),
        ]
    );
    assert!(first
        .invocations
        .iter()
        .all(|invocation| invocation.context.pointer == Some(PlatformPointerId::new(1))));
    assert!(runtime.element_interaction(&target).unwrap().moving);

    let captured = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 90.0, 70.0, 40))
            .unwrap(),
    );
    assert_eq!(captured.target.as_ref(), Some(&target));
    assert_eq!(captured.invocations.len(), 1);
    assert_eq!(captured.invocations[0].event, NativeEventKind::Move);
    assert_eq!(
        captured.invocations[0].context.delta,
        Some(PlatformPoint::new(66.0, 53.0))
    );
    assert!(runtime.element_interaction(&target).unwrap().moving);

    let released = input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                90.0,
                70.0,
                50,
            ))
            .unwrap(),
    );
    assert_eq!(
        released
            .invocations
            .iter()
            .map(|invocation| invocation.event)
            .collect::<Vec<_>>(),
        vec![NativeEventKind::MoveEnd]
    );
    assert!(!runtime.element_interaction(&target).unwrap().moving);
}

#[test]
fn failed_move_reducer_restores_origin_sequence_and_transient_state() {
    let mut runtime = runtime();
    runtime.render(move_tree("bg-black")).unwrap();
    let target = PlatformElementId::new("4:root/4:move").unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();

    let error = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Moved, 24.0, 17.0, 20),
            |_| Err(GuiError::host("injected move reducer failure")),
        )
        .unwrap_err();
    assert!(error.to_string().contains("injected move reducer failure"));
    assert_eq!(runtime.event_sequence(), 1);
    assert!(!runtime.element_interaction(&target).unwrap().moving);
    assert_eq!(runtime.stats().reducer_failures, 1);

    let retried = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Moved, 24.0, 17.0, 30),
            |_| Ok(SelfDrawnActionPropagation::Continue),
        )
        .unwrap();
    let retried = input(retried);
    assert_eq!(retried.event_sequence, 2);
    assert_eq!(
        retried.invocations[1].context.delta,
        Some(PlatformPoint::new(4.0, -3.0))
    );
    assert!(runtime.element_interaction(&target).unwrap().moving);
}

#[test]
fn keyed_render_preserves_active_move_and_its_incremental_origin() {
    let mut runtime = runtime();
    runtime.render(move_tree("bg-black")).unwrap();
    let target = PlatformElementId::new("4:root/4:move").unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 24.0, 17.0, 20))
        .unwrap();

    runtime.render(move_tree("bg-white")).unwrap();
    assert!(runtime.element_interaction(&target).unwrap().moving);
    let moved = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 25.0, 19.0, 30))
            .unwrap(),
    );
    assert_eq!(moved.invocations.len(), 1);
    assert_eq!(moved.invocations[0].event, NativeEventKind::Move);
    assert_eq!(
        moved.invocations[0].context.delta,
        Some(PlatformPoint::new(1.0, 2.0))
    );
}

#[test]
fn arrow_key_emits_complete_handled_one_unit_move_before_key_down() {
    let mut runtime = runtime();
    runtime.render(move_tree("bg-black")).unwrap();
    runtime
        .handle_event(key_event("Tab", PlatformKeyState::Pressed, 10))
        .unwrap();

    let dispatch = input(
        runtime
            .handle_event(key_event("Left", PlatformKeyState::Pressed, 20))
            .unwrap(),
    );
    assert_eq!(
        dispatch
            .invocations
            .iter()
            .map(|invocation| invocation.event)
            .collect::<Vec<_>>(),
        vec![
            NativeEventKind::MoveStart,
            NativeEventKind::Move,
            NativeEventKind::MoveEnd,
            NativeEventKind::KeyDown,
        ]
    );
    assert_eq!(dispatch.invocations[0].context.delta, None);
    assert_eq!(
        dispatch.invocations[1].context.delta,
        Some(PlatformPoint::new(-1.0, 0.0))
    );
    assert_eq!(dispatch.invocations[2].context.delta, None);
    assert!(dispatch
        .invocations
        .iter()
        .all(|invocation| invocation.context.handled_activation));
    assert!(
        !runtime
            .element_interaction(dispatch.target.as_ref().unwrap())
            .unwrap()
            .moving
    );
}

#[test]
fn style_only_moving_variant_is_a_pointer_target_without_an_action() {
    let root = NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("move", NativeRole::View)
                .with_props(NativeProps::new().web(WebProps::new().class_name(
                "absolute left-[10px] top-[10px] h-[30px] w-[50px] data-[moving=true]:opacity-60",
            ))),
        );
    let mut runtime = runtime();
    runtime.render(root).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    let dispatch = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 23.0, 25.0, 20))
            .unwrap(),
    );
    let target = PlatformElementId::new("4:root/4:move").unwrap();
    assert_eq!(dispatch.target.as_ref(), Some(&target));
    assert!(dispatch.invocations.is_empty());
    assert!(runtime.element_interaction(&target).unwrap().moving);
}

#[test]
fn long_press_recognition_ends_active_move_before_terminal_action() {
    let mut runtime = runtime();
    runtime.render(long_press_move_tree()).unwrap();
    let target = PlatformElementId::new("4:root/4:move").unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 24.0, 20.0, 20))
        .unwrap();

    let dispatch = runtime.advance_interaction_time(100_010).unwrap().unwrap();
    assert_eq!(
        dispatch
            .invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.event))
            .collect::<Vec<_>>(),
        vec![
            ("longEnd", NativeEventKind::LongPressEnd),
            ("pressEnd", NativeEventKind::PressCancel),
            ("moveEnd", NativeEventKind::MoveEnd),
            ("longPress", NativeEventKind::LongPress),
        ]
    );
    assert!(!runtime.element_interaction(&target).unwrap().moving);
}

#[test]
fn concurrent_pointer_moves_keep_shared_moving_state_until_the_last_end() {
    let mut runtime = runtime();
    runtime.render(move_tree("bg-black")).unwrap();
    let target = PlatformElementId::new("4:root/4:move").unwrap();
    for pointer in [1, 2] {
        runtime
            .handle_event(pointer_event_for(
                PlatformPointerId::new(pointer),
                PlatformPointerPhase::Pressed,
                20.0,
                20.0,
                pointer * 10,
            ))
            .unwrap();
        let moved = input(
            runtime
                .handle_event(pointer_event_for(
                    PlatformPointerId::new(pointer),
                    PlatformPointerPhase::Moved,
                    20.0 + pointer as f64,
                    20.0,
                    pointer * 10 + 1,
                ))
                .unwrap(),
        );
        assert!(moved
            .invocations
            .iter()
            .all(|invocation| invocation.context.pointer == Some(PlatformPointerId::new(pointer))));
    }
    assert!(runtime.element_interaction(&target).unwrap().moving);

    runtime
        .handle_event(pointer_event_for(
            PlatformPointerId::new(1),
            PlatformPointerPhase::Released,
            21.0,
            20.0,
            40,
        ))
        .unwrap();
    assert!(runtime.element_interaction(&target).unwrap().moving);
    runtime
        .handle_event(pointer_event_for(
            PlatformPointerId::new(2),
            PlatformPointerPhase::Cancelled,
            22.0,
            20.0,
            50,
        ))
        .unwrap();
    assert!(!runtime.element_interaction(&target).unwrap().moving);
}
