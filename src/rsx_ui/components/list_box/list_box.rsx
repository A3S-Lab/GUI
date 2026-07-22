use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::UseSelectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiListBoxProps {
    pub class_name: String,
    pub value: String,
    pub selected_keys: Option<Selection>,
    pub default_selected_keys: Option<Selection>,
    pub disabled_keys: BTreeSet<CollectionKey>,
    pub on_action: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
    pub selection_behavior: String,
    pub disabled_behavior: String,
    pub disallow_empty_selection: bool,
    pub should_focus_wrap: bool,
    pub escape_key_behavior: String,
}

pub fn ui_list_box(cx: &mut ComponentCx<UiListBoxProps>) -> RSX {
    cx.use_selection(|props: &UiListBoxProps| {
        UseSelectionProps::new()
            .value(Some(props.value.clone()))
            .selected_keys(props.selected_keys.clone())
            .default_selected_keys(props.default_selected_keys.clone())
            .disabled_keys(props.disabled_keys.clone())
            .on_action(Some(props.on_action.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
            .selection_behavior(Some(props.selection_behavior.clone()))
            .disabled_behavior(Some(props.disabled_behavior.clone()))
            .disallow_empty_selection(props.disallow_empty_selection)
            .should_focus_wrap(props.should_focus_wrap)
            .escape_key_behavior(Some(props.escape_key_behavior.clone()))
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
