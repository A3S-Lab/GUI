use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCheckboxProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCheckboxProps {
    pub class_name: String,
    pub value: String,
    pub on_change: String,
    pub is_checked: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_checkbox(cx: &mut ComponentCx<UiCheckboxProps>) -> RSX {
    cx.use_checkbox(|props: &UiCheckboxProps| {
        UseCheckboxProps::new()
            .value(Some(props.value.clone()))
            .on_change(Some(props.on_change.clone()))
            .checked(props.is_checked)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiCheckboxProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Checkbox
            key="root"
            {...props.checkboxProps}
            data-slot="checkbox"
            class="peer size-4 shrink-0 rounded-[4px] border border-hairline-strong outline-none transition-colors focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[checked=true]:border-primary data-[checked=true]:bg-primary data-[checked=true]:text-on-primary aria-invalid:border-semantic-error aria-invalid:ring-semantic-error/20"
            className={props.className}
        >
            <Slot key="content" />
        </Checkbox>
    )
}
