use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFormProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFormProps {
    pub class_name: String,
    pub label: Option<String>,
    pub on_submit: Option<String>,
    pub on_reset: Option<String>,
    pub on_invalid: Option<String>,
    pub validation_behavior: Option<String>,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub no_validate: bool,
}

pub fn ui_form(cx: &mut ComponentCx<UiFormProps>) -> RSX {
    cx.use_form(|props: &UiFormProps| {
        UseFormProps::new()
            .label(props.label.clone())
            .on_submit(props.on_submit.clone())
            .on_reset(props.on_reset.clone())
            .on_invalid(props.on_invalid.clone())
            .validation_behavior(props.validation_behavior.clone())
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .no_validate(props.no_validate)
    });
    cx.use_prop("className", |props: &UiFormProps| props.class_name.clone());

    crate::rsx!(
        <Form
            key="root"
            {...props.formProps}
            data-slot="form"
            data-disabled={props.isDisabled}
            data-invalid={props.isInvalid}
            class="grid gap-4"
            className={props.className}
        >
            <Slot key="content" />
        </Form>
    )
}
