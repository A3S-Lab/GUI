use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::super::serde_helpers::is_false;
use super::shared::non_empty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseKeyboardInteractionProps {
    on_key_down: Option<String>,
    on_key_up: Option<String>,
    is_disabled: bool,
    is_keyboard_active: bool,
    tab_index: i32,
}

impl Default for UseKeyboardInteractionProps {
    fn default() -> Self {
        Self {
            on_key_down: None,
            on_key_up: None,
            is_disabled: false,
            is_keyboard_active: false,
            tab_index: 0,
        }
    }
}

impl UseKeyboardInteractionProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_key_down(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_key_down = non_empty(action);
        self
    }

    pub fn on_key_up(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_key_up = non_empty(action);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn keyboard_active(mut self, keyboard_active: bool) -> Self {
        self.is_keyboard_active = keyboard_active;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseKeyboardInteractionResult {
    pub is_keyboard_active: bool,
    pub keyboard_interaction_props: KeyboardInteractionProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardInteractionProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_key_down: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_key_up: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-keyboard-active")]
    pub data_keyboard_active: bool,
}

pub fn use_keyboard_interaction(
    props: UseKeyboardInteractionProps,
) -> UseKeyboardInteractionResult {
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };
    UseKeyboardInteractionResult {
        is_keyboard_active: props.is_keyboard_active,
        keyboard_interaction_props: KeyboardInteractionProps {
            tab_index,
            on_key_down: props.on_key_down,
            on_key_up: props.on_key_up,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_keyboard_active: props.is_keyboard_active,
        },
    }
}

pub fn use_keyboard_interaction_value(props: UseKeyboardInteractionProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_keyboard_interaction(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_keyboard_interaction hook did not serialize: {error}"
        ))
    })
}
