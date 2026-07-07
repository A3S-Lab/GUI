use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTextFieldProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_text_field(cx: &mut ComponentCx<UiTextFieldProps>) -> RSX {
    cx.use_text_field(|props: &UiTextFieldProps| {
        UseTextFieldProps::new()
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiTextFieldProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTextFieldProps| props.label.clone());

    crate::rsx!(
        <TextField
            key="root"
            {...props.fieldProps}
            data-slot="text-field"
            class="grid gap-2"
            className={props.className}
            label={props.label}
        >
            <Label
                key="label"
                data-slot="text-field-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Input
                key="input"
                {...props.inputProps}
                data-slot="text-field-input"
                class="h-11 w-full min-w-0 rounded-md border border-hairline-strong bg-canvas px-4 py-3 text-sm text-ink outline-none transition-colors placeholder:text-mute disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-semantic-error"
            />
            <Slot key="content" />
        </TextField>
    )
}
