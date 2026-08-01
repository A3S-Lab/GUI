use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDropProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDroppableProps {
    pub class_name: String,
    pub label: Option<String>,
    pub on_drop: Option<String>,
    pub on_drop_enter: Option<String>,
    pub on_drop_exit: Option<String>,
    pub on_drop_move: Option<String>,
    pub on_drop_activate: Option<String>,
    pub accepted_drag_types: Option<String>,
    pub drop_operation: Option<String>,
    pub get_drop_operation: Option<String>,
    pub is_disabled: bool,
    pub is_drop_target: bool,
}

pub fn ui_droppable(cx: &mut ComponentCx<UiDroppableProps>) -> RSX {
    cx.use_drop(|props: &UiDroppableProps| {
        UseDropProps::new()
            .label(props.label.clone())
            .on_drop(props.on_drop.clone())
            .on_drop_enter(props.on_drop_enter.clone())
            .on_drop_exit(props.on_drop_exit.clone())
            .on_drop_move(props.on_drop_move.clone())
            .on_drop_activate(props.on_drop_activate.clone())
            .accepted_drag_types(props.accepted_drag_types.clone())
            .drop_operation(props.drop_operation.clone())
            .get_drop_operation(props.get_drop_operation.clone())
            .disabled(props.is_disabled)
            .drop_target(props.is_drop_target)
    });
    cx.use_prop("className", |props: &UiDroppableProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dropProps}
            {...props.dropButtonProps}
            data-slot="droppable"
            data-drop-target={props.isDropTarget}
            data-get-drop-operation-policy={props.getDropOperation}
            class="outline-none data-[drop-target=true]:ring-[2px] data-[drop-target=true]:ring-ink/40 disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
