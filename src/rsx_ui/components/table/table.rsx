use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTableProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTableProps {
    pub class_name: String,
    pub label: String,
    pub on_drag_start: String,
    pub on_drag_move: String,
    pub on_drag_end: String,
    pub on_drop: String,
    pub on_drop_enter: String,
    pub on_drop_move: String,
    pub on_drop_exit: String,
    pub on_root_drop: String,
    pub on_item_drop: String,
    pub on_insert: String,
    pub allowed_drop_operations: String,
    pub accepted_drag_types: String,
    pub drop_operation: String,
    pub drop_orientation: String,
}

pub fn ui_table(cx: &mut ComponentCx<UiTableProps>) -> RSX {
    cx.use_table(|props: &UiTableProps| UseTableProps::new().label(Some(props.label.clone())));
    cx.use_prop("className", |props: &UiTableProps| props.class_name.clone());

    crate::rsx!(
        <Table
            key="root"
            {...props.tableProps}
            data-slot="table"
            data-draggable-collection="true"
            data-droppable-collection="true"
            data-drop-orientation={props.dropOrientation}
            allowedDropOperations={props.allowedDropOperations}
            acceptedDragTypes={props.acceptedDragTypes}
            dropOperation={props.dropOperation}
            onDragStart={props.onDragStart}
            onDragMove={props.onDragMove}
            onDragEnd={props.onDragEnd}
            onDrop={props.onDrop}
            onDropEnter={props.onDropEnter}
            onDropMove={props.onDropMove}
            onDropExit={props.onDropExit}
            onRootDrop={props.onRootDrop}
            onItemDrop={props.onItemDrop}
            onInsert={props.onInsert}
            class="w-full caption-bottom text-sm"
            className={props.className}
        >
            <Slot key="content" />
        </Table>
    )
}
