use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsContentProps {
    pub class_name: String,
    pub value: String,
}

pub fn ui_tabs_content(cx: &mut ComponentCx<UiTabsContentProps>) -> RSX {
    cx.use_prop("className", |props: &UiTabsContentProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiTabsContentProps| props.value.clone());

    crate::rsx!(
        <TabPanel
            key="root"
            data-slot="tabs-content"
            class="flex-1 outline-none"
            className={props.className}
            value={props.value}
        >
            <Slot key="content" />
        </TabPanel>
    )
}
