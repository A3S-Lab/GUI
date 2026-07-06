use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDisclosureGroupProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_disclosure_group(cx: &mut ComponentCx<UiDisclosureGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiDisclosureGroupProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiDisclosureGroupProps| {
        props.label.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            data-slot="disclosure-group"
            class="grid gap-2"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Group>
    )
}
