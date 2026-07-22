use crate::rsx_ui::variants::ui_badge_variants;
use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_foundation_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiLabel",
        ui_label,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiText",
        ui_text,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDescription",
        ui_description,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiHeading",
        ui_heading,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?
            .default_prop("level", 2_u32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLink",
        ui_link,
        passthrough_contract()?
            .default_prop("href", "")?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiPressable",
        ui_pressable,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiHoverable",
        ui_hoverable,
        passthrough_contract()?
            .default_prop("onHoverStart", "")?
            .default_prop("onHoverEnd", "")?
            .default_prop("onHoverChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isHovered", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiKeyboardTarget",
        ui_keyboard_target,
        passthrough_contract()?
            .default_prop("onKeyDown", "")?
            .default_prop("onKeyUp", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isKeyboardActive", false)?
            .default_prop("tabIndex", 0_i32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiClipboardTarget",
        ui_clipboard_target,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onCopy", "")?
            .default_prop("onCut", "")?
            .default_prop("onPaste", "")?
            .default_prop("copyValue", "")?
            .default_prop("copyMimeType", "text/plain")?
            .default_prop("acceptedMimeTypes", "")?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLongPressable",
        ui_long_pressable,
        passthrough_contract()?
            .default_prop("onLongPressStart", "")?
            .default_prop("onLongPressEnd", "")?
            .default_prop("onLongPress", "")?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?
            .default_prop("accessibilityDescription", "")?
            .default_prop("threshold", 500_u64)?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("isLongPressed", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMovable",
        ui_movable,
        passthrough_contract()?
            .default_prop("onMoveStart", "")?
            .default_prop("onMove", "")?
            .default_prop("onMoveEnd", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isMoving", false)?
            .default_prop("xDelta", 0.0)?
            .default_prop("yDelta", 0.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFocusable",
        ui_focusable,
        passthrough_contract()?
            .default_prop("onFocus", "")?
            .default_prop("onBlur", "")?
            .default_prop("onFocusChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isFocused", false)?
            .default_prop("autoFocus", false)?
            .default_prop("tabIndex", 0_i32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFocusRing",
        ui_focus_ring,
        passthrough_contract()?
            .default_prop("onFocus", "")?
            .default_prop("onBlur", "")?
            .default_prop("onFocusChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isFocused", false)?
            .default_prop("isFocusVisible", false)?
            .default_prop("isFocusWithin", false)?
            .default_prop("autoFocus", false)?
            .default_prop("tabIndex", 0_i32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFocusScope",
        ui_focus_scope,
        passthrough_contract()?
            .default_prop("contain", false)?
            .default_prop("restoreFocus", false)?
            .default_prop("autoFocus", false)?
            .default_prop("isDisabled", false)?
            .default_prop("tabIndex", -1_i32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiI18nProvider",
        ui_i18n_provider,
        passthrough_contract()?
            .default_prop("locale", "")?
            .default_prop("direction", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiVisuallyHidden",
        ui_visually_hidden,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSharedElement",
        ui_shared_element,
        passthrough_contract()?.default_prop("id", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSharedElementTransition",
        ui_shared_element_transition,
        passthrough_contract()?
            .default_prop("id", "")?
            .default_prop("isTransitioning", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBreadcrumbs",
        ui_breadcrumbs,
        passthrough_contract()?.default_prop("label", "Breadcrumbs")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBreadcrumb",
        ui_breadcrumb,
        passthrough_contract()?
            .default_prop("href", "")?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTextarea",
        ui_textarea,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("rows", "")?
            .default_prop("cols", "")?
            .default_prop("maxLength", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTextArea",
        ui_textarea,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("rows", "")?
            .default_prop("cols", "")?
            .default_prop("maxLength", "")?,
        None,
    )?;
    let component =
        with_builtin_template(component, "UiCard", ui_card, passthrough_contract()?, None)?;
    let component = with_builtin_template(
        component,
        "UiCardHeader",
        ui_card_header,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardTitle",
        ui_card_title,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardDescription",
        ui_card_description,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardContent",
        ui_card_content,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardFooter",
        ui_card_footer,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBadge",
        ui_badge,
        passthrough_contract()?,
        Some(ui_badge_variants()?),
    )?;
    Ok(component)
}
