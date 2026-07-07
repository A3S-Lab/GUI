use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseMoveProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMovableProps {
    pub class_name: String,
    pub on_move_start: Option<String>,
    pub on_move: Option<String>,
    pub on_move_end: Option<String>,
    pub is_disabled: bool,
    pub is_moving: bool,
    pub x_delta: f64,
    pub y_delta: f64,
}

pub fn ui_movable(cx: &mut ComponentCx<UiMovableProps>) -> RSX {
    cx.use_move(|props: &UiMovableProps| {
        UseMoveProps::new()
            .on_move_start(props.on_move_start.clone())
            .on_move(props.on_move.clone())
            .on_move_end(props.on_move_end.clone())
            .disabled(props.is_disabled)
            .moving(props.is_moving)
            .x_delta(props.x_delta)
            .y_delta(props.y_delta)
    });
    cx.use_prop("className", |props: &UiMovableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.moveProps}
            data-slot="movable"
            data-moving={props.isMoving}
            data-x-delta={props.xDelta}
            data-y-delta={props.yDelta}
            class="outline-none touch-none select-none data-[moving=true]:cursor-grabbing disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
