use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiModalProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
    pub on_close: String,
}

pub fn ui_modal(cx: &mut ComponentCx<UiModalProps>) -> RSX {
    cx.use_overlay(|props: &UiModalProps| {
        UseOverlayProps::new()
            .open(props.is_open)
            .on_close(Some(props.on_close.clone()))
    });
    cx.use_prop("className", |props: &UiModalProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiModalProps| props.label.clone());

    crate::rsx!(
        <dialog
            key="root"
            {...props.overlayProps}
            data-slot="modal"
            class="fixed inset-0 z-50 grid place-items-center bg-canvas/80 p-6 text-ink outline-none"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </dialog>
    )
}
