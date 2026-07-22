use super::*;
use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::NativeKeyModifiers;
use crate::native::{ElementKey, NativeElement, NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::style::TextDirection;
use crate::web::WebProps;

fn node(
    id: u64,
    parent: Option<u64>,
    key: &str,
    role: NativeRole,
    props: NativeProps,
) -> MountedNodeSnapshot {
    MountedNodeSnapshot {
        node: HostNodeId::new(id),
        parent: parent.map(HostNodeId::new),
        key: ElementKey::new(key),
        role,
        props,
    }
}

fn list(mode: &str) -> Vec<MountedNodeSnapshot> {
    vec![
        node(
            1,
            None,
            "people",
            NativeRole::ListBox,
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", mode)
                    .on_selection_change("selectPerson"),
            ),
        ),
        node(
            2,
            Some(1),
            "ada-key",
            NativeRole::ListBoxItem,
            NativeProps::new().value("Ada"),
        ),
        node(
            3,
            Some(1),
            "grace-key",
            NativeRole::ListBoxItem,
            NativeProps::new().value("Grace").disabled(true),
        ),
        node(
            4,
            Some(1),
            "linus-key",
            NativeRole::ListBoxItem,
            NativeProps::new().value("Linus"),
        ),
    ]
}

#[test]
fn mounted_selection_uses_element_keys_and_only_aliases_native_values() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("single")).unwrap();

    let update = registry
        .apply_event(
            &NativeEvent::new(HostNodeId::new(1), NativeEventKind::SelectionChange).value("Ada"),
        )
        .unwrap()
        .unwrap();

    assert!(update.changed);
    assert_eq!(update.event_value(), "ada-key");
    assert_eq!(
        registry.key_for_item(HostNodeId::new(2)).unwrap().as_str(),
        "ada-key"
    );
    assert_eq!(
        registry.manager(HostNodeId::new(1)).unwrap().selection(),
        &Selection::keys([CollectionKey::from("ada-key")])
    );
}

#[test]
fn native_projection_marks_and_unmarks_items_owned_by_collection_actions() {
    let mut native = NativeElement::new("people", NativeRole::ListBox)
        .with_props(NativeProps::new().web(WebProps::new().event("onAction", "openPerson")))
        .child(NativeElement::new("ada", NativeRole::ListBoxItem));
    let registry = MountedSelectionRegistry::new();

    registry.project_native_tree(&mut native);
    assert_eq!(
        native.children[0]
            .props
            .metadata
            .get(COLLECTION_ACTION_METADATA_KEY)
            .map(String::as_str),
        Some("true")
    );

    native.props.web.events.remove("onAction");
    registry.project_native_tree(&mut native);
    assert!(!native.children[0]
        .props
        .metadata
        .contains_key(COLLECTION_ACTION_METADATA_KEY));
}

#[test]
fn multiple_selection_toggles_keys_and_ignores_disabled_items() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("multiple")).unwrap();

    let first = registry
        .apply_event(&NativeEvent::new(
            HostNodeId::new(2),
            NativeEventKind::SelectionChange,
        ))
        .unwrap()
        .unwrap();
    let disabled = registry
        .apply_event(&NativeEvent::new(
            HostNodeId::new(3),
            NativeEventKind::SelectionChange,
        ))
        .unwrap()
        .unwrap();
    let second = registry
        .apply_event(&NativeEvent::new(
            HostNodeId::new(4),
            NativeEventKind::SelectionChange,
        ))
        .unwrap()
        .unwrap();

    assert!(first.changed);
    assert!(!disabled.changed);
    assert!(second.changed);
    assert_eq!(second.event_value(), r#"["ada-key","linus-key"]"#);
}

#[test]
fn shift_selection_extends_from_the_stable_anchor_and_skips_disabled_items() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("multiple")).unwrap();
    registry
        .apply_event(&NativeEvent::new(
            HostNodeId::new(2),
            NativeEventKind::SelectionChange,
        ))
        .unwrap();

    let update = registry
        .apply_event(
            &NativeEvent::new(HostNodeId::new(4), NativeEventKind::SelectionChange)
                .modifiers(NativeKeyModifiers::new().shift(true)),
        )
        .unwrap()
        .unwrap();

    assert_eq!(update.event_value(), r#"["ada-key","linus-key"]"#);
}

#[test]
fn controlled_all_selection_survives_newly_loaded_items() {
    let mut snapshot = list("multiple");
    snapshot[0].props.web = snapshot[0]
        .props
        .web
        .clone()
        .attribute("selectedKeys", "\"all\"");
    snapshot[0].props.metadata = snapshot[0].props.web.metadata();
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&snapshot).unwrap();

    snapshot.push(node(
        5,
        Some(1),
        "new-key",
        NativeRole::ListBoxItem,
        NativeProps::new().value("New"),
    ));
    registry.sync(&snapshot).unwrap();

    let manager = registry.manager(HostNodeId::new(1)).unwrap();
    assert_eq!(manager.selection(), &Selection::All);
    assert!(manager.is_selected(&CollectionKey::from("new-key")));
}

#[test]
fn tab_items_are_owned_by_tabs_instead_of_the_structural_tab_list() {
    let snapshot = vec![
        node(
            10,
            None,
            "tabs",
            NativeRole::Tabs,
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectTab"),
            ),
        ),
        node(
            11,
            Some(10),
            "list",
            NativeRole::TabList,
            NativeProps::new(),
        ),
        node(
            12,
            Some(11),
            "settings",
            NativeRole::Tab,
            NativeProps::new().value("Settings"),
        ),
    ];
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&snapshot).unwrap();

    assert_eq!(
        registry.collection_for_item(HostNodeId::new(12)),
        Some(HostNodeId::new(10))
    );
    assert!(registry.manager(HostNodeId::new(11)).is_some());
    assert!(registry
        .manager(HostNodeId::new(11))
        .unwrap()
        .selected_loaded_keys()
        .is_empty());
}

#[test]
fn keyboard_navigation_skips_disabled_items_and_follows_replace_selection() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("single")).unwrap();

    let navigation = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(2), NativeEventKind::KeyDown).value("Down"),
            TextDirection::Ltr,
        )
        .unwrap();

    assert_eq!(navigation.target, HostNodeId::new(4));
    assert!(navigation.select);
    assert!(registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(4), NativeEventKind::KeyDown).value("ArrowDown"),
            TextDirection::Ltr,
        )
        .is_none());
}

#[test]
fn toggle_selection_navigation_moves_focus_without_selecting() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("multiple")).unwrap();

    let navigation = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(2), NativeEventKind::KeyDown).value("ArrowDown"),
            TextDirection::Ltr,
        )
        .unwrap();

    assert_eq!(navigation.target, HostNodeId::new(4));
    assert!(!navigation.select);
}

#[test]
fn collection_keyboard_commands_produce_complete_select_all_and_clear_snapshots() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&list("multiple")).unwrap();

    let select_all = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(2), NativeEventKind::KeyDown)
                .value("a")
                .modifiers(NativeKeyModifiers::new().control(true)),
            TextDirection::Ltr,
        )
        .unwrap()
        .selection
        .unwrap();

    assert_eq!(select_all.collection, HostNodeId::new(1));
    assert_eq!(select_all.selection, Selection::All);
    assert_eq!(select_all.event_value(), r#""all""#);
    registry
        .apply_event(
            &NativeEvent::new(HostNodeId::new(1), NativeEventKind::SelectionChange)
                .value(select_all.event_value()),
        )
        .unwrap();
    assert_eq!(
        registry.manager(HostNodeId::new(1)).unwrap().selection(),
        &Selection::All
    );

    let clear = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(4), NativeEventKind::KeyDown).value("Escape"),
            TextDirection::Ltr,
        )
        .unwrap()
        .selection
        .unwrap();

    assert_eq!(clear.selection, Selection::empty());
    assert_eq!(clear.event_value(), "[]");
}

#[test]
fn escape_key_behavior_none_leaves_the_key_unhandled() {
    let mut snapshot = list("multiple");
    snapshot[0].props.web = snapshot[0]
        .props
        .web
        .clone()
        .attribute("escapeKeyBehavior", "none");
    snapshot[0].props.metadata = snapshot[0].props.web.metadata();
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&snapshot).unwrap();

    assert!(registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(2), NativeEventKind::KeyDown).value("Escape"),
            TextDirection::Ltr,
        )
        .is_none());
}

#[test]
fn automatic_tabs_wrap_and_mirror_horizontal_navigation_in_rtl() {
    let snapshot = vec![
        node(
            10,
            None,
            "tabs",
            NativeRole::Tabs,
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .on_selection_change("selectTab"),
            ),
        ),
        node(
            11,
            Some(10),
            "list",
            NativeRole::TabList,
            NativeProps::new().orientation(crate::geometry::Orientation::Horizontal),
        ),
        node(
            12,
            Some(11),
            "settings",
            NativeRole::Tab,
            NativeProps::new().value("Settings"),
        ),
        node(
            13,
            Some(11),
            "account",
            NativeRole::Tab,
            NativeProps::new().value("Account"),
        ),
    ];
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&snapshot).unwrap();

    let next = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(12), NativeEventKind::KeyDown).value("ArrowLeft"),
            TextDirection::Rtl,
        )
        .unwrap();
    let wrapped = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(13), NativeEventKind::KeyDown).value("ArrowLeft"),
            TextDirection::Rtl,
        )
        .unwrap();

    assert_eq!(next.target, HostNodeId::new(13));
    assert!(next.select);
    assert_eq!(wrapped.target, HostNodeId::new(12));
}

#[test]
fn manual_tabs_move_focus_without_selecting() {
    let snapshot = vec![
        node(
            10,
            None,
            "tabs",
            NativeRole::Tabs,
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .attribute("keyboardActivation", "manual"),
            ),
        ),
        node(
            11,
            Some(10),
            "list",
            NativeRole::TabList,
            NativeProps::new().orientation(crate::geometry::Orientation::Horizontal),
        ),
        node(
            12,
            Some(11),
            "settings",
            NativeRole::Tab,
            NativeProps::new(),
        ),
        node(13, Some(11), "account", NativeRole::Tab, NativeProps::new()),
    ];
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&snapshot).unwrap();

    let navigation = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(12), NativeEventKind::KeyDown).value("ArrowRight"),
            TextDirection::Ltr,
        )
        .unwrap();

    assert_eq!(navigation.target, HostNodeId::new(13));
    assert!(!navigation.select);
}

fn tree() -> Vec<MountedNodeSnapshot> {
    vec![
        node(
            20,
            None,
            "files",
            NativeRole::Tree,
            NativeProps::new().web(
                WebProps::new()
                    .attribute("data-selection-mode", "single")
                    .attribute("defaultExpandedKeys", r#"["documents"]"#)
                    .on_selection_change("selectFile")
                    .event("onExpandedChange", "setExpanded"),
            ),
        ),
        node(
            21,
            Some(20),
            "documents",
            NativeRole::TreeItem,
            NativeProps::new().label("Documents").web(
                WebProps::new()
                    .attribute("data-tree-level", "1")
                    .attribute("data-has-child-items", "true"),
            ),
        ),
        node(
            22,
            Some(20),
            "resume",
            NativeRole::TreeItem,
            NativeProps::new().label("Resume").web(
                WebProps::new()
                    .attribute("data-tree-level", "2")
                    .attribute("data-tree-parent-key", "documents"),
            ),
        ),
        node(
            23,
            Some(20),
            "photos",
            NativeRole::TreeItem,
            NativeProps::new()
                .label("Photos")
                .web(WebProps::new().attribute("data-tree-level", "1")),
        ),
    ]
}

#[test]
fn tree_navigation_only_visits_expanded_descendants() {
    let mut registry = MountedSelectionRegistry::new();
    registry.sync(&tree()).unwrap();

    let child = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowDown"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(child.target, HostNodeId::new(22));

    let collapse = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowLeft"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(collapse.target, HostNodeId::new(21));
    assert_eq!(collapse.expansion.unwrap().event_value(), "[]");

    let sibling = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowDown"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(sibling.target, HostNodeId::new(23));

    let typeahead = registry.keyboard_navigation(
        &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("r"),
        TextDirection::Ltr,
    );
    assert!(typeahead.is_none());
}

#[test]
fn tree_horizontal_arrows_expand_enter_and_return_to_parent_in_ltr_and_rtl() {
    let mut registry = MountedSelectionRegistry::new();
    let mut collapsed = tree();
    collapsed[0]
        .props
        .web
        .attributes
        .remove("defaultExpandedKeys");
    collapsed[0].props.metadata = collapsed[0].props.web.metadata();
    registry.sync(&collapsed).unwrap();

    let expand = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowRight"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(expand.target, HostNodeId::new(21));
    assert_eq!(expand.expansion.unwrap().event_value(), r#"["documents"]"#);

    let enter = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowRight"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(enter.target, HostNodeId::new(22));

    let parent = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(22), NativeEventKind::KeyDown).value("ArrowLeft"),
            TextDirection::Ltr,
        )
        .unwrap();
    assert_eq!(parent.target, HostNodeId::new(21));

    let rtl_collapse = registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowRight"),
            TextDirection::Rtl,
        )
        .unwrap();
    assert_eq!(rtl_collapse.target, HostNodeId::new(21));
    assert_eq!(rtl_collapse.expansion.unwrap().event_value(), "[]");
}

#[test]
fn tree_expansion_preserves_uncontrolled_state_and_resyncs_controlled_keys() {
    let mut registry = MountedSelectionRegistry::new();
    let snapshot = tree();
    registry.sync(&snapshot).unwrap();
    registry
        .keyboard_navigation(
            &NativeEvent::new(HostNodeId::new(21), NativeEventKind::KeyDown).value("ArrowLeft"),
            TextDirection::Ltr,
        )
        .unwrap();

    registry.sync(&snapshot).unwrap();
    let projection = registry.tree_projections().pop().unwrap();
    assert!(projection.expanded_keys.is_empty());
    assert!(
        projection
            .items
            .iter()
            .find(|item| item.node == HostNodeId::new(22))
            .unwrap()
            .hidden
    );

    let mut controlled = snapshot;
    controlled[0].props.web = controlled[0]
        .props
        .web
        .clone()
        .attribute("expandedKeys", r#"["documents"]"#);
    controlled[0].props.metadata = controlled[0].props.web.metadata();
    registry.sync(&controlled).unwrap();
    let projection = registry.tree_projections().pop().unwrap();
    assert_eq!(
        projection.expanded_keys,
        BTreeSet::from([CollectionKey::from("documents")])
    );
    assert!(
        !projection
            .items
            .iter()
            .find(|item| item.node == HostNodeId::new(22))
            .unwrap()
            .hidden
    );
}
