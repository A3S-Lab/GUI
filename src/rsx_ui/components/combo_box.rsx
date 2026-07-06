use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiComboBoxProps {
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

pub fn ui_combo_box(cx: &mut ComponentCx<UiComboBoxProps>) -> RSX {
    cx.use_prop("className", |props: &UiComboBoxProps| {
        props.class_name.clone()
    });
    cx.use_prop("label", |props: &UiComboBoxProps| props.label.clone());
    cx.use_prop("value", |props: &UiComboBoxProps| props.value.clone());
    cx.use_prop("inputValue", |props: &UiComboBoxProps| {
        props.input_value.clone()
    });
    cx.use_prop("placeholder", |props: &UiComboBoxProps| {
        props.placeholder.clone()
    });
    cx.use_prop("onChange", |props: &UiComboBoxProps| {
        props.on_change.clone()
    });
    cx.use_prop("onSelectionChange", |props: &UiComboBoxProps| {
        props.on_selection_change.clone()
    });
    cx.use_prop("isDisabled", |props: &UiComboBoxProps| props.is_disabled);
    cx.use_prop("isRequired", |props: &UiComboBoxProps| props.is_required);
    cx.use_prop("isInvalid", |props: &UiComboBoxProps| props.is_invalid);
    cx.use_prop("isReadOnly", |props: &UiComboBoxProps| props.is_read_only);

    crate::rsx!(
        <ComboBox
            key="root"
            data-slot="combo-box"
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
                data-slot="combo-box-label"
                class="text-sm font-medium leading-none text-foreground"
                label={props.label}
            />
            <Group
                key="control"
                data-slot="combo-box-control"
                class="flex h-9 w-full min-w-0 items-center rounded-md border border-input bg-transparent shadow-xs focus-within:border-ring focus-within:ring-[3px] focus-within:ring-ring/50"
            >
                <Input
                    key="input"
                    data-slot="combo-box-input"
                    class="min-w-0 flex-1 bg-transparent px-3 py-1 text-base outline-none placeholder:text-muted-foreground md:text-sm"
                    value={props.inputValue}
                    placeholder={props.placeholder}
                    onInput={props.onChange}
                    disabled={props.isDisabled}
                    required={props.isRequired}
                    readonly={props.isReadOnly}
                    aria-invalid={props.isInvalid}
                />
                <Button
                    key="trigger"
                    data-slot="combo-box-trigger"
                    class="inline-flex h-8 shrink-0 items-center justify-center px-2 text-muted-foreground transition-colors hover:text-foreground"
                    disabled={props.isDisabled}
                >
                    Toggle
                </Button>
            </Group>
            <Slot key="content" />
        </ComboBox>
    )
}
