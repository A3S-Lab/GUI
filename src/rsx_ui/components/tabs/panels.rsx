use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabPanelsProps {
    pub class_name: String,
}

pub fn ui_tab_panels(cx: &mut ComponentCx<UiTabPanelsProps>) -> RSX {
    cx.use_prop("className", |props: &UiTabPanelsProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            data-slot="tab-panels"
            class="grid gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
