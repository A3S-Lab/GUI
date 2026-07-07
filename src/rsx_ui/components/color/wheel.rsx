use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorRangeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorWheelProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn ui_color_wheel(cx: &mut ComponentCx<UiColorWheelProps>) -> RSX {
    cx.use_color_wheel(|props: &UiColorWheelProps| {
        UseColorRangeProps::new()
            .label(Some(props.label.clone()))
            .channel(Some("hue"))
            .value_number(props.value_number)
            .min_value(0.0)
            .max_value(360.0)
            .step_value(1.0)
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiColorWheelProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.colorWheelProps}
            data-slot="color-wheel"
            class="relative size-40 rounded-full border border-hairline-strong bg-surface-strong"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
