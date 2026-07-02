use crate::html::{canonical_html_tag, HtmlActivationProps};
use crate::web::WebProps;

use super::attributes::html_string_attribute;

pub(super) fn html_activation_props_from_tag(tag: &str, web: &WebProps) -> HtmlActivationProps {
    let Some(tag) = canonical_html_tag(tag) else {
        return HtmlActivationProps::default();
    };

    let attributes = &web.attributes;
    let mut activation = HtmlActivationProps::default();

    match tag {
        "button" => {
            activation.command = html_string_attribute(attributes, &["command"]);
            activation.command_for =
                html_string_attribute(attributes, &["commandfor", "commandFor"]);
            activation.popover_target =
                html_string_attribute(attributes, &["popovertarget", "popoverTarget"]);
            activation.popover_target_action =
                html_string_attribute(attributes, &["popovertargetaction", "popoverTargetAction"]);
        }
        "input" if is_button_like_input(attributes.get("type").map(String::as_str)) => {
            activation.popover_target =
                html_string_attribute(attributes, &["popovertarget", "popoverTarget"]);
            activation.popover_target_action =
                html_string_attribute(attributes, &["popovertargetaction", "popoverTargetAction"]);
        }
        _ => {}
    }

    activation
}

fn is_button_like_input(input_type: Option<&str>) -> bool {
    matches!(
        input_type
            .unwrap_or("text")
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "button" | "submit" | "reset" | "image"
    )
}
