use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{TableSectionKind, UseTableSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarGridHeaderProps {
    pub class_name: String,
}

pub fn ui_calendar_grid_header(cx: &mut ComponentCx<UiCalendarGridHeaderProps>) -> RSX {
    cx.use_table_section(|_props: &UiCalendarGridHeaderProps| {
        UseTableSectionProps::new().kind(TableSectionKind::Header)
    });
    cx.use_prop("className", |props: &UiCalendarGridHeaderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableSection
            key="root"
            data-slot="calendar-grid-header"
            {...props.tableSectionProps}
            data-table-section="header"
            class="[&_tr]:border-b-0"
            className={props.className}
        >
            <Slot key="content" />
        </TableSection>
    )
}
