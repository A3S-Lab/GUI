use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_command_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiToolbar",
        ui_toolbar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("orientation", "horizontal")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToggleButton",
        ui_toggle_button,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("isSelected", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToggleButtonGroup",
        ui_toggle_button_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("orientation", "horizontal")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiKeyboard",
        ui_keyboard,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSelectionIndicator",
        ui_selection_indicator,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    Ok(component)
}
