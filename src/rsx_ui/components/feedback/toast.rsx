use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseToastProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToastProps {
    pub class_name: String,
    pub title: String,
    pub description: String,
    pub on_close: String,
}

pub fn ui_toast(cx: &mut ComponentCx<UiToastProps>) -> RSX {
    cx.use_toast(|props: &UiToastProps| {
        UseToastProps::new()
            .title(Some(props.title.clone()))
            .description(Some(props.description.clone()))
            .on_close(Some(props.on_close.clone()))
    });
    cx.use_prop("className", |props: &UiToastProps| props.class_name.clone());

    crate::rsx!(
        <Group
            key="root"
            {...props.toastProps}
            data-slot="toast"
            class="grid gap-1 rounded-md border border-hairline bg-canvas p-3 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
