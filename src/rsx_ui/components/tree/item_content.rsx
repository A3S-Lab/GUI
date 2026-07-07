use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeItemContentProps {
    pub class_name: String,
}

pub fn ui_tree_item_content(cx: &mut ComponentCx<UiTreeItemContentProps>) -> RSX {
    cx.use_prop("className", |props: &UiTreeItemContentProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            data-slot="tree-item-content"
            class="flex items-center gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
