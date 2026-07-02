use crate::html::{canonical_html_tag, HtmlMicrodataProps};
use crate::web::WebProps;

use super::attributes::{bool_attribute, html_string_attribute};

pub(super) fn html_microdata_props_from_tag(tag: &str, web: &WebProps) -> HtmlMicrodataProps {
    if canonical_html_tag(tag).is_none() {
        return HtmlMicrodataProps::default();
    }

    let attributes = &web.attributes;
    HtmlMicrodataProps {
        item_scope: bool_attribute(attributes, &["itemscope", "itemScope"]).unwrap_or(false),
        item_prop: html_string_attribute(attributes, &["itemprop", "itemProp"]),
        item_type: html_string_attribute(attributes, &["itemtype", "itemType"]),
        item_id: html_string_attribute(attributes, &["itemid", "itemID", "itemId"]),
        item_ref: html_string_attribute(attributes, &["itemref", "itemRef"]),
    }
}
