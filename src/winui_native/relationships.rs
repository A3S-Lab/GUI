use super::*;
use crate::accessibility::relationship_registry::{
    AccessibilityRelationshipField, ResolvedAccessibilityReferences,
    ResolvedAccessibilityRelationships,
};
use crate::accessibility::AccessibilityRelationshipProps;

impl WinUiNativeSurface {
    pub(super) fn set_accessibility_relationships(
        &mut self,
        source: HostNodeId,
        relationships: AccessibilityRelationshipProps,
    ) -> GuiResult<()> {
        let cleared = self
            .accessibility_relationships
            .update_relationships(source, relationships);
        self.reset_accessibility_relationships(source, &cleared)?;
        self.reconcile_accessibility_relationship_source(source)
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
            self.reconcile_accessibility_relationships()?;
        }
        Ok(())
    }

    pub(super) fn forget_accessibility_relationship_node(
        &mut self,
        node: HostNodeId,
    ) -> GuiResult<()> {
        if self.accessibility_relationships.remove_node(node) {
            self.reconcile_accessibility_relationships()?;
        }
        Ok(())
    }

    fn reconcile_accessibility_relationships(&self) -> GuiResult<()> {
        for (source, relationships) in self.accessibility_relationships.resolve_all() {
            self.apply_accessibility_relationships(source, &relationships)?;
        }
        Ok(())
    }

    fn reconcile_accessibility_relationship_source(&self, source: HostNodeId) -> GuiResult<()> {
        if let Some(relationships) = self.accessibility_relationships.resolve(source) {
            self.apply_accessibility_relationships(source, &relationships)?;
        }
        Ok(())
    }

    fn apply_accessibility_relationships(
        &self,
        source: HostNodeId,
        relationships: &ResolvedAccessibilityRelationships,
    ) -> GuiResult<()> {
        let Some(source) = self
            .widgets
            .get(&source)
            .and_then(WinUiOsWidget::ui_element)
        else {
            return Ok(());
        };
        if let Some(references) = relationships.labelled_by.as_ref() {
            let target = references
                .is_single_complete()
                .then(|| {
                    references
                        .nodes
                        .first()
                        .and_then(|node| self.widgets.get(node))
                        .and_then(WinUiOsWidget::ui_element)
                })
                .flatten();
            map_winui(
                "failed to set WinUI labelled-by accessibility relationship",
                automation::set_labeled_by(&source, target.as_ref()),
            )?;
        }
        if let Some(references) = relationships.described_by.as_ref() {
            let targets = self.resolved_dependency_objects(references);
            map_winui(
                "failed to set WinUI described-by accessibility relationship",
                automation::replace_described_by(&source, &targets),
            )?;
        }
        if let Some(references) = relationships.controls.as_ref() {
            let targets = self.resolved_ui_elements(references);
            map_winui(
                "failed to set WinUI controls accessibility relationship",
                automation::replace_controlled_peers(&source, &targets),
            )?;
        }
        if let Some(references) = relationships.flow_to.as_ref() {
            let targets = self.resolved_dependency_objects(references);
            map_winui(
                "failed to set WinUI flow-to accessibility relationship",
                automation::replace_flows_to(&source, &targets),
            )?;
        }
        Ok(())
    }

    fn resolved_ui_elements(
        &self,
        references: &ResolvedAccessibilityReferences,
    ) -> Vec<xaml::UIElement> {
        if !references.is_complete() {
            return Vec::new();
        }
        let targets = references
            .nodes
            .iter()
            .filter_map(|node| self.widgets.get(node))
            .filter_map(WinUiOsWidget::ui_element)
            .collect::<Vec<_>>();
        (targets.len() == references.nodes.len())
            .then_some(targets)
            .unwrap_or_default()
    }

    fn resolved_dependency_objects(
        &self,
        references: &ResolvedAccessibilityReferences,
    ) -> Vec<xaml::DependencyObject> {
        let elements = self.resolved_ui_elements(references);
        let expected = elements.len();
        let targets = elements
            .into_iter()
            .filter_map(|element| element.cast().ok())
            .collect::<Vec<_>>();
        (targets.len() == expected)
            .then_some(targets)
            .unwrap_or_default()
    }

    fn reset_accessibility_relationships(
        &self,
        source: HostNodeId,
        fields: &[AccessibilityRelationshipField],
    ) -> GuiResult<()> {
        let Some(source) = self
            .widgets
            .get(&source)
            .and_then(WinUiOsWidget::ui_element)
        else {
            return Ok(());
        };
        for field in fields {
            match field {
                AccessibilityRelationshipField::LabelledBy => map_winui(
                    "failed to clear WinUI labelled-by accessibility relationship",
                    automation::set_labeled_by(&source, None),
                )?,
                AccessibilityRelationshipField::DescribedBy => map_winui(
                    "failed to clear WinUI described-by accessibility relationship",
                    automation::replace_described_by(&source, &[]),
                )?,
                AccessibilityRelationshipField::Controls => map_winui(
                    "failed to clear WinUI controls accessibility relationship",
                    automation::replace_controlled_peers(&source, &[]),
                )?,
                AccessibilityRelationshipField::FlowTo => map_winui(
                    "failed to clear WinUI flow-to accessibility relationship",
                    automation::replace_flows_to(&source, &[]),
                )?,
                AccessibilityRelationshipField::Details
                | AccessibilityRelationshipField::Owns
                | AccessibilityRelationshipField::ErrorMessage
                | AccessibilityRelationshipField::ActiveDescendant => {}
            }
        }
        Ok(())
    }
}
