use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTreeProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
}

pub fn ui_tree(cx: &mut ComponentCx<UiTreeProps>) -> RSX {
    cx.use_prop("className", |props: &UiTreeProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiTreeProps| props.label.clone());
    cx.use_prop("value", |props: &UiTreeProps| props.value.clone());
    cx.use_prop("onSelectionChange", |props: &UiTreeProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiTreeProps| props.is_disabled);

    crate::rsx!(
        <ListBox
            key="root"
            data-slot="tree"
            class="grid gap-1"
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
