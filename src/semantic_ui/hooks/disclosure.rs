use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDisclosureProps {
    on_expanded_change: Option<String>,
    is_expanded: bool,
    is_disabled: bool,
}

impl UseDisclosureProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_expanded_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_expanded_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDisclosureGroupProps {
    label: Option<String>,
    expanded_keys: Option<String>,
    on_expanded_change: Option<String>,
    allows_multiple_expanded: bool,
    is_disabled: bool,
}

impl UseDisclosureGroupProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn expanded_keys(mut self, expanded_keys: Option<impl Into<String>>) -> Self {
        self.expanded_keys = expanded_keys
            .map(Into::into)
            .filter(|expanded_keys| !expanded_keys.is_empty());
        self
    }

    pub fn on_expanded_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_expanded_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn allows_multiple_expanded(mut self, allows_multiple_expanded: bool) -> Self {
        self.allows_multiple_expanded = allows_multiple_expanded;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDisclosureResult {
    pub is_expanded: bool,
    pub disclosure_props: DisclosureProps,
    pub disclosure_trigger_props: DisclosureTriggerProps,
    pub disclosure_panel_props: DisclosurePanelProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDisclosureGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_keys: Option<String>,
    pub allows_multiple_expanded: bool,
    pub is_disabled: bool,
    pub disclosure_group_props: DisclosureGroupProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisclosureGroupProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_expanded_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disclosure-group")]
    pub data_disclosure_group: bool,
    #[serde(rename = "data-expanded-keys", skip_serializing_if = "Option::is_none")]
    pub data_expanded_keys: Option<String>,
    #[serde(rename = "data-allows-multiple-expanded")]
    pub data_allows_multiple_expanded: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisclosureProps {
    pub expanded: bool,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-expanded")]
    pub data_expanded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_expanded_change: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisclosureTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-expanded")]
    pub data_expanded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisclosurePanelProps {
    #[serde(rename = "data-expanded")]
    pub data_expanded: bool,
    #[serde(rename = "aria-hidden", skip_serializing_if = "is_false")]
    pub aria_hidden: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub hidden: bool,
}

pub fn use_disclosure(props: UseDisclosureProps) -> UseDisclosureResult {
    UseDisclosureResult {
        is_expanded: props.is_expanded,
        disclosure_props: DisclosureProps {
            expanded: props.is_expanded,
            aria_expanded: props.is_expanded,
            data_expanded: props.is_expanded,
            on_expanded_change: props.on_expanded_change.clone(),
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
        disclosure_trigger_props: DisclosureTriggerProps {
            role: "button",
            tab_index: 0,
            aria_expanded: props.is_expanded,
            data_expanded: props.is_expanded,
            on_press: props.on_expanded_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
        disclosure_panel_props: DisclosurePanelProps {
            data_expanded: props.is_expanded,
            aria_hidden: !props.is_expanded,
            hidden: !props.is_expanded,
        },
    }
}

pub fn use_disclosure_group(props: UseDisclosureGroupProps) -> UseDisclosureGroupResult {
    UseDisclosureGroupResult {
        label: props.label.clone(),
        expanded_keys: props.expanded_keys.clone(),
        allows_multiple_expanded: props.allows_multiple_expanded,
        is_disabled: props.is_disabled,
        disclosure_group_props: DisclosureGroupProps {
            role: "group",
            label: props.label,
            on_expanded_change: props.on_expanded_change,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_disclosure_group: true,
            data_expanded_keys: props.expanded_keys,
            data_allows_multiple_expanded: props.allows_multiple_expanded,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_disclosure_value(props: UseDisclosureProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_disclosure(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_disclosure hook did not serialize: {error}"
        ))
    })
}

pub fn use_disclosure_group_value(props: UseDisclosureGroupProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_disclosure_group(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_disclosure_group hook did not serialize: {error}"
        ))
    })
}
