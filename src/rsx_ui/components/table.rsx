use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_table(cx: &mut ComponentCx<UiTableProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiTableProps| props.label.clone());

    crate::rsx!(
        <Table
            key="root"
            data-slot="table"
            class="w-full caption-bottom text-sm"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Table>
    )
}
