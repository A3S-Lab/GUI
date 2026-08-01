use crate::platform_host::{PlatformElementId, PlatformKeyState, PlatformPointerPhase};
use crate::NativeEventKind;

use super::drag_drop_tests::{
    action_events, drag_drop_tree, input, style_only_tree, SOURCE_ID, TARGET_A_ID, TARGET_B_ID,
};
use super::tests::{key_event, pointer_event, runtime};
use super::SelfDrawnDropOperation;

#[test]
fn keyboard_drag_tabs_only_to_compatible_target_and_drops_with_enter() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree(
            "application/pdf,text/plain",
            "image/*",
            "application/*,image/png",
            "bg-black",
        ))
        .unwrap();
    runtime
        .handle_event(key_event("Tab", PlatformKeyState::Pressed, 10))
        .unwrap();

    let started = input(
        runtime
            .handle_event(key_event("Enter", PlatformKeyState::Pressed, 20))
            .unwrap(),
    );
    assert_eq!(
        action_events(&started),
        vec![
            ("dragStart", NativeEventKind::DragStart),
            ("sourceKeyDown", NativeEventKind::KeyDown),
        ]
    );
    let entered = input(
        runtime
            .handle_event(key_event("Tab", PlatformKeyState::Pressed, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&entered),
        vec![("enterb", NativeEventKind::DropEnter)]
    );
    assert_eq!(
        runtime.focused_element().map(|id| id.as_str()),
        Some(TARGET_B_ID)
    );

    let dropped = input(
        runtime
            .handle_event(key_event("Enter", PlatformKeyState::Pressed, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&dropped),
        vec![
            ("dropb", NativeEventKind::Drop),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_eq!(
        dropped.invocations[0]
            .context
            .drag
            .as_ref()
            .unwrap()
            .drop_operation,
        SelfDrawnDropOperation::Copy
    );
}

#[test]
fn keyboard_escape_exits_active_target_and_cancels_drag() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    for (key, timestamp) in [("Tab", 10), ("Enter", 20), ("Tab", 30)] {
        runtime
            .handle_event(key_event(key, PlatformKeyState::Pressed, timestamp))
            .unwrap();
    }
    let canceled = input(
        runtime
            .handle_event(key_event("Escape", PlatformKeyState::Pressed, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&canceled),
        vec![
            ("exita", NativeEventKind::DropExit),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_eq!(
        canceled
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .drop_operation,
        SelfDrawnDropOperation::Cancel
    );
}

#[test]
fn keyboard_escape_cancels_an_active_pointer_drag() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    for (phase, x, timestamp) in [
        (PlatformPointerPhase::Pressed, 10.0, 10),
        (PlatformPointerPhase::Moved, 15.0, 20),
        (PlatformPointerPhase::Moved, 45.0, 30),
    ] {
        runtime
            .handle_event(pointer_event(phase, x, 20.0, timestamp))
            .unwrap();
    }

    let canceled = input(
        runtime
            .handle_event(key_event("Escape", PlatformKeyState::Pressed, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&canceled),
        vec![
            ("exita", NativeEventKind::DropExit),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_eq!(
        canceled
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .drop_operation,
        SelfDrawnDropOperation::Cancel
    );
    assert!(
        !runtime
            .element_interaction(&PlatformElementId::new(SOURCE_ID).unwrap())
            .unwrap()
            .dragging
    );
}

#[test]
fn style_only_drag_source_and_drop_target_update_without_actions() {
    let mut runtime = runtime();
    runtime.render(style_only_tree()).unwrap();
    let source = PlatformElementId::new(SOURCE_ID).unwrap();
    let target = PlatformElementId::new(TARGET_A_ID).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();
    let started = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20))
            .unwrap(),
    );
    assert!(started.invocations.is_empty());
    assert!(runtime.element_interaction(&source).unwrap().dragging);
    let entered = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
            .unwrap(),
    );
    assert!(entered.invocations.is_empty());
    assert!(runtime.element_interaction(&target).unwrap().drop_target);
}
