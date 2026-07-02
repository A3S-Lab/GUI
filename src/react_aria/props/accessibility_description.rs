use crate::accessibility::AccessibilityDescriptionProps;

use super::AriaProps;

impl AriaProps {
    pub fn accessibility_description(
        mut self,
        accessibility_description: AccessibilityDescriptionProps,
    ) -> Self {
        self.accessibility_description = accessibility_description;
        self
    }

    pub fn accessibility_description_text(mut self, description: impl Into<String>) -> Self {
        self.accessibility_description.description = Some(description.into());
        self
    }

    pub fn role_description(mut self, role_description: impl Into<String>) -> Self {
        self.accessibility_description.role_description = Some(role_description.into());
        self
    }

    pub fn key_shortcuts(mut self, key_shortcuts: impl Into<String>) -> Self {
        self.accessibility_description.key_shortcuts = Some(key_shortcuts.into());
        self
    }

    pub fn value_text(mut self, value_text: impl Into<String>) -> Self {
        self.accessibility_description.value_text = Some(value_text.into());
        self
    }
}
