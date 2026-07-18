use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorPickerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorPickerProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
}

pub fn ui_color_picker(cx: &mut ComponentCx<UiColorPickerProps>) -> RSX {
    cx.use_color_picker(|props: &UiColorPickerProps| {
        UseColorPickerProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiColorPickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.colorPickerProps}
            data-slot="color-picker"
            class="grid gap-3 rounded-md border border-hairline bg-canvas p-3 text-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
