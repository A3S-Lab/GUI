use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseFileTriggerProps {
    on_press: Option<String>,
    on_select: Option<String>,
    accepted_file_types: Option<String>,
    allows_multiple: bool,
    is_disabled: bool,
    is_pressed: bool,
}

impl UseFileTriggerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = non_empty(action);
        self
    }

    pub fn on_select(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_select = non_empty(action);
        self
    }

    pub fn accepted_file_types(mut self, accepted_file_types: Option<impl Into<String>>) -> Self {
        self.accepted_file_types = non_empty(accepted_file_types);
        self
    }

    pub fn allows_multiple(mut self, allows_multiple: bool) -> Self {
        self.allows_multiple = allows_multiple;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.is_pressed = pressed;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFileTriggerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_file_types: Option<String>,
    pub allows_multiple: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub file_trigger_props: FileTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_select: Option<String>,
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accepted_file_types: Option<String>,
    #[serde(rename = "multiple", skip_serializing_if = "is_false")]
    pub allows_multiple: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDropZoneProps {
    label: Option<String>,
    on_drop: Option<String>,
    on_drag_enter: Option<String>,
    on_drag_leave: Option<String>,
    is_disabled: bool,
}

impl UseDropZoneProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn on_drop(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drop = non_empty(action);
        self
    }

    pub fn on_drag_enter(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drag_enter = non_empty(action);
        self
    }

    pub fn on_drag_leave(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drag_leave = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDropZoneResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub is_disabled: bool,
    pub drop_zone_props: DropZoneProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropZoneProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_enter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_leave: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_file_trigger(props: UseFileTriggerProps) -> UseFileTriggerResult {
    UseFileTriggerResult {
        accepted_file_types: props.accepted_file_types.clone(),
        allows_multiple: props.allows_multiple,
        is_disabled: props.is_disabled,
        is_pressed: props.is_pressed,
        file_trigger_props: FileTriggerProps {
            role: "button",
            tab_index: if props.is_disabled { -1 } else { 0 },
            on_press: props.on_press,
            on_select: props.on_select,
            accepted_file_types: props.accepted_file_types,
            allows_multiple: props.allows_multiple,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_pressed: props.is_pressed,
        },
    }
}

pub fn use_drop_zone(props: UseDropZoneProps) -> UseDropZoneResult {
    UseDropZoneResult {
        label: props.label.clone(),
        is_disabled: props.is_disabled,
        drop_zone_props: DropZoneProps {
            label: props.label,
            on_drop: props.on_drop,
            on_drag_enter: props.on_drag_enter,
            on_drag_leave: props.on_drag_leave,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_file_trigger_value(props: UseFileTriggerProps) -> GuiResult<JsonValue> {
    serialize_hook("use_file_trigger", use_file_trigger(props))
}

pub fn use_drop_zone_value(props: UseDropZoneProps) -> GuiResult<JsonValue> {
    serialize_hook("use_drop_zone", use_drop_zone(props))
}

fn serialize_hook<T: Serialize>(hook: &str, value: T) -> GuiResult<JsonValue> {
    serde_json::to_value(value).map_err(|error| {
        GuiError::invalid_tree(format!("semantic {hook} hook did not serialize: {error}"))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
