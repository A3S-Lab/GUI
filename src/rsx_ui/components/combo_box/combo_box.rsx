use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseComboBoxProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiComboBoxProps {
    pub class_name: String,
    pub label: String,
    pub value: String,
    pub input_value: String,
    pub placeholder: String,
    pub on_change: String,
    pub on_selection_change: String,
    pub on_open_change: String,
    pub is_open: bool,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub selection_mode: String,
}

pub fn ui_combo_box(cx: &mut ComponentCx<UiComboBoxProps>) -> RSX {
    cx.use_combo_box(|props: &UiComboBoxProps| {
        UseComboBoxProps::new()
            .label(Some(props.label.clone()))
            .value(Some(props.value.clone()))
            .input_value(Some(props.input_value.clone()))
            .placeholder(Some(props.placeholder.clone()))
            .on_change(Some(props.on_change.clone()))
            .on_selection_change(Some(props.on_selection_change.clone()))
            .on_open_change(Some(props.on_open_change.clone()))
            .open(props.is_open)
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(Some(props.selection_mode.clone()))
    });
    cx.use_prop("className", |props: &UiComboBoxProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <ComboBox
            key="root"
            {...props.comboBoxProps}
            data-slot="combo-box"
            class="grid gap-2"
            className={props.className}
        >
            <Label
                key="label"
                data-slot="combo-box-label"
                class="text-sm font-medium leading-none text-ink"
                label={props.label}
            />
            <Group
                key="control"
                data-slot="combo-box-control"
                class="flex h-9 w-full min-w-0 items-center rounded-md border border-hairline-strong bg-surface-card focus-within:border-ink focus-within:ring-[2px] focus-within:ring-ink/40"
            >
                <Input
                    key="input"
                    {...props.comboBoxInputProps}
                    data-slot="combo-box-input"
                    class="min-w-0 flex-1 bg-transparent px-3 py-1.5 text-sm text-ink outline-none placeholder:text-muted md:text-sm"
                />
                <Button
                    key="trigger"
                    {...props.comboBoxTriggerProps}
                    data-slot="combo-box-trigger"
                    class="inline-flex h-9 shrink-0 items-center justify-center px-3 text-body hover:text-ink"
                >
                    Toggle
                </Button>
            </Group>
            <Slot key="content" />
        </ComboBox>
    )
}
