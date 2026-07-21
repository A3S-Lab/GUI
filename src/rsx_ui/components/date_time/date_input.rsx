use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDateInputProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDateInputProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_date_input(cx: &mut ComponentCx<UiDateInputProps>) -> RSX {
    cx.use_date_input(|props: &UiDateInputProps| {
        UseDateInputProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiDateInputProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dateInputProps}
            data-slot="date-input"
            class="flex h-9 w-full min-w-0 items-center rounded-md border border-hairline-strong bg-surface-card focus-within:border-ink focus-within:ring-[2px] focus-within:ring-ink/40"
            className={props.className}
        >
            <Slot key="content" />
        </Group>
    )
}
