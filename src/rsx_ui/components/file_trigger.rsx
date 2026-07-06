use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UsePressProps;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiFileTriggerProps {
    pub class_name: String,
    pub on_press: Option<String>,
    pub on_select: String,
    pub accepted_file_types: String,
    pub allows_multiple: bool,
    pub is_disabled: bool,
    pub is_pressed: bool,
}

pub fn ui_file_trigger(cx: &mut ComponentCx<UiFileTriggerProps>) -> RSX {
    cx.use_button(|props: &UiFileTriggerProps| {
        UsePressProps::new()
            .on_press(props.on_press.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiFileTriggerProps| {
        props.class_name.clone()
    });
    cx.use_prop("onSelect", |props: &UiFileTriggerProps| {
        props.on_select.clone()
    });
    cx.use_prop("acceptedFileTypes", |props: &UiFileTriggerProps| {
        props.accepted_file_types.clone()
    });
    cx.use_prop("allowsMultiple", |props: &UiFileTriggerProps| {
        props.allows_multiple
    });

    crate::rsx!(
        <button
            key="root"
            {...props.pressProps}
            data-slot="file-trigger"
            data-pressed={props.isPressed}
            class="inline-flex h-9 items-center justify-center gap-2 rounded-md border border-input bg-background px-3 text-sm font-medium shadow-xs outline-none transition-[color,box-shadow] hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            accept={props.acceptedFileTypes}
            multiple={props.allowsMultiple}
            onSelect={props.onSelect}
        >
            <Slot key="content" />
        </button>
    )
}
