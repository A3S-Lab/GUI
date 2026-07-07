use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::selection::SelectionMode;
use super::serde_helpers::{is_false, is_none_or_false};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTreeProps {
    label: Option<String>,
    value: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    selection_mode: SelectionMode,
}

impl UseTreeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
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
pub struct UseTreeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selection_mode: &'static str,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub tree_props: TreeProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTreeItemProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
    is_expanded: Option<bool>,
}

impl UseTreeItemProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into).filter(|value| !value.is_empty());
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = text_value
            .map(Into::into)
            .filter(|text_value| !text_value.is_empty());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn expanded(mut self, expanded: Option<bool>) -> Self {
        self.is_expanded = expanded;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTreeItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expanded: Option<bool>,
    pub tree_item_props: TreeItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeItemProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub is_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub selected: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expanded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    #[serde(rename = "aria-expanded", skip_serializing_if = "is_none_or_false")]
    pub aria_expanded: Option<bool>,
    #[serde(rename = "data-expanded", skip_serializing_if = "Option::is_none")]
    pub data_expanded: Option<bool>,
}

pub fn use_tree(props: UseTreeProps) -> UseTreeResult {
    let selection_mode = props.selection_mode.as_str();
    let selected_value = props.value;

    UseTreeResult {
        label: props.label.clone(),
        selected_value: selected_value.clone(),
        selection_mode,
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        tree_props: TreeProps {
            role: "tree",
            label: props.label.clone(),
            aria_label: props.label,
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

pub fn use_tree_item(props: UseTreeItemProps) -> UseTreeItemResult {
    UseTreeItemResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        is_expanded: props.is_expanded,
        tree_item_props: TreeItemProps {
            role: "treeitem",
            tab_index: if props.is_disabled { -1 } else { 0 },
            value: props.value,
            text_value: props.text_value,
            is_selected: props.is_selected,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
            is_disabled: props.is_disabled,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_disabled: props.is_disabled,
            is_expanded: props.is_expanded,
            expanded: props.is_expanded,
            aria_expanded: props.is_expanded,
            data_expanded: props.is_expanded,
        },
    }
}

pub fn use_tree_value(props: UseTreeProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_tree(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_tree hook did not serialize: {error}"))
    })
}

pub fn use_tree_item_value(props: UseTreeItemProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_tree_item(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_tree_item hook did not serialize: {error}"
        ))
    })
}
