use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_table(cx: &mut ComponentCx<UiTableProps>) -> RSX {
    cx.use_table(|props: &UiTableProps| UseTableProps::new().label(Some(props.label.clone())));
    cx.use_prop("className", |props: &UiTableProps| props.class_name.clone());

    crate::rsx!(
        <Table
            key="root"
            {...props.tableProps}
            data-slot="table"
            class="w-full caption-bottom text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </Table>
    )
}
