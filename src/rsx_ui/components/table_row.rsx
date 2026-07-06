use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableRowProps {
    pub class_name: String,
    pub is_selected: bool,
}

pub fn ui_table_row(cx: &mut ComponentCx<UiTableRowProps>) -> RSX {
    cx.use_prop("className", |props: &UiTableRowProps| {
        props.class_name.clone()
    });
    cx.use_prop("isSelected", |props: &UiTableRowProps| props.is_selected);

    crate::rsx!(
        <TableRow
            key="root"
            data-slot="table-row"
            data-selected={props.isSelected}
            class="border-b border-border transition-colors hover:bg-muted/50 data-[selected=true]:bg-muted"
            className={props.className}
            selected={props.isSelected}
        >
            <Slot key="content" />
        </TableRow>
    )
}
