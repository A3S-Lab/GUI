use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq)]
pub struct UseRangeProps {
    value_number: f64,
    min_value: f64,
    max_value: f64,
    step_value: f64,
    on_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSliderTrackProps {
    orientation: Option<String>,
    is_disabled: bool,
}

impl UseSliderTrackProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = orientation
            .map(Into::into)
            .filter(|orientation| !orientation.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseSliderFillProps {
    orientation: Option<String>,
    value_number: f64,
    is_disabled: bool,
}

impl Default for UseSliderFillProps {
    fn default() -> Self {
        Self {
            orientation: None,
            value_number: 0.0,
            is_disabled: false,
        }
    }
}

impl UseSliderFillProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = orientation
            .map(Into::into)
            .filter(|orientation| !orientation.is_empty());
        self
    }

    pub fn value_number(mut self, value_number: f64) -> Self {
        self.value_number = value_number;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseSliderOutputProps {
    label: Option<String>,
    value: Option<String>,
    value_number: f64,
}

impl Default for UseSliderOutputProps {
    fn default() -> Self {
        Self {
            label: None,
            value: None,
            value_number: 0.0,
        }
    }
}

impl UseSliderOutputProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into).filter(|value| !value.is_empty());
        self
    }

    pub fn value_number(mut self, value_number: f64) -> Self {
        self.value_number = value_number;
        self
    }
}

impl Default for UseRangeProps {
    fn default() -> Self {
        Self {
            value_number: 0.0,
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

impl UseRangeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value_number(mut self, value_number: f64) -> Self {
        self.value_number = value_number;
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
pub struct UseRangeResult {
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub value_percent: f64,
    pub range_props: RangeProps,
    pub range_input_props: RangeInputProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeProps {
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
pub struct RangeInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
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
pub struct UseSliderTrackResult {
    pub orientation: &'static str,
    pub is_disabled: bool,
    pub slider_track_props: SliderTrackProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSliderFillResult {
    pub orientation: &'static str,
    pub value_number: f64,
    pub is_disabled: bool,
    pub slider_fill_props: SliderFillProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSliderOutputResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub value_number: f64,
    pub slider_output_props: SliderOutputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderTrackProps {
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderFillProps {
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
    pub value_number: f64,
    #[serde(rename = "data-value-number")]
    pub data_value_number: f64,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderOutputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    pub value_number: f64,
    #[serde(rename = "data-value-number")]
    pub data_value_number: f64,
}

pub fn use_range(props: UseRangeProps) -> UseRangeResult {
    let min_value = finite_or(props.min_value, 0.0);
    let max_value = finite_or(props.max_value, 100.0).max(min_value);
    let step_value = finite_or(props.step_value, 1.0);
    let step_value = if step_value > 0.0 { step_value } else { 1.0 };
    let value_number = finite_or(props.value_number, min_value).clamp(min_value, max_value);
    let value_percent = if max_value > min_value {
        ((value_number - min_value) / (max_value - min_value) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let range_props = RangeProps {
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

    UseRangeResult {
        value_number,
        min_value,
        max_value,
        step_value,
        value_percent,
        range_input_props: RangeInputProps {
            input_type: "number",
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
        },
        range_props,
    }
}

pub fn use_slider_track(props: UseSliderTrackProps) -> UseSliderTrackResult {
    let orientation = orientation_value(props.orientation);

    UseSliderTrackResult {
        orientation,
        is_disabled: props.is_disabled,
        slider_track_props: SliderTrackProps {
            orientation,
            data_orientation: orientation,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_slider_fill(props: UseSliderFillProps) -> UseSliderFillResult {
    let orientation = orientation_value(props.orientation);
    let value_number = finite_or(props.value_number, 0.0);

    UseSliderFillResult {
        orientation,
        value_number,
        is_disabled: props.is_disabled,
        slider_fill_props: SliderFillProps {
            orientation,
            data_orientation: orientation,
            value_number,
            data_value_number: value_number,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_slider_output(props: UseSliderOutputProps) -> UseSliderOutputResult {
    let value_number = finite_or(props.value_number, 0.0);

    UseSliderOutputResult {
        label: props.label.clone(),
        value: props.value.clone(),
        value_number,
        slider_output_props: SliderOutputProps {
            label: props.label,
            data_value: props.value.clone(),
            value: props.value,
            value_number,
            data_value_number: value_number,
        },
    }
}

pub fn use_range_value(props: UseRangeProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_range(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_range hook did not serialize: {error}"
        ))
    })
}

pub fn use_slider_track_value(props: UseSliderTrackProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_slider_track(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_slider_track hook did not serialize: {error}"
        ))
    })
}

pub fn use_slider_fill_value(props: UseSliderFillProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_slider_fill(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_slider_fill hook did not serialize: {error}"
        ))
    })
}

pub fn use_slider_output_value(props: UseSliderOutputProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_slider_output(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_slider_output hook did not serialize: {error}"
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

fn orientation_value(orientation: Option<String>) -> &'static str {
    match orientation
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("vertical") => "vertical",
        _ => "horizontal",
    }
}
