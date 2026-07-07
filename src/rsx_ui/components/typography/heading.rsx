use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseHeadingProps;

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
    cx.use_heading(|props: &UiHeadingProps| {
        UseHeadingProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
            .level(props.level)
    });
    cx.use_prop("className", |props: &UiHeadingProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Heading
            key="root"
            {...props.headingProps}
            data-slot="heading"
            class="scroll-m-20 text-xl font-semibold tracking-normal text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Heading>
    )
}
