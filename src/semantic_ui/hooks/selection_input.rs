use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionInputMode {
    None,
    #[default]
    Single,
    Multiple,
}

impl SelectionInputMode {
    fn from_option(value: Option<impl Into<String>>) -> Self {
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

    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseComboBoxProps {
    label: Option<String>,
    value: Option<String>,
    input_value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    on_selection_change: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    selection_mode: SelectionInputMode,
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
        self.value = non_empty(value);
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
        self.on_selection_change = non_empty(action);
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
        self.is_disabled = disabled;
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
        self.is_read_only = read_only;
        self
    }

    pub fn selection_mode(mut self, selection_mode: Option<impl Into<String>>) -> Self {
        self.selection_mode = SelectionInputMode::from_option(selection_mode);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
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
    value: Option<String>,
    input_value: Option<String>,
    placeholder: Option<String>,
    on_change: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    selection_mode: SelectionInputMode,
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
        self.value = non_empty(value);
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
        self.on_selection_change = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
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
        self.is_read_only = read_only;
        self
    }

    pub fn selection_mode(mut self, selection_mode: Option<impl Into<String>>) -> Self {
        self.selection_mode = SelectionInputMode::from_option(selection_mode);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
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
    value: Option<String>,
    placeholder: Option<String>,
    on_selection_change: Option<String>,
    on_open_change: Option<String>,
    is_open: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    selection_mode: SelectionInputMode,
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
        self.value = non_empty(value);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = non_empty(placeholder);
        self
    }

    pub fn on_selection_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_selection_change = non_empty(action);
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
        self.is_disabled = disabled;
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
        self.is_read_only = read_only;
        self
    }

    pub fn selection_mode(mut self, selection_mode: Option<impl Into<String>>) -> Self {
        self.selection_mode = SelectionInputMode::from_option(selection_mode);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub selection_mode: &'static str,
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

pub fn use_combo_box(props: UseComboBoxProps) -> UseComboBoxResult {
    let selection_mode = props.selection_mode.as_str();
    let is_multiple = props.selection_mode == SelectionInputMode::Multiple;
    let trigger = selection_input_trigger_props(
        props.is_open,
        props.on_open_change.clone(),
        props.is_disabled,
    );

    UseComboBoxResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        input_value: props.input_value.clone(),
        placeholder: props.placeholder.clone(),
        selection_mode,
        is_open: props.is_open,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        combo_box_props: ComboBoxProps {
            label: props.label,
            value: props.value.clone(),
            placeholder: props.placeholder.clone(),
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_selected_value: props.value,
            data_input_value: props.input_value.clone(),
            data_selection_mode: selection_mode,
            aria_multiselectable: is_multiple,
        },
        combo_box_input_props: ComboBoxInputProps {
            value: props.input_value,
            placeholder: props.placeholder,
            on_change: props.on_change.clone(),
            on_input: props.on_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
        },
        combo_box_trigger_props: trigger,
    }
}

pub fn use_autocomplete(props: UseAutocompleteProps) -> UseAutocompleteResult {
    let selection_mode = props.selection_mode.as_str();
    let is_multiple = props.selection_mode == SelectionInputMode::Multiple;

    UseAutocompleteResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        input_value: props.input_value.clone(),
        placeholder: props.placeholder.clone(),
        selection_mode,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        autocomplete_props: AutocompleteProps {
            label: props.label,
            value: props.value.clone(),
            placeholder: props.placeholder.clone(),
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            data_selected_value: props.value,
            data_input_value: props.input_value.clone(),
            data_selection_mode: selection_mode,
            aria_multiselectable: is_multiple,
        },
        autocomplete_input_props: ComboBoxInputProps {
            value: props.input_value,
            placeholder: props.placeholder,
            on_change: props.on_change.clone(),
            on_input: props.on_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
        },
    }
}

pub fn use_select(props: UseSelectProps) -> UseSelectResult {
    let selection_mode = props.selection_mode.as_str();
    let is_multiple = props.selection_mode == SelectionInputMode::Multiple;
    let trigger = selection_input_trigger_props(
        props.is_open,
        props.on_open_change.clone(),
        props.is_disabled,
    );

    UseSelectResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        placeholder: props.placeholder.clone(),
        selection_mode,
        is_open: props.is_open,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        select_props: SelectProps {
            label: props.label,
            value: props.value.clone(),
            placeholder: props.placeholder,
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_selected_value: props.value,
            data_selection_mode: selection_mode,
            aria_multiselectable: is_multiple,
        },
        select_trigger_props: trigger,
    }
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

pub fn use_combo_box_value(props: UseComboBoxProps) -> GuiResult<JsonValue> {
    serialize_hook("use_combo_box", use_combo_box(props))
}

pub fn use_autocomplete_value(props: UseAutocompleteProps) -> GuiResult<JsonValue> {
    serialize_hook("use_autocomplete", use_autocomplete(props))
}

pub fn use_select_value(props: UseSelectProps) -> GuiResult<JsonValue> {
    serialize_hook("use_select", use_select(props))
}

pub fn use_select_display_value(props: UseSelectDisplayProps) -> GuiResult<JsonValue> {
    serialize_hook("use_select_display", use_select_display(props))
}

pub fn use_combo_box_display_value(props: UseComboBoxDisplayProps) -> GuiResult<JsonValue> {
    serialize_hook("use_combo_box_display", use_combo_box_display(props))
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

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
