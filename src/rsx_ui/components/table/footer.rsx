use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{TableSectionKind, UseTableSectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableFooterProps {
    pub class_name: String,
}

pub fn ui_table_footer(cx: &mut ComponentCx<UiTableFooterProps>) -> RSX {
    cx.use_table_section(|_props: &UiTableFooterProps| {
        UseTableSectionProps::new().kind(TableSectionKind::Footer)
    });
    cx.use_prop("className", |props: &UiTableFooterProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableSection
            key="root"
            data-slot="table-footer"
            {...props.tableSectionProps}
            data-table-section="footer"
            class="border-t border-hairline bg-surface-strong/50 font-medium [&>tr]:last:border-b-0"
            className={props.className}
        >
            <Slot key="content" />
        </TableSection>
    )
}
