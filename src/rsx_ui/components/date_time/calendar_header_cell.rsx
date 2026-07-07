use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableColumnProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarHeaderCellProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_calendar_header_cell(cx: &mut ComponentCx<UiCalendarHeaderCellProps>) -> RSX {
    cx.use_table_column(|props: &UiCalendarHeaderCellProps| {
        UseTableColumnProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiCalendarHeaderCellProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableColumn
            key="root"
            {...props.tableColumnProps}
            data-slot="calendar-header-cell"
            class="h-8 w-8 rounded-md text-center text-xs font-normal text-body"
            className={props.className}
        >
            <Slot key="content" />
        </TableColumn>
    )
}
