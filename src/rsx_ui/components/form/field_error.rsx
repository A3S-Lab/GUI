use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFieldErrorProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_field_error(cx: &mut ComponentCx<UiFieldErrorProps>) -> RSX {
    cx.use_field_error(|props: &UiFieldErrorProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiFieldErrorProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Text
            key="root"
            {...props.fieldErrorProps}
            data-slot="field-error"
            data-invalid="true"
            class="text-sm font-medium text-semantic-error"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
