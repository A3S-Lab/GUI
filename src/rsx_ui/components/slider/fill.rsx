use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSliderFillProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSliderFillProps {
    pub class_name: String,
    pub orientation: String,
    pub value_number: f64,
    pub is_disabled: bool,
}

pub fn ui_slider_fill(cx: &mut ComponentCx<UiSliderFillProps>) -> RSX {
    cx.use_slider_fill(|props: &UiSliderFillProps| {
        UseSliderFillProps::new()
            .orientation(Some(props.orientation.clone()))
            .value_number(props.value_number)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiSliderFillProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.sliderFillProps}
            data-slot="slider-fill"
            class="absolute inset-y-0 left-0 bg-primary transition-[width,height] data-[orientation=vertical]:inset-x-0 data-[orientation=vertical]:bottom-0 data-[orientation=vertical]:top-auto"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
