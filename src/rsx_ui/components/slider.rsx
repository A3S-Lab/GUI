use crate::rsx_app::{ComponentCx, RSX};

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
}

pub fn ui_slider(cx: &mut ComponentCx<UiSliderProps>) -> RSX {
    cx.use_prop("className", |props: &UiSliderProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiSliderProps| props.label.clone());
    cx.use_prop("valueNumber", |props: &UiSliderProps| props.value_number);
    cx.use_prop("minValue", |props: &UiSliderProps| props.min_value);
    cx.use_prop("maxValue", |props: &UiSliderProps| props.max_value);
    cx.use_prop("stepValue", |props: &UiSliderProps| props.step_value);
    cx.use_prop("onChange", |props: &UiSliderProps| props.on_change.clone());
    cx.use_prop("isDisabled", |props: &UiSliderProps| props.is_disabled);

    crate::rsx!(
        <Slider
            key="root"
            data-slot="slider"
            class="relative flex w-full touch-none select-none items-center gap-2 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            label={props.label}
            valueNumber={props.valueNumber}
            minValue={props.minValue}
            maxValue={props.maxValue}
            stepValue={props.stepValue}
            onChange={props.onChange}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Slider>
    )
}
