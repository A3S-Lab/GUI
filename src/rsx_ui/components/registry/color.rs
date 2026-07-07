use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_color_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiColorPicker",
        ui_color_picker,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorArea",
        ui_color_area,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("xChannel", "saturation")?
            .default_prop("yChannel", "brightness")?
            .default_prop("xValue", 0.0)?
            .default_prop("yValue", 0.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorThumb",
        ui_color_thumb,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("xValue", 0.0)?
            .default_prop("yValue", 0.0)?
            .default_prop("onPress", "")?
            .default_prop("isDragging", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorField",
        ui_color_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "#000000")?
            .default_prop("colorSpace", "srgb")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorSlider",
        ui_color_slider,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("channel", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 360.0)?
            .default_prop("stepValue", 1.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorWheel",
        ui_color_wheel,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorWheelTrack",
        ui_color_wheel_track,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorSwatch",
        ui_color_swatch,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorSwatchPicker",
        ui_color_swatch_picker,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiColorSwatchPickerItem",
        ui_color_swatch_picker_item,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    Ok(component)
}
