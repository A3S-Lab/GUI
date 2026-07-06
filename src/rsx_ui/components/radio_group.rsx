use crate::rsx_app::{ComponentCx, RSX};

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
}

pub fn ui_radio_group(cx: &mut ComponentCx<UiRadioGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiRadioGroupProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiRadioGroupProps| props.label.clone());
    cx.use_prop("value", |props: &UiRadioGroupProps| props.value.clone());
    cx.use_prop("onSelectionChange", |props: &UiRadioGroupProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiRadioGroupProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiRadioGroupProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiRadioGroupProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiRadioGroupProps| props.is_read_only);

    crate::rsx!(
        <RadioGroup
            key="root"
            data-slot="radio-group"
            class="grid gap-3"
            className={props.className}
            label={props.label}
            value={props.value}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            readonly={props.isReadOnly}
        >
            <Slot key="content" />
        </RadioGroup>
    )
}
