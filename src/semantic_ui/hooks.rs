use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsePressProps {
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    is_disabled: bool,
    is_pressed: bool,
}

impl UsePressProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_press_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_start = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_press_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_end = action.map(Into::into).filter(|action| !action.is_empty());
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
pub struct UsePressResult {
    pub is_pressed: bool,
    pub press_props: PressProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PressProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
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
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
}

pub fn use_press(props: UsePressProps) -> UsePressResult {
    UsePressResult {
        is_pressed: props.is_pressed,
        press_props: PressProps {
            role: "button",
            tab_index: 0,
            on_press: props.on_press,
            on_press_start: props.on_press_start,
            on_press_end: props.on_press_end,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_pressed: props.is_pressed,
        },
    }
}

pub fn use_press_value(props: UsePressProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_press(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_press hook did not serialize: {error}"
        ))
    })
}

fn is_false(value: &bool) -> bool {
    !*value
}
