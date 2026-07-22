use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_range_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiSlider",
        ui_slider,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?
            .default_prop("stepValue", 1.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSliderTrack",
        ui_slider_track,
        passthrough_contract()?.default_prop("orientation", "horizontal")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSliderFill",
        ui_slider_fill,
        passthrough_contract()?
            .default_prop("orientation", "horizontal")?
            .default_prop("valueNumber", 0.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSliderThumb",
        ui_slider_thumb,
        passthrough_contract()?
            .default_prop("valueNumber", 0.0)?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isDragging", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSliderOutput",
        ui_slider_output,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("valueNumber", 0.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiProgressBar",
        ui_progress_bar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMeter",
        ui_meter,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?,
        None,
    )?;
    Ok(component)
}
