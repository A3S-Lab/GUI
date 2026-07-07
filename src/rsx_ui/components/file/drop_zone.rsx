use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDropZoneProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDropZoneProps {
    pub class_name: String,
    pub label: String,
    pub on_drop: String,
    pub on_drag_enter: String,
    pub on_drag_leave: String,
    pub is_disabled: bool,
}

pub fn ui_drop_zone(cx: &mut ComponentCx<UiDropZoneProps>) -> RSX {
    cx.use_drop_zone(|props: &UiDropZoneProps| {
        UseDropZoneProps::new()
            .label(Some(props.label.clone()))
            .on_drop(Some(props.on_drop.clone()))
            .on_drag_enter(Some(props.on_drag_enter.clone()))
            .on_drag_leave(Some(props.on_drag_leave.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiDropZoneProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dropZoneProps}
            data-slot="drop-zone"
            class="grid min-h-24 place-items-center rounded-lg border border-dashed border-hairline-strong bg-canvas-soft p-6 text-sm text-body transition-colors"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
