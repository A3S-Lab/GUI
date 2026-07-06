use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiTagProps| props.class_name.clone());
    cx.use_prop("value", |props: &UiTagProps| props.value.clone());
    cx.use_prop("textValue", |props: &UiTagProps| props.text_value.clone());
    cx.use_prop("onRemove", |props: &UiTagProps| props.on_remove.clone());
    cx.use_prop("isSelected", |props: &UiTagProps| props.is_selected);
    cx.use_prop("isDisabled", |props: &UiTagProps| props.is_disabled);

    crate::rsx!(
        <ListBoxItem
            key="root"
            data-slot="tag"
            data-selected={props.isSelected}
            class="inline-flex h-7 items-center gap-1 rounded-md border border-border bg-secondary px-2 text-xs font-medium text-secondary-foreground outline-none transition-colors hover:bg-accent hover:text-accent-foreground data-[selected=true]:border-primary disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            value={props.value}
            textValue={props.textValue}
            selected={props.isSelected}
            disabled={props.isDisabled}
            onRemove={props.onRemove}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
