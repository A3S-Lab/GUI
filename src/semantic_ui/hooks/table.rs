use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableProps {
    label: Option<String>,
}

impl UseTableProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub table_props: TableProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TableSectionKind {
    Header,
    Body,
    Footer,
}

impl Default for TableSectionKind {
    fn default() -> Self {
        Self::Body
    }
}

impl TableSectionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Body => "body",
            Self::Footer => "footer",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableSectionProps {
    kind: TableSectionKind,
    label: Option<String>,
}

impl UseTableSectionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn kind(mut self, kind: TableSectionKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableSectionResult {
    pub section_kind: TableSectionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub table_section_props: TableSectionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSectionProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(rename = "data-table-section")]
    pub data_table_section: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableRowProps {
    is_selected: bool,
    is_disabled: bool,
}

impl UseTableRowProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableRowResult {
    pub is_selected: bool,
    pub is_disabled: bool,
    pub table_row_props: TableRowProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "is_false")]
    pub is_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub selected: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub is_disabled: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableCellProps {
    label: Option<String>,
    text_value: Option<String>,
}

impl UseTableCellProps {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableCellResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub table_cell_props: TableCellProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableColumnProps {
    label: Option<String>,
    text_value: Option<String>,
}

impl UseTableColumnProps {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableColumnResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub table_column_props: TableColumnProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseTableCaptionProps {
    label: Option<String>,
    text_value: Option<String>,
}

impl UseTableCaptionProps {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseTableCaptionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub table_caption_props: TableCaptionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCaptionProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

pub fn use_table(props: UseTableProps) -> UseTableResult {
    UseTableResult {
        label: props.label.clone(),
        table_props: TableProps {
            role: "table",
            label: props.label.clone(),
            aria_label: props.label,
        },
    }
}

pub fn use_table_section(props: UseTableSectionProps) -> UseTableSectionResult {
    UseTableSectionResult {
        section_kind: props.kind,
        label: props.label.clone(),
        table_section_props: TableSectionProps {
            role: "rowgroup",
            label: props.label.clone(),
            aria_label: props.label,
            data_table_section: props.kind.as_str(),
        },
    }
}

pub fn use_table_row(props: UseTableRowProps) -> UseTableRowResult {
    UseTableRowResult {
        is_selected: props.is_selected,
        is_disabled: props.is_disabled,
        table_row_props: TableRowProps {
            role: "row",
            is_selected: props.is_selected,
            selected: props.is_selected,
            aria_selected: props.is_selected,
            data_selected: props.is_selected,
            is_disabled: props.is_disabled,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_table_cell(props: UseTableCellProps) -> UseTableCellResult {
    let label = props.label.or_else(|| props.text_value.clone());

    UseTableCellResult {
        label: label.clone(),
        text_value: props.text_value.clone(),
        table_cell_props: TableCellProps {
            role: "cell",
            label,
            text_value: props.text_value,
        },
    }
}

pub fn use_table_column(props: UseTableColumnProps) -> UseTableColumnResult {
    let label = props.label.or_else(|| props.text_value.clone());

    UseTableColumnResult {
        label: label.clone(),
        text_value: props.text_value.clone(),
        table_column_props: TableColumnProps {
            role: "columnheader",
            label,
            text_value: props.text_value,
        },
    }
}

pub fn use_table_caption(props: UseTableCaptionProps) -> UseTableCaptionResult {
    let label = props.label.or_else(|| props.text_value.clone());

    UseTableCaptionResult {
        label: label.clone(),
        text_value: props.text_value.clone(),
        table_caption_props: TableCaptionProps {
            label,
            text_value: props.text_value,
        },
    }
}

pub fn use_table_value(props: UseTableProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table hook did not serialize: {error}"
        ))
    })
}

pub fn use_table_section_value(props: UseTableSectionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table_section(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table_section hook did not serialize: {error}"
        ))
    })
}

pub fn use_table_row_value(props: UseTableRowProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table_row(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table_row hook did not serialize: {error}"
        ))
    })
}

pub fn use_table_cell_value(props: UseTableCellProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table_cell(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table_cell hook did not serialize: {error}"
        ))
    })
}

pub fn use_table_column_value(props: UseTableColumnProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table_column(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table_column hook did not serialize: {error}"
        ))
    })
}

pub fn use_table_caption_value(props: UseTableCaptionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_table_caption(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_table_caption hook did not serialize: {error}"
        ))
    })
}
