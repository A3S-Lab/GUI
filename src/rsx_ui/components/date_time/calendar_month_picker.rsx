use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{UseFieldProps, UseSelectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarMonthPickerProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_calendar_month_picker(cx: &mut ComponentCx<UiCalendarMonthPickerProps>) -> RSX {
    cx.use_field(|props: &UiCalendarMonthPickerProps| {
        UseFieldProps::new()
            .label(Some(props.label.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_selection(|props: &UiCalendarMonthPickerProps| {
        UseSelectionProps::new()
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiCalendarMonthPickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Select
            key="root"
            {...props.fieldProps}
            {...props.selectionProps}
            data-slot="calendar-month-picker"
            class="min-w-28"
            className={props.className}
        >
            <Slot key="content" />
        </Select>
    )
}
