use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorRangeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorSliderProps {
    pub class_name: String,
    pub label: String,
    pub channel: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn ui_color_slider(cx: &mut ComponentCx<UiColorSliderProps>) -> RSX {
    cx.use_color_slider(|props: &UiColorSliderProps| {
        UseColorRangeProps::new()
            .label(Some(props.label.clone()))
            .channel(Some(props.channel.clone()))
            .value_number(props.value_number)
            .min_value(props.min_value)
            .max_value(props.max_value)
            .step_value(props.step_value)
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiColorSliderProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Slider
            key="root"
            {...props.colorSliderProps}
            data-slot="color-slider"
            class="relative flex w-full touch-none select-none items-center gap-2 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </Slider>
    )
}
