use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseToastRegionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiToastRegionProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_toast_region(cx: &mut ComponentCx<UiToastRegionProps>) -> RSX {
    cx.use_toast_region(|props: &UiToastRegionProps| {
        UseToastRegionProps::new().label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiToastRegionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.toastRegionProps}
            data-slot="toast-region"
            class="fixed bottom-4 right-4 z-50 grid w-80 gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
