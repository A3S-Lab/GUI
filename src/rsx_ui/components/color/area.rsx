use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorAreaProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorAreaProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub x_channel: String,
    pub y_channel: String,
    pub x_value: f64,
    pub y_value: f64,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn ui_color_area(cx: &mut ComponentCx<UiColorAreaProps>) -> RSX {
    cx.use_color_area(|props: &UiColorAreaProps| {
        UseColorAreaProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .x_channel(Some(props.x_channel.clone()))
            .y_channel(Some(props.y_channel.clone()))
            .x_value(props.x_value)
            .y_value(props.y_value)
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiColorAreaProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.colorAreaProps}
            data-slot="color-area"
            class="relative h-40 w-full overflow-hidden rounded-md border border-hairline-strong bg-surface-strong"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
