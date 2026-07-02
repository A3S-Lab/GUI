use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityStateProps,
    AccessibilityStructureProps,
};
use crate::web::WebProps;

use super::attributes::{bool_attribute, html_string_attribute, i32_attribute, u32_attribute};

pub(super) fn accessibility_relationship_props_from_web(
    web: &WebProps,
) -> AccessibilityRelationshipProps {
    let attributes = &web.attributes;
    AccessibilityRelationshipProps {
        labelled_by: html_string_attribute(attributes, &["aria-labelledby"]),
        described_by: html_string_attribute(attributes, &["aria-describedby"]),
        details: html_string_attribute(attributes, &["aria-details"]),
        controls: html_string_attribute(attributes, &["aria-controls"]),
        owns: html_string_attribute(attributes, &["aria-owns"]),
        flow_to: html_string_attribute(attributes, &["aria-flowto"]),
        error_message: html_string_attribute(attributes, &["aria-errormessage"]),
        active_descendant: html_string_attribute(attributes, &["aria-activedescendant"]),
    }
}

pub(super) fn accessibility_description_props_from_web(
    web: &WebProps,
) -> AccessibilityDescriptionProps {
    let attributes = &web.attributes;
    AccessibilityDescriptionProps {
        description: html_string_attribute(attributes, &["aria-description"]),
        role_description: html_string_attribute(attributes, &["aria-roledescription"]),
        key_shortcuts: html_string_attribute(attributes, &["aria-keyshortcuts"]),
        value_text: html_string_attribute(attributes, &["aria-valuetext"]),
    }
}

pub(super) fn accessibility_structure_props_from_web(
    web: &WebProps,
) -> AccessibilityStructureProps {
    let attributes = &web.attributes;
    AccessibilityStructureProps {
        level: u32_attribute(attributes, &["aria-level"]),
        position_in_set: i32_attribute(attributes, &["aria-posinset"]),
        set_size: i32_attribute(attributes, &["aria-setsize"]),
        row_count: i32_attribute(attributes, &["aria-rowcount"]),
        row_index: i32_attribute(attributes, &["aria-rowindex"]),
        row_span: u32_attribute(attributes, &["aria-rowspan"]),
        column_count: i32_attribute(attributes, &["aria-colcount"]),
        column_index: i32_attribute(attributes, &["aria-colindex"]),
        column_span: u32_attribute(attributes, &["aria-colspan"]),
        row_index_text: html_string_attribute(attributes, &["aria-rowindextext"]),
        column_index_text: html_string_attribute(attributes, &["aria-colindextext"]),
        sort: html_string_attribute(attributes, &["aria-sort"]),
    }
}

pub(super) fn accessibility_state_props_from_web(web: &WebProps) -> AccessibilityStateProps {
    let attributes = &web.attributes;
    AccessibilityStateProps {
        hidden: bool_attribute(attributes, &["aria-hidden"]),
        autocomplete: html_string_attribute(attributes, &["aria-autocomplete"]),
        multiline: bool_attribute(attributes, &["aria-multiline"]),
        current: html_string_attribute(attributes, &["aria-current"]),
        has_popup: html_string_attribute(attributes, &["aria-haspopup"]),
        pressed: html_string_attribute(attributes, &["aria-pressed"]),
        live: html_string_attribute(attributes, &["aria-live"]),
        atomic: bool_attribute(attributes, &["aria-atomic"]),
        busy: bool_attribute(attributes, &["aria-busy"]),
        relevant: html_string_attribute(attributes, &["aria-relevant"]),
        modal: bool_attribute(attributes, &["aria-modal"]),
    }
}
