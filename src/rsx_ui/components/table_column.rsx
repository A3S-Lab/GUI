use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableColumnProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_table_column(cx: &mut ComponentCx<UiTableColumnProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableColumnProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTableColumnProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiTableColumnProps| {
        props.text_value.clone()
    });

    crate::rsx!(
        <TableColumn
            key="root"
            data-slot="table-column"
            class="h-10 px-2 text-left align-middle text-sm font-medium text-muted-foreground"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
        >
            <Slot key="content" />
        </TableColumn>
    )
}
