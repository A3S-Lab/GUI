use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorSwatchPickerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorSwatchPickerProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_color_swatch_picker(cx: &mut ComponentCx<UiColorSwatchPickerProps>) -> RSX {
    cx.use_color_swatch_picker(|props: &UiColorSwatchPickerProps| {
        UseColorSwatchPickerProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiColorSwatchPickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBox
            key="root"
            {...props.colorSwatchPickerProps}
            data-slot="color-swatch-picker"
            class="flex flex-wrap gap-2"
            className={props.className}
        >
            <Slot key="content" />
        </ListBox>
    )
}
