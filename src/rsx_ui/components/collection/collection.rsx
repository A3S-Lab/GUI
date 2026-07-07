use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCollectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCollectionProps {
    pub class_name: String,
    pub label: String,
    pub item_count: usize,
    pub is_empty: bool,
    pub is_disabled: bool,
}

pub fn ui_collection(cx: &mut ComponentCx<UiCollectionProps>) -> RSX {
    cx.use_collection(|props: &UiCollectionProps| {
        UseCollectionProps::new()
            .label(Some(props.label.clone()))
            .item_count(props.item_count)
            .empty(props.is_empty)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiCollectionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.collectionProps}
            data-slot="collection"
            data-empty={props.isEmpty}
            data-disabled={props.isDisabled}
            class="grid gap-2 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
