use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::i18n::{NumberFormatOptions, NumberFormatStyle, NumberGrouping, NumberSignDisplay};
use crate::native::{
    format_normalized_number, normalize_range_value, step_range_value, RangeStepDirection,
};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq)]
pub struct UseNumberFieldProps {
    label: Option<String>,
    value_number: f64,
    placeholder: Option<String>,
    min_value: f64,
    max_value: f64,
    step_value: Option<f64>,
    format_options: NumberFormatOptions,
    on_change: Option<String>,
    increment_aria_label: Option<String>,
    decrement_aria_label: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    is_wheel_disabled: bool,
}

impl Default for UseNumberFieldProps {
    fn default() -> Self {
        Self {
            label: None,
            value_number: 0.0,
            placeholder: None,
            min_value: 0.0,
            max_value: 100.0,
            step_value: None,
            format_options: NumberFormatOptions::default(),
            on_change: None,
            increment_aria_label: None,
            decrement_aria_label: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
            is_wheel_disabled: false,
        }
    }
}

impl UseNumberFieldProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn value_number(mut self, value_number: f64) -> Self {
        self.value_number = value_number;
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = placeholder
            .map(Into::into)
            .filter(|placeholder| !placeholder.is_empty());
        self
    }

    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = min_value;
        self
    }

    pub fn max_value(mut self, max_value: f64) -> Self {
        self.max_value = max_value;
        self
    }

    pub fn step_value(mut self, step_value: f64) -> Self {
        self.step_value = Some(step_value);
        self
    }

    pub fn optional_step_value(mut self, step_value: Option<f64>) -> Self {
        self.step_value = step_value;
        self
    }

    pub fn format_options(mut self, format_options: NumberFormatOptions) -> Self {
        self.format_options = format_options;
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn increment_aria_label(mut self, label: Option<impl Into<String>>) -> Self {
        self.increment_aria_label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn decrement_aria_label(mut self, label: Option<impl Into<String>>) -> Self {
        self.decrement_aria_label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn wheel_disabled(mut self, wheel_disabled: bool) -> Self {
        self.is_wheel_disabled = wheel_disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseNumberFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub value_number: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub format_options: NumberFormatOptions,
    pub format_style: NumberFormatStyle,
    pub value_percent: f64,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_wheel_disabled: bool,
    pub can_increment: bool,
    pub can_decrement: bool,
    pub number_field_props: NumberFieldProps,
    pub number_field_input_props: NumberFieldInputProps,
    pub increment_button_props: NumberFieldButtonProps,
    pub decrement_button_props: NumberFieldButtonProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
    #[serde(rename = "data-number-field-input")]
    pub data_number_field_input: bool,
    #[serde(rename = "data-number-field-announce")]
    pub data_number_field_announce: bool,
    #[serde(rename = "data-number-field-role-description")]
    pub data_number_field_role_description: &'static str,
    #[serde(
        rename = "data-number-field-wheel-disabled",
        skip_serializing_if = "is_false"
    )]
    pub data_number_field_wheel_disabled: bool,
    #[serde(rename = "aria-roledescription")]
    pub aria_role_description: &'static str,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    #[serde(rename = "data-number-style")]
    pub data_number_style: NumberFormatStyle,
    #[serde(rename = "data-number-grouping")]
    pub data_number_grouping: NumberGrouping,
    #[serde(rename = "data-number-minimum-fraction-digits")]
    pub data_number_minimum_fraction_digits: u8,
    #[serde(rename = "data-number-maximum-fraction-digits")]
    pub data_number_maximum_fraction_digits: u8,
    #[serde(rename = "data-number-sign-display")]
    pub data_number_sign_display: NumberSignDisplay,
    #[serde(rename = "aria-valuenow")]
    pub aria_value_now: f64,
    #[serde(rename = "aria-valuemin")]
    pub aria_value_min: f64,
    #[serde(rename = "aria-valuemax")]
    pub aria_value_max: f64,
    #[serde(rename = "data-value-number")]
    pub data_value_number: f64,
    #[serde(rename = "data-value-percent")]
    pub data_value_percent: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_input: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldButtonProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "aria-label")]
    pub aria_label: String,
    #[serde(rename = "actionValue")]
    pub action_value: String,
    #[serde(rename = "data-number-field-step")]
    pub data_number_field_step: &'static str,
    #[serde(
        rename = "data-number-field-step-label",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_number_field_step_label: Option<&'static str>,
    #[serde(
        rename = "data-number-field-label",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_number_field_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    pub disabled: bool,
    #[serde(rename = "aria-disabled")]
    pub aria_disabled: bool,
}

pub fn use_number_field(props: UseNumberFieldProps) -> UseNumberFieldResult {
    let min_value = finite_or(props.min_value, 0.0);
    let max_value = finite_or(props.max_value, 100.0).max(min_value);
    let default_step = match props.format_options.style {
        NumberFormatStyle::Decimal => 1.0,
        NumberFormatStyle::Percent => 0.01,
    };
    let step_value = props
        .step_value
        .map(|value| positive_or(finite_or(value, default_step), default_step))
        .unwrap_or(default_step);
    let value_number = normalize_range_value(
        finite_or(props.value_number, min_value),
        Some(min_value),
        Some(max_value),
        Some(step_value),
    )
    .unwrap_or(min_value);
    let value_percent = if max_value > min_value {
        ((value_number - min_value) / (max_value - min_value) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let increment_value = step_range_value(
        Some(value_number),
        Some(min_value),
        Some(max_value),
        Some(step_value),
        RangeStepDirection::Increment,
    )
    .unwrap_or(value_number);
    let decrement_value = step_range_value(
        Some(value_number),
        Some(min_value),
        Some(max_value),
        Some(step_value),
        RangeStepDirection::Decrement,
    )
    .unwrap_or(value_number);
    let can_increment = !props.is_disabled && !props.is_read_only && increment_value > value_number;
    let can_decrement = !props.is_disabled && !props.is_read_only && decrement_value < value_number;
    let uses_default_increment_label = props.increment_aria_label.is_none();
    let uses_default_decrement_label = props.decrement_aria_label.is_none();
    let increment_aria_label = number_field_button_label(
        props.increment_aria_label.as_deref(),
        "Increase",
        props.label.as_deref(),
    );
    let decrement_aria_label = number_field_button_label(
        props.decrement_aria_label.as_deref(),
        "Decrease",
        props.label.as_deref(),
    );

    let number_field_props = NumberFieldProps {
        role: "group",
        label: props.label.clone(),
        disabled: props.is_disabled,
        aria_disabled: props.is_disabled,
        required: props.is_required,
        aria_required: props.is_required,
        invalid: props.is_invalid,
        aria_invalid: props.is_invalid,
        read_only: props.is_read_only,
        aria_read_only: props.is_read_only,
    };

    let number_field_input_props = NumberFieldInputProps {
        input_type: "number",
        data_number_field_input: true,
        data_number_field_announce: true,
        data_number_field_role_description: "auto",
        data_number_field_wheel_disabled: props.is_wheel_disabled,
        aria_role_description: "Number field",
        aria_label: props.label.clone(),
        placeholder: props.placeholder.clone(),
        value_number,
        min_value,
        max_value,
        step_value,
        data_number_style: props.format_options.style,
        data_number_grouping: props.format_options.grouping,
        data_number_minimum_fraction_digits: props
            .format_options
            .resolved_minimum_fraction_digits(),
        data_number_maximum_fraction_digits: props
            .format_options
            .resolved_maximum_fraction_digits(),
        data_number_sign_display: props.format_options.sign_display,
        aria_value_now: value_number,
        aria_value_min: min_value,
        aria_value_max: max_value,
        data_value_number: value_number,
        data_value_percent: value_percent,
        on_change: props.on_change.clone(),
        on_input: props.on_change.clone(),
        disabled: props.is_disabled,
        aria_disabled: props.is_disabled,
        required: props.is_required,
        aria_required: props.is_required,
        invalid: props.is_invalid,
        aria_invalid: props.is_invalid,
        read_only: props.is_read_only,
        aria_read_only: props.is_read_only,
    };
    let increment_button_props = NumberFieldButtonProps {
        role: "button",
        tab_index: -1,
        aria_label: increment_aria_label,
        action_value: format_normalized_number(increment_value),
        data_number_field_step: "increment",
        data_number_field_step_label: uses_default_increment_label.then_some("auto"),
        data_number_field_label: if uses_default_increment_label {
            props.label.clone()
        } else {
            None
        },
        on_press: props.on_change.clone(),
        disabled: !can_increment,
        aria_disabled: !can_increment,
    };
    let decrement_button_props = NumberFieldButtonProps {
        role: "button",
        tab_index: -1,
        aria_label: decrement_aria_label,
        action_value: format_normalized_number(decrement_value),
        data_number_field_step: "decrement",
        data_number_field_step_label: uses_default_decrement_label.then_some("auto"),
        data_number_field_label: if uses_default_decrement_label {
            props.label.clone()
        } else {
            None
        },
        on_press: props.on_change.clone(),
        disabled: !can_decrement,
        aria_disabled: !can_decrement,
    };

    UseNumberFieldResult {
        label: props.label,
        value_number,
        placeholder: props.placeholder,
        min_value,
        max_value,
        step_value,
        format_options: props.format_options,
        format_style: props.format_options.style,
        value_percent,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        is_wheel_disabled: props.is_wheel_disabled,
        can_increment,
        can_decrement,
        number_field_props,
        number_field_input_props,
        increment_button_props,
        decrement_button_props,
    }
}

pub fn use_number_field_value(props: UseNumberFieldProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_number_field(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_number_field hook did not serialize: {error}"
        ))
    })
}

fn finite_or(value: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}

fn positive_or(value: f64, fallback: f64) -> f64 {
    if value > 0.0 {
        value
    } else {
        fallback
    }
}

fn number_field_button_label(
    custom: Option<&str>,
    action: &str,
    field_label: Option<&str>,
) -> String {
    if let Some(custom) = custom {
        return custom.to_string();
    }
    match field_label.filter(|label| !label.is_empty()) {
        Some(label) => format!("{action} {label}"),
        None => action.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{use_number_field, use_number_field_value, UseNumberFieldProps};

    #[test]
    fn number_field_exposes_stepper_parts_and_boundary_state() {
        let result = use_number_field(
            UseNumberFieldProps::new()
                .label(Some("Quantity"))
                .value_number(8.0)
                .min_value(2.0)
                .max_value(11.0)
                .step_value(3.0)
                .on_change(Some("setQuantity")),
        );

        assert!(result.can_increment);
        assert!(result.can_decrement);
        assert_eq!(result.increment_button_props.action_value, "11");
        assert_eq!(result.decrement_button_props.action_value, "5");
        assert_eq!(
            result.increment_button_props.aria_label,
            "Increase Quantity"
        );
        assert_eq!(
            result.decrement_button_props.aria_label,
            "Decrease Quantity"
        );
        assert_eq!(
            result.increment_button_props.on_press.as_deref(),
            Some("setQuantity")
        );
        assert_eq!(
            result.increment_button_props.data_number_field_step_label,
            Some("auto")
        );
        assert_eq!(
            result
                .increment_button_props
                .data_number_field_label
                .as_deref(),
            Some("Quantity")
        );
        assert!(result.number_field_input_props.data_number_field_announce);
        assert_eq!(
            result
                .number_field_input_props
                .data_number_field_role_description,
            "auto"
        );
        assert_eq!(
            result.number_field_input_props.aria_role_description,
            "Number field"
        );
        assert_eq!(result.increment_button_props.tab_index, -1);
    }

    #[test]
    fn number_field_disables_steppers_at_bounds_and_honors_custom_labels() {
        let result = use_number_field(
            UseNumberFieldProps::new()
                .label(Some("Quantity"))
                .value_number(11.0)
                .min_value(2.0)
                .max_value(11.0)
                .step_value(3.0)
                .increment_aria_label(Some("Add one batch"))
                .decrement_aria_label(Some("Remove one batch")),
        );

        assert!(!result.can_increment);
        assert!(result.increment_button_props.disabled);
        assert!(result.can_decrement);
        assert_eq!(result.increment_button_props.aria_label, "Add one batch");
        assert_eq!(result.decrement_button_props.aria_label, "Remove one batch");
        assert_eq!(
            result.increment_button_props.data_number_field_step_label,
            None
        );
        assert_eq!(
            result.decrement_button_props.data_number_field_step_label,
            None
        );
    }

    #[test]
    fn number_field_serializes_stepper_hook_paths() {
        let value =
            use_number_field_value(UseNumberFieldProps::new().wheel_disabled(true)).unwrap();

        assert!(value.get("incrementButtonProps").is_some(), "{value}");
        assert!(value.get("decrementButtonProps").is_some(), "{value}");
        assert!(value.get("canIncrement").is_some(), "{value}");
        assert!(value.get("canDecrement").is_some(), "{value}");
        assert_eq!(value.get("isWheelDisabled"), Some(&serde_json::json!(true)));
        assert_eq!(
            value
                .get("numberFieldInputProps")
                .and_then(|props| props.get("data-number-field-wheel-disabled")),
            Some(&serde_json::json!(true))
        );
    }
}
