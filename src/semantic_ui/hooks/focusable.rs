use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFocusableProps {
    on_focus: Option<String>,
    on_blur: Option<String>,
    on_focus_change: Option<String>,
    is_disabled: bool,
    is_focused: bool,
    auto_focus: bool,
    tab_index: i32,
}

impl Default for UseFocusableProps {
    fn default() -> Self {
        Self {
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            is_disabled: false,
            is_focused: false,
            auto_focus: false,
            tab_index: 0,
        }
    }
}

impl UseFocusableProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_focus(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_blur(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_blur = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_focus_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFocusableResult {
    pub is_focused: bool,
    pub focus_props: FocusProps,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFocusRingProps {
    on_focus: Option<String>,
    on_blur: Option<String>,
    on_focus_change: Option<String>,
    is_disabled: bool,
    is_focused: bool,
    is_focus_visible: bool,
    is_focus_within: bool,
    auto_focus: bool,
    tab_index: i32,
}

impl Default for UseFocusRingProps {
    fn default() -> Self {
        Self {
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            is_disabled: false,
            is_focused: false,
            is_focus_visible: false,
            is_focus_within: false,
            auto_focus: false,
            tab_index: 0,
        }
    }
}

impl UseFocusRingProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_focus(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_blur(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_blur = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_focus_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_focus_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.is_focus_visible = focus_visible;
        self
    }

    pub fn focus_within(mut self, focus_within: bool) -> Self {
        self.is_focus_within = focus_within;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFocusRingResult {
    pub is_focused: bool,
    pub is_focus_visible: bool,
    pub is_focus_within: bool,
    pub focus_ring_props: FocusRingProps,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFocusScopeProps {
    contain: bool,
    restore_focus: bool,
    auto_focus: bool,
    is_disabled: bool,
    tab_index: i32,
}

impl Default for UseFocusScopeProps {
    fn default() -> Self {
        Self {
            contain: false,
            restore_focus: false,
            auto_focus: false,
            is_disabled: false,
            tab_index: -1,
        }
    }
}

impl UseFocusScopeProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contain(mut self, contain: bool) -> Self {
        self.contain = contain;
        self
    }

    pub fn restore_focus(mut self, restore_focus: bool) -> Self {
        self.restore_focus = restore_focus;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn tab_index(mut self, tab_index: i32) -> Self {
        self.tab_index = tab_index;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseFocusScopeResult {
    pub contain: bool,
    pub restore_focus: bool,
    pub auto_focus: bool,
    pub is_disabled: bool,
    pub focus_scope_props: FocusScopeProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_blur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus_change: Option<String>,
    #[serde(rename = "autoFocus", skip_serializing_if = "is_false")]
    pub auto_focus: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-focused")]
    pub data_focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusRingProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_blur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_focus_change: Option<String>,
    #[serde(rename = "autoFocus", skip_serializing_if = "is_false")]
    pub auto_focus: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-focused")]
    pub data_focused: bool,
    #[serde(rename = "data-focus-visible")]
    pub data_focus_visible: bool,
    #[serde(rename = "data-focus-within")]
    pub data_focus_within: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusScopeProps {
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "data-auto-focus", skip_serializing_if = "is_false")]
    pub auto_focus: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub inert: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
    #[serde(rename = "data-focus-scope")]
    pub data_focus_scope: bool,
    #[serde(rename = "data-contain")]
    pub data_contain: bool,
    #[serde(rename = "data-restore-focus")]
    pub data_restore_focus: bool,
}

pub fn use_focusable(props: UseFocusableProps) -> UseFocusableResult {
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };
    UseFocusableResult {
        is_focused: props.is_focused,
        focus_props: FocusProps {
            tab_index,
            on_focus: props.on_focus,
            on_blur: props.on_blur,
            on_focus_change: props.on_focus_change,
            auto_focus: props.auto_focus,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_focused: props.is_focused,
        },
    }
}

pub fn use_focus_ring(props: UseFocusRingProps) -> UseFocusRingResult {
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };
    UseFocusRingResult {
        is_focused: props.is_focused,
        is_focus_visible: props.is_focus_visible,
        is_focus_within: props.is_focus_within,
        focus_ring_props: FocusRingProps {
            tab_index,
            on_focus: props.on_focus,
            on_blur: props.on_blur,
            on_focus_change: props.on_focus_change,
            auto_focus: props.auto_focus,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_focused: props.is_focused,
            data_focus_visible: props.is_focus_visible,
            data_focus_within: props.is_focus_within,
        },
    }
}

pub fn use_focus_scope(props: UseFocusScopeProps) -> UseFocusScopeResult {
    let tab_index = if props.is_disabled {
        -1
    } else {
        props.tab_index
    };
    UseFocusScopeResult {
        contain: props.contain,
        restore_focus: props.restore_focus,
        auto_focus: props.auto_focus,
        is_disabled: props.is_disabled,
        focus_scope_props: FocusScopeProps {
            tab_index,
            auto_focus: props.auto_focus,
            inert: props.is_disabled,
            aria_disabled: props.is_disabled,
            data_focus_scope: true,
            data_contain: props.contain,
            data_restore_focus: props.restore_focus,
        },
    }
}

pub fn use_focusable_value(props: UseFocusableProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_focusable(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_focusable hook did not serialize: {error}"
        ))
    })
}

pub fn use_focus_ring_value(props: UseFocusRingProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_focus_ring(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_focus_ring hook did not serialize: {error}"
        ))
    })
}

pub fn use_focus_scope_value(props: UseFocusScopeProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_focus_scope(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_focus_scope hook did not serialize: {error}"
        ))
    })
}
