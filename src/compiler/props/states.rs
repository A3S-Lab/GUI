use std::collections::BTreeMap;

use crate::compiler::CompiledJsxNode;
use crate::html::canonical_html_tag;
use crate::web::WebProps;

use super::attributes::{
    bool_attribute, non_empty_string_attribute, non_empty_string_value, number_attribute,
    parse_number_attribute, string_attribute,
};

pub(super) fn html_textarea_child_value(tag: &str, children: &[CompiledJsxNode]) -> Option<String> {
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

pub(super) fn has_explicit_textarea_value(attributes: &BTreeMap<String, String>) -> bool {
    attributes.contains_key("value") || attributes.contains_key("defaultValue")
}

pub(super) fn html_details_open_state(tag: &str, web: &WebProps) -> Option<bool> {
    match canonical_html_tag(tag)? {
        "details" => bool_attribute(&web.attributes, &["open"]),
        _ => None,
    }
}

pub(super) fn html_placeholder_state(tag: &str, web: &WebProps) -> Option<String> {
    match canonical_html_tag(tag)? {
        "input" | "textarea" => {
            non_empty_string_attribute(&web.attributes, &["placeholder"]).map(str::to_string)
        }
        _ => None,
    }
}

pub(super) fn html_string_value_state(tag: &str, web: &WebProps) -> Option<String> {
    match canonical_html_tag(tag)? {
        "data" | "option" => {
            non_empty_string_attribute(&web.attributes, &["value"]).map(str::to_string)
        }
        _ => None,
    }
}

pub(super) fn html_form_control_value_state(tag: &str, web: &WebProps) -> Option<String> {
    match canonical_html_tag(tag)? {
        "textarea" => {
            string_attribute(&web.attributes, &["value", "defaultValue"]).map(str::to_string)
        }
        "input" if input_value_attribute_projects_to_value(web) => {
            string_attribute(&web.attributes, &["value", "defaultValue"]).map(str::to_string)
        }
        _ => None,
    }
}

pub(super) fn html_numeric_value_state(
    tag: &str,
    web: &WebProps,
    value: Option<&str>,
) -> Option<f64> {
    match canonical_html_tag(tag)? {
        "meter" | "progress" => value
            .and_then(parse_number_attribute)
            .or_else(|| number_attribute(&web.attributes, &["value"])),
        "input" if html_input_type_is(web, "range") || html_input_type_is(web, "number") => value
            .and_then(parse_number_attribute)
            .or_else(|| number_attribute(&web.attributes, &["value", "defaultValue"])),
        _ => None,
    }
}

pub(super) fn html_range_step_state(tag: &str, web: &WebProps) -> Option<f64> {
    match canonical_html_tag(tag)? {
        "input" if html_input_type_is(web, "range") => number_attribute(&web.attributes, &["step"]),
        _ => None,
    }
}

pub(super) fn html_number_text_value_state(
    tag: &str,
    web: &WebProps,
    value_number: Option<f64>,
) -> Option<String> {
    match canonical_html_tag(tag)? {
        "input" if html_input_type_is(web, "number") => value_number.map(|value| value.to_string()),
        _ => None,
    }
}

fn html_input_type_is(web: &WebProps, expected: &str) -> bool {
    web.attributes
        .get("type")
        .is_some_and(|value| value.trim().eq_ignore_ascii_case(expected))
}

fn input_value_attribute_projects_to_value(web: &WebProps) -> bool {
    let Some(input_type) = web.attributes.get("type").map(|value| value.trim()) else {
        return true;
    };

    !["button", "submit", "reset", "image"]
        .iter()
        .any(|button_type| input_type.eq_ignore_ascii_case(button_type))
}

pub(super) fn html_fallback_label(
    tag: &str,
    web: &WebProps,
    value: Option<&str>,
) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textarea_child_value_preserves_all_text_children() {
        let children = vec![
            CompiledJsxNode::Text {
                key: "a".to_string(),
                value: "hello".to_string(),
            },
            CompiledJsxNode::Text {
                key: "b".to_string(),
                value: " world".to_string(),
            },
        ];

        assert_eq!(
            html_textarea_child_value("textarea", &children).as_deref(),
            Some("hello world")
        );
    }

    #[test]
    fn explicit_textarea_value_detects_html_and_react_names() {
        assert!(has_explicit_textarea_value(&BTreeMap::from([(
            "defaultValue".to_string(),
            "draft".to_string()
        )])));
        assert!(!has_explicit_textarea_value(&BTreeMap::new()));
    }

    #[test]
    fn submit_fallback_label_matches_html_defaults() {
        let web = WebProps::new().attribute("type", "submit");

        assert_eq!(
            html_fallback_label("input", &web, None).as_deref(),
            Some("Submit")
        );
    }
}
