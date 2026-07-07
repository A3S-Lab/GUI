use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDateFieldProps {
    label: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    granularity: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseDateFieldProps {
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

    pub fn granularity(mut self, granularity: Option<impl Into<String>>) -> Self {
        self.granularity = non_empty(granularity);
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
pub struct UseDateFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub date_field_props: DateFieldProps,
    pub date_field_input_props: DateFieldInputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFieldProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
    #[serde(rename = "data-granularity", skip_serializing_if = "Option::is_none")]
    pub data_granularity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFieldInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
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
pub struct UseTimeFieldProps {
    label: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    granularity: Option<String>,
    hour_cycle: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseTimeFieldProps {
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

    pub fn granularity(mut self, granularity: Option<impl Into<String>>) -> Self {
        self.granularity = non_empty(granularity);
        self
    }

    pub fn hour_cycle(mut self, hour_cycle: Option<impl Into<String>>) -> Self {
        self.hour_cycle = non_empty(hour_cycle);
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
pub struct UseTimeFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_cycle: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub time_field_props: TimeFieldProps,
    pub time_field_input_props: TimeFieldInputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFieldProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
    #[serde(rename = "data-granularity", skip_serializing_if = "Option::is_none")]
    pub data_granularity: Option<String>,
    #[serde(rename = "data-hour-cycle", skip_serializing_if = "Option::is_none")]
    pub data_hour_cycle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFieldInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
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
pub struct UseDateInputProps {
    label: Option<String>,
    value: Option<String>,
    is_disabled: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseDateInputProps {
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
pub struct UseDateInputResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub date_input_props: DateInputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateInputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDateSegmentProps {
    segment_type: Option<String>,
    value: Option<String>,
    text_value: Option<String>,
    placeholder: Option<String>,
    is_placeholder: bool,
    is_disabled: bool,
    is_invalid: bool,
}

impl UseDateSegmentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn segment_type(mut self, segment_type: Option<impl Into<String>>) -> Self {
        self.segment_type = non_empty(segment_type);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = non_empty(text_value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn placeholder_segment(mut self, placeholder: bool) -> Self {
        self.is_placeholder = placeholder;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDateSegmentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub is_placeholder: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub date_segment_props: DateSegmentProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateSegmentProps {
    #[serde(rename = "data-type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "data-placeholder")]
    pub data_placeholder: bool,
    #[serde(rename = "data-invalid")]
    pub data_invalid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCalendarProps {
    label: Option<String>,
    value: Option<String>,
    on_change: Option<String>,
    is_disabled: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseCalendarProps {
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
pub struct UseCalendarResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub calendar_props: CalendarProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarProps {
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
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseRangeCalendarProps {
    label: Option<String>,
    start_value: Option<String>,
    end_value: Option<String>,
    on_change: Option<String>,
    is_disabled: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseRangeCalendarProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn start_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.start_value = non_empty(value);
        self
    }

    pub fn end_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.end_value = non_empty(value);
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
pub struct UseRangeCalendarResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub range_calendar_props: RangeCalendarProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeCalendarProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-start-value", skip_serializing_if = "Option::is_none")]
    pub data_start_value: Option<String>,
    #[serde(rename = "data-end-value", skip_serializing_if = "Option::is_none")]
    pub data_end_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCalendarCellProps {
    value: Option<String>,
    text_value: Option<String>,
    action_value: Option<String>,
    on_press: Option<String>,
    is_selected: bool,
    is_disabled: bool,
    is_unavailable: bool,
    is_outside_month: bool,
    is_today: bool,
    is_pressed: bool,
}

impl UseCalendarCellProps {
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

    pub fn action_value(mut self, action_value: Option<impl Into<String>>) -> Self {
        self.action_value = non_empty(action_value);
        self
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = non_empty(action);
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

    pub fn unavailable(mut self, unavailable: bool) -> Self {
        self.is_unavailable = unavailable;
        self
    }

    pub fn outside_month(mut self, outside_month: bool) -> Self {
        self.is_outside_month = outside_month;
        self
    }

    pub fn today(mut self, today: bool) -> Self {
        self.is_today = today;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.is_pressed = pressed;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCalendarCellResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_unavailable: bool,
    pub is_outside_month: bool,
    pub is_today: bool,
    pub is_pressed: bool,
    pub calendar_cell_props: CalendarCellProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarCellProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
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
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
    #[serde(rename = "data-unavailable")]
    pub data_unavailable: bool,
    #[serde(rename = "data-outside-month")]
    pub data_outside_month: bool,
    #[serde(rename = "data-today")]
    pub data_today: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDatePickerProps {
    label: Option<String>,
    value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseDatePickerProps {
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

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = non_empty(action);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
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
pub struct UseDatePickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub date_picker_props: DatePickerProps,
    pub date_picker_input_props: DatePickerInputProps,
    pub date_picker_trigger_props: DatePickerTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatePickerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_open_change: Option<String>,
    #[serde(rename = "data-open")]
    pub data_open: bool,
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
pub struct DatePickerInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatePickerTriggerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDateRangePickerProps {
    label: Option<String>,
    start_value: Option<String>,
    end_value: Option<String>,
    placeholder: Option<String>,
    on_start_change: Option<String>,
    on_end_change: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseDateRangePickerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn start_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.start_value = non_empty(value);
        self
    }

    pub fn end_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.end_value = non_empty(value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_start_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_start_change = non_empty(action);
        self
    }

    pub fn on_end_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_end_change = non_empty(action);
        self
    }

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = non_empty(action);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
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
pub struct UseDateRangePickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub date_range_picker_props: DateRangePickerProps,
    pub date_range_picker_start_input_props: DateRangePickerInputProps,
    pub date_range_picker_end_input_props: DateRangePickerInputProps,
    pub date_range_picker_trigger_props: DatePickerTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRangePickerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_open_change: Option<String>,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "data-start-value", skip_serializing_if = "Option::is_none")]
    pub data_start_value: Option<String>,
    #[serde(rename = "data-end-value", skip_serializing_if = "Option::is_none")]
    pub data_end_value: Option<String>,
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
pub struct DateRangePickerInputProps {
    #[serde(rename = "type")]
    pub input_type: &'static str,
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

pub fn use_date_field(props: UseDateFieldProps) -> UseDateFieldResult {
    UseDateFieldResult {
        label: props.label.clone(),
        value: props.value.clone(),
        placeholder: props.placeholder.clone(),
        granularity: props.granularity.clone(),
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        date_field_props: DateFieldProps {
            label: props.label,
            disabled: props.is_disabled,
            required: props.is_required,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
            data_granularity: props.granularity,
        },
        date_field_input_props: DateFieldInputProps {
            input_type: "date",
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

pub fn use_time_field(props: UseTimeFieldProps) -> UseTimeFieldResult {
    UseTimeFieldResult {
        label: props.label.clone(),
        value: props.value.clone(),
        placeholder: props.placeholder.clone(),
        granularity: props.granularity.clone(),
        hour_cycle: props.hour_cycle.clone(),
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        time_field_props: TimeFieldProps {
            label: props.label,
            disabled: props.is_disabled,
            required: props.is_required,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
            data_granularity: props.granularity,
            data_hour_cycle: props.hour_cycle,
        },
        time_field_input_props: TimeFieldInputProps {
            input_type: "time",
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

pub fn use_date_input(props: UseDateInputProps) -> UseDateInputResult {
    UseDateInputResult {
        label: props.label.clone(),
        value: props.value.clone(),
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        date_input_props: DateInputProps {
            label: props.label,
            data_value: props.value,
            disabled: props.is_disabled,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
    }
}

pub fn use_date_segment(props: UseDateSegmentProps) -> UseDateSegmentResult {
    let label = props.value.clone().or_else(|| props.placeholder.clone());
    let text_value = props
        .text_value
        .clone()
        .or_else(|| props.value.clone())
        .or_else(|| props.placeholder.clone());

    UseDateSegmentResult {
        segment_type: props.segment_type.clone(),
        value: props.value,
        text_value: text_value.clone(),
        placeholder: props.placeholder,
        is_placeholder: props.is_placeholder,
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        date_segment_props: DateSegmentProps {
            data_type: props.segment_type,
            data_placeholder: props.is_placeholder,
            data_invalid: props.is_invalid,
            label,
            text_value,
            disabled: props.is_disabled,
        },
    }
}

pub fn use_calendar(props: UseCalendarProps) -> UseCalendarResult {
    UseCalendarResult {
        label: props.label.clone(),
        value: props.value.clone(),
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        calendar_props: CalendarProps {
            label: props.label,
            value: props.value.clone(),
            data_value: props.value,
            on_change: props.on_change,
            disabled: props.is_disabled,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
    }
}

pub fn use_range_calendar(props: UseRangeCalendarProps) -> UseRangeCalendarResult {
    UseRangeCalendarResult {
        label: props.label.clone(),
        start_value: props.start_value.clone(),
        end_value: props.end_value.clone(),
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        range_calendar_props: RangeCalendarProps {
            label: props.label,
            data_start_value: props.start_value,
            data_end_value: props.end_value,
            on_change: props.on_change,
            disabled: props.is_disabled,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
    }
}

pub fn use_calendar_cell(props: UseCalendarCellProps) -> UseCalendarCellResult {
    let disabled = props.is_disabled || props.is_unavailable;

    UseCalendarCellResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: disabled,
        is_unavailable: props.is_unavailable,
        is_outside_month: props.is_outside_month,
        is_today: props.is_today,
        is_pressed: props.is_pressed,
        calendar_cell_props: CalendarCellProps {
            role: "button",
            tab_index: if disabled { -1 } else { 0 },
            value: props.value.clone(),
            text_value: props.text_value,
            data_value: props.value,
            action_value: props.action_value,
            on_press: props.on_press,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
            disabled,
            aria_disabled: disabled,
            data_disabled: disabled,
            data_unavailable: props.is_unavailable,
            data_outside_month: props.is_outside_month,
            data_today: props.is_today,
            data_pressed: props.is_pressed,
        },
    }
}

pub fn use_date_picker(props: UseDatePickerProps) -> UseDatePickerResult {
    UseDatePickerResult {
        label: props.label.clone(),
        value: props.value.clone(),
        placeholder: props.placeholder.clone(),
        is_open: props.is_open,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        date_picker_props: DatePickerProps {
            label: props.label,
            value: props.value.clone(),
            on_change: props.on_change.clone(),
            on_open_change: props.on_open_change.clone(),
            data_open: props.is_open,
            disabled: props.is_disabled,
            required: props.is_required,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
        date_picker_input_props: DatePickerInputProps {
            input_type: "date",
            value: props.value,
            placeholder: props.placeholder,
            on_input: props.on_change,
            disabled: props.is_disabled,
            required: props.is_required,
            readonly: props.is_read_only,
            aria_invalid: props.is_invalid,
        },
        date_picker_trigger_props: DatePickerTriggerProps {
            on_press: props.on_open_change,
            disabled: props.is_disabled,
        },
    }
}

pub fn use_date_range_picker(props: UseDateRangePickerProps) -> UseDateRangePickerResult {
    let shared_input =
        |value: Option<String>, on_input: Option<String>| DateRangePickerInputProps {
            input_type: "date",
            value,
            placeholder: props.placeholder.clone(),
            on_input,
            disabled: props.is_disabled,
            required: props.is_required,
            readonly: props.is_read_only,
            aria_invalid: props.is_invalid,
        };

    UseDateRangePickerResult {
        label: props.label.clone(),
        start_value: props.start_value.clone(),
        end_value: props.end_value.clone(),
        placeholder: props.placeholder.clone(),
        is_open: props.is_open,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        date_range_picker_props: DateRangePickerProps {
            label: props.label,
            on_open_change: props.on_open_change.clone(),
            data_open: props.is_open,
            data_start_value: props.start_value.clone(),
            data_end_value: props.end_value.clone(),
            disabled: props.is_disabled,
            required: props.is_required,
            aria_invalid: props.is_invalid,
            readonly: props.is_read_only,
        },
        date_range_picker_start_input_props: shared_input(props.start_value, props.on_start_change),
        date_range_picker_end_input_props: shared_input(props.end_value, props.on_end_change),
        date_range_picker_trigger_props: DatePickerTriggerProps {
            on_press: props.on_open_change,
            disabled: props.is_disabled,
        },
    }
}

pub fn use_date_field_value(props: UseDateFieldProps) -> GuiResult<JsonValue> {
    serialize_hook("use_date_field", use_date_field(props))
}

pub fn use_time_field_value(props: UseTimeFieldProps) -> GuiResult<JsonValue> {
    serialize_hook("use_time_field", use_time_field(props))
}

pub fn use_date_input_value(props: UseDateInputProps) -> GuiResult<JsonValue> {
    serialize_hook("use_date_input", use_date_input(props))
}

pub fn use_date_segment_value(props: UseDateSegmentProps) -> GuiResult<JsonValue> {
    serialize_hook("use_date_segment", use_date_segment(props))
}

pub fn use_calendar_value(props: UseCalendarProps) -> GuiResult<JsonValue> {
    serialize_hook("use_calendar", use_calendar(props))
}

pub fn use_range_calendar_value(props: UseRangeCalendarProps) -> GuiResult<JsonValue> {
    serialize_hook("use_range_calendar", use_range_calendar(props))
}

pub fn use_calendar_cell_value(props: UseCalendarCellProps) -> GuiResult<JsonValue> {
    serialize_hook("use_calendar_cell", use_calendar_cell(props))
}

pub fn use_date_picker_value(props: UseDatePickerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_date_picker", use_date_picker(props))
}

pub fn use_date_range_picker_value(props: UseDateRangePickerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_date_range_picker", use_date_range_picker(props))
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
