use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::{is_false, is_none_or_false};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCollectionItemProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
    is_expanded: Option<bool>,
}

impl UseCollectionItemProps {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCollectionItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expanded: Option<bool>,
    pub collection_item_props: CollectionItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionItemProps {
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
}

pub fn use_collection_item(props: UseCollectionItemProps) -> UseCollectionItemResult {
    UseCollectionItemResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        is_expanded: props.is_expanded,
        collection_item_props: CollectionItemProps {
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
        },
    }
}

pub fn use_collection_item_value(props: UseCollectionItemProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_collection_item(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_collection_item hook did not serialize: {error}"
        ))
    })
}
