use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGridListItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_grid_list_item(cx: &mut ComponentCx<UiGridListItemProps>) -> RSX {
    cx.use_prop("className", |props: &UiGridListItemProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiGridListItemProps| props.value.clone());
    cx.use_prop("textValue", |props: &UiGridListItemProps| {
        props.text_value.clone()
    });
    cx.use_prop("isSelected", |props: &UiGridListItemProps| {
        props.is_selected
    });
    cx.use_prop("isDisabled", |props: &UiGridListItemProps| {
        props.is_disabled
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            data-slot="grid-list-item"
            data-selected={props.isSelected}
            class="rounded-md border border-border bg-card p-3 text-card-foreground outline-none transition-colors hover:bg-accent hover:text-accent-foreground data-[selected=true]:border-primary data-[selected=true]:bg-accent"
            className={props.className}
            value={props.value}
            textValue={props.textValue}
            selected={props.isSelected}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
