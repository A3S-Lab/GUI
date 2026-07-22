use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::selection::{CollectionKey, Selection};

use super::super::selection::{use_selection, UseSelectionProps};
use super::super::serde_helpers::is_false;
use super::non_empty;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseColorSwatchPickerProps {
    label: Option<String>,
    selection: UseSelectionProps,
}

impl UseColorSwatchPickerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.selection = self.selection.value(value);
        self
    }

    pub fn selected_keys(mut self, selected_keys: impl Into<Option<Selection>>) -> Self {
        self.selection = self.selection.selected_keys(selected_keys);
        self
    }

    pub fn default_selected_keys(mut self, selected_keys: impl Into<Option<Selection>>) -> Self {
        self.selection = self.selection.default_selected_keys(selected_keys);
        self
    }

    pub fn disabled_keys<I, K>(mut self, disabled_keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<CollectionKey>,
    {
        self.selection = self.selection.disabled_keys(disabled_keys);
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.selection = self.selection.on_selection_change(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.selection = self.selection.disabled(disabled);
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.selection = self.selection.read_only(read_only);
        self
    }

    pub fn selection_mode(mut self, selection_mode: Option<impl Into<String>>) -> Self {
        self.selection = self.selection.selection_mode(selection_mode);
        self
    }

    pub fn selection_behavior(mut self, selection_behavior: Option<impl AsRef<str>>) -> Self {
        self.selection = self.selection.selection_behavior(selection_behavior);
        self
    }

    pub fn disabled_behavior(mut self, disabled_behavior: Option<impl AsRef<str>>) -> Self {
        self.selection = self.selection.disabled_behavior(disabled_behavior);
        self
    }

    pub fn disallow_empty_selection(mut self, disallow: bool) -> Self {
        self.selection = self.selection.disallow_empty_selection(disallow);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSwatchPickerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selected_keys: Selection,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub color_swatch_picker_props: ColorSwatchPickerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSwatchPickerProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
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
pub struct UseColorSwatchPickerItemProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
}

impl UseColorSwatchPickerItemProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = non_empty(text_value);
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseColorSwatchPickerItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub color_swatch_picker_item_props: ColorSwatchPickerItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSwatchPickerItemProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "data-value", skip_serializing_if = "Option::is_none")]
    pub data_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub selected: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_color_swatch_picker(props: UseColorSwatchPickerProps) -> UseColorSwatchPickerResult {
    let selection = use_selection(props.selection);
    let selection_props = selection.selection_props;
    UseColorSwatchPickerResult {
        label: props.label.clone(),
        selected_value: selection.selected_value,
        selected_keys: selection.selected_keys,
        selection_mode: selection.selection_mode,
        selection_behavior: selection.selection_behavior,
        disabled_behavior: selection.disabled_behavior,
        is_disabled: selection_props.disabled,
        is_read_only: selection_props.read_only,
        color_swatch_picker_props: ColorSwatchPickerProps {
            label: props.label,
            value: selection_props.value,
            selected_keys: selection_props.selected_keys,
            default_selected_keys: selection_props.default_selected_keys,
            disabled_keys: selection_props.disabled_keys,
            selection_behavior: selection_props.selection_behavior,
            disabled_behavior: selection_props.disabled_behavior,
            disallow_empty_selection: selection_props.disallow_empty_selection,
            on_selection_change: selection_props.on_selection_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
            data_selected_value: selection_props.data_selected_value,
            data_selection_mode: selection_props.data_selection_mode,
            aria_multiselectable: selection_props.aria_multiselectable,
        },
    }
}

pub fn use_color_swatch_picker_item(
    props: UseColorSwatchPickerItemProps,
) -> UseColorSwatchPickerItemResult {
    UseColorSwatchPickerItemResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        color_swatch_picker_item_props: ColorSwatchPickerItemProps {
            value: props.value.clone(),
            text_value: props.text_value,
            data_value: props.value,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_color_swatch_picker_value(props: UseColorSwatchPickerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_color_swatch_picker", use_color_swatch_picker(props))
}

pub fn use_color_swatch_picker_item_value(
    props: UseColorSwatchPickerItemProps,
) -> GuiResult<JsonValue> {
    serialize_hook(
        "use_color_swatch_picker_item",
        use_color_swatch_picker_item(props),
    )
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}
