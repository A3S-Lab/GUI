use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCollectionItemProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGridListItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_grid_list_item(cx: &mut ComponentCx<UiGridListItemProps>) -> RSX {
    cx.use_collection_item(|props: &UiGridListItemProps| {
        UseCollectionItemProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiGridListItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.collectionItemProps}
            data-slot="grid-list-item"
            class="rounded-md border border-hairline-strong bg-canvas p-3 text-ink outline-none transition-colors hover:bg-surface-strong hover:text-ink data-[selected=true]:border-primary data-[selected=true]:bg-surface-strong"
            className={props.className}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
