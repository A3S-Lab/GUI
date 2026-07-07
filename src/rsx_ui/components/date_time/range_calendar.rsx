use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRangeCalendarProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRangeCalendarProps {
    pub class_name: String,
    pub label: String,
    pub start_value: String,
    pub end_value: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_range_calendar(cx: &mut ComponentCx<UiRangeCalendarProps>) -> RSX {
    cx.use_range_calendar(|props: &UiRangeCalendarProps| {
        UseRangeCalendarProps::new()
            .label(Some(props.label.clone()))
            .start_value(Some(props.start_value.clone()))
            .end_value(Some(props.end_value.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiRangeCalendarProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.rangeCalendarProps}
            data-slot="range-calendar"
            class="grid gap-3 rounded-lg border border-hairline-strong bg-canvas p-4 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
