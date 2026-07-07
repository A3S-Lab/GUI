use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseClipboardProps {
    on_copy: Option<String>,
    on_cut: Option<String>,
    on_paste: Option<String>,
    copy_value: Option<String>,
    copy_mime_type: Option<String>,
    accepted_mime_types: Option<String>,
    is_disabled: bool,
}

impl Default for UseClipboardProps {
    fn default() -> Self {
        Self {
            on_copy: None,
            on_cut: None,
            on_paste: None,
            copy_value: None,
            copy_mime_type: None,
            accepted_mime_types: None,
            is_disabled: false,
        }
    }
}

impl UseClipboardProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_copy(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_copy = non_empty(action);
        self
    }

    pub fn on_cut(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_cut = non_empty(action);
        self
    }

    pub fn on_paste(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_paste = non_empty(action);
        self
    }

    pub fn copy_value(mut self, value: Option<impl Into<String>>) -> Self {
        self.copy_value = non_empty(value);
        self
    }

    pub fn copy_mime_type(mut self, mime_type: Option<impl Into<String>>) -> Self {
        self.copy_mime_type = non_empty(mime_type);
        self
    }

    pub fn accepted_mime_types(mut self, mime_types: Option<impl Into<String>>) -> Self {
        self.accepted_mime_types = non_empty(mime_types);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseClipboardResult {
    pub is_clipboard_disabled: bool,
    pub clipboard_props: ClipboardProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_cut: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_paste: Option<String>,
    #[serde(rename = "data-copy-value", skip_serializing_if = "Option::is_none")]
    pub data_copy_value: Option<String>,
    #[serde(
        rename = "data-copy-mime-type",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_copy_mime_type: Option<String>,
    #[serde(
        rename = "data-accepted-mime-types",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_accepted_mime_types: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-clipboard-disabled")]
    pub data_clipboard_disabled: bool,
}

pub fn use_clipboard(props: UseClipboardProps) -> UseClipboardResult {
    UseClipboardResult {
        is_clipboard_disabled: props.is_disabled,
        clipboard_props: ClipboardProps {
            role: "textbox",
            tab_index: if props.is_disabled { -1 } else { 0 },
            on_copy: props.on_copy,
            on_cut: props.on_cut,
            on_paste: props.on_paste,
            data_copy_value: props.copy_value,
            data_copy_mime_type: props.copy_mime_type,
            data_accepted_mime_types: props.accepted_mime_types,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_clipboard_disabled: props.is_disabled,
        },
    }
}

pub fn use_clipboard_value(props: UseClipboardProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_clipboard(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_clipboard hook did not serialize: {error}"
        ))
    })
}
