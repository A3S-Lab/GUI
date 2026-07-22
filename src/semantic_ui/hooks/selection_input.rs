use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::selection::{CollectionKey, Selection};

use super::selection::{use_selection, UseSelectionProps};
use super::serde_helpers::is_false;

mod display;
pub use display::{
    use_combo_box_display, use_combo_box_display_value, use_select_display,
    use_select_display_value, SelectValueProps, UseComboBoxDisplayProps, UseComboBoxDisplayResult,
    UseSelectDisplayProps, UseSelectDisplayResult,
};

pub type SelectionInputMode = crate::selection::SelectionMode;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseComboBoxProps {
    label: Option<String>,
    input_value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_required: bool,
    is_invalid: bool,
    selection: UseSelectionProps,
}

impl UseComboBoxProps {
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

    pub fn input_value(mut self, input_value: Option<impl Into<String>>) -> Self {
        self.input_value = input_value.map(Into::into);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.selection = self.selection.on_selection_change(action);
        self
    }

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = non_empty(action);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.selection = self.selection.disabled(disabled);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
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
pub struct UseComboBoxResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selected_keys: Selection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub combo_box_props: ComboBoxProps,
    pub combo_box_input_props: ComboBoxInputProps,
    pub combo_box_trigger_props: SelectionInputTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBoxProps {
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
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_selection_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(
        rename = "data-selected-value",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_selected_value: Option<String>,
    #[serde(rename = "data-input-value", skip_serializing_if = "Option::is_none")]
    pub data_input_value: Option<String>,
    #[serde(rename = "data-selection-mode")]
    pub data_selection_mode: &'static str,
    #[serde(rename = "aria-multiselectable", skip_serializing_if = "is_false")]
    pub aria_multiselectable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBoxInputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_input: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionInputTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "aria-haspopup")]
    pub aria_haspopup: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseAutocompleteProps {
    label: Option<String>,
    input_value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    is_required: bool,
    is_invalid: bool,
    selection: UseSelectionProps,
}

impl UseAutocompleteProps {
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

    pub fn input_value(mut self, input_value: Option<impl Into<String>>) -> Self {
        self.input_value = input_value.map(Into::into);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
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

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
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
pub struct UseAutocompleteResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selected_keys: Selection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub autocomplete_props: AutocompleteProps,
    pub autocomplete_input_props: ComboBoxInputProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteProps {
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
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_selection_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
    #[serde(
        rename = "data-selected-value",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_selected_value: Option<String>,
    #[serde(rename = "data-input-value", skip_serializing_if = "Option::is_none")]
    pub data_input_value: Option<String>,
    #[serde(rename = "data-selection-mode")]
    pub data_selection_mode: &'static str,
    #[serde(rename = "aria-multiselectable", skip_serializing_if = "is_false")]
    pub aria_multiselectable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseSelectProps {
    label: Option<String>,
    placeholder: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_required: bool,
    is_invalid: bool,
    selection: UseSelectionProps,
}

impl UseSelectProps {
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

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.selection = self.selection.on_selection_change(action);
        self
    }

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = non_empty(action);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.selection = self.selection.disabled(disabled);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
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
pub struct UseSelectResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selected_keys: Selection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
    pub selection_behavior: &'static str,
    pub disabled_behavior: &'static str,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub select_props: SelectProps,
    pub select_trigger_props: SelectionInputTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectProps {
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
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_selection_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(rename = "aria-required", skip_serializing_if = "is_false")]
    pub aria_required: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub invalid: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
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

pub fn use_combo_box(props: UseComboBoxProps) -> UseComboBoxResult {
    let selection = use_selection(props.selection);
    let selection_mode = selection.selection_mode;
    let selection_behavior = selection.selection_behavior;
    let disabled_behavior = selection.disabled_behavior;
    let selected_value = selection.selected_value;
    let selected_keys = selection.selected_keys;
    let selection_props = selection.selection_props;
    let trigger = selection_input_trigger_props(
        props.is_open,
        props.on_open_change.clone(),
        selection_props.disabled,
    );

    UseComboBoxResult {
        label: props.label.clone(),
        selected_value,
        selected_keys,
        input_value: props.input_value.clone(),
        placeholder: props.placeholder.clone(),
        selection_mode,
        selection_behavior,
        disabled_behavior,
        is_open: props.is_open,
        is_disabled: selection_props.disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: selection_props.read_only,
        combo_box_props: ComboBoxProps {
            label: props.label,
            value: selection_props.value,
            selected_keys: selection_props.selected_keys,
            default_selected_keys: selection_props.default_selected_keys,
            disabled_keys: selection_props.disabled_keys,
            selection_behavior: selection_props.selection_behavior,
            disabled_behavior: selection_props.disabled_behavior,
            disallow_empty_selection: selection_props.disallow_empty_selection,
            placeholder: props.placeholder.clone(),
            on_selection_change: selection_props.on_selection_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_selected_value: selection_props.data_selected_value,
            data_input_value: props.input_value.clone(),
            data_selection_mode: selection_props.data_selection_mode,
            aria_multiselectable: selection_props.aria_multiselectable,
        },
        combo_box_input_props: ComboBoxInputProps {
            value: props.input_value,
            placeholder: props.placeholder,
            on_change: props.on_change.clone(),
            on_input: props.on_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
        },
        combo_box_trigger_props: trigger,
    }
}

pub fn use_autocomplete(props: UseAutocompleteProps) -> UseAutocompleteResult {
    let selection = use_selection(props.selection);
    let selection_mode = selection.selection_mode;
    let selection_behavior = selection.selection_behavior;
    let disabled_behavior = selection.disabled_behavior;
    let selected_value = selection.selected_value;
    let selected_keys = selection.selected_keys;
    let selection_props = selection.selection_props;

    UseAutocompleteResult {
        label: props.label.clone(),
        selected_value,
        selected_keys,
        input_value: props.input_value.clone(),
        placeholder: props.placeholder.clone(),
        selection_mode,
        selection_behavior,
        disabled_behavior,
        is_disabled: selection_props.disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: selection_props.read_only,
        autocomplete_props: AutocompleteProps {
            label: props.label,
            value: selection_props.value,
            selected_keys: selection_props.selected_keys,
            default_selected_keys: selection_props.default_selected_keys,
            disabled_keys: selection_props.disabled_keys,
            selection_behavior: selection_props.selection_behavior,
            disabled_behavior: selection_props.disabled_behavior,
            disallow_empty_selection: selection_props.disallow_empty_selection,
            placeholder: props.placeholder.clone(),
            on_selection_change: selection_props.on_selection_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
            data_selected_value: selection_props.data_selected_value,
            data_input_value: props.input_value.clone(),
            data_selection_mode: selection_props.data_selection_mode,
            aria_multiselectable: selection_props.aria_multiselectable,
        },
        autocomplete_input_props: ComboBoxInputProps {
            value: props.input_value,
            placeholder: props.placeholder,
            on_change: props.on_change.clone(),
            on_input: props.on_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
        },
    }
}

pub fn use_select(props: UseSelectProps) -> UseSelectResult {
    let selection = use_selection(props.selection);
    let selection_mode = selection.selection_mode;
    let selection_behavior = selection.selection_behavior;
    let disabled_behavior = selection.disabled_behavior;
    let selected_value = selection.selected_value;
    let selected_keys = selection.selected_keys;
    let selection_props = selection.selection_props;
    let trigger = selection_input_trigger_props(
        props.is_open,
        props.on_open_change.clone(),
        selection_props.disabled,
    );

    UseSelectResult {
        label: props.label.clone(),
        selected_value,
        selected_keys,
        placeholder: props.placeholder.clone(),
        selection_mode,
        selection_behavior,
        disabled_behavior,
        is_open: props.is_open,
        is_disabled: selection_props.disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: selection_props.read_only,
        select_props: SelectProps {
            label: props.label,
            value: selection_props.value,
            selected_keys: selection_props.selected_keys,
            default_selected_keys: selection_props.default_selected_keys,
            disabled_keys: selection_props.disabled_keys,
            selection_behavior: selection_props.selection_behavior,
            disabled_behavior: selection_props.disabled_behavior,
            disallow_empty_selection: selection_props.disallow_empty_selection,
            placeholder: props.placeholder,
            on_selection_change: selection_props.on_selection_change,
            disabled: selection_props.disabled,
            aria_disabled: selection_props.aria_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: selection_props.read_only,
            aria_read_only: selection_props.aria_read_only,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_selected_value: selection_props.data_selected_value,
            data_selection_mode: selection_props.data_selection_mode,
            aria_multiselectable: selection_props.aria_multiselectable,
        },
        select_trigger_props: trigger,
    }
}

pub fn use_combo_box_value(props: UseComboBoxProps) -> GuiResult<JsonValue> {
    serialize_hook("use_combo_box", use_combo_box(props))
}

pub fn use_autocomplete_value(props: UseAutocompleteProps) -> GuiResult<JsonValue> {
    serialize_hook("use_autocomplete", use_autocomplete(props))
}

pub fn use_select_value(props: UseSelectProps) -> GuiResult<JsonValue> {
    serialize_hook("use_select", use_select(props))
}

fn selection_input_trigger_props(
    is_open: bool,
    on_open_change: Option<String>,
    is_disabled: bool,
) -> SelectionInputTriggerProps {
    SelectionInputTriggerProps {
        role: "button",
        tab_index: if is_disabled { -1 } else { 0 },
        aria_expanded: is_open,
        data_open: is_open,
        aria_haspopup: "listbox",
        on_press: on_open_change,
        disabled: is_disabled,
        aria_disabled: is_disabled,
    }
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
