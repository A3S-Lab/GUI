use crate::rsx_app::{ComponentCx, RSX};

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
    cx.use_prop("className", |props: &UiTextFieldProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiTextFieldProps| props.label.clone());
    cx.use_prop("value", |props: &UiTextFieldProps| props.value.clone());
    cx.use_prop("placeholder", |props: &UiTextFieldProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onChange", |props: &UiTextFieldProps| {
        props.on_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiTextFieldProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiTextFieldProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiTextFieldProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiTextFieldProps| props.is_read_only);

    crate::rsx!(
        <TextField
            key="root"
            data-slot="text-field"
            class="grid gap-2"
            className={props.className}
            label={props.label}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            readonly={props.isReadOnly}
        >
            <Label
                key="label"
                data-slot="text-field-label"
                class="text-sm font-medium leading-none text-foreground"
                label={props.label}
            />
            <Input
                key="input"
                data-slot="text-field-input"
                class="h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs outline-none transition-[color,box-shadow] placeholder:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive"
                value={props.value}
                placeholder={props.placeholder}
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
