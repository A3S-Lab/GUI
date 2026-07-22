use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseRadioGroupProps {
    label: Option<String>,
    value: Option<String>,
    default_value: Option<String>,
    on_selection_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
    selection_mode: Option<String>,
}

impl UseRadioGroupProps {
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

    pub fn default_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.default_value = value.map(Into::into).filter(|value| !value.is_empty());
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
        self.selection_mode = selection_mode
            .map(Into::into)
            .filter(|selection_mode| !selection_mode.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseRadioGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub selection_mode: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub radio_group_props: RadioGroupProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioGroupProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
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
    #[serde(rename = "data-selection-mode")]
    pub data_selection_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseRadioProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
}

impl UseRadioProps {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseRadioResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub radio_props: RadioProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioProps {
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
    #[serde(skip_serializing_if = "is_false")]
    pub is_checked: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub checked: bool,
    #[serde(rename = "aria-checked")]
    pub aria_checked: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(rename = "data-checked")]
    pub data_checked: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

pub fn use_radio_group(props: UseRadioGroupProps) -> UseRadioGroupResult {
    let selection_mode = props.selection_mode.unwrap_or_else(|| "single".to_string());
    let selected_value = props.value.clone().or_else(|| props.default_value.clone());

    UseRadioGroupResult {
        label: props.label.clone(),
        selected_value: selected_value.clone(),
        selection_mode: selection_mode.clone(),
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        radio_group_props: RadioGroupProps {
            role: "radiogroup",
            label: props.label.clone(),
            aria_label: props.label,
            value: props.value.clone(),
            default_value: props.default_value,
            on_selection_change: props.on_selection_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            data_selected_value: selected_value,
            data_selection_mode: selection_mode,
        },
    }
}

pub fn use_radio(props: UseRadioProps) -> UseRadioResult {
    UseRadioResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_checked: props.is_selected,
        is_disabled: props.is_disabled,
        radio_props: RadioProps {
            role: "radio",
            tab_index: -1,
            value: props.value,
            text_value: props.text_value,
            is_selected: props.is_selected,
            selected: props.is_selected,
            is_checked: props.is_selected,
            checked: props.is_selected,
            aria_checked: props.is_selected,
            data_selected: props.is_selected,
            data_checked: props.is_selected,
            is_disabled: props.is_disabled,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_radio_group_value(props: UseRadioGroupProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_radio_group(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_radio_group hook did not serialize: {error}"
        ))
    })
}

pub fn use_radio_value(props: UseRadioProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_radio(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_radio hook did not serialize: {error}"
        ))
    })
}
