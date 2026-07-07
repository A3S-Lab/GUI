use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseDateRangePickerProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiDateRangePickerProps {
    pub class_name: String,
    pub label: String,
    pub start_value: String,
    pub end_value: String,
    pub placeholder: String,
    pub on_start_change: String,
    pub on_end_change: String,
    pub on_open_change: String,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
}

pub fn ui_date_range_picker(cx: &mut ComponentCx<UiDateRangePickerProps>) -> RSX {
    cx.use_date_range_picker(|props: &UiDateRangePickerProps| {
        UseDateRangePickerProps::new()
            .label(Some(props.label.clone()))
            .start_value(Some(props.start_value.clone()))
            .end_value(Some(props.end_value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_start_change(Some(props.on_start_change.clone()))
            .on_end_change(Some(props.on_end_change.clone()))
            .on_open_change(Some(props.on_open_change.clone()))
            .open(props.is_open)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
    });
    cx.use_prop("className", |props: &UiDateRangePickerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <Group
            key="root"
            {...props.dateRangePickerProps}
            data-slot="date-range-picker"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="date-range-picker-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Group
                key="control"
                data-slot="date-range-picker-control"
                class="flex h-11 w-full min-w-0 items-center gap-2 rounded-md border border-hairline-strong bg-canvas px-4 focus-within:border-ink focus-within:ring-[3px] focus-within:ring-ring/50"
            >
                <Input
                    key="start-input"
                    {...props.dateRangePickerStartInputProps}
                    data-slot="date-range-picker-start-input"
                    class="min-w-0 flex-1 bg-transparent px-1 py-3 text-sm text-ink outline-none placeholder:text-mute md:text-sm"
                />
                <Text key="separator" data-slot="date-range-picker-separator">-</Text>
                <Input
                    key="end-input"
                    {...props.dateRangePickerEndInputProps}
                    data-slot="date-range-picker-end-input"
                    class="min-w-0 flex-1 bg-transparent px-1 py-3 text-sm text-ink outline-none placeholder:text-mute md:text-sm"
                />
                <Button
                    key="trigger"
                    {...props.dateRangePickerTriggerProps}
                    data-slot="date-range-picker-trigger"
                    class="inline-flex h-11 shrink-0 items-center justify-center px-3 text-body transition-colors hover:text-ink"
                >
                    Open
                </Button>
            </Group>
            <Slot key="content" />
        </Group>
    )
}
