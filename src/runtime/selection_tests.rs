use super::*;
use crate::event::NativeEventKind;
use crate::input::{NativeEventContext, NativeInputModality};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{BlueprintHost, Gtk4Adapter, PlatformCommand, PlatformPlanningHost};
use crate::selection::{CollectionKey, Selection};
use crate::web::WebProps;

fn item(key: &str, value: &str) -> NativeElement {
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(NativeProps::new().value(value))
}

fn multiple_list(children: impl IntoIterator<Item = NativeElement>) -> NativeElement {
    let mut list = NativeElement::new("people", NativeRole::ListBox).with_props(
        NativeProps::new().web(
            WebProps::new()
                .attribute("data-selection-mode", "multiple")
                .on_selection_change("selectPeople"),
        ),
    );
    list.children.extend(children);
    list
}

fn action_list(selection_mode: &str, selection_behavior: &str) -> NativeElement {
    NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", selection_mode)
                    .attribute("data-selection-behavior", selection_behavior)
                    .event("onAction", "openPerson")
                    .on_selection_change("selectPeople"),
            ),
        )
        .child(item("ada-key", "Ada"))
        .child(item("linus-key", "Linus"))
}

fn pointer_context(modality: NativeInputModality, click_count: u8) -> NativeEventContext {
    NativeEventContext::new()
        .modality(modality)
        .click_count(click_count)
}

#[test]
fn mounted_item_selection_bubbles_a_stable_key_set_and_updates_siblings() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();
    let children = runtime.renderer.child_ids(root);

    let first = runtime
        .handle_native_event_with_changes(NativeEvent::new(
            children[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();
    let second = runtime
        .handle_native_event_with_changes(NativeEvent::new(
            children[1],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    assert_eq!(first.event.value.as_deref(), Some(r#"["ada-key"]"#));
    assert_eq!(
        second.event.value.as_deref(),
        Some(r#"["ada-key","linus-key"]"#)
    );
    assert_eq!(second.invocations.len(), 1);
    assert_eq!(second.invocations[0].node, children[1]);
    assert_eq!(second.invocations[0].current_target(), root);
    assert_eq!(
        second.invocations[0].selection().unwrap(),
        Some(Selection::keys([
            CollectionKey::from("ada-key"),
            CollectionKey::from("linus-key")
        ]))
    );
    assert!(runtime.interactions().node(children[0]).unwrap().selected);
    assert!(runtime.interactions().node(children[1]).unwrap().selected);
    assert!(
        runtime
            .host()
            .blueprint(children[0])
            .unwrap()
            .control_state
            .selected
    );
    assert!(
        runtime
            .host()
            .blueprint(children[1])
            .unwrap()
            .control_state
            .selected
    );
}

#[test]
fn uncontrolled_selection_survives_a_keyed_reorder() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();
    let ada = runtime.renderer.child_ids(root)[0];
    runtime
        .handle_native_event_with_changes(NativeEvent::new(ada, NativeEventKind::SelectionChange))
        .unwrap();

    runtime
        .render_native(&multiple_list([
            item("linus-key", "Linus"),
            item("ada-key", "Ada Lovelace"),
        ]))
        .unwrap();
    let reordered = runtime.renderer.child_ids(root);

    assert_eq!(reordered[1], ada);
    assert!(runtime.interactions().node(ada).unwrap().selected);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("ada-key")])
    );
}

#[test]
fn controlled_selected_keys_project_into_mounted_interaction_state() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let list = multiple_list([item("ada-key", "Ada"), item("linus-key", "Linus")]).with_props(
        NativeProps::new().web(
            WebProps::new()
                .attribute("data-selection-mode", "multiple")
                .attribute("selectedKeys", r#"["linus-key"]"#)
                .on_selection_change("selectPeople"),
        ),
    );
    let root = runtime.render_native(&list).unwrap();
    let children = runtime.renderer.child_ids(root);

    assert!(!runtime.interactions().node(children[0]).unwrap().selected);
    assert!(runtime.interactions().node(children[1]).unwrap().selected);
    assert!(
        !runtime
            .host()
            .blueprint(children[0])
            .unwrap()
            .control_state
            .selected
    );
    assert!(
        runtime
            .host()
            .blueprint(children[1])
            .unwrap()
            .control_state
            .selected
    );
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("linus-key")])
    );
}

#[test]
fn a_container_native_value_is_resolved_to_the_item_key() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value("Linus"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some(r#"["linus-key"]"#));
    assert_eq!(handled.invocations[0].current_target(), root);
}

#[test]
fn a_container_native_snapshot_replaces_the_previous_multi_selection() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();
    let children = runtime.renderer.child_ids(root);
    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            children[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value(r#"["Linus"]"#),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some(r#"["linus-key"]"#));
    assert_eq!(handled.invocations.len(), 1);
    assert_eq!(handled.invocations[0].current_target(), root);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("linus-key")])
    );
    assert!(!runtime.interactions().node(children[0]).unwrap().selected);
    assert!(runtime.interactions().node(children[1]).unwrap().selected);
    assert!(
        !runtime
            .host()
            .blueprint(children[0])
            .unwrap()
            .control_state
            .selected
    );
    assert!(
        runtime
            .host()
            .blueprint(children[1])
            .unwrap()
            .control_state
            .selected
    );
}

#[test]
fn controlled_all_projects_newly_loaded_items_before_native_creation() {
    let props = || {
        NativeProps::new().web(
            WebProps::new()
                .attribute("data-selection-mode", "multiple")
                .attribute("selectedKeys", "\"all\"")
                .on_selection_change("selectPeople"),
        )
    };
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let first = NativeElement::new("people", NativeRole::ListBox)
        .with_props(props())
        .child(item("ada-key", "Ada"));
    let root = runtime.render_native(&first).unwrap();

    let second = NativeElement::new("people", NativeRole::ListBox)
        .with_props(props())
        .child(item("ada-key", "Ada"))
        .child(item("new-key", "New"));
    runtime.render_native(&second).unwrap();
    let new_item = runtime.renderer.child_ids(root)[1];

    assert!(
        runtime
            .host()
            .blueprint(new_item)
            .unwrap()
            .control_state
            .selected
    );
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::All
    );
}

#[test]
fn duplicate_explicit_collection_keys_fail_before_mutating_the_host() {
    let duplicate = |element_key: &str| {
        item(element_key, element_key).with_props(
            NativeProps::new()
                .value(element_key)
                .web(WebProps::new().attribute("data-collection-key", "duplicate")),
        )
    };
    let list = multiple_list([duplicate("first"), duplicate("second")]);
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let error = runtime.render_native(&list).unwrap_err();

    assert!(error.to_string().contains("duplicate key"));
    assert!(runtime.host().nodes().is_empty());
}

#[test]
fn item_focus_updates_the_collection_focused_key() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();
    let linus = runtime.renderer.child_ids(root)[1];

    runtime
        .handle_native_event_with_changes(NativeEvent::new(linus, NativeEventKind::Focus))
        .unwrap();
    let manager = runtime.selections().manager(root).unwrap();
    assert_eq!(manager.focused_key().unwrap().as_str(), "linus-key");
    assert!(manager.is_focused());

    runtime
        .handle_native_event_with_changes(NativeEvent::new(linus, NativeEventKind::Blur))
        .unwrap();
    let manager = runtime.selections().manager(root).unwrap();
    assert_eq!(manager.focused_key().unwrap().as_str(), "linus-key");
    assert!(!manager.is_focused());
}

#[test]
fn radio_group_default_value_initializes_uncontrolled_selection_by_item_value() {
    let group = NativeElement::new("theme", NativeRole::RadioGroup)
        .with_props(NativeProps::new().web(WebProps::new().attribute("defaultValue", "dark")))
        .child(
            NativeElement::new("light-key", NativeRole::Radio)
                .with_props(NativeProps::new().value("light")),
        )
        .child(
            NativeElement::new("dark-key", NativeRole::Radio)
                .with_props(NativeProps::new().value("dark")),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let root = runtime.render_native(&group).unwrap();
    let children = runtime.renderer.child_ids(root);

    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("dark-key")])
    );
    assert_eq!(
        runtime
            .host()
            .blueprint(children[0])
            .unwrap()
            .control_state
            .checked,
        Some(false)
    );
    assert_eq!(
        runtime
            .host()
            .blueprint(children[1])
            .unwrap()
            .control_state
            .checked,
        Some(true)
    );
}

#[test]
fn replace_selection_arrow_navigation_moves_native_focus_and_selection() {
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectPeople"),
            ),
        )
        .child(item("ada-key", "Ada"))
        .child(
            item("grace-key", "Grace").with_props(NativeProps::new().value("Grace").disabled(true)),
        )
        .child(item("linus-key", "Linus"));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime.render_native(&list).unwrap();
    let children = runtime.renderer.child_ids(root);
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(children[0], NativeEventKind::KeyDown).value("Down"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, children[2]);
    assert_eq!(handled.event.value.as_deref(), Some("linus-key"));
    assert_eq!(handled.invocations.len(), 1);
    assert_eq!(handled.invocations[0].current_target(), root);
    assert_eq!(runtime.host().focused(), Some(children[2]));
    assert_eq!(
        runtime.host().commands().first(),
        Some(&PlatformCommand::RequestFocus { id: children[2] })
    );
    assert!(matches!(
        runtime.host().commands().get(1),
        Some(PlatformCommand::Update { id, .. }) if *id == children[2]
    ));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("linus-key")])
    );
}

#[test]
fn toggle_selection_arrow_navigation_moves_only_native_focus() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&multiple_list([
            item("ada-key", "Ada"),
            item("linus-key", "Linus"),
        ]))
        .unwrap();
    let children = runtime.renderer.child_ids(root);
    runtime
        .handle_native_event_with_changes(NativeEvent::new(children[0], NativeEventKind::Focus))
        .unwrap();
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(children[0], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert!(handled.invocations.is_empty());
    assert_eq!(runtime.host().focused(), Some(children[1]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );

    runtime
        .handle_native_event_with_changes(NativeEvent::new(children[1], NativeEventKind::Focus))
        .unwrap();
    assert_eq!(
        runtime
            .selections()
            .manager(root)
            .unwrap()
            .focused_key()
            .unwrap()
            .as_str(),
        "linus-key"
    );
}

#[test]
fn explicit_key_handler_owns_collection_arrow_navigation() {
    let first = NativeElement::new("ada-key", NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .value("Ada")
            .web(WebProps::new().on_key_down("handleKey")),
    );
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectPeople"),
            ),
        )
        .child(first)
        .child(item("linus-key", "Linus"));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("handleKey");
    runtime.actions_mut().register("selectPeople");
    let root = runtime.render_native(&list).unwrap();
    let first = runtime.renderer.child_ids(root)[0];
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(first, NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert_eq!(handled.invocations[0].action, "handleKey");
    assert!(runtime.host().commands().is_empty());
}

#[test]
fn automatic_tabs_mirror_rtl_navigation_and_manual_tabs_do_not_select() {
    fn tabs_tree(manual: bool) -> NativeElement {
        let mut web = WebProps::new()
            .attribute("data-selection-mode", "single")
            .on_selection_change("selectTab");
        if manual {
            web = web.attribute("keyboardActivation", "manual");
        }
        NativeElement::new("tabs", NativeRole::Tabs)
            .with_props(NativeProps::new().dir("rtl").web(web))
            .child(
                NativeElement::new("tab-list", NativeRole::TabList)
                    .with_props(
                        NativeProps::new().orientation(crate::geometry::Orientation::Horizontal),
                    )
                    .child(
                        NativeElement::new("settings", NativeRole::Tab)
                            .with_props(NativeProps::new().value("Settings")),
                    )
                    .child(
                        NativeElement::new("account", NativeRole::Tab)
                            .with_props(NativeProps::new().value("Account")),
                    ),
            )
    }

    let mut automatic = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    automatic.actions_mut().register("selectTab");
    let root = automatic.render_native(&tabs_tree(false)).unwrap();
    let tab_list = automatic.renderer.child_ids(root)[0];
    let tabs = automatic.renderer.child_ids(tab_list);
    let handled = automatic
        .handle_native_event_with_changes(
            NativeEvent::new(tabs[0], NativeEventKind::KeyDown).value("ArrowLeft"),
        )
        .unwrap();
    assert_eq!(handled.event.node, tabs[1]);
    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(automatic.host().focused(), Some(tabs[1]));

    let mut manual = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    manual.actions_mut().register("selectTab");
    let root = manual.render_native(&tabs_tree(true)).unwrap();
    let tab_list = manual.renderer.child_ids(root)[0];
    let tabs = manual.renderer.child_ids(tab_list);
    let handled = manual
        .handle_native_event_with_changes(
            NativeEvent::new(tabs[0], NativeEventKind::KeyDown).value("ArrowLeft"),
        )
        .unwrap();
    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert!(handled.invocations.is_empty());
    assert_eq!(manual.host().focused(), Some(tabs[1]));
    assert_eq!(
        manual.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn list_box_enter_dispatches_action_while_space_remains_selection() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&action_list("single", "replace"))
        .unwrap();
    let ada = runtime.renderer.child_ids(root)[0];
    let keyboard = pointer_context(NativeInputModality::Keyboard, 0).handled_activation(true);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::PressStart).context(keyboard),
        )
        .unwrap();
    let key_down = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::KeyDown)
                .value("Enter")
                .context(keyboard),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::Press).context(keyboard),
        )
        .unwrap();

    assert_eq!(key_down.event.kind, NativeEventKind::KeyDown);
    assert!(key_down.invocations.is_empty());
    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.event.value.as_deref(), Some("ada-key"));
    assert_eq!(action.invocations.len(), 1);
    assert_eq!(action.invocations[0].action, "openPerson");
    assert_eq!(action.invocations[0].node, ada);
    assert_eq!(action.invocations[0].current_target(), root);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );

    let selection = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::KeyDown)
                .value("Space")
                .context(NativeEventContext::new().modality(NativeInputModality::Keyboard)),
        )
        .unwrap();
    assert_eq!(selection.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(selection.invocations[0].action, "selectPeople");
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("ada-key")])
    );
}

#[test]
fn toggle_list_action_suppresses_the_native_selection_when_empty() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&action_list("multiple", "toggle"))
        .unwrap();
    let ada = runtime.renderer.child_ids(root)[0];
    let mouse = pointer_context(NativeInputModality::Mouse, 1);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::PressStart).context(mouse),
        )
        .unwrap();
    let suppressed = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value(r#"["Ada"]"#),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::Press).context(mouse),
        )
        .unwrap();

    assert!(suppressed.invocations.is_empty());
    assert_eq!(suppressed.event.value.as_deref(), Some("[]"));
    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.invocations[0].action, "openPerson");
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
    assert!(
        !runtime
            .host()
            .blueprint(ada)
            .unwrap()
            .control_state
            .selected
    );
}

#[test]
fn replace_list_uses_single_click_for_selection_and_double_click_for_action() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&action_list("single", "replace"))
        .unwrap();
    let ada = runtime.renderer.child_ids(root)[0];
    let first_click = pointer_context(NativeInputModality::Mouse, 1);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::PressStart).context(first_click),
        )
        .unwrap();
    let selected = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value("Ada"),
        )
        .unwrap();
    let first_press = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::Press).context(first_click),
        )
        .unwrap();

    assert_eq!(selected.invocations[0].action, "selectPeople");
    assert_eq!(first_press.event.kind, NativeEventKind::Press);
    assert!(first_press.invocations.is_empty());

    let second_click = pointer_context(NativeInputModality::Mouse, 2);
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::PressStart).context(second_click),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::Press).context(second_click),
        )
        .unwrap();

    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.invocations[0].action, "openPerson");
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("ada-key")])
    );
}

#[test]
fn touch_tap_prefers_action_and_reverts_the_native_selection() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&action_list("single", "replace"))
        .unwrap();
    let ada = runtime.renderer.child_ids(root)[0];
    let touch = pointer_context(NativeInputModality::Touch, 1);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::PressStart).context(touch),
        )
        .unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value("Ada"),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(ada, NativeEventKind::Press).context(touch),
        )
        .unwrap();

    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.invocations[0].action, "openPerson");
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn touch_long_press_enters_selection_mode_until_the_selection_is_cleared() {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    runtime.actions_mut().register("selectPeople");
    let root = runtime
        .render_native(&action_list("multiple", "toggle"))
        .unwrap();
    let mut items = runtime.renderer.child_ids(root);
    let touch = pointer_context(NativeInputModality::Touch, 1);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::PressStart).context(touch),
        )
        .unwrap();
    let long_press = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::LongPress).context(touch),
        )
        .unwrap();

    assert_eq!(long_press.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(long_press.event.value.as_deref(), Some(r#"["ada-key"]"#));
    assert_eq!(long_press.invocations[0].action, "selectPeople");
    assert!(long_press
        .invocations
        .iter()
        .all(|invocation| invocation.action != "openPerson"));

    assert_eq!(
        runtime
            .render_native(&action_list("multiple", "toggle"))
            .unwrap(),
        root
    );
    items = runtime.renderer.child_ids(root);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::PressStart).context(touch),
        )
        .unwrap();
    let second_selection = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange)
                .value(r#"["ada-key","linus-key"]"#),
        )
        .unwrap();
    let second_press = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::Press).context(touch),
        )
        .unwrap();

    assert_eq!(second_selection.invocations[0].action, "selectPeople");
    assert_eq!(second_press.event.kind, NativeEventKind::Press);
    assert!(second_press.invocations.is_empty());
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([
            CollectionKey::from("ada-key"),
            CollectionKey::from("linus-key"),
        ])
    );

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(root, NativeEventKind::SelectionChange).value("[]"),
        )
        .unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::PressStart).context(touch),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::Press).context(touch),
        )
        .unwrap();

    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.invocations[0].action, "openPerson");
}

#[test]
fn disabled_behavior_selection_preserves_action_but_disabled_items_do_not() {
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "multiple")
                    .attribute("data-selection-behavior", "toggle")
                    .attribute("disabledKeys", r#"["ada-key"]"#)
                    .attribute("disabledBehavior", "selection")
                    .event("onAction", "openPerson"),
            ),
        )
        .child(item("ada-key", "Ada"))
        .child(
            item("linus-key", "Linus").with_props(NativeProps::new().value("Linus").disabled(true)),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openPerson");
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);
    let keyboard = NativeEventContext::new().modality(NativeInputModality::Keyboard);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::PressStart).context(keyboard),
        )
        .unwrap();
    let action = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::Press).context(keyboard),
        )
        .unwrap();
    assert_eq!(action.event.kind, NativeEventKind::Action);
    assert_eq!(action.invocations[0].action, "openPerson");

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::PressStart).context(keyboard),
        )
        .unwrap();
    let disabled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::Press).context(keyboard),
        )
        .unwrap();
    assert_eq!(disabled.event.kind, NativeEventKind::Press);
    assert!(disabled.invocations.is_empty());
}

#[test]
fn tree_item_enter_bubbles_action_without_mutating_selection() {
    let tree = NativeElement::new("files", NativeRole::Tree)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .event("onAction", "openFile")
                    .on_selection_change("selectFile"),
            ),
        )
        .child(
            NativeElement::new("readme-key", NativeRole::TreeItem)
                .with_props(NativeProps::new().value("README.md")),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("openFile");
    runtime.actions_mut().register("selectFile");
    let root = runtime.render_native(&tree).unwrap();
    let item = runtime.renderer.child_ids(root)[0];
    let keyboard = NativeEventContext::new().modality(NativeInputModality::Keyboard);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(item, NativeEventKind::PressStart).context(keyboard),
        )
        .unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(item, NativeEventKind::Press).context(keyboard),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::Action);
    assert_eq!(handled.event.value.as_deref(), Some("readme-key"));
    assert_eq!(handled.invocations[0].action, "openFile");
    assert_eq!(handled.invocations[0].current_target(), root);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}
