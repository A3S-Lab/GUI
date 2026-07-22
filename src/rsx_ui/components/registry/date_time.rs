use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::{passthrough_contract, selection_contract};
use super::with_builtin_template;

pub(super) fn with_date_time_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiDatePicker",
        ui_date_picker,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("onOpenChange", "")?
            .default_prop("isOpen", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDateRangePicker",
        ui_date_range_picker,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("startValue", "")?
            .default_prop("endValue", "")?
            .default_prop("placeholder", "")?
            .default_prop("onStartChange", "")?
            .default_prop("onEndChange", "")?
            .default_prop("onOpenChange", "")?
            .default_prop("isOpen", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendar",
        ui_calendar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRangeCalendar",
        ui_range_calendar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("startValue", "")?
            .default_prop("endValue", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarHeading",
        ui_calendar_heading,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("level", 2_u32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarGrid",
        ui_calendar_grid,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarGridHeader",
        ui_calendar_grid_header,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarGridBody",
        ui_calendar_grid_body,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarHeaderCell",
        ui_calendar_header_cell,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarCell",
        ui_calendar_cell,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("actionValue", "")?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?
            .default_prop("isUnavailable", false)?
            .default_prop("isOutsideMonth", false)?
            .default_prop("isToday", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarMonthPicker",
        ui_calendar_month_picker,
        selection_contract()?.default_prop("label", "Month")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCalendarYearPicker",
        ui_calendar_year_picker,
        selection_contract()?.default_prop("label", "Year")?,
        None,
    )?;
    Ok(component)
}
