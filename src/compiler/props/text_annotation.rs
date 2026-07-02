use crate::html::{canonical_html_tag, HtmlTextAnnotationProps};
use crate::web::WebProps;

use super::attributes::html_string_attribute;

pub(super) fn html_text_annotation_props_from_tag(
    tag: &str,
    web: &WebProps,
) -> HtmlTextAnnotationProps {
    let Some(tag) = canonical_html_tag(tag) else {
        return HtmlTextAnnotationProps::default();
    };

    let attributes = &web.attributes;
    let mut annotation = HtmlTextAnnotationProps::default();

    match tag {
        "blockquote" | "q" | "del" | "ins" => {
            annotation.cite = html_string_attribute(attributes, &["cite"]);
        }
        _ => {}
    }

    match tag {
        "del" | "ins" | "time" => {
            annotation.date_time = html_string_attribute(attributes, &["datetime", "dateTime"]);
        }
        _ => {}
    }

    annotation
}
