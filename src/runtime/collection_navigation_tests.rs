use super::*;
use crate::event::NativeEventKind;
use crate::geometry::{Rect, Size};
use crate::input::NativeKeyModifiers;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{
    AppKitAdapter, Gtk4Adapter, PlatformAdapter, PlatformCommand, PlatformPlanningHost,
    WinUiAdapter,
};
use crate::selection::{CollectionKey, CollectionLayoutSnapshot, Selection};
use crate::web::WebProps;

fn collection(
    role: NativeRole,
    item_role: NativeRole,
    mode: &str,
    behavior: Option<&str>,
) -> NativeElement {
    let mut web = WebProps::new()
        .attribute("data-selection-mode", mode)
        .on_selection_change("selectItem");
    if let Some(behavior) = behavior {
        web = web.attribute("selectionBehavior", behavior);
    }
    NativeElement::new("collection", role)
        .with_props(NativeProps::new().web(web))
        .child(NativeElement::new("first", item_role).with_props(NativeProps::new().value("First")))
        .child(
            NativeElement::new("second", item_role).with_props(NativeProps::new().value("Second")),
        )
        .child(NativeElement::new("third", item_role).with_props(NativeProps::new().value("Third")))
}

fn runtime_for(
    element: &NativeElement,
) -> (GuiRuntime<PlatformPlanningHost<Gtk4Adapter>>, HostNodeId) {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectItem");
    let root = runtime.render_native(element).unwrap();
    (runtime, root)
}

fn assert_replace_navigation<A: PlatformAdapter>(adapter: A) {
    let list = collection(NativeRole::ListBox, NativeRole::ListBoxItem, "single", None);
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    runtime.actions_mut().register("selectItem");
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, items[1]);
    assert_eq!(handled.invocations[0].current_target(), root);
    assert_eq!(runtime.host().focused(), Some(items[1]));
    assert_eq!(
        runtime.host().commands().first(),
        Some(&PlatformCommand::RequestFocus { id: items[1] })
    );
    assert!(matches!(
        runtime.host().commands().get(1),
        Some(PlatformCommand::Update { id, .. }) if *id == items[1]
    ));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("second")])
    );
}

fn assert_typeahead_navigation<A: PlatformAdapter>(adapter: A) {
    let list = collection(NativeRole::ListBox, NativeRole::ListBoxItem, "single", None);
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    runtime.actions_mut().register("selectItem");
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("s"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, items[1]);
    assert_eq!(
        runtime.host().commands().first(),
        Some(&PlatformCommand::RequestFocus { id: items[1] })
    );
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("second")])
    );
}

fn assert_select_all_and_clear<A: PlatformAdapter>(adapter: A, modifiers: NativeKeyModifiers) {
    let list = collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "multiple",
        None,
    );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    runtime.actions_mut().register("selectItem");
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);

    let selected = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown)
                .value("a")
                .modifiers(modifiers),
        )
        .unwrap();

    assert_eq!(selected.event.node, root);
    assert_eq!(selected.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(selected.event.value.as_deref(), Some(r#""all""#));
    assert_eq!(selected.invocations[0].action, "selectItem");
    assert_eq!(selected.invocations[0].current_target(), root);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::All
    );
    assert!(items.iter().all(|item| {
        runtime
            .host()
            .blueprint(*item)
            .is_some_and(|blueprint| blueprint.control_state.selected)
    }));

    let cleared = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();

    assert_eq!(cleared.event.node, root);
    assert_eq!(cleared.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(cleared.event.value.as_deref(), Some("[]"));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
    assert!(items.iter().all(|item| {
        runtime
            .host()
            .blueprint(*item)
            .is_some_and(|blueprint| !blueprint.control_state.selected)
    }));
}

#[test]
fn collection_navigation_has_the_same_focus_and_selection_contract_on_all_adapters() {
    assert_replace_navigation(AppKitAdapter);
    assert_replace_navigation(Gtk4Adapter);
    assert_replace_navigation(WinUiAdapter);
    assert_typeahead_navigation(AppKitAdapter);
    assert_typeahead_navigation(Gtk4Adapter);
    assert_typeahead_navigation(WinUiAdapter);
    assert_select_all_and_clear(AppKitAdapter, NativeKeyModifiers::new().meta(true));
    assert_select_all_and_clear(Gtk4Adapter, NativeKeyModifiers::new().control(true));
    assert_select_all_and_clear(WinUiAdapter, NativeKeyModifiers::new().control(true));
}

#[test]
fn move_hook_owns_arrow_keys_before_collection_navigation() {
    let list = collection(NativeRole::ListBox, NativeRole::ListBoxItem, "single", None);
    let mut list = list;
    list.children[0].props.web = list.children[0]
        .props
        .web
        .clone()
        .event("onMove", "moveItem");
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectItem");
    runtime.actions_mut().register("moveItem");
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);
    runtime.host_mut().clear_commands();

    let key = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert_eq!(key.event.kind, NativeEventKind::KeyDown);
    assert!(key.invocations.is_empty());
    assert!(runtime.host().commands().is_empty());
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn list_box_typeahead_uses_locale_sensitive_item_text() {
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem"),
            ),
        )
        .child(
            NativeElement::new("alpha", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Alpha")),
        )
        .child(
            NativeElement::new("eclair", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Éclair")),
        )
        .child(
            NativeElement::new("zulu", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Zulu")),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectItem");
    runtime.i18n_mut().set_default_locale(Some("fr-FR"));
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("e"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, items[1]);
    assert_eq!(runtime.host().focused(), Some(items[1]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("eclair")])
    );
}

#[test]
fn typeahead_buffers_keys_and_prefers_explicit_text_value() {
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem"),
            ),
        )
        .child(
            NativeElement::new("alpha", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Alpha")),
        )
        .child(
            NativeElement::new("gamma", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Gamma")),
        )
        .child(
            NativeElement::new("green", NativeRole::ListBoxItem).with_props(
                NativeProps::new()
                    .label("Visible fallback")
                    .web(WebProps::new().attribute("textValue", "Green")),
            ),
        );
    let (mut runtime, root) = runtime_for(&list);
    let items = runtime.renderer.child_ids(root);

    let first = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("g"),
        )
        .unwrap();
    runtime.render_native(&list).unwrap();
    let rerendered_items = runtime.renderer.child_ids(root);
    assert_eq!(rerendered_items, items);
    let second = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(rerendered_items[1], NativeEventKind::KeyDown).value("r"),
        )
        .unwrap();

    assert_eq!(first.event.node, items[1]);
    assert_eq!(second.event.node, items[2]);
    assert_eq!(runtime.host().focused(), Some(items[2]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("green")])
    );
}

#[test]
fn menu_typeahead_moves_focus_only_and_ignores_control_shortcuts() {
    let menu = NativeElement::new("actions", NativeRole::Menu)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem"),
            ),
        )
        .child(
            NativeElement::new("copy", NativeRole::MenuItem)
                .with_props(NativeProps::new().label("Copy")),
        )
        .child(
            NativeElement::new("paste", NativeRole::MenuItem)
                .with_props(NativeProps::new().label("Paste")),
        );
    let (mut runtime, root) = runtime_for(&menu);
    let items = runtime.renderer.child_ids(root);

    let shortcut = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown)
                .value("p")
                .modifiers(NativeKeyModifiers::new().control(true)),
        )
        .unwrap();
    let typeahead = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("p"),
        )
        .unwrap();

    assert_eq!(shortcut.event.kind, NativeEventKind::KeyDown);
    assert!(shortcut.invocations.is_empty());
    assert_eq!(typeahead.event.kind, NativeEventKind::KeyDown);
    assert!(typeahead.invocations.is_empty());
    assert_eq!(runtime.host().focused(), Some(items[1]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn typeahead_can_focus_an_item_disabled_only_for_selection() {
    let list = NativeElement::new("people", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .attribute("disabledBehavior", "selection")
                    .attribute("disabledKeys", r#"["beta"]"#)
                    .on_selection_change("selectItem"),
            ),
        )
        .child(
            NativeElement::new("alpha", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Alpha")),
        )
        .child(
            NativeElement::new("beta", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().label("Beta")),
        );
    let (mut runtime, root) = runtime_for(&list);
    let items = runtime.renderer.child_ids(root);

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("b"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert!(handled.invocations.is_empty());
    assert_eq!(runtime.host().focused(), Some(items[1]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn tree_typeahead_replaces_selection() {
    let tree = NativeElement::new("files", NativeRole::Tree)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem"),
            ),
        )
        .child(
            NativeElement::new("documents", NativeRole::TreeItem)
                .with_props(NativeProps::new().label("Documents")),
        )
        .child(
            NativeElement::new("photos", NativeRole::TreeItem)
                .with_props(NativeProps::new().label("Photos")),
        );
    let (mut runtime, root) = runtime_for(&tree);
    let items = runtime.renderer.child_ids(root);

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("p"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, items[1]);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("photos")])
    );
}

#[test]
fn selection_input_typeahead_moves_popup_focus_without_committing() {
    for role in [NativeRole::Select, NativeRole::ComboBox] {
        let input = NativeElement::new("input", role)
            .with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .attribute("data-selection-mode", "single")
                        .on_selection_change("selectItem"),
                ),
            )
            .child(
                NativeElement::new("options", NativeRole::ListBox)
                    .child(
                        NativeElement::new("alpha", NativeRole::ListBoxItem)
                            .with_props(NativeProps::new().label("Alpha")),
                    )
                    .child(
                        NativeElement::new("beta", NativeRole::ListBoxItem)
                            .with_props(NativeProps::new().label("Beta")),
                    ),
            );
        let (mut runtime, root) = runtime_for(&input);
        let list = runtime.renderer.child_ids(root)[0];
        let items = runtime.renderer.child_ids(list);

        let handled = runtime
            .handle_native_event_with_changes(
                NativeEvent::new(items[0], NativeEventKind::KeyDown).value("b"),
            )
            .unwrap();

        assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
        assert!(handled.invocations.is_empty());
        assert_eq!(runtime.host().focused(), Some(items[1]));
        assert_eq!(
            runtime.selections().manager(root).unwrap().selection(),
            &Selection::empty()
        );
    }
}

#[test]
fn menu_arrows_wrap_focus_without_changing_selection() {
    let mut menu = collection(NativeRole::Menu, NativeRole::MenuItem, "single", None);
    menu.props.web = menu.props.web.attribute("shouldFocusWrap", "true");
    let (mut runtime, root) = runtime_for(&menu);
    let items = runtime.renderer.child_ids(root);

    let next = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();
    let wrapped = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[2], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert!(next.invocations.is_empty());
    assert_eq!(runtime.host().focused(), Some(items[0]));
    assert_eq!(wrapped.event.kind, NativeEventKind::KeyDown);
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}

#[test]
fn tree_arrows_move_focus_and_replace_selection() {
    let (mut runtime, root) = runtime_for(&collection(
        NativeRole::Tree,
        NativeRole::TreeItem,
        "single",
        None,
    ));
    let items = runtime.renderer.child_ids(root);

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowDown"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.event.node, items[1]);
    assert_eq!(handled.invocations[0].action, "selectItem");
    assert_eq!(runtime.host().focused(), Some(items[1]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("second")])
    );
}

#[test]
fn tree_expansion_projects_visibility_and_routes_the_expanded_key_set() {
    let tree = NativeElement::new("files", NativeRole::Tree)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem")
                    .event("onExpandedChange", "setExpanded"),
            ),
        )
        .child(
            NativeElement::new("documents", NativeRole::TreeItem).with_props(
                NativeProps::new().label("Documents").web(
                    WebProps::new()
                        .attribute("data-tree-level", "1")
                        .attribute("data-has-child-items", "true"),
                ),
            ),
        )
        .child(
            NativeElement::new("resume", NativeRole::TreeItem).with_props(
                NativeProps::new().label("Resume").web(
                    WebProps::new()
                        .attribute("data-tree-level", "2")
                        .attribute("data-tree-parent-key", "documents"),
                ),
            ),
        )
        .child(
            NativeElement::new("photos", NativeRole::TreeItem).with_props(
                NativeProps::new()
                    .label("Photos")
                    .web(WebProps::new().attribute("data-tree-level", "1")),
            ),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("selectItem");
    runtime.actions_mut().register("setExpanded");
    let root = runtime.render_native(&tree).unwrap();
    let items = runtime.renderer.child_ids(root);

    assert!(
        runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );

    let expanded = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowRight"),
        )
        .unwrap();

    assert_eq!(expanded.event.node, root);
    assert_eq!(expanded.event.kind, NativeEventKind::Toggle);
    assert_eq!(expanded.event.value.as_deref(), Some(r#"["documents"]"#));
    assert_eq!(expanded.invocations[0].action, "setExpanded");
    assert!(
        !runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );
    assert_eq!(
        runtime
            .host()
            .blueprint(items[0])
            .unwrap()
            .control_state
            .expanded,
        Some(true)
    );

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowRight"),
        )
        .unwrap();
    assert_eq!(runtime.host().focused(), Some(items[1]));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::KeyDown).value("ArrowLeft"),
        )
        .unwrap();
    let collapsed = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowLeft"),
        )
        .unwrap();
    assert_eq!(collapsed.event.value.as_deref(), Some("[]"));
    assert!(
        runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );
}

#[test]
fn tree_expansion_rolls_back_when_the_expanded_change_action_fails() {
    let tree = NativeElement::new("files", NativeRole::Tree)
        .with_props(
            NativeProps::new().web(WebProps::new().event("onExpandedChange", "missingAction")),
        )
        .child(
            NativeElement::new("documents", NativeRole::TreeItem).with_props(
                NativeProps::new()
                    .label("Documents")
                    .web(WebProps::new().attribute("data-has-child-items", "true")),
            ),
        )
        .child(
            NativeElement::new("resume", NativeRole::TreeItem).with_props(
                NativeProps::new()
                    .label("Resume")
                    .web(WebProps::new().attribute("data-tree-parent-key", "documents")),
            ),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime.render_native(&tree).unwrap();
    let items = runtime.renderer.child_ids(root);

    let error = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowRight"),
        )
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("unregistered action missingAction"));
    assert_eq!(
        runtime
            .host()
            .blueprint(items[0])
            .unwrap()
            .control_state
            .expanded,
        Some(false)
    );
    assert!(
        runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );
}

#[test]
fn controlled_tree_expanded_keys_override_optimistic_state_on_rerender() {
    let tree = |expanded_keys: &str| {
        NativeElement::new("files", NativeRole::Tree)
            .with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .attribute("expandedKeys", expanded_keys)
                        .event("onExpandedChange", "setExpanded"),
                ),
            )
            .child(
                NativeElement::new("documents", NativeRole::TreeItem).with_props(
                    NativeProps::new()
                        .label("Documents")
                        .web(WebProps::new().attribute("data-has-child-items", "true")),
                ),
            )
            .child(
                NativeElement::new("resume", NativeRole::TreeItem).with_props(
                    NativeProps::new()
                        .label("Resume")
                        .web(WebProps::new().attribute("data-tree-parent-key", "documents")),
                ),
            )
    };
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    runtime.actions_mut().register("setExpanded");
    let root = runtime.render_native(&tree(r#"["documents"]"#)).unwrap();
    let items = runtime.renderer.child_ids(root);
    assert!(
        !runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );
    runtime
        .handle_native_event_with_changes(NativeEvent::new(items[1], NativeEventKind::Focus))
        .unwrap();

    let rerendered_root = runtime.render_native(&tree("[]")).unwrap();
    let rerendered_items = runtime.renderer.child_ids(rerendered_root);

    assert_eq!(rerendered_root, root);
    assert_eq!(rerendered_items, items);
    assert_eq!(runtime.host().focused(), Some(items[0]));
    assert!(
        runtime
            .host()
            .blueprint(items[1])
            .unwrap()
            .control_state
            .hidden
    );
    assert_eq!(
        runtime
            .host()
            .blueprint(items[0])
            .unwrap()
            .control_state
            .expanded,
        Some(false)
    );
}

#[test]
fn radio_arrows_select_and_wrap() {
    let (mut runtime, root) = runtime_for(&collection(
        NativeRole::RadioGroup,
        NativeRole::Radio,
        "single",
        None,
    ));
    let items = runtime.renderer.child_ids(root);

    let next = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("ArrowRight"),
        )
        .unwrap();
    let wrapped = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[2], NativeEventKind::KeyDown).value("ArrowRight"),
        )
        .unwrap();

    assert_eq!(next.event.node, items[1]);
    assert_eq!(wrapped.event.node, items[0]);
    assert_eq!(runtime.host().focused(), Some(items[0]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first")])
    );
}

#[test]
fn home_end_and_unmeasured_page_keys_use_portable_collection_boundaries() {
    let (mut runtime, root) = runtime_for(&collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "single",
        None,
    ));
    let items = runtime.renderer.child_ids(root);

    let end = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("End"),
        )
        .unwrap();
    let page_up = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[2], NativeEventKind::KeyDown).value("Page_Up"),
        )
        .unwrap();

    assert_eq!(end.event.node, items[2]);
    assert_eq!(page_up.event.node, items[0]);
    assert_eq!(runtime.host().focused(), Some(items[0]));
}

#[test]
fn measured_page_navigation_uses_layout_and_skips_disabled_items() {
    let list = NativeElement::new("collection", NativeRole::ListBox)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectItem"),
            ),
        )
        .child(NativeElement::new("a", NativeRole::ListBoxItem))
        .child(NativeElement::new("b", NativeRole::ListBoxItem))
        .child(
            NativeElement::new("c", NativeRole::ListBoxItem)
                .with_props(NativeProps::new().disabled(true)),
        )
        .child(NativeElement::new("d", NativeRole::ListBoxItem))
        .child(NativeElement::new("e", NativeRole::ListBoxItem));
    let (mut runtime, root) = runtime_for(&list);
    let items = runtime.renderer.child_ids(root);
    let mut layout =
        CollectionLayoutSnapshot::new(Rect::new(0.0, 20.0, 200.0, 100.0), Size::new(200.0, 200.0));
    for (index, key) in ["a", "b", "c", "d", "e"].into_iter().enumerate() {
        layout.insert_item_rect(key, Rect::new(0.0, index as f64 * 40.0, 200.0, 40.0));
    }
    runtime.set_collection_layout(root, layout).unwrap();

    let page_down = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("PageDown"),
        )
        .unwrap();
    let page_up = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[4], NativeEventKind::KeyDown).value("PageUp"),
        )
        .unwrap();

    assert_eq!(page_down.event.node, items[3]);
    assert_eq!(page_up.event.node, items[1]);
    assert_eq!(runtime.host().focused(), Some(items[1]));
}

#[test]
fn shift_extends_replace_selection_while_control_moves_only_focus() {
    let (mut runtime, root) = runtime_for(&collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "multiple",
        Some("replace"),
    ));
    let items = runtime.renderer.child_ids(root);
    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            items[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let extended = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown)
                .value("ArrowDown")
                .modifiers(NativeKeyModifiers::new().shift(true)),
        )
        .unwrap();
    let focus_only = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[1], NativeEventKind::KeyDown)
                .value("ArrowDown")
                .modifiers(NativeKeyModifiers::new().control(true)),
        )
        .unwrap();

    assert_eq!(extended.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(focus_only.event.kind, NativeEventKind::KeyDown);
    assert!(focus_only.invocations.is_empty());
    assert_eq!(runtime.host().focused(), Some(items[2]));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first"), CollectionKey::from("second")])
    );
}

#[test]
fn escape_key_behavior_none_preserves_selection_and_the_original_key_event() {
    let mut list = collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "multiple",
        None,
    );
    list.props.web = list.props.web.attribute("escapeKeyBehavior", "none");
    let (mut runtime, root) = runtime_for(&list);
    let items = runtime.renderer.child_ids(root);
    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            items[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert!(handled.invocations.is_empty());
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first")])
    );
}

#[test]
fn disallow_empty_selection_prevents_escape_from_clearing_the_last_item() {
    let mut list = collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "multiple",
        None,
    );
    list.props.web = list.props.web.attribute("disallowEmptySelection", "true");
    let (mut runtime, root) = runtime_for(&list);
    let items = runtime.renderer.child_ids(root);
    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            items[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert!(handled.invocations.is_empty());
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first")])
    );
}

#[test]
fn keyboard_selection_command_rolls_back_state_and_host_projection_on_action_failure() {
    let mut list = collection(
        NativeRole::ListBox,
        NativeRole::ListBoxItem,
        "multiple",
        None,
    );
    list.props.web = list
        .props
        .web
        .attribute("defaultSelectedKeys", r#"["first"]"#);
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime.render_native(&list).unwrap();
    let items = runtime.renderer.child_ids(root);

    let error = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap_err();

    assert!(error.to_string().contains("unregistered action selectItem"));
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first")])
    );
    assert!(
        runtime
            .host()
            .blueprint(items[0])
            .unwrap()
            .control_state
            .selected
    );
}

#[test]
fn tree_item_space_uses_the_shared_selection_path() {
    let (mut runtime, root) = runtime_for(&collection(
        NativeRole::Tree,
        NativeRole::TreeItem,
        "single",
        None,
    ));
    let item = runtime.renderer.child_ids(root)[0];

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(item, NativeEventKind::KeyDown).value("Space"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::SelectionChange);
    assert_eq!(handled.invocations[0].action, "selectItem");
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::keys([CollectionKey::from("first")])
    );
}

#[test]
fn tree_reuses_the_shared_select_all_and_escape_contract() {
    let tree = collection(NativeRole::Tree, NativeRole::TreeItem, "multiple", None);
    let (mut runtime, root) = runtime_for(&tree);
    let items = runtime.renderer.child_ids(root);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::KeyDown)
                .value("A")
                .modifiers(NativeKeyModifiers::new().meta(true)),
        )
        .unwrap();
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::All
    );

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[2], NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();
    assert_eq!(
        runtime.selections().manager(root).unwrap().selection(),
        &Selection::empty()
    );
}
