use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTreeItemProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_tree_item(cx: &mut ComponentCx<UiTreeItemProps>) -> RSX {
    cx.use_tree_item(|props: &UiTreeItemProps| {
        UseTreeItemProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .expanded(Some(props.is_expanded))
    });
    cx.use_prop("className", |props: &UiTreeItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TreeItem
            key="root"
            {...props.treeItemProps}
            data-slot="tree-item"
            class="rounded-md px-2 py-1 text-sm outline-none hover:bg-surface-strong hover:text-ink data-[selected=true]:bg-surface-strong data-[selected=true]:text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </TreeItem>
    )
}
