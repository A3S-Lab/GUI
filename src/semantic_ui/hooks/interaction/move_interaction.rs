use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::{finite_or, non_empty};

#[derive(Debug, Clone, PartialEq)]
pub struct UseMoveProps {
    on_move_start: Option<String>,
    on_move: Option<String>,
    on_move_end: Option<String>,
    is_disabled: bool,
    is_moving: bool,
    x_delta: f64,
    y_delta: f64,
}

impl Default for UseMoveProps {
    fn default() -> Self {
        Self {
            on_move_start: None,
            on_move: None,
            on_move_end: None,
            is_disabled: false,
            is_moving: false,
            x_delta: 0.0,
            y_delta: 0.0,
        }
    }
}

impl UseMoveProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_move_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_move_start = non_empty(action);
        self
    }

    pub fn on_move(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_move = non_empty(action);
        self
    }

    pub fn on_move_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_move_end = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn moving(mut self, moving: bool) -> Self {
        self.is_moving = moving;
        self
    }

    pub fn x_delta(mut self, x_delta: f64) -> Self {
        self.x_delta = finite_or(x_delta, 0.0);
        self
    }

    pub fn y_delta(mut self, y_delta: f64) -> Self {
        self.y_delta = finite_or(y_delta, 0.0);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseMoveResult {
    pub is_moving: bool,
    pub x_delta: f64,
    pub y_delta: f64,
    pub move_props: MoveProps,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_move_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_move: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_move_end: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-moving")]
    pub data_moving: bool,
    #[serde(rename = "data-x-delta")]
    pub data_x_delta: f64,
    #[serde(rename = "data-y-delta")]
    pub data_y_delta: f64,
}

pub fn use_move(props: UseMoveProps) -> UseMoveResult {
    let x_delta = finite_or(props.x_delta, 0.0);
    let y_delta = finite_or(props.y_delta, 0.0);
    UseMoveResult {
        is_moving: props.is_moving,
        x_delta,
        y_delta,
        move_props: MoveProps {
            tab_index: if props.is_disabled { -1 } else { 0 },
            on_move_start: props.on_move_start,
            on_move: props.on_move,
            on_move_end: props.on_move_end,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_moving: props.is_moving,
            data_x_delta: x_delta,
            data_y_delta: y_delta,
        },
    }
}

pub fn use_move_value(props: UseMoveProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_move(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_move hook did not serialize: {error}"))
    })
}
