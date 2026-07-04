use crate::geometry::Orientation;
use crate::web::WebProps;

use super::attributes::{
    bool_attribute, invalid_attribute, non_empty_string_attribute, number_attribute,
    positive_u32_attribute, string_attribute, u32_attribute,
};

#[derive(Debug, Default)]
pub(super) struct WebSemanticAliases {
    pub(super) disabled: Option<bool>,
    pub(super) required: Option<bool>,
    pub(super) invalid: Option<bool>,
    pub(super) read_only: Option<bool>,
    pub(super) multiple: Option<bool>,
    pub(super) auto_focus: Option<bool>,
    pub(super) selected: Option<bool>,
    pub(super) checked: Option<bool>,
    pub(super) expanded: Option<bool>,
    pub(super) placeholder: Option<String>,
    pub(super) orientation: Option<Orientation>,
    pub(super) min_value: Option<f64>,
    pub(super) max_value: Option<f64>,
    pub(super) value_number: Option<f64>,
    pub(super) step_value: Option<f64>,
    pub(super) autocomplete: Option<String>,
    pub(super) input_mode: Option<String>,
    pub(super) enter_key_hint: Option<String>,
    pub(super) auto_capitalize: Option<String>,
    pub(super) auto_correct: Option<String>,
    pub(super) virtual_keyboard_policy: Option<String>,
    pub(super) pattern: Option<String>,
    pub(super) min_length: Option<u32>,
    pub(super) max_length: Option<u32>,
    pub(super) rows: Option<u32>,
    pub(super) cols: Option<u32>,
    pub(super) size: Option<u32>,
}

impl WebSemanticAliases {
    pub(super) fn from_web(web: &WebProps) -> Self {
        let attributes = &web.attributes;
        Self {
            disabled: bool_attribute(attributes, &["disabled", "aria-disabled"]),
            required: bool_attribute(attributes, &["required", "aria-required"]),
            invalid: invalid_attribute(attributes, &["invalid", "aria-invalid"]),
            read_only: bool_attribute(attributes, &["readonly", "readOnly", "aria-readonly"]),
            multiple: bool_attribute(attributes, &["multiple", "aria-multiselectable"]),
            auto_focus: bool_attribute(attributes, &["autofocus", "autoFocus"]),
            selected: bool_attribute(attributes, &["selected", "aria-selected"]),
            checked: bool_attribute(attributes, &["checked", "aria-checked"]),
            expanded: bool_attribute(attributes, &["expanded", "aria-expanded"]),
            placeholder: non_empty_string_attribute(attributes, &["aria-placeholder"])
                .map(str::to_string),
            orientation: string_attribute(attributes, &["orientation", "aria-orientation"])
                .and_then(parse_orientation),
            min_value: number_attribute(attributes, &["min", "aria-valuemin"]),
            max_value: number_attribute(attributes, &["max", "aria-valuemax"]),
            value_number: number_attribute(attributes, &["aria-valuenow"]),
            step_value: number_attribute(attributes, &["step"]),
            autocomplete: non_empty_string_attribute(attributes, &["autocomplete", "autoComplete"])
                .map(str::to_string),
            input_mode: non_empty_string_attribute(attributes, &["inputmode", "inputMode"])
                .map(str::to_string),
            enter_key_hint: non_empty_string_attribute(
                attributes,
                &["enterkeyhint", "enterKeyHint"],
            )
            .map(str::to_string),
            auto_capitalize: non_empty_string_attribute(
                attributes,
                &["autocapitalize", "autoCapitalize"],
            )
            .map(str::to_string),
            auto_correct: non_empty_string_attribute(attributes, &["autocorrect", "autoCorrect"])
                .map(str::to_string),
            virtual_keyboard_policy: non_empty_string_attribute(
                attributes,
                &["virtualkeyboardpolicy", "virtualKeyboardPolicy"],
            )
            .map(str::to_string),
            pattern: non_empty_string_attribute(attributes, &["pattern"]).map(str::to_string),
            min_length: u32_attribute(attributes, &["minlength", "minLength"]),
            max_length: u32_attribute(attributes, &["maxlength", "maxLength"]),
            rows: positive_u32_attribute(attributes, &["rows"]),
            cols: positive_u32_attribute(attributes, &["cols"]),
            size: positive_u32_attribute(attributes, &["size"]),
        }
    }
}

fn parse_orientation(value: &str) -> Option<Orientation> {
    match value.trim().to_ascii_lowercase().as_str() {
        "horizontal" => Some(Orientation::Horizontal),
        "vertical" => Some(Orientation::Vertical),
        _ => None,
    }
}
