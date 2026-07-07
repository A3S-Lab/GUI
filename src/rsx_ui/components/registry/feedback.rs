use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_feedback_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiToastRegion",
        ui_toast_region,
        passthrough_contract()?.default_prop("label", "Notifications")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToast",
        ui_toast,
        passthrough_contract()?
            .default_prop("title", "")?
            .default_prop("description", "")?
            .default_prop("onClose", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiVirtualizer",
        ui_virtualizer,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("layout", "list")?
            .default_prop("orientation", "vertical")?
            .default_prop("itemCount", 0)?
            .default_prop("estimatedItemSize", 40)?
            .default_prop("visibleStart", 0)?
            .default_prop("visibleEnd", 0)?
            .default_prop("overscan", 2)?
            .default_prop("gap", 0)?
            .default_prop("padding", 0)?
            .default_prop("isScrolling", false)?
            .default_prop("isDisabled", false)?
            .default_prop("tabIndex", 0)?,
        None,
    )?;
    Ok(component)
}
