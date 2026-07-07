use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRangeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMeterProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
}

pub fn ui_meter(cx: &mut ComponentCx<UiMeterProps>) -> RSX {
    cx.use_range(|props: &UiMeterProps| {
        UseRangeProps::new()
            .value_number(props.value_number)
            .min_value(props.min_value)
            .max_value(props.max_value)
    });
    cx.use_prop("className", |props: &UiMeterProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiMeterProps| props.label.clone());

    crate::rsx!(
        <Meter
            key="root"
            {...props.rangeProps}
            data-slot="meter"
            class="relative h-2 w-full overflow-hidden rounded-full bg-surface-strong"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </Meter>
    )
}
