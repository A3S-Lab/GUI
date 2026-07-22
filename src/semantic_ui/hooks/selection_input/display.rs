use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::non_empty;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSelectDisplayProps {
    value: Option<String>,
    placeholder: Option<String>,
}

impl UseSelectDisplayProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSelectDisplayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
    pub is_placeholder: bool,
    pub select_value_props: SelectValueProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseComboBoxDisplayProps {
    value: Option<String>,
    placeholder: Option<String>,
}

impl UseComboBoxDisplayProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseComboBoxDisplayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
    pub is_placeholder: bool,
    pub combo_box_value_props: SelectValueProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectValueProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(rename = "data-placeholder")]
    pub data_placeholder: bool,
}

pub fn use_select_display(props: UseSelectDisplayProps) -> UseSelectDisplayResult {
    let (display_value, is_placeholder) =
        selection_value_display(props.value.clone(), props.placeholder.clone());
    UseSelectDisplayResult {
        value: props.value.clone(),
        display_value: display_value.clone(),
        is_placeholder,
        select_value_props: SelectValueProps {
            value: props.value.clone(),
            placeholder: props.placeholder,
            label: display_value,
            data_value: props.value,
            data_placeholder: is_placeholder,
        },
    }
}

pub fn use_combo_box_display(props: UseComboBoxDisplayProps) -> UseComboBoxDisplayResult {
    let (display_value, is_placeholder) =
        selection_value_display(props.value.clone(), props.placeholder.clone());
    UseComboBoxDisplayResult {
        value: props.value.clone(),
        display_value: display_value.clone(),
        is_placeholder,
        combo_box_value_props: SelectValueProps {
            value: props.value.clone(),
            placeholder: props.placeholder,
            label: display_value,
            data_value: props.value,
            data_placeholder: is_placeholder,
        },
    }
}

pub fn use_select_display_value(props: UseSelectDisplayProps) -> GuiResult<JsonValue> {
    serialize_hook("use_select_display", use_select_display(props))
}

pub fn use_combo_box_display_value(props: UseComboBoxDisplayProps) -> GuiResult<JsonValue> {
    serialize_hook("use_combo_box_display", use_combo_box_display(props))
}

fn selection_value_display(
    value: Option<String>,
    placeholder: Option<String>,
) -> (Option<String>, bool) {
    match value {
        Some(value) if !value.is_empty() => (Some(value), false),
        _ => (placeholder, true),
    }
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}
