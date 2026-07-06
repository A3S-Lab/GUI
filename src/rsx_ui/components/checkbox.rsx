use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCheckboxProps {
    pub class_name: String,
    pub on_change: String,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
}

pub fn ui_checkbox(cx: &mut ComponentCx<UiCheckboxProps>) -> RSX {
    cx.use_prop("className", |props: &UiCheckboxProps| {
        props.class_name.clone()
    });
    cx.use_prop("onChange", |props: &UiCheckboxProps| {
        props.on_change.clone()
    });
    cx.use_prop("isChecked", |props: &UiCheckboxProps| props.is_checked);
    cx.use_prop("isDisabled", |props: &UiCheckboxProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiCheckboxProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiCheckboxProps| props.is_invalid);

    crate::rsx!(
        <Checkbox
            key="root"
            data-slot="checkbox"
            class="peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[checked=true]:border-primary data-[checked=true]:bg-primary data-[checked=true]:text-primary-foreground aria-invalid:border-destructive aria-invalid:ring-destructive/20"
            className={props.className}
            checked={props.isChecked}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            onChange={props.onChange}
        >
            <Slot key="content" />
        </Checkbox>
    )
}
