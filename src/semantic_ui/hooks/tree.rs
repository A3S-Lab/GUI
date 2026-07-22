use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::selection::{
    CollectionKey, DisabledBehavior, EscapeKeyBehavior, Selection, SelectionBehavior,
};

use super::selection::{use_selection, SelectionMode, UseSelectionProps};
use super::serde_helpers::{is_false, is_none_or_false};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTreeProps {
    label: Option<String>,
    value: Option<String>,
    selected_keys: Option<Selection>,
    default_selected_keys: Option<Selection>,
    disabled_keys: BTreeSet<CollectionKey>,
    on_action: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    selection_mode: SelectionMode,
    selection_behavior: Option<SelectionBehavior>,
    disabled_behavior: DisabledBehavior,
    disallow_empty_selection: bool,
    should_focus_wrap: bool,
    escape_key_behavior: EscapeKeyBehavior,
    expanded_keys: Option<BTreeSet<CollectionKey>>,
    default_expanded_keys: Option<BTreeSet<CollectionKey>>,
    on_expanded_change: Option<String>,
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

    pub fn selected_keys(mut self, selected_keys: impl Into<Option<Selection>>) -> Self {
        self.selected_keys = selected_keys.into();
        self
    }

    pub fn default_selected_keys(mut self, selected_keys: impl Into<Option<Selection>>) -> Self {
        self.default_selected_keys = selected_keys.into();
        self
    }

    pub fn disabled_keys<I, K>(mut self, disabled_keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<CollectionKey>,
    {
        self.disabled_keys = disabled_keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_selection_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_action(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_action = action.map(Into::into).filter(|action| !action.is_empty());
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

    pub fn selection_behavior(mut self, selection_behavior: Option<impl AsRef<str>>) -> Self {
        self.selection_behavior = SelectionBehavior::from_name(selection_behavior);
        self
    }

    pub fn disabled_behavior(mut self, disabled_behavior: Option<impl AsRef<str>>) -> Self {
        self.disabled_behavior = DisabledBehavior::from_name(disabled_behavior).unwrap_or_default();
        self
    }

    pub fn disallow_empty_selection(mut self, disallow: bool) -> Self {
        self.disallow_empty_selection = disallow;
        self
    }

    pub fn should_focus_wrap(mut self, should_wrap: bool) -> Self {
        self.should_focus_wrap = should_wrap;
        self
    }

    pub fn escape_key_behavior(mut self, behavior: Option<impl AsRef<str>>) -> Self {
        self.escape_key_behavior = EscapeKeyBehavior::from_name(behavior);
        self
    }

    pub fn expanded_keys(
        mut self,
        expanded_keys: impl Into<Option<BTreeSet<CollectionKey>>>,
    ) -> Self {
        self.expanded_keys = expanded_keys.into();
        self
    }

    pub fn default_expanded_keys(
        mut self,
        expanded_keys: impl Into<Option<BTreeSet<CollectionKey>>>,
    ) -> Self {
        self.default_expanded_keys = expanded_keys.into();
        self
    }

    pub fn on_expanded_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_expanded_change = action.map(Into::into).filter(|action| !action.is_empty());
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
    pub selected_keys: Selection,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub escape_key_behavior: &'static str,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub expanded_keys: BTreeSet<CollectionKey>,
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
    #[serde(rename = "selectedKeys", skip_serializing_if = "Option::is_none")]
    pub selected_keys: Option<Selection>,
    #[serde(
        rename = "defaultSelectedKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_selected_keys: Option<Selection>,
    #[serde(rename = "disabledKeys", skip_serializing_if = "BTreeSet::is_empty")]
    pub disabled_keys: BTreeSet<CollectionKey>,
    #[serde(rename = "selectionBehavior")]
    pub selection_behavior: &'static str,
    #[serde(rename = "disabledBehavior")]
    pub disabled_behavior: &'static str,
    #[serde(rename = "disallowEmptySelection", skip_serializing_if = "is_false")]
    pub disallow_empty_selection: bool,
    #[serde(rename = "shouldFocusWrap", skip_serializing_if = "is_false")]
    pub should_focus_wrap: bool,
    #[serde(rename = "escapeKeyBehavior")]
    pub escape_key_behavior: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_action: Option<String>,
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
    #[serde(rename = "expandedKeys", skip_serializing_if = "Option::is_none")]
    pub expanded_keys: Option<BTreeSet<CollectionKey>>,
    #[serde(
        rename = "defaultExpandedKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_expanded_keys: Option<BTreeSet<CollectionKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_expanded_change: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTreeItemProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
    is_expanded: Option<bool>,
    has_child_items: bool,
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

    pub fn has_child_items(mut self, has_child_items: bool) -> Self {
        self.has_child_items = has_child_items;
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
    pub has_child_items: bool,
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
    #[serde(rename = "hasChildItems", skip_serializing_if = "is_false")]
    pub has_child_items: bool,
    #[serde(rename = "data-has-child-items", skip_serializing_if = "is_false")]
    pub data_has_child_items: bool,
}

pub fn use_tree(props: UseTreeProps) -> UseTreeResult {
    let expanded_keys = props
        .expanded_keys
        .clone()
        .or_else(|| props.default_expanded_keys.clone())
        .unwrap_or_default();
    let selection = use_selection(
        UseSelectionProps::new()
            .value(props.value)
            .selected_keys(props.selected_keys)
            .default_selected_keys(props.default_selected_keys)
            .disabled_keys(props.disabled_keys)
            .on_action(props.on_action)
            .on_selection_change(props.on_selection_change)
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.as_str()))
            .selection_behavior(props.selection_behavior.map(SelectionBehavior::as_str))
            .disabled_behavior(Some(props.disabled_behavior.as_str()))
            .disallow_empty_selection(props.disallow_empty_selection)
            .should_focus_wrap(props.should_focus_wrap)
            .escape_key_behavior(Some(props.escape_key_behavior.as_str())),
    );
    let selection_props = selection.selection_props;

    UseTreeResult {
        label: props.label.clone(),
        selected_value: selection.selected_value.clone(),
        selected_keys: selection.selected_keys,
        selection_mode: selection.selection_mode,
        selection_behavior: selection.selection_behavior,
        disabled_behavior: selection.disabled_behavior,
        escape_key_behavior: selection.escape_key_behavior,
        is_disabled: props.is_disabled,
        is_read_only: props.is_read_only,
        expanded_keys,
        tree_props: TreeProps {
            role: "tree",
            label: props.label.clone(),
            aria_label: props.label,
            value: selection_props.value,
            selected_keys: selection_props.selected_keys,
            default_selected_keys: selection_props.default_selected_keys,
            disabled_keys: selection_props.disabled_keys,
            selection_behavior: selection_props.selection_behavior,
            disabled_behavior: selection_props.disabled_behavior,
            disallow_empty_selection: selection_props.disallow_empty_selection,
            should_focus_wrap: selection_props.should_focus_wrap,
            escape_key_behavior: selection_props.escape_key_behavior,
            on_action: selection_props.on_action,
            on_selection_change: selection_props.on_selection_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
            data_selected_value: selection_props.data_selected_value,
            data_selection_mode: selection_props.data_selection_mode,
            aria_multiselectable: selection_props.aria_multiselectable,
            expanded_keys: props.expanded_keys,
            default_expanded_keys: props.default_expanded_keys,
            on_expanded_change: props.on_expanded_change,
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
        has_child_items: props.has_child_items,
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
            has_child_items: props.has_child_items,
            data_has_child_items: props.has_child_items,
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
