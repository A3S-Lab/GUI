use std::collections::BTreeSet;

use crate::rsx_app::{ComponentCx, RSX};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::{UseFieldProps, UseSelectionProps};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCalendarYearPickerProps {
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
}

pub fn ui_calendar_year_picker(cx: &mut ComponentCx<UiCalendarYearPickerProps>) -> RSX {
    cx.use_field(|props: &UiCalendarYearPickerProps| {
        UseFieldProps::new()
            .label(Some(props.label.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_selection(|props: &UiCalendarYearPickerProps| {
        UseSelectionProps::new()
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
    });
    cx.use_prop("className", |props: &UiCalendarYearPickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Select
            key="root"
            {...props.fieldProps}
            {...props.selectionProps}
            data-slot="calendar-year-picker"
            class="min-w-20"
            className={props.className}
        >
            <Slot key="content" />
        </Select>
    )
}
