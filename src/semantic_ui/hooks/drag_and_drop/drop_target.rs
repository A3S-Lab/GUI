use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDropProps {
    label: Option<String>,
    on_drop: Option<String>,
    on_drop_enter: Option<String>,
    on_drop_exit: Option<String>,
    on_drop_move: Option<String>,
    accepted_drag_types: Option<String>,
    drop_operation: Option<String>,
    is_disabled: bool,
    is_drop_target: bool,
}

impl UseDropProps {
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

    pub fn on_drop_enter(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drop_enter = non_empty(action);
        self
    }

    pub fn on_drop_exit(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drop_exit = non_empty(action);
        self
    }

    pub fn on_drop_move(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drop_move = non_empty(action);
        self
    }

    pub fn accepted_drag_types(mut self, accepted_drag_types: Option<impl Into<String>>) -> Self {
        self.accepted_drag_types = non_empty(accepted_drag_types);
        self
    }

    pub fn drop_operation(mut self, drop_operation: Option<impl Into<String>>) -> Self {
        self.drop_operation = non_empty(drop_operation);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn drop_target(mut self, drop_target: bool) -> Self {
        self.is_drop_target = drop_target;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDropResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub is_disabled: bool,
    pub is_drop_target: bool,
    pub drop_props: DropProps,
    pub drop_button_props: DropButtonProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop_enter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop_exit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop_move: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_drag_types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_operation: Option<String>,
    #[serde(
        rename = "data-accepted-drag-types",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_accepted_drag_types: Option<String>,
    #[serde(
        rename = "data-drop-operation",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_drop_operation: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-drop-target")]
    pub data_drop_target: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropButtonProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drop: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-drop-target")]
    pub data_drop_target: bool,
}

pub fn use_drop(props: UseDropProps) -> UseDropResult {
    let tab_index = if props.is_disabled { -1 } else { 0 };
    UseDropResult {
        label: props.label.clone(),
        is_disabled: props.is_disabled,
        is_drop_target: props.is_drop_target,
        drop_props: DropProps {
            label: props.label,
            on_drop: props.on_drop.clone(),
            on_drop_enter: props.on_drop_enter,
            on_drop_exit: props.on_drop_exit,
            on_drop_move: props.on_drop_move,
            accepted_drag_types: props.accepted_drag_types.clone(),
            drop_operation: props.drop_operation.clone(),
            data_accepted_drag_types: props.accepted_drag_types,
            data_drop_operation: props.drop_operation,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_drop_target: props.is_drop_target,
        },
        drop_button_props: DropButtonProps {
            role: "button",
            tab_index,
            on_drop: props.on_drop,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_drop_target: props.is_drop_target,
        },
    }
}

pub fn use_drop_value(props: UseDropProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_drop(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_drop hook did not serialize: {error}"))
    })
}
