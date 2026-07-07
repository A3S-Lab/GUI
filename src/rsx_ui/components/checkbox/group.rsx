use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseCheckboxGroupProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiCheckboxGroupProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub on_change: String,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_checkbox_group(cx: &mut ComponentCx<UiCheckboxGroupProps>) -> RSX {
    cx.use_checkbox_group(|props: &UiCheckboxGroupProps| {
        UseCheckboxGroupProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .on_change(Some(props.on_change.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiCheckboxGroupProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <FieldSet
            key="root"
            {...props.checkboxGroupProps}
            data-slot="checkbox-group"
            class="grid gap-3"
            className={props.className}
            label={props.label}
        >
            <Slot key="content" />
        </FieldSet>
    )
}
