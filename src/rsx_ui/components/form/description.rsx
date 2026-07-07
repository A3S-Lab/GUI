use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDescriptionProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_description(cx: &mut ComponentCx<UiDescriptionProps>) -> RSX {
    cx.use_description(|props: &UiDescriptionProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiDescriptionProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Text
            key="root"
            {...props.descriptionProps}
            data-slot="description"
            class="text-sm text-body"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
