use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCalendarProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_calendar(cx: &mut ComponentCx<UiCalendarProps>) -> RSX {
    cx.use_calendar(|props: &UiCalendarProps| {
        UseCalendarProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiCalendarProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.calendarProps}
            data-slot="calendar"
            class="grid gap-3 rounded-md border border-hairline bg-canvas p-3 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
