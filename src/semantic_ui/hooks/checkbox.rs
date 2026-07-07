use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCheckboxProps {
    value: Option<String>,
    on_change: Option<String>,
    is_checked: bool,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseCheckboxProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = non_empty(value);
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.is_checked = checked;
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCheckboxGroupProps {
    label: Option<String>,
    value: Option<String>,
    on_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseCheckboxGroupProps {
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

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = non_empty(action);
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCheckboxResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub is_checked: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub checkbox_props: CheckboxProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    pub checked: bool,
    #[serde(rename = "aria-checked")]
    pub aria_checked: bool,
    #[serde(rename = "data-checked")]
    pub data_checked: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
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
pub struct UseCheckboxGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_value: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub checkbox_group_props: CheckboxGroupProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxGroupProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<String>,
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
}

pub fn use_checkbox(props: UseCheckboxProps) -> UseCheckboxResult {
    UseCheckboxResult {
        value: props.value.clone(),
        is_checked: props.is_checked,
        is_selected: props.is_checked,
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        checkbox_props: CheckboxProps {
            role: "checkbox",
            tab_index: if props.is_disabled { -1 } else { 0 },
            checked: props.is_checked,
            aria_checked: props.is_checked,
            data_checked: props.is_checked,
            data_selected: props.is_checked,
            value: props.value,
            on_change: props.on_change,
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

pub fn use_checkbox_group(props: UseCheckboxGroupProps) -> UseCheckboxGroupResult {
    UseCheckboxGroupResult {
        label: props.label.clone(),
        selected_value: props.value.clone(),
        is_disabled: props.is_disabled,
        is_required: props.is_required,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        checkbox_group_props: CheckboxGroupProps {
            role: "group",
            label: props.label.clone(),
            aria_label: props.label,
            value: props.value.clone(),
            on_change: props.on_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            required: props.is_required,
            aria_required: props.is_required,
            invalid: props.is_invalid,
            aria_invalid: props.is_invalid,
            read_only: props.is_read_only,
            aria_read_only: props.is_read_only,
            data_selected_value: props.value,
        },
    }
}

pub fn use_checkbox_value(props: UseCheckboxProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_checkbox(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_checkbox hook did not serialize: {error}"
        ))
    })
}

pub fn use_checkbox_group_value(props: UseCheckboxGroupProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_checkbox_group(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_checkbox_group hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
