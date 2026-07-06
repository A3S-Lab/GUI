use super::support::*;

#[test]
fn lowers_compiled_semantic_ui_button_json_to_native_button() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r##"
        {
          "kind": "element",
          "key": "save",
          "tag": "Button",
          "importSource": "a3s-rsx",
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

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
fn rejects_unstable_compiled_node_identities() {
    let duplicate_child_keys: CompiledRsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "toolbar",
          "tag": "Toolbar",
          "children": [
            {"kind": "element", "key": "save", "tag": "Button"},
            {"kind": "text", "key": "save", "value": "Save"}
          ]
        }
        "#,
    )
    .unwrap();
    let empty_tag = CompiledRsxNode::Element {
        key: "empty-tag".to_string(),
        tag: String::new(),
        import_source: None,
        props: CompiledProps::default(),
        children: Vec::new(),
    };

    let bridge = RsxCompilerBridge::new();
    let duplicate_error = bridge.lower_to_native(&duplicate_child_keys).unwrap_err();
    let empty_tag_error = bridge.lower_to_native(&empty_tag).unwrap_err();

    assert!(duplicate_error
        .to_string()
        .contains("sibling nodes need unique keys"));
    assert!(empty_tag_error
        .to_string()
        .contains("compiled elements need non-empty tags"));
}

#[test]
fn lowers_intrinsic_form_text_field_shape_to_native_text_field() {
    let compiled = CompiledRsxNode::Element {
        key: "email-field".to_string(),
        tag: "TextField".to_string(),
        import_source: Some("a3s-rsx".to_string()),
        props: CompiledProps {
            is_required: true,
            ..CompiledProps::default()
        },
        children: vec![
            CompiledRsxNode::Element {
                key: "email-label".to_string(),
                tag: "Label".to_string(),
                import_source: Some("a3s-rsx".to_string()),
                props: CompiledProps::default(),
                children: vec![CompiledRsxNode::Text {
                    key: "email-label-text".to_string(),
                    value: "Email".to_string(),
                }],
            },
            CompiledRsxNode::Element {
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.label.as_deref(), Some("Email"));
    assert_eq!(native.props.placeholder.as_deref(), Some("you@example.com"));
    assert!(native.props.required);
}

#[test]
fn lowers_web_and_aria_attribute_aliases_to_native_control_state() {
    let compiled: CompiledRsxNode = serde_json::from_str(
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

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
    let compiled: CompiledRsxNode = serde_json::from_str(
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

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
    let compiled: CompiledRsxNode = serde_json::from_str(
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

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
    let compiled: CompiledRsxNode = serde_json::from_str(
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

    let native = RsxCompilerBridge::new().lower_to_native(&compiled).unwrap();

    assert_eq!(native.role, NativeRole::Menu);
    assert_eq!(native.children.len(), 1);
    assert_eq!(native.children[0].role, NativeRole::MenuItem);
    assert_eq!(native.children[0].props.label.as_deref(), Some("Open"));
    assert_eq!(native.children[0].props.value.as_deref(), Some("open"));
    assert_eq!(native.children[0].props.action.as_deref(), Some("openFile"));
}

#[test]
fn lowers_every_supported_semantic_component_tag() {
    let bridge = RsxCompilerBridge::new();

    for component in SemanticComponent::ALL {
        let tag = component.as_str();
        let compiled = CompiledRsxNode::Element {
            key: tag.to_string(),
            tag: tag.to_string(),
            import_source: Some("a3s-rsx".to_string()),
            props: CompiledProps {
                label: Some(format!("{tag} label")),
                ..CompiledProps::default()
            },
            children: vec![CompiledRsxNode::Text {
                key: format!("{tag}-text"),
                value: format!("{tag} text"),
            }],
        };

        let native = bridge
            .lower_to_native(&compiled)
            .unwrap_or_else(|err| panic!("{tag} should lower to native IR: {err}"));

        assert_eq!(native.role, expected_native_role(*component), "{tag}");
    }
}

fn expected_native_role(component: SemanticComponent) -> NativeRole {
    match component {
        SemanticComponent::Button => NativeRole::Button,
        SemanticComponent::Label | SemanticComponent::SelectValue => NativeRole::Text,
        SemanticComponent::Document => NativeRole::Document,
        SemanticComponent::DocumentHead => NativeRole::DocumentHead,
        SemanticComponent::DocumentBody => NativeRole::DocumentBody,
        SemanticComponent::DocumentTitle => NativeRole::DocumentTitle,
        SemanticComponent::Metadata => NativeRole::Metadata,
        SemanticComponent::ResourceLink => NativeRole::ResourceLink,
        SemanticComponent::StyleSheet => NativeRole::StyleSheet,
        SemanticComponent::Script => NativeRole::Script,
        SemanticComponent::Template => NativeRole::Template,
        SemanticComponent::Slot => NativeRole::Slot,
        SemanticComponent::Text => NativeRole::Text,
        SemanticComponent::Abbreviation => NativeRole::Abbreviation,
        SemanticComponent::Citation => NativeRole::Citation,
        SemanticComponent::Definition => NativeRole::Definition,
        SemanticComponent::DataValue => NativeRole::DataValue,
        SemanticComponent::InsertedText => NativeRole::InsertedText,
        SemanticComponent::DeletedText => NativeRole::DeletedText,
        SemanticComponent::MarkedText => NativeRole::MarkedText,
        SemanticComponent::Time => NativeRole::Time,
        SemanticComponent::Emphasis => NativeRole::Emphasis,
        SemanticComponent::StrongText => NativeRole::StrongText,
        SemanticComponent::Code => NativeRole::Code,
        SemanticComponent::KeyboardInput => NativeRole::KeyboardInput,
        SemanticComponent::SampleOutput => NativeRole::SampleOutput,
        SemanticComponent::Variable => NativeRole::Variable,
        SemanticComponent::InlineQuote => NativeRole::InlineQuote,
        SemanticComponent::Subscript => NativeRole::Subscript,
        SemanticComponent::Superscript => NativeRole::Superscript,
        SemanticComponent::SmallText => NativeRole::SmallText,
        SemanticComponent::BoldText => NativeRole::BoldText,
        SemanticComponent::ItalicText => NativeRole::ItalicText,
        SemanticComponent::StruckText => NativeRole::StruckText,
        SemanticComponent::UnderlinedText => NativeRole::UnderlinedText,
        SemanticComponent::BidirectionalIsolate => NativeRole::BidirectionalIsolate,
        SemanticComponent::BidirectionalOverride => NativeRole::BidirectionalOverride,
        SemanticComponent::Paragraph => NativeRole::Paragraph,
        SemanticComponent::PreformattedText => NativeRole::PreformattedText,
        SemanticComponent::BlockQuote => NativeRole::BlockQuote,
        SemanticComponent::ContactAddress => NativeRole::ContactAddress,
        SemanticComponent::LineBreak => NativeRole::LineBreak,
        SemanticComponent::WordBreakOpportunity => NativeRole::WordBreakOpportunity,
        SemanticComponent::NoBreakText => NativeRole::NoBreakText,
        SemanticComponent::CenteredText => NativeRole::CenteredText,
        SemanticComponent::FontText => NativeRole::FontText,
        SemanticComponent::BigText => NativeRole::BigText,
        SemanticComponent::TeletypeText => NativeRole::TeletypeText,
        SemanticComponent::Applet => NativeRole::Applet,
        SemanticComponent::BackgroundSound => NativeRole::BackgroundSound,
        SemanticComponent::Frame => NativeRole::Frame,
        SemanticComponent::FrameSet => NativeRole::FrameSet,
        SemanticComponent::NoEmbedFallback => NativeRole::NoEmbedFallback,
        SemanticComponent::NoFramesFallback => NativeRole::NoFramesFallback,
        SemanticComponent::Marquee => NativeRole::Marquee,
        SemanticComponent::Math => NativeRole::Math,
        SemanticComponent::NextId => NativeRole::NextId,
        SemanticComponent::SelectedContent => NativeRole::SelectedContent,
        SemanticComponent::Heading => NativeRole::Heading,
        SemanticComponent::HeadingGroup => NativeRole::HeadingGroup,
        SemanticComponent::Ruby => NativeRole::Ruby,
        SemanticComponent::RubyBase => NativeRole::RubyBase,
        SemanticComponent::RubyText => NativeRole::RubyText,
        SemanticComponent::RubyParenthesis => NativeRole::RubyParenthesis,
        SemanticComponent::RubyTextContainer => NativeRole::RubyTextContainer,
        SemanticComponent::Main => NativeRole::Main,
        SemanticComponent::Navigation => NativeRole::Navigation,
        SemanticComponent::Header => NativeRole::Header,
        SemanticComponent::Footer => NativeRole::Footer,
        SemanticComponent::Article => NativeRole::Article,
        SemanticComponent::Section => NativeRole::Section,
        SemanticComponent::Aside => NativeRole::Aside,
        SemanticComponent::Search => NativeRole::Search,
        SemanticComponent::Disclosure => NativeRole::Disclosure,
        SemanticComponent::DisclosureSummary => NativeRole::DisclosureSummary,
        SemanticComponent::Figure => NativeRole::Figure,
        SemanticComponent::FigureCaption => NativeRole::FigureCaption,
        SemanticComponent::DescriptionList => NativeRole::DescriptionList,
        SemanticComponent::DescriptionTerm => NativeRole::DescriptionTerm,
        SemanticComponent::DescriptionDetails => NativeRole::DescriptionDetails,
        SemanticComponent::Image => NativeRole::Image,
        SemanticComponent::Media => NativeRole::Media,
        SemanticComponent::Canvas => NativeRole::Canvas,
        SemanticComponent::EmbeddedContent => NativeRole::EmbeddedContent,
        SemanticComponent::Link => NativeRole::Link,
        SemanticComponent::ImageMap => NativeRole::ImageMap,
        SemanticComponent::ImageMapArea => NativeRole::ImageMapArea,
        SemanticComponent::TextField | SemanticComponent::Input => NativeRole::TextField,
        SemanticComponent::Checkbox => NativeRole::Checkbox,
        SemanticComponent::Switch => NativeRole::Switch,
        SemanticComponent::RadioGroup => NativeRole::RadioGroup,
        SemanticComponent::Radio => NativeRole::Radio,
        SemanticComponent::FieldSet => NativeRole::FieldSet,
        SemanticComponent::Legend => NativeRole::Legend,
        SemanticComponent::OptionGroup => NativeRole::OptionGroup,
        SemanticComponent::Output => NativeRole::Output,
        SemanticComponent::Meter => NativeRole::Meter,
        SemanticComponent::ComboBox => NativeRole::ComboBox,
        SemanticComponent::Select => NativeRole::Select,
        SemanticComponent::ListBox => NativeRole::ListBox,
        SemanticComponent::ListBoxItem => NativeRole::ListBoxItem,
        SemanticComponent::Dialog => NativeRole::Dialog,
        SemanticComponent::Popover => NativeRole::Popover,
        SemanticComponent::Tabs => NativeRole::Tabs,
        SemanticComponent::TabList => NativeRole::TabList,
        SemanticComponent::Tab => NativeRole::Tab,
        SemanticComponent::TabPanel => NativeRole::TabPanel,
        SemanticComponent::Group => NativeRole::View,
        SemanticComponent::Form => NativeRole::Form,
        SemanticComponent::Menu => NativeRole::Menu,
        SemanticComponent::MenuItem => NativeRole::MenuItem,
        SemanticComponent::Separator => NativeRole::Separator,
        SemanticComponent::Slider => NativeRole::Slider,
        SemanticComponent::ProgressBar => NativeRole::ProgressBar,
        SemanticComponent::Toolbar => NativeRole::Toolbar,
        SemanticComponent::Table => NativeRole::Table,
        SemanticComponent::TableSection => NativeRole::TableSection,
        SemanticComponent::TableRow => NativeRole::TableRow,
        SemanticComponent::TableCell => NativeRole::TableCell,
        SemanticComponent::TableColumn => NativeRole::TableColumn,
        SemanticComponent::TableCaption => NativeRole::TableCaption,
    }
}
