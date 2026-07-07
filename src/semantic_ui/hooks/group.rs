use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseGroupProps {
    label: Option<String>,
    on_hover_start: Option<String>,
    on_hover_end: Option<String>,
    on_hover_change: Option<String>,
    on_focus: Option<String>,
    on_blur: Option<String>,
    on_focus_change: Option<String>,
    is_disabled: bool,
    is_invalid: bool,
    is_read_only: bool,
    is_hovered: bool,
    is_focused: bool,
    is_focus_visible: bool,
    is_focus_within: bool,
    auto_focus: bool,
    tab_index: i32,
}

impl UseGroupProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn on_hover_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_start = non_empty(action);
        self
    }

    pub fn on_hover_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_end = non_empty(action);
        self
    }

    pub fn on_hover_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_hover_change = non_empty(action);
        self
    }

    pub fn on_focus(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus = non_empty(action);
        self
    }

    pub fn on_blur(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_blur = non_empty(action);
        self
    }

    pub fn on_focus_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus_change = non_empty(action);
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

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.is_hovered = hovered;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.is_focus_visible = focus_visible;
        self
    }

    pub fn focus_within(mut self, focus_within: bool) -> Self {
        self.is_focus_within = focus_within;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub is_focus_within: bool,
    pub group_props: GroupProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hover_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_blur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus_change: Option<String>,
    #[serde(rename = "autoFocus", skip_serializing_if = "is_false")]
    pub auto_focus: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "aria-invalid", skip_serializing_if = "is_false")]
    pub aria_invalid: bool,
    #[serde(rename = "aria-readonly", skip_serializing_if = "is_false")]
    pub aria_read_only: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
    #[serde(rename = "data-invalid")]
    pub data_invalid: bool,
    #[serde(rename = "data-readonly")]
    pub data_read_only: bool,
    #[serde(rename = "data-hovered")]
    pub data_hovered: bool,
    #[serde(rename = "data-focused")]
    pub data_focused: bool,
    #[serde(rename = "data-focus-visible")]
    pub data_focus_visible: bool,
    #[serde(rename = "data-focus-within")]
    pub data_focus_within: bool,
}

pub fn use_group(props: UseGroupProps) -> UseGroupResult {
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };
    UseGroupResult {
        label: props.label.clone(),
        is_disabled: props.is_disabled,
        is_invalid: props.is_invalid,
        is_read_only: props.is_read_only,
        is_hovered: props.is_hovered,
        is_focused: props.is_focused,
        is_focus_visible: props.is_focus_visible,
        is_focus_within: props.is_focus_within,
        group_props: GroupProps {
            role: "group",
            label: props.label,
            tab_index,
            on_hover_start: props.on_hover_start,
            on_hover_end: props.on_hover_end,
            on_hover_change: props.on_hover_change,
            on_focus: props.on_focus,
            on_blur: props.on_blur,
            on_focus_change: props.on_focus_change,
            auto_focus: props.auto_focus,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            aria_invalid: props.is_invalid,
            aria_read_only: props.is_read_only,
            data_disabled: props.is_disabled,
            data_invalid: props.is_invalid,
            data_read_only: props.is_read_only,
            data_hovered: props.is_hovered,
            data_focused: props.is_focused,
            data_focus_visible: props.is_focus_visible,
            data_focus_within: props.is_focus_within,
        },
    }
}

pub fn use_group_value(props: UseGroupProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_group(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_group hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
