use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct TreeExpansionState {
    expanded_keys: BTreeSet<CollectionKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedTreeExpansionUpdate {
    pub collection: HostNodeId,
    pub expanded_keys: BTreeSet<CollectionKey>,
}

impl MountedTreeExpansionUpdate {
    pub fn event_value(&self) -> String {
        serde_json::to_string(&self.expanded_keys).unwrap_or_else(|_| "[]".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedTreeItemProjection {
    pub node: HostNodeId,
    pub expanded: Option<bool>,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedTreeProjection {
    pub collection: HostNodeId,
    pub expanded_keys: BTreeSet<CollectionKey>,
    pub items: Vec<MountedTreeItemProjection>,
}

pub(super) fn normalize_items(items: &mut [MountedSelectionItem]) {
    let resolved_parents = items
        .iter()
        .map(|item| {
            item.parent_key
                .as_ref()
                .map(|parent| resolve_key_alias(parent, items))
        })
        .collect::<Vec<_>>();
    for (item, parent) in items.iter_mut().zip(resolved_parents) {
        item.parent_key = parent;
    }

    let parent_keys = items
        .iter()
        .filter_map(|item| item.parent_key.clone())
        .collect::<BTreeSet<_>>();
    let mut levels = BTreeMap::<CollectionKey, u32>::new();
    for item in items {
        item.has_child_items |= parent_keys.contains(&item.key);
        if item.level == 1 {
            if let Some(parent_level) = item
                .parent_key
                .as_ref()
                .and_then(|parent| levels.get(parent).copied())
            {
                item.level = parent_level.saturating_add(1);
            }
        }
        levels.insert(item.key.clone(), item.level);
    }
}

pub(super) fn configure_state(
    container: &MountedNodeSnapshot,
    items: &[MountedSelectionItem],
    previous: Option<TreeExpansionState>,
    was_mounted: bool,
) -> Option<TreeExpansionState> {
    if container.role != NativeRole::Tree {
        return None;
    }

    let controlled = expanded_key_attribute(
        &container.props,
        &["expandedKeys", "expanded-keys", "data-expanded-keys"],
        items,
    );
    let initial = (!was_mounted).then(|| {
        expanded_key_attribute(
            &container.props,
            &[
                "defaultExpandedKeys",
                "default-expanded-keys",
                "data-default-expanded-keys",
            ],
            items,
        )
        .unwrap_or_else(|| {
            items
                .iter()
                .filter(|item| item.expanded == Some(true))
                .map(|item| item.key.clone())
                .collect()
        })
    });
    Some(TreeExpansionState {
        expanded_keys: controlled
            .or(initial)
            .or_else(|| previous.map(|state| state.expanded_keys))
            .unwrap_or_default(),
    })
}

pub(super) fn projections(
    collections: &BTreeMap<HostNodeId, MountedCollection>,
) -> Vec<MountedTreeProjection> {
    collections
        .iter()
        .filter_map(|(collection_node, collection)| {
            let state = collection.tree.as_ref()?;
            Some(MountedTreeProjection {
                collection: *collection_node,
                expanded_keys: state.expanded_keys.clone(),
                items: collection
                    .items
                    .iter()
                    .map(|item| MountedTreeItemProjection {
                        node: item.node,
                        expanded: item
                            .has_child_items
                            .then(|| state.expanded_keys.contains(&item.key)),
                        hidden: !is_visible(collection, item),
                    })
                    .collect(),
            })
        })
        .collect()
}

pub(super) fn focus_fallback(
    collection: &MountedCollection,
    focused: HostNodeId,
) -> Option<HostNodeId> {
    let item = collection.items.iter().find(|item| item.node == focused)?;
    if is_visible(collection, item) {
        return None;
    }

    let mut parent = item.parent_key.as_ref();
    for _ in 0..collection.items.len() {
        let Some(parent_key) = parent else {
            break;
        };
        let Some(parent_item) = collection
            .items
            .iter()
            .find(|candidate| &candidate.key == parent_key)
        else {
            break;
        };
        if is_visible(collection, parent_item) && !collection.manager.is_disabled(&parent_item.key)
        {
            return Some(parent_item.node);
        }
        parent = parent_item.parent_key.as_ref();
    }
    visible_focusable_keys(collection)
        .first()
        .and_then(|key| node_for_key(collection, key))
}

pub(crate) fn apply_item_tree_props(props: &mut NativeProps, expanded: Option<bool>, hidden: bool) {
    props.expanded = expanded;
    props.hidden = hidden;
    if let Some(expanded) = expanded {
        let value = expanded.to_string();
        props
            .web
            .attributes
            .insert("aria-expanded".to_string(), value.clone());
        props
            .web
            .attributes
            .insert("data-expanded".to_string(), value.clone());
        props
            .metadata
            .insert("aria-expanded".to_string(), value.clone());
        props.metadata.insert("data-expanded".to_string(), value);
    }
}

pub(super) fn step_target(
    collection: &MountedCollection,
    step: CollectionNavigationStep,
    current: Option<&CollectionKey>,
) -> Option<CollectionKey> {
    let keys = visible_focusable_keys(collection);
    match step {
        CollectionNavigationStep::First | CollectionNavigationStep::PageAbove => {
            keys.first().cloned().cloned()
        }
        CollectionNavigationStep::Last | CollectionNavigationStep::PageBelow => {
            keys.last().cloned().cloned()
        }
        CollectionNavigationStep::Next => {
            let Some(index) =
                current.and_then(|current| keys.iter().position(|key| *key == current))
            else {
                return keys.first().cloned().cloned();
            };
            keys.get(index.saturating_add(1))
                .or_else(|| {
                    collection
                        .navigation
                        .should_focus_wrap
                        .then(|| keys.first())
                        .flatten()
                })
                .cloned()
                .cloned()
        }
        CollectionNavigationStep::Previous => {
            let Some(index) =
                current.and_then(|current| keys.iter().position(|key| *key == current))
            else {
                return keys.last().cloned().cloned();
            };
            index
                .checked_sub(1)
                .and_then(|index| keys.get(index))
                .or_else(|| {
                    collection
                        .navigation
                        .should_focus_wrap
                        .then(|| keys.last())
                        .flatten()
                })
                .cloned()
                .cloned()
        }
    }
}

pub(super) fn horizontal_navigation(
    collection: &mut MountedCollection,
    event: &NativeEvent,
    direction: TextDirection,
    key: &str,
    current: Option<&CollectionKey>,
) -> Option<MountedSelectionNavigation> {
    let expand_key = if direction == TextDirection::Rtl {
        "ArrowLeft"
    } else {
        "ArrowRight"
    };
    let collapse_key = if direction == TextDirection::Rtl {
        "ArrowRight"
    } else {
        "ArrowLeft"
    };
    if key != expand_key && key != collapse_key {
        return None;
    }

    let current = current?.clone();
    let item = collection.items.iter().find(|item| item.key == current)?;
    let current_node = item.node;
    let has_child_items = item.has_child_items;
    let parent_key = item.parent_key.clone();
    let state = collection.tree.as_mut()?;

    if key == expand_key {
        if !has_child_items {
            return None;
        }
        if state.expanded_keys.insert(current.clone()) {
            return Some(MountedSelectionNavigation {
                target: current_node,
                select: false,
                expansion: Some(expansion_update(collection)),
                selection: None,
            });
        }
        let target_key = collection
            .items
            .iter()
            .find(|candidate| {
                candidate.parent_key.as_ref() == Some(&current)
                    && is_visible(collection, candidate)
                    && !collection.manager.is_disabled(&candidate.key)
            })?
            .key
            .clone();
        let target = node_for_key(collection, &target_key)?;
        return Some(MountedSelectionNavigation {
            target,
            select: should_select_during_navigation(collection, event, &target_key),
            expansion: None,
            selection: None,
        });
    }

    if has_child_items && state.expanded_keys.remove(&current) {
        return Some(MountedSelectionNavigation {
            target: current_node,
            select: false,
            expansion: Some(expansion_update(collection)),
            selection: None,
        });
    }
    let parent_key = parent_key?;
    if collection.manager.is_disabled(&parent_key) {
        return None;
    }
    let target = node_for_key(collection, &parent_key)?;
    Some(MountedSelectionNavigation {
        target,
        select: should_select_during_navigation(collection, event, &parent_key),
        expansion: None,
        selection: None,
    })
}

pub(super) fn is_visible(collection: &MountedCollection, item: &MountedSelectionItem) -> bool {
    if item.author_hidden {
        return false;
    }
    let Some(state) = collection.tree.as_ref() else {
        return true;
    };
    let mut parent = item.parent_key.as_ref();
    for _ in 0..collection.items.len() {
        let Some(parent_key) = parent else {
            return true;
        };
        let Some(parent_item) = collection
            .items
            .iter()
            .find(|candidate| &candidate.key == parent_key)
        else {
            return true;
        };
        if parent_item.author_hidden || !state.expanded_keys.contains(parent_key) {
            return false;
        }
        parent = parent_item.parent_key.as_ref();
    }
    false
}

pub(super) fn visible_focusable_keys(collection: &MountedCollection) -> Vec<&CollectionKey> {
    collection
        .items
        .iter()
        .filter(|item| is_visible(collection, item) && !collection.manager.is_disabled(&item.key))
        .map(|item| &item.key)
        .collect()
}

fn expansion_update(collection: &MountedCollection) -> MountedTreeExpansionUpdate {
    MountedTreeExpansionUpdate {
        collection: collection.node,
        expanded_keys: collection
            .tree
            .as_ref()
            .map(|state| state.expanded_keys.clone())
            .unwrap_or_default(),
    }
}

fn node_for_key(collection: &MountedCollection, key: &CollectionKey) -> Option<HostNodeId> {
    collection
        .items
        .iter()
        .find(|item| &item.key == key)
        .map(|item| item.node)
}

fn expanded_key_attribute(
    props: &NativeProps,
    names: &[&str],
    items: &[MountedSelectionItem],
) -> Option<BTreeSet<CollectionKey>> {
    let selection = attribute(props, names).and_then(decode_selection)?;
    selection.explicit_keys().map(|keys| {
        keys.iter()
            .map(|key| resolve_key_alias(key, items))
            .collect()
    })
}
