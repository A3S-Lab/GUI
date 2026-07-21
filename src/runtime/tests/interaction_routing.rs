use super::super::*;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

#[test]
fn runtime_updates_interaction_state_before_dispatching_action() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
    )
    .unwrap();
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEmail");

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let invocation = runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("a@b.c"),
        )
        .unwrap();

    assert_eq!(invocation.action, "setEmail");
    assert_eq!(
        runtime
            .interactions()
            .node(root_id)
            .unwrap()
            .value
            .as_deref(),
        Some("a@b.c")
    );
}

#[test]
fn runtime_accessibility_tree_reflects_interaction_state() {
    let tree = NativeElement::new("settings", NativeRole::Form)
        .child(
            NativeElement::new("email", NativeRole::TextField).with_props(
                NativeProps::new()
                    .label("Email")
                    .value("old@example.com")
                    .web(WebProps::new().on_change("setEmail")),
            ),
        )
        .child(
            NativeElement::new("notifications", NativeRole::Switch).with_props(
                NativeProps::new()
                    .label("Notifications")
                    .checked(false)
                    .web(WebProps::new().on_change("setNotifications")),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEmail");
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&tree).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    let email = children[0];
    let notifications = children[1];

    runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(email, crate::event::NativeEventKind::Change)
                .value("new@example.com"),
        )
        .unwrap();
    runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(notifications, crate::event::NativeEventKind::Toggle)
                .value("true"),
        )
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.children[0].node, Some(email));
    assert_eq!(
        accessibility.children[0].value.as_deref(),
        Some("new@example.com")
    );
    assert_eq!(accessibility.children[1].node, Some(notifications));
    assert_eq!(accessibility.children[1].checked, Some(true));
}

#[test]
fn runtime_routes_expanded_toggle_with_current_boolean_payload() {
    let element = NativeElement::new("details", NativeRole::Disclosure).with_props(
        NativeProps::new()
            .label("Details")
            .expanded(false)
            .web(WebProps::new().on_expanded_change("setOpen")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setOpen");

    let root_id = runtime.render_native(&element).unwrap();
    let first = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                .value("on"),
        )
        .unwrap();
    let second = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                .value("not-a-bool"),
        )
        .unwrap();

    assert_eq!(first.event.value.as_deref(), Some("true"));
    assert_eq!(
        first
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("true")
    );
    assert_eq!(first.interaction_changes[0].after.expanded, Some(true));
    assert_eq!(second.event.value.as_deref(), Some("false"));
    assert_eq!(
        second
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("false")
    );
    assert_eq!(second.interaction_changes[0].after.expanded, Some(false));
    assert_eq!(runtime.accessibility_tree().unwrap().expanded, Some(false));
}

#[test]
fn runtime_routes_checked_toggle_with_current_boolean_payload() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .checked(false)
            .web(WebProps::new().on_change("setNotifications")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let first = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                .value("1"),
        )
        .unwrap();
    let second = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                .value("not-a-bool"),
        )
        .unwrap();

    assert_eq!(first.event.value.as_deref(), Some("true"));
    assert_eq!(
        first
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("true")
    );
    assert_eq!(first.interaction_changes[0].after.checked, Some(true));
    assert_eq!(second.event.value.as_deref(), Some("false"));
    assert_eq!(
        second
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("false")
    );
    assert_eq!(second.interaction_changes[0].after.checked, Some(false));
}

#[test]
fn runtime_routes_checked_change_with_current_boolean_payload() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .checked(false)
            .web(WebProps::new().on_change("setNotifications")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let first = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("on"),
        )
        .unwrap();
    let second = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("not-a-bool"),
        )
        .unwrap();

    assert_eq!(first.event.value.as_deref(), Some("true"));
    assert_eq!(
        first
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("true")
    );
    assert_eq!(first.interaction_changes[0].after.checked, Some(true));
    assert_eq!(second.event.value.as_deref(), Some("false"));
    assert_eq!(
        second
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("false")
    );
    assert_eq!(second.interaction_changes[0].after.checked, Some(false));
    assert_eq!(
        runtime.actions().invocations()[1].value.as_deref(),
        Some("false")
    );
}

#[test]
fn runtime_routes_switch_space_key_to_toggle_action() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .checked(false)
            .web(WebProps::new().on_change("setNotifications")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value(" "),
        )
        .unwrap();

    assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
    assert_eq!(handled.event.value.as_deref(), Some("true"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.event),
        Some(crate::event::NativeEventKind::Toggle)
    );
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("true")
    );
    assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
}

#[test]
fn runtime_explicit_key_down_prevents_keyboard_toggle_normalization() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .checked(false)
            .web(
                WebProps::new()
                    .on_change("setNotifications")
                    .on_key_down("handleKeyDown"),
            ),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");
    runtime.actions_mut().register("handleKeyDown");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value("space"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, crate::event::NativeEventKind::KeyDown);
    assert_eq!(handled.event.value.as_deref(), Some(" "));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("handleKeyDown")
    );
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some(" ")
    );
    assert!(handled.interaction_changes.is_empty());
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
}

#[test]
fn runtime_empty_key_down_handler_does_not_block_keyboard_toggle_normalization() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .checked(false)
            .web(
                WebProps::new()
                    .on_change("setNotifications")
                    .on_key_down(""),
            ),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value(" "),
        )
        .unwrap();

    assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("setNotifications")
    );
    assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
}

#[test]
fn runtime_ancestor_key_down_prevents_keyboard_toggle_normalization() {
    let element = NativeElement::new("row", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().on_key_down("handleRowKey")))
        .child(
            NativeElement::new("notifications", NativeRole::Switch).with_props(
                NativeProps::new()
                    .label("Notifications")
                    .checked(false)
                    .web(WebProps::new().on_change("setNotifications")),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("handleRowKey");
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let switch = runtime.host().node(root_id).unwrap().children[0];
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(switch, crate::event::NativeEventKind::KeyDown)
                .value(" "),
        )
        .unwrap();

    assert_eq!(handled.event.kind, crate::event::NativeEventKind::KeyDown);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("handleRowKey")
    );
    assert!(handled.interaction_changes.is_empty());
    assert_eq!(
        runtime.accessibility_tree().unwrap().children[0].checked,
        Some(false)
    );
}

#[test]
fn runtime_routes_radio_space_key_to_selection_action() {
    let element = NativeElement::new("dark", NativeRole::Radio).with_props(
        NativeProps::new()
            .label("Dark")
            .value("dark")
            .web(WebProps::new().on_change("setTheme")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTheme");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value("Space"),
        )
        .unwrap();

    assert_eq!(
        handled.event.kind,
        crate::event::NativeEventKind::SelectionChange
    );
    assert_eq!(handled.event.value.as_deref(), Some("dark"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.event),
        Some(crate::event::NativeEventKind::SelectionChange)
    );
    assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
    assert!(handled.interaction_changes[0].after.selected);
}

#[test]
fn runtime_infers_container_selection_value_from_selected_child() {
    let element = NativeElement::new("theme", NativeRole::Select)
        .with_props(
            NativeProps::new()
                .label("Theme")
                .web(WebProps::new().on_selection_change("setTheme")),
        )
        .child(
            NativeElement::new("compact", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Compact").value("compact")),
        )
        .child(
            NativeElement::new("comfortable", NativeRole::ListBoxItem).with_props(
                NativeProps::new()
                    .label("Comfortable")
                    .value("comfortable")
                    .selected(true),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTheme");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value(" "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("comfortable"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("comfortable")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("comfortable")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("comfortable")
    );
}

#[test]
fn runtime_infers_selectable_node_value_from_empty_selection_payload() {
    let element = NativeElement::new("compact", NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .label("Compact")
            .value("compact")
            .web(WebProps::new().on_selection_change("setTheme")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTheme");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value(""),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("compact"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("compact")
    );
    assert!(handled.interaction_changes[0].after.selected);
}
