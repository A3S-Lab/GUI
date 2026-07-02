use std::collections::BTreeMap;

use crate::html::canonical_html_tag;
use crate::web::WebProps;

use super::attributes::{
    bool_attribute, html_present_string_attribute, html_string_attribute, string_attribute,
};

#[derive(Debug, Default)]
pub(super) struct HtmlControlAliases {
    pub(super) name: Option<String>,
    pub(super) form: Option<String>,
    pub(super) input_type: Option<String>,
    pub(super) accept: Option<String>,
    pub(super) capture: Option<String>,
    pub(super) alt: Option<String>,
    pub(super) src: Option<String>,
    pub(super) list: Option<String>,
    pub(super) dirname: Option<String>,
    pub(super) form_action: Option<String>,
    pub(super) form_enctype: Option<String>,
    pub(super) form_method: Option<String>,
    pub(super) form_target: Option<String>,
    pub(super) form_no_validate: bool,
}

impl HtmlControlAliases {
    pub(super) fn from_tag(tag: &str, web: &WebProps) -> Self {
        let Some(tag) = canonical_html_tag(tag) else {
            return Self::default();
        };

        let attributes = &web.attributes;
        let mut aliases = Self::default();

        if matches!(
            tag,
            "button" | "fieldset" | "input" | "object" | "output" | "select" | "textarea"
        ) {
            aliases.name = html_string_attribute(attributes, &["name"]);
            aliases.form = html_string_attribute(attributes, &["form"]);
        }

        match tag {
            "form" => {
                aliases.name = html_string_attribute(attributes, &["name"]);
                aliases.form_action = html_string_attribute(attributes, &["action"]);
                aliases.form_enctype = html_string_attribute(attributes, &["enctype", "encType"]);
                aliases.form_method = html_string_attribute(attributes, &["method"]);
                aliases.form_target = html_string_attribute(attributes, &["target"]);
                aliases.form_no_validate =
                    bool_attribute(attributes, &["novalidate", "noValidate"]).unwrap_or(false);
            }
            "button" => {
                aliases.input_type = html_string_attribute(attributes, &["type"]);
                if html_button_is_submit(attributes) {
                    aliases.read_submit_overrides(attributes);
                }
            }
            "input" => {
                aliases.input_type = html_string_attribute(attributes, &["type"]);
                aliases.accept = html_string_attribute(attributes, &["accept"]);
                aliases.capture = html_present_string_attribute(attributes, &["capture"]);
                aliases.alt = html_string_attribute(attributes, &["alt"]);
                aliases.src = html_string_attribute(attributes, &["src"]);
                aliases.list = html_string_attribute(attributes, &["list"]);
                aliases.dirname = html_string_attribute(attributes, &["dirname"]);
                if html_input_is_submit(attributes) {
                    aliases.read_submit_overrides(attributes);
                }
            }
            "textarea" => {
                aliases.dirname = html_string_attribute(attributes, &["dirname"]);
            }
            _ => {}
        }

        aliases
    }

    fn read_submit_overrides(&mut self, attributes: &BTreeMap<String, String>) {
        self.form_action = html_string_attribute(attributes, &["formaction", "formAction"]);
        self.form_enctype = html_string_attribute(attributes, &["formenctype", "formEncType"]);
        self.form_method = html_string_attribute(attributes, &["formmethod", "formMethod"]);
        self.form_target = html_string_attribute(attributes, &["formtarget", "formTarget"]);
        self.form_no_validate =
            bool_attribute(attributes, &["formnovalidate", "formNoValidate"]).unwrap_or(false);
    }
}

fn html_button_is_submit(attributes: &BTreeMap<String, String>) -> bool {
    match string_attribute(attributes, &["type"]) {
        Some(value) => value.trim().eq_ignore_ascii_case("submit"),
        None => true,
    }
}

fn html_input_is_submit(attributes: &BTreeMap<String, String>) -> bool {
    string_attribute(attributes, &["type"]).is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "submit" | "image"
        )
    })
}
