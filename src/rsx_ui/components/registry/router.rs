use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_router_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiRouter",
        ui_router,
        passthrough_contract()?
            .default_prop("currentPath", "")?
            .default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRoutes",
        ui_routes,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRoute",
        ui_route,
        passthrough_contract()?
            .default_prop("path", "")?
            .default_prop("label", "")?
            .default_prop("isActive", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiNavLink",
        ui_nav_link,
        passthrough_contract()?
            .default_prop("to", "")?
            .default_prop("onNavigate", "")?
            .default_prop("isActive", false)?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiNavigateButton",
        ui_navigate_button,
        passthrough_contract()?
            .default_prop("to", "")?
            .default_prop("onNavigate", "")?
            .default_prop("isActive", false)?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    Ok(component)
}
