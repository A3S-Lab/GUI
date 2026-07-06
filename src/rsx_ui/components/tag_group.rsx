use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTagGroupProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
}

pub fn ui_tag_group(cx: &mut ComponentCx<UiTagGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiTagGroupProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTagGroupProps| props.label.clone());
    cx.use_prop("value", |props: &UiTagGroupProps| props.value.clone());
    cx.use_prop("onSelectionChange", |props: &UiTagGroupProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiTagGroupProps| props.is_disabled);

    crate::rsx!(
        <ListBox
            key="root"
            data-slot="tag-group"
            class="flex flex-wrap items-center gap-2"
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
