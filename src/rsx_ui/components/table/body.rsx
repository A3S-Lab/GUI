use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{TableSectionKind, UseTableSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableBodyProps {
    pub class_name: String,
}

pub fn ui_table_body(cx: &mut ComponentCx<UiTableBodyProps>) -> RSX {
    cx.use_table_section(|_props: &UiTableBodyProps| {
        UseTableSectionProps::new().kind(TableSectionKind::Body)
    });
    cx.use_prop("className", |props: &UiTableBodyProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableSection
            key="root"
            data-slot="table-body"
            {...props.tableSectionProps}
            data-table-section="body"
            class="[&_tr:last-child]:border-0"
            className={props.className}
        >
            <Slot key="content" />
        </TableSection>
    )
}
