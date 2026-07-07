use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCollectionProps {
    label: Option<String>,
    item_count: usize,
    is_empty: bool,
    is_disabled: bool,
}

impl UseCollectionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn item_count(mut self, item_count: usize) -> Self {
        self.item_count = item_count;
        self
    }

    pub fn empty(mut self, empty: bool) -> Self {
        self.is_empty = empty;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCollectionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub item_count: usize,
    pub is_empty: bool,
    pub is_disabled: bool,
    pub collection_props: CollectionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-collection")]
    pub data_collection: bool,
    #[serde(rename = "data-item-count")]
    pub data_item_count: usize,
    #[serde(rename = "data-empty")]
    pub data_empty: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CollectionSectionKind {
    Generic,
    ListBox,
    GridList,
    Menu,
    Tree,
}

impl Default for CollectionSectionKind {
    fn default() -> Self {
        Self::Generic
    }
}

impl CollectionSectionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::ListBox => "list-box",
            Self::GridList => "grid-list",
            Self::Menu => "menu",
            Self::Tree => "tree",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseCollectionSectionProps {
    label: Option<String>,
    collection_kind: CollectionSectionKind,
    is_disabled: bool,
}

impl UseCollectionSectionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn collection_kind(mut self, kind: CollectionSectionKind) -> Self {
        self.collection_kind = kind;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCollectionSectionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub collection_kind: CollectionSectionKind,
    pub is_disabled: bool,
    pub collection_section_props: CollectionSectionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSectionProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-collection-section")]
    pub data_collection_section: bool,
    #[serde(rename = "data-collection-kind")]
    pub data_collection_kind: &'static str,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

pub fn use_collection(props: UseCollectionProps) -> UseCollectionResult {
    UseCollectionResult {
        label: props.label.clone(),
        item_count: props.item_count,
        is_empty: props.is_empty,
        is_disabled: props.is_disabled,
        collection_props: CollectionProps {
            role: "group",
            label: props.label,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_collection: true,
            data_item_count: props.item_count,
            data_empty: props.is_empty,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_collection_section(props: UseCollectionSectionProps) -> UseCollectionSectionResult {
    UseCollectionSectionResult {
        label: props.label.clone(),
        collection_kind: props.collection_kind,
        is_disabled: props.is_disabled,
        collection_section_props: CollectionSectionProps {
            role: "group",
            label: props.label.clone(),
            aria_label: props.label,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_collection_section: true,
            data_collection_kind: props.collection_kind.as_str(),
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_collection_value(props: UseCollectionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_collection(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_collection hook did not serialize: {error}"
        ))
    })
}

pub fn use_collection_section_value(props: UseCollectionSectionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_collection_section(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_collection_section hook did not serialize: {error}"
        ))
    })
}
