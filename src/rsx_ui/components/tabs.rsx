use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsProps {
    pub class_name: String,
    pub value: String,
    pub on_selection_change: String,
}

pub fn ui_tabs(cx: &mut ComponentCx<UiTabsProps>) -> RSX {
    cx.use_prop("className", |props: &UiTabsProps| props.class_name.clone());
    cx.use_prop("value", |props: &UiTabsProps| props.value.clone());
    cx.use_prop("onSelectionChange", |props: &UiTabsProps| {
        props.on_selection_change.clone()
    });

    crate::rsx!(
        <Tabs
            key="root"
            data-slot="tabs"
            class="flex flex-col gap-2"
            className={props.className}
            value={props.value}
            onSelectionChange={props.onSelectionChange}
        >
            <Slot key="content" />
        </Tabs>
    )
}
