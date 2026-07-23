use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiModalOverlayProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
}

pub fn ui_modal_overlay(cx: &mut ComponentCx<UiModalOverlayProps>) -> RSX {
    cx.use_overlay(|props: &UiModalOverlayProps| {
        UseOverlayProps::new().open(props.is_open).managed(false)
    });
    cx.use_prop("className", |props: &UiModalOverlayProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiModalOverlayProps| props.label.clone());

    crate::rsx!(
        <Group
            key="root"
            {...props.overlayProps}
            data-slot="modal-overlay"
            class="fixed inset-0 z-50 grid place-items-center bg-surface-card/80 p-3 text-ink"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Group>
    )
}
