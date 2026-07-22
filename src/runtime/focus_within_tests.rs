use super::*;
use crate::event::NativeEventKind;
use crate::input::NativeEventContext;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

fn focus_tree() -> NativeElement {
    NativeElement::new("group", NativeRole::View)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .event("onFocus", "parentFocus")
                    .event("onFocusWithin", "focusWithin")
                    .event("onBlurWithin", "blurWithin")
                    .event("onFocusWithinChange", "focusWithinChange"),
            ),
        )
        .child(
            NativeElement::new("first", NativeRole::Button).with_props(
                NativeProps::new().label("First").web(
                    WebProps::new()
                        .event("onFocus", "firstFocus")
                        .event("onBlur", "firstBlur"),
                ),
            ),
        )
        .child(
            NativeElement::new("second", NativeRole::Button).with_props(
                NativeProps::new().label("Second").web(
                    WebProps::new()
                        .event("onFocus", "secondFocus")
                        .event("onBlur", "secondBlur"),
                ),
            ),
        )
}

fn runtime() -> GuiRuntime<PlatformPlanningHost<Gtk4Adapter>> {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    for action in [
        "parentFocus",
        "focusWithin",
        "blurWithin",
        "focusWithinChange",
        "firstFocus",
        "firstBlur",
        "secondFocus",
        "secondBlur",
    ] {
        runtime.actions_mut().register(action);
    }
    runtime
}

#[test]
fn direct_focus_callbacks_do_not_bubble_to_ancestors() {
    let mut runtime = runtime();
    let root = runtime.render_native(&focus_tree()).unwrap();
    let first = runtime.host().node(root).unwrap().children[0];

    let handled = runtime
        .handle_native_event_with_changes(NativeEvent::new(first, NativeEventKind::Focus))
        .unwrap();

    assert_eq!(
        handled
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        ["firstFocus", "focusWithin", "focusWithinChange"]
    );
    assert!(!handled
        .invocations
        .iter()
        .any(|invocation| invocation.action == "parentFocus"));
}

#[test]
fn focus_within_fires_only_when_focus_crosses_the_subtree_boundary() {
    let mut runtime = runtime();
    let root = runtime.render_native(&focus_tree()).unwrap();
    let children = runtime.host().node(root).unwrap().children.clone();
    let first = children[0];
    let second = children[1];

    let entered = runtime
        .handle_native_event_with_changes(NativeEvent::new(first, NativeEventKind::Focus))
        .unwrap();
    assert_eq!(
        entered
            .invocations
            .iter()
            .map(|invocation| (
                invocation.action.as_str(),
                invocation.current_target(),
                invocation.value.as_deref(),
            ))
            .collect::<Vec<_>>(),
        [
            ("firstFocus", first, Some("true")),
            ("focusWithin", root, Some("true")),
            ("focusWithinChange", root, Some("true")),
        ]
    );
    assert!(runtime.interactions().node(root).unwrap().focus_within);

    let blurred = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(first, NativeEventKind::Blur)
                .context(NativeEventContext::new().related_target(second)),
        )
        .unwrap();
    assert_eq!(
        blurred
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        ["firstBlur"]
    );

    let focused = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(second, NativeEventKind::Focus)
                .context(NativeEventContext::new().related_target(first)),
        )
        .unwrap();
    assert_eq!(
        focused
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        ["secondFocus"]
    );
    assert!(runtime.interactions().node(root).unwrap().focus_within);

    let exited = runtime
        .handle_native_event_with_changes(NativeEvent::new(second, NativeEventKind::Blur))
        .unwrap();
    assert_eq!(
        exited
            .invocations
            .iter()
            .map(|invocation| (
                invocation.action.as_str(),
                invocation.current_target(),
                invocation.value.as_deref(),
            ))
            .collect::<Vec<_>>(),
        [
            ("secondBlur", second, Some("false")),
            ("blurWithin", root, Some("false")),
            ("focusWithinChange", root, Some("false")),
        ]
    );
    assert!(!runtime.interactions().node(root).unwrap().focus_within);

    let reentered = runtime
        .handle_native_event_with_changes(NativeEvent::new(second, NativeEventKind::Focus))
        .unwrap();
    assert_eq!(
        reentered
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        ["secondFocus", "focusWithin", "focusWithinChange"]
    );
    assert!(runtime.interactions().node(root).unwrap().focus_within);
}

#[test]
fn focus_within_action_failure_rolls_back_focus_state() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("firstFocus");
    let root = runtime.render_native(&focus_tree()).unwrap();
    let first = runtime.host().node(root).unwrap().children[0];

    let error = runtime
        .handle_native_event_with_changes(NativeEvent::new(first, NativeEventKind::Focus))
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("unregistered action focusWithin"));
    assert_eq!(runtime.focus_owner, None);
    assert!(runtime.interactions().focused_node().is_none());
    assert!(runtime.interactions().node(root).is_none());
}
