use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseToastProps {
    title: Option<String>,
    description: Option<String>,
    on_close: Option<String>,
}

impl UseToastProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: Option<impl Into<String>>) -> Self {
        self.title = non_empty(title);
        self
    }

    pub fn description(mut self, description: Option<impl Into<String>>) -> Self {
        self.description = non_empty(description);
        self
    }

    pub fn on_close(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_close = non_empty(action);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseToastRegionProps {
    label: Option<String>,
}

impl UseToastRegionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseToastResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub toast_props: ToastProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseToastRegionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub toast_region_props: ToastRegionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToastProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "data-description", skip_serializing_if = "Option::is_none")]
    pub data_description: Option<String>,
    #[serde(rename = "aria-live")]
    pub aria_live: &'static str,
    #[serde(rename = "aria-atomic")]
    pub aria_atomic: bool,
    #[serde(rename = "data-toast")]
    pub data_toast: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_close: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToastRegionProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-live")]
    pub aria_live: &'static str,
    #[serde(rename = "data-toast-region")]
    pub data_toast_region: bool,
}

pub fn use_toast(props: UseToastProps) -> UseToastResult {
    let label = props.title.clone().or_else(|| props.description.clone());

    UseToastResult {
        title: props.title.clone(),
        description: props.description.clone(),
        toast_props: ToastProps {
            role: "status",
            label,
            title: props.title,
            data_description: props.description,
            aria_live: "polite",
            aria_atomic: true,
            data_toast: true,
            on_close: props.on_close,
        },
    }
}

pub fn use_toast_region(props: UseToastRegionProps) -> UseToastRegionResult {
    UseToastRegionResult {
        label: props.label.clone(),
        toast_region_props: ToastRegionProps {
            role: "region",
            label: props.label,
            aria_live: "polite",
            data_toast_region: true,
        },
    }
}

pub fn use_toast_value(props: UseToastProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_toast(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_toast hook did not serialize: {error}"
        ))
    })
}

pub fn use_toast_region_value(props: UseToastRegionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_toast_region(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_toast_region hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
