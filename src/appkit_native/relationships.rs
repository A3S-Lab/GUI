use super::*;
use crate::accessibility::relationship_registry::{
    AccessibilityRelationshipField, ResolvedAccessibilityRelationships,
};
use crate::accessibility::AccessibilityRelationshipProps;

impl AppKitNativeSurface {
    pub(super) fn set_accessibility_relationships(
        &mut self,
        source: HostNodeId,
        relationships: AccessibilityRelationshipProps,
    ) -> GuiResult<()> {
        let cleared = self
            .accessibility_relationships
            .update_relationships(source, relationships);
        self.reset_accessibility_relationships(source, &cleared);
        self.reconcile_accessibility_relationship_source(source);
        Ok(())
    }

    pub(super) fn set_accessibility_relationship_metadata(
        &mut self,
        node: HostNodeId,
        metadata: &BTreeMap<String, String>,
    ) -> GuiResult<()> {
        if self
            .accessibility_relationships
            .update_metadata(node, metadata)
        {
            self.reconcile_accessibility_relationships();
        }
        Ok(())
    }

    pub(super) fn forget_accessibility_relationship_node(
        &mut self,
        node: HostNodeId,
    ) -> GuiResult<()> {
        if self.accessibility_relationships.remove_node(node) {
            self.reconcile_accessibility_relationships();
        }
        Ok(())
    }

    fn reconcile_accessibility_relationships(&self) {
        for (source, relationships) in self.accessibility_relationships.resolve_all() {
            self.apply_accessibility_relationships(source, &relationships);
        }
    }

    fn reconcile_accessibility_relationship_source(&self, source: HostNodeId) {
        if let Some(relationships) = self.accessibility_relationships.resolve(source) {
            self.apply_accessibility_relationships(source, &relationships);
        }
    }

    fn apply_accessibility_relationships(
        &self,
        source: HostNodeId,
        relationships: &ResolvedAccessibilityRelationships,
    ) {
        let Some(source) = self.widgets.get(&source).and_then(AppKitOsWidget::as_view) else {
            return;
        };
        let Some(labelled_by) = relationships.labelled_by.as_ref() else {
            return;
        };
        let target = if labelled_by.is_single_complete() {
            labelled_by
                .nodes
                .first()
                .and_then(|node| self.widgets.get(node))
                .and_then(appkit_label_accessibility_object)
        } else {
            None
        };
        unsafe {
            source.setAccessibilityTitleUIElement(target);
        }
    }

    fn reset_accessibility_relationships(
        &self,
        source: HostNodeId,
        fields: &[AccessibilityRelationshipField],
    ) {
        if !fields.contains(&AccessibilityRelationshipField::LabelledBy) {
            return;
        }
        let Some(source) = self.widgets.get(&source).and_then(AppKitOsWidget::as_view) else {
            return;
        };
        unsafe {
            source.setAccessibilityTitleUIElement(None);
        }
    }
}

fn appkit_accessibility_object(view: &NSView) -> &AnyObject {
    view.as_super().as_super().as_super()
}

fn appkit_label_accessibility_object(widget: &AppKitOsWidget) -> Option<&AnyObject> {
    match widget {
        AppKitOsWidget::TextField(text_field) if !text_field.isEditable() => Some(
            appkit_accessibility_object(text_field.as_super().as_super()),
        ),
        _ => None,
    }
}
