use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_separator_component<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiSeparator",
        ui_separator,
        passthrough_contract()?.default_prop("orientation", "horizontal")?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_landmark_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiMain",
        ui_main,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "main")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiNavigation",
        ui_navigation,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "navigation")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiHeader",
        ui_header,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "header")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFooter",
        ui_footer,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "footer")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSection",
        ui_section,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "section")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiArticle",
        ui_article,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "article")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiAside",
        ui_aside,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "aside")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSearch",
        ui_search,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("landmarkKind", "search")?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_group_component<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiGroup",
        ui_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onHoverStart", "")?
            .default_prop("onHoverEnd", "")?
            .default_prop("onHoverChange", "")?
            .default_prop("onFocus", "")?
            .default_prop("onBlur", "")?
            .default_prop("onFocusChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isInvalid", false)?
            .default_prop("isReadOnly", false)?
            .default_prop("isHovered", false)?
            .default_prop("isFocused", false)?
            .default_prop("isFocusVisible", false)?
            .default_prop("isFocusWithin", false)?
            .default_prop("autoFocus", false)?
            .default_prop("tabIndex", 0_i32)?,
        None,
    )?;
    Ok(component)
}
