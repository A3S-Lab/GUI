use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{TableSectionKind, UseTableSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarGridBodyProps {
    pub class_name: String,
}

pub fn ui_calendar_grid_body(cx: &mut ComponentCx<UiCalendarGridBodyProps>) -> RSX {
    cx.use_table_section(|_props: &UiCalendarGridBodyProps| {
        UseTableSectionProps::new().kind(TableSectionKind::Body)
    });
    cx.use_prop("className", |props: &UiCalendarGridBodyProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableSection
            key="root"
            data-slot="calendar-grid-body"
            {...props.tableSectionProps}
            data-table-section="body"
            class="[&_tr:last-child]:border-0"
            className={props.className}
        >
            <Slot key="content" />
        </TableSection>
    )
}
