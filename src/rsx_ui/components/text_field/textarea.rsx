use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseTextFieldProps;

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
    cx.use_text_field(|props: &UiTextareaProps| {
        UseTextFieldProps::new()
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .read_only(props.is_read_only)
            .invalid(props.is_invalid)
    });
    cx.use_prop("className", |props: &UiTextareaProps| {
        props.class_name.clone()
    });
    cx.use_prop("rows", |props: &UiTextareaProps| props.rows.clone());
    cx.use_prop("cols", |props: &UiTextareaProps| props.cols.clone());
    cx.use_prop("maxLength", |props: &UiTextareaProps| {
        props.max_length.clone()
    });

    crate::rsx!(
        <textarea
            key="root"
            {...props.inputProps}
            data-slot="textarea"
            class="border-hairline-strong placeholder:text-muted focus-visible:border-ink focus-visible:ring-ink/40 aria-invalid:border-semantic-error flex field-sizing-content min-h-20 w-full rounded-md border bg-surface-card px-3 py-1.5 text-sm text-ink outline-none focus-visible:ring-[2px] disabled:cursor-not-allowed disabled:bg-surface-strong disabled:text-muted-soft md:text-sm"
            className={props.className}
            rows={props.rows}
            cols={props.cols}
            maxLength={props.maxLength}
        />
    )
}
