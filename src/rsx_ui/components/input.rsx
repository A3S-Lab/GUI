use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiInputProps| props.class_name.clone());
    cx.use_prop("type", |props: &UiInputProps| props.input_type.clone());
    cx.use_prop("value", |props: &UiInputProps| props.value.clone());
    cx.use_prop("placeholder", |props: &UiInputProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onChange", |props: &UiInputProps| props.on_change.clone());
    cx.use_prop("isDisabled", |props: &UiInputProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiInputProps| props.is_required);
    cx.use_prop("isReadOnly", |props: &UiInputProps| props.is_read_only);
    cx.use_prop("isInvalid", |props: &UiInputProps| props.is_invalid);

    crate::rsx!(
        <input
            key="root"
            data-slot="input"
            class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30 focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40"
            className={props.className}
            type={props.type}
            value={props.value}
            placeholder={props.placeholder}
            onInput={props.onChange}
            disabled={props.isDisabled}
            required={props.isRequired}
            readonly={props.isReadOnly}
            aria-invalid={props.isInvalid}
        />
    )
}
