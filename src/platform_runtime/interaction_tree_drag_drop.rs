use crate::geometry::Orientation;
use crate::platform_host::{PlatformElementId, PlatformPoint};

use super::drag_drop::{
    SelfDrawnDropItem, SelfDrawnDropOperation, SelfDrawnDropTarget, SelfDrawnMatchedDropTarget,
};
use super::drag_drop_collection::{
    SelfDrawnCollectionDropConfig, SelfDrawnCollectionDropTarget, SelfDrawnDropPosition,
};
use super::drop_policy::{SelfDrawnDropPolicyEvaluation, SelfDrawnDropPolicyTarget};
use super::interaction_tree::{contains, SelfDrawnInteractionNode, SelfDrawnInteractionTree};

impl SelfDrawnInteractionTree {
    pub(super) fn drop_target_at(
        &self,
        point: PlatformPoint,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        for region in self.hit_regions.iter().rev() {
            if !contains(region.bounds, point) {
                continue;
            }
            let node = self.node(&region.id)?;
            if !node.available {
                return None;
            }
            let path = self.ancestors_inclusive(&region.id);
            for id in &path {
                let node = self.node(id)?;
                if let Some(config) = node.collection_drop.as_ref() {
                    return self.collection_target_at(
                        id,
                        config,
                        &path,
                        point,
                        items,
                        types,
                        allowed_operations,
                        source_collection,
                        dragging_keys,
                        drop_policy,
                    );
                }
                let Some(target) = node.drop_target.as_ref() else {
                    continue;
                };
                return self.generic_match(
                    id,
                    target,
                    items,
                    types,
                    allowed_operations,
                    drop_policy,
                );
            }
            return None;
        }
        None
    }

    pub(super) fn compatible_drop_targets(
        &self,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Vec<SelfDrawnMatchedDropTarget> {
        self.tree_order
            .iter()
            .filter_map(|id| {
                let node = self.node(id).filter(|node| node.available)?;
                if let Some(config) = node.collection_drop.as_ref() {
                    let is_internal = source_collection == Some(id);
                    return self
                        .collection_keyboard_positions(id, config, is_internal)
                        .into_iter()
                        .find_map(|descriptor| {
                            self.collection_match(
                                id,
                                config,
                                descriptor,
                                items,
                                types,
                                allowed_operations,
                                source_collection,
                                dragging_keys,
                                drop_policy,
                            )
                        });
                }
                let target = node.drop_target.as_ref()?;
                self.generic_match(id, target, items, types, allowed_operations, drop_policy)
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn compatible_drop_target(
        &self,
        id: &PlatformElementId,
        collection: Option<&PlatformElementId>,
        collection_target: Option<&SelfDrawnCollectionDropTarget>,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        if let (Some(collection), Some(descriptor)) = (collection, collection_target) {
            let config = self
                .node(collection)
                .filter(|node| node.available)?
                .collection_drop
                .as_ref()?;
            return self.collection_match(
                collection,
                config,
                descriptor.clone(),
                items,
                types,
                allowed_operations,
                source_collection,
                dragging_keys,
                drop_policy,
            );
        }
        let node = self.node(id).filter(|node| node.available)?;
        let target = node.drop_target.as_ref()?;
        self.generic_match(id, target, items, types, allowed_operations, drop_policy)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn keyboard_collection_target(
        &self,
        collection: &PlatformElementId,
        current: &SelfDrawnCollectionDropTarget,
        key: &str,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        let config = self.node(collection)?.collection_drop.as_ref()?;
        let direction = match (config.orientation, key) {
            (Orientation::Vertical, "ArrowDown") | (Orientation::Horizontal, "ArrowRight") => 1_i8,
            (Orientation::Vertical, "ArrowUp") | (Orientation::Horizontal, "ArrowLeft") => -1_i8,
            (_, "Home") => -2_i8,
            (_, "End") => 2_i8,
            _ => return None,
        };
        let is_internal = source_collection == Some(collection);
        let positions = self
            .collection_keyboard_positions(collection, config, is_internal)
            .into_iter()
            .filter_map(|target| {
                self.collection_match(
                    collection,
                    config,
                    target,
                    items,
                    types,
                    allowed_operations,
                    source_collection,
                    dragging_keys,
                    drop_policy,
                )
            })
            .collect::<Vec<_>>();
        let current_index = positions.iter().position(|candidate| {
            candidate.collection_target.as_ref().is_some_and(|target| {
                self.collection_targets_equivalent(collection, target, current)
            })
        })?;
        let next_index = match direction {
            -2 => 0,
            2 => positions.len().saturating_sub(1),
            -1 => current_index.checked_sub(1)?,
            1 if current_index + 1 < positions.len() => current_index + 1,
            _ => return None,
        };
        positions.get(next_index).cloned()
    }

    fn generic_match(
        &self,
        id: &PlatformElementId,
        target: &SelfDrawnDropTarget,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        if !target.accepts(types) {
            return None;
        }
        let operation = target.get_drop_operation_policy().map_or_else(
            || target.default_operation_for(allowed_operations),
            |policy_id| {
                drop_policy.get_drop_operation(
                    policy_id,
                    SelfDrawnDropPolicyTarget::Generic { id: id.clone() },
                    types,
                    allowed_operations,
                )
            },
        );
        (operation != SelfDrawnDropOperation::Cancel).then(|| SelfDrawnMatchedDropTarget {
            id: id.clone(),
            collection: None,
            collection_target: None,
            operation,
            item_indices: target.matching_item_indices(items),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn collection_target_at(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        path: &[PlatformElementId],
        point: PlatformPoint,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        let is_internal = source_collection == Some(collection);
        let descriptor =
            self.collection_descriptor_at(collection, config, path, point, is_internal)?;
        let matched = self.collection_match(
            collection,
            config,
            descriptor.clone(),
            items,
            types,
            allowed_operations,
            source_collection,
            dragging_keys,
            drop_policy,
        );
        if matched.is_some() || matches!(descriptor, SelfDrawnCollectionDropTarget::Root) {
            return matched;
        }
        self.collection_match(
            collection,
            config,
            SelfDrawnCollectionDropTarget::Root,
            items,
            types,
            allowed_operations,
            source_collection,
            dragging_keys,
            drop_policy,
        )
    }

    fn collection_descriptor_at(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        path: &[PlatformElementId],
        point: PlatformPoint,
        is_internal: bool,
    ) -> Option<SelfDrawnCollectionDropTarget> {
        if config.allows_insert(is_internal) {
            if let Some(target) = path.iter().find_map(|id| {
                let node = self.node(id)?;
                (self.collection_owner(id).as_ref() == Some(collection))
                    .then(|| node.drop_indicator_target.clone())
                    .flatten()
            }) {
                return Some(target);
            }
        }
        if let Some(item) = path.iter().find_map(|id| {
            let node = self.node(id)?;
            (node.collection_item_key.is_some()
                && self.collection_owner(id).as_ref() == Some(collection))
            .then_some(node)
        }) {
            return descriptor_for_item(item, config, point, is_internal);
        }
        if let Some(item) = self
            .collection_items(collection)
            .into_iter()
            .find(|item| contains(item.bounds, point))
        {
            return descriptor_for_item(item, config, point, is_internal);
        }
        if config.allows_insert(is_internal) {
            if let Some(target) = self.nearest_insertion_target(collection, config, point, true) {
                return Some(target);
            }
        }
        if config.allows_root(is_internal) {
            return Some(SelfDrawnCollectionDropTarget::Root);
        }
        config
            .allows_insert(is_internal)
            .then(|| self.nearest_insertion_target(collection, config, point, false))
            .flatten()
    }

    fn nearest_insertion_target(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        point: PlatformPoint,
        require_cross_axis_overlap: bool,
    ) -> Option<SelfDrawnCollectionDropTarget> {
        let items = self.collection_items(collection);
        let item = items.into_iter().min_by(|a, b| {
            insertion_distance(a, config.orientation, point, require_cross_axis_overlap).total_cmp(
                &insertion_distance(b, config.orientation, point, require_cross_axis_overlap),
            )
        })?;
        let distance =
            insertion_distance(item, config.orientation, point, require_cross_axis_overlap);
        if !distance.is_finite() {
            return None;
        }
        let before = match config.orientation {
            Orientation::Vertical => point.y < item.bounds.y + item.bounds.height / 2.0,
            Orientation::Horizontal => point.x < item.bounds.x + item.bounds.width / 2.0,
        };
        Some(SelfDrawnCollectionDropTarget::Item {
            key: item.collection_item_key.clone()?,
            drop_position: if before {
                SelfDrawnDropPosition::Before
            } else {
                SelfDrawnDropPosition::After
            },
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn collection_match(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        descriptor: SelfDrawnCollectionDropTarget,
        items: &[SelfDrawnDropItem],
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
        source_collection: Option<&PlatformElementId>,
        dragging_keys: &[String],
        drop_policy: &mut SelfDrawnDropPolicyEvaluation<'_>,
    ) -> Option<SelfDrawnMatchedDropTarget> {
        if !self.collection_target_allowed(
            collection,
            config,
            &descriptor,
            source_collection,
            dragging_keys,
        ) {
            return None;
        }
        if !config.target.accepts(types) {
            return None;
        }
        let policy_target = SelfDrawnDropPolicyTarget::Collection {
            id: collection.clone(),
            target: descriptor.clone(),
        };
        if !config.has_low_level_drop()
            && matches!(
                descriptor,
                SelfDrawnCollectionDropTarget::Item {
                    drop_position: SelfDrawnDropPosition::On,
                    ..
                }
            )
        {
            if let Some(policy_id) = config.should_accept_item_drop_policy() {
                let accepted =
                    drop_policy.should_accept_item_drop(policy_id, policy_target.clone(), types);
                if !accepted && !config.has_low_level_drop() {
                    return None;
                }
            }
        }
        let operation = config.target.get_drop_operation_policy().map_or_else(
            || config.target.default_operation_for(allowed_operations),
            |policy_id| {
                drop_policy.get_drop_operation(policy_id, policy_target, types, allowed_operations)
            },
        );
        if operation == SelfDrawnDropOperation::Cancel {
            return None;
        }
        let id = self.visual_target(collection, &descriptor)?;
        Some(SelfDrawnMatchedDropTarget {
            id,
            collection: Some(collection.clone()),
            collection_target: Some(descriptor),
            operation,
            item_indices: if config.has_low_level_drop() {
                (0..items.len()).collect()
            } else {
                config.target.matching_item_indices(items)
            },
        })
    }

    fn visual_target(
        &self,
        collection: &PlatformElementId,
        descriptor: &SelfDrawnCollectionDropTarget,
    ) -> Option<PlatformElementId> {
        if matches!(descriptor, SelfDrawnCollectionDropTarget::Root) {
            return Some(collection.clone());
        }
        if let Some(indicator) = self.tree_order.iter().find(|id| {
            self.node(id).is_some_and(|node| {
                node.drop_indicator_target.as_ref() == Some(descriptor)
                    && self.collection_owner(id).as_ref() == Some(collection)
            })
        }) {
            return Some(indicator.clone());
        }
        let key = descriptor.key()?;
        self.collection_items(collection)
            .into_iter()
            .find(|item| item.collection_item_key.as_deref() == Some(key))
            .map(|item| item.id.clone())
    }

    fn collection_keyboard_positions(
        &self,
        collection: &PlatformElementId,
        config: &SelfDrawnCollectionDropConfig,
        is_internal: bool,
    ) -> Vec<SelfDrawnCollectionDropTarget> {
        let items = self.collection_items(collection);
        let mut targets = Vec::new();
        if config.allows_root(is_internal) {
            targets.push(SelfDrawnCollectionDropTarget::Root);
        }
        if config.allows_insert(is_internal) {
            if let Some(first) = items
                .first()
                .and_then(|item| item.collection_item_key.clone())
            {
                targets.push(SelfDrawnCollectionDropTarget::Item {
                    key: first,
                    drop_position: SelfDrawnDropPosition::Before,
                });
            }
        }
        for item in items {
            let Some(key) = item.collection_item_key.clone() else {
                continue;
            };
            if config.allows_item(is_internal) {
                targets.push(SelfDrawnCollectionDropTarget::Item {
                    key: key.clone(),
                    drop_position: SelfDrawnDropPosition::On,
                });
            }
            if config.allows_insert(is_internal) {
                targets.push(SelfDrawnCollectionDropTarget::Item {
                    key,
                    drop_position: SelfDrawnDropPosition::After,
                });
            }
        }
        targets
    }
}

fn descriptor_for_item(
    item: &SelfDrawnInteractionNode,
    config: &SelfDrawnCollectionDropConfig,
    point: PlatformPoint,
    is_internal: bool,
) -> Option<SelfDrawnCollectionDropTarget> {
    let key = item.collection_item_key.clone()?;
    let ratio = match config.orientation {
        Orientation::Vertical => (point.y - item.bounds.y) / item.bounds.height.max(1.0),
        Orientation::Horizontal => (point.x - item.bounds.x) / item.bounds.width.max(1.0),
    };
    let drop_position = match (
        config.allows_insert(is_internal),
        config.allows_item(is_internal),
    ) {
        (true, true) if ratio < 0.25 => SelfDrawnDropPosition::Before,
        (true, true) if ratio >= 0.75 => SelfDrawnDropPosition::After,
        (true, true) => SelfDrawnDropPosition::On,
        (true, false) if ratio < 0.5 => SelfDrawnDropPosition::Before,
        (true, false) => SelfDrawnDropPosition::After,
        (false, true) => SelfDrawnDropPosition::On,
        (false, false) if config.allows_root(is_internal) => {
            return Some(SelfDrawnCollectionDropTarget::Root);
        }
        (false, false) => return None,
    };
    Some(SelfDrawnCollectionDropTarget::Item { key, drop_position })
}

fn insertion_distance(
    item: &SelfDrawnInteractionNode,
    orientation: Orientation,
    point: PlatformPoint,
    require_cross_axis_overlap: bool,
) -> f64 {
    let (primary, start, end, cross, cross_start, cross_end) = match orientation {
        Orientation::Vertical => (
            point.y,
            item.bounds.y,
            item.bounds.y + item.bounds.height,
            point.x,
            item.bounds.x,
            item.bounds.x + item.bounds.width,
        ),
        Orientation::Horizontal => (
            point.x,
            item.bounds.x,
            item.bounds.x + item.bounds.width,
            point.y,
            item.bounds.y,
            item.bounds.y + item.bounds.height,
        ),
    };
    if require_cross_axis_overlap && (cross < cross_start || cross >= cross_end) {
        return f64::INFINITY;
    }
    if primary < start {
        start - primary
    } else if primary >= end {
        primary - end
    } else {
        0.0
    }
}
