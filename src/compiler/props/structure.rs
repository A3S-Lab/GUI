use crate::html::{canonical_html_tag, HtmlCollectionProps};
use crate::web::WebProps;

use super::attributes::{
    bool_attribute, html_string_attribute, i32_attribute, parse_i32_attribute, u32_attribute,
};

pub(super) fn html_collection_props_from_tag(
    tag: &str,
    web: &WebProps,
    value: Option<&str>,
) -> HtmlCollectionProps {
    let Some(tag) = canonical_html_tag(tag) else {
        return HtmlCollectionProps::default();
    };

    let attributes = &web.attributes;
    let mut collection = HtmlCollectionProps::default();

    match tag {
        "td" | "th" => {
            collection.column_span = u32_attribute(attributes, &["colspan", "colSpan"]);
            collection.row_span = u32_attribute(attributes, &["rowspan", "rowSpan"]);
            collection.headers = html_string_attribute(attributes, &["headers"]);
            collection.scope = html_string_attribute(attributes, &["scope"]);
            collection.cell_abbr = html_string_attribute(attributes, &["abbr"]);
        }
        "col" | "colgroup" => {
            collection.column_span = u32_attribute(attributes, &["span"]);
        }
        "ol" => {
            collection.list_start = i32_attribute(attributes, &["start"]);
            collection.list_reversed = bool_attribute(attributes, &["reversed"]).unwrap_or(false);
            collection.list_type = html_string_attribute(attributes, &["type"]);
        }
        "ul" | "menu" => {
            collection.list_type = html_string_attribute(attributes, &["type"]);
        }
        "li" => {
            collection.list_type = html_string_attribute(attributes, &["type"]);
            collection.list_item_value = i32_attribute(attributes, &["value"])
                .or_else(|| value.and_then(parse_i32_attribute));
        }
        _ => {}
    }

    collection
}
