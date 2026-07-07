use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq)]
pub struct UseNumberFieldProps {
    label: Option<String>,
    value_number: f64,
    placeholder: Option<String>,
    min_value: f64,
    max_value: f64,
    step_value: f64,
    on_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl Default for UseNumberFieldProps {
    fn default() -> Self {
        Self {
            label: None,
            value_number: 0.0,
            placeholder: None,
            min_value: 0.0,
            max_value: 100.0,
            step_value: 1.0,
            on_change: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
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
        self.step_value = step_value;
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = action.map(Into::into).filter(|action| !action.is_empty());
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
    pub value_percent: f64,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub number_field_props: NumberFieldProps,
    pub number_field_input_props: NumberFieldInputProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
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

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFieldInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
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

pub fn use_number_field(props: UseNumberFieldProps) -> UseNumberFieldResult {
    let min_value = finite_or(props.min_value, 0.0);
    let max_value = finite_or(props.max_value, 100.0).max(min_value);
    let step_value = positive_or(finite_or(props.step_value, 1.0), 1.0);
    let value_number = finite_or(props.value_number, min_value).clamp(min_value, max_value);
    let value_percent = if max_value > min_value {
        ((value_number - min_value) / (max_value - min_value) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let number_field_props = NumberFieldProps {
        label: props.label.clone(),
        value_number,
        min_value,
        max_value,
        step_value,
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

    let number_field_input_props = NumberFieldInputProps {
        input_type: "number",
        placeholder: props.placeholder.clone(),
        value_number,
        min_value,
        max_value,
        step_value,
        aria_value_now: value_number,
        aria_value_min: min_value,
        aria_value_max: max_value,
        data_value_number: value_number,
        data_value_percent: value_percent,
        on_change: props.on_change.clone(),
        on_input: props.on_change,
        disabled: props.is_disabled,
        aria_disabled: props.is_disabled,
        required: props.is_required,
        aria_required: props.is_required,
        invalid: props.is_invalid,
        aria_invalid: props.is_invalid,
        read_only: props.is_read_only,
        aria_read_only: props.is_read_only,
    };

    UseNumberFieldResult {
        label: props.label,
        value_number,
        placeholder: props.placeholder,
        min_value,
        max_value,
        step_value,
        value_percent,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        number_field_props,
        number_field_input_props,
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
