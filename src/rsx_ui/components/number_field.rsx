use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiNumberFieldProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiNumberFieldProps| props.label.clone());
    cx.use_prop("valueNumber", |props: &UiNumberFieldProps| {
        props.value_number
    });
    cx.use_prop("placeholder", |props: &UiNumberFieldProps| {
        props.placeholder.clone()
    });
    cx.use_prop("minValue", |props: &UiNumberFieldProps| props.min_value);
    cx.use_prop("maxValue", |props: &UiNumberFieldProps| props.max_value);
    cx.use_prop("stepValue", |props: &UiNumberFieldProps| props.step_value);
    cx.use_prop("onChange", |props: &UiNumberFieldProps| {
        props.on_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiNumberFieldProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiNumberFieldProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiNumberFieldProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiNumberFieldProps| {
        props.is_read_only
    });

    crate::rsx!(
        <TextField
            key="root"
            data-slot="number-field"
            class="grid gap-2"
            className={props.className}
            label={props.label}
            valueNumber={props.valueNumber}
            minValue={props.minValue}
            maxValue={props.maxValue}
            stepValue={props.stepValue}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            readonly={props.isReadOnly}
        >
            <Label
                key="label"
                data-slot="number-field-label"
                class="text-sm font-medium leading-none text-foreground"
                label={props.label}
            />
            <Input
                key="input"
                data-slot="number-field-input"
                class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive"
                type="number"
                valueNumber={props.valueNumber}
                placeholder={props.placeholder}
                minValue={props.minValue}
                maxValue={props.maxValue}
                stepValue={props.stepValue}
                onInput={props.onChange}
                disabled={props.isDisabled}
                required={props.isRequired}
                readonly={props.isReadOnly}
                aria-invalid={props.isInvalid}
            />
            <Slot key="content" />
        </TextField>
    )
}
