use crate::accessibility::AccessibilityStateProps;

use super::AriaProps;

impl AriaProps {
    pub fn accessibility_state(mut self, accessibility_state: AccessibilityStateProps) -> Self {
        self.accessibility_state = accessibility_state;
        self
    }

    pub fn accessibility_hidden(mut self, hidden: Option<bool>) -> Self {
        self.accessibility_state.hidden = hidden;
        self
    }

    pub fn accessibility_autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.accessibility_state.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn accessibility_multiline(mut self, multiline: Option<bool>) -> Self {
        self.accessibility_state.multiline = multiline;
        self
    }

    pub fn current(mut self, current: impl Into<String>) -> Self {
        self.accessibility_state.current = Some(current.into());
        self
    }

    pub fn has_popup(mut self, has_popup: impl Into<String>) -> Self {
        self.accessibility_state.has_popup = Some(has_popup.into());
        self
    }

    pub fn pressed(mut self, pressed: impl Into<String>) -> Self {
        self.accessibility_state.pressed = Some(pressed.into());
        self
    }

    pub fn live(mut self, live: impl Into<String>) -> Self {
        self.accessibility_state.live = Some(live.into());
        self
    }

    pub fn atomic(mut self, atomic: Option<bool>) -> Self {
        self.accessibility_state.atomic = atomic;
        self
    }

    pub fn busy(mut self, busy: Option<bool>) -> Self {
        self.accessibility_state.busy = busy;
        self
    }

    pub fn relevant(mut self, relevant: impl Into<String>) -> Self {
        self.accessibility_state.relevant = Some(relevant.into());
        self
    }

    pub fn modal(mut self, modal: Option<bool>) -> Self {
        self.accessibility_state.modal = modal;
        self
    }
}
