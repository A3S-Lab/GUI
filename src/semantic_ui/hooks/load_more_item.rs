use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseLoadMoreItemProps {
    label: Option<String>,
    text_value: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    on_press: Option<String>,
    is_loading: bool,
    is_disabled: bool,
}

impl Default for UseLoadMoreItemProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            action_value: None,
            action_payload: JsonValue::Null,
            on_press: None,
            is_loading: false,
            is_disabled: false,
        }
    }
}

impl UseLoadMoreItemProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = text_value
            .map(Into::into)
            .filter(|text_value| !text_value.is_empty());
        self
    }

    pub fn action_value(mut self, action_value: Option<impl Into<String>>) -> Self {
        self.action_value = action_value
            .map(Into::into)
            .filter(|action_value| !action_value.is_empty());
        self
    }

    pub fn action_payload(mut self, action_payload: JsonValue) -> Self {
        self.action_payload = action_payload;
        self
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseLoadMoreItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(skip_serializing_if = "JsonValue::is_null")]
    pub action_payload: JsonValue,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub load_more_item_props: LoadMoreItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadMoreItemProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(skip_serializing_if = "JsonValue::is_null")]
    pub action_payload: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
    pub is_loading: bool,
    #[serde(rename = "aria-busy", skip_serializing_if = "is_false")]
    pub aria_busy: bool,
    #[serde(rename = "data-loading")]
    pub data_loading: bool,
}

pub fn use_load_more_item(props: UseLoadMoreItemProps) -> UseLoadMoreItemResult {
    let is_disabled = props.is_disabled || props.is_loading;
    let text_value = props.text_value.clone().or_else(|| props.label.clone());

    UseLoadMoreItemResult {
        label: props.label.clone(),
        text_value: text_value.clone(),
        action_value: props.action_value.clone(),
        action_payload: props.action_payload.clone(),
        is_loading: props.is_loading,
        is_disabled,
        load_more_item_props: LoadMoreItemProps {
            label: props.label,
            text_value,
            action_value: props.action_value,
            action_payload: props.action_payload,
            on_press: props.on_press,
            is_disabled,
            disabled: is_disabled,
            aria_disabled: is_disabled,
            data_disabled: is_disabled,
            is_loading: props.is_loading,
            aria_busy: props.is_loading,
            data_loading: props.is_loading,
        },
    }
}

pub fn use_load_more_item_value(props: UseLoadMoreItemProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_load_more_item(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_load_more_item hook did not serialize: {error}"
        ))
    })
}
