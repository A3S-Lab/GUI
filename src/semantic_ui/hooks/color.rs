use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorFieldProps {
    label: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    color_space: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseColorFieldProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
        self
    }

    pub fn color_space(mut self, color_space: Option<impl Into<String>>) -> Self {
        self.color_space = non_empty(color_space);
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub color_field_props: ColorFieldProps,
    pub color_field_input_props: ColorFieldInputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorFieldProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-color-space", skip_serializing_if = "Option::is_none")]
    pub data_color_space: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorFieldInputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_input: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorPickerProps {
    label: Option<String>,
    value: Option<String>,
    on_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
}

impl UseColorPickerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorPickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub color_picker_props: ColorPickerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPickerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UseColorAreaProps {
    label: Option<String>,
    value: Option<String>,
    x_channel: Option<String>,
    y_channel: Option<String>,
    x_value: f64,
    y_value: f64,
    on_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
}

impl UseColorAreaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn x_channel(mut self, x_channel: Option<impl Into<String>>) -> Self {
        self.x_channel = non_empty(x_channel);
        self
    }

    pub fn y_channel(mut self, y_channel: Option<impl Into<String>>) -> Self {
        self.y_channel = non_empty(y_channel);
        self
    }

    pub fn x_value(mut self, x_value: f64) -> Self {
        self.x_value = x_value;
        self
    }

    pub fn y_value(mut self, y_value: f64) -> Self {
        self.y_value = y_value;
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorAreaResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_channel: Option<String>,
    pub x_value: f64,
    pub y_value: f64,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub color_area_props: ColorAreaProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorAreaProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(rename = "data-x-channel", skip_serializing_if = "Option::is_none")]
    pub data_x_channel: Option<String>,
    #[serde(rename = "data-y-channel", skip_serializing_if = "Option::is_none")]
    pub data_y_channel: Option<String>,
    #[serde(rename = "data-x-value")]
    pub data_x_value: f64,
    #[serde(rename = "data-y-value")]
    pub data_y_value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseColorRangeProps {
    label: Option<String>,
    channel: Option<String>,
    value_number: f64,
    min_value: f64,
    max_value: f64,
    step_value: f64,
    on_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
}

impl Default for UseColorRangeProps {
    fn default() -> Self {
        Self {
            label: None,
            channel: None,
            value_number: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            step_value: 1.0,
            on_change: None,
            is_disabled: false,
            is_read_only: false,
        }
    }
}

impl UseColorRangeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn channel(mut self, channel: Option<impl Into<String>>) -> Self {
        self.channel = non_empty(channel);
        self
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
        self.on_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSliderResult {
    #[serde(flatten)]
    pub range: ColorRangeState,
    pub color_slider_props: ColorRangeProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorWheelResult {
    #[serde(flatten)]
    pub range: ColorRangeState,
    pub color_wheel_props: ColorRangeProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorRangeState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub value_percent: f64,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorRangeProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-channel", skip_serializing_if = "Option::is_none")]
    pub data_channel: Option<String>,
    #[serde(rename = "data-value")]
    pub data_value: f64,
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
    pub readonly: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorSwatchPickerProps {
    label: Option<String>,
    value: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    selection_mode: Option<String>,
}

impl UseColorSwatchPickerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_selection_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn selection_mode(mut self, selection_mode: Option<impl Into<String>>) -> Self {
        self.selection_mode = non_empty(selection_mode);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSwatchPickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selection_mode: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub color_swatch_picker_props: ColorSwatchPickerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSwatchPickerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_selection_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
    #[serde(
        rename = "data-selected-value",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_selected_value: Option<String>,
    #[serde(rename = "data-selection-mode")]
    pub data_selection_mode: String,
    #[serde(rename = "aria-multiselectable", skip_serializing_if = "is_false")]
    pub aria_multiselectable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorSwatchPickerItemProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
}

impl UseColorSwatchPickerItemProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = non_empty(text_value);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSwatchPickerItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub color_swatch_picker_item_props: ColorSwatchPickerItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSwatchPickerItemProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub selected: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorSwatchProps {
    label: Option<String>,
    value: Option<String>,
    is_disabled: bool,
}

impl UseColorSwatchProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSwatchResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub is_disabled: bool,
    pub color_swatch_props: ColorSwatchProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSwatchProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseColorThumbProps {
    value: Option<String>,
    x_value: f64,
    y_value: f64,
    action_value: Option<String>,
    action_payload: JsonValue,
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    is_disabled: bool,
    is_pressed: bool,
    is_dragging: bool,
}

impl Default for UseColorThumbProps {
    fn default() -> Self {
        Self {
            value: None,
            x_value: 0.0,
            y_value: 0.0,
            action_value: None,
            action_payload: JsonValue::Null,
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            is_disabled: false,
            is_pressed: false,
            is_dragging: false,
        }
    }
}

impl UseColorThumbProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn x_value(mut self, x_value: f64) -> Self {
        self.x_value = x_value;
        self
    }

    pub fn y_value(mut self, y_value: f64) -> Self {
        self.y_value = y_value;
        self
    }

    pub fn action_value(mut self, action_value: Option<impl Into<String>>) -> Self {
        self.action_value = non_empty(action_value);
        self
    }

    pub fn action_payload(mut self, action_payload: JsonValue) -> Self {
        self.action_payload = action_payload;
        self
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = non_empty(action);
        self
    }

    pub fn on_press_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_start = non_empty(action);
        self
    }

    pub fn on_press_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_end = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.is_pressed = pressed;
        self
    }

    pub fn dragging(mut self, dragging: bool) -> Self {
        self.is_dragging = dragging;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorThumbResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub x_value: f64,
    pub y_value: f64,
    pub is_pressed: bool,
    pub is_dragging: bool,
    pub is_disabled: bool,
    pub color_thumb_props: ColorThumbProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorThumbProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(rename = "data-x-value")]
    pub data_x_value: f64,
    #[serde(rename = "data-y-value")]
    pub data_y_value: f64,
    #[serde(rename = "data-dragging")]
    pub data_dragging: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    pub action_payload: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_end: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_color_field(props: UseColorFieldProps) -> UseColorFieldResult {
    UseColorFieldResult {
        label: props.label.clone(),
        value: props.value.clone(),
        placeholder: props.placeholder.clone(),
        color_space: props.color_space.clone(),
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        color_field_props: ColorFieldProps {
            label: props.label,
            data_color_space: props.color_space,
            disabled: props.is_disabled,
            required: props.is_required,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
        color_field_input_props: ColorFieldInputProps {
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_change,
            disabled: props.is_disabled,
            required: props.is_required,
            readonly: props.is_read_only,
            aria_invalid: props.is_invalid,
        },
    }
}

pub fn use_color_picker(props: UseColorPickerProps) -> UseColorPickerResult {
    UseColorPickerResult {
        label: props.label.clone(),
        value: props.value.clone(),
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        color_picker_props: ColorPickerProps {
            label: props.label,
            value: props.value.clone(),
            data_value: props.value,
            on_change: props.on_change,
            disabled: props.is_disabled,
            readonly: props.is_read_only,
        },
    }
}

pub fn use_color_area(props: UseColorAreaProps) -> UseColorAreaResult {
    let x_value = finite_or(props.x_value, 0.0).clamp(0.0, 100.0);
    let y_value = finite_or(props.y_value, 0.0).clamp(0.0, 100.0);

    UseColorAreaResult {
        label: props.label.clone(),
        value: props.value.clone(),
        x_channel: props.x_channel.clone(),
        y_channel: props.y_channel.clone(),
        x_value,
        y_value,
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        color_area_props: ColorAreaProps {
            label: props.label,
            data_value: props.value,
            data_x_channel: props.x_channel,
            data_y_channel: props.y_channel,
            data_x_value: x_value,
            data_y_value: y_value,
            on_change: props.on_change,
            disabled: props.is_disabled,
            readonly: props.is_read_only,
        },
    }
}

pub fn use_color_slider(props: UseColorRangeProps) -> UseColorSliderResult {
    let (range, color_slider_props) = color_range(props, None);
    UseColorSliderResult {
        range,
        color_slider_props,
    }
}

pub fn use_color_wheel(props: UseColorRangeProps) -> UseColorWheelResult {
    let (range, color_wheel_props) = color_range(props, Some("hue"));
    UseColorWheelResult {
        range,
        color_wheel_props,
    }
}

pub fn use_color_swatch_picker(props: UseColorSwatchPickerProps) -> UseColorSwatchPickerResult {
    let selection_mode = match props.selection_mode.as_deref() {
        Some("none") => "none",
        Some("multiple") => "multiple",
        _ => "single",
    }
    .to_string();
    let is_multiple = selection_mode == "multiple";

    UseColorSwatchPickerResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        selection_mode: selection_mode.clone(),
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        color_swatch_picker_props: ColorSwatchPickerProps {
            label: props.label,
            value: props.value.clone(),
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            data_selected_value: props.value,
            data_selection_mode: selection_mode,
            aria_multiselectable: is_multiple,
        },
    }
}

pub fn use_color_swatch_picker_item(
    props: UseColorSwatchPickerItemProps,
) -> UseColorSwatchPickerItemResult {
    UseColorSwatchPickerItemResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        color_swatch_picker_item_props: ColorSwatchPickerItemProps {
            value: props.value.clone(),
            text_value: props.text_value,
            data_value: props.value,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_color_swatch(props: UseColorSwatchProps) -> UseColorSwatchResult {
    UseColorSwatchResult {
        label: props.label.clone(),
        value: props.value.clone(),
        is_disabled: props.is_disabled,
        color_swatch_props: ColorSwatchProps {
            label: props.label,
            data_value: props.value,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_color_thumb(props: UseColorThumbProps) -> UseColorThumbResult {
    let x_value = finite_or(props.x_value, 0.0).clamp(0.0, 100.0);
    let y_value = finite_or(props.y_value, 0.0).clamp(0.0, 100.0);

    UseColorThumbResult {
        value: props.value.clone(),
        x_value,
        y_value,
        is_pressed: props.is_pressed,
        is_dragging: props.is_dragging,
        is_disabled: props.is_disabled,
        color_thumb_props: ColorThumbProps {
            role: "button",
            tab_index: if props.is_disabled { -1 } else { 0 },
            value: props.value.clone(),
            data_value: props.value,
            data_x_value: x_value,
            data_y_value: y_value,
            data_dragging: props.is_dragging,
            data_pressed: props.is_pressed,
            action_value: props.action_value,
            action_payload: props.action_payload,
            on_press: props.on_press,
            on_press_start: props.on_press_start,
            on_press_end: props.on_press_end,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_color_field_value(props: UseColorFieldProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_field", use_color_field(props))
}

pub fn use_color_picker_value(props: UseColorPickerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_picker", use_color_picker(props))
}

pub fn use_color_area_value(props: UseColorAreaProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_area", use_color_area(props))
}

pub fn use_color_slider_value(props: UseColorRangeProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_slider", use_color_slider(props))
}

pub fn use_color_wheel_value(props: UseColorRangeProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_wheel", use_color_wheel(props))
}

pub fn use_color_swatch_picker_value(props: UseColorSwatchPickerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_swatch_picker", use_color_swatch_picker(props))
}

pub fn use_color_swatch_picker_item_value(
    props: UseColorSwatchPickerItemProps,
) -> GuiResult<JsonValue> {
    serialize_hook(
        "use_color_swatch_picker_item",
        use_color_swatch_picker_item(props),
    )
}

pub fn use_color_swatch_value(props: UseColorSwatchProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_swatch", use_color_swatch(props))
}

pub fn use_color_thumb_value(props: UseColorThumbProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_thumb", use_color_thumb(props))
}

fn color_range(
    props: UseColorRangeProps,
    fallback_channel: Option<&str>,
) -> (ColorRangeState, ColorRangeProps) {
    let min_value = finite_or(props.min_value, 0.0);
    let max_value = finite_or(props.max_value, 100.0).max(min_value);
    let step_value = {
        let value = finite_or(props.step_value, 1.0);
        if value > 0.0 {
            value
        } else {
            1.0
        }
    };
    let value_number = finite_or(props.value_number, min_value).clamp(min_value, max_value);
    let value_percent = if max_value > min_value {
        ((value_number - min_value) / (max_value - min_value) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let channel = props
        .channel
        .or_else(|| fallback_channel.map(ToOwned::to_owned));

    let state = ColorRangeState {
        label: props.label.clone(),
        channel: channel.clone(),
        value_number,
        min_value,
        max_value,
        step_value,
        value_percent,
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
    };
    let color_range_props = ColorRangeProps {
        label: props.label,
        data_channel: channel,
        data_value: value_number,
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
        readonly: props.is_read_only,
        aria_read_only: props.is_read_only,
    };

    (state, color_range_props)
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}

fn finite_or(value: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
