use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

use super::serde_helpers::is_false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverlayTriggerKind {
    #[default]
    None,
    Dialog,
    Menu,
    ListBox,
    Tooltip,
    Tree,
}

impl OverlayTriggerKind {
    fn from_option(value: Option<impl Into<String>>) -> Self {
        match value
            .map(Into::into)
            .map(|value| value.to_ascii_lowercase())
            .as_deref()
        {
            Some("dialog") => Self::Dialog,
            Some("menu") => Self::Menu,
            Some("listbox") | Some("list-box") => Self::ListBox,
            Some("tooltip") => Self::Tooltip,
            Some("tree") => Self::Tree,
            _ => Self::None,
        }
    }

    fn aria_haspopup(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Dialog => Some("dialog"),
            Self::Menu => Some("menu"),
            Self::ListBox => Some("listbox"),
            Self::Tooltip => Some("true"),
            Self::Tree => Some("tree"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseOverlayProps {
    is_open: bool,
    on_open_change: Option<String>,
    on_close: Option<String>,
    is_disabled: bool,
    trigger_kind: OverlayTriggerKind,
    is_managed: bool,
    is_modal: bool,
    is_underlay: bool,
    is_dismissable: bool,
    is_keyboard_dismiss_disabled: bool,
    should_close_on_blur: bool,
    contain_focus: bool,
    restore_focus: bool,
    auto_focus: bool,
}

impl Default for UseOverlayProps {
    fn default() -> Self {
        Self {
            is_open: false,
            on_open_change: None,
            on_close: None,
            is_disabled: false,
            trigger_kind: OverlayTriggerKind::None,
            is_managed: true,
            is_modal: false,
            is_underlay: false,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: false,
            should_close_on_blur: false,
            contain_focus: false,
            restore_focus: false,
            auto_focus: false,
        }
    }
}

impl UseOverlayProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn on_open_change(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_open_change = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn on_close(mut self, action: Option<impl Into<String>>) -> Self {
        self.on_close = action.map(Into::into).filter(|action| !action.is_empty());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn trigger_kind(mut self, trigger_kind: Option<impl Into<String>>) -> Self {
        self.trigger_kind = OverlayTriggerKind::from_option(trigger_kind);
        self
    }

    pub fn managed(mut self, managed: bool) -> Self {
        self.is_managed = managed;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.is_modal = modal;
        self
    }

    pub fn underlay(mut self, underlay: bool) -> Self {
        self.is_underlay = underlay;
        self
    }

    pub fn dismissable(mut self, dismissable: bool) -> Self {
        self.is_dismissable = dismissable;
        self
    }

    pub fn keyboard_dismiss_disabled(mut self, disabled: bool) -> Self {
        self.is_keyboard_dismiss_disabled = disabled;
        self
    }

    pub fn close_on_blur(mut self, close: bool) -> Self {
        self.should_close_on_blur = close;
        self
    }

    pub fn contain_focus(mut self, contain: bool) -> Self {
        self.contain_focus = contain;
        self
    }

    pub fn restore_focus(mut self, restore: bool) -> Self {
        self.restore_focus = restore;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseOverlayResult {
    pub is_open: bool,
    pub overlay_props: OverlayProps,
    pub overlay_trigger_props: OverlayTriggerProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayProps {
    pub open: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "data-overlay", skip_serializing_if = "is_false")]
    pub data_overlay: bool,
    #[serde(rename = "data-overlay-modal", skip_serializing_if = "is_false")]
    pub data_overlay_modal: bool,
    #[serde(rename = "data-overlay-underlay", skip_serializing_if = "is_false")]
    pub data_overlay_underlay: bool,
    #[serde(rename = "data-overlay-dismissable", skip_serializing_if = "is_false")]
    pub data_overlay_dismissable: bool,
    #[serde(
        rename = "data-overlay-keyboard-dismiss-disabled",
        skip_serializing_if = "is_false"
    )]
    pub data_overlay_keyboard_dismiss_disabled: bool,
    #[serde(
        rename = "data-overlay-close-on-blur",
        skip_serializing_if = "is_false"
    )]
    pub data_overlay_close_on_blur: bool,
    #[serde(rename = "data-focus-scope", skip_serializing_if = "is_false")]
    pub data_focus_scope: bool,
    #[serde(rename = "data-contain", skip_serializing_if = "is_false")]
    pub data_contain: bool,
    #[serde(rename = "data-restore-focus", skip_serializing_if = "is_false")]
    pub data_restore_focus: bool,
    #[serde(rename = "data-auto-focus", skip_serializing_if = "is_false")]
    pub data_auto_focus: bool,
    #[serde(rename = "aria-modal", skip_serializing_if = "is_false")]
    pub aria_modal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_open_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_close: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayTriggerProps {
    pub role: &'static str,
    #[serde(rename = "tabIndex")]
    pub tab_index: i32,
    #[serde(rename = "aria-expanded")]
    pub aria_expanded: bool,
    #[serde(rename = "data-open")]
    pub data_open: bool,
    #[serde(rename = "data-overlay-trigger", skip_serializing_if = "is_false")]
    pub data_overlay_trigger: bool,
    #[serde(rename = "aria-haspopup", skip_serializing_if = "Option::is_none")]
    pub aria_haspopup: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_press: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(rename = "aria-disabled", skip_serializing_if = "is_false")]
    pub aria_disabled: bool,
}

pub fn use_overlay(props: UseOverlayProps) -> UseOverlayResult {
    let trigger_action = props
        .on_open_change
        .clone()
        .or_else(|| props.on_close.clone());

    let managed_open = props.is_managed && props.is_open;
    UseOverlayResult {
        is_open: props.is_open,
        overlay_props: OverlayProps {
            open: props.is_open,
            data_open: props.is_open,
            data_overlay: managed_open,
            data_overlay_modal: managed_open && props.is_modal,
            data_overlay_underlay: managed_open && props.is_underlay,
            data_overlay_dismissable: managed_open && props.is_dismissable,
            data_overlay_keyboard_dismiss_disabled: managed_open
                && props.is_keyboard_dismiss_disabled,
            data_overlay_close_on_blur: managed_open && props.should_close_on_blur,
            data_focus_scope: managed_open
                && (props.contain_focus || props.restore_focus || props.auto_focus),
            data_contain: managed_open && props.contain_focus,
            data_restore_focus: managed_open && props.restore_focus,
            data_auto_focus: managed_open && props.auto_focus,
            aria_modal: managed_open && props.is_modal,
            on_open_change: props.on_open_change,
            on_close: props.on_close,
        },
        overlay_trigger_props: OverlayTriggerProps {
            role: "button",
            tab_index: 0,
            aria_expanded: props.is_open,
            data_open: props.is_open,
            data_overlay_trigger: props.trigger_kind != OverlayTriggerKind::None,
            aria_haspopup: props.trigger_kind.aria_haspopup(),
            on_press: trigger_action,
            disabled: props.is_disabled,
            aria_disabled: props.is_disabled,
        },
    }
}

pub fn use_overlay_value(props: UseOverlayProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_overlay(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_overlay hook did not serialize: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managed_overlay_serializes_runtime_and_focus_contract() {
        let result = use_overlay(
            UseOverlayProps::new()
                .open(true)
                .on_close(Some("close"))
                .modal(true)
                .underlay(true)
                .dismissable(true)
                .keyboard_dismiss_disabled(true)
                .close_on_blur(true)
                .contain_focus(true)
                .restore_focus(true)
                .auto_focus(true),
        );

        assert!(result.overlay_props.data_overlay);
        assert!(result.overlay_props.data_overlay_modal);
        assert!(result.overlay_props.data_overlay_underlay);
        assert!(result.overlay_props.data_overlay_dismissable);
        assert!(result.overlay_props.data_overlay_keyboard_dismiss_disabled);
        assert!(result.overlay_props.data_overlay_close_on_blur);
        assert!(result.overlay_props.data_focus_scope);
        assert!(result.overlay_props.data_contain);
        assert!(result.overlay_props.data_restore_focus);
        assert!(result.overlay_props.data_auto_focus);
        assert!(result.overlay_props.aria_modal);
        assert_eq!(result.overlay_props.on_close.as_deref(), Some("close"));
    }

    #[test]
    fn closed_or_unmanaged_overlay_does_not_join_the_runtime_stack() {
        let closed = use_overlay(
            UseOverlayProps::new()
                .modal(true)
                .restore_focus(true)
                .auto_focus(true),
        );
        let unmanaged = use_overlay(
            UseOverlayProps::new()
                .open(true)
                .managed(false)
                .modal(true)
                .restore_focus(true),
        );

        assert!(!closed.overlay_props.data_overlay);
        assert!(!closed.overlay_props.data_focus_scope);
        assert!(!unmanaged.overlay_props.data_overlay);
        assert!(!unmanaged.overlay_props.data_focus_scope);
    }
}
