use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToastProps {
    pub class_name: String,
    pub title: String,
    pub description: String,
    pub on_close: String,
}

pub fn ui_toast(cx: &mut ComponentCx<UiToastProps>) -> RSX {
    cx.use_prop("className", |props: &UiToastProps| props.class_name.clone());
    cx.use_prop("title", |props: &UiToastProps| props.title.clone());
    cx.use_prop("description", |props: &UiToastProps| {
        props.description.clone()
    });
    cx.use_prop("onClose", |props: &UiToastProps| props.on_close.clone());

    crate::rsx!(
        <Group
            key="root"
            data-slot="toast"
            class="grid gap-1 rounded-md border border-border bg-background p-4 text-foreground shadow-lg"
            className={props.className}
            label={props.title}
            title={props.title}
            data-description={props.description}
            onClose={props.onClose}
        >
            <Slot key="content" />
        </Group>
    )
}
