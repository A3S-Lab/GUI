use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{UseButtonProps, UseOverlayProps};

#[derive(Debug, Clone, PartialEq)]
pub struct UiDialogTriggerProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub on_press_up: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_open: bool,
    pub class_name: String,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiDialogTriggerProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            is_disabled: false,
            is_pressed: false,
            is_open: false,
            class_name: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_dialog_trigger(cx: &mut ComponentCx<UiDialogTriggerProps>) -> RSX {
    cx.use_overlay(|props: &UiDialogTriggerProps| {
        UseOverlayProps::new()
            .open(props.is_open)
            .on_open_change(props.on_press.clone())
            .disabled(props.is_disabled)
            .trigger_kind(Some("dialog"))
    });
    cx.use_button(|props: &UiDialogTriggerProps| {
        UseButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiDialogTriggerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            {...props.overlayTriggerProps}
            data-slot="dialog-trigger"
            data-pressed={props.isPressed}
            class="inline-flex h-10 items-center justify-center gap-2 rounded-md border border-primary bg-primary px-[18px] py-2 text-sm font-medium leading-none text-on-primary transition-colors active:bg-primary-active disabled:pointer-events-none disabled:bg-surface-strong disabled:text-muted-soft focus-visible:ring-[3px] focus-visible:ring-ring/50"
            className={props.className}
            aria-haspopup="dialog"
        >
            <Slot key="content" />
        </button>
    )
}
