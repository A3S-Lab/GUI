use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableCellProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_cell(cx: &mut ComponentCx<UiTableCellProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableCellProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTableCellProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiTableCellProps| {
        props.text_value.clone()
    });

    crate::rsx!(
        <TableCell
            key="root"
            data-slot="table-cell"
            class="p-2 align-middle text-sm"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
        >
            <Slot key="content" />
        </TableCell>
    )
}
