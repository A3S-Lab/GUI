use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiMeterProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
}

pub fn ui_meter(cx: &mut ComponentCx<UiMeterProps>) -> RSX {
    cx.use_prop("className", |props: &UiMeterProps| props.class_name.clone());
    cx.use_prop("label", |props: &UiMeterProps| props.label.clone());
    cx.use_prop("valueNumber", |props: &UiMeterProps| props.value_number);
    cx.use_prop("minValue", |props: &UiMeterProps| props.min_value);
    cx.use_prop("maxValue", |props: &UiMeterProps| props.max_value);

    crate::rsx!(
        <Meter
            key="root"
            data-slot="meter"
            class="relative h-2 w-full overflow-hidden rounded-full bg-secondary"
            className={props.className}
            label={props.label}
            valueNumber={props.valueNumber}
            minValue={props.minValue}
            maxValue={props.maxValue}
        >
            <Slot key="content" />
        </Meter>
    )
}
