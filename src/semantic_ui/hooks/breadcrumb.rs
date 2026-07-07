use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseBreadcrumbsProps {
    label: Option<String>,
}

impl UseBreadcrumbsProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseBreadcrumbsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub breadcrumbs_props: BreadcrumbsProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreadcrumbsProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-breadcrumbs")]
    pub data_breadcrumbs: bool,
}

pub fn use_breadcrumbs(props: UseBreadcrumbsProps) -> UseBreadcrumbsResult {
    UseBreadcrumbsResult {
        label: props.label.clone(),
        breadcrumbs_props: BreadcrumbsProps {
            label: props.label,
            data_breadcrumbs: true,
        },
    }
}

pub fn use_breadcrumbs_value(props: UseBreadcrumbsProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_breadcrumbs(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_breadcrumbs hook did not serialize: {error}"
        ))
    })
}
