use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDateSegmentProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDateSegmentProps {
    pub class_name: String,
    pub segment_type: String,
    pub value: String,
    pub text_value: String,
    pub placeholder: String,
    pub is_placeholder: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
}

pub fn ui_date_segment(cx: &mut ComponentCx<UiDateSegmentProps>) -> RSX {
    cx.use_date_segment(|props: &UiDateSegmentProps| {
        UseDateSegmentProps::new()
            .segment_type(Some(props.segment_type.clone()))
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .placeholder_segment(props.is_placeholder)
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
    });
    cx.use_prop("className", |props: &UiDateSegmentProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Text
            key="root"
            {...props.dateSegmentProps}
            data-slot="date-segment"
            class="inline-flex min-w-[1.5ch] items-center justify-center rounded-sm px-0.5 text-sm text-ink outline-none data-[placeholder=true]:text-body data-[invalid=true]:text-semantic-error"
            className={props.className}
        >
            <Slot key="content" />
        </Text>
    )
}
