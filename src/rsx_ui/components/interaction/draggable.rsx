use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDragProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDraggableProps {
    pub class_name: String,
    pub on_drag_start: Option<String>,
    pub on_drag_move: Option<String>,
    pub on_drag_end: Option<String>,
    pub drag_type: Option<String>,
    pub drag_value: Option<String>,
    pub is_disabled: bool,
    pub is_dragging: bool,
}

pub fn ui_draggable(cx: &mut ComponentCx<UiDraggableProps>) -> RSX {
    cx.use_drag(|props: &UiDraggableProps| {
        UseDragProps::new()
            .on_drag_start(props.on_drag_start.clone())
            .on_drag_move(props.on_drag_move.clone())
            .on_drag_end(props.on_drag_end.clone())
            .drag_type(props.drag_type.clone())
            .drag_value(props.drag_value.clone())
            .disabled(props.is_disabled)
            .dragging(props.is_dragging)
    });
    cx.use_prop("className", |props: &UiDraggableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dragProps}
            data-slot="draggable"
            data-dragging={props.isDragging}
            class="outline-none transition-opacity data-[dragging=true]:opacity-70 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
