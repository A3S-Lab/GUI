use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTextProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_text(cx: &mut ComponentCx<UiTextProps>) -> RSX {
    cx.use_text(|props: &UiTextProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiTextProps| props.class_name.clone());

    crate::rsx!(
        <Text
            key="root"
            {...props.textProps}
            data-slot="text"
            class="text-sm leading-6 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
