use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiInputProps {
    pub class_name: String,
    pub input_type: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_read_only: bool,
    pub is_invalid: bool,
}

pub fn ui_input(cx: &mut ComponentCx<UiInputProps>) -> RSX {
    cx.use_text_field(|props: &UiInputProps| {
        UseTextFieldProps::new()
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .input_type(Some(props.input_type.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .read_only(props.is_read_only)
            .invalid(props.is_invalid)
    });
    cx.use_prop("className", |props: &UiInputProps| props.class_name.clone());

    crate::rsx!(
        <input
            key="root"
            {...props.inputProps}
            data-slot="input"
            class="h-9 w-full min-w-0 rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm text-ink outline-none selection:bg-ink selection:text-canvas file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-ink placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
            className={props.className}
        />
    )
}
