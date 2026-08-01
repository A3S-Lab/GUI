use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

/// One declarative drag item containing all of its text representations.
pub type DragItem = BTreeMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseDragProps {
    on_drag_start: Option<String>,
    on_drag_move: Option<String>,
    on_drag_end: Option<String>,
    drag_type: Option<String>,
    drag_value: Option<String>,
    drag_items: Option<String>,
    allowed_drop_operations: Option<String>,
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

    /// Supplies the React Aria-style `getItems()` result as typed maps.
    pub fn drag_items(mut self, drag_items: Vec<DragItem>) -> Self {
        self.drag_items = encode_drag_items(drag_items);
        self
    }

    /// Supplies encoded drag items for declarative RSX/TSX protocol props.
    pub fn drag_items_json(mut self, drag_items: Option<impl Into<String>>) -> Self {
        self.drag_items = non_empty(drag_items).and_then(normalize_drag_items_json);
        self
    }

    pub fn allowed_drop_operations(
        mut self,
        allowed_drop_operations: Option<impl Into<String>>,
    ) -> Self {
        self.allowed_drop_operations = non_empty(allowed_drop_operations);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_drop_operations: Option<String>,
    #[serde(rename = "data-drag-type", skip_serializing_if = "Option::is_none")]
    pub data_drag_type: Option<String>,
    #[serde(rename = "data-drag-value", skip_serializing_if = "Option::is_none")]
    pub data_drag_value: Option<String>,
    #[serde(rename = "data-drag-items", skip_serializing_if = "Option::is_none")]
    pub data_drag_items: Option<String>,
    #[serde(
        rename = "data-allowed-drop-operations",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_allowed_drop_operations: Option<String>,
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
            drag_items: props.drag_items.clone(),
            allowed_drop_operations: props.allowed_drop_operations.clone(),
            data_drag_type: props.drag_type,
            data_drag_value: props.drag_value,
            data_drag_items: props.drag_items,
            data_allowed_drop_operations: props.allowed_drop_operations,
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

fn normalize_drag_items_json(raw: String) -> Option<String> {
    let items = serde_json::from_str::<Vec<DragItem>>(&raw).ok()?;
    encode_drag_items(items)
}

fn encode_drag_items(items: Vec<DragItem>) -> Option<String> {
    if items.is_empty() {
        return None;
    }
    let items = items
        .into_iter()
        .map(|item| {
            item.into_iter()
                .filter_map(|(format, value)| {
                    let format = format.trim();
                    (!format.is_empty()).then(|| (format.to_string(), value))
                })
                .collect::<DragItem>()
        })
        .collect::<Vec<_>>();
    if items.iter().any(DragItem::is_empty) {
        return None;
    }
    serde_json::to_string(&items).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_drag_items_encode_multiple_items_and_formats() {
        let result = use_drag(UseDragProps::new().drag_items(vec![
            DragItem::from([
                ("text/plain".to_string(), "alpha".to_string()),
                ("text/html".to_string(), "<b>alpha</b>".to_string()),
            ]),
            DragItem::from([("text/plain".to_string(), "beta".to_string())]),
        ]));

        let encoded = result.drag_props.data_drag_items.as_deref().unwrap();
        let items = serde_json::from_str::<Vec<DragItem>>(encoded).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(
            items[0].get("text/html").map(String::as_str),
            Some("<b>alpha</b>")
        );
        assert_eq!(items[1].get("text/plain").map(String::as_str), Some("beta"));
    }

    #[test]
    fn malformed_or_empty_drag_items_are_not_lowered() {
        assert!(
            use_drag(UseDragProps::new().drag_items_json(Some("not-json")))
                .drag_props
                .data_drag_items
                .is_none()
        );
        assert!(use_drag(UseDragProps::new().drag_items(Vec::new()))
            .drag_props
            .data_drag_items
            .is_none());
    }
}

pub fn use_drag_value(props: UseDragProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_drag(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_drag hook did not serialize: {error}"))
    })
}
