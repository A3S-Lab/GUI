use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq)]
pub struct UiHeadingProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
    pub level: u32,
}

impl Default for UiHeadingProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            label: String::new(),
            text_value: String::new(),
            level: 2,
        }
    }
}

pub fn ui_heading(cx: &mut ComponentCx<UiHeadingProps>) -> RSX {
    cx.use_prop("className", |props: &UiHeadingProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiHeadingProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiHeadingProps| {
        props.text_value.clone()
    });
    cx.use_prop("level", |props: &UiHeadingProps| props.level);

    crate::rsx!(
        <Heading
            key="root"
            data-slot="heading"
            class="scroll-m-20 text-xl font-semibold tracking-normal text-foreground"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
            aria-level={props.level}
        >
            <Slot key="content" />
        </Heading>
    )
}
