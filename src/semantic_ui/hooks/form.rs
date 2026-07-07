use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseFormProps {
    label: Option<String>,
    on_submit: Option<String>,
    on_reset: Option<String>,
    on_invalid: Option<String>,
    validation_behavior: Option<String>,
    is_disabled: bool,
    is_invalid: bool,
    no_validate: bool,
}

impl UseFormProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn on_submit(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_submit = non_empty(action);
        self
    }

    pub fn on_reset(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_reset = non_empty(action);
        self
    }

    pub fn on_invalid(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_invalid = non_empty(action);
        self
    }

    pub fn validation_behavior(mut self, validation_behavior: Option<impl Into<String>>) -> Self {
        self.validation_behavior = non_empty(validation_behavior);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
        self
    }

    pub fn no_validate(mut self, no_validate: bool) -> Self {
        self.no_validate = no_validate;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFormResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_behavior: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub no_validate: bool,
    pub form_props: FormProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_reset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_invalid: Option<String>,
    #[serde(rename = "noValidate", skip_serializing_if = "is_false")]
    pub no_validate: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
    #[serde(rename = "data-invalid")]
    pub data_invalid: bool,
    #[serde(
        rename = "data-validation-behavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_validation_behavior: Option<String>,
}

pub fn use_form(props: UseFormProps) -> UseFormResult {
    UseFormResult {
        label: props.label.clone(),
        validation_behavior: props.validation_behavior.clone(),
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        no_validate: props.no_validate,
        form_props: FormProps {
            label: props.label,
            on_submit: props.on_submit,
            on_reset: props.on_reset,
            on_invalid: props.on_invalid,
            no_validate: props.no_validate,
            aria_disabled: props.is_disabled,
            aria_invalid: props.is_invalid,
            data_disabled: props.is_disabled,
            data_invalid: props.is_invalid,
            data_validation_behavior: props.validation_behavior,
        },
    }
}

pub fn use_form_value(props: UseFormProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_form(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_form hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
