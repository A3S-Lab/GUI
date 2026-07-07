use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTextProps {
    label: Option<String>,
    text_value: Option<String>,
}

impl UseTextProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = text_value
            .map(Into::into)
            .filter(|text_value| !text_value.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseHeadingProps {
    label: Option<String>,
    text_value: Option<String>,
    level: u32,
}

impl UseHeadingProps {
    pub fn new() -> Self {
        Self {
            label: None,
            text_value: None,
            level: 2,
        }
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = text_value
            .map(Into::into)
            .filter(|text_value| !text_value.is_empty());
        self
    }

    pub fn level(mut self, level: u32) -> Self {
        self.level = level.clamp(1, 6);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTextResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub text_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseLabelResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub label_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseDescriptionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub description_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFieldErrorResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub field_error_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseLegendResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub legend_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseVisuallyHiddenResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub visually_hidden_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseKeyboardResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub keyboard_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseListBoxHeaderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub list_box_header_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseGridListHeaderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub grid_list_header_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTreeHeaderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub tree_header_props: TextProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseHeadingResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub level: u32,
    pub heading_props: HeadingProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(rename = "aria-level")]
    pub aria_level: u32,
}

pub fn use_text(props: UseTextProps) -> UseTextResult {
    let text_props = text_props(&props);
    UseTextResult {
        label: text_props.label.clone(),
        text_value: text_props.text_value.clone(),
        text_props,
    }
}

pub fn use_label(props: UseTextProps) -> UseLabelResult {
    let label_props = text_props(&props);
    UseLabelResult {
        label: label_props.label.clone(),
        text_value: label_props.text_value.clone(),
        label_props,
    }
}

pub fn use_description(props: UseTextProps) -> UseDescriptionResult {
    let description_props = text_props(&props);
    UseDescriptionResult {
        label: description_props.label.clone(),
        text_value: description_props.text_value.clone(),
        description_props,
    }
}

pub fn use_field_error(props: UseTextProps) -> UseFieldErrorResult {
    let field_error_props = text_props(&props);
    UseFieldErrorResult {
        label: field_error_props.label.clone(),
        text_value: field_error_props.text_value.clone(),
        field_error_props,
    }
}

pub fn use_legend(props: UseTextProps) -> UseLegendResult {
    let legend_props = text_props(&props);
    UseLegendResult {
        label: legend_props.label.clone(),
        text_value: legend_props.text_value.clone(),
        legend_props,
    }
}

pub fn use_visually_hidden(props: UseTextProps) -> UseVisuallyHiddenResult {
    let visually_hidden_props = text_props(&props);
    UseVisuallyHiddenResult {
        label: visually_hidden_props.label.clone(),
        text_value: visually_hidden_props.text_value.clone(),
        visually_hidden_props,
    }
}

pub fn use_keyboard(props: UseTextProps) -> UseKeyboardResult {
    let keyboard_props = text_props(&props);
    UseKeyboardResult {
        label: keyboard_props.label.clone(),
        text_value: keyboard_props.text_value.clone(),
        keyboard_props,
    }
}

pub fn use_list_box_header(props: UseTextProps) -> UseListBoxHeaderResult {
    let list_box_header_props = text_props(&props);
    UseListBoxHeaderResult {
        label: list_box_header_props.label.clone(),
        text_value: list_box_header_props.text_value.clone(),
        list_box_header_props,
    }
}

pub fn use_grid_list_header(props: UseTextProps) -> UseGridListHeaderResult {
    let grid_list_header_props = text_props(&props);
    UseGridListHeaderResult {
        label: grid_list_header_props.label.clone(),
        text_value: grid_list_header_props.text_value.clone(),
        grid_list_header_props,
    }
}

pub fn use_tree_header(props: UseTextProps) -> UseTreeHeaderResult {
    let tree_header_props = text_props(&props);
    UseTreeHeaderResult {
        label: tree_header_props.label.clone(),
        text_value: tree_header_props.text_value.clone(),
        tree_header_props,
    }
}

pub fn use_heading(props: UseHeadingProps) -> UseHeadingResult {
    let heading_props = HeadingProps {
        label: props.label,
        text_value: props.text_value,
        aria_level: props.level.clamp(1, 6),
    };

    UseHeadingResult {
        label: heading_props.label.clone(),
        text_value: heading_props.text_value.clone(),
        level: heading_props.aria_level,
        heading_props,
    }
}

pub fn use_text_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_text", use_text(props))
}

pub fn use_label_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_label", use_label(props))
}

pub fn use_description_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_description", use_description(props))
}

pub fn use_field_error_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_field_error", use_field_error(props))
}

pub fn use_legend_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_legend", use_legend(props))
}

pub fn use_visually_hidden_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_visually_hidden", use_visually_hidden(props))
}

pub fn use_keyboard_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_keyboard", use_keyboard(props))
}

pub fn use_list_box_header_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_list_box_header", use_list_box_header(props))
}

pub fn use_grid_list_header_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_grid_list_header", use_grid_list_header(props))
}

pub fn use_tree_header_value(props: UseTextProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_tree_header", use_tree_header(props))
}

pub fn use_heading_value(props: UseHeadingProps) -> GuiResult<JsonValue> {
    serialize_text_hook("use_heading", use_heading(props))
}

fn text_props(props: &UseTextProps) -> TextProps {
    TextProps {
        label: props.label.clone(),
        text_value: props.text_value.clone(),
    }
}

fn serialize_text_hook<T: Serialize>(hook_name: &str, result: T) -> GuiResult<JsonValue> {
    serde_json::to_value(result).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic {hook_name} hook did not serialize: {error}"
        ))
    })
}
