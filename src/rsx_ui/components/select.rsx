use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiSelectProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_selection_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_select(cx: &mut ComponentCx<UiSelectProps>) -> RSX {
    cx.use_prop("className", |props: &UiSelectProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiSelectProps| props.label.clone());
    cx.use_prop("value", |props: &UiSelectProps| props.value.clone());
    cx.use_prop("placeholder", |props: &UiSelectProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onSelectionChange", |props: &UiSelectProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiSelectProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiSelectProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiSelectProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiSelectProps| props.is_read_only);

    crate::rsx!(
        <Select
            key="root"
            data-slot="select"
            class="grid gap-2"
            className={props.className}
            label={props.label}
            value={props.value}
            placeholder={props.placeholder}
            onSelectionChange={props.onSelectionChange}
            disabled={props.isDisabled}
            required={props.isRequired}
            aria-invalid={props.isInvalid}
            readonly={props.isReadOnly}
        >
            <Slot key="content" />
        </Select>
    )
}
