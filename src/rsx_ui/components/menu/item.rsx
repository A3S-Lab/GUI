use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};
use crate::semantic_ui::UseMenuItemProps;

#[derive(Debug, Clone, PartialEq)]
pub struct UiMenuItemProps {
    pub class_name: String,
    pub on_action: String,
    pub action_value: String,
    pub action_payload: JsonValue,
    pub text_value: String,
    pub is_disabled: bool,
    pub is_selected: bool,
}

impl Default for UiMenuItemProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            on_action: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
            text_value: String::new(),
            is_disabled: false,
            is_selected: false,
        }
    }
}

pub fn ui_menu_item(cx: &mut ComponentCx<UiMenuItemProps>) -> RSX {
    cx.use_menu_item(|props: &UiMenuItemProps| {
        UseMenuItemProps::new()
            .text_value(Some(props.text_value.clone()))
            .action_value(Some(props.action_value.clone()))
            .on_action(Some(props.on_action.clone()))
            .disabled(props.is_disabled)
            .selected(props.is_selected)
    });
    cx.use_prop("className", |props: &UiMenuItemProps| {
        props.class_name.clone()
    });
    cx.use_prop("actionPayload", |props: &UiMenuItemProps| {
        props.action_payload.clone()
    });

    crate::rsx!(
        <MenuItem
            key="root"
            {...props.menuItemProps}
            data-slot="menu-item"
            class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-surface-strong focus:text-ink disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            actionPayload={props.actionPayload}
        >
            <Slot key="content" />
        </MenuItem>
    )
}
