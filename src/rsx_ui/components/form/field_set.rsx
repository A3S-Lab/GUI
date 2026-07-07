use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFieldSetProps {
    pub class_name: String,
    pub label: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_field_set(cx: &mut ComponentCx<UiFieldSetProps>) -> RSX {
    cx.use_field(|props: &UiFieldSetProps| {
        UseFieldProps::new()
            .label(Some(props.label.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiFieldSetProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <FieldSet
            key="root"
            {...props.fieldProps}
            data-slot="field-set"
            class="grid gap-3 rounded-lg border border-hairline-strong p-4"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </FieldSet>
    )
}
