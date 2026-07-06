use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFieldSetProps {
    pub class_name: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_invalid: bool,
}

pub fn ui_field_set(cx: &mut ComponentCx<UiFieldSetProps>) -> RSX {
    cx.use_prop("className", |props: &UiFieldSetProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiFieldSetProps| props.label.clone());
    cx.use_prop("isDisabled", |props: &UiFieldSetProps| props.is_disabled);
    cx.use_prop("isInvalid", |props: &UiFieldSetProps| props.is_invalid);

    crate::rsx!(
        <FieldSet
            key="root"
            data-slot="field-set"
            class="grid gap-3 rounded-md border border-border p-4"
            className={props.className}
            label={props.label}
            disabled={props.isDisabled}
            aria-invalid={props.isInvalid}
        >
            <Slot key="content" />
        </FieldSet>
    )
}
