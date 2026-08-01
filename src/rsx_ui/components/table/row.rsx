use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableRowProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableRowProps {
    pub class_name: String,
    pub id: String,
    pub is_selected: bool,
    pub is_draggable: bool,
    pub drag_type: String,
    pub drag_value: String,
    pub drag_items: String,
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
            data-collection-drop-item="true"
            data-collection-key={props.id}
            id={props.id}
            draggable={props.isDraggable}
            dragType={props.dragType}
            dragValue={props.dragValue}
            dragItems={props.dragItems}
            class="border-b border-hairline hover:bg-surface-strong/50 data-[selected=true]:bg-surface-strong"
            className={props.className}
        >
            <Slot key="content" />
        </TableRow>
    )
}
