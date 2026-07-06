use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiModalProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
    pub on_close: String,
}

pub fn ui_modal(cx: &mut ComponentCx<UiModalProps>) -> RSX {
    cx.use_prop("className", |props: &UiModalProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiModalProps| props.label.clone());
    cx.use_prop("isOpen", |props: &UiModalProps| props.is_open);
    cx.use_prop("onClose", |props: &UiModalProps| props.on_close.clone());

    crate::rsx!(
        <dialog
            key="root"
            data-slot="modal"
            class="fixed inset-0 z-50 grid place-items-center bg-background/80 p-6 text-foreground outline-none"
            className={props.className}
            label={props.label}
            open={props.isOpen}
            onClose={props.onClose}
        >
            <Slot key="content" />
        </dialog>
    )
}
