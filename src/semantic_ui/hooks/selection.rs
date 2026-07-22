use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
pub use crate::selection::SelectionMode;
use crate::selection::{
    CollectionKey, DisabledBehavior, EscapeKeyBehavior, Selection, SelectionBehavior,
};

use super::serde_helpers::is_false;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSelectionProps {
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
}

impl UseSelectionProps {
    pub fn new() -> Self {
        Self::default()
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
        self.selection_mode = SelectionMode::from_name(selection_mode.map(Into::into));
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSelectionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selected_keys: Selection,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub escape_key_behavior: &'static str,
    pub selection_props: SelectionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionProps {
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
}

pub fn use_selection(props: UseSelectionProps) -> UseSelectionResult {
    let selection_mode = props.selection_mode.as_str();
    let selection_behavior = props.selection_behavior.unwrap_or_else(|| {
        if props.selection_mode == SelectionMode::Multiple {
            SelectionBehavior::Toggle
        } else {
            SelectionBehavior::Replace
        }
    });
    let controlled_selected_keys = props.selected_keys.clone().or_else(|| {
        props
            .value
            .as_ref()
            .map(|value| Selection::keys([CollectionKey::new(value.clone())]))
    });
    let selected_keys = controlled_selected_keys
        .clone()
        .or_else(|| props.default_selected_keys.clone())
        .unwrap_or_else(Selection::empty);
    let selected_value = if props.selected_keys.is_some() {
        first_selected_value(&selected_keys)
    } else {
        props
            .value
            .clone()
            .or_else(|| first_selected_value(&selected_keys))
    };
    let controlled_value = props.selected_keys.is_none().then(|| props.value).flatten();

    UseSelectionResult {
        selected_value: selected_value.clone(),
        selected_keys: selected_keys.clone(),
        selection_mode,
        selection_behavior: selection_behavior.as_str(),
        disabled_behavior: props.disabled_behavior.as_str(),
        escape_key_behavior: props.escape_key_behavior.as_str(),
        selection_props: SelectionProps {
            value: controlled_value,
            selected_keys: controlled_selected_keys,
            default_selected_keys: props.default_selected_keys,
            disabled_keys: props.disabled_keys,
            selection_behavior: selection_behavior.as_str(),
            disabled_behavior: props.disabled_behavior.as_str(),
            disallow_empty_selection: props.disallow_empty_selection,
            should_focus_wrap: props.should_focus_wrap,
            escape_key_behavior: props.escape_key_behavior.as_str(),
            on_action: props.on_action,
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

fn first_selected_value(selection: &Selection) -> Option<String> {
    selection
        .explicit_keys()
        .and_then(|keys| keys.iter().next())
        .map(|key| key.as_str().to_string())
}

pub fn use_selection_value(props: UseSelectionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_selection(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_selection hook did not serialize: {error}"
        ))
    })
}
