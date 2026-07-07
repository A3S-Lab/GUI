use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLegendProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_legend(cx: &mut ComponentCx<UiLegendProps>) -> RSX {
    cx.use_legend(|props: &UiLegendProps| {
        UseTextProps::new()
            .label(Some(props.label.clone()))
            .text_value(Some(props.text_value.clone()))
    });
    cx.use_prop("className", |props: &UiLegendProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Legend
            key="root"
            {...props.legendProps}
            data-slot="legend"
            class="text-sm font-medium text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Legend>
    )
}
