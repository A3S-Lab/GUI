use std::collections::{BTreeMap, BTreeSet};

use super::AccessibilityRelationshipProps;
use crate::host::HostNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessibilityRelationshipField {
    LabelledBy,
    DescribedBy,
    Details,
    Controls,
    Owns,
    FlowTo,
    ErrorMessage,
    ActiveDescendant,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ResolvedAccessibilityReferences {
    pub(crate) nodes: Vec<HostNodeId>,
    pub(crate) unresolved: Vec<String>,
}

impl ResolvedAccessibilityReferences {
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_complete(&self) -> bool {
        self.unresolved.is_empty()
    }

    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_single_complete(&self) -> bool {
        self.nodes.len() == 1 && self.is_complete()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ResolvedAccessibilityRelationships {
    pub(crate) labelled_by: Option<ResolvedAccessibilityReferences>,
    pub(crate) described_by: Option<ResolvedAccessibilityReferences>,
    pub(crate) details: Option<ResolvedAccessibilityReferences>,
    pub(crate) controls: Option<ResolvedAccessibilityReferences>,
    pub(crate) owns: Option<ResolvedAccessibilityReferences>,
    pub(crate) flow_to: Option<ResolvedAccessibilityReferences>,
    pub(crate) error_message: Option<ResolvedAccessibilityReferences>,
    pub(crate) active_descendant: Option<ResolvedAccessibilityReferences>,
}

#[derive(Debug, Default)]
pub(crate) struct AccessibilityRelationshipRegistry {
    node_ids: BTreeMap<HostNodeId, String>,
    nodes_by_id: BTreeMap<String, BTreeSet<HostNodeId>>,
    relationships: BTreeMap<HostNodeId, AccessibilityRelationshipProps>,
}

impl AccessibilityRelationshipRegistry {
    pub(crate) fn update_relationships(
        &mut self,
        source: HostNodeId,
        relationships: AccessibilityRelationshipProps,
    ) -> Vec<AccessibilityRelationshipField> {
        let previous = self.relationships.get(&source);
        let cleared = cleared_fields(previous, &relationships);
        if relationships == AccessibilityRelationshipProps::default() {
            self.relationships.remove(&source);
        } else {
            self.relationships.insert(source, relationships);
        }
        cleared
    }

    pub(crate) fn update_metadata(
        &mut self,
        node: HostNodeId,
        metadata: &BTreeMap<String, String>,
    ) -> bool {
        self.update_node_id(node, metadata.get("id").and_then(|value| valid_id(value)))
    }

    pub(crate) fn remove_node(&mut self, node: HostNodeId) -> bool {
        let relationships_removed = self.relationships.remove(&node).is_some();
        let id_removed = self.update_node_id(node, None);
        relationships_removed || id_removed
    }

    pub(crate) fn resolve(&self, source: HostNodeId) -> Option<ResolvedAccessibilityRelationships> {
        let relationships = self.relationships.get(&source)?;
        Some(ResolvedAccessibilityRelationships {
            labelled_by: self.resolve_value(relationships.labelled_by.as_deref()),
            described_by: self.resolve_value(relationships.described_by.as_deref()),
            details: self.resolve_value(relationships.details.as_deref()),
            controls: self.resolve_value(relationships.controls.as_deref()),
            owns: self.resolve_value(relationships.owns.as_deref()),
            flow_to: self.resolve_value(relationships.flow_to.as_deref()),
            error_message: self.resolve_value(relationships.error_message.as_deref()),
            active_descendant: self.resolve_value(relationships.active_descendant.as_deref()),
        })
    }

    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn resolve_all(&self) -> Vec<(HostNodeId, ResolvedAccessibilityRelationships)> {
        self.relationships
            .keys()
            .filter_map(|source| self.resolve(*source).map(|resolved| (*source, resolved)))
            .collect()
    }

    fn resolve_value(&self, value: Option<&str>) -> Option<ResolvedAccessibilityReferences> {
        let value = value?;
        let mut seen = BTreeSet::new();
        let mut resolved = ResolvedAccessibilityReferences::default();
        for token in value.split_whitespace() {
            if !seen.insert(token) {
                continue;
            }
            match self.nodes_by_id.get(token) {
                Some(nodes) if nodes.len() == 1 => {
                    if let Some(node) = nodes.first() {
                        resolved.nodes.push(*node);
                    }
                }
                _ => resolved.unresolved.push(token.to_string()),
            }
        }
        Some(resolved)
    }

    fn update_node_id(&mut self, node: HostNodeId, new_id: Option<String>) -> bool {
        if self.node_ids.get(&node) == new_id.as_ref() {
            return false;
        }
        if let Some(previous) = self.node_ids.remove(&node) {
            let remove_entry = if let Some(nodes) = self.nodes_by_id.get_mut(&previous) {
                nodes.remove(&node);
                nodes.is_empty()
            } else {
                false
            };
            if remove_entry {
                self.nodes_by_id.remove(&previous);
            }
        }
        if let Some(new_id) = new_id {
            self.nodes_by_id
                .entry(new_id.clone())
                .or_default()
                .insert(node);
            self.node_ids.insert(node, new_id);
        }
        true
    }
}

fn valid_id(value: &str) -> Option<String> {
    (!value.is_empty() && !value.chars().any(char::is_whitespace)).then(|| value.to_string())
}

fn cleared_fields(
    previous: Option<&AccessibilityRelationshipProps>,
    current: &AccessibilityRelationshipProps,
) -> Vec<AccessibilityRelationshipField> {
    let Some(previous) = previous else {
        return Vec::new();
    };
    [
        (
            AccessibilityRelationshipField::LabelledBy,
            previous.labelled_by.is_some() && current.labelled_by.is_none(),
        ),
        (
            AccessibilityRelationshipField::DescribedBy,
            previous.described_by.is_some() && current.described_by.is_none(),
        ),
        (
            AccessibilityRelationshipField::Details,
            previous.details.is_some() && current.details.is_none(),
        ),
        (
            AccessibilityRelationshipField::Controls,
            previous.controls.is_some() && current.controls.is_none(),
        ),
        (
            AccessibilityRelationshipField::Owns,
            previous.owns.is_some() && current.owns.is_none(),
        ),
        (
            AccessibilityRelationshipField::FlowTo,
            previous.flow_to.is_some() && current.flow_to.is_none(),
        ),
        (
            AccessibilityRelationshipField::ErrorMessage,
            previous.error_message.is_some() && current.error_message.is_none(),
        ),
        (
            AccessibilityRelationshipField::ActiveDescendant,
            previous.active_descendant.is_some() && current.active_descendant.is_none(),
        ),
    ]
    .into_iter()
    .filter_map(|(field, cleared)| cleared.then_some(field))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(id: &str) -> BTreeMap<String, String> {
        BTreeMap::from([("id".to_string(), id.to_string())])
    }

    #[test]
    fn resolves_forward_idref_lists_in_declared_order() {
        let source = HostNodeId::new(1);
        let label_a = HostNodeId::new(2);
        let label_b = HostNodeId::new(3);
        let option = HostNodeId::new(4);
        let mut registry = AccessibilityRelationshipRegistry::default();
        registry.update_relationships(
            source,
            AccessibilityRelationshipProps::default()
                .labelled_by("label-a label-b label-a")
                .active_descendant("option"),
        );

        let unresolved = registry.resolve(source).unwrap();
        assert_eq!(
            unresolved.labelled_by.unwrap().unresolved,
            ["label-a", "label-b"]
        );

        registry.update_metadata(label_b, &metadata("label-b"));
        registry.update_metadata(option, &metadata("option"));
        registry.update_metadata(label_a, &metadata("label-a"));
        let resolved = registry.resolve(source).unwrap();

        assert_eq!(
            resolved.labelled_by.unwrap().nodes,
            [label_a, label_b],
            "IDREF order must not depend on native mount order"
        );
        assert_eq!(
            resolved.active_descendant.unwrap().nodes,
            [option],
            "a forward single IDREF resolves once its target mounts"
        );
    }

    #[test]
    fn duplicate_ids_stay_unresolved_until_the_ambiguity_is_removed() {
        let source = HostNodeId::new(1);
        let first = HostNodeId::new(2);
        let second = HostNodeId::new(3);
        let mut registry = AccessibilityRelationshipRegistry::default();
        registry.update_relationships(
            source,
            AccessibilityRelationshipProps::default().controls("popup"),
        );
        registry.update_metadata(first, &metadata("popup"));
        registry.update_metadata(second, &metadata("popup"));

        let ambiguous = registry.resolve(source).unwrap().controls.unwrap();
        assert!(ambiguous.nodes.is_empty());
        assert_eq!(ambiguous.unresolved, ["popup"]);

        registry.update_metadata(second, &metadata("other"));
        assert_eq!(
            registry.resolve(source).unwrap().controls.unwrap().nodes,
            [first]
        );

        registry.remove_node(first);
        assert_eq!(
            registry
                .resolve(source)
                .unwrap()
                .controls
                .unwrap()
                .unresolved,
            ["popup"]
        );
    }

    #[test]
    fn relationship_updates_report_only_fields_that_require_native_reset() {
        let source = HostNodeId::new(1);
        let mut registry = AccessibilityRelationshipRegistry::default();
        registry.update_relationships(
            source,
            AccessibilityRelationshipProps::default()
                .labelled_by("label")
                .controls("popup"),
        );

        assert_eq!(
            registry.update_relationships(
                source,
                AccessibilityRelationshipProps::default().controls("popup"),
            ),
            [AccessibilityRelationshipField::LabelledBy]
        );
        assert_eq!(
            registry.update_relationships(source, AccessibilityRelationshipProps::default()),
            [AccessibilityRelationshipField::Controls]
        );
        assert!(registry.resolve(source).is_none());
    }

    #[test]
    fn invalid_dom_ids_do_not_enter_the_native_reference_index() {
        let source = HostNodeId::new(1);
        let target = HostNodeId::new(2);
        let mut registry = AccessibilityRelationshipRegistry::default();
        registry.update_relationships(
            source,
            AccessibilityRelationshipProps::default().described_by("label bad"),
        );
        registry.update_metadata(target, &metadata(" label "));

        let resolved = registry.resolve(source).unwrap().described_by.unwrap();
        assert!(resolved.nodes.is_empty());
        assert_eq!(resolved.unresolved, ["label", "bad"]);

        registry.update_metadata(target, &metadata("bad id"));
        assert_eq!(
            registry
                .resolve(source)
                .unwrap()
                .described_by
                .unwrap()
                .unresolved,
            ["label", "bad"]
        );
    }
}
