use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiRadioProps {
    pub class_name: String,
    pub value: String,
    pub text_value: String,
    pub is_selected: bool,
    pub is_disabled: bool,
}

pub fn ui_radio(cx: &mut ComponentCx<UiRadioProps>) -> RSX {
    cx.use_prop("className", |props: &UiRadioProps| props.class_name.clone());
    cx.use_prop("value", |props: &UiRadioProps| props.value.clone());
    cx.use_prop("textValue", |props: &UiRadioProps| props.text_value.clone());
    cx.use_prop("isSelected", |props: &UiRadioProps| props.is_selected);
    cx.use_prop("isDisabled", |props: &UiRadioProps| props.is_disabled);

    crate::rsx!(
        <Radio
            key="root"
            data-slot="radio"
            data-selected={props.isSelected}
            class="aspect-square size-4 shrink-0 rounded-full border border-input text-primary shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[selected=true]:border-primary data-[selected=true]:bg-primary"
            className={props.className}
            value={props.value}
            textValue={props.textValue}
            selected={props.isSelected}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </Radio>
    )
}
