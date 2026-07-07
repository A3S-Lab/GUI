use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTextFieldProps {
    value: Option<String>,
    placeholder: Option<String>,
    input_type: Option<String>,
    on_change: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseTextFieldProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into);
        self
    }

    pub fn placeholder(mut self, placeholder: Option<impl Into<String>>) -> Self {
        self.placeholder = placeholder
            .map(Into::into)
            .filter(|placeholder| !placeholder.is_empty());
        self
    }

    pub fn input_type(mut self, input_type: Option<impl Into<String>>) -> Self {
        self.input_type = input_type
            .map(Into::into)
            .filter(|input_type| !input_type.is_empty());
        self
    }

    pub fn on_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_change = action.map(Into::into).filter(|action| !action.is_empty());
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
pub struct UseTextFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub input_props: TextInputProps,
    pub field_props: TextFieldProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextInputProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
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
pub struct TextFieldProps {
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

pub fn use_text_field(props: UseTextFieldProps) -> UseTextFieldResult {
    UseTextFieldResult {
        value: props.value.clone(),
        input_props: TextInputProps {
            value: props.value,
            placeholder: props.placeholder,
            input_type: props.input_type,
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
        field_props: TextFieldProps {
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

pub fn use_text_field_value(props: UseTextFieldProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_text_field(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_text_field hook did not serialize: {error}"
        ))
    })
}
