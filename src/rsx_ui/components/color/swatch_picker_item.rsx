use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorSwatchPickerItemProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorSwatchPickerItemProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_color_swatch_picker_item(cx: &mut ComponentCx<UiColorSwatchPickerItemProps>) -> RSX {
    cx.use_color_swatch_picker_item(|props: &UiColorSwatchPickerItemProps| {
        UseColorSwatchPickerItemProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiColorSwatchPickerItemProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ListBoxItem
            key="root"
            {...props.colorSwatchPickerItemProps}
            data-slot="color-swatch-picker-item"
            class="relative flex size-9 cursor-default select-none items-center justify-center rounded-md border border-hairline-strong p-0.5 outline-none transition-colors focus:ring-[3px] focus:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 data-[selected=true]:ring-[3px] data-[selected=true]:ring-ring/50"
            className={props.className}
        >
            <Slot key="content" />
        </ListBoxItem>
    )
}
