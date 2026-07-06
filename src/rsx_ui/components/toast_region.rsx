use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToastRegionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_toast_region(cx: &mut ComponentCx<UiToastRegionProps>) -> RSX {
    cx.use_prop("className", |props: &UiToastRegionProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiToastRegionProps| props.label.clone());

    crate::rsx!(
        <Group
            key="root"
            data-slot="toast-region"
            class="fixed bottom-4 right-4 z-50 grid w-80 gap-2"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Group>
    )
}
