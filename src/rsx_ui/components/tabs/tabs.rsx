use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTabsProps {
    pub class_name: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_tabs(cx: &mut ComponentCx<UiTabsProps>) -> RSX {
    cx.use_selection(|props: &UiTabsProps| {
        UseSelectionProps::new()
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiTabsProps| props.class_name.clone());

    crate::rsx!(
        <Tabs
            key="root"
            {...props.selectionProps}
            data-slot="tabs"
            class="flex flex-col gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </Tabs>
    )
}
