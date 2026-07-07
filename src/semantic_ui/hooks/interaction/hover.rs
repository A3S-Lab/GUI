use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseHoverProps {
    on_hover_start: Option<String>,
    on_hover_end: Option<String>,
    on_hover_change: Option<String>,
    is_disabled: bool,
    is_hovered: bool,
}

impl UseHoverProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_hover_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_start = non_empty(action);
        self
    }

    pub fn on_hover_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_end = non_empty(action);
        self
    }

    pub fn on_hover_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.is_hovered = hovered;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseHoverResult {
    pub is_hovered: bool,
    pub hover_props: HoverProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-hovered")]
    pub data_hovered: bool,
}

pub fn use_hover(props: UseHoverProps) -> UseHoverResult {
    UseHoverResult {
        is_hovered: props.is_hovered,
        hover_props: HoverProps {
            on_hover_start: props.on_hover_start,
            on_hover_end: props.on_hover_end,
            on_hover_change: props.on_hover_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_hovered: props.is_hovered,
        },
    }
}

pub fn use_hover_value(props: UseHoverProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_hover(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_hover hook did not serialize: {error}"
        ))
    })
}
