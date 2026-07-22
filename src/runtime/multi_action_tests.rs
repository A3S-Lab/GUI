use super::*;
use crate::input::NativeInputModality;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

fn bubbling_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().on_hover_change("parentHoverChange")))
        .child(
            NativeElement::new("target", NativeRole::Button).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .on_hover_start("targetHoverStart")
                        .on_hover_change("targetHoverChange"),
                ),
            ),
        )
}

#[test]
fn native_event_dispatches_target_callbacks_then_bubbles_nearest_first() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    for action in ["targetHoverStart", "targetHoverChange", "parentHoverChange"] {
        runtime.actions_mut().register(action);
    }
    let root = runtime.render_native(&bubbling_tree()).unwrap();
    let target = runtime.renderer.child_ids(root)[0];

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(target, crate::event::NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        )
        .unwrap();

    assert_eq!(
        handled
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["targetHoverStart", "targetHoverChange", "parentHoverChange"]
    );
    assert!(handled
        .invocations
        .iter()
        .all(|invocation| invocation.node == target));
    assert_eq!(handled.invocations[0].current_target(), target);
    assert_eq!(handled.invocations[1].current_target(), target);
    assert_eq!(handled.invocations[2].current_target(), root);
    assert_eq!(handled.invocation, handled.invocations.first().cloned());
    assert!(runtime.interactions().node(target).unwrap().hovered);
}

#[test]
fn close_events_are_scoped_to_the_native_target() {
    let tree = NativeElement::new("window", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().event("onCloseRequest", "closeWindow")))
        .child(
            NativeElement::new("dialog", NativeRole::Dialog).with_props(
                NativeProps::new().web(WebProps::new().event("onClose", "closeDialog")),
            ),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    for action in ["closeDialog", "closeWindow"] {
        runtime.actions_mut().register(action);
    }
    let root = runtime.render_native(&tree).unwrap();
    let dialog = runtime.renderer.child_ids(root)[0];

    let handled = runtime
        .handle_native_event_with_changes(NativeEvent::new(
            dialog,
            crate::event::NativeEventKind::Close,
        ))
        .unwrap();

    assert_eq!(
        handled
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["closeDialog"]
    );
}

#[test]
fn press_change_callbacks_receive_normalized_lifecycle_values() {
    let tree = NativeElement::new("target", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_press_start("start")
                .on_press_end("end")
                .on_press_change("change"),
        ),
    );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    for action in ["start", "end", "change"] {
        runtime.actions_mut().register(action);
    }
    let target = runtime.render_native(&tree).unwrap();

    let start = runtime
        .handle_native_event_with_changes(NativeEvent::new(
            target,
            crate::event::NativeEventKind::PressStart,
        ))
        .unwrap();
    assert_eq!(
        start
            .invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.value.as_deref()))
            .collect::<Vec<_>>(),
        vec![("start", Some("true")), ("change", Some("true"))]
    );

    let end = runtime
        .handle_native_event_with_changes(NativeEvent::new(
            target,
            crate::event::NativeEventKind::PressCancel,
        ))
        .unwrap();
    assert_eq!(
        end.invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.value.as_deref()))
            .collect::<Vec<_>>(),
        vec![("end", Some("false")), ("change", Some("false"))]
    );
}
