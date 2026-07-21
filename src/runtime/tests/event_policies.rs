use super::super::*;
use crate::html::HtmlDialogProps;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

#[test]
fn runtime_dispatch_stays_strict_for_unbound_events() {
    let element =
        NativeElement::new("save", NativeRole::Button).with_props(NativeProps::new().label("Save"));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let error = runtime
        .dispatch_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap_err();

    assert!(error.to_string().contains("no registered RSX action"));
    assert!(runtime.interactions().node(root_id).unwrap().focused);
}

#[test]
fn runtime_rolls_back_interactions_after_unregistered_action() {
    let element = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Name")
            .value("Ada")
            .web(WebProps::new().on_change("setName")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    assert!(runtime.interactions().node(root_id).unwrap().focused);
    let error = runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("Grace"),
        )
        .unwrap_err();

    assert!(error.to_string().contains("unregistered action setName"));
    let state = runtime.interactions().node(root_id).unwrap();
    assert!(state.focused);
    assert_eq!(state.value.as_deref(), Some("Ada"));
    assert_eq!(runtime.interactions().changes().len(), 1);
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("Ada")
    );
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_treats_empty_action_ids_as_unbound_events() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .web(WebProps::new().on_press("")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let error = runtime
        .dispatch_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap_err();

    assert!(error.to_string().contains("no registered RSX action"));
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_suppresses_disabled_press_actions() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .disabled(true)
            .web(WebProps::new().on_press("saveDocument")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();
    let error = runtime
        .dispatch_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap_err();

    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert!(runtime.actions().invocations().is_empty());
    assert!(error.to_string().contains("no registered RSX action"));
}

#[test]
fn runtime_suppresses_disabled_toggle_state_changes() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .disabled(true)
            .checked(false)
            .web(WebProps::new().on_change("setNotifications")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let toggle = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Toggle,
        ))
        .unwrap();
    let key = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value(" "),
        )
        .unwrap();

    assert!(toggle.invocation.is_none());
    assert!(toggle.interaction_changes.is_empty());
    assert_eq!(toggle.event.value, None);
    assert!(key.invocation.is_none());
    assert_eq!(key.event.kind, crate::event::NativeEventKind::KeyDown);
    assert!(key.interaction_changes.is_empty());
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_suppresses_disabled_ancestor_user_events() {
    let element = NativeElement::new("review-gate", NativeRole::FieldSet)
        .with_props(NativeProps::new().label("Review gate").disabled(true))
        .child(
            NativeElement::new("finish-review", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Complete review")
                    .web(WebProps::new().on_press("finishReview")),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("finishReview");

    let root_id = runtime.render_native(&element).unwrap();
    let button_id = runtime.host().node(root_id).unwrap().children[0];
    let press = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            button_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();
    let key = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(button_id, crate::event::NativeEventKind::KeyDown)
                .value("Enter"),
        )
        .unwrap();
    let focus = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            button_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();

    assert!(press.invocation.is_none());
    assert!(press.interaction_changes.is_empty());
    assert!(key.invocation.is_none());
    assert_eq!(key.event.kind, crate::event::NativeEventKind::KeyDown);
    assert!(key.interaction_changes.is_empty());
    assert!(focus.invocation.is_none());
    assert_eq!(focus.interaction_changes.len(), 1);
    assert!(runtime.interactions().node(button_id).unwrap().focused);
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_allows_disabled_focus_state_changes() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .disabled(true)
            .web(WebProps::new().on_focus("inspectSave")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("inspectSave");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();

    assert!(handled.invocation.is_some());
    assert_eq!(handled.interaction_changes.len(), 1);
    assert!(runtime.interactions().node(root_id).unwrap().focused);
    assert_eq!(runtime.actions().invocations().len(), 1);
}

#[test]
fn runtime_suppresses_invisible_focus_and_actions() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new().label("Save").hidden(true).web(
            WebProps::new()
                .on_focus("inspectSave")
                .on_press("saveDocument"),
        ),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("inspectSave");
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&element).unwrap();
    let focus = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    let press = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();

    assert!(focus.invocation.is_none());
    assert!(focus.interaction_changes.is_empty());
    assert!(press.invocation.is_none());
    assert!(press.interaction_changes.is_empty());
    assert!(runtime.accessibility_tree().is_none());
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_suppresses_non_rendered_style_actions() {
    let cases = [
        ("display", "none"),
        ("visibility", "hidden"),
        ("visibility", "collapse"),
        ("contentVisibility", "hidden"),
    ];

    for (property, value) in cases {
        let element = NativeElement::new(format!("{property}-{value}"), NativeRole::Button)
            .with_props(
                NativeProps::new().label("Save").web(
                    WebProps::new()
                        .style(property, value)
                        .on_press("saveDocument"),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert!(
            handled.invocation.is_none(),
            "{property}: {value} should suppress invocation"
        );
        assert!(
            handled.interaction_changes.is_empty(),
            "{property}: {value} should suppress interaction changes"
        );
        assert!(
            runtime.accessibility_tree().is_none(),
            "{property}: {value} should suppress accessibility projection"
        );
        assert!(
            runtime.actions().invocations().is_empty(),
            "{property}: {value} should suppress action dispatch"
        );
    }
}

#[test]
fn runtime_suppresses_closed_dialog_subtree_actions() {
    let element = NativeElement::new("dialog", NativeRole::Dialog)
        .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)))
        .child(
            NativeElement::new("save", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Save")
                    .web(WebProps::new().on_press("saveDocument")),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&element).unwrap();
    let child = runtime.host().node(root_id).unwrap().children[0];
    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            child,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();

    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert!(runtime.accessibility_tree().is_none());
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_accessibility_tree_prunes_invisible_inert_and_aria_hidden_subtrees() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        )
        .child(
            NativeElement::new("archive", NativeRole::Button)
                .with_props(NativeProps::new().label("Archive").hidden(true)),
        )
        .child(
            NativeElement::new("delete", NativeRole::Button)
                .with_props(NativeProps::new().label("Delete").inert(true)),
        )
        .child(
            NativeElement::new("preview", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Preview")
                    .accessibility_hidden(Some(true)),
            ),
        )
        .child(
            NativeElement::new("details", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Details")
                    .web(WebProps::new().style("display", "none")),
            ),
        )
        .child(
            NativeElement::new("filters", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Filters")
                    .web(WebProps::new().style("visibility", "hidden")),
            ),
        )
        .child(
            NativeElement::new("summary", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Summary")
                    .web(WebProps::new().style("contentVisibility", "hidden")),
            ),
        )
        .child(
            NativeElement::new("activity", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Activity")
                    .web(WebProps::new().style("interactivity", "inert")),
            ),
        )
        .child(
            NativeElement::new("dialog", NativeRole::Dialog)
                .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)))
                .child(
                    NativeElement::new("close", NativeRole::Button)
                        .with_props(NativeProps::new().label("Close")),
                ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_native(&element).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
}

#[test]
fn runtime_routes_aria_hidden_actions() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .accessibility_hidden(Some(true))
            .web(WebProps::new().on_press("saveDocument")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();

    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("saveDocument")
    );
    assert!(runtime.accessibility_tree().is_none());
    assert_eq!(runtime.actions().invocations().len(), 1);
}

#[test]
fn runtime_suppresses_inert_subtree_actions() {
    let cases = [
        (
            "html inert",
            "tools-html-inert",
            NativeProps::new().inert(true),
        ),
        (
            "css interactivity inert",
            "tools-css-interactivity-inert",
            NativeProps::new().web(WebProps::new().style("interactivity", "inert")),
        ),
    ];

    for (name, key, props) in cases {
        let element = NativeElement::new(key, NativeRole::Toolbar)
            .with_props(props)
            .child(
                NativeElement::new("save", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Save")
                        .web(WebProps::new().on_press("saveDocument")),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let child = runtime.host().node(root_id).unwrap().children[0];
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                child,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert!(handled.invocation.is_none(), "{name}");
        assert!(handled.interaction_changes.is_empty(), "{name}");
        assert!(runtime.accessibility_tree().is_none(), "{name}");
        assert!(runtime.actions().invocations().is_empty(), "{name}");
    }
}

#[test]
fn runtime_suppresses_read_only_change_actions() {
    let element = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Name")
            .value("Ada")
            .read_only(true)
            .web(WebProps::new().on_change("setName")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setName");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("Grace"),
        )
        .unwrap();

    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("Ada")
    );
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_suppresses_read_only_selection_actions() {
    let element = NativeElement::new("theme", NativeRole::Select)
        .with_props(
            NativeProps::new()
                .label("Theme")
                .read_only(true)
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
    let inferred = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();
    let explicit = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value("compact"),
        )
        .unwrap();

    assert_eq!(inferred.event.value.as_deref(), Some("comfortable"));
    assert!(inferred.invocation.is_none());
    assert!(inferred.interaction_changes.is_empty());
    assert_eq!(explicit.event.value.as_deref(), Some("compact"));
    assert!(explicit.invocation.is_none());
    assert!(explicit.interaction_changes.is_empty());
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("comfortable")
    );
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_suppresses_read_only_ancestor_selection_value_events() {
    let element = NativeElement::new("theme", NativeRole::RadioGroup)
        .with_props(
            NativeProps::new()
                .label("Theme")
                .read_only(true)
                .web(WebProps::new().on_selection_change("setTheme")),
        )
        .child(
            NativeElement::new("light", NativeRole::Radio).with_props(
                NativeProps::new()
                    .label("Light")
                    .value("light")
                    .selected(true)
                    .checked(true),
            ),
        )
        .child(
            NativeElement::new("dark", NativeRole::Radio)
                .with_props(NativeProps::new().label("Dark").value("dark")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTheme");

    let root_id = runtime.render_native(&element).unwrap();
    let dark_id = runtime.host().node(root_id).unwrap().children[1];
    let selection = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            dark_id,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();
    let toggle = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            dark_id,
            crate::event::NativeEventKind::Toggle,
        ))
        .unwrap();

    assert_eq!(selection.event.value.as_deref(), Some("dark"));
    assert!(selection.invocation.is_none());
    assert!(selection.interaction_changes.is_empty());
    assert_eq!(toggle.event.value.as_deref(), None);
    assert!(toggle.invocation.is_none());
    assert!(toggle.interaction_changes.is_empty());
    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.value.as_deref(), Some("light"));
    assert!(accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(true));
    assert!(!accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(false));
    assert!(runtime.actions().invocations().is_empty());
}
