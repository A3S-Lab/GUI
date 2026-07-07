use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    None,
    #[default]
    Single,
    Multiple,
}

impl SelectionMode {
    pub(super) fn from_option(value: Option<impl Into<String>>) -> Self {
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

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSelectionProps {
    value: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    selection_mode: SelectionMode,
}

impl UseSelectionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into).filter(|value| !value.is_empty());
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_selection_change = action.map(Into::into).filter(|action| !action.is_empty());
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
        self.selection_mode = SelectionMode::from_option(selection_mode);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSelectionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selection_mode: &'static str,
    pub selection_props: SelectionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionProps {
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
    pub data_selection_mode: &'static str,
    #[serde(rename = "aria-multiselectable", skip_serializing_if = "is_false")]
    pub aria_multiselectable: bool,
}

pub fn use_selection(props: UseSelectionProps) -> UseSelectionResult {
    let selection_mode = props.selection_mode.as_str();
    let selected_value = props.value;

    UseSelectionResult {
        selected_value: selected_value.clone(),
        selection_mode,
        selection_props: SelectionProps {
            value: selected_value.clone(),
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            data_selected_value: selected_value,
            data_selection_mode: selection_mode,
            aria_multiselectable: props.selection_mode == SelectionMode::Multiple,
        },
    }
}

pub fn use_selection_value(props: UseSelectionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_selection(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_selection hook did not serialize: {error}"
        ))
    })
}
