use crate::platform_host::PlatformElementId;

use super::drag_drop::{SelfDrawnDragSource, SelfDrawnDropItem};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionTree {
    pub(super) fn drag_source(&self, target: &PlatformElementId) -> Option<&SelfDrawnDragSource> {
        self.node(target)
            .filter(|node| node.available)
            .and_then(|node| node.drag_source.as_ref())
    }

    pub(super) fn drag_source_for_start(
        &self,
        target: &PlatformElementId,
    ) -> Option<SelfDrawnDragSource> {
        let node = self.node(target).filter(|node| node.available)?;
        let mut source = node.drag_source.clone()?;
        let Some(collection) = self.draggable_collection_owner(target) else {
            return Some(source);
        };
        let Some(key) = node.collection_item_key.clone() else {
            return Some(source);
        };

        source.collection = Some(collection.clone());
        source.dragging_keys = vec![key];
        source.dragging_nodes = vec![target.clone()];
        self.apply_collection_allowed_operations(&collection, &mut source);
        if !node.props.selected {
            return Some(source);
        }

        let mut keys = Vec::new();
        let mut items = Vec::new();
        let mut nodes = Vec::new();
        for candidate in self.collection_items(&collection) {
            if !candidate.props.selected {
                continue;
            }
            let Some(candidate_source) = candidate.drag_source.as_ref() else {
                continue;
            };
            let Some(candidate_key) = candidate.collection_item_key.as_ref() else {
                continue;
            };
            keys.push(candidate_key.clone());
            nodes.push(candidate.id.clone());
            items.extend(candidate_source.items.iter().cloned());
        }
        if !keys.is_empty() {
            source.dragging_keys = keys;
            source.dragging_nodes = nodes;
            source.items = items;
            source.types = item_types(&source.items);
        }
        Some(source)
    }

    fn draggable_collection_owner(&self, id: &PlatformElementId) -> Option<PlatformElementId> {
        self.ancestors_inclusive(id)
            .into_iter()
            .skip(1)
            .find(|ancestor| {
                self.node(ancestor)
                    .is_some_and(|node| node.draggable_collection)
            })
    }

    fn apply_collection_allowed_operations(
        &self,
        collection: &PlatformElementId,
        source: &mut SelfDrawnDragSource,
    ) {
        let Some(node) = self.node(collection) else {
            return;
        };
        if let Some(operations) = SelfDrawnDragSource::allowed_operations_from_props(&node.props) {
            source.allowed_operations = operations;
        }
    }
}

fn item_types(items: &[SelfDrawnDropItem]) -> Vec<String> {
    items
        .iter()
        .flat_map(SelfDrawnDropItem::types)
        .fold(Vec::new(), |mut types, drag_type| {
            if !types.contains(drag_type) {
                types.push(drag_type.clone());
            }
            types
        })
}
