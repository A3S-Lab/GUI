use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseColorFieldProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiColorFieldProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub color_space: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_color_field(cx: &mut ComponentCx<UiColorFieldProps>) -> RSX {
    cx.use_color_field(|props: &UiColorFieldProps| {
        UseColorFieldProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .color_space(Some(props.color_space.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiColorFieldProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <TextField
            key="root"
            {...props.colorFieldProps}
            data-slot="color-field"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="color-field-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Input
                key="input"
                {...props.colorFieldInputProps}
                data-slot="color-field-input"
                class="h-9 w-full min-w-0 rounded-md border border-hairline-strong bg-surface-card px-3 py-1.5 text-sm font-mono text-ink outline-none selection:bg-ink selection:text-canvas file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-ink placeholder:text-muted disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[2px] focus-visible:ring-ink/40 aria-invalid:border-semantic-error"
            />
            <Slot key="content" />
        </TextField>
    )
}
