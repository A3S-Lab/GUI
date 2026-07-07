use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableCellProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableCellProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_cell(cx: &mut ComponentCx<UiTableCellProps>) -> RSX {
    cx.use_table_cell(|props: &UiTableCellProps| {
        UseTableCellProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiTableCellProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableCell
            key="root"
            {...props.tableCellProps}
            data-slot="table-cell"
            class="p-2 align-middle text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </TableCell>
    )
}
