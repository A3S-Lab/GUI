use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiLegendProps {
    pub class_name: String,
    pub label: String,
    pub text_value: String,
}

pub fn ui_legend(cx: &mut ComponentCx<UiLegendProps>) -> RSX {
    cx.use_prop("className", |props: &UiLegendProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiLegendProps| props.label.clone());
    cx.use_prop("textValue", |props: &UiLegendProps| {
        props.text_value.clone()
    });

    crate::rsx!(
        <Legend
            key="root"
            data-slot="legend"
            class="text-sm font-medium text-foreground"
            className={props.className}
            label={props.label}
            textValue={props.textValue}
        >
            <Slot key="content" />
        </Legend>
    )
}
