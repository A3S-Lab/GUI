use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::html::{canonical_html_tag, HTML_TAG_METADATA_KEY};
use crate::react_aria::AriaProps;
use crate::svg::{canonical_svg_tag, SVG_TAG_METADATA_KEY};
use crate::web::WebProps;

use super::{CompiledJsxNode, CompiledOrientation, CompiledProps};

impl CompiledProps {
    pub(super) fn into_aria_props_for_tag(
        mut self,
        tag: &str,
        children: &[CompiledJsxNode],
    ) -> AriaProps {
        if self.value.is_none() && !has_explicit_textarea_value(&self.attributes) {
            self.value = html_textarea_child_value(tag, children);
        }

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
        let html_string_value = html_string_value_state(tag, &web);
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
        props.value = self.value.or(html_string_value);
        props.placeholder = self
            .placeholder
            .or(semantic.placeholder)
            .or(html_placeholder);
        props.action = self.action;
        props.is_disabled = self.is_disabled || semantic.disabled.unwrap_or(false);
        props.is_required = self.is_required || semantic.required.unwrap_or(false);
        props.is_invalid = self.is_invalid || semantic.invalid.unwrap_or(false);
        props.is_read_only = semantic.read_only.unwrap_or(false);
        props.is_multiple = semantic.multiple.unwrap_or(false);
        props.auto_focus = semantic.auto_focus.unwrap_or(false);
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
        props.autocomplete = semantic.autocomplete;
        props.input_mode = semantic.input_mode;
        props.pattern = semantic.pattern;
        props.min_length = semantic.min_length;
        props.max_length = semantic.max_length;
        props.rows = semantic.rows;
        props.cols = semantic.cols;
        props.size = semantic.size;
        props
    }
}

fn html_textarea_child_value(tag: &str, children: &[CompiledJsxNode]) -> Option<String> {
    if canonical_html_tag(tag)? != "textarea" {
        return None;
    }

    let mut value = String::new();
    let mut has_text = false;
    for child in children {
        if let CompiledJsxNode::Text { value: text, .. } = child {
            value.push_str(text);
            has_text = true;
        }
    }
    has_text.then_some(value)
}

fn has_explicit_textarea_value(attributes: &BTreeMap<String, String>) -> bool {
    attributes.contains_key("value") || attributes.contains_key("defaultValue")
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

fn html_string_value_state(tag: &str, web: &WebProps) -> Option<String> {
    match canonical_html_tag(tag)? {
        "data" | "option" => {
            non_empty_string_attribute(&web.attributes, &["value"]).map(str::to_string)
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
    read_only: Option<bool>,
    multiple: Option<bool>,
    auto_focus: Option<bool>,
    selected: Option<bool>,
    checked: Option<bool>,
    expanded: Option<bool>,
    placeholder: Option<String>,
    orientation: Option<Orientation>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    value_number: Option<f64>,
    step_value: Option<f64>,
    autocomplete: Option<String>,
    input_mode: Option<String>,
    pattern: Option<String>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    rows: Option<u32>,
    cols: Option<u32>,
    size: Option<u32>,
}

impl WebSemanticAliases {
    fn from_web(web: &WebProps) -> Self {
        let attributes = &web.attributes;
        Self {
            disabled: bool_attribute(attributes, &["disabled", "aria-disabled"]),
            required: bool_attribute(attributes, &["required", "aria-required"]),
            invalid: invalid_attribute(attributes, &["invalid", "aria-invalid"]),
            read_only: bool_attribute(attributes, &["readonly", "readOnly", "aria-readonly"]),
            multiple: bool_attribute(attributes, &["multiple", "aria-multiselectable"]),
            auto_focus: bool_attribute(attributes, &["autofocus", "autoFocus"]),
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
            autocomplete: non_empty_string_attribute(attributes, &["autocomplete", "autoComplete"])
                .map(str::to_string),
            input_mode: non_empty_string_attribute(attributes, &["inputmode", "inputMode"])
                .map(str::to_string),
            pattern: non_empty_string_attribute(attributes, &["pattern"]).map(str::to_string),
            min_length: u32_attribute(attributes, &["minlength", "minLength"]),
            max_length: u32_attribute(attributes, &["maxlength", "maxLength"]),
            rows: u32_attribute(attributes, &["rows"]),
            cols: u32_attribute(attributes, &["cols"]),
            size: u32_attribute(attributes, &["size"]),
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

fn u32_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<u32> {
    string_attribute(attributes, names).and_then(parse_u32_attribute)
}

fn parse_number_attribute(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
}

fn parse_u32_attribute(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
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
