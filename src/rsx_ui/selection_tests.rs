use crate::compiler::{CompiledRsxNode, RsxCompilerBridge};
use crate::native::NativeRole;
use crate::rsx_app::RsxComponent;

#[test]
fn builtin_list_box_remains_uncontrolled_when_no_selection_value_is_supplied() {
    let component = RsxComponent::<()>::new(
        "uncontrolled-list-box",
        r#"
        <UiListBox key="items" selectionMode="multiple">
          <UiListBoxItem key="alpha">Alpha</UiListBoxItem>
        </UiListBox>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let list_box = find_by_slot(&frame.root, "list-box").unwrap();
    let CompiledRsxNode::Element { props, .. } = list_box else {
        panic!("list box element");
    };

    assert!(props.value.is_none());
    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("selectionBehavior")
            .map(String::as_str),
        Some("toggle")
    );
    assert_eq!(
        props.attributes.get("disabledBehavior").map(String::as_str),
        Some("all")
    );
}

#[test]
fn builtin_list_box_forwards_the_complete_selection_option_surface() {
    let component = RsxComponent::<()>::new(
        "configured-list-box",
        r#"
        <UiListBox
          key="items"
          defaultSelectedKeys={state.defaults}
          disabledKeys={state.disabled}
          selectionMode="multiple"
          selectionBehavior="replace"
          disabledBehavior="selection"
          disallowEmptySelection={true}
          shouldFocusWrap={true}
          escapeKeyBehavior="none"
          onAction={openItem}
        >
          <UiListBoxItem key="alpha">Alpha</UiListBoxItem>
          <UiListBoxItem key="beta">Beta</UiListBoxItem>
        </UiListBox>
        "#,
    )
    .unwrap()
    .use_state("defaults", |_| vec!["beta"])
    .use_state("disabled", |_| vec!["alpha"])
    .use_reducer("openItem", |_state, _invocation| Ok(()));

    let frame = component.render(&()).unwrap();
    let list_box = find_by_slot(&frame.root, "list-box").unwrap();
    let CompiledRsxNode::Element { props, .. } = list_box else {
        panic!("list box element");
    };

    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("defaultSelectedKeys")
            .map(String::as_str),
        Some(r#"["beta"]"#)
    );
    assert_eq!(
        props.attributes.get("disabledKeys").map(String::as_str),
        Some(r#"["alpha"]"#)
    );
    assert_eq!(
        props
            .attributes
            .get("selectionBehavior")
            .map(String::as_str),
        Some("replace")
    );
    assert_eq!(
        props.attributes.get("disabledBehavior").map(String::as_str),
        Some("selection")
    );
    assert_eq!(
        props
            .attributes
            .get("disallowEmptySelection")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("shouldFocusWrap").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("escapeKeyBehavior")
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        props.events.get("onAction").map(String::as_str),
        Some("openItem")
    );
}

#[test]
fn builtin_tabs_forward_manual_keyboard_activation() {
    let component = RsxComponent::<()>::new(
        "manual-tabs",
        r#"
        <UiTabs key="tabs" keyboardActivation="manual">
          <UiTabsList key="list" orientation="horizontal">
            <UiTab key="first" value="first">First</UiTab>
            <UiTab key="second" value="second">Second</UiTab>
          </UiTabsList>
        </UiTabs>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let tabs = find_by_slot(&frame.root, "tabs").unwrap();
    let CompiledRsxNode::Element { props, .. } = tabs else {
        panic!("tabs element");
    };

    assert_eq!(
        props
            .attributes
            .get("keyboardActivation")
            .map(String::as_str),
        Some("manual")
    );
}

#[test]
fn builtin_tree_uses_the_same_key_set_selection_contract() {
    let component = RsxComponent::<()>::new(
        "configured-tree",
        r#"
        <UiTree
          key="files"
          defaultSelectedKeys={state.defaults}
          defaultExpandedKeys={state.expanded}
          disabledKeys={state.disabled}
          selectionMode="multiple"
          selectionBehavior="replace"
          disallowEmptySelection={true}
          escapeKeyBehavior="none"
          onAction={openFile}
          onExpandedChange={setExpanded}
        >
          <UiTreeItem key="src">src</UiTreeItem>
          <UiTreeItem key="target">target</UiTreeItem>
        </UiTree>
        "#,
    )
    .unwrap()
    .use_state("defaults", |_| vec!["src"])
    .use_state("expanded", |_| vec!["src"])
    .use_state("disabled", |_| vec!["target"])
    .use_reducer("openFile", |_state, _invocation| Ok(()))
    .use_reducer("setExpanded", |_state, _invocation| Ok(()));

    let frame = component.render(&()).unwrap();
    let tree = find_by_slot(&frame.root, "tree").unwrap();
    let CompiledRsxNode::Element { props, .. } = tree else {
        panic!("tree element");
    };

    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("defaultSelectedKeys")
            .map(String::as_str),
        Some(r#"["src"]"#)
    );
    assert_eq!(
        props.attributes.get("disabledKeys").map(String::as_str),
        Some(r#"["target"]"#)
    );
    assert_eq!(
        props
            .attributes
            .get("defaultExpandedKeys")
            .map(String::as_str),
        Some(r#"["src"]"#)
    );
    assert_eq!(
        props.events.get("onExpandedChange").map(String::as_str),
        Some("setExpanded")
    );
    assert_eq!(
        props.events.get("onAction").map(String::as_str),
        Some("openFile")
    );
    assert_eq!(
        props
            .attributes
            .get("selectionBehavior")
            .map(String::as_str),
        Some("replace")
    );
    assert_eq!(
        props
            .attributes
            .get("disallowEmptySelection")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("escapeKeyBehavior")
            .map(String::as_str),
        Some("none")
    );
}

#[test]
fn builtin_tree_lowers_nested_items_to_hierarchical_flat_rows() {
    let component = RsxComponent::<()>::new(
        "nested-tree",
        r#"
        <UiTree key="files" label="Files">
          <UiTreeItem key="documents" textValue="Documents">
            <UiTreeItemContent key="documents-content">Documents</UiTreeItemContent>
            <UiTreeItem key="resume" textValue="Resume">
              <UiTreeItemContent key="resume-content">Resume</UiTreeItemContent>
            </UiTreeItem>
          </UiTreeItem>
          <UiTreeItem key="photos" textValue="Photos">Photos</UiTreeItem>
        </UiTree>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let tree = find_by_slot(&frame.root, "tree").unwrap();
    let native = RsxCompilerBridge::new().lower_to_native(tree).unwrap();

    assert_eq!(native.role, NativeRole::Tree);
    assert_eq!(native.children.len(), 3);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Documents"));
    assert_eq!(native.children[1].props.label.as_deref(), Some("Resume"));
    assert_eq!(native.children[2].props.label.as_deref(), Some("Photos"));
    assert_eq!(native.children[0].props.expanded, Some(false));
    assert_eq!(
        native.children[1]
            .props
            .metadata
            .get("data-tree-parent-key")
            .map(String::as_str),
        Some(native.children[0].key.as_str())
    );
}

#[test]
fn builtin_select_reuses_collection_selection_without_forcing_controlled_state() {
    let component = RsxComponent::<()>::new(
        "configured-select",
        r#"
        <UiSelect
          key="density"
          label="Density"
          defaultSelectedKeys={state.defaults}
          disabledKeys={state.disabled}
          disabledBehavior="selection"
          disallowEmptySelection={true}
        >
          <UiListBoxItem key="compact">Compact</UiListBoxItem>
          <UiListBoxItem key="comfortable">Comfortable</UiListBoxItem>
        </UiSelect>
        "#,
    )
    .unwrap()
    .use_state("defaults", |_| vec!["comfortable"])
    .use_state("disabled", |_| vec!["compact"]);

    let frame = component.render(&()).unwrap();
    let select = find_by_slot(&frame.root, "select").unwrap();
    let CompiledRsxNode::Element { props, .. } = select else {
        panic!("select element");
    };

    assert!(props.value.is_none());
    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("defaultSelectedKeys")
            .map(String::as_str),
        Some(r#"["comfortable"]"#)
    );
    assert_eq!(
        props.attributes.get("disabledKeys").map(String::as_str),
        Some(r#"["compact"]"#)
    );
    assert_eq!(
        props.attributes.get("disabledBehavior").map(String::as_str),
        Some("selection")
    );
}

#[test]
fn builtin_menu_uses_stable_key_selection_options() {
    let component = RsxComponent::<()>::new(
        "configured-menu",
        r#"
        <UiMenu
          key="actions"
          label="Actions"
          defaultSelectedKeys={state.defaults}
          selectionMode="multiple"
          selectionBehavior="toggle"
        >
          <UiMenuItem key="copy" textValue="Copy">Copy</UiMenuItem>
          <UiMenuItem key="paste" textValue="Paste">Paste</UiMenuItem>
        </UiMenu>
        "#,
    )
    .unwrap()
    .use_state("defaults", |_| vec!["copy"]);

    let frame = component.render(&()).unwrap();
    let menu = find_by_slot(&frame.root, "menu").unwrap();
    let CompiledRsxNode::Element { props, .. } = menu else {
        panic!("menu element");
    };

    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("defaultSelectedKeys")
            .map(String::as_str),
        Some(r#"["copy"]"#)
    );
    assert_eq!(
        props
            .attributes
            .get("selectionBehavior")
            .map(String::as_str),
        Some("toggle")
    );
}

#[test]
fn builtin_radio_group_keeps_default_value_uncontrolled() {
    let component = RsxComponent::<()>::new(
        "default-radio-group",
        r#"
        <UiRadioGroup key="theme" label="Theme" defaultValue="dark">
          <UiRadio key="light" value="light">Light</UiRadio>
          <UiRadio key="dark" value="dark">Dark</UiRadio>
        </UiRadioGroup>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let group = find_by_slot(&frame.root, "radio-group").unwrap();
    let CompiledRsxNode::Element { props, .. } = group else {
        panic!("radio group element");
    };

    assert!(props.value.is_none());
    assert_eq!(
        props.attributes.get("defaultValue").map(String::as_str),
        Some("dark")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selected-value")
            .map(String::as_str),
        Some("dark")
    );
}

#[test]
fn color_swatch_picker_reuses_the_collection_selection_contract() {
    let component = RsxComponent::<()>::new(
        "default-swatch-picker",
        r##"
        <UiColorSwatchPicker
          key="colors"
          defaultSelectedKeys={state.defaults}
          selectionMode="multiple"
        >
          <UiColorSwatchPickerItem key="red" value="#f00" />
          <UiColorSwatchPickerItem key="blue" value="#00f" />
        </UiColorSwatchPicker>
        "##,
    )
    .unwrap()
    .use_state("defaults", |_| vec!["red"]);

    let frame = component.render(&()).unwrap();
    let picker = find_by_slot(&frame.root, "color-swatch-picker").unwrap();
    let CompiledRsxNode::Element { props, .. } = picker else {
        panic!("color swatch picker element");
    };

    assert!(!props.attributes.contains_key("selectedKeys"));
    assert_eq!(
        props
            .attributes
            .get("defaultSelectedKeys")
            .map(String::as_str),
        Some(r#"["red"]"#)
    );
    assert_eq!(
        props
            .attributes
            .get("selectionBehavior")
            .map(String::as_str),
        Some("toggle")
    );
}

fn find_by_slot<'a>(node: &'a CompiledRsxNode, slot: &str) -> Option<&'a CompiledRsxNode> {
    match node {
        CompiledRsxNode::Element {
            props, children, ..
        } => {
            if props.attributes.get("data-slot").map(String::as_str) == Some(slot) {
                return Some(node);
            }
            children.iter().find_map(|child| find_by_slot(child, slot))
        }
        CompiledRsxNode::Text { .. } => None,
    }
}
