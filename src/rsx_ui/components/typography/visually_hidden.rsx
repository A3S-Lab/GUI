use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiVisuallyHiddenProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_visually_hidden(cx: &mut ComponentCx<UiVisuallyHiddenProps>) -> RSX {
    cx.use_visually_hidden(|props: &UiVisuallyHiddenProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiVisuallyHiddenProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Text
            key="root"
            {...props.visuallyHiddenProps}
            data-slot="visually-hidden"
            class="sr-only"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
