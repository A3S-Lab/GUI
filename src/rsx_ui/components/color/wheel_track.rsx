use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorWheelTrackProps {
    pub class_name: String,
    pub label: String,
}

pub fn ui_color_wheel_track(cx: &mut ComponentCx<UiColorWheelTrackProps>) -> RSX {
    cx.use_field(|props: &UiColorWheelTrackProps| {
        UseFieldProps::new().label(Some(props.label.clone()))
    });
    cx.use_prop("className", |props: &UiColorWheelTrackProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.fieldProps}
            data-slot="color-wheel-track"
            class="relative size-40 rounded-full border border-hairline-strong bg-canvas-soft"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
