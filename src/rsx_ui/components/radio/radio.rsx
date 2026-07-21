use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseRadioProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRadioProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_radio(cx: &mut ComponentCx<UiRadioProps>) -> RSX {
    cx.use_radio(|props: &UiRadioProps| {
        UseRadioProps::new()
            .value(Some(props.value.clone()))
            .text_value(Some(props.text_value.clone()))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
    });
    cx.use_prop("className", |props: &UiRadioProps| props.class_name.clone());

    crate::rsx!(
        <Radio
            key="root"
            {...props.radioProps}
            data-slot="radio"
            class="aspect-square size-4 shrink-0 rounded-full border border-hairline-strong text-ink outline-none focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 disabled:cursor-not-allowed disabled:opacity-50 data-[selected=true]:border-ink data-[selected=true]:bg-ink"
            className={props.className}
        >
            <Slot key="content" />
        </Radio>
    )
}
