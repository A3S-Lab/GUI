use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseHeadingProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarHeadingProps {
    pub class_name: String,
    pub label: String,
    pub level: u32,
}

pub fn ui_calendar_heading(cx: &mut ComponentCx<UiCalendarHeadingProps>) -> RSX {
    cx.use_heading(|props: &UiCalendarHeadingProps| {
        UseHeadingProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.label.clone()))
            .level(props.level)
    });
    cx.use_prop("className", |props: &UiCalendarHeadingProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Heading
            key="root"
            {...props.headingProps}
            data-slot="calendar-heading"
            class="text-sm font-medium text-ink"
            className={props.className}
            label={props.label}
            aria-level={props.level}
        >
            <Slot key="content" />
        </Heading>
    )
}
