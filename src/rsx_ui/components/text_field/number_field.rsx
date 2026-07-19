use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseNumberFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiNumberFieldProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub placeholder: String,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_number_field(cx: &mut ComponentCx<UiNumberFieldProps>) -> RSX {
    cx.use_number_field(|props: &UiNumberFieldProps| {
        UseNumberFieldProps::new()
            .label(Some(props.label.clone()))
            .value_number(props.value_number)
            .placeholder(Some(props.placeholder.clone()))
            .min_value(props.min_value)
            .max_value(props.max_value)
            .step_value(props.step_value)
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiNumberFieldProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TextField
            key="root"
            {...props.numberFieldProps}
            data-slot="number-field"
            class="grid gap-2"
            className={props.className}
            label={props.label}
        >
            <Label
                key="label"
                data-slot="number-field-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Input
                key="input"
                {...props.numberFieldInputProps}
                data-slot="number-field-input"
                class="h-9 w-full min-w-0 rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm text-ink outline-none selection:bg-ink selection:text-canvas file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-ink placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
            />
            <Slot key="content" />
        </TextField>
    )
}
