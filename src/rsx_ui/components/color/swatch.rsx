use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorSwatchProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorSwatchProps {
    pub class_name: String,
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
}

pub fn ui_color_swatch(cx: &mut ComponentCx<UiColorSwatchProps>) -> RSX {
    cx.use_color_swatch(|props: &UiColorSwatchProps| {
        UseColorSwatchProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiColorSwatchProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.colorSwatchProps}
            data-slot="color-swatch"
            class="size-8 rounded-md border border-hairline-strong"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
