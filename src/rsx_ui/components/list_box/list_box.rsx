use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxProps {
    pub class_name: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_list_box(cx: &mut ComponentCx<UiListBoxProps>) -> RSX {
    cx.use_selection(|props: &UiListBoxProps| {
        UseSelectionProps::new()
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiListBoxProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBox
            key="root"
            {...props.selectionProps}
            data-slot="list-box"
            class="max-h-72 min-w-32 overflow-auto rounded-md border border-hairline-strong bg-surface-card p-1 text-ink outline-none"
            className={props.className}
        >
            <Slot key="content" />
        </ListBox>
    )
}
