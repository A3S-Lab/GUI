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
    #[serde(default, alias = "step")]
    pub step_value: Option<f64>,
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
            step_value: None,
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
        "Link" => Ok(AriaComponent::Link),
        "a" => Ok(component_for_anchor_element(props)),
        "Label" | "label" => Ok(AriaComponent::Label),
        "Document" | "html" => Ok(AriaComponent::Document),
        "DocumentHead" | "head" => Ok(AriaComponent::DocumentHead),
        "DocumentBody" | "body" => Ok(AriaComponent::DocumentBody),
        "DocumentTitle" | "title" => Ok(AriaComponent::DocumentTitle),
        "Metadata" | "base" | "meta" => Ok(AriaComponent::Metadata),
        "ResourceLink" | "link" => Ok(AriaComponent::ResourceLink),
        "StyleSheet" | "style" => Ok(AriaComponent::StyleSheet),
        "Script" | "script" | "noscript" => Ok(AriaComponent::Script),
        "Template" | "template" => Ok(AriaComponent::Template),
        "Slot" | "slot" => Ok(AriaComponent::Slot),
        "Abbreviation" | "abbr" | "acronym" => Ok(AriaComponent::Abbreviation),
        "Citation" | "cite" => Ok(AriaComponent::Citation),
        "Definition" | "dfn" => Ok(AriaComponent::Definition),
        "DataValue" | "data" => Ok(AriaComponent::DataValue),
        "InsertedText" | "ins" => Ok(AriaComponent::InsertedText),
        "DeletedText" | "del" => Ok(AriaComponent::DeletedText),
        "MarkedText" | "mark" => Ok(AriaComponent::MarkedText),
        "Time" | "time" => Ok(AriaComponent::Time),
        "Emphasis" | "em" => Ok(AriaComponent::Emphasis),
        "StrongText" | "strong" => Ok(AriaComponent::StrongText),
        "Code" | "code" => Ok(AriaComponent::Code),
        "KeyboardInput" | "kbd" => Ok(AriaComponent::KeyboardInput),
        "SampleOutput" | "samp" => Ok(AriaComponent::SampleOutput),
        "Variable" | "var" => Ok(AriaComponent::Variable),
        "InlineQuote" | "q" => Ok(AriaComponent::InlineQuote),
        "Subscript" | "sub" => Ok(AriaComponent::Subscript),
        "Superscript" | "sup" => Ok(AriaComponent::Superscript),
        "SmallText" | "small" => Ok(AriaComponent::SmallText),
        "BoldText" | "b" => Ok(AriaComponent::BoldText),
        "ItalicText" | "i" => Ok(AriaComponent::ItalicText),
        "StruckText" | "s" | "strike" => Ok(AriaComponent::StruckText),
        "UnderlinedText" | "u" => Ok(AriaComponent::UnderlinedText),
        "BidirectionalIsolate" | "bdi" => Ok(AriaComponent::BidirectionalIsolate),
        "BidirectionalOverride" | "bdo" => Ok(AriaComponent::BidirectionalOverride),
        "Paragraph" | "p" => Ok(AriaComponent::Paragraph),
        "PreformattedText" | "pre" | "listing" | "plaintext" | "xmp" => {
            Ok(AriaComponent::PreformattedText)
        }
        "BlockQuote" | "blockquote" => Ok(AriaComponent::BlockQuote),
        "ContactAddress" | "address" => Ok(AriaComponent::ContactAddress),
        "LineBreak" | "br" => Ok(AriaComponent::LineBreak),
        "WordBreakOpportunity" | "wbr" => Ok(AriaComponent::WordBreakOpportunity),
        "NoBreakText" | "nobr" => Ok(AriaComponent::NoBreakText),
        "CenteredText" | "center" => Ok(AriaComponent::CenteredText),
        "FontText" | "font" | "basefont" => Ok(AriaComponent::FontText),
        "BigText" | "big" => Ok(AriaComponent::BigText),
        "TeletypeText" | "tt" => Ok(AriaComponent::TeletypeText),
        "Applet" | "applet" => Ok(AriaComponent::Applet),
        "BackgroundSound" | "bgsound" => Ok(AriaComponent::BackgroundSound),
        "Frame" | "frame" => Ok(AriaComponent::Frame),
        "FrameSet" | "frameset" => Ok(AriaComponent::FrameSet),
        "NoEmbedFallback" | "noembed" => Ok(AriaComponent::NoEmbedFallback),
        "NoFramesFallback" | "noframes" => Ok(AriaComponent::NoFramesFallback),
        "Marquee" | "marquee" => Ok(AriaComponent::Marquee),
        "Math" | "math" => Ok(AriaComponent::Math),
        "NextId" | "nextid" => Ok(AriaComponent::NextId),
        "SelectedContent" | "selectedcontent" => Ok(AriaComponent::SelectedContent),
        "Text" | "span" => Ok(AriaComponent::Text),
        "Heading" => Ok(AriaComponent::Heading),
        "HeadingGroup" | "hgroup" => Ok(AriaComponent::HeadingGroup),
        "Ruby" | "ruby" => Ok(AriaComponent::Ruby),
        "RubyBase" | "rb" => Ok(AriaComponent::RubyBase),
        "RubyText" | "rt" => Ok(AriaComponent::RubyText),
        "RubyParenthesis" | "rp" => Ok(AriaComponent::RubyParenthesis),
        "RubyTextContainer" | "rtc" => Ok(AriaComponent::RubyTextContainer),
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
        "Form" | "form" => Ok(AriaComponent::Form),
        "FieldSet" | "fieldset" => Ok(AriaComponent::FieldSet),
        "Legend" | "legend" => Ok(AriaComponent::Legend),
        "OptionGroup" | "optgroup" => Ok(AriaComponent::OptionGroup),
        "Output" | "output" => Ok(AriaComponent::Output),
        "Meter" | "meter" => Ok(AriaComponent::Meter),
        "ImageMap" | "map" => Ok(AriaComponent::ImageMap),
        "ImageMapArea" | "area" => Ok(AriaComponent::ImageMapArea),
        "Select" | "select" => Ok(AriaComponent::Select),
        "SelectValue" => Ok(AriaComponent::SelectValue),
        "ListBox" | "ul" | "ol" | "datalist" | "dir" => Ok(AriaComponent::ListBox),
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
        "Disclosure" | "details" => Ok(AriaComponent::Disclosure),
        "DisclosureSummary" | "summary" => Ok(AriaComponent::DisclosureSummary),
        "Figure" | "figure" => Ok(AriaComponent::Figure),
        "FigureCaption" | "figcaption" => Ok(AriaComponent::FigureCaption),
        "DescriptionList" | "dl" => Ok(AriaComponent::DescriptionList),
        "DescriptionTerm" | "dt" => Ok(AriaComponent::DescriptionTerm),
        "DescriptionDetails" | "dd" => Ok(AriaComponent::DescriptionDetails),
        "Group" | "div" => Ok(AriaComponent::Group),
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

fn component_for_anchor_element(props: &CompiledProps) -> AriaComponent {
    if component_for_intrinsic_tag("a", &props.attributes) == Some(AriaComponent::Link) {
        AriaComponent::Link
    } else if props.events.contains_key("onClick") || props.events.contains_key("onPress") {
        AriaComponent::Button
    } else {
        AriaComponent::Group
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
        let html_fallback_label = html_fallback_label(tag, &web, self.value.as_deref());
        let html_details_open = html_details_open_state(tag, &web);
        let html_placeholder = html_placeholder_state(tag, &web);
        let html_numeric_value = html_numeric_value_state(tag, &web, self.value.as_deref());
        let html_range_step = html_range_step_state(tag, &web);
        let semantic = WebSemanticAliases::from_web(&web);

        let orientation = self.orientation.map(|orientation| match orientation {
            CompiledOrientation::Horizontal => Orientation::Horizontal,
            CompiledOrientation::Vertical => Orientation::Vertical,
        });

        let mut props = AriaProps::new().web(web);
        props.label = self.label.or(html_fallback_label);
        props.text_value = self.text_value;
        props.value = self.value;
        props.placeholder = self
            .placeholder
            .or(semantic.placeholder)
            .or(html_placeholder);
        props.action = self.action;
        props.is_disabled = self.is_disabled || semantic.disabled.unwrap_or(false);
        props.is_required = self.is_required || semantic.required.unwrap_or(false);
        props.is_invalid = self.is_invalid || semantic.invalid.unwrap_or(false);
        props.is_selected = self.is_selected || semantic.selected.unwrap_or(false);
        props.is_checked = self.is_checked.or(semantic.checked);
        props.is_expanded = self.is_expanded.or(semantic.expanded).or(html_details_open);
        props.orientation = orientation.or(semantic.orientation);
        props.min_value = self.min_value.or(semantic.min_value);
        props.max_value = self.max_value.or(semantic.max_value);
        props.value_number = self
            .value_number
            .or(semantic.value_number)
            .or(html_numeric_value);
        props.step_value = self.step_value.or(semantic.step_value).or(html_range_step);
        props
    }
}

fn html_details_open_state(tag: &str, web: &WebProps) -> Option<bool> {
    match canonical_html_tag(tag)? {
        "details" => bool_attribute(&web.attributes, &["open"]),
        _ => None,
    }
}

fn html_placeholder_state(tag: &str, web: &WebProps) -> Option<String> {
    match canonical_html_tag(tag)? {
        "input" | "textarea" => {
            non_empty_string_attribute(&web.attributes, &["placeholder"]).map(str::to_string)
        }
        _ => None,
    }
}

fn html_numeric_value_state(tag: &str, web: &WebProps, value: Option<&str>) -> Option<f64> {
    match canonical_html_tag(tag)? {
        "meter" | "progress" => value
            .and_then(parse_number_attribute)
            .or_else(|| number_attribute(&web.attributes, &["value"])),
        "input" if html_input_type_is(web, "range") || html_input_type_is(web, "number") => value
            .and_then(parse_number_attribute)
            .or_else(|| number_attribute(&web.attributes, &["value"])),
        _ => None,
    }
}

fn html_range_step_state(tag: &str, web: &WebProps) -> Option<f64> {
    match canonical_html_tag(tag)? {
        "input" if html_input_type_is(web, "range") => number_attribute(&web.attributes, &["step"]),
        _ => None,
    }
}

fn html_input_type_is(web: &WebProps, expected: &str) -> bool {
    web.attributes
        .get("type")
        .is_some_and(|value| value.trim().eq_ignore_ascii_case(expected))
}

fn html_fallback_label(tag: &str, web: &WebProps, value: Option<&str>) -> Option<String> {
    if web.attributes.contains_key("aria-label") {
        return None;
    }
    match canonical_html_tag(tag)? {
        "area" | "img" => non_empty_string_attribute(&web.attributes, &["alt"]).map(str::to_string),
        "input" if html_input_type_is(web, "image") => {
            non_empty_string_attribute(&web.attributes, &["alt"])
                .or_else(|| non_empty_string_value(value))
                .map(str::to_string)
        }
        "input" if html_input_type_is(web, "submit") => Some(
            non_empty_string_value(value)
                .or_else(|| non_empty_string_attribute(&web.attributes, &["value"]))
                .unwrap_or("Submit")
                .to_string(),
        ),
        "input" if html_input_type_is(web, "reset") => Some(
            non_empty_string_value(value)
                .or_else(|| non_empty_string_attribute(&web.attributes, &["value"]))
                .unwrap_or("Reset")
                .to_string(),
        ),
        "input" if html_input_type_is(web, "button") => non_empty_string_value(value)
            .or_else(|| non_empty_string_attribute(&web.attributes, &["value"]))
            .map(str::to_string),
        "optgroup" | "option" => {
            non_empty_string_attribute(&web.attributes, &["label"]).map(str::to_string)
        }
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
    placeholder: Option<String>,
    orientation: Option<Orientation>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    value_number: Option<f64>,
    step_value: Option<f64>,
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
            placeholder: non_empty_string_attribute(attributes, &["aria-placeholder"])
                .map(str::to_string),
            orientation: string_attribute(attributes, &["orientation", "aria-orientation"])
                .and_then(parse_orientation),
            min_value: number_attribute(attributes, &["min", "aria-valuemin"]),
            max_value: number_attribute(attributes, &["max", "aria-valuemax"]),
            value_number: number_attribute(attributes, &["aria-valuenow"]),
            step_value: number_attribute(attributes, &["step"]),
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

fn non_empty_string_attribute<'a>(
    attributes: &'a BTreeMap<String, String>,
    names: &[&str],
) -> Option<&'a str> {
    string_attribute(attributes, names)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn non_empty_string_value(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn bool_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_bool_attribute)
}

fn invalid_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_invalid_attribute)
}

fn number_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<f64> {
    string_attribute(attributes, names).and_then(parse_number_attribute)
}

fn parse_number_attribute(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
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
    fn lowers_html_placeholder_attributes_to_native_text_fields() {
        let bridge = ReactCompilerBridge::new();
        let input = CompiledJsxNode::Element {
            key: "email".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("type".to_string(), "email".to_string()),
                    ("placeholder".to_string(), "you@example.com".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let textarea = CompiledJsxNode::Element {
            key: "message".to_string(),
            tag: "textarea".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([(
                    "placeholder".to_string(),
                    "Write a message".to_string(),
                )]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };

        let native_input = bridge.lower_to_native(&input).unwrap();
        let native_textarea = bridge.lower_to_native(&textarea).unwrap();

        assert_eq!(native_input.role, NativeRole::TextField);
        assert_eq!(
            native_input.props.placeholder.as_deref(),
            Some("you@example.com")
        );
        assert_eq!(native_textarea.role, NativeRole::TextField);
        assert_eq!(
            native_textarea.props.placeholder.as_deref(),
            Some("Write a message")
        );
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
                  "aria-placeholder": "Volume",
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
        assert_eq!(native.props.placeholder.as_deref(), Some("Volume"));
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
    fn lowers_html_link_and_image_map_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let link = CompiledJsxNode::Element {
            key: "docs-link".to_string(),
            tag: "a".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("href".to_string(), "/docs".to_string())]),
                ..CompiledProps::default()
            },
            children: vec![CompiledJsxNode::Text {
                key: "docs-link-text".to_string(),
                value: "Docs".to_string(),
            }],
        };

        let native_link = bridge.lower_to_native(&link).unwrap();
        assert_eq!(native_link.role, NativeRole::Link);
        assert_eq!(native_link.props.label.as_deref(), Some("Docs"));
        assert_eq!(
            native_link
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some("a")
        );
        assert_eq!(
            native_link
                .props
                .web
                .attributes
                .get("href")
                .map(String::as_str),
            Some("/docs")
        );

        let clickable_anchor = CompiledJsxNode::Element {
            key: "archive-anchor".to_string(),
            tag: "a".to_string(),
            import_source: None,
            props: CompiledProps {
                events: BTreeMap::from([("onClick".to_string(), "archive".to_string())]),
                ..CompiledProps::default()
            },
            children: vec![CompiledJsxNode::Text {
                key: "archive-anchor-text".to_string(),
                value: "Archive".to_string(),
            }],
        };

        let native_clickable_anchor = bridge.lower_to_native(&clickable_anchor).unwrap();
        assert_eq!(native_clickable_anchor.role, NativeRole::Button);
        assert_eq!(
            native_clickable_anchor.props.label.as_deref(),
            Some("Archive")
        );
        assert_eq!(
            native_clickable_anchor.props.action.as_deref(),
            Some("archive")
        );

        let image_map = CompiledJsxNode::Element {
            key: "hero-map".to_string(),
            tag: "map".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("name".to_string(), "hero-map".to_string())]),
                ..CompiledProps::default()
            },
            children: vec![CompiledJsxNode::Element {
                key: "cta-area".to_string(),
                tag: "area".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        ("href".to_string(), "/signup".to_string()),
                        ("alt".to_string(), "Sign up".to_string()),
                        ("shape".to_string(), "rect".to_string()),
                        ("coords".to_string(), "0,0,120,48".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            }],
        };

        let native_image_map = bridge.lower_to_native(&image_map).unwrap();
        assert_eq!(native_image_map.role, NativeRole::ImageMap);
        assert_eq!(native_image_map.children.len(), 1);
        assert_eq!(native_image_map.children[0].role, NativeRole::ImageMapArea);
        assert_eq!(
            native_image_map.children[0].props.label.as_deref(),
            Some("Sign up")
        );
        assert_eq!(
            native_image_map.children[0]
                .props
                .web
                .attributes
                .get("href")
                .map(String::as_str),
            Some("/signup")
        );
    }

    #[test]
    fn lowers_html_document_metadata_template_and_slot_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let document = CompiledJsxNode::Element {
            key: "document".to_string(),
            tag: "html".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "head".to_string(),
                    tag: "head".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![
                        CompiledJsxNode::Element {
                            key: "title".to_string(),
                            tag: "title".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "title-text".to_string(),
                                value: "Dashboard".to_string(),
                            }],
                        },
                        CompiledJsxNode::Element {
                            key: "base".to_string(),
                            tag: "base".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([(
                                    "href".to_string(),
                                    "https://example.test/".to_string(),
                                )]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                        CompiledJsxNode::Element {
                            key: "description".to_string(),
                            tag: "meta".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([
                                    ("name".to_string(), "description".to_string()),
                                    ("content".to_string(), "Native dashboard".to_string()),
                                ]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                        CompiledJsxNode::Element {
                            key: "stylesheet".to_string(),
                            tag: "link".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([
                                    ("rel".to_string(), "stylesheet".to_string()),
                                    ("href".to_string(), "/app.css".to_string()),
                                ]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                        CompiledJsxNode::Element {
                            key: "style".to_string(),
                            tag: "style".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "style-text".to_string(),
                                value: ".card{display:grid}".to_string(),
                            }],
                        },
                        CompiledJsxNode::Element {
                            key: "script".to_string(),
                            tag: "script".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([(
                                    "src".to_string(),
                                    "/app.js".to_string(),
                                )]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                        CompiledJsxNode::Element {
                            key: "noscript".to_string(),
                            tag: "noscript".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "noscript-text".to_string(),
                                value: "JavaScript is disabled".to_string(),
                            }],
                        },
                        CompiledJsxNode::Element {
                            key: "card-template".to_string(),
                            tag: "template".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Element {
                                key: "template-card".to_string(),
                                tag: "div".to_string(),
                                import_source: None,
                                props: CompiledProps::default(),
                                children: Vec::new(),
                            }],
                        },
                    ],
                },
                CompiledJsxNode::Element {
                    key: "body".to_string(),
                    tag: "body".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![
                        CompiledJsxNode::Element {
                            key: "hero-heading".to_string(),
                            tag: "hgroup".to_string(),
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
                                        value: "Dashboard".to_string(),
                                    }],
                                },
                                CompiledJsxNode::Element {
                                    key: "tagline".to_string(),
                                    tag: "p".to_string(),
                                    import_source: None,
                                    props: CompiledProps::default(),
                                    children: vec![CompiledJsxNode::Text {
                                        key: "tagline-text".to_string(),
                                        value: "Operational summary".to_string(),
                                    }],
                                },
                            ],
                        },
                        CompiledJsxNode::Element {
                            key: "actions-slot".to_string(),
                            tag: "slot".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([(
                                    "name".to_string(),
                                    "actions".to_string(),
                                )]),
                                ..CompiledProps::default()
                            },
                            children: Vec::new(),
                        },
                    ],
                },
            ],
        };

        let native = bridge.lower_to_native(&document).unwrap();
        assert_eq!(native.role, NativeRole::Document);
        assert_eq!(native.children[0].role, NativeRole::DocumentHead);
        assert_eq!(
            native.children[0].children[0].role,
            NativeRole::DocumentTitle
        );
        assert_eq!(
            native.children[0].children[0].props.label.as_deref(),
            Some("Dashboard")
        );
        assert_eq!(native.children[0].children[1].role, NativeRole::Metadata);
        assert_eq!(
            native.children[0].children[2]
                .props
                .web
                .attributes
                .get("content")
                .map(String::as_str),
            Some("Native dashboard")
        );
        assert_eq!(
            native.children[0].children[3].role,
            NativeRole::ResourceLink
        );
        assert_eq!(native.children[0].children[4].role, NativeRole::StyleSheet);
        assert_eq!(native.children[0].children[5].role, NativeRole::Script);
        assert_eq!(native.children[0].children[6].role, NativeRole::Script);
        assert_eq!(native.children[0].children[7].role, NativeRole::Template);
        assert_eq!(native.children[1].role, NativeRole::DocumentBody);
        assert_eq!(
            native.children[1].children[0].role,
            NativeRole::HeadingGroup
        );
        assert_eq!(
            native.children[1].children[0].props.label.as_deref(),
            Some("Dashboard")
        );
        assert_eq!(
            native.children[1].children[0].children[0].role,
            NativeRole::Heading
        );
        assert_eq!(native.children[1].children[1].role, NativeRole::Slot);
        assert_eq!(
            native.children[1].children[1]
                .props
                .web
                .attributes
                .get("name")
                .map(String::as_str),
            Some("actions")
        );
    }

    #[test]
    fn lowers_html_ruby_annotation_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let ruby = CompiledJsxNode::Element {
            key: "ruby".to_string(),
            tag: "ruby".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "base".to_string(),
                    tag: "rb".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "base-text".to_string(),
                        value: "漢".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "open-parenthesis".to_string(),
                    tag: "rp".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "open-parenthesis-text".to_string(),
                        value: "(".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "text".to_string(),
                    tag: "rt".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "text-value".to_string(),
                        value: "kan".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "close-parenthesis".to_string(),
                    tag: "rp".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "close-parenthesis-text".to_string(),
                        value: ")".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "container".to_string(),
                    tag: "rtc".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Element {
                        key: "alternate-text".to_string(),
                        tag: "rt".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "alternate-text-value".to_string(),
                            value: "Han".to_string(),
                        }],
                    }],
                },
            ],
        };

        let native = bridge.lower_to_native(&ruby).unwrap();
        assert_eq!(native.role, NativeRole::Ruby);
        assert_eq!(native.props.label.as_deref(), Some("漢"));
        assert_eq!(native.children[0].role, NativeRole::RubyBase);
        assert_eq!(native.children[0].props.label.as_deref(), Some("漢"));
        assert_eq!(native.children[1].role, NativeRole::RubyParenthesis);
        assert_eq!(native.children[1].props.label.as_deref(), Some("("));
        assert_eq!(native.children[2].role, NativeRole::RubyText);
        assert_eq!(native.children[2].props.label.as_deref(), Some("kan"));
        assert_eq!(native.children[3].role, NativeRole::RubyParenthesis);
        assert_eq!(native.children[3].props.label.as_deref(), Some(")"));
        assert_eq!(native.children[4].role, NativeRole::RubyTextContainer);
        assert_eq!(native.children[4].props.label.as_deref(), Some("Han"));
        assert_eq!(native.children[4].children[0].role, NativeRole::RubyText);
    }

    #[test]
    fn lowers_html_text_annotation_tags_to_native_roles() {
        fn text_annotation(key: &str, tag: &str, text: &str) -> CompiledJsxNode {
            CompiledJsxNode::Element {
                key: key.to_string(),
                tag: tag.to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: format!("{key}-text"),
                    value: text.to_string(),
                }],
            }
        }

        let bridge = ReactCompilerBridge::new();
        let root = CompiledJsxNode::Element {
            key: "annotations".to_string(),
            tag: "div".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "abbr".to_string(),
                    tag: "abbr".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "title".to_string(),
                            "HyperText Markup Language".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "abbr-text".to_string(),
                        value: "HTML".to_string(),
                    }],
                },
                text_annotation("cite", "cite", "Spec"),
                text_annotation("dfn", "dfn", "Term"),
                CompiledJsxNode::Element {
                    key: "data".to_string(),
                    tag: "data".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("value".to_string(), "42".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "data-text".to_string(),
                        value: "Answer".to_string(),
                    }],
                },
                text_annotation("ins", "ins", "added"),
                text_annotation("del", "del", "removed"),
                text_annotation("mark", "mark", "highlight"),
                CompiledJsxNode::Element {
                    key: "time".to_string(),
                    tag: "time".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "datetime".to_string(),
                            "2026-07-02".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "time-text".to_string(),
                        value: "Today".to_string(),
                    }],
                },
            ],
        };

        let native = bridge.lower_to_native(&root).unwrap();
        assert_eq!(native.role, NativeRole::View);
        let expected = [
            (NativeRole::Abbreviation, "HTML"),
            (NativeRole::Citation, "Spec"),
            (NativeRole::Definition, "Term"),
            (NativeRole::DataValue, "Answer"),
            (NativeRole::InsertedText, "added"),
            (NativeRole::DeletedText, "removed"),
            (NativeRole::MarkedText, "highlight"),
            (NativeRole::Time, "Today"),
        ];
        for (index, (role, label)) in expected.iter().enumerate() {
            assert_eq!(native.children[index].role, *role);
            assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
        }
        assert_eq!(
            native.children[0]
                .props
                .web
                .attributes
                .get("title")
                .map(String::as_str),
            Some("HyperText Markup Language")
        );
        assert_eq!(
            native.children[3]
                .props
                .web
                .attributes
                .get("value")
                .map(String::as_str),
            Some("42")
        );
        assert_eq!(
            native.children[7]
                .props
                .web
                .attributes
                .get("datetime")
                .map(String::as_str),
            Some("2026-07-02")
        );
    }

    #[test]
    fn lowers_html_phrasing_text_tags_to_native_roles() {
        fn phrasing(key: &str, tag: &str, text: &str) -> CompiledJsxNode {
            CompiledJsxNode::Element {
                key: key.to_string(),
                tag: tag.to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: format!("{key}-text"),
                    value: text.to_string(),
                }],
            }
        }

        let bridge = ReactCompilerBridge::new();
        let root = CompiledJsxNode::Element {
            key: "phrasing".to_string(),
            tag: "p".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                phrasing("em", "em", "emphasized"),
                phrasing("strong", "strong", "important"),
                phrasing("code", "code", "let value = 1;"),
                phrasing("kbd", "kbd", "Command K"),
                phrasing("samp", "samp", "OK"),
                phrasing("var", "var", "x"),
                CompiledJsxNode::Element {
                    key: "quote".to_string(),
                    tag: "q".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "cite".to_string(),
                            "https://example.test/spec".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "quote-text".to_string(),
                        value: "quoted".to_string(),
                    }],
                },
                phrasing("sub", "sub", "2"),
                phrasing("sup", "sup", "3"),
                phrasing("small", "small", "fine print"),
                phrasing("b", "b", "attention"),
                phrasing("i", "i", "idiomatic"),
                phrasing("s", "s", "obsolete"),
                phrasing("u", "u", "annotation"),
                phrasing("bdi", "bdi", "مرحبا"),
                CompiledJsxNode::Element {
                    key: "bdo".to_string(),
                    tag: "bdo".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("dir".to_string(), "rtl".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "bdo-text".to_string(),
                        value: "abc".to_string(),
                    }],
                },
            ],
        };

        let native = bridge.lower_to_native(&root).unwrap();
        assert_eq!(native.role, NativeRole::Paragraph);
        let expected = [
            (NativeRole::Emphasis, "emphasized"),
            (NativeRole::StrongText, "important"),
            (NativeRole::Code, "let value = 1;"),
            (NativeRole::KeyboardInput, "Command K"),
            (NativeRole::SampleOutput, "OK"),
            (NativeRole::Variable, "x"),
            (NativeRole::InlineQuote, "quoted"),
            (NativeRole::Subscript, "2"),
            (NativeRole::Superscript, "3"),
            (NativeRole::SmallText, "fine print"),
            (NativeRole::BoldText, "attention"),
            (NativeRole::ItalicText, "idiomatic"),
            (NativeRole::StruckText, "obsolete"),
            (NativeRole::UnderlinedText, "annotation"),
            (NativeRole::BidirectionalIsolate, "مرحبا"),
            (NativeRole::BidirectionalOverride, "abc"),
        ];
        for (index, (role, label)) in expected.iter().enumerate() {
            assert_eq!(native.children[index].role, *role);
            assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
        }
        assert_eq!(
            native.children[6]
                .props
                .web
                .attributes
                .get("cite")
                .map(String::as_str),
            Some("https://example.test/spec")
        );
        assert_eq!(
            native.children[15]
                .props
                .web
                .attributes
                .get("dir")
                .map(String::as_str),
            Some("rtl")
        );
    }

    #[test]
    fn lowers_html_flow_and_legacy_text_tags_to_native_roles() {
        fn flow(key: &str, tag: &str, text: &str) -> CompiledJsxNode {
            CompiledJsxNode::Element {
                key: key.to_string(),
                tag: tag.to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: format!("{key}-text"),
                    value: text.to_string(),
                }],
            }
        }

        let bridge = ReactCompilerBridge::new();
        let root = CompiledJsxNode::Element {
            key: "flow".to_string(),
            tag: "div".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                flow("paragraph", "p", "Paragraph"),
                flow("pre", "pre", "line 1\nline 2"),
                CompiledJsxNode::Element {
                    key: "blockquote".to_string(),
                    tag: "blockquote".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "cite".to_string(),
                            "https://example.test/quote".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Element {
                        key: "quote-p".to_string(),
                        tag: "p".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "quote-p-text".to_string(),
                            value: "Quoted paragraph".to_string(),
                        }],
                    }],
                },
                flow("address", "address", "help@example.test"),
                CompiledJsxNode::Element {
                    key: "break".to_string(),
                    tag: "br".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: Vec::new(),
                },
                CompiledJsxNode::Element {
                    key: "word-break".to_string(),
                    tag: "wbr".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: Vec::new(),
                },
                flow("nobr", "nobr", "No break"),
                flow("center", "center", "Centered"),
                CompiledJsxNode::Element {
                    key: "font".to_string(),
                    tag: "font".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("color".to_string(), "red".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "font-text".to_string(),
                        value: "Font text".to_string(),
                    }],
                },
                flow("big", "big", "Big"),
                flow("tt", "tt", "Teletype"),
                flow("listing", "listing", "Legacy listing"),
                flow("plaintext", "plaintext", "Plain text"),
                flow("xmp", "xmp", "Example"),
                flow("basefont", "basefont", "Base font"),
                CompiledJsxNode::Element {
                    key: "directory".to_string(),
                    tag: "dir".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Element {
                        key: "directory-item".to_string(),
                        tag: "li".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "directory-item-text".to_string(),
                            value: "Item".to_string(),
                        }],
                    }],
                },
            ],
        };

        let native = bridge.lower_to_native(&root).unwrap();
        assert_eq!(native.role, NativeRole::View);
        let expected = [
            (NativeRole::Paragraph, "Paragraph"),
            (NativeRole::PreformattedText, "line 1\nline 2"),
            (NativeRole::BlockQuote, "Quoted paragraph"),
            (NativeRole::ContactAddress, "help@example.test"),
            (NativeRole::LineBreak, ""),
            (NativeRole::WordBreakOpportunity, ""),
            (NativeRole::NoBreakText, "No break"),
            (NativeRole::CenteredText, "Centered"),
            (NativeRole::FontText, "Font text"),
            (NativeRole::BigText, "Big"),
            (NativeRole::TeletypeText, "Teletype"),
            (NativeRole::PreformattedText, "Legacy listing"),
            (NativeRole::PreformattedText, "Plain text"),
            (NativeRole::PreformattedText, "Example"),
            (NativeRole::FontText, "Base font"),
            (NativeRole::ListBox, "Item"),
        ];
        for (index, (role, label)) in expected.iter().enumerate() {
            assert_eq!(native.children[index].role, *role);
            if label.is_empty() {
                assert_eq!(native.children[index].props.label, None);
            } else {
                assert_eq!(native.children[index].props.label.as_deref(), Some(*label));
            }
        }
        assert_eq!(
            native.children[2]
                .props
                .web
                .attributes
                .get("cite")
                .map(String::as_str),
            Some("https://example.test/quote")
        );
        assert_eq!(native.children[2].children[0].role, NativeRole::Paragraph);
        assert_eq!(
            native.children[8]
                .props
                .web
                .attributes
                .get("color")
                .map(String::as_str),
            Some("red")
        );
        assert_eq!(
            native.children[15].children[0].role,
            NativeRole::ListBoxItem
        );
    }

    #[test]
    fn lowers_html_remaining_legacy_and_foreign_tags_to_native_roles() {
        fn container(key: &str, tag: &str, text: &str) -> CompiledJsxNode {
            CompiledJsxNode::Element {
                key: key.to_string(),
                tag: tag.to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: format!("{key}-text"),
                    value: text.to_string(),
                }],
            }
        }

        let bridge = ReactCompilerBridge::new();
        let root = CompiledJsxNode::Element {
            key: "legacy".to_string(),
            tag: "div".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "applet".to_string(),
                    tag: "applet".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "code".to_string(),
                            "Demo.class".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: vec![CompiledJsxNode::Text {
                        key: "applet-text".to_string(),
                        value: "Applet fallback".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "bgsound".to_string(),
                    tag: "bgsound".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("src".to_string(), "/tone.wav".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                },
                CompiledJsxNode::Element {
                    key: "frameset".to_string(),
                    tag: "frameset".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Element {
                        key: "frame".to_string(),
                        tag: "frame".to_string(),
                        import_source: None,
                        props: CompiledProps {
                            attributes: BTreeMap::from([(
                                "src".to_string(),
                                "/legacy-frame.html".to_string(),
                            )]),
                            ..CompiledProps::default()
                        },
                        children: Vec::new(),
                    }],
                },
                container("noembed", "noembed", "No embed fallback"),
                container("noframes", "noframes", "No frames fallback"),
                container("marquee", "marquee", "Moving text"),
                container("math", "math", "x+y"),
                CompiledJsxNode::Element {
                    key: "nextid".to_string(),
                    tag: "nextid".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([("n".to_string(), "z42".to_string())]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                },
                container("selected-content", "selectedcontent", "Selected option"),
            ],
        };

        let native = bridge.lower_to_native(&root).unwrap();
        assert_eq!(native.role, NativeRole::View);
        let expected = [
            (NativeRole::Applet, Some("Applet fallback")),
            (NativeRole::BackgroundSound, None),
            (NativeRole::FrameSet, None),
            (NativeRole::NoEmbedFallback, Some("No embed fallback")),
            (NativeRole::NoFramesFallback, Some("No frames fallback")),
            (NativeRole::Marquee, Some("Moving text")),
            (NativeRole::Math, Some("x+y")),
            (NativeRole::NextId, None),
            (NativeRole::SelectedContent, Some("Selected option")),
        ];
        for (index, (role, label)) in expected.iter().enumerate() {
            assert_eq!(native.children[index].role, *role);
            assert_eq!(native.children[index].props.label.as_deref(), *label);
        }
        assert_eq!(
            native.children[0]
                .props
                .web
                .attributes
                .get("code")
                .map(String::as_str),
            Some("Demo.class")
        );
        assert_eq!(
            native.children[1]
                .props
                .web
                .attributes
                .get("src")
                .map(String::as_str),
            Some("/tone.wav")
        );
        assert_eq!(native.children[2].children[0].role, NativeRole::Frame);
        assert_eq!(
            native.children[2].children[0]
                .props
                .web
                .attributes
                .get("src")
                .map(String::as_str),
            Some("/legacy-frame.html")
        );
        assert_eq!(
            native.children[7]
                .props
                .web
                .attributes
                .get("n")
                .map(String::as_str),
            Some("z42")
        );
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
    fn lowers_html_disclosure_figure_and_description_list_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let disclosure = CompiledJsxNode::Element {
            key: "release-notes".to_string(),
            tag: "details".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("open".to_string(), String::new())]),
                ..CompiledProps::default()
            },
            children: vec![
                CompiledJsxNode::Element {
                    key: "release-summary".to_string(),
                    tag: "summary".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "release-summary-text".to_string(),
                        value: "Release details".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "release-body".to_string(),
                    tag: "p".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "release-body-text".to_string(),
                        value: "Native semantic roles are preserved.".to_string(),
                    }],
                },
            ],
        };

        let native_disclosure = bridge.lower_to_native(&disclosure).unwrap();
        assert_eq!(native_disclosure.role, NativeRole::Disclosure);
        assert_eq!(native_disclosure.props.expanded, Some(true));
        assert_eq!(
            native_disclosure
                .props
                .metadata
                .get(HTML_TAG_METADATA_KEY)
                .map(String::as_str),
            Some("details")
        );
        assert_eq!(
            native_disclosure.children[0].role,
            NativeRole::DisclosureSummary
        );
        assert_eq!(
            native_disclosure.children[0].props.label.as_deref(),
            Some("Release details")
        );

        let figure = CompiledJsxNode::Element {
            key: "chart".to_string(),
            tag: "figure".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "chart-image".to_string(),
                    tag: "img".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([(
                            "alt".to_string(),
                            "Revenue chart".to_string(),
                        )]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                },
                CompiledJsxNode::Element {
                    key: "chart-caption".to_string(),
                    tag: "figcaption".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "chart-caption-text".to_string(),
                        value: "Revenue by quarter".to_string(),
                    }],
                },
            ],
        };

        let native_figure = bridge.lower_to_native(&figure).unwrap();
        assert_eq!(native_figure.role, NativeRole::Figure);
        assert_eq!(native_figure.children[0].role, NativeRole::Image);
        assert_eq!(
            native_figure.children[0].props.label.as_deref(),
            Some("Revenue chart")
        );
        assert_eq!(native_figure.children[1].role, NativeRole::FigureCaption);
        assert_eq!(
            native_figure.children[1].props.label.as_deref(),
            Some("Revenue by quarter")
        );

        let description_list = CompiledJsxNode::Element {
            key: "terms".to_string(),
            tag: "dl".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: vec![
                CompiledJsxNode::Element {
                    key: "term".to_string(),
                    tag: "dt".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "term-text".to_string(),
                        value: "IR".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "details".to_string(),
                    tag: "dd".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "details-text".to_string(),
                        value: "Intermediate representation".to_string(),
                    }],
                },
            ],
        };

        let native_description_list = bridge.lower_to_native(&description_list).unwrap();
        assert_eq!(native_description_list.role, NativeRole::DescriptionList);
        assert_eq!(
            native_description_list.children[0].role,
            NativeRole::DescriptionTerm
        );
        assert_eq!(
            native_description_list.children[0].props.label.as_deref(),
            Some("IR")
        );
        assert_eq!(
            native_description_list.children[1].role,
            NativeRole::DescriptionDetails
        );
        assert_eq!(
            native_description_list.children[1].props.label.as_deref(),
            Some("Intermediate representation")
        );
    }

    #[test]
    fn lowers_html_form_grouping_and_value_tags_to_native_roles() {
        let bridge = ReactCompilerBridge::new();
        let form = CompiledJsxNode::Element {
            key: "settings".to_string(),
            tag: "form".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("aria-label".to_string(), "Settings".to_string())]),
                ..CompiledProps::default()
            },
            children: vec![
                CompiledJsxNode::Element {
                    key: "notifications".to_string(),
                    tag: "fieldset".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![
                        CompiledJsxNode::Element {
                            key: "notifications-legend".to_string(),
                            tag: "legend".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Text {
                                key: "notifications-legend-text".to_string(),
                                value: "Notifications".to_string(),
                            }],
                        },
                        CompiledJsxNode::Element {
                            key: "notification-level".to_string(),
                            tag: "select".to_string(),
                            import_source: None,
                            props: CompiledProps::default(),
                            children: vec![CompiledJsxNode::Element {
                                key: "standard-options".to_string(),
                                tag: "optgroup".to_string(),
                                import_source: None,
                                props: CompiledProps {
                                    attributes: BTreeMap::from([(
                                        "label".to_string(),
                                        "Standard".to_string(),
                                    )]),
                                    ..CompiledProps::default()
                                },
                                children: vec![CompiledJsxNode::Element {
                                    key: "daily".to_string(),
                                    tag: "option".to_string(),
                                    import_source: None,
                                    props: CompiledProps {
                                        attributes: BTreeMap::from([(
                                            "label".to_string(),
                                            "Daily".to_string(),
                                        )]),
                                        ..CompiledProps::default()
                                    },
                                    children: Vec::new(),
                                }],
                            }],
                        },
                    ],
                },
                CompiledJsxNode::Element {
                    key: "result".to_string(),
                    tag: "output".to_string(),
                    import_source: None,
                    props: CompiledProps::default(),
                    children: vec![CompiledJsxNode::Text {
                        key: "result-text".to_string(),
                        value: "Saved".to_string(),
                    }],
                },
                CompiledJsxNode::Element {
                    key: "quota".to_string(),
                    tag: "meter".to_string(),
                    import_source: None,
                    props: CompiledProps {
                        attributes: BTreeMap::from([
                            ("min".to_string(), "0".to_string()),
                            ("max".to_string(), "10".to_string()),
                            ("value".to_string(), "7".to_string()),
                        ]),
                        ..CompiledProps::default()
                    },
                    children: Vec::new(),
                },
            ],
        };

        let native = bridge.lower_to_native(&form).unwrap();
        assert_eq!(native.role, NativeRole::Form);
        assert_eq!(native.props.label.as_deref(), Some("Settings"));
        assert_eq!(native.children[0].role, NativeRole::FieldSet);
        assert_eq!(
            native.children[0].props.label.as_deref(),
            Some("Notifications")
        );
        assert_eq!(native.children[0].children[0].role, NativeRole::Legend);
        assert_eq!(
            native.children[0].children[0].props.label.as_deref(),
            Some("Notifications")
        );
        assert_eq!(native.children[0].children[1].role, NativeRole::Select);
        assert_eq!(
            native.children[0].children[1].children[0].role,
            NativeRole::OptionGroup
        );
        assert_eq!(
            native.children[0].children[1].children[0]
                .props
                .label
                .as_deref(),
            Some("Standard")
        );
        assert_eq!(
            native.children[0].children[1].children[0].children[0].role,
            NativeRole::ListBoxItem
        );
        assert_eq!(
            native.children[0].children[1].children[0].children[0]
                .props
                .label
                .as_deref(),
            Some("Daily")
        );
        assert_eq!(native.children[1].role, NativeRole::Output);
        assert_eq!(native.children[1].props.label.as_deref(), Some("Saved"));
        assert_eq!(native.children[2].role, NativeRole::Meter);
        assert_eq!(native.children[2].props.min, Some(0.0));
        assert_eq!(native.children[2].props.max, Some(10.0));
        assert_eq!(native.children[2].props.current, Some(7.0));
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

        let submit = CompiledJsxNode::Element {
            key: "submit".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                value: Some("Save changes".to_string()),
                attributes: BTreeMap::from([("type".to_string(), "submit".to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_submit = bridge.lower_to_native(&submit).unwrap();

        assert_eq!(native_submit.role, NativeRole::Button);
        assert_eq!(native_submit.props.label.as_deref(), Some("Save changes"));

        let submit_default = CompiledJsxNode::Element {
            key: "submit-default".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("type".to_string(), "submit".to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_submit_default = bridge.lower_to_native(&submit_default).unwrap();

        assert_eq!(native_submit_default.role, NativeRole::Button);
        assert_eq!(native_submit_default.props.label.as_deref(), Some("Submit"));

        let reset = CompiledJsxNode::Element {
            key: "reset".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([("type".to_string(), "reset".to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_reset = bridge.lower_to_native(&reset).unwrap();

        assert_eq!(native_reset.role, NativeRole::Button);
        assert_eq!(native_reset.props.label.as_deref(), Some("Reset"));

        let button = CompiledJsxNode::Element {
            key: "button".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("type".to_string(), "button".to_string()),
                    ("value".to_string(), "Open panel".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_button = bridge.lower_to_native(&button).unwrap();

        assert_eq!(native_button.role, NativeRole::Button);
        assert_eq!(native_button.props.label.as_deref(), Some("Open panel"));

        let image_button = CompiledJsxNode::Element {
            key: "image-submit".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                attributes: BTreeMap::from([
                    ("type".to_string(), "image".to_string()),
                    ("alt".to_string(), "Search".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_image_button = bridge.lower_to_native(&image_button).unwrap();

        assert_eq!(native_image_button.role, NativeRole::Button);
        assert_eq!(native_image_button.props.label.as_deref(), Some("Search"));

        let number = CompiledJsxNode::Element {
            key: "quantity".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                value: Some("7".to_string()),
                attributes: BTreeMap::from([
                    ("type".to_string(), "number".to_string()),
                    ("min".to_string(), "1".to_string()),
                    ("max".to_string(), "10".to_string()),
                    ("step".to_string(), "0.5".to_string()),
                ]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_number = bridge.lower_to_native(&number).unwrap();

        assert_eq!(native_number.role, NativeRole::TextField);
        assert_eq!(native_number.props.value.as_deref(), Some("7"));
        assert_eq!(native_number.props.current, Some(7.0));
        assert_eq!(native_number.props.min, Some(1.0));
        assert_eq!(native_number.props.max, Some(10.0));
        assert_eq!(native_number.props.step, Some(0.5));
        assert_eq!(
            native_number.props.metadata.get("type").map(String::as_str),
            Some("number")
        );

        let range = CompiledJsxNode::Element {
            key: "volume".to_string(),
            tag: "input".to_string(),
            import_source: None,
            props: CompiledProps {
                value: Some("42".to_string()),
                min_value: Some(0.0),
                max_value: Some(100.0),
                step_value: Some(5.0),
                attributes: BTreeMap::from([("type".to_string(), "range".to_string())]),
                ..CompiledProps::default()
            },
            children: Vec::new(),
        };
        let native_range = bridge.lower_to_native(&range).unwrap();

        assert_eq!(native_range.role, NativeRole::Slider);
        assert_eq!(native_range.props.current, Some(42.0));
        assert_eq!(native_range.props.min, Some(0.0));
        assert_eq!(native_range.props.max, Some(100.0));
        assert_eq!(native_range.props.step, Some(5.0));
        assert_eq!(
            native_range.props.metadata.get("type").map(String::as_str),
            Some("range")
        );
    }
}
