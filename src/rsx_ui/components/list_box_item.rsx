use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_list_box_item(cx: &mut ComponentCx<UiListBoxItemProps>) -> RSX {
    cx.use_prop("className", |props: &UiListBoxItemProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiListBoxItemProps| props.value.clone());
    cx.use_prop("textValue", |props: &UiListBoxItemProps| {
        props.text_value.clone()
    });
    cx.use_prop("isSelected", |props: &UiListBoxItemProps| props.is_selected);
    cx.use_prop("isDisabled", |props: &UiListBoxItemProps| props.is_disabled);

    crate::rsx!(
        <ListBoxItem
            key="root"
            data-slot="list-box-item"
            data-selected={props.isSelected}
            class="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground"
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
