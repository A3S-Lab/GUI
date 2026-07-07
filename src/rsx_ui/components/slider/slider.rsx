use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRangeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSliderProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn ui_slider(cx: &mut ComponentCx<UiSliderProps>) -> RSX {
    cx.use_range(|props: &UiSliderProps| {
        UseRangeProps::new()
            .value_number(props.value_number)
            .min_value(props.min_value)
            .max_value(props.max_value)
            .step_value(props.step_value)
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiSliderProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiSliderProps| props.label.clone());

    crate::rsx!(
        <Slider
            key="root"
            {...props.rangeProps}
            data-slot="slider"
            class="relative flex w-full touch-none select-none items-center gap-2 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Slider>
    )
}
