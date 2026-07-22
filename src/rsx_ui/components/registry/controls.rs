use crate::rsx_ui::variants::ui_button_variants;
use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_input_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiButton",
        ui_button,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("onPressUp", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        Some(ui_button_variants()?),
    )?;
    let component = with_builtin_template(
        component,
        "UiInput",
        ui_input,
        passthrough_contract()?
            .default_prop("type", "text")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTextField",
        ui_text_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSearchField",
        ui_search_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("inputType", "search")?
            .default_prop("value", "")?
            .default_prop("placeholder", "Search")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiNumberField",
        ui_number_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("placeholder", "")?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?
            .default_prop("stepValue", 1.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDateField",
        ui_date_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("granularity", "day")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDateInput",
        ui_date_input,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDateSegment",
        ui_date_segment,
        passthrough_contract()?
            .default_prop("segmentType", "")?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("placeholder", "")?
            .default_prop("isPlaceholder", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTimeField",
        ui_time_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("granularity", "minute")?
            .default_prop("hourCycle", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_form_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiForm",
        ui_form,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onSubmit", "")?
            .default_prop("onReset", "")?
            .default_prop("onInvalid", "")?
            .default_prop("validationBehavior", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isInvalid", false)?
            .default_prop("noValidate", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFieldError",
        ui_field_error,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFieldSet",
        ui_field_set,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isRequired", false)?
            .default_prop("isReadOnly", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLegend",
        ui_legend,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCheckbox",
        ui_checkbox,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("onChange", "")?
            .default_prop("isChecked", false)?
            .default_prop("isDisabled", false)?
            .default_prop("isRequired", false)?
            .default_prop("isInvalid", false)?
            .default_prop("isReadOnly", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCheckboxGroup",
        ui_checkbox_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("onChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isRequired", false)?
            .default_prop("isInvalid", false)?
            .default_prop("isReadOnly", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSwitch",
        ui_switch,
        passthrough_contract()?
            .default_prop("onChange", "")?
            .default_prop("isChecked", false)?
            .default_prop("isDisabled", false)?
            .default_prop("isRequired", false)?
            .default_prop("isInvalid", false)?
            .default_prop("isReadOnly", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRadioGroup",
        ui_radio_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("defaultValue", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isRequired", false)?
            .default_prop("isInvalid", false)?
            .default_prop("isReadOnly", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRadio",
        ui_radio,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    Ok(component)
}
