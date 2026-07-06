use crate::rsx_app::{ComponentCx, RSX};

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
}

pub fn ui_autocomplete(cx: &mut ComponentCx<UiAutocompleteProps>) -> RSX {
    cx.use_prop("className", |props: &UiAutocompleteProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiAutocompleteProps| props.label.clone());
    cx.use_prop("value", |props: &UiAutocompleteProps| props.value.clone());
    cx.use_prop("inputValue", |props: &UiAutocompleteProps| {
        props.input_value.clone()
    });
    cx.use_prop("placeholder", |props: &UiAutocompleteProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onChange", |props: &UiAutocompleteProps| {
        props.on_change.clone()
    });
    cx.use_prop("onSelectionChange", |props: &UiAutocompleteProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiAutocompleteProps| {
        props.is_disabled
    });
    cx.use_prop("isRequired", |props: &UiAutocompleteProps| {
        props.is_required
    });
    cx.use_prop("isInvalid", |props: &UiAutocompleteProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiAutocompleteProps| {
        props.is_read_only
    });

    crate::rsx!(
        <ComboBox
            key="root"
            data-slot="autocomplete"
            class="grid gap-2"
            className={props.className}
            label={props.label}
            value={props.value}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            readonly={props.isReadOnly}
        >
            <Label
                key="label"
                data-slot="autocomplete-label"
                class="text-sm font-medium leading-none text-foreground"
                label={props.label}
            />
            <Input
                key="input"
                data-slot="autocomplete-input"
                class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive"
                value={props.inputValue}
                placeholder={props.placeholder}
                onInput={props.onChange}
                disabled={props.isDisabled}
                required={props.isRequired}
                readonly={props.isReadOnly}
                aria-invalid={props.isInvalid}
            />
            <Slot key="content" />
        </ComboBox>
    )
}
