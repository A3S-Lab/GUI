use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseToggleProps {
    on_change: Option<String>,
    is_selected: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseToggleButtonProps {
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    on_press_up: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    is_selected: bool,
    is_disabled: bool,
    is_pressed: bool,
}

impl Default for UseToggleButtonProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            action_value: None,
            action_payload: JsonValue::Null,
            is_selected: false,
            is_disabled: false,
            is_pressed: false,
        }
    }
}

impl UseToggleButtonProps {
    pub fn new() -> Self {
        Self::default()
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

    pub fn on_press_up(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_up = non_empty(action);
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

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseToggleButtonGroupProps {
    label: Option<String>,
    value: Option<String>,
    orientation: ToggleButtonGroupOrientation,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    selection_mode: ToggleButtonGroupSelectionMode,
}

impl UseToggleButtonGroupProps {
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

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = ToggleButtonGroupOrientation::from_option(orientation);
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
        self.selection_mode = ToggleButtonGroupSelectionMode::from_option(selection_mode);
        self
    }
}

impl UseToggleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn checked(self, checked: bool) -> Self {
        self.selected(checked)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ToggleButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleButtonGroupOrientation {
    fn from_option(value: Option<impl Into<String>>) -> Self {
        match value
            .map(Into::into)
            .map(|value| value.to_ascii_lowercase())
            .as_deref()
        {
            Some("vertical") => Self::Vertical,
            _ => Self::Horizontal,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ToggleButtonGroupSelectionMode {
    None,
    #[default]
    Single,
    Multiple,
}

impl ToggleButtonGroupSelectionMode {
    fn from_option(value: Option<impl Into<String>>) -> Self {
        match value
            .map(Into::into)
            .map(|value| value.to_ascii_lowercase())
            .as_deref()
        {
            Some("none") => Self::None,
            Some("multiple") => Self::Multiple,
            _ => Self::Single,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseToggleResult {
    pub is_selected: bool,
    pub is_checked: bool,
    pub toggle_props: ToggleProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleProps {
    pub checked: bool,
    #[serde(rename = "aria-checked")]
    pub aria_checked: bool,
    #[serde(rename = "data-checked")]
    pub data_checked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
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
pub struct UseToggleButtonResult {
    pub is_selected: bool,
    pub is_pressed: bool,
    pub is_disabled: bool,
    pub toggle_button_props: ToggleButtonProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleButtonProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_up: Option<String>,
    #[serde(rename = "actionValue", skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(rename = "actionPayload", skip_serializing_if = "JsonValue::is_null")]
    pub action_payload: JsonValue,
    pub selected: bool,
    #[serde(rename = "aria-pressed")]
    pub aria_pressed: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseToggleButtonGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub orientation: &'static str,
    pub selection_mode: &'static str,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub toggle_button_group_props: ToggleButtonGroupProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleButtonGroupProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_selection_change: Option<String>,
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
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
    pub data_selection_mode: &'static str,
    #[serde(rename = "aria-multiselectable", skip_serializing_if = "is_false")]
    pub aria_multiselectable: bool,
}

pub fn use_toggle(props: UseToggleProps) -> UseToggleResult {
    UseToggleResult {
        is_selected: props.is_selected,
        is_checked: props.is_selected,
        toggle_props: ToggleProps {
            checked: props.is_selected,
            aria_checked: props.is_selected,
            data_checked: props.is_selected,
            on_change: props.on_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
        },
    }
}

pub fn use_toggle_button(props: UseToggleButtonProps) -> UseToggleButtonResult {
    UseToggleButtonResult {
        is_selected: props.is_selected,
        is_pressed: props.is_pressed,
        is_disabled: props.is_disabled,
        toggle_button_props: ToggleButtonProps {
            role: "button",
            tab_index: if props.is_disabled { -1 } else { 0 },
            on_press: props.on_press,
            on_press_start: props.on_press_start,
            on_press_end: props.on_press_end,
            on_press_up: props.on_press_up,
            action_value: props.action_value,
            action_payload: props.action_payload,
            selected: props.is_selected,
            aria_pressed: props.is_selected,
            data_selected: props.is_selected,
            data_pressed: props.is_pressed,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_toggle_button_group(props: UseToggleButtonGroupProps) -> UseToggleButtonGroupResult {
    let orientation = props.orientation.as_str();
    let selection_mode = props.selection_mode.as_str();
    let is_multiple = props.selection_mode == ToggleButtonGroupSelectionMode::Multiple;

    UseToggleButtonGroupResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        orientation,
        selection_mode,
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        toggle_button_group_props: ToggleButtonGroupProps {
            label: props.label,
            value: props.value.clone(),
            on_selection_change: props.on_selection_change,
            orientation,
            data_orientation: orientation,
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

pub fn use_toggle_value(props: UseToggleProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_toggle(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_toggle hook did not serialize: {error}"
        ))
    })
}

pub fn use_toggle_button_value(props: UseToggleButtonProps) -> GuiResult<JsonValue> {
    serialize_hook("use_toggle_button", use_toggle_button(props))
}

pub fn use_toggle_button_group_value(props: UseToggleButtonGroupProps) -> GuiResult<JsonValue> {
    serialize_hook("use_toggle_button_group", use_toggle_button_group(props))
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
