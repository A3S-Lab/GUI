use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGroupProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_group(cx: &mut ComponentCx<UiGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiGroupProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiGroupProps| props.label.clone());

    crate::rsx!(
        <Group
            key="root"
            data-slot="group"
            class="grid gap-2"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Group>
    )
}
