use crate::html::{canonical_html_tag, HtmlShadowProps};
use crate::web::WebProps;

use super::attributes::html_string_attribute;

pub(super) fn html_shadow_props_from_tag(tag: &str, web: &WebProps) -> HtmlShadowProps {
    if canonical_html_tag(tag).is_none() {
        return HtmlShadowProps::default();
    }

    let attributes = &web.attributes;
    HtmlShadowProps {
        slot_name: html_string_attribute(attributes, &["slot"]),
        part: html_string_attribute(attributes, &["part"]),
        export_parts: html_string_attribute(attributes, &["exportparts", "exportParts"]),
    }
}
