use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseNumberFieldProps;
use crate::{NumberFormatOptions, NumberFormatStyle, NumberGrouping, NumberSignDisplay};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiNumberFieldProps {
    pub class_name: String,
    pub label: String,
    pub value_number: f64,
    pub placeholder: String,
    pub min_value: f64,
    pub max_value: f64,
    pub step_value: f64,
    pub format_style: String,
    pub grouping: String,
    pub minimum_fraction_digits: Option<u8>,
    pub maximum_fraction_digits: Option<u8>,
    pub sign_display: String,
    pub increment_aria_label: String,
    pub decrement_aria_label: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_wheel_disabled: bool,
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
            .format_options(number_format_options(props))
            .on_change(Some(props.on_change.clone()))
            .increment_aria_label(Some(props.increment_aria_label.clone()))
            .decrement_aria_label(Some(props.decrement_aria_label.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .wheel_disabled(props.is_wheel_disabled)
    });
    cx.use_prop("className", |props: &UiNumberFieldProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
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
            <Group
                key="controls"
                data-slot="number-field-controls"
                class="flex items-center"
            >
                <button
                    key="decrement"
                    {...props.decrementButtonProps}
                    data-slot="number-field-decrement"
                    class="inline-flex h-9 min-w-9 items-center justify-center rounded-md border border-r-0 border-hairline-strong bg-surface-card px-2 text-sm font-medium text-ink outline-none disabled:pointer-events-none disabled:text-muted-soft focus-visible:ring-[2px] focus-visible:ring-ink/40"
                >
                    -
                </button>
                <Input
                    key="input"
                    {...props.numberFieldInputProps}
                    data-slot="number-field-input"
                    class="h-9 w-full min-w-0 rounded-none border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm text-ink outline-none selection:bg-ink selection:text-canvas file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-ink placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
                />
                <button
                    key="increment"
                    {...props.incrementButtonProps}
                    data-slot="number-field-increment"
                    class="inline-flex h-9 min-w-9 items-center justify-center rounded-md border border-l-0 border-hairline-strong bg-surface-card px-2 text-sm font-medium text-ink outline-none disabled:pointer-events-none disabled:text-muted-soft focus-visible:ring-[2px] focus-visible:ring-ink/40"
                >
                    +
                </button>
            </Group>
            <Slot key="content" />
        </Group>
    )
}

fn number_format_options(props: &UiNumberFieldProps) -> NumberFormatOptions {
    NumberFormatOptions {
        style: props
            .format_style
            .parse::<NumberFormatStyle>()
            .unwrap_or_default(),
        grouping: props.grouping.parse::<NumberGrouping>().unwrap_or_default(),
        minimum_fraction_digits: props.minimum_fraction_digits,
        maximum_fraction_digits: props.maximum_fraction_digits,
        sign_display: props
            .sign_display
            .parse::<NumberSignDisplay>()
            .unwrap_or_default(),
    }
}
