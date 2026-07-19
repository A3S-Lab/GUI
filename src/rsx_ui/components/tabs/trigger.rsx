use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTabProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsTriggerProps {
    pub class_name: String,
    pub value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_tabs_trigger(cx: &mut ComponentCx<UiTabsTriggerProps>) -> RSX {
    cx.use_tab(|props: &UiTabsTriggerProps| {
        UseTabProps::new()
            .value(Some(props.value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiTabsTriggerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Tab
            key="root"
            {...props.tabProps}
            data-slot="tabs-trigger"
            class="data-[selected=true]:bg-canvas data-[selected=true]:text-ink focus-visible:ring-ink/40 focus-visible:outline-ink inline-flex h-8 flex-1 items-center justify-center gap-1.5 whitespace-nowrap rounded-sm border border-transparent px-3 py-1 text-sm font-medium focus-visible:ring-[2px] disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Tab>
    )
}
