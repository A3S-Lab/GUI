use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSliderOutputProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSliderOutputProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub value_number: f64,
}

pub fn ui_slider_output(cx: &mut ComponentCx<UiSliderOutputProps>) -> RSX {
    cx.use_slider_output(|props: &UiSliderOutputProps| {
        UseSliderOutputProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .value_number(props.value_number)
    });
    cx.use_prop("className", |props: &UiSliderOutputProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Output
            key="root"
            {...props.sliderOutputProps}
            data-slot="slider-output"
            class="text-sm tabular-nums text-body"
            className={props.className}
        >
            <Slot key="content" />
        </Output>
    )
}
