use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseAutocompleteProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiAutocompleteProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub input_value: String,
    pub placeholder: String,
    pub on_change: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_autocomplete(cx: &mut ComponentCx<UiAutocompleteProps>) -> RSX {
    cx.use_autocomplete(|props: &UiAutocompleteProps| {
        UseAutocompleteProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .input_value(Some(props.input_value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiAutocompleteProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ComboBox
            key="root"
            {...props.autocompleteProps}
            data-slot="autocomplete"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="autocomplete-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Input
                key="input"
                {...props.autocompleteInputProps}
                data-slot="autocomplete-input"
                class="h-11 w-full min-w-0 rounded-md border border-hairline-strong bg-canvas px-4 py-3 text-sm text-ink outline-none transition-colors placeholder:text-mute disabled:pointer-events-none disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm focus-visible:border-ink focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-semantic-error"
            />
            <Slot key="content" />
        </ComboBox>
    )
}
