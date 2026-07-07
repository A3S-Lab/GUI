use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverlayTriggerKind {
    #[default]
    None,
    Dialog,
    Menu,
    ListBox,
    Tooltip,
    Tree,
}

impl OverlayTriggerKind {
    fn from_option(value: Option<impl Into<String>>) -> Self {
        match value
            .map(Into::into)
            .map(|value| value.to_ascii_lowercase())
            .as_deref()
        {
            Some("dialog") => Self::Dialog,
            Some("menu") => Self::Menu,
            Some("listbox") | Some("list-box") => Self::ListBox,
            Some("tooltip") => Self::Tooltip,
            Some("tree") => Self::Tree,
            _ => Self::None,
        }
    }

    fn aria_haspopup(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Dialog => Some("dialog"),
            Self::Menu => Some("menu"),
            Self::ListBox => Some("listbox"),
            Self::Tooltip => Some("true"),
            Self::Tree => Some("tree"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseOverlayProps {
    is_open: bool,
    on_open_change: Option<String>,
    on_close: Option<String>,
    is_disabled: bool,
    trigger_kind: OverlayTriggerKind,
}

impl UseOverlayProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_close(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_close = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn trigger_kind(mut self, trigger_kind: Option<impl Into<String>>) -> Self {
        self.trigger_kind = OverlayTriggerKind::from_option(trigger_kind);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseOverlayResult {
    pub is_open: bool,
    pub overlay_props: OverlayProps,
    pub overlay_trigger_props: OverlayTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayProps {
    pub open: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_open_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_close: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "aria-haspopup", skip_serializing_if = "Option::is_none")]
    pub aria_haspopup: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_overlay(props: UseOverlayProps) -> UseOverlayResult {
    let trigger_action = props
        .on_open_change
        .clone()
        .or_else(|| props.on_close.clone());

    UseOverlayResult {
        is_open: props.is_open,
        overlay_props: OverlayProps {
            open: props.is_open,
            data_open: props.is_open,
            on_open_change: props.on_open_change,
            on_close: props.on_close,
        },
        overlay_trigger_props: OverlayTriggerProps {
            role: "button",
            tab_index: 0,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            aria_haspopup: props.trigger_kind.aria_haspopup(),
            on_press: trigger_action,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_overlay_value(props: UseOverlayProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_overlay(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_overlay hook did not serialize: {error}"
        ))
    })
}
