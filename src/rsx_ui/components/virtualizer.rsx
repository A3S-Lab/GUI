use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiVirtualizerProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_virtualizer(cx: &mut ComponentCx<UiVirtualizerProps>) -> RSX {
    cx.use_prop("className", |props: &UiVirtualizerProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiVirtualizerProps| props.label.clone());

    crate::rsx!(
        <Group
            key="root"
            data-slot="virtualizer"
            class="grid min-h-0 overflow-auto"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Group>
    )
}
