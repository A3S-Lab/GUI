use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRadioGroupProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRadioGroupProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_radio_group(cx: &mut ComponentCx<UiRadioGroupProps>) -> RSX {
    cx.use_radio_group(|props: &UiRadioGroupProps| {
        UseRadioGroupProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiRadioGroupProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <RadioGroup
            key="root"
            {...props.radioGroupProps}
            data-slot="radio-group"
            class="grid gap-3"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </RadioGroup>
    )
}
