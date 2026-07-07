use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTabListProps {
    label: Option<String>,
    orientation: Option<String>,
    is_disabled: bool,
}

impl UseTabListProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = orientation
            .map(Into::into)
            .filter(|orientation| !orientation.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTabListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub orientation: String,
    pub is_disabled: bool,
    pub tab_list_props: TabListProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabListProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(rename = "aria-orientation")]
    pub aria_orientation: String,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTabProps {
    value: Option<String>,
    text_value: Option<String>,
    is_selected: bool,
    is_disabled: bool,
}

impl UseTabProps {
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
pub struct UseTabResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub tab_props: TabProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabProps {
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
    #[serde(rename = "aria-selected")]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTabPanelProps {
    value: Option<String>,
}

impl UseTabPanelProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: Option<impl Into<String>>) -> Self {
        self.value = value.map(Into::into).filter(|value| !value.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTabPanelResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub tab_panel_props: TabPanelProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabPanelProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub fn use_tab_list(props: UseTabListProps) -> UseTabListResult {
    let orientation = props
        .orientation
        .unwrap_or_else(|| "horizontal".to_string());

    UseTabListResult {
        label: props.label.clone(),
        orientation: orientation.clone(),
        is_disabled: props.is_disabled,
        tab_list_props: TabListProps {
            role: "tablist",
            label: props.label.clone(),
            aria_label: props.label,
            aria_orientation: orientation,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_tab(props: UseTabProps) -> UseTabResult {
    UseTabResult {
        value: props.value.clone(),
        text_value: props.text_value.clone(),
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        tab_props: TabProps {
            role: "tab",
            tab_index: if props.is_selected { 0 } else { -1 },
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
        },
    }
}

pub fn use_tab_panel(props: UseTabPanelProps) -> UseTabPanelResult {
    UseTabPanelResult {
        value: props.value.clone(),
        tab_panel_props: TabPanelProps {
            role: "tabpanel",
            tab_index: 0,
            value: props.value,
        },
    }
}

pub fn use_tab_list_value(props: UseTabListProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_tab_list(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_tab_list hook did not serialize: {error}"
        ))
    })
}

pub fn use_tab_value(props: UseTabProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_tab(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_tab hook did not serialize: {error}"))
    })
}

pub fn use_tab_panel_value(props: UseTabPanelProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_tab_panel(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_tab_panel hook did not serialize: {error}"
        ))
    })
}
