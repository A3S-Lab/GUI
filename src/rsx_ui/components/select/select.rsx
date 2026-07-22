use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::UseSelectProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub selected_keys: Option<Selection>,
    pub default_selected_keys: Option<Selection>,
    pub disabled_keys: BTreeSet<CollectionKey>,
    pub placeholder: String,
    pub on_selection_change: String,
    pub on_open_change: String,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
    pub selection_behavior: String,
    pub disabled_behavior: String,
    pub disallow_empty_selection: bool,
}

pub fn ui_select(cx: &mut ComponentCx<UiSelectProps>) -> RSX {
    cx.use_select(|props: &UiSelectProps| {
        UseSelectProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .selected_keys(props.selected_keys.clone())
            .default_selected_keys(props.default_selected_keys.clone())
            .disabled_keys(props.disabled_keys.clone())
            .placeholder(Some(props.placeholder.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .on_open_change(Some(props.on_open_change.clone()))
            .open(props.is_open)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
            .selection_behavior(Some(props.selection_behavior.clone()))
            .disabled_behavior(Some(props.disabled_behavior.clone()))
            .disallow_empty_selection(props.disallow_empty_selection)
    });
    cx.use_prop("className", |props: &UiSelectProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Select
            key="root"
            {...props.selectProps}
            data-slot="select"
            class="grid gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </Select>
    )
}
