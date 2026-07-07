use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableColumnProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableColumnProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_column(cx: &mut ComponentCx<UiTableColumnProps>) -> RSX {
    cx.use_table_column(|props: &UiTableColumnProps| {
        UseTableColumnProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiTableColumnProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TableColumn
            key="root"
            {...props.tableColumnProps}
            data-slot="table-column"
            class="h-10 px-2 text-left align-middle text-sm font-medium text-body"
            className={props.className}
        >
            <Slot key="content" />
        </TableColumn>
    )
}
