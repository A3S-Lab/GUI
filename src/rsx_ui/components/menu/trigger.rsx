use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::{UseButtonProps, UseOverlayProps};

#[derive(Debug, Clone, PartialEq)]
pub struct UiMenuTriggerProps {
    pub on_press: Option<String>,
    pub on_press_start: Option<String>,
    pub on_press_end: Option<String>,
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_open: bool,
    pub class_name: String,
    pub action_value: String,
    pub action_payload: JsonValue,
}

impl Default for UiMenuTriggerProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            is_disabled: false,
            is_pressed: false,
            is_open: false,
            class_name: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
        }
    }
}

pub fn ui_menu_trigger(cx: &mut ComponentCx<UiMenuTriggerProps>) -> RSX {
    cx.use_overlay(|props: &UiMenuTriggerProps| {
        UseOverlayProps::new()
            .open(props.is_open)
            .on_open_change(props.on_press.clone())
            .disabled(props.is_disabled)
            .trigger_kind(Some("menu"))
    });
    cx.use_button(|props: &UiMenuTriggerProps| {
        UseButtonProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
    });
    cx.use_prop("className", |props: &UiMenuTriggerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <button
            key="root"
            {...props.buttonProps}
            {...props.overlayTriggerProps}
            data-slot="menu-trigger"
            data-pressed={props.isPressed}
            class="inline-flex h-10 items-center justify-center gap-2 rounded-md border border-hairline-strong bg-canvas px-[18px] text-sm font-medium text-ink transition-colors active:bg-surface-strong disabled:pointer-events-none disabled:text-muted-soft focus-visible:ring-[3px] focus-visible:ring-ring/50"
            className={props.className}
            aria-haspopup="menu"
        >
            <Slot key="content" />
        </button>
    )
}
