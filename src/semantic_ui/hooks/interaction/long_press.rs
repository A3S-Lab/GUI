use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseLongPressProps {
    on_long_press_start: Option<String>,
    on_long_press_end: Option<String>,
    on_long_press: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    accessibility_description: Option<String>,
    threshold: u64,
    is_disabled: bool,
    is_pressed: bool,
    is_long_pressed: bool,
}

impl Default for UseLongPressProps {
    fn default() -> Self {
        Self {
            on_long_press_start: None,
            on_long_press_end: None,
            on_long_press: None,
            action_value: None,
            action_payload: JsonValue::Null,
            accessibility_description: None,
            threshold: 500,
            is_disabled: false,
            is_pressed: false,
            is_long_pressed: false,
        }
    }
}

impl UseLongPressProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_long_press_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_long_press_start = non_empty(action);
        self
    }

    pub fn on_long_press_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_long_press_end = non_empty(action);
        self
    }

    pub fn on_long_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_long_press = non_empty(action);
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

    pub fn accessibility_description(
        mut self,
        accessibility_description: Option<impl Into<String>>,
    ) -> Self {
        self.accessibility_description = non_empty(accessibility_description);
        self
    }

    pub fn threshold(mut self, threshold: u64) -> Self {
        self.threshold = threshold.clamp(1, 60_000);
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

    pub fn long_pressed(mut self, long_pressed: bool) -> Self {
        self.is_long_pressed = long_pressed;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseLongPressResult {
    pub is_pressed: bool,
    pub is_long_pressed: bool,
    pub long_press_props: LongPressProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongPressProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_long_press_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_long_press_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_long_press: Option<String>,
    #[serde(rename = "actionValue", skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(rename = "actionPayload", skip_serializing_if = "JsonValue::is_null")]
    pub action_payload: JsonValue,
    #[serde(rename = "aria-description", skip_serializing_if = "Option::is_none")]
    pub accessibility_description: Option<String>,
    pub threshold: u64,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
    #[serde(rename = "data-long-pressed")]
    pub data_long_pressed: bool,
}

pub fn use_long_press(props: UseLongPressProps) -> UseLongPressResult {
    UseLongPressResult {
        is_pressed: props.is_pressed,
        is_long_pressed: props.is_long_pressed,
        long_press_props: LongPressProps {
            role: "button",
            tab_index: 0,
            on_long_press_start: props.on_long_press_start,
            on_long_press_end: props.on_long_press_end,
            on_long_press: props.on_long_press,
            action_value: props.action_value,
            action_payload: props.action_payload,
            accessibility_description: props.accessibility_description,
            threshold: props.threshold,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_pressed: props.is_pressed,
            data_long_pressed: props.is_long_pressed,
        },
    }
}

pub fn use_long_press_value(props: UseLongPressProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_long_press(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_long_press hook did not serialize: {error}"
        ))
    })
}
