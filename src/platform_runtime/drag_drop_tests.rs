use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{PlatformElementId, PlatformPointerPhase};
use crate::web::WebProps;
use crate::{GuiError, NativeEventKind, PlatformPoint};

use super::tests::{pointer_event, runtime};
use super::{
    SelfDrawnActionPropagation, SelfDrawnDropOperation, SelfDrawnHostEventOutcome,
    SelfDrawnInputDispatch,
};

pub(super) const SOURCE_ID: &str = "4:root/6:source";
pub(super) const TARGET_A_ID: &str = "4:root/8:target-a";
pub(super) const TARGET_B_ID: &str = "4:root/8:target-b";

pub(super) fn drag_drop_tree(
    source_types: &str,
    target_a_types: &str,
    target_b_types: &str,
    source_color: &str,
) -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("source", NativeRole::View).with_props(
                NativeProps::new()
                    .tab_index(Some(0))
                    .draggable("true")
                    .web(
                        WebProps::new()
                            .class_name(format!(
                                "absolute left-[5px] top-[10px] h-[30px] w-[25px] {source_color} data-[dragging=true]:opacity-60"
                            ))
                            .attribute("data-drag-type", source_types)
                            .attribute("data-drag-value", "alpha")
                            .attribute("data-allowed-drop-operations", "copy,move,link")
                            .event("onDragStart", "dragStart")
                            .event("onDragMove", "dragMove")
                            .event("onDragEnd", "dragEnd")
                            .on_key_down("sourceKeyDown"),
                    ),
            ),
        )
        .child(drop_target(
            "target-a",
            40,
            target_a_types,
            "move",
            "a",
            -1,
        ))
        .child(drop_target(
            "target-b",
            70,
            target_b_types,
            "copy",
            "b",
            -1,
        ))
}

fn drop_target(
    key: &str,
    left: u32,
    accepted_types: &str,
    operation: &str,
    action_suffix: &str,
    tab_index: i32,
) -> NativeElement {
    NativeElement::new(key, NativeRole::View).with_props(
        NativeProps::new().tab_index(Some(tab_index)).web(
            WebProps::new()
                .class_name(format!(
                    "absolute left-[{left}px] top-[10px] h-[30px] w-[25px] bg-white data-[drop-target=true]:opacity-60"
                ))
                .attribute("data-accepted-drag-types", accepted_types)
                .attribute("data-drop-operation", operation)
                .event("onDropEnter", format!("enter{action_suffix}"))
                .event("onDropMove", format!("move{action_suffix}"))
                .event("onDropActivate", format!("activate{action_suffix}"))
                .event("onDropExit", format!("exit{action_suffix}"))
                .event("onDrop", format!("drop{action_suffix}")),
        ),
    )
}

pub(super) fn style_only_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("source", NativeRole::View).with_props(
                NativeProps::new().draggable("true").web(
                    WebProps::new()
                        .class_name(
                            "absolute left-[5px] top-[10px] h-[30px] w-[25px] bg-black data-[dragging=true]:opacity-60",
                        )
                        .attribute("data-drag-type", "text/plain")
                        .attribute("data-drag-value", "style-value"),
                ),
            ),
        )
        .child(
            NativeElement::new("target-a", NativeRole::View).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .class_name(
                            "absolute left-[40px] top-[10px] h-[30px] w-[25px] bg-white data-[drop-target=true]:opacity-60",
                        )
                        .attribute("data-accepted-drag-types", "text/*"),
                ),
            ),
        )
}

pub(super) fn input(outcome: SelfDrawnHostEventOutcome) -> SelfDrawnInputDispatch {
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("raw input should produce an input dispatch");
    };
    dispatch
}

pub(super) fn action_events(dispatch: &SelfDrawnInputDispatch) -> Vec<(&str, NativeEventKind)> {
    dispatch
        .invocations
        .iter()
        .map(|invocation| (invocation.action.as_str(), invocation.event))
        .collect()
}

#[test]
fn pointer_drag_negotiates_wildcard_target_and_reports_local_drop_context() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree(
            "image/png,text/plain",
            "image/*,application/json",
            "text/plain",
            "bg-black",
        ))
        .unwrap();
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
    assert_eq!(
        action_events(&started),
        vec![
            ("dragStart", NativeEventKind::DragStart),
            ("dragMove", NativeEventKind::DragMove),
        ]
    );
    assert!(runtime.element_interaction(&source).unwrap().dragging);

    let entered = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&entered),
        vec![
            ("dragMove", NativeEventKind::DragMove),
            ("entera", NativeEventKind::DropEnter),
            ("movea", NativeEventKind::DropMove),
        ]
    );
    let drop_move = entered.invocations.last().unwrap();
    assert_eq!(drop_move.node, target);
    assert_eq!(
        drop_move.context.position,
        Some(PlatformPoint::new(5.0, 10.0))
    );
    let drag = drop_move.context.drag.as_ref().unwrap();
    assert_eq!(drag.types, ["image/png", "text/plain"]);
    assert_eq!(drag.value.as_deref(), Some("alpha"));
    assert_eq!(drag.items.len(), 1);
    assert_eq!(drag.items[0].get_text("image/png"), Some("alpha"));
    assert_eq!(drag.items[0].get_text("text/plain"), Some("alpha"));
    assert_eq!(drag.drop_operation, SelfDrawnDropOperation::Move);
    assert_eq!(drop_move.value(), Some("alpha"));
    assert!(runtime.element_interaction(&target).unwrap().drop_target);

    let dropped = input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                45.0,
                20.0,
                40,
            ))
            .unwrap(),
    );
    assert_eq!(
        action_events(&dropped),
        vec![
            ("dropa", NativeEventKind::Drop),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_eq!(
        dropped
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .drop_operation,
        SelfDrawnDropOperation::Move
    );
    assert!(!runtime.element_interaction(&source).unwrap().dragging);
    assert!(!runtime.element_interaction(&target).unwrap().drop_target);
}

#[test]
fn incompatible_types_never_activate_or_drop_and_end_with_cancel() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree(
            "application/pdf",
            "image/*,text/plain",
            "application/json",
            "bg-black",
        ))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20))
        .unwrap();
    let rejected = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&rejected),
        vec![("dragMove", NativeEventKind::DragMove)]
    );
    assert!(
        !runtime
            .element_interaction(&PlatformElementId::new(TARGET_A_ID).unwrap())
            .unwrap()
            .drop_target
    );

    let ended = input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                45.0,
                20.0,
                40,
            ))
            .unwrap(),
    );
    assert_eq!(
        action_events(&ended),
        vec![("dragEnd", NativeEventKind::DragEnd)]
    );
    assert_eq!(
        ended.invocations[0]
            .context
            .drag
            .as_ref()
            .unwrap()
            .drop_operation,
        SelfDrawnDropOperation::Cancel
    );
}

#[test]
fn moving_between_targets_orders_exit_before_enter_and_move() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "text/*", "bg-black"))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
        .unwrap();

    let transitioned = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 75.0, 20.0, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&transitioned),
        vec![
            ("dragMove", NativeEventKind::DragMove),
            ("exita", NativeEventKind::DropExit),
            ("enterb", NativeEventKind::DropEnter),
            ("moveb", NativeEventKind::DropMove),
        ]
    );
    assert!(
        !runtime
            .element_interaction(&PlatformElementId::new(TARGET_A_ID).unwrap())
            .unwrap()
            .drop_target
    );
    assert!(
        runtime
            .element_interaction(&PlatformElementId::new(TARGET_B_ID).unwrap())
            .unwrap()
            .drop_target
    );
}

#[test]
fn pointer_cancel_exits_target_then_ends_source_with_cancel_operation() {
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
            .handle_event(pointer_event(
                PlatformPointerPhase::Cancelled,
                45.0,
                20.0,
                40,
            ))
            .unwrap(),
    );
    assert_eq!(
        action_events(&canceled),
        vec![
            ("exita", NativeEventKind::DropExit),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert!(canceled.invocations.iter().all(|invocation| {
        invocation.context.drag.as_ref().unwrap().drop_operation == SelfDrawnDropOperation::Cancel
    }));
}

#[test]
fn failed_drag_reducer_restores_candidate_origin_sequence_and_state() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    let source = PlatformElementId::new(SOURCE_ID).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();

    let error = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20),
            |_| Err(GuiError::host("injected drag reducer failure")),
        )
        .unwrap_err();
    assert!(error.to_string().contains("injected drag reducer failure"));
    assert_eq!(runtime.event_sequence(), 1);
    assert!(!runtime.element_interaction(&source).unwrap().dragging);

    let retried = input(
        runtime
            .handle_event_with_reducer(
                pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 30),
                |_| Ok(SelfDrawnActionPropagation::Continue),
            )
            .unwrap(),
    );
    assert_eq!(retried.event_sequence, 2);
    assert_eq!(
        retried.invocations[1].context.delta,
        Some(PlatformPoint::new(5.0, 0.0))
    );
    assert!(runtime.element_interaction(&source).unwrap().dragging);
}

#[test]
fn keyed_render_preserves_active_drag_target_and_incremental_position() {
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
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-white"))
        .unwrap();
    let source = PlatformElementId::new(SOURCE_ID).unwrap();
    let target = PlatformElementId::new(TARGET_A_ID).unwrap();
    assert!(runtime.element_interaction(&source).unwrap().dragging);
    assert!(runtime.element_interaction(&target).unwrap().drop_target);

    let moved = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 46.0, 22.0, 40))
            .unwrap(),
    );
    assert_eq!(moved.invocations[0].event, NativeEventKind::DragMove);
    assert_eq!(
        moved.invocations[0].context.delta,
        Some(PlatformPoint::new(1.0, 2.0))
    );
    assert_eq!(
        moved.invocations.last().unwrap().event,
        NativeEventKind::DropMove
    );
}
