use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCheckboxGroupProps {
    pub class_name: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
}

pub fn ui_checkbox_group(cx: &mut ComponentCx<UiCheckboxGroupProps>) -> RSX {
    cx.use_prop("className", |props: &UiCheckboxGroupProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiCheckboxGroupProps| props.label.clone());
    cx.use_prop("isDisabled", |props: &UiCheckboxGroupProps| {
        props.is_disabled
    });
    cx.use_prop("isRequired", |props: &UiCheckboxGroupProps| {
        props.is_required
    });
    cx.use_prop("isInvalid", |props: &UiCheckboxGroupProps| props.is_invalid);

    crate::rsx!(
        <FieldSet
            key="root"
            data-slot="checkbox-group"
            class="grid gap-3"
            className={props.className}
            label={props.label}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
        >
            <Slot key="content" />
        </FieldSet>
    )
}
