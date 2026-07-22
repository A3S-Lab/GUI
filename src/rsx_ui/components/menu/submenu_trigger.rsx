use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseSubmenuTriggerProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiSubmenuTriggerProps {
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

impl Default for UiSubmenuTriggerProps {
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

pub fn ui_submenu_trigger(cx: &mut ComponentCx<UiSubmenuTriggerProps>) -> RSX {
    cx.use_submenu_trigger(|props: &UiSubmenuTriggerProps| {
        UseSubmenuTriggerProps::new()
            .on_press(props.on_press.clone())
            .on_press_start(props.on_press_start.clone())
            .on_press_end(props.on_press_end.clone())
            .on_press_up(props.on_press_up.clone())
            .action_value(Some(props.action_value.clone()))
            .action_payload(props.action_payload.clone())
            .disabled(props.is_disabled)
            .pressed(props.is_pressed)
            .open(props.is_open)
    });
    cx.use_prop("className", |props: &UiSubmenuTriggerProps| {
        props.class_name.clone()
    });

    crate::rsx!(
        <MenuItem
            key="root"
            {...props.submenuTriggerProps}
            data-slot="submenu-trigger"
            data-pressed={props.isPressed}
            data-open={props.isOpen}
            class="relative flex cursor-default select-none items-center justify-between gap-4 rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-surface-strong focus:text-ink disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
        >
            <Slot key="content" />
        </MenuItem>
    )
}
