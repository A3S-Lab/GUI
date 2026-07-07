use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseFieldProps {
    label: Option<String>,
    is_disabled: bool,
    is_required: bool,
    is_invalid: bool,
    is_read_only: bool,
}

impl UseFieldProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
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
pub struct UseFieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub field_props: FieldProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldProps {
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

pub fn use_field(props: UseFieldProps) -> UseFieldResult {
    UseFieldResult {
        label: props.label,
        field_props: FieldProps {
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

pub fn use_field_value(props: UseFieldProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_field(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_field hook did not serialize: {error}"
        ))
    })
}
