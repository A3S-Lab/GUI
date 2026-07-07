use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseOverlayProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDialogProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
    pub on_close: String,
}

pub fn ui_dialog(cx: &mut ComponentCx<UiDialogProps>) -> RSX {
    cx.use_overlay(|props: &UiDialogProps| {
        UseOverlayProps::new()
            .open(props.is_open)
            .on_close(Some(props.on_close.clone()))
    });
    cx.use_prop("className", |props: &UiDialogProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDialogProps| props.label.clone());

    crate::rsx!(
        <dialog
            key="root"
            {...props.overlayProps}
            data-slot="dialog"
            class="fixed left-1/2 top-1/2 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-lg border border-hairline-strong bg-canvas p-6 text-ink outline-none"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </dialog>
    )
}
