use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSeparatorProps {
    orientation: Option<String>,
}

impl UseSeparatorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = orientation
            .map(Into::into)
            .filter(|orientation| !orientation.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseToolbarProps {
    label: Option<String>,
    orientation: Option<String>,
    is_disabled: bool,
}

impl UseToolbarProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSeparatorResult {
    pub orientation: &'static str,
    pub separator_props: SeparatorProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseToolbarResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub orientation: &'static str,
    pub is_disabled: bool,
    pub toolbar_props: ToolbarProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeparatorProps {
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolbarProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_separator(props: UseSeparatorProps) -> UseSeparatorResult {
    let orientation = orientation_value(props.orientation);

    UseSeparatorResult {
        orientation,
        separator_props: SeparatorProps {
            orientation,
            data_orientation: orientation,
        },
    }
}

pub fn use_toolbar(props: UseToolbarProps) -> UseToolbarResult {
    let orientation = orientation_value(props.orientation);

    UseToolbarResult {
        label: props.label.clone(),
        orientation,
        is_disabled: props.is_disabled,
        toolbar_props: ToolbarProps {
            label: props.label,
            orientation,
            data_orientation: orientation,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_separator_value(props: UseSeparatorProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_separator(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_separator hook did not serialize: {error}"
        ))
    })
}

pub fn use_toolbar_value(props: UseToolbarProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_toolbar(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_toolbar hook did not serialize: {error}"
        ))
    })
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
