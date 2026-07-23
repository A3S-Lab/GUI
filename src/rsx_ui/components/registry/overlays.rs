use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_dialog_disclosure_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiDialog",
        ui_dialog,
        passthrough_contract()?
            .default_prop("overlayType", "dialog")?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?
            .default_prop("onClose", "")?
            .default_prop("isDismissable", false)?
            .default_prop("isKeyboardDismissDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiModal",
        ui_modal,
        passthrough_contract()?
            .default_prop("overlayType", "modal")?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?
            .default_prop("onClose", "")?
            .default_prop("isDismissable", false)?
            .default_prop("isKeyboardDismissDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiModalOverlay",
        ui_modal_overlay,
        passthrough_contract()?
            .default_prop("overlayType", "underlay")?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosure",
        ui_disclosure,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isExpanded", false)?
            .default_prop("onExpandedChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosureGroup",
        ui_disclosure_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("expandedKeys", "")?
            .default_prop("allowsMultipleExpanded", false)?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosureSummary",
        ui_disclosure_summary,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("isExpanded", false)?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_popover_tooltip_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiPopover",
        ui_popover,
        passthrough_contract()?
            .default_prop("overlayType", "popover")?
            .default_prop("isOpen", false)?
            .default_prop("onClose", "")?
            .default_prop("isNonModal", false)?
            .default_prop("isKeyboardDismissDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiOverlayArrow",
        ui_overlay_arrow,
        passthrough_contract()?.default_prop("placement", "top")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTooltip",
        ui_tooltip,
        passthrough_contract()?
            .default_prop("overlayType", "tooltip")?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTooltipTrigger",
        ui_tooltip_trigger,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isOpen", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_overlay_trigger_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiDialogTrigger",
        ui_dialog_trigger,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isOpen", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosurePanel",
        ui_disclosure_panel,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isExpanded", false)?,
        None,
    )?;
    Ok(component)
}
