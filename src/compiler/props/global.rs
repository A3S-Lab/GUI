use std::collections::BTreeMap;

use crate::web::WebProps;

use super::attributes::{
    bool_attribute, html_present_string_attribute, html_string_attribute, i32_attribute,
    string_attribute,
};

#[derive(Debug, Default)]
pub(super) struct HtmlGlobalAliases {
    pub(super) title: Option<String>,
    pub(super) hidden: bool,
    pub(super) lang: Option<String>,
    pub(super) dir: Option<String>,
    pub(super) tab_index: Option<i32>,
    pub(super) explicit_role: Option<String>,
    pub(super) access_key: Option<String>,
    pub(super) content_editable: Option<String>,
    pub(super) draggable: Option<String>,
    pub(super) spell_check: Option<bool>,
    pub(super) translate: Option<bool>,
    pub(super) inert: bool,
    pub(super) popover: Option<String>,
    pub(super) anchor: Option<String>,
    pub(super) custom_element_is: Option<String>,
    pub(super) nonce: Option<String>,
}

impl HtmlGlobalAliases {
    pub(super) fn from_web(web: &WebProps) -> Self {
        let attributes = &web.attributes;
        Self {
            title: html_string_attribute(attributes, &["title"]),
            hidden: bool_attribute(attributes, &["hidden"]).unwrap_or(false),
            lang: html_string_attribute(attributes, &["lang", "xml:lang"]),
            dir: html_string_attribute(attributes, &["dir"]),
            tab_index: i32_attribute(attributes, &["tabindex", "tabIndex"]),
            explicit_role: html_string_attribute(attributes, &["role"]),
            access_key: html_string_attribute(attributes, &["accesskey", "accessKey"]),
            content_editable: html_present_string_attribute(
                attributes,
                &["contenteditable", "contentEditable"],
            )
            .map(|value| {
                if value.is_empty() {
                    "true".to_string()
                } else {
                    value
                }
            }),
            draggable: html_string_attribute(attributes, &["draggable"]),
            spell_check: bool_attribute(attributes, &["spellcheck", "spellCheck"]),
            translate: translate_attribute(attributes),
            inert: bool_attribute(attributes, &["inert"]).unwrap_or(false),
            popover: popover_attribute(attributes),
            anchor: html_string_attribute(attributes, &["anchor"]),
            custom_element_is: html_string_attribute(attributes, &["is"]),
            nonce: html_string_attribute(attributes, &["nonce"]),
        }
    }
}

fn translate_attribute(attributes: &BTreeMap<String, String>) -> Option<bool> {
    string_attribute(attributes, &["translate"]).and_then(|value| {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "yes" | "true" => Some(true),
            "no" | "false" => Some(false),
            _ => None,
        }
    })
}

fn popover_attribute(attributes: &BTreeMap<String, String>) -> Option<String> {
    string_attribute(attributes, &["popover"]).and_then(|value| {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "true" => Some("auto".to_string()),
            "false" => None,
            _ => Some(value.trim().to_string()),
        }
    })
}
