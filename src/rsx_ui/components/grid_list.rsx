use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGridListProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
}

pub fn ui_grid_list(cx: &mut ComponentCx<UiGridListProps>) -> RSX {
    cx.use_prop("className", |props: &UiGridListProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiGridListProps| props.label.clone());
    cx.use_prop("value", |props: &UiGridListProps| props.value.clone());
    cx.use_prop("onSelectionChange", |props: &UiGridListProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiGridListProps| props.is_disabled);

    crate::rsx!(
        <ListBox
            key="root"
            data-slot="grid-list"
            class="grid gap-2"
            className={props.className}
            label={props.label}
            value={props.value}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </ListBox>
    )
}
