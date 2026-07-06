use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFormProps {
    pub class_name: String,
    pub label: String,
    pub on_submit: String,
}

pub fn ui_form(cx: &mut ComponentCx<UiFormProps>) -> RSX {
    cx.use_prop("className", |props: &UiFormProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiFormProps| props.label.clone());
    cx.use_prop("onSubmit", |props: &UiFormProps| props.on_submit.clone());

    crate::rsx!(
        <Form
            key="root"
            data-slot="form"
            class="grid gap-4"
            className={props.className}
            label={props.label}
            onSubmit={props.onSubmit}
        >
            <Slot key="content" />
        </Form>
    )
}
