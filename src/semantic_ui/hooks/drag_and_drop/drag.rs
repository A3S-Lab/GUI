use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDragProps {
    on_drag_start: Option<String>,
    on_drag_move: Option<String>,
    on_drag_end: Option<String>,
    drag_type: Option<String>,
    drag_value: Option<String>,
    is_disabled: bool,
    is_dragging: bool,
}

impl UseDragProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_drag_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drag_start = non_empty(action);
        self
    }

    pub fn on_drag_move(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drag_move = non_empty(action);
        self
    }

    pub fn on_drag_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_drag_end = non_empty(action);
        self
    }

    pub fn drag_type(mut self, drag_type: Option<impl Into<String>>) -> Self {
        self.drag_type = non_empty(drag_type);
        self
    }

    pub fn drag_value(mut self, drag_value: Option<impl Into<String>>) -> Self {
        self.drag_value = non_empty(drag_value);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn dragging(mut self, dragging: bool) -> Self {
        self.is_dragging = dragging;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDragResult {
    pub is_dragging: bool,
    pub drag_props: DragProps,
    pub drag_button_props: DragButtonProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DragProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_move: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_value: Option<String>,
    #[serde(rename = "data-drag-type", skip_serializing_if = "Option::is_none")]
    pub data_drag_type: Option<String>,
    #[serde(rename = "data-drag-value", skip_serializing_if = "Option::is_none")]
    pub data_drag_value: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-dragging")]
    pub data_dragging: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DragButtonProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_drag_start: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "aria-pressed")]
    pub aria_pressed: bool,
    #[serde(rename = "data-dragging")]
    pub data_dragging: bool,
}

pub fn use_drag(props: UseDragProps) -> UseDragResult {
    let tab_index = if props.is_disabled { -1 } else { 0 };
    UseDragResult {
        is_dragging: props.is_dragging,
        drag_props: DragProps {
            draggable: (!props.is_disabled).then_some("true"),
            on_drag_start: props.on_drag_start.clone(),
            on_drag_move: props.on_drag_move,
            on_drag_end: props.on_drag_end,
            drag_type: props.drag_type.clone(),
            drag_value: props.drag_value.clone(),
            data_drag_type: props.drag_type,
            data_drag_value: props.drag_value,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_dragging: props.is_dragging,
        },
        drag_button_props: DragButtonProps {
            role: "button",
            tab_index,
            on_drag_start: props.on_drag_start,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            aria_pressed: props.is_dragging,
            data_dragging: props.is_dragging,
        },
    }
}

pub fn use_drag_value(props: UseDragProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_drag(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_drag hook did not serialize: {error}"))
    })
}
