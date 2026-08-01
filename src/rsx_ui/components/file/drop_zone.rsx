use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDropZoneProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDropZoneProps {
    pub class_name: String,
    pub label: String,
    pub on_drop: String,
    pub on_drag_enter: String,
    pub on_drag_leave: String,
    pub on_drop_enter: Option<String>,
    pub on_drop_move: Option<String>,
    pub on_drop_exit: Option<String>,
    pub accepted_drag_types: Option<String>,
    pub drop_operation: Option<String>,
    pub is_disabled: bool,
    pub is_drop_target: bool,
}

pub fn ui_drop_zone(cx: &mut ComponentCx<UiDropZoneProps>) -> RSX {
    cx.use_drop_zone(|props: &UiDropZoneProps| {
        UseDropZoneProps::new()
            .label(Some(props.label.clone()))
            .on_drop(Some(props.on_drop.clone()))
            .on_drag_enter(Some(props.on_drag_enter.clone()))
            .on_drag_leave(Some(props.on_drag_leave.clone()))
            .on_drop_enter(props.on_drop_enter.clone())
            .on_drop_move(props.on_drop_move.clone())
            .on_drop_exit(props.on_drop_exit.clone())
            .accepted_drag_types(props.accepted_drag_types.clone())
            .drop_operation(props.drop_operation.clone())
            .disabled(props.is_disabled)
            .drop_target(props.is_drop_target)
    });
    cx.use_prop("className", |props: &UiDropZoneProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dropZoneProps}
            data-slot="drop-zone"
            data-drop-target={props.isDropTarget}
            class="grid min-h-24 place-items-center rounded-md border border-dashed border-hairline-strong bg-canvas-soft p-3 text-sm text-body outline-none data-[drop-target=true]:ring-[2px] data-[drop-target=true]:ring-ink/40"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
