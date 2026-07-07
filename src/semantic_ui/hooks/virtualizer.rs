use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseVirtualizerProps {
    label: Option<String>,
    layout: Option<String>,
    orientation: Option<String>,
    item_count: usize,
    estimated_item_size: u32,
    visible_start: usize,
    visible_end: usize,
    overscan: usize,
    gap: u32,
    padding: u32,
    is_scrolling: bool,
    is_disabled: bool,
    tab_index: i32,
}

impl Default for UseVirtualizerProps {
    fn default() -> Self {
        Self {
            label: None,
            layout: Some("list".to_string()),
            orientation: Some("vertical".to_string()),
            item_count: 0,
            estimated_item_size: 40,
            visible_start: 0,
            visible_end: 0,
            overscan: 2,
            gap: 0,
            padding: 0,
            is_scrolling: false,
            is_disabled: false,
            tab_index: 0,
        }
    }
}

impl UseVirtualizerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = non_empty(label);
        self
    }

    pub fn layout(mut self, layout: Option<impl Into<String>>) -> Self {
        self.layout = non_empty(layout);
        self
    }

    pub fn orientation(mut self, orientation: Option<impl Into<String>>) -> Self {
        self.orientation = non_empty(orientation);
        self
    }

    pub fn item_count(mut self, item_count: usize) -> Self {
        self.item_count = item_count;
        self
    }

    pub fn estimated_item_size(mut self, estimated_item_size: u32) -> Self {
        self.estimated_item_size = estimated_item_size.max(1);
        self
    }

    pub fn visible_start(mut self, visible_start: usize) -> Self {
        self.visible_start = visible_start;
        self
    }

    pub fn visible_end(mut self, visible_end: usize) -> Self {
        self.visible_end = visible_end;
        self
    }

    pub fn overscan(mut self, overscan: usize) -> Self {
        self.overscan = overscan;
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn scrolling(mut self, scrolling: bool) -> Self {
        self.is_scrolling = scrolling;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseVirtualizerResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub layout: String,
    pub orientation: String,
    pub item_count: usize,
    pub estimated_item_size: u32,
    pub visible_start: usize,
    pub visible_end: usize,
    pub overscan: usize,
    pub gap: u32,
    pub padding: u32,
    pub is_scrolling: bool,
    pub is_disabled: bool,
    pub virtualizer_props: VirtualizerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualizerProps {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-virtualizer")]
    pub data_virtualizer: bool,
    #[serde(rename = "data-layout")]
    pub data_layout: String,
    #[serde(rename = "data-orientation")]
    pub data_orientation: String,
    #[serde(rename = "data-item-count")]
    pub data_item_count: usize,
    #[serde(rename = "data-estimated-item-size")]
    pub data_estimated_item_size: u32,
    #[serde(rename = "data-visible-start")]
    pub data_visible_start: usize,
    #[serde(rename = "data-visible-end")]
    pub data_visible_end: usize,
    #[serde(rename = "data-overscan")]
    pub data_overscan: usize,
    #[serde(rename = "data-gap")]
    pub data_gap: u32,
    #[serde(rename = "data-padding")]
    pub data_padding: u32,
    #[serde(rename = "data-scrolling")]
    pub data_scrolling: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

pub fn use_virtualizer(props: UseVirtualizerProps) -> UseVirtualizerResult {
    let layout = normalized_layout(props.layout);
    let orientation = normalized_orientation(props.orientation);
    let visible_start = props.visible_start.min(props.item_count);
    let visible_end = props.visible_end.min(props.item_count).max(visible_start);
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };

    UseVirtualizerResult {
        label: props.label.clone(),
        layout: layout.clone(),
        orientation: orientation.clone(),
        item_count: props.item_count,
        estimated_item_size: props.estimated_item_size,
        visible_start,
        visible_end,
        overscan: props.overscan,
        gap: props.gap,
        padding: props.padding,
        is_scrolling: props.is_scrolling,
        is_disabled: props.is_disabled,
        virtualizer_props: VirtualizerProps {
            role: layout.clone(),
            label: props.label,
            tab_index,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_virtualizer: true,
            data_layout: layout,
            data_orientation: orientation,
            data_item_count: props.item_count,
            data_estimated_item_size: props.estimated_item_size,
            data_visible_start: visible_start,
            data_visible_end: visible_end,
            data_overscan: props.overscan,
            data_gap: props.gap,
            data_padding: props.padding,
            data_scrolling: props.is_scrolling,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_virtualizer_value(props: UseVirtualizerProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_virtualizer(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_virtualizer hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}

fn normalized_layout(layout: Option<String>) -> String {
    match layout
        .as_deref()
        .unwrap_or("list")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "grid" => "grid".to_string(),
        _ => "list".to_string(),
    }
}

fn normalized_orientation(orientation: Option<String>) -> String {
    match orientation
        .as_deref()
        .unwrap_or("vertical")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "horizontal" => "horizontal".to_string(),
        _ => "vertical".to_string(),
    }
}
