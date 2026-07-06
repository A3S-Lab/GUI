use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsTriggerProps {
    pub class_name: String,
    pub value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_tabs_trigger(cx: &mut ComponentCx<UiTabsTriggerProps>) -> RSX {
    cx.use_prop("className", |props: &UiTabsTriggerProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiTabsTriggerProps| props.value.clone());
    cx.use_prop("isSelected", |props: &UiTabsTriggerProps| props.is_selected);
    cx.use_prop("isDisabled", |props: &UiTabsTriggerProps| props.is_disabled);

    crate::rsx!(
        <Tab
            key="root"
            data-slot="tabs-trigger"
            data-selected={props.isSelected}
            class="data-[selected=true]:bg-background data-[selected=true]:text-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:shadow-sm"
            className={props.className}
            value={props.value}
            selected={props.isSelected}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Tab>
    )
}
