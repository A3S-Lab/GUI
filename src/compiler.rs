use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::Orientation;
use crate::html::{canonical_html_tag, component_for_intrinsic_tag, HTML_TAG_METADATA_KEY};
use crate::native::NativeElement;
use crate::react_aria::{AriaComponent, AriaElement, AriaProps, ReactAriaMapper};
use crate::svg::{canonical_svg_tag, component_for_svg_tag, SVG_TAG_METADATA_KEY};
use crate::web::WebProps;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CompiledJsxNode {
    Element {
        key: String,
        tag: String,
        #[serde(default)]
        import_source: Option<String>,
        #[serde(default)]
        props: CompiledProps,
        #[serde(default)]
        children: Vec<CompiledJsxNode>,
    },
    Text {
        key: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledProps {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub text_value: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default, alias = "aria-label")]
    pub aria_label: Option<String>,
    #[serde(default, alias = "disabled", alias = "aria-disabled")]
    pub is_disabled: bool,
    #[serde(default, alias = "required", alias = "aria-required")]
    pub is_required: bool,
    #[serde(default, alias = "invalid", alias = "aria-invalid")]
    pub is_invalid: bool,
    #[serde(default, alias = "selected", alias = "aria-selected")]
    pub is_selected: bool,
    #[serde(default, alias = "checked", alias = "aria-checked")]
    pub is_checked: Option<bool>,
    #[serde(default, alias = "expanded", alias = "aria-expanded")]
    pub is_expanded: Option<bool>,
    #[serde(default, alias = "aria-orientation")]
    pub orientation: Option<CompiledOrientation>,
    #[serde(default, alias = "min", alias = "aria-valuemin")]
    pub min_value: Option<f64>,
    #[serde(default, alias = "max", alias = "aria-valuemax")]
    pub max_value: Option<f64>,
    #[serde(default, alias = "current", alias = "aria-valuenow")]
    pub value_number: Option<f64>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub class_name: Option<String>,
    #[serde(default)]
    pub style: BTreeMap<String, CompiledStyleValue>,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    #[serde(default)]
    pub events: BTreeMap<String, String>,
}

impl Default for CompiledProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            value: None,
            placeholder: None,
            action: None,
            aria_label: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            id: None,
            class_name: None,
            style: BTreeMap::new(),
            attributes: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompiledOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompiledStyleValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl CompiledStyleValue {
    pub fn to_portable_value(&self) -> String {
        match self {
            CompiledStyleValue::String(value) => value.clone(),
            CompiledStyleValue::Number(value) => {
                if value.fract() == 0.0 {
                    format!("{value:.0}")
                } else {
                    value.to_string()
                }
            }
            CompiledStyleValue::Bool(value) => value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReactCompilerBridge {
    mapper: ReactAriaMapper,
}

impl ReactCompilerBridge {
    pub fn new() -> Self {
        Self {
            mapper: ReactAriaMapper::new(),
        }
    }

    pub fn lower_to_aria(&self, node: &CompiledJsxNode) -> GuiResult<AriaElement> {
        lower_node(node)
    }

    pub fn lower_to_native(&self, node: &CompiledJsxNode) -> GuiResult<NativeElement> {
        let aria = self.lower_to_aria(node)?;
        self.mapper.map(&aria)
    }
}

fn lower_node(node: &CompiledJsxNode) -> GuiResult<AriaElement> {
    match node {
        CompiledJsxNode::Text { key, value } => Ok(AriaElement::text(key.clone(), value.clone())),
        CompiledJsxNode::Element {
            key,
            tag,
            props,
            children,
            ..
        } => {
            let component = component_from_jsx_tag(tag, props)?;
            let mut element = AriaElement::new(key.clone(), component)
                .with_props(props.clone().into_aria_props_for_tag(tag));
            element.children = children
                .iter()
                .map(lower_node)
                .collect::<GuiResult<Vec<_>>>()?;
            Ok(element)
        }
    }
}

fn component_from_jsx_tag(tag: &str, props: &CompiledProps) -> GuiResult<AriaComponent> {
    match tag {
        "Button" | "button" => Ok(AriaComponent::Button),
        "Label" | "label" => Ok(AriaComponent::Label),
        "Text" | "span" | "p" | "strong" | "em" => Ok(AriaComponent::Text),
        "Heading" => Ok(AriaComponent::Heading),
        "TextField" => Ok(AriaComponent::TextField),
        "Input" | "textarea" => Ok(AriaComponent::Input),
        "input" => component_for_intrinsic_tag(tag, &props.attributes).ok_or_else(|| {
            GuiError::UnsupportedAriaComponent {
                component: tag.to_string(),
            }
        }),
        "Checkbox" => Ok(AriaComponent::Checkbox),
        "Switch" => Ok(AriaComponent::Switch),
        "RadioGroup" => Ok(AriaComponent::RadioGroup),
        "Radio" => Ok(AriaComponent::Radio),
        "Select" | "select" => Ok(AriaComponent::Select),
        "SelectValue" => Ok(AriaComponent::SelectValue),
        "ListBox" | "ul" | "ol" => Ok(AriaComponent::ListBox),
        "ListBoxItem" | "option" | "li" => Ok(AriaComponent::ListBoxItem),
        "Dialog" | "dialog" => Ok(AriaComponent::Dialog),
        "Popover" => Ok(AriaComponent::Popover),
        "Tabs" => Ok(AriaComponent::Tabs),
        "TabList" => Ok(AriaComponent::TabList),
        "Tab" => Ok(AriaComponent::Tab),
        "TabPanel" => Ok(AriaComponent::TabPanel),
        "Main" => Ok(AriaComponent::Main),
        "Navigation" => Ok(AriaComponent::Navigation),
        "Header" => Ok(AriaComponent::Header),
        "Footer" => Ok(AriaComponent::Footer),
        "Article" => Ok(AriaComponent::Article),
        "Section" => Ok(AriaComponent::Section),
        "Aside" => Ok(AriaComponent::Aside),
        "Search" => Ok(AriaComponent::Search),
        "Group" | "Form" | "form" | "div" => Ok(AriaComponent::Group),
        "Menu" => Ok(AriaComponent::Menu),
        "MenuItem" => Ok(AriaComponent::MenuItem),
        "Separator" | "hr" => Ok(AriaComponent::Separator),
        "Slider" => Ok(AriaComponent::Slider),
        "ProgressBar" | "progress" => Ok(AriaComponent::ProgressBar),
        "Toolbar" => Ok(AriaComponent::Toolbar),
        other => component_for_intrinsic_tag(other, &props.attributes)
            .or_else(|| component_for_svg_tag(other))
            .ok_or_else(|| GuiError::UnsupportedAriaComponent {
                component: other.to_string(),
            }),
    }
}

impl CompiledProps {
    fn into_aria_props_for_tag(self, tag: &str) -> AriaProps {
        let mut web = WebProps::new();
        if let Some(html_tag) = canonical_html_tag(tag) {
            web = web.attribute(HTML_TAG_METADATA_KEY, html_tag);
        }
        if let Some(svg_tag) = canonical_svg_tag(tag) {
            web = web.attribute(SVG_TAG_METADATA_KEY, svg_tag);
        }
        if let Some(id) = self.id {
            web = web.id(id);
        }
        if let Some(class_name) = self.class_name {
            web = web.class_name(class_name);
        }
        if let Some(label) = self.aria_label {
            web = web.attribute("aria-label", label);
        }
        for (property, value) in self.style {
            web = web.style(property, value.to_portable_value());
        }
        for (name, value) in self.attributes {
            web = web.attribute(name, value);
        }
        for (name, action) in self.events {
            web = web.event(name, action);
        }
        let html_fallback_label = html_fallback_label(tag, &web);
        let semantic = WebSemanticAliases::from_web(&web);

        let orientation = self.orientation.map(|orientation| match orientation {
            CompiledOrientation::Horizontal => Orientation::Horizontal,
            CompiledOrientation::Vertical => Orientation::Vertical,
        });

        let mut props = AriaProps::new().web(web);
        props.label = self.label.or(html_fallback_label);
        props.text_value = self.text_value;
        props.value = self.value;
        props.placeholder = self.placeholder;
        props.action = self.action;
        props.is_disabled = self.is_disabled || semantic.disabled.unwrap_or(false);
        props.is_required = self.is_required || semantic.required.unwrap_or(false);
        props.is_invalid = self.is_invalid || semantic.invalid.unwrap_or(false);
        props.is_selected = self.is_selected || semantic.selected.unwrap_or(false);
        props.is_checked = self.is_checked.or(semantic.checked);
        props.is_expanded = self.is_expanded.or(semantic.expanded);
        props.orientation = orientation.or(semantic.orientation);
        props.min_value = self.min_value.or(semantic.min_value);
        props.max_value = self.max_value.or(semantic.max_value);
        props.value_number = self.value_number.or(semantic.value_number);
        props
    }
}

fn html_fallback_label(tag: &str, web: &WebProps) -> Option<String> {
    if web.attributes.contains_key("aria-label") {
        return None;
    }
    match canonical_html_tag(tag)? {
        "area" | "img" => web
            .attributes
            .get("alt")
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string),
        _ => None,
    }
}

#[derive(Debug, Default)]
struct WebSemanticAliases {
    disabled: Option<bool>,
    required: Option<bool>,
    invalid: Option<bool>,
    selected: Option<bool>,
    checked: Option<bool>,
    expanded: Option<bool>,
    orientation: Option<Orientation>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    value_number: Option<f64>,
}

impl WebSemanticAliases {
    fn from_web(web: &WebProps) -> Self {
        let attributes = &web.attributes;
        Self {
            disabled: bool_attribute(attributes, &["disabled", "aria-disabled"]),
            required: bool_attribute(attributes, &["required", "aria-required"]),
            invalid: invalid_attribute(attributes, &["invalid", "aria-invalid"]),
            selected: bool_attribute(attributes, &["selected", "aria-selected"]),
            checked: bool_attribute(attributes, &["checked", "aria-checked"]),
            expanded: bool_attribute(attributes, &["expanded", "aria-expanded"]),
            orientation: string_attribute(attributes, &["orientation", "aria-orientation"])
                .and_then(parse_orientation),
            min_value: number_attribute(attributes, &["min", "aria-valuemin"]),
            max_value: number_attribute(attributes, &["max", "aria-valuemax"]),
            value_number: number_attribute(attributes, &["aria-valuenow"]),
        }
    }
}

fn string_attribute<'a>(
    attributes: &'a BTreeMap<String, String>,
    names: &[&str],
) -> Option<&'a str> {
    names
        .iter()
        .find_map(|name| attributes.get(*name).map(String::as_str))
}

fn bool_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_bool_attribute)
}

fn invalid_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_invalid_attribute)
}

fn number_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<f64> {
    string_attribute(attributes, names).and_then(|value| value.parse::<f64>().ok())
}

fn parse_bool_attribute(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_invalid_attribute(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "true" | "grammar" | "spelling" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_orientation(value: &str) -> Option<Orientation> {
    match value.trim().to_ascii_lowercase().as_str() {
        "horizontal" => Some(Orientation::Horizontal),
        "vertical" => Some(Orientation::Vertical),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::{HTML_ELEMENTS, HTML_TAG_METADATA_KEY};
    use crate::native::NativeRole;
    use crate::svg::{SVG_ELEMENTS, SVG_TAG_METADATA_KEY};

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
                "events": {"onClick": "saveDocument"}
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
                  "aria-selected": "true",
                  "aria-expanded": "true",
                  "aria-orientation": "horizontal",
                  "aria-valuemin": "0",
                  "aria-valuemax": "100",
                  "aria-valuenow": "50"
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
        assert!(native.props.selected);
        assert_eq!(native.props.expanded, Some(true));
        assert_eq!(native.props.orientation, Some(Orientation::Horizontal));
        assert_eq!(native.props.min, Some(0.0));
        assert_eq!(native.props.max, Some(100.0));
        assert_eq!(native.props.current, Some(50.0));
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

    #[test]
    fn lowers_all_known_html_elements_without_rejecting_intrinsic_tags() {
        let bridge = ReactCompilerBridge::new();

        for tag in HTML_ELEMENTS {
            let props = if *tag == "input" {
                CompiledProps {
                    attributes: BTreeMap::from([("type".to_string(), "checkbox".to_string())]),
                    ..CompiledProps::default()
                }
            } else {
                CompiledProps::default()
            };
            let compiled = CompiledJsxNode::Element {
                key: format!("{tag}-key"),
                tag: tag.to_string(),
                import_source: None,
                props,
                children: Vec::new(),
            };

            let native = bridge
                .lower_to_native(&compiled)
                .unwrap_or_else(|error| panic!("{tag} should lower to native IR: {error}"));

            assert_eq!(
                native
                    .props
                    .metadata
                    .get(HTML_TAG_METADATA_KEY)
                    .map(String::as_str),
                Some(*tag)
            );
        }
    }

    #[test]
    fn lowers_all_known_svg_elements_without_rejecting_intrinsic_tags() {
        let bridge = ReactCompilerBridge::new();

        for tag in SVG_ELEMENTS {
            let compiled = CompiledJsxNode::Element {
                key: format!("{tag}-key"),
                tag: tag.to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            };

            let native = bridge
                .lower_to_native(&compiled)
                .unwrap_or_else(|error| panic!("{tag} should lower to native IR: {error}"));

            assert_eq!(
                native
                    .props
                    .metadata
                    .get(SVG_TAG_METADATA_KEY)
                    .map(String::as_str),
                Some(*tag)
            );
        }
    }

    #[test]
    fn lowers_html_embedded_media_and_table_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let img = CompiledJsxNode::Element {
            key: "hero".to_string(),
            tag: "img".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("alt".to_string(), "Product screenshot".to_string()),
                    ("src".to_string(), "/hero.png".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };

        let native_img = bridge.lower_to_native(&img).unwrap();
        assert_eq!(native_img.role, NativeRole::Image);
        assert_eq!(
            native_img.props.label.as_deref(),
            Some("Product screenshot")
        );
        assert_eq!(
            native_img
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some("img")
        );

        let video = CompiledJsxNode::Element {
            key: "demo-video".to_string(),
            tag: "video".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![CompiledJsxNode::Element {
                key: "demo-source".to_string(),
                tag: "source".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([("src".to_string(), "/demo.mp4".to_string())]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            }],
        };

        let native_video = bridge.lower_to_native(&video).unwrap();
        assert_eq!(native_video.role, NativeRole::Media);
        assert_eq!(native_video.children.len(), 1);
        assert_eq!(native_video.children[0].role, NativeRole::EmbeddedContent);

        let table = CompiledJsxNode::Element {
            key: "metrics".to_string(),
            tag: "table".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "metrics-caption".to_string(),
                    tag: "caption".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "metrics-caption-text".to_string(),
                        value: "Metrics".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "metrics-body".to_string(),
                    tag: "tbody".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Element {
                        key: "metrics-row".to_string(),
                        tag: "tr".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Element {
                            key: "metrics-cell".to_string(),
                            tag: "td".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "metrics-cell-text".to_string(),
                                value: "42".to_string(),
                            }],
                        }],
                    }],
                },
            ],
        };

        let native_table = bridge.lower_to_native(&table).unwrap();
        assert_eq!(native_table.role, NativeRole::Table);
        assert_eq!(native_table.children[0].role, NativeRole::TableCaption);
        assert_eq!(
            native_table.children[0].props.label.as_deref(),
            Some("Metrics")
        );
        assert_eq!(native_table.children[1].role, NativeRole::TableSection);
        assert_eq!(
            native_table.children[1].children[0].role,
            NativeRole::TableRow
        );
        assert_eq!(
            native_table.children[1].children[0].children[0].role,
            NativeRole::TableCell
        );
        assert_eq!(
            native_table.children[1].children[0].children[0]
                .props
                .label
                .as_deref(),
            Some("42")
        );
    }

    #[test]
    fn lowers_html_sectioning_landmark_and_heading_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let tree = CompiledJsxNode::Element {
            key: "main".to_string(),
            tag: "main".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "top-nav".to_string(),
                    tag: "nav".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "aria-label".to_string(),
                            "Primary navigation".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                },
                CompiledJsxNode::Element {
                    key: "article".to_string(),
                    tag: "article".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![
                        CompiledJsxNode::Element {
                            key: "headline".to_string(),
                            tag: "h1".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "headline-text".to_string(),
                                value: "Release notes".to_string(),
                            }],
                        },
                        CompiledJsxNode::Element {
                            key: "summary".to_string(),
                            tag: "section".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([(
                                    "aria-label".to_string(),
                                    "Summary".to_string(),
                                )]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                    ],
                },
                CompiledJsxNode::Element {
                    key: "related".to_string(),
                    tag: "aside".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: Vec::new(),
                },
                CompiledJsxNode::Element {
                    key: "search".to_string(),
                    tag: "search".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: Vec::new(),
                },
            ],
        };

        let native = bridge.lower_to_native(&tree).unwrap();
        assert_eq!(native.role, NativeRole::Main);
        assert_eq!(
            native
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some("main")
        );
        assert_eq!(native.children[0].role, NativeRole::Navigation);
        assert_eq!(
            native.children[0].props.label.as_deref(),
            Some("Primary navigation")
        );
        assert_eq!(native.children[1].role, NativeRole::Article);
        assert_eq!(native.children[1].children[0].role, NativeRole::Heading);
        assert_eq!(
            native.children[1].children[0].props.label.as_deref(),
            Some("Release notes")
        );
        assert_eq!(native.children[1].children[1].role, NativeRole::Section);
        assert_eq!(
            native.children[1].children[1].props.label.as_deref(),
            Some("Summary")
        );
        assert_eq!(native.children[2].role, NativeRole::Aside);
        assert_eq!(native.children[3].role, NativeRole::Search);
    }

    #[test]
    fn lowers_html_input_types_to_native_form_roles() {
        let bridge = ReactCompilerBridge::new();
        let input = |input_type: &str| CompiledJsxNode::Element {
            key: format!("{input_type}-input"),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("type".to_string(), input_type.to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };

        assert_eq!(
            bridge.lower_to_native(&input("checkbox")).unwrap().role,
            NativeRole::Checkbox
        );
        assert_eq!(
            bridge.lower_to_native(&input("radio")).unwrap().role,
            NativeRole::Radio
        );
        assert_eq!(
            bridge.lower_to_native(&input("range")).unwrap().role,
            NativeRole::Slider
        );
        assert_eq!(
            bridge.lower_to_native(&input("email")).unwrap().role,
            NativeRole::TextField
        );
    }
}
