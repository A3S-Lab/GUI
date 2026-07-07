use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSliderTrackProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSliderTrackProps {
    pub class_name: String,
    pub orientation: String,
    pub is_disabled: bool,
}

pub fn ui_slider_track(cx: &mut ComponentCx<UiSliderTrackProps>) -> RSX {
    cx.use_slider_track(|props: &UiSliderTrackProps| {
        UseSliderTrackProps::new()
            .orientation(Some(props.orientation.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiSliderTrackProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.sliderTrackProps}
            data-slot="slider-track"
            class="relative h-2 w-full grow overflow-hidden rounded-full bg-surface-strong data-[orientation=vertical]:h-full data-[orientation=vertical]:w-2"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
