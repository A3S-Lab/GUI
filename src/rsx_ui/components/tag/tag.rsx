use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCollectionItemProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTagProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub on_remove: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_tag(cx: &mut ComponentCx<UiTagProps>) -> RSX {
    cx.use_collection_item(|props: &UiTagProps| {
        UseCollectionItemProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiTagProps| props.class_name.clone());
    cx.use_prop("onRemove", |props: &UiTagProps| props.on_remove.clone());

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.collectionItemProps}
            data-slot="tag"
            class="inline-flex min-h-7 items-center gap-1 rounded-full border border-transparent bg-surface-strong px-2.5 py-1 text-xs font-medium text-ink outline-none transition-colors active:bg-surface-pressed data-[selected=true]:bg-primary data-[selected=true]:text-on-primary disabled:pointer-events-none disabled:text-muted-soft"
            className={props.className}
            onRemove={props.onRemove}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
