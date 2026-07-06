use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsListProps {
    pub class_name: String,
}

pub fn ui_tabs_list(cx: &mut ComponentCx<UiTabsListProps>) -> RSX {
    cx.use_prop("className", |props: &UiTabsListProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TabList
            key="root"
            data-slot="tabs-list"
            class="bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]"
            className={props.className}
        >
            <Slot key="content" />
        </TabList>
    )
}
