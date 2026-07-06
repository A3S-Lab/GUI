use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiTreeItemProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiTreeItemProps| props.value.clone());
    cx.use_prop("textValue", |props: &UiTreeItemProps| {
        props.text_value.clone()
    });
    cx.use_prop("isExpanded", |props: &UiTreeItemProps| props.is_expanded);
    cx.use_prop("isSelected", |props: &UiTreeItemProps| props.is_selected);
    cx.use_prop("isDisabled", |props: &UiTreeItemProps| props.is_disabled);

    crate::rsx!(
        <ListBoxItem
            key="root"
            data-slot="tree-item"
            data-expanded={props.isExpanded}
            data-selected={props.isSelected}
            class="rounded-md px-2 py-1 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground"
            className={props.className}
            value={props.value}
            textValue={props.textValue}
            expanded={props.isExpanded}
            selected={props.isSelected}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
