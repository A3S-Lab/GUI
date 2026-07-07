use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTabListProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsListProps {
    pub class_name: String,
    pub label: String,
    pub orientation: String,
    pub is_disabled: bool,
}

pub fn ui_tabs_list(cx: &mut ComponentCx<UiTabsListProps>) -> RSX {
    cx.use_tab_list(|props: &UiTabsListProps| {
        UseTabListProps::new()
            .label(Some(props.label.clone()))
            .orientation(Some(props.orientation.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiTabsListProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TabList
            key="root"
            {...props.tabListProps}
            data-slot="tabs-list"
            class="inline-flex h-10 w-fit items-center justify-center rounded-md border border-hairline bg-surface-strong p-1 text-body"
            className={props.className}
        >
            <Slot key="content" />
        </TabList>
    )
}
