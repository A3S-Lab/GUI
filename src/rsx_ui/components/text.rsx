use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTextProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_text(cx: &mut ComponentCx<UiTextProps>) -> RSX {
    cx.use_prop("className", |props: &UiTextProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiTextProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiTextProps| props.text_value.clone());

    crate::rsx!(
        <Text
            key="root"
            data-slot="text"
            class="text-sm leading-6 text-foreground"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
        >
            <Slot key="content" />
        </Text>
    )
}
