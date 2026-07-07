use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarGridProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_calendar_grid(cx: &mut ComponentCx<UiCalendarGridProps>) -> RSX {
    cx.use_table(|props: &UiCalendarGridProps| {
        UseTableProps::new().label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiCalendarGridProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Table
            key="root"
            {...props.tableProps}
            data-slot="calendar-grid"
            class="w-full border-collapse space-y-1 text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </Table>
    )
}
