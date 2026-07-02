use crate::html::{canonical_html_tag, HtmlFormAssociationProps};
use crate::web::WebProps;

use super::attributes::{html_string_attribute, number_attribute};

pub(super) fn html_form_association_props_from_tag(
    tag: &str,
    web: &WebProps,
) -> HtmlFormAssociationProps {
    let Some(tag) = canonical_html_tag(tag) else {
        return HtmlFormAssociationProps::default();
    };

    let attributes = &web.attributes;
    let mut association = HtmlFormAssociationProps::default();

    match tag {
        "label" => {
            association.label_for = html_string_attribute(attributes, &["for", "htmlFor"]);
        }
        "output" => {
            association.output_for = html_string_attribute(attributes, &["for", "htmlFor"]);
        }
        "meter" => {
            association.meter_low = number_attribute(attributes, &["low"]);
            association.meter_high = number_attribute(attributes, &["high"]);
            association.meter_optimum = number_attribute(attributes, &["optimum"]);
        }
        _ => {}
    }

    association
}
