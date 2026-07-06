use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDialogProps {
    pub class_name: String,
    pub label: String,
    pub is_open: bool,
    pub on_close: String,
}

pub fn ui_dialog(cx: &mut ComponentCx<UiDialogProps>) -> RSX {
    cx.use_prop("className", |props: &UiDialogProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDialogProps| props.label.clone());
    cx.use_prop("isOpen", |props: &UiDialogProps| props.is_open);
    cx.use_prop("onClose", |props: &UiDialogProps| props.on_close.clone());

    crate::rsx!(
        <dialog
            key="root"
            data-slot="dialog"
            class="fixed left-1/2 top-1/2 grid w-full max-w-lg -translate-x-1/2 -translate-y-1/2 gap-4 rounded-lg border border-border bg-background p-6 text-foreground shadow-lg outline-none"
            className={props.className}
            label={props.label}
            open={props.isOpen}
            onClose={props.onClose}
        >
            <Slot key="content" />
        </dialog>
    )
}
