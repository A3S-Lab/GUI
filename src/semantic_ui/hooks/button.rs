use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseButtonProps {
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    on_press_up: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    is_disabled: bool,
    is_pressed: bool,
}

impl Default for UseButtonProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            action_value: None,
            action_payload: JsonValue::Null,
            is_disabled: false,
            is_pressed: false,
        }
    }
}

impl UseButtonProps {
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.is_pressed = pressed;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseButtonResult {
    pub is_pressed: bool,
    pub button_props: ButtonProps,
    pub press_props: ButtonProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonProps {
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
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
}

pub fn use_button(props: UseButtonProps) -> UseButtonResult {
    let button_props = ButtonProps {
        role: "button",
        tab_index: 0,
        on_press: props.on_press,
        on_press_start: props.on_press_start,
        on_press_end: props.on_press_end,
        on_press_up: props.on_press_up,
        action_value: props.action_value,
        action_payload: props.action_payload,
        disabled: props.is_disabled,
        aria_disabled: props.is_disabled,
        data_pressed: props.is_pressed,
    };

    UseButtonResult {
        is_pressed: props.is_pressed,
        press_props: button_props.clone(),
        button_props,
    }
}

pub fn use_button_value(props: UseButtonProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_button(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_button hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
