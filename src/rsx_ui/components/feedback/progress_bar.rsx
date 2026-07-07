use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRangeProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiProgressBarProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub min_value: f64,
    pub max_value: f64,
}

pub fn ui_progress_bar(cx: &mut ComponentCx<UiProgressBarProps>) -> RSX {
    cx.use_range(|props: &UiProgressBarProps| {
        UseRangeProps::new()
            .value_number(props.value_number)
            .min_value(props.min_value)
            .max_value(props.max_value)
    });
    cx.use_prop("className", |props: &UiProgressBarProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiProgressBarProps| props.label.clone());

    crate::rsx!(
        <ProgressBar
            key="root"
            {...props.rangeProps}
            data-slot="progress-bar"
            class="relative h-2 w-full overflow-hidden rounded-full bg-surface-strong"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </ProgressBar>
    )
}
