use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseLinkProps {
    href: Option<String>,
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    on_press_up: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    is_disabled: bool,
    is_pressed: bool,
}

impl Default for UseLinkProps {
    fn default() -> Self {
        Self {
            href: None,
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

impl UseLinkProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn href(mut self, href: Option<impl Into<String>>) -> Self {
        self.href = non_empty(href);
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
pub struct UseLinkResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub link_props: LinkProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
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

pub fn use_link(props: UseLinkProps) -> UseLinkResult {
    UseLinkResult {
        href: props.href.clone(),
        is_disabled: props.is_disabled,
        is_pressed: props.is_pressed,
        link_props: LinkProps {
            role: "link",
            tab_index: if props.is_disabled { -1 } else { 0 },
            href: props.href,
            on_press: props.on_press,
            on_press_start: props.on_press_start,
            on_press_end: props.on_press_end,
            on_press_up: props.on_press_up,
            action_value: props.action_value,
            action_payload: props.action_payload,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_pressed: props.is_pressed,
        },
    }
}

pub fn use_link_value(props: UseLinkProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_link(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_link hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
