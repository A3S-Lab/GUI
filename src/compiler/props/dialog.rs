use crate::html::{canonical_html_tag, HtmlDialogProps};
use crate::web::WebProps;

use super::attributes::{bool_attribute, string_attribute};

pub(super) fn html_dialog_props_from_tag(tag: &str, web: &WebProps) -> HtmlDialogProps {
    if canonical_html_tag(tag) != Some("dialog") {
        return HtmlDialogProps::default();
    }

    let attributes = &web.attributes;
    let open = bool_attribute(attributes, &["open"])
        .or_else(|| string_attribute(attributes, &["open"]).map(|_| true))
        .unwrap_or(false);

    HtmlDialogProps::default().open(open)
}
