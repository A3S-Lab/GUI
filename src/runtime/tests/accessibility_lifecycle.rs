use super::super::*;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

#[test]
fn runtime_accessibility_tree_uses_rerendered_control_state_after_interaction() {
    let first = NativeElement::new("email", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Email")
            .value("old@example.com")
            .web(WebProps::new().on_change("setEmail")),
    );
    let second = NativeElement::new("email", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Email")
            .value("controlled@example.com")
            .web(WebProps::new().on_change("setEmail")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEmail");

    let root_id = runtime.render_native(&first).unwrap();
    runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("local@example.com"),
        )
        .unwrap();
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("local@example.com")
    );

    let second_id = runtime.render_native(&second).unwrap();

    assert_eq!(second_id, root_id);
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("controlled@example.com")
    );
}

#[test]
fn runtime_interactions_start_from_rerendered_control_state() {
    let first = NativeElement::new("notifications", NativeRole::Switch)
        .with_props(NativeProps::new().label("Notifications").checked(false));
    let second = NativeElement::new("notifications", NativeRole::Switch)
        .with_props(NativeProps::new().label("Notifications").checked(false));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&first).unwrap();
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Toggle,
        ))
        .unwrap();
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));

    let second_id = runtime.render_native(&second).unwrap();
    assert_eq!(second_id, root_id);
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));

    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Toggle,
        ))
        .unwrap();

    assert_eq!(handled.interaction_changes.len(), 1);
    assert_eq!(handled.interaction_changes[0].before.checked, Some(false));
    assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
}

#[test]
fn runtime_accessibility_tree_preserves_focus_across_rerender() {
    let first =
        NativeElement::new("save", NativeRole::Button).with_props(NativeProps::new().label("Save"));
    let second = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().label("Saved"));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&first).unwrap();
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    let second_id = runtime.render_native(&second).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(second_id, root_id);
    assert_eq!(accessibility.label.as_deref(), Some("Saved"));
    assert!(accessibility.focused);
}

#[test]
fn runtime_prunes_interaction_state_for_unmounted_nodes() {
    let first = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
    let second = NativeElement::new("tools", NativeRole::Toolbar).child(
        NativeElement::new("cancel", NativeRole::Button)
            .with_props(NativeProps::new().label("Cancel")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&first).unwrap();
    let save = runtime.host().node(root_id).unwrap().children[0];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            save,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    assert!(runtime.interactions().node(save).unwrap().focused);

    runtime.render_native(&second).unwrap();

    assert!(runtime.interactions().node(save).is_none());
    assert!(runtime.interactions().changes().is_empty());
    assert!(!runtime.accessibility_tree().unwrap().children[0].focused);
}

#[test]
fn runtime_ignores_native_events_for_unmounted_nodes() {
    let first = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("save", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Save")
                    .web(WebProps::new().on_press("saveDocument")),
            ),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
    let second = NativeElement::new("tools", NativeRole::Toolbar).child(
        NativeElement::new("cancel", NativeRole::Button)
            .with_props(NativeProps::new().label("Cancel")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&first).unwrap();
    let save = runtime.host().node(root_id).unwrap().children[0];
    runtime.render_native(&second).unwrap();

    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            save,
            crate::event::NativeEventKind::Press,
        ))
        .unwrap();

    assert_eq!(handled.event.node, save);
    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert!(runtime.actions().invocations().is_empty());
    assert!(runtime.host().node(save).is_none());
}

#[test]
fn runtime_prunes_interaction_state_for_non_interactive_rerendered_subtrees() {
    let first = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("primary", NativeRole::View).child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save")),
            ),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
    let second = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("primary", NativeRole::View)
                .with_props(NativeProps::new().hidden(true))
                .child(
                    NativeElement::new("save", NativeRole::Button)
                        .with_props(NativeProps::new().label("Save")),
                ),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel").auto_focus(true)),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&first).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    let save = runtime.host().node(children[0]).unwrap().children[0];
    let cancel = children[1];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            save,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    assert!(runtime.interactions().node(save).unwrap().focused);

    runtime.render_native(&second).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(runtime.interactions().node(save).is_none());
    assert!(runtime.interactions().node(cancel).is_none());
    assert!(runtime.interactions().has_focus_history());
    assert!(runtime.interactions().changes().is_empty());
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Cancel"));
    assert!(!accessibility.children[0].focused);
}

#[test]
fn runtime_accessibility_tree_projects_selection_value_to_children() {
    let tree = NativeElement::new("project", NativeRole::Select)
        .with_props(
            NativeProps::new()
                .label("Project")
                .web(WebProps::new().on_selection_change("setProject")),
        )
        .child(
            NativeElement::new("a3s", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("A3S").value("a3s").selected(true)),
        )
        .child(
            NativeElement::new("other", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Other").value("other")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setProject");

    let root_id = runtime.render_native(&tree).unwrap();
    runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value("other"),
        )
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.value.as_deref(), Some("other"));
    assert!(!accessibility.children[0].selected);
    assert!(accessibility.children[1].selected);
}

#[test]
fn runtime_accessibility_tree_projects_single_listbox_child_selection_to_siblings() {
    let list_box = NativeElement::new("project", NativeRole::ListBox)
        .with_props(NativeProps::new().label("Project"))
        .child(
            NativeElement::new("a3s", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("A3S").value("a3s").selected(true)),
        )
        .child(
            NativeElement::new("other", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Other").value("other")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&list_box).unwrap();
    let other = runtime.host().node(root_id).unwrap().children[1];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            other,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.multiple);
    assert_eq!(accessibility.value.as_deref(), Some("other"));
    assert!(!accessibility.children[0].selected);
    assert!(accessibility.children[1].selected);
}

#[test]
fn runtime_accessibility_tree_preserves_multiple_listbox_child_selections() {
    let list_box = NativeElement::new("project", NativeRole::ListBox)
        .with_props(NativeProps::new().label("Project").multiple(true))
        .child(
            NativeElement::new("a3s", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("A3S").value("a3s").selected(true)),
        )
        .child(
            NativeElement::new("other", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Other").value("other")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&list_box).unwrap();
    let other = runtime.host().node(root_id).unwrap().children[1];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            other,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(accessibility.multiple);
    assert!(accessibility.children[0].selected);
    assert!(accessibility.children[1].selected);
}

#[test]
fn runtime_accessibility_tree_projects_radio_group_value_to_checked_child() {
    let tree = NativeElement::new("theme", NativeRole::RadioGroup)
        .with_props(
            NativeProps::new()
                .label("Theme")
                .web(WebProps::new().on_selection_change("setTheme")),
        )
        .child(
            NativeElement::new("light", NativeRole::Radio).with_props(
                NativeProps::new()
                    .label("Light")
                    .value("light")
                    .selected(true),
            ),
        )
        .child(
            NativeElement::new("dark", NativeRole::Radio)
                .with_props(NativeProps::new().label("Dark").value("dark")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTheme");

    let root_id = runtime.render_native(&tree).unwrap();
    runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value("dark"),
        )
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(false));
    assert!(accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(true));

    runtime
        .handle_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::SelectionChange)
                .value("light"),
        )
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(true));
    assert!(!accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(false));
}

#[test]
fn runtime_accessibility_tree_reflects_direct_radio_selection_as_checked() {
    let tree = NativeElement::new("theme", NativeRole::RadioGroup)
        .with_props(NativeProps::new().label("Theme"))
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

    let root_id = runtime.render_native(&tree).unwrap();
    let radio_id = runtime.host().node(root_id).unwrap().children[1];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            radio_id,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(false));
    assert!(accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(true));
}

#[test]
fn runtime_bubbles_child_selection_to_parent_action_with_value() {
    let tree = NativeElement::new("theme", NativeRole::RadioGroup)
        .with_props(
            NativeProps::new()
                .label("Theme")
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

    let root_id = runtime.render_native(&tree).unwrap();
    let radio_id = runtime.host().node(root_id).unwrap().children[1];
    let handled = runtime
        .handle_native_event_with_changes(crate::event::NativeEvent::new(
            radio_id,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();
    let invocation = handled.invocation.unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("dark"));
    assert_eq!(invocation.node, radio_id);
    assert_eq!(invocation.action, "setTheme");
    assert_eq!(invocation.value.as_deref(), Some("dark"));
    assert_eq!(handled.interaction_changes.len(), 1);
    assert_eq!(handled.interaction_changes[0].node, radio_id);
    assert_eq!(
        handled.interaction_changes[0].after.value.as_deref(),
        Some("dark")
    );

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.value.as_deref(), Some("dark"));
    assert!(!accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(false));
    assert!(accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(true));
}

#[test]
fn runtime_accessibility_tree_projects_direct_tab_selection_to_siblings() {
    let tree = NativeElement::new("settings", NativeRole::Tabs)
        .with_props(NativeProps::new().label("Settings"))
        .child(
            NativeElement::new("profile", NativeRole::Tab)
                .with_props(NativeProps::new().label("Profile").selected(true)),
        )
        .child(
            NativeElement::new("billing", NativeRole::Tab)
                .with_props(NativeProps::new().label("Billing")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&tree).unwrap();
    let tab_id = runtime.host().node(root_id).unwrap().children[1];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            tab_id,
            crate::event::NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].selected);
    assert!(accessibility.children[1].selected);
}
