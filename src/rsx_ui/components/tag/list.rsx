use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTagListProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_tag_list(cx: &mut ComponentCx<UiTagListProps>) -> RSX {
    cx.use_selection(|props: &UiTagListProps| {
        UseSelectionProps::new()
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiTagListProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTagListProps| props.label.clone());

    crate::rsx!(
        <ListBox
            key="root"
            {...props.selectionProps}
            data-slot="tag-list"
            class="flex flex-wrap items-center gap-2"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </ListBox>
    )
}
