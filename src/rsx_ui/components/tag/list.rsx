use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::UseSelectionProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTagListProps {
    pub class_name: String,
    pub label: String,
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
    pub escape_key_behavior: String,
}

pub fn ui_tag_list(cx: &mut ComponentCx<UiTagListProps>) -> RSX {
    cx.use_selection(|props: &UiTagListProps| {
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
            .escape_key_behavior(Some(props.escape_key_behavior.clone()))
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
