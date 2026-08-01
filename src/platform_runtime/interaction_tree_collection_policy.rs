use std::collections::BTreeSet;

use crate::native::NativeRole;
use crate::platform_host::PlatformElementId;

use super::drag_drop::SelfDrawnDragSession;
use super::drag_drop_collection::{
    SelfDrawnCollectionDropConfig, SelfDrawnCollectionDropTarget, SelfDrawnDropPosition,
};
use super::drop_policy::{SelfDrawnDropPolicyEvaluation, SelfDrawnDropPolicyTarget};
use super::interaction_tree::{SelfDrawnInteractionNode, SelfDrawnInteractionTree};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CollectionParentIdentity {
    Item(String),
    Section(PlatformElementId),
}

impl SelfDrawnInteractionTree {
    pub(super) fn collection_items(
        &self,
        collection: &PlatformElementId,
    ) -> Vec<&SelfDrawnInteractionNode> {
        self.tree_order
            .iter()
            .filter_map(|id| {
                let node = self.node(id)?;
                (node.available
                    && node.collection_item_key.is_some()
                    && self.collection_owner(id).as_ref() == Some(collection))
                .then_some(node)
            })
            .collect()
    }

    pub(super) fn collection_owner(&self, id: &PlatformElementId) -> Option<PlatformElementId> {
        self.ancestors_inclusive(id)
            .into_iter()
            .skip(1)
            .find(|ancestor| {
                self.node(ancestor)
                    .is_some_and(|node| node.collection_container)
            })
    }

    pub(super) fn collection_target_allowed(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        target: &SelfDrawnCollectionDropTarget,
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
    ) -> bool {
        let is_internal = source_collection == Some(collection);
        if is_internal && self.is_self_or_descendant_target(collection, target, dragging_keys) {
            return false;
        }
        if config.has_low_level_drop() {
            return true;
        }

        match target {
            SelfDrawnCollectionDropTarget::Root => !is_internal && config.has_root_drop(),
            SelfDrawnCollectionDropTarget::Item {
                drop_position: SelfDrawnDropPosition::On,
                ..
            } => config.has_item_drop() || (is_internal && config.has_move()),
            SelfDrawnCollectionDropTarget::Item { .. } if is_internal => {
                config.has_move()
                    || (config.has_reorder()
                        && self.is_dragging_within_parent(collection, target, dragging_keys))
            }
            SelfDrawnCollectionDropTarget::Item { .. } => config.has_insert(),
        }
    }

    pub(super) fn collection_drop_actions<'a>(
        &'a self,
        collection: &PlatformElementId,
        target: &SelfDrawnCollectionDropTarget,
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
    ) -> Vec<&'a str> {
        let Some(config) = self
            .node(collection)
            .and_then(|node| node.collection_drop.as_ref())
        else {
            return Vec::new();
        };
        if let Some(action) = config.low_level_action() {
            return vec![action];
        }

        let is_internal = source_collection == Some(collection);
        let mut actions = Vec::new();
        match target {
            SelfDrawnCollectionDropTarget::Root if !is_internal => {
                actions.extend(config.root_action());
            }
            SelfDrawnCollectionDropTarget::Item {
                drop_position: SelfDrawnDropPosition::On,
                ..
            } => {
                actions.extend(config.item_action());
                if is_internal {
                    actions.extend(config.move_action());
                }
            }
            SelfDrawnCollectionDropTarget::Item { .. } if is_internal => {
                actions.extend(config.move_action());
                if self.is_dragging_within_parent(collection, target, dragging_keys) {
                    actions.extend(config.reorder_action());
                }
            }
            SelfDrawnCollectionDropTarget::Item { .. } => {
                actions.extend(config.insert_action());
            }
            SelfDrawnCollectionDropTarget::Root => {}
        }
        actions
    }

    pub(super) fn filter_collection_drop_items(
        &self,
        session: &mut SelfDrawnDragSession,
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) {
        let (Some(collection), Some(target)) = (
            session.current_collection.as_ref(),
            session.current_collection_target.as_ref(),
        ) else {
            return;
        };
        let Some(config) = self
            .node(collection)
            .and_then(|node| node.collection_drop.as_ref())
        else {
            session.current_item_indices.clear();
            return;
        };
        if config.has_low_level_drop()
            || !matches!(
                target,
                SelfDrawnCollectionDropTarget::Item {
                    drop_position: SelfDrawnDropPosition::On,
                    ..
                }
            )
        {
            return;
        }
        let Some(policy_id) = config.should_accept_item_drop_policy() else {
            return;
        };
        let policy_target = SelfDrawnDropPolicyTarget::Collection {
            id: collection.clone(),
            target: target.clone(),
        };
        session.current_item_indices = session
            .current_item_indices
            .iter()
            .copied()
            .filter(|index| {
                session.items.get(*index).is_some_and(|item| {
                    drop_policy.should_accept_item_drop(
                        policy_id,
                        policy_target.clone(),
                        item.types(),
                    )
                })
            })
            .collect();
    }

    pub(super) fn collection_drop_is_low_level(&self, collection: &PlatformElementId) -> bool {
        self.node(collection)
            .and_then(|node| node.collection_drop.as_ref())
            .is_some_and(SelfDrawnCollectionDropConfig::has_low_level_drop)
    }

    pub(super) fn collection_targets_equivalent(
        &self,
        collection: &PlatformElementId,
        first: &SelfDrawnCollectionDropTarget,
        second: &SelfDrawnCollectionDropTarget,
    ) -> bool {
        if first == second {
            return true;
        }
        let (
            SelfDrawnCollectionDropTarget::Item {
                key: first_key,
                drop_position: first_position,
            },
            SelfDrawnCollectionDropTarget::Item {
                key: second_key,
                drop_position: second_position,
            },
        ) = (first, second)
        else {
            return false;
        };
        let (before_key, after_key) = match (first_position, second_position) {
            (SelfDrawnDropPosition::After, SelfDrawnDropPosition::Before) => {
                (second_key, first_key)
            }
            (SelfDrawnDropPosition::Before, SelfDrawnDropPosition::After) => {
                (first_key, second_key)
            }
            _ => return false,
        };
        let keys = self
            .collection_items(collection)
            .into_iter()
            .filter_map(|item| item.collection_item_key.as_deref())
            .collect::<Vec<_>>();
        let Some(after_index) = keys.iter().position(|key| *key == after_key) else {
            return false;
        };
        keys.get(after_index.saturating_add(1)).copied() == Some(before_key.as_str())
    }

    pub(super) fn has_selected_collection_ancestor(
        &self,
        collection: &PlatformElementId,
        item: &SelfDrawnInteractionNode,
        selected_keys: &BTreeSet<String>,
    ) -> bool {
        let mut parent = self.collection_parent_item_key(collection, item);
        let mut seen = BTreeSet::new();
        while let Some(key) = parent {
            if selected_keys.contains(&key) {
                return true;
            }
            if !seen.insert(key.clone()) {
                return false;
            }
            parent = self
                .collection_item_by_key(collection, &key)
                .and_then(|node| self.collection_parent_item_key(collection, node));
        }
        false
    }

    fn is_self_or_descendant_target(
        &self,
        collection: &PlatformElementId,
        target: &SelfDrawnCollectionDropTarget,
        dragging_keys: &[String],
    ) -> bool {
        let SelfDrawnCollectionDropTarget::Item { key, drop_position } = target else {
            return false;
        };
        if *drop_position == SelfDrawnDropPosition::On
            && dragging_keys.iter().any(|dragging| dragging == key)
        {
            return true;
        }
        let Some(item) = self.collection_item_by_key(collection, key) else {
            return false;
        };
        let dragging_keys = dragging_keys
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut parent = self.collection_parent_item_key(collection, item);
        let mut seen = BTreeSet::new();
        while let Some(key) = parent {
            if dragging_keys.contains(key.as_str()) {
                return true;
            }
            if !seen.insert(key.clone()) {
                return false;
            }
            parent = self
                .collection_item_by_key(collection, &key)
                .and_then(|node| self.collection_parent_item_key(collection, node));
        }
        false
    }

    fn is_dragging_within_parent(
        &self,
        collection: &PlatformElementId,
        target: &SelfDrawnCollectionDropTarget,
        dragging_keys: &[String],
    ) -> bool {
        let Some(target_key) = target.key() else {
            return false;
        };
        let Some(target_item) = self.collection_item_by_key(collection, target_key) else {
            return false;
        };
        let target_parent = self.collection_parent_identity(collection, target_item);
        !dragging_keys.is_empty()
            && dragging_keys.iter().all(|key| {
                self.collection_item_by_key(collection, key)
                    .is_some_and(|item| {
                        self.collection_parent_identity(collection, item) == target_parent
                    })
            })
    }

    fn collection_item_by_key(
        &self,
        collection: &PlatformElementId,
        key: &str,
    ) -> Option<&SelfDrawnInteractionNode> {
        self.collection_items(collection)
            .into_iter()
            .find(|item| item.collection_item_key.as_deref() == Some(key))
    }

    fn collection_parent_identity(
        &self,
        collection: &PlatformElementId,
        item: &SelfDrawnInteractionNode,
    ) -> Option<CollectionParentIdentity> {
        if let Some(key) = explicit_parent_key(item) {
            return Some(CollectionParentIdentity::Item(key.to_string()));
        }
        let mut parent = item.parent.clone();
        while let Some(id) = parent {
            if &id == collection {
                return None;
            }
            let node = self.node(&id)?;
            if let Some(key) = node.collection_item_key.as_ref() {
                return Some(CollectionParentIdentity::Item(key.clone()));
            }
            if is_collection_section(node) {
                return Some(CollectionParentIdentity::Section(id));
            }
            parent = node.parent.clone();
        }
        None
    }

    fn collection_parent_item_key(
        &self,
        collection: &PlatformElementId,
        item: &SelfDrawnInteractionNode,
    ) -> Option<String> {
        if let Some(key) = explicit_parent_key(item) {
            return Some(key.to_string());
        }
        let mut parent = item.parent.clone();
        while let Some(id) = parent {
            if &id == collection {
                return None;
            }
            let node = self.node(&id)?;
            if let Some(key) = node.collection_item_key.as_ref() {
                return Some(key.clone());
            }
            parent = node.parent.clone();
        }
        None
    }
}

fn explicit_parent_key(item: &SelfDrawnInteractionNode) -> Option<&str> {
    ["data-tree-parent-key", "data-collection-parent-key"]
        .into_iter()
        .find_map(|name| {
            item.props
                .web
                .attributes
                .get(name)
                .or_else(|| item.props.metadata.get(name))
                .map(String::as_str)
                .map(str::trim)
                .filter(|key| !key.is_empty())
        })
}

fn is_collection_section(node: &SelfDrawnInteractionNode) -> bool {
    node.role == NativeRole::TableSection
        || node
            .props
            .web
            .attributes
            .get("data-collection-section")
            .or_else(|| node.props.metadata.get("data-collection-section"))
            .is_some_and(|value| value.is_empty() || value.eq_ignore_ascii_case("true"))
}
