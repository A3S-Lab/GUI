use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCalendarCellProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarCellProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub on_press: Option<String>,
    pub action_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_unavailable: bool,
    pub is_outside_month: bool,
    pub is_today: bool,
    pub is_pressed: bool,
}

pub fn ui_calendar_cell(cx: &mut ComponentCx<UiCalendarCellProps>) -> RSX {
    cx.use_calendar_cell(|props: &UiCalendarCellProps| {
        UseCalendarCellProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .action_value(Some(props.action_value.clone()))
            .on_press(props.on_press.clone())
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .unavailable(props.is_unavailable)
            .outside_month(props.is_outside_month)
            .today(props.is_today)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiCalendarCellProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.calendarCellProps}
            data-slot="calendar-cell"
            class="inline-flex h-8 w-8 items-center justify-center rounded-md text-sm outline-none hover:bg-surface-strong hover:text-ink focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:bg-ink data-[selected=true]:text-canvas data-[today=true]:border data-[today=true]:border-ink data-[outside-month=true]:text-body data-[unavailable=true]:line-through"
            className={props.className}
        >
            <Slot key="content" />
        </button>
    )
}
