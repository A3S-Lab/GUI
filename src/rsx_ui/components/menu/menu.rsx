use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::UseMenuProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMenuProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub selected_keys: Option<Selection>,
    pub default_selected_keys: Option<Selection>,
    pub disabled_keys: BTreeSet<CollectionKey>,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
    pub selection_behavior: String,
    pub disabled_behavior: String,
    pub disallow_empty_selection: bool,
    pub should_focus_wrap: bool,
}

pub fn ui_menu(cx: &mut ComponentCx<UiMenuProps>) -> RSX {
    cx.use_menu(|props: &UiMenuProps| {
        UseMenuProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .selected_keys(props.selected_keys.clone())
            .default_selected_keys(props.default_selected_keys.clone())
            .disabled_keys(props.disabled_keys.clone())
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
            .selection_behavior(Some(props.selection_behavior.clone()))
            .disabled_behavior(Some(props.disabled_behavior.clone()))
            .disallow_empty_selection(props.disallow_empty_selection)
            .should_focus_wrap(props.should_focus_wrap)
    });
    cx.use_prop("className", |props: &UiMenuProps| props.class_name.clone());

    crate::rsx!(
        <Menu
            key="root"
            {...props.menuProps}
            data-slot="menu"
            class="min-w-32 overflow-hidden rounded-md border bg-canvas p-1 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Menu>
    )
}
