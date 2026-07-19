use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableRowProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableRowProps {
    pub class_name: String,
    pub is_selected: bool,
}

pub fn ui_table_row(cx: &mut ComponentCx<UiTableRowProps>) -> RSX {
    cx.use_table_row(|props: &UiTableRowProps| UseTableRowProps::new().selected(props.is_selected));
    cx.use_prop("className", |props: &UiTableRowProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableRow
            key="root"
            {...props.tableRowProps}
            data-slot="table-row"
            class="border-b border-hairline hover:bg-surface-strong/50 data-[selected=true]:bg-surface-strong"
            className={props.className}
        >
            <Slot key="content" />
        </TableRow>
    )
}
