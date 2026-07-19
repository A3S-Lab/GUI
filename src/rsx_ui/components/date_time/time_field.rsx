use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTimeFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTimeFieldProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub granularity: String,
    pub hour_cycle: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_time_field(cx: &mut ComponentCx<UiTimeFieldProps>) -> RSX {
    cx.use_time_field(|props: &UiTimeFieldProps| {
        UseTimeFieldProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .granularity(Some(props.granularity.clone()))
            .hour_cycle(Some(props.hour_cycle.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiTimeFieldProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TextField
            key="root"
            {...props.timeFieldProps}
            data-slot="time-field"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="time-field-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Input
                key="input"
                {...props.timeFieldInputProps}
                data-slot="time-field-input"
                class="h-9 w-full min-w-0 rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm text-ink outline-none selection:bg-ink selection:text-canvas file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-ink placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
            />
            <Slot key="content" />
        </TextField>
    )
}
