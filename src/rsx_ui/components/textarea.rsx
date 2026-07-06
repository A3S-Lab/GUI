use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiTextareaProps {
    pub class_name: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_read_only: bool,
    pub is_invalid: bool,
    pub rows: String,
    pub cols: String,
    pub max_length: String,
}

pub fn ui_textarea(cx: &mut ComponentCx<UiTextareaProps>) -> RSX {
    cx.use_prop("className", |props: &UiTextareaProps| {
        props.class_name.clone()
    });
    cx.use_prop("value", |props: &UiTextareaProps| props.value.clone());
    cx.use_prop("placeholder", |props: &UiTextareaProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onChange", |props: &UiTextareaProps| {
        props.on_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiTextareaProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiTextareaProps| props.is_required);
    cx.use_prop("isReadOnly", |props: &UiTextareaProps| props.is_read_only);
    cx.use_prop("isInvalid", |props: &UiTextareaProps| props.is_invalid);
    cx.use_prop("rows", |props: &UiTextareaProps| props.rows.clone());
    cx.use_prop("cols", |props: &UiTextareaProps| props.cols.clone());
    cx.use_prop("maxLength", |props: &UiTextareaProps| {
        props.max_length.clone()
    });

    crate::rsx!(
        <textarea
            key="root"
            data-slot="textarea"
            class="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
            className={props.className}
            value={props.value}
            placeholder={props.placeholder}
            onInput={props.onChange}
            disabled={props.isDisabled}
            required={props.isRequired}
            readonly={props.isReadOnly}
            aria-invalid={props.isInvalid}
            rows={props.rows}
            cols={props.cols}
            maxLength={props.maxLength}
        />
    )
}
