use serde_json::Value as JsonValue;

use crate::rsx_app::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq)]
pub struct UiMenuItemProps {
    pub class_name: String,
    pub on_action: String,
    pub action_value: String,
    pub action_payload: JsonValue,
    pub is_disabled: bool,
}

impl Default for UiMenuItemProps {
    fn default() -> Self {
        Self {
            class_name: String::new(),
            on_action: String::new(),
            action_value: String::new(),
            action_payload: JsonValue::Null,
            is_disabled: false,
        }
    }
}

pub fn ui_menu_item(cx: &mut ComponentCx<UiMenuItemProps>) -> RSX {
    cx.use_prop("className", |props: &UiMenuItemProps| {
        props.class_name.clone()
    });
    cx.use_prop("onAction", |props: &UiMenuItemProps| {
        props.on_action.clone()
    });
    cx.use_prop("actionValue", |props: &UiMenuItemProps| {
        props.action_value.clone()
    });
    cx.use_prop("actionPayload", |props: &UiMenuItemProps| {
        props.action_payload.clone()
    });
    cx.use_prop("isDisabled", |props: &UiMenuItemProps| props.is_disabled);

    crate::rsx!(
        <MenuItem
            key="root"
            data-slot="menu-item"
            class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
            className={props.className}
            onPress={props.onAction}
            actionValue={props.actionValue}
            actionPayload={props.actionPayload}
            disabled={props.isDisabled}
        >
            <Slot key="content" />
        </MenuItem>
    )
}
