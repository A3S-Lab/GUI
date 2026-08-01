use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDropIndicatorProps {
    orientation: Option<String>,
    target_key: Option<String>,
    drop_position: Option<String>,
    is_target: bool,
}

impl UseDropIndicatorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = orientation
            .map(Into::into)
            .filter(|orientation| !orientation.is_empty());
        self
    }

    pub fn target(mut self, target: bool) -> Self {
        self.is_target = target;
        self
    }

    pub fn target_key(mut self, key: Option<impl Into<String>>) -> Self {
        self.target_key = key.map(Into::into).filter(|key| !key.is_empty());
        self
    }

    pub fn drop_position(mut self, position: Option<impl Into<String>>) -> Self {
        self.drop_position = position
            .map(Into::into)
            .filter(|position| !position.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSelectionIndicatorProps {
    label: Option<String>,
    is_selected: bool,
}

impl UseSelectionIndicatorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDropIndicatorResult {
    pub orientation: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_position: Option<&'static str>,
    pub is_target: bool,
    pub drop_indicator_props: DropIndicatorProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSelectionIndicatorResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub is_selected: bool,
    pub selection_indicator_props: SelectionIndicatorProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropIndicatorProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    pub orientation: &'static str,
    #[serde(rename = "data-orientation")]
    pub data_orientation: &'static str,
    pub is_target: bool,
    #[serde(rename = "data-target")]
    pub data_target: bool,
    #[serde(rename = "data-collection-drop-indicator")]
    pub data_collection_drop_indicator: bool,
    #[serde(
        rename = "data-drop-target-key",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_drop_target_key: Option<String>,
    #[serde(rename = "data-drop-position", skip_serializing_if = "Option::is_none")]
    pub data_drop_position: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionIndicatorProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub is_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub selected: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
}

pub fn use_drop_indicator(props: UseDropIndicatorProps) -> UseDropIndicatorResult {
    let orientation = orientation_value(props.orientation);
    let drop_position = drop_position_value(props.drop_position);

    UseDropIndicatorResult {
        orientation,
        target_key: props.target_key.clone(),
        drop_position,
        is_target: props.is_target,
        drop_indicator_props: DropIndicatorProps {
            tab_index: -1,
            orientation,
            data_orientation: orientation,
            is_target: props.is_target,
            data_target: props.is_target,
            data_collection_drop_indicator: true,
            data_drop_target_key: props.target_key,
            data_drop_position: drop_position,
        },
    }
}

pub fn use_selection_indicator(props: UseSelectionIndicatorProps) -> UseSelectionIndicatorResult {
    UseSelectionIndicatorResult {
        label: props.label.clone(),
        is_selected: props.is_selected,
        selection_indicator_props: SelectionIndicatorProps {
            label: props.label,
            is_selected: props.is_selected,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
        },
    }
}

pub fn use_drop_indicator_value(props: UseDropIndicatorProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_drop_indicator(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_drop_indicator hook did not serialize: {error}"
        ))
    })
}

pub fn use_selection_indicator_value(props: UseSelectionIndicatorProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_selection_indicator(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_selection_indicator hook did not serialize: {error}"
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

fn drop_position_value(position: Option<String>) -> Option<&'static str> {
    match position
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("before") => Some("before"),
        Some("after") => Some("after"),
        _ => None,
    }
}
