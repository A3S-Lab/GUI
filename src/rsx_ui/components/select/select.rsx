use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSelectProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_selection_change: String,
    pub on_open_change: String,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_select(cx: &mut ComponentCx<UiSelectProps>) -> RSX {
    cx.use_select(|props: &UiSelectProps| {
        UseSelectProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .on_open_change(Some(props.on_open_change.clone()))
            .open(props.is_open)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
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
