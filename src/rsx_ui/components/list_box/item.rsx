use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCollectionItemProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_list_box_item(cx: &mut ComponentCx<UiListBoxItemProps>) -> RSX {
    cx.use_collection_item(|props: &UiListBoxItemProps| {
        UseCollectionItemProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiListBoxItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.collectionItemProps}
            data-slot="list-box-item"
            class="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-surface-strong focus:text-ink disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:bg-surface-strong data-[selected=true]:text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
