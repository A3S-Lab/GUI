use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiProgressBarProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
}

pub fn ui_progress_bar(cx: &mut ComponentCx<UiProgressBarProps>) -> RSX {
    cx.use_prop("className", |props: &UiProgressBarProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiProgressBarProps| props.label.clone());
    cx.use_prop("valueNumber", |props: &UiProgressBarProps| {
        props.value_number
    });
    cx.use_prop("minValue", |props: &UiProgressBarProps| props.min_value);
    cx.use_prop("maxValue", |props: &UiProgressBarProps| props.max_value);

    crate::rsx!(
        <ProgressBar
            key="root"
            data-slot="progress-bar"
            class="relative h-2 w-full overflow-hidden rounded-full bg-secondary"
            className={props.className}
            label={props.label}
            valueNumber={props.valueNumber}
            minValue={props.minValue}
            maxValue={props.maxValue}
        >
            <Slot key="content" />
        </ProgressBar>
    )
}
