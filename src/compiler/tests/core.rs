use super::support::*;

#[test]
fn lowers_compiled_react_aria_button_json_to_native_button() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r##"
        {
          "kind": "element",
          "key": "save",
          "tag": "Button",
          "importSource": "react-aria-components",
          "props": {
            "className": "primary",
            "style": {"minWidth": 280, "backgroundColor": "#663399"},
            "attributes": {"aria-label": "Save document", "data-testid": "save-button"},
            "events": {"onClick": "saveDocument"},
            "actionLabels": {"saveDocument": "Save document"}
          },
          "children": [
            {"kind": "text", "key": "save-text", "value": "Save"}
          ]
        }
        "##,
    )
    .unwrap();

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::Button);
    assert_eq!(native.props.label.as_deref(), Some("Save document"));
    assert_eq!(native.props.action.as_deref(), Some("saveDocument"));
    assert_eq!(
        native.props.web.style.get("minWidth").map(String::as_str),
        Some("280")
    );
    assert_eq!(
        native.props.metadata.get("data-testid").map(String::as_str),
        Some("save-button")
    );
}

#[test]
fn lowers_intrinsic_form_text_field_shape_to_native_text_field() {
    let compiled = CompiledJsxNode::Element {
        key: "email-field".to_string(),
        tag: "TextField".to_string(),
        import_source: Some("react-aria-components".to_string()),
        props: CompiledProps {
            is_required: true,
            ..CompiledProps::default()
        },
        children: vec![
            CompiledJsxNode::Element {
                key: "email-label".to_string(),
                tag: "Label".to_string(),
                import_source: Some("react-aria-components".to_string()),
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "email-label-text".to_string(),
                    value: "Email".to_string(),
                }],
            },
            CompiledJsxNode::Element {
                key: "email-input".to_string(),
                tag: "input".to_string(),
                import_source: None,
                props: CompiledProps {
                    placeholder: Some("you@example.com".to_string()),
                    value: Some("a@b.c".to_string()),
                    events: BTreeMap::from([("onChange".to_string(), "setEmail".to_string())]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
        ],
    };

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.label.as_deref(), Some("Email"));
    assert_eq!(native.props.placeholder.as_deref(), Some("you@example.com"));
    assert!(native.props.required);
}

#[test]
fn lowers_web_and_aria_attribute_aliases_to_native_control_state() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "volume",
          "tag": "Slider",
          "props": {
            "attributes": {
              "aria-label": "Volume",
              "aria-disabled": "true",
              "aria-required": "true",
              "aria-invalid": "spelling",
              "aria-readonly": "true",
              "aria-selected": "true",
              "aria-expanded": "true",
              "aria-placeholder": "Volume",
              "aria-orientation": "horizontal",
              "aria-valuemin": "0",
              "aria-valuemax": "100",
              "aria-valuenow": "50",
              "aria-labelledby": "volume-label",
              "aria-describedby": "volume-help",
              "aria-controls": "volume-output",
              "aria-description": "Volume in percent",
              "aria-roledescription": "volume slider",
              "aria-keyshortcuts": "Alt+ArrowUp",
              "aria-valuetext": "Half volume",
              "aria-level": "2",
              "aria-posinset": "3",
              "aria-setsize": "10",
              "aria-rowcount": "20",
              "aria-rowindex": "4",
              "aria-rowspan": "2",
              "aria-colcount": "6",
              "aria-colindex": "5",
              "aria-colspan": "3",
              "aria-rowindextext": "Row four",
              "aria-colindextext": "Column five",
              "aria-sort": "ascending",
              "aria-hidden": "false",
              "aria-autocomplete": "list",
              "aria-multiline": "true",
              "aria-current": "page",
              "aria-haspopup": "dialog",
              "aria-pressed": "mixed",
              "aria-live": "polite",
              "aria-atomic": "true",
              "aria-busy": "false",
              "aria-relevant": "additions text",
              "aria-modal": "true"
            }
          }
        }
        "#,
    )
    .unwrap();

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::Slider);
    assert_eq!(native.props.label.as_deref(), Some("Volume"));
    assert!(native.props.disabled);
    assert!(native.props.required);
    assert!(native.props.invalid);
    assert!(native.props.read_only);
    assert!(native.props.selected);
    assert_eq!(native.props.expanded, Some(true));
    assert_eq!(native.props.placeholder.as_deref(), Some("Volume"));
    assert_eq!(native.props.orientation, Some(Orientation::Horizontal));
    assert_eq!(native.props.min, Some(0.0));
    assert_eq!(native.props.max, Some(100.0));
    assert_eq!(native.props.current, Some(50.0));
    assert_eq!(
        native
            .props
            .accessibility_relationships
            .labelled_by
            .as_deref(),
        Some("volume-label")
    );
    assert_eq!(
        native
            .props
            .accessibility_relationships
            .described_by
            .as_deref(),
        Some("volume-help")
    );
    assert_eq!(
        native.props.accessibility_relationships.controls.as_deref(),
        Some("volume-output")
    );
    assert_eq!(
        native
            .props
            .accessibility_description
            .description
            .as_deref(),
        Some("Volume in percent")
    );
    assert_eq!(
        native
            .props
            .accessibility_description
            .role_description
            .as_deref(),
        Some("volume slider")
    );
    assert_eq!(
        native
            .props
            .accessibility_description
            .key_shortcuts
            .as_deref(),
        Some("Alt+ArrowUp")
    );
    assert_eq!(
        native.props.accessibility_description.value_text.as_deref(),
        Some("Half volume")
    );
    assert_eq!(native.props.accessibility_structure.level, Some(2));
    assert_eq!(
        native.props.accessibility_structure.position_in_set,
        Some(3)
    );
    assert_eq!(native.props.accessibility_structure.set_size, Some(10));
    assert_eq!(native.props.accessibility_structure.row_count, Some(20));
    assert_eq!(native.props.accessibility_structure.row_index, Some(4));
    assert_eq!(native.props.accessibility_structure.row_span, Some(2));
    assert_eq!(native.props.accessibility_structure.column_count, Some(6));
    assert_eq!(native.props.accessibility_structure.column_index, Some(5));
    assert_eq!(native.props.accessibility_structure.column_span, Some(3));
    assert_eq!(
        native
            .props
            .accessibility_structure
            .row_index_text
            .as_deref(),
        Some("Row four")
    );
    assert_eq!(
        native
            .props
            .accessibility_structure
            .column_index_text
            .as_deref(),
        Some("Column five")
    );
    assert_eq!(
        native.props.accessibility_structure.sort.as_deref(),
        Some("ascending")
    );
    assert_eq!(native.props.accessibility_state.hidden, Some(false));
    assert_eq!(
        native.props.accessibility_state.autocomplete.as_deref(),
        Some("list")
    );
    assert_eq!(native.props.accessibility_state.multiline, Some(true));
    assert_eq!(
        native.props.accessibility_state.current.as_deref(),
        Some("page")
    );
    assert_eq!(
        native.props.accessibility_state.has_popup.as_deref(),
        Some("dialog")
    );
    assert_eq!(
        native.props.accessibility_state.pressed.as_deref(),
        Some("mixed")
    );
    assert_eq!(
        native.props.accessibility_state.live.as_deref(),
        Some("polite")
    );
    assert_eq!(native.props.accessibility_state.atomic, Some(true));
    assert_eq!(native.props.accessibility_state.busy, Some(false));
    assert_eq!(
        native.props.accessibility_state.relevant.as_deref(),
        Some("additions text")
    );
    assert_eq!(native.props.accessibility_state.modal, Some(true));
}

#[test]
fn lowers_radio_group_and_radios_to_native_selection_controls() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "theme",
          "tag": "RadioGroup",
          "props": {
            "label": "Theme",
            "events": {"onChange": "setTheme"}
          },
          "children": [
            {
              "kind": "element",
              "key": "light",
              "tag": "Radio",
              "props": {"textValue": "Light", "value": "light"}
            },
            {
              "kind": "element",
              "key": "dark",
              "tag": "Radio",
              "props": {
                "textValue": "Dark",
                "value": "dark",
                "isSelected": true
              }
            }
          ]
        }
        "#,
    )
    .unwrap();

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::RadioGroup);
    assert_eq!(native.props.label.as_deref(), Some("Theme"));
    assert_eq!(native.props.action.as_deref(), Some("setTheme"));
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[1].role, NativeRole::Radio);
    assert_eq!(native.children[1].props.label.as_deref(), Some("Dark"));
    assert_eq!(native.children[1].props.value.as_deref(), Some("dark"));
    assert_eq!(native.children[1].props.checked, Some(true));
}

#[test]
fn folds_compiled_tabs_into_native_tab_items_with_panels() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "settings",
          "tag": "Tabs",
          "props": {"events": {"onSelectionChange": "setTab"}},
          "children": [
            {
              "kind": "element",
              "key": "settings-tabs",
              "tag": "TabList",
              "children": [
                {
                  "kind": "element",
                  "key": "profile-tab",
                  "tag": "Tab",
                  "props": {"textValue": "Profile", "isSelected": true}
                },
                {
                  "kind": "element",
                  "key": "billing-tab",
                  "tag": "Tab",
                  "props": {"textValue": "Billing"}
                }
              ]
            },
            {
              "kind": "element",
              "key": "profile-panel",
              "tag": "TabPanel",
              "children": [
                {"kind": "text", "key": "profile-title", "value": "Profile settings"}
              ]
            },
            {
              "kind": "element",
              "key": "billing-panel",
              "tag": "TabPanel",
              "children": [
                {"kind": "text", "key": "billing-title", "value": "Billing settings"}
              ]
            }
          ]
        }
        "#,
    )
    .unwrap();

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::Tabs);
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setTab")
    );
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[0].role, NativeRole::Tab);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Profile"));
    assert!(native.children[0].props.selected);
    assert_eq!(native.children[0].children[0].role, NativeRole::TabPanel);
    assert_eq!(
        native.children[0].children[0].children[0]
            .props
            .label
            .as_deref(),
        Some("Profile settings")
    );
}

#[test]
fn lowers_compiled_menu_to_native_menu_items() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "file-menu",
          "tag": "Menu",
          "children": [
            {
              "kind": "element",
              "key": "open",
              "tag": "MenuItem",
              "props": {
                "value": "open",
                "events": {"onPress": "openFile"}
              },
              "children": [{"kind": "text", "key": "open-text", "value": "Open"}]
            }
          ]
        }
        "#,
    )
    .unwrap();

    let native = ReactCompilerBridge::new()
        .lower_to_native(&compiled)
        .unwrap();

    assert_eq!(native.role, NativeRole::Menu);
    assert_eq!(native.children.len(), 1);
    assert_eq!(native.children[0].role, NativeRole::MenuItem);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Open"));
    assert_eq!(native.children[0].props.value.as_deref(), Some("open"));
    assert_eq!(native.children[0].props.action.as_deref(), Some("openFile"));
}
