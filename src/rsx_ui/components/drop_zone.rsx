use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiDropZoneProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDropZoneProps| props.label.clone());
    cx.use_prop("onDrop", |props: &UiDropZoneProps| props.on_drop.clone());
    cx.use_prop("onDragEnter", |props: &UiDropZoneProps| {
        props.on_drag_enter.clone()
    });
    cx.use_prop("onDragLeave", |props: &UiDropZoneProps| {
        props.on_drag_leave.clone()
    });
    cx.use_prop("isDisabled", |props: &UiDropZoneProps| props.is_disabled);

    crate::rsx!(
        <Group
            key="root"
            data-slot="drop-zone"
            class="grid min-h-24 place-items-center rounded-md border border-dashed border-border bg-muted/30 p-6 text-sm text-muted-foreground transition-colors"
            className={props.className}
            label={props.label}
            disabled={props.isDisabled}
            onDrop={props.onDrop}
            onDragEnter={props.onDragEnter}
            onDragLeave={props.onDragLeave}
        >
            <Slot key="content" />
        </Group>
    )
}
