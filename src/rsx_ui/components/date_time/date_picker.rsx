use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDatePickerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDatePickerProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub on_change: String,
    pub on_open_change: String,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_date_picker(cx: &mut ComponentCx<UiDatePickerProps>) -> RSX {
    cx.use_date_picker(|props: &UiDatePickerProps| {
        UseDatePickerProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .on_open_change(Some(props.on_open_change.clone()))
            .open(props.is_open)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiDatePickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.datePickerProps}
            data-slot="date-picker"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="date-picker-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Group
                key="control"
                data-slot="date-picker-control"
                class="flex h-9 w-full min-w-0 items-center rounded-md border border-hairline-strong bg-surface-card focus-within:border-ink focus-within:ring-[2px] focus-within:ring-ink/40"
            >
                <Input
                    key="input"
                    {...props.datePickerInputProps}
                    data-slot="date-picker-input"
                    class="min-w-0 flex-1 bg-transparent px-3 py-1.5 text-sm text-ink outline-none placeholder:text-muted md:text-sm"
                />
                <Button
                    key="trigger"
                    {...props.datePickerTriggerProps}
                    data-slot="date-picker-trigger"
                    class="inline-flex h-9 shrink-0 items-center justify-center px-3 text-body hover:text-ink"
                >
                    Open
                </Button>
            </Group>
            <Slot key="content" />
        </Group>
    )
}
