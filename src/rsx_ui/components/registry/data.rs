use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::{passthrough_contract, selection_contract};
use super::with_builtin_template;

pub(super) fn with_data_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiTable",
        ui_table,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableHeader",
        ui_table_header,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableBody",
        ui_table_body,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableFooter",
        ui_table_footer,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiResizableTableContainer",
        ui_resizable_table_container,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableRow",
        ui_table_row,
        passthrough_contract()?.default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRow",
        ui_table_row,
        passthrough_contract()?.default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableColumn",
        ui_table_column,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColumn",
        ui_table_column,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColumnResizer",
        ui_column_resizer,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isResizing", false)?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 0.0)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableCell",
        ui_table_cell,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCell",
        ui_table_cell,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableLoadMoreItem",
        ui_table_load_more_item,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onPress", "")?
            .default_prop("isLoading", false)?
            .default_prop("isDisabled", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableCaption",
        ui_table_caption,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabs",
        ui_tabs,
        selection_contract()?.default_prop("keyboardActivation", "automatic")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabsList",
        ui_tabs_list,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("orientation", "horizontal")?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabList",
        ui_tabs_list,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("orientation", "horizontal")?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabPanels",
        ui_tab_panels,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabsTrigger",
        ui_tabs_trigger,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTab",
        ui_tabs_trigger,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabsContent",
        ui_tabs_content,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabPanel",
        ui_tabs_content,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )?;
    Ok(component)
}
