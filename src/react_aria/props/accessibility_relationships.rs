use crate::accessibility::AccessibilityRelationshipProps;

use super::AriaProps;

impl AriaProps {
    pub fn accessibility_relationships(
        mut self,
        accessibility_relationships: AccessibilityRelationshipProps,
    ) -> Self {
        self.accessibility_relationships = accessibility_relationships;
        self
    }

    pub fn labelled_by(mut self, labelled_by: impl Into<String>) -> Self {
        self.accessibility_relationships.labelled_by = Some(labelled_by.into());
        self
    }

    pub fn described_by(mut self, described_by: impl Into<String>) -> Self {
        self.accessibility_relationships.described_by = Some(described_by.into());
        self
    }

    pub fn accessibility_details(mut self, details: impl Into<String>) -> Self {
        self.accessibility_relationships.details = Some(details.into());
        self
    }

    pub fn accessibility_controls(mut self, controls: impl Into<String>) -> Self {
        self.accessibility_relationships.controls = Some(controls.into());
        self
    }

    pub fn accessibility_owns(mut self, owns: impl Into<String>) -> Self {
        self.accessibility_relationships.owns = Some(owns.into());
        self
    }

    pub fn flow_to(mut self, flow_to: impl Into<String>) -> Self {
        self.accessibility_relationships.flow_to = Some(flow_to.into());
        self
    }

    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.accessibility_relationships.error_message = Some(error_message.into());
        self
    }

    pub fn active_descendant(mut self, active_descendant: impl Into<String>) -> Self {
        self.accessibility_relationships.active_descendant = Some(active_descendant.into());
        self
    }
}
