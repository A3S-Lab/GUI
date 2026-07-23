use super::*;
use crate::accessibility::relationship_registry::{
    AccessibilityRelationshipField, ResolvedAccessibilityReferences,
    ResolvedAccessibilityRelationships,
};
use crate::accessibility::AccessibilityRelationshipProps;

#[derive(Debug, Clone, Copy)]
enum GtkRelationshipKind {
    LabelledBy,
    DescribedBy,
    Details,
    Controls,
    Owns,
    FlowTo,
    ErrorMessage,
}

impl Gtk4NativeSurface {
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
        let Some(source) = self.widgets.get(&source) else {
            return;
        };
        self.apply_reference_list(
            source,
            relationships.labelled_by.as_ref(),
            GtkRelationshipKind::LabelledBy,
        );
        self.apply_reference_list(
            source,
            relationships.described_by.as_ref(),
            GtkRelationshipKind::DescribedBy,
        );
        self.apply_reference_list(
            source,
            relationships.details.as_ref(),
            GtkRelationshipKind::Details,
        );
        self.apply_reference_list(
            source,
            relationships.controls.as_ref(),
            GtkRelationshipKind::Controls,
        );
        self.apply_reference_list(
            source,
            relationships.owns.as_ref(),
            GtkRelationshipKind::Owns,
        );
        self.apply_reference_list(
            source,
            relationships.flow_to.as_ref(),
            GtkRelationshipKind::FlowTo,
        );
        self.apply_reference_list(
            source,
            relationships.error_message.as_ref(),
            GtkRelationshipKind::ErrorMessage,
        );
        if let Some(references) = relationships.active_descendant.as_ref() {
            let target = if references.is_single_complete() {
                references
                    .nodes
                    .first()
                    .and_then(|node| self.widgets.get(node))
            } else {
                None
            };
            if let Some(target) = target {
                source.update_relation(&[gtk::accessible::Relation::ActiveDescendant(
                    target.upcast_ref(),
                )]);
            } else {
                source.reset_relation(gtk::AccessibleRelation::ActiveDescendant);
            }
        }
    }

    fn apply_reference_list(
        &self,
        source: &gtk::Widget,
        references: Option<&ResolvedAccessibilityReferences>,
        kind: GtkRelationshipKind,
    ) {
        let Some(references) = references else {
            return;
        };
        let targets = references
            .nodes
            .iter()
            .filter_map(|node| self.widgets.get(node).cloned())
            .collect::<Vec<_>>();
        if !references.is_complete()
            || targets.len() != references.nodes.len()
            || targets.is_empty()
        {
            source.reset_relation(gtk_accessible_relation(kind));
            return;
        }
        let targets = targets
            .iter()
            .map(|target| target.upcast_ref::<gtk::Accessible>())
            .collect::<Vec<_>>();
        let relation = match kind {
            GtkRelationshipKind::LabelledBy => gtk::accessible::Relation::LabelledBy(&targets),
            GtkRelationshipKind::DescribedBy => gtk::accessible::Relation::DescribedBy(&targets),
            GtkRelationshipKind::Details => gtk::accessible::Relation::Details(&targets),
            GtkRelationshipKind::Controls => gtk::accessible::Relation::Controls(&targets),
            GtkRelationshipKind::Owns => gtk::accessible::Relation::Owns(&targets),
            GtkRelationshipKind::FlowTo => gtk::accessible::Relation::FlowTo(&targets),
            GtkRelationshipKind::ErrorMessage => gtk::accessible::Relation::ErrorMessage(&targets),
        };
        source.update_relation(&[relation]);
    }

    fn reset_accessibility_relationships(
        &self,
        source: HostNodeId,
        fields: &[AccessibilityRelationshipField],
    ) {
        let Some(source) = self.widgets.get(&source) else {
            return;
        };
        for field in fields {
            source.reset_relation(match field {
                AccessibilityRelationshipField::LabelledBy => gtk::AccessibleRelation::LabelledBy,
                AccessibilityRelationshipField::DescribedBy => gtk::AccessibleRelation::DescribedBy,
                AccessibilityRelationshipField::Details => gtk::AccessibleRelation::Details,
                AccessibilityRelationshipField::Controls => gtk::AccessibleRelation::Controls,
                AccessibilityRelationshipField::Owns => gtk::AccessibleRelation::Owns,
                AccessibilityRelationshipField::FlowTo => gtk::AccessibleRelation::FlowTo,
                AccessibilityRelationshipField::ErrorMessage => {
                    gtk::AccessibleRelation::ErrorMessage
                }
                AccessibilityRelationshipField::ActiveDescendant => {
                    gtk::AccessibleRelation::ActiveDescendant
                }
            });
        }
    }
}

fn gtk_accessible_relation(kind: GtkRelationshipKind) -> gtk::AccessibleRelation {
    match kind {
        GtkRelationshipKind::LabelledBy => gtk::AccessibleRelation::LabelledBy,
        GtkRelationshipKind::DescribedBy => gtk::AccessibleRelation::DescribedBy,
        GtkRelationshipKind::Details => gtk::AccessibleRelation::Details,
        GtkRelationshipKind::Controls => gtk::AccessibleRelation::Controls,
        GtkRelationshipKind::Owns => gtk::AccessibleRelation::Owns,
        GtkRelationshipKind::FlowTo => gtk::AccessibleRelation::FlowTo,
        GtkRelationshipKind::ErrorMessage => gtk::AccessibleRelation::ErrorMessage,
    }
}
