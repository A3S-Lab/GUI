use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseMenuProps {
    label: Option<String>,
    is_disabled: bool,
}

impl UseMenuProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseMenuResult {
    pub menu_props: MenuProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "aria-label", skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseMenuItemProps {
    text_value: Option<String>,
    action_value: Option<String>,
    on_action: Option<String>,
    is_disabled: bool,
    is_selected: bool,
}

impl UseMenuItemProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_value(mut self, text_value: Option<impl Into<String>>) -> Self {
        self.text_value = text_value
            .map(Into::into)
            .filter(|text_value| !text_value.is_empty());
        self
    }

    pub fn action_value(mut self, action_value: Option<impl Into<String>>) -> Self {
        self.action_value = action_value
            .map(Into::into)
            .filter(|action_value| !action_value.is_empty());
        self
    }

    pub fn on_action(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_action = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseSubmenuTriggerProps {
    on_press: Option<String>,
    on_press_start: Option<String>,
    on_press_end: Option<String>,
    action_value: Option<String>,
    action_payload: JsonValue,
    is_disabled: bool,
    is_pressed: bool,
    is_open: bool,
}

impl Default for UseSubmenuTriggerProps {
    fn default() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            action_value: None,
            action_payload: JsonValue::Null,
            is_disabled: false,
            is_pressed: false,
            is_open: false,
        }
    }
}

impl UseSubmenuTriggerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press = non_empty(action);
        self
    }

    pub fn on_press_start(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_start = non_empty(action);
        self
    }

    pub fn on_press_end(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_press_end = non_empty(action);
        self
    }

    pub fn action_value(mut self, action_value: Option<impl Into<String>>) -> Self {
        self.action_value = non_empty(action_value);
        self
    }

    pub fn action_payload(mut self, action_payload: JsonValue) -> Self {
        self.action_payload = action_payload;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.is_pressed = pressed;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseMenuItemResult {
    pub is_disabled: bool,
    pub is_selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    pub menu_item_props: MenuItemProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuItemProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "aria-selected", skip_serializing_if = "is_false")]
    pub aria_selected: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub is_selected: bool,
    #[serde(rename = "data-selected")]
    pub data_selected: bool,
    #[serde(rename = "data-disabled")]
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseSubmenuTriggerResult {
    pub is_disabled: bool,
    pub is_pressed: bool,
    pub is_open: bool,
    pub submenu_trigger_props: SubmenuTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmenuTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press_end: Option<String>,
    #[serde(rename = "actionValue", skip_serializing_if = "Option::is_none")]
    pub action_value: Option<String>,
    #[serde(rename = "actionPayload", skip_serializing_if = "JsonValue::is_null")]
    pub action_payload: JsonValue,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "aria-haspopup")]
    pub aria_haspopup: &'static str,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "data-pressed")]
    pub data_pressed: bool,
}

pub fn use_menu(props: UseMenuProps) -> UseMenuResult {
    UseMenuResult {
        label: props.label.clone(),
        menu_props: MenuProps {
            role: "menu",
            label: props.label.clone(),
            aria_label: props.label,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_menu_item(props: UseMenuItemProps) -> UseMenuItemResult {
    UseMenuItemResult {
        is_disabled: props.is_disabled,
        is_selected: props.is_selected,
        text_value: props.text_value.clone(),
        menu_item_props: MenuItemProps {
            role: "menuitem",
            tab_index: -1,
            text_value: props.text_value,
            action_value: props.action_value,
            on_press: props.on_action,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            aria_selected: props.is_selected,
            is_selected: props.is_selected,
            data_selected: props.is_selected,
            data_disabled: props.is_disabled,
        },
    }
}

pub fn use_submenu_trigger(props: UseSubmenuTriggerProps) -> UseSubmenuTriggerResult {
    UseSubmenuTriggerResult {
        is_disabled: props.is_disabled,
        is_pressed: props.is_pressed,
        is_open: props.is_open,
        submenu_trigger_props: SubmenuTriggerProps {
            role: "menuitem",
            tab_index: -1,
            on_press: props.on_press,
            on_press_start: props.on_press_start,
            on_press_end: props.on_press_end,
            action_value: props.action_value,
            action_payload: props.action_payload,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            aria_haspopup: "menu",
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_pressed: props.is_pressed,
        },
    }
}

pub fn use_menu_value(props: UseMenuProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_menu(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_menu hook did not serialize: {error}"))
    })
}

pub fn use_menu_item_value(props: UseMenuItemProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_menu_item(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_menu_item hook did not serialize: {error}"
        ))
    })
}

pub fn use_submenu_trigger_value(props: UseSubmenuTriggerProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_submenu_trigger(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_submenu_trigger hook did not serialize: {error}"
        ))
    })
}

fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
