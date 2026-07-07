use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTabPanelProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsContentProps {
    pub class_name: String,
    pub value: String,
}

pub fn ui_tabs_content(cx: &mut ComponentCx<UiTabsContentProps>) -> RSX {
    cx.use_tab_panel(|props: &UiTabsContentProps| {
        UseTabPanelProps::new().value(Some(props.value.clone()))
    });
    cx.use_prop("className", |props: &UiTabsContentProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TabPanel
            key="root"
            {...props.tabPanelProps}
            data-slot="tabs-content"
            class="flex-1 outline-none"
            className={props.className}
        >
            <Slot key="content" />
        </TabPanel>
    )
}
