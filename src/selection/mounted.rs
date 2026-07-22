use std::collections::{BTreeMap, BTreeSet};

use std::time::Instant;

use crate::error::GuiResult;
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation;
use crate::host::HostNodeId;
use crate::input::NativeInputModality;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::style::TextDirection;

use super::typeahead::{find_match, TypeaheadCandidate, TypeaheadState};
use super::{
    CollectionItem, CollectionKey, DisabledBehavior, EscapeKeyBehavior, KeyedCollection, Selection,
    SelectionBehavior, SelectionManager, SelectionMode,
};

mod tree;
mod tree_navigation;
pub(crate) use tree::{apply_item_selection_props, validate_native_collection_keys};
pub(crate) use tree_navigation::{
    apply_item_tree_props, MountedTreeExpansionUpdate, MountedTreeProjection,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MountedSelectionItem {
    node: HostNodeId,
    key: CollectionKey,
    value: Option<String>,
    label: Option<String>,
    text_value: String,
    role: NativeRole,
    selected: bool,
    disabled: bool,
    parent_key: Option<CollectionKey>,
    level: u32,
    has_child_items: bool,
    author_hidden: bool,
    expanded: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MountedCollection {
    node: HostNodeId,
    manager: SelectionManager,
    items: Vec<MountedSelectionItem>,
    path: Vec<String>,
    navigation: CollectionNavigationConfig,
    typeahead: TypeaheadState,
    tree: Option<tree_navigation::TreeExpansionState>,
    touch_selection_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollectionNavigationConfig {
    owner_role: NativeRole,
    navigation_role: NativeRole,
    orientation: Orientation,
    should_focus_wrap: bool,
    keyboard_activation_manual: bool,
    escape_key_behavior: EscapeKeyBehavior,
}

/// Selection state attached to stable nodes in the currently mounted tree.
///
/// Collection identity is based on declarative element keys. Values and labels
/// are accepted only as compatibility aliases for native controls that report
/// their displayed value instead of the item key.
#[derive(Debug, Clone, Default)]
pub struct MountedSelectionRegistry {
    collections: BTreeMap<HostNodeId, MountedCollection>,
    item_owners: BTreeMap<HostNodeId, HostNodeId>,
    item_keys: BTreeMap<HostNodeId, CollectionKey>,
    item_action_owners: BTreeMap<HostNodeId, HostNodeId>,
    action_presses: BTreeMap<HostNodeId, MountedActionPress>,
    pending_selection_suppression: BTreeSet<HostNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MountedActionPress {
    item: HostNodeId,
    action_owner: HostNodeId,
    selection_owner: HostNodeId,
    key: CollectionKey,
    primary: bool,
    secondary: bool,
    modality: NativeInputModality,
    click_count: u8,
    touch_selection_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedCollectionAction {
    pub item: HostNodeId,
    pub key: CollectionKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedSelectionUpdate {
    pub collection: HostNodeId,
    pub mode: SelectionMode,
    pub selection: Selection,
    pub items: Vec<(HostNodeId, NativeRole, bool)>,
    pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedSelectionNavigation {
    pub target: HostNodeId,
    pub select: bool,
    pub expansion: Option<MountedTreeExpansionUpdate>,
    pub selection: Option<MountedKeyboardSelectionUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MountedKeyboardSelectionUpdate {
    pub collection: HostNodeId,
    pub selection: Selection,
}

impl MountedSelectionUpdate {
    pub fn event_value(&self) -> String {
        if self.mode == SelectionMode::Single {
            return match &self.selection {
                Selection::Keys(keys) => keys
                    .iter()
                    .next()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "[]".to_string()),
                Selection::All => "all".to_string(),
            };
        }

        serde_json::to_string(&self.selection).unwrap_or_else(|_| "[]".to_string())
    }
}

impl MountedKeyboardSelectionUpdate {
    pub fn event_value(&self) -> String {
        serde_json::to_string(&self.selection).unwrap_or_else(|_| "[]".to_string())
    }
}

impl MountedCollectionAction {
    pub fn event_value(&self) -> String {
        self.key.to_string()
    }
}

impl MountedSelectionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn manager(&self, collection: HostNodeId) -> Option<&SelectionManager> {
        self.collections
            .get(&collection)
            .map(|collection| &collection.manager)
    }

    pub fn collection_for_item(&self, item: HostNodeId) -> Option<HostNodeId> {
        self.item_owners.get(&item).copied()
    }

    pub fn key_for_item(&self, item: HostNodeId) -> Option<&CollectionKey> {
        self.item_keys.get(&item)
    }

    pub(crate) fn has_action_for_item(&self, item: HostNodeId) -> bool {
        self.action_policy(item).is_some()
    }

    pub(crate) fn begin_action_press(&mut self, event: &NativeEvent) {
        if event.kind != NativeEventKind::PressStart {
            return;
        }
        let Some(press) = self.action_press(event) else {
            return;
        };

        self.action_presses
            .retain(|_, active| active.selection_owner != press.selection_owner);
        self.pending_selection_suppression
            .remove(&press.selection_owner);
        if suppresses_native_selection(&press) {
            self.pending_selection_suppression
                .insert(press.selection_owner);
        }
        self.action_presses.insert(press.item, press);
    }

    pub(crate) fn cancel_action_press(&mut self, event: &NativeEvent) {
        if event.kind != NativeEventKind::PressCancel {
            return;
        }
        if let Some(press) = self.action_presses.remove(&event.node) {
            self.pending_selection_suppression
                .remove(&press.selection_owner);
        }
    }

    pub(crate) fn take_action(&mut self, event: &NativeEvent) -> Option<MountedCollectionAction> {
        if event.kind != NativeEventKind::Press {
            return None;
        }
        let press = self
            .action_presses
            .remove(&event.node)
            .or_else(|| self.action_press(event))?;
        let click_count = event.context.click_count.max(press.click_count);
        let fires = match press.modality {
            NativeInputModality::Keyboard | NativeInputModality::Virtual => true,
            NativeInputModality::Touch | NativeInputModality::Pen => !press.touch_selection_mode,
            NativeInputModality::Mouse => press.primary || (press.secondary && click_count == 2),
            NativeInputModality::Unknown => press.primary,
        };
        fires.then_some(MountedCollectionAction {
            item: press.item,
            key: press.key,
        })
    }

    pub(crate) fn take_suppressed_native_selection(&mut self, event: &NativeEvent) -> bool {
        event.kind == NativeEventKind::SelectionChange
            && self.collections.contains_key(&event.node)
            && self.pending_selection_suppression.remove(&event.node)
    }

    pub(crate) fn take_long_press_selection(&mut self, event: &NativeEvent) -> bool {
        if event.kind != NativeEventKind::LongPress
            || !matches!(
                event.effective_modality(),
                NativeInputModality::Touch | NativeInputModality::Pen
            )
        {
            return false;
        }
        let Some((_, selection_owner, key, _, _)) = self.action_policy(event.node) else {
            return false;
        };
        let Some(collection) = self.collections.get_mut(&selection_owner) else {
            return false;
        };
        if !collection.manager.can_select_item(&key) {
            return false;
        }

        collection.touch_selection_mode = true;
        self.action_presses.remove(&event.node);
        true
    }

    pub fn collection_nodes(&self) -> impl Iterator<Item = HostNodeId> + '_ {
        self.collections.keys().copied()
    }

    pub(crate) fn projections(&self) -> Vec<MountedSelectionUpdate> {
        self.collections
            .iter()
            .map(|(node, collection)| MountedSelectionUpdate {
                collection: *node,
                mode: collection.manager.mode(),
                selection: collection.manager.selection().clone(),
                items: collection
                    .items
                    .iter()
                    .map(|item| {
                        (
                            item.node,
                            item.role,
                            collection.manager.is_selected(&item.key),
                        )
                    })
                    .collect(),
                changed: false,
            })
            .collect()
    }

    pub(crate) fn project_native_tree(&self, root: &mut NativeElement) {
        let collections = self
            .collections
            .values()
            .map(|collection| (collection.path.clone(), collection))
            .collect::<BTreeMap<_, _>>();
        tree::project_native_tree(root, &collections);
    }

    pub(crate) fn tree_projections(&self) -> Vec<MountedTreeProjection> {
        tree_navigation::projections(&self.collections)
    }

    pub(crate) fn tree_focus_fallback(&self, focused: HostNodeId) -> Option<HostNodeId> {
        let collection = self.item_owners.get(&focused)?;
        tree_navigation::focus_fallback(self.collections.get(collection)?, focused)
    }

    pub(crate) fn sync(&mut self, snapshot: &[MountedNodeSnapshot]) -> GuiResult<()> {
        let nodes = snapshot
            .iter()
            .map(|node| (node.node, node))
            .collect::<BTreeMap<_, _>>();
        let mut grouped = snapshot
            .iter()
            .filter(|node| is_selection_container(node.role))
            .map(|node| (node.node, Vec::new()))
            .collect::<BTreeMap<_, Vec<&MountedNodeSnapshot>>>();

        for item in snapshot.iter().filter(|node| is_selection_item(node.role)) {
            if let Some(owner) = selection_owner(item.parent, &nodes) {
                grouped.entry(owner).or_default().push(item);
            }
        }

        let mut previous = std::mem::take(&mut self.collections);
        let mut collections = BTreeMap::new();
        let mut item_owners = BTreeMap::new();
        let mut item_keys = BTreeMap::new();
        let mut item_action_owners = BTreeMap::new();

        for (collection_node, item_snapshots) in grouped {
            let Some(container) = nodes.get(&collection_node).copied() else {
                continue;
            };
            let previous_collection = previous.remove(&collection_node);
            let was_mounted = previous_collection.is_some();
            let (mut manager, typeahead, previous_tree, mut touch_selection_mode) =
                previous_collection
                    .map(|collection| {
                        (
                            collection.manager,
                            collection.typeahead,
                            collection.tree,
                            collection.touch_selection_mode,
                        )
                    })
                    .unwrap_or_else(|| {
                        (
                            SelectionManager::new(selection_mode(&container.props)),
                            TypeaheadState::default(),
                            None,
                            false,
                        )
                    });
            let mut items = item_snapshots
                .iter()
                .map(|item| MountedSelectionItem {
                    node: item.node,
                    key: collection_key(item.key.as_str(), &item.props),
                    value: item.props.value.clone(),
                    label: item.props.label.clone(),
                    text_value: collection_item_text(&item.props),
                    role: item.role,
                    selected: item.props.selected || item.props.checked == Some(true),
                    disabled: item.props.disabled,
                    parent_key: tree_parent_key(item, &nodes),
                    level: tree_level(item, &nodes),
                    has_child_items: optional_boolean_attribute(
                        &item.props,
                        &["hasChildItems", "data-has-child-items"],
                    )
                    .unwrap_or(item.props.expanded.is_some()),
                    author_hidden: item.props.hidden,
                    expanded: item.props.expanded,
                })
                .collect::<Vec<_>>();
            tree_navigation::normalize_items(&mut items);
            let collection = KeyedCollection::new(items.iter().map(|item| {
                CollectionItem::new(item.key.clone(), item.node).disabled(item.disabled)
            }))?;
            let path = tree::mounted_node_path(collection_node, &nodes);
            let navigation_node = navigation_container(container, &item_snapshots, &nodes);
            let navigation = navigation_config(container, navigation_node);
            let tree =
                tree_navigation::configure_state(container, &items, previous_tree, was_mounted);

            configure_manager(
                &mut manager,
                &container.props,
                &items,
                &collection,
                was_mounted,
            )?;
            touch_selection_mode &= !manager.selection().is_empty();

            for item in &items {
                item_owners.insert(item.node, collection_node);
                item_keys.insert(item.node, item.key.clone());
            }
            for item in &item_snapshots {
                if let Some(action_owner) = collection_action_owner(item.parent, &nodes) {
                    item_action_owners.insert(item.node, action_owner);
                }
            }
            collections.insert(
                collection_node,
                MountedCollection {
                    node: collection_node,
                    manager,
                    items,
                    path,
                    navigation,
                    typeahead,
                    tree,
                    touch_selection_mode,
                },
            );
        }

        self.collections = collections;
        self.item_owners = item_owners;
        self.item_keys = item_keys;
        self.action_presses.retain(|item, press| {
            item_action_owners.get(item) == Some(&press.action_owner)
                && self.item_owners.get(item) == Some(&press.selection_owner)
                && self.item_keys.get(item) == Some(&press.key)
        });
        self.pending_selection_suppression
            .retain(|collection| self.collections.contains_key(collection));
        self.item_action_owners = item_action_owners;
        Ok(())
    }

    fn action_policy(
        &self,
        item: HostNodeId,
    ) -> Option<(HostNodeId, HostNodeId, CollectionKey, bool, bool)> {
        let action_owner = self.item_action_owners.get(&item).copied()?;
        let selection_owner = self.item_owners.get(&item).copied()?;
        let key = self.item_keys.get(&item)?.clone();
        let collection = self.collections.get(&selection_owner)?;
        let mounted_item = collection
            .items
            .iter()
            .find(|candidate| candidate.node == item)?;
        if mounted_item.disabled || collection.manager.is_disabled(&key) {
            return None;
        }

        let allows_selection = collection.manager.can_select_item(&key);
        let primary = if collection.manager.behavior() == SelectionBehavior::Replace {
            !allows_selection
        } else {
            !allows_selection || collection.manager.selection().is_empty()
        };
        let secondary =
            allows_selection && collection.manager.behavior() == SelectionBehavior::Replace;
        Some((action_owner, selection_owner, key, primary, secondary))
    }

    fn action_press(&self, event: &NativeEvent) -> Option<MountedActionPress> {
        let (action_owner, selection_owner, key, primary, secondary) =
            self.action_policy(event.node)?;
        let touch_selection_mode = self.collections.get(&selection_owner)?.touch_selection_mode;
        Some(MountedActionPress {
            item: event.node,
            action_owner,
            selection_owner,
            key,
            primary,
            secondary,
            modality: event.effective_modality(),
            click_count: event.context.click_count,
            touch_selection_mode,
        })
    }

    pub(crate) fn apply_event(
        &mut self,
        event: &NativeEvent,
    ) -> GuiResult<Option<MountedSelectionUpdate>> {
        let collection_node = self.item_owners.get(&event.node).copied().or_else(|| {
            self.collections
                .contains_key(&event.node)
                .then_some(event.node)
        });
        let Some(collection_node) = collection_node else {
            return Ok(None);
        };
        let item_key = self.item_keys.get(&event.node).cloned();
        let collection = self
            .collections
            .get_mut(&collection_node)
            .expect("mounted selection owner must reference a collection");
        let before = collection.manager.selection().clone();

        let handled = if let Some(key) = item_key {
            apply_key_event(&mut collection.manager, &key, event)
        } else if let Some(selection) =
            aggregate_selection(event.value.as_deref(), &collection.items)
        {
            collection.manager.set_selection(normalize_for_mode(
                collection.manager.mode(),
                selection,
                &collection.items,
            ))?
        } else if let Some(key) = event
            .value
            .as_deref()
            .and_then(|value| resolve_item_key(value, &collection.items))
        {
            apply_key_event(&mut collection.manager, &key, event)
        } else {
            return Ok(None);
        };

        let selection = collection.manager.selection().clone();
        if selection.is_empty() {
            collection.touch_selection_mode = false;
        }
        let items = collection
            .items
            .iter()
            .map(|item| {
                (
                    item.node,
                    item.role,
                    collection.manager.is_selected(&item.key),
                )
            })
            .collect();
        Ok(Some(MountedSelectionUpdate {
            collection: collection_node,
            mode: collection.manager.mode(),
            changed: handled && selection != before,
            selection,
            items,
        }))
    }

    pub(crate) fn apply_focus_event(&mut self, event: &NativeEvent) -> bool {
        if !matches!(
            event.kind,
            crate::event::NativeEventKind::Focus | crate::event::NativeEventKind::Blur
        ) {
            return false;
        }
        let collection_node = self.item_owners.get(&event.node).copied().or_else(|| {
            self.collections
                .contains_key(&event.node)
                .then_some(event.node)
        });
        let Some(collection_node) = collection_node else {
            return false;
        };
        let item_key = self.item_keys.get(&event.node).cloned();
        let collection = self
            .collections
            .get_mut(&collection_node)
            .expect("mounted focus owner must reference a collection");

        match event.kind {
            crate::event::NativeEventKind::Focus => {
                let key_changed = item_key
                    .map(|key| collection.manager.set_focused_key(Some(key)))
                    .unwrap_or(false);
                collection.manager.set_focused(true) || key_changed
            }
            crate::event::NativeEventKind::Blur => collection.manager.set_focused(false),
            _ => false,
        }
    }

    #[cfg(test)]
    pub(crate) fn keyboard_navigation(
        &mut self,
        event: &NativeEvent,
        direction: TextDirection,
    ) -> Option<MountedSelectionNavigation> {
        self.keyboard_navigation_with_locale(event, direction, None)
    }

    pub(crate) fn keyboard_navigation_with_locale(
        &mut self,
        event: &NativeEvent,
        direction: TextDirection,
        locale: Option<&str>,
    ) -> Option<MountedSelectionNavigation> {
        if event.kind != crate::event::NativeEventKind::KeyDown {
            return None;
        }
        let collection_node = self.item_owners.get(&event.node).copied().or_else(|| {
            self.collections
                .contains_key(&event.node)
                .then_some(event.node)
        })?;
        let key = event.value.as_deref().map(crate::event::native_key_value)?;
        let event_item_key = self.item_keys.get(&event.node).cloned();
        let collection = self.collections.get_mut(&collection_node)?;
        let current_key = event_item_key
            .or_else(|| collection.manager.focused_key().cloned())
            .or_else(|| collection.manager.first_selected_key().cloned());
        let current_key_ref = current_key.as_ref();
        if let Some(selection) = keyboard_selection_update(collection, event, &key) {
            return Some(MountedSelectionNavigation {
                target: event.node,
                select: false,
                expansion: None,
                selection: Some(selection),
            });
        }
        if collection.navigation.navigation_role == NativeRole::Tree {
            if let Some(navigation) = tree_navigation::horizontal_navigation(
                collection,
                event,
                direction,
                &key,
                current_key_ref,
            ) {
                return Some(navigation);
            }
        }
        let target_key =
            if let Some(step) = navigation_step(&collection.navigation, direction, &key) {
                if collection.navigation.navigation_role == NativeRole::Tree {
                    tree_navigation::step_target(collection, step, current_key_ref)
                } else {
                    match step {
                        CollectionNavigationStep::First => collection.manager.first_focusable_key(),
                        CollectionNavigationStep::Last => collection.manager.last_focusable_key(),
                        CollectionNavigationStep::Next => collection.manager.next_focusable_key(
                            current_key_ref,
                            collection.navigation.should_focus_wrap,
                        ),
                        CollectionNavigationStep::Previous => {
                            collection.manager.previous_focusable_key(
                                current_key_ref,
                                collection.navigation.should_focus_wrap,
                            )
                        }
                    }
                    .cloned()
                }
            } else {
                typeahead_target(
                    collection,
                    event,
                    &key,
                    current_key_ref,
                    locale,
                    Instant::now(),
                )
            }?;
        let target = collection
            .items
            .iter()
            .find(|item| item.key == target_key)?
            .node;
        if target == event.node {
            return None;
        }

        Some(MountedSelectionNavigation {
            target,
            select: should_select_during_navigation(collection, event, &target_key),
            expansion: None,
            selection: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollectionNavigationStep {
    First,
    Last,
    Next,
    Previous,
}

fn navigation_container<'a>(
    container: &'a MountedNodeSnapshot,
    items: &[&'a MountedNodeSnapshot],
    nodes: &BTreeMap<HostNodeId, &'a MountedNodeSnapshot>,
) -> &'a MountedNodeSnapshot {
    let mut parent = items.first().and_then(|item| item.parent);
    while let Some(node) = parent.and_then(|parent| nodes.get(&parent).copied()) {
        if is_selection_container(node.role) {
            return node;
        }
        if node.node == container.node {
            break;
        }
        parent = node.parent;
    }
    container
}

fn navigation_config(
    owner: &MountedNodeSnapshot,
    navigation: &MountedNodeSnapshot,
) -> CollectionNavigationConfig {
    let orientation = navigation
        .props
        .orientation
        .or(owner.props.orientation)
        .unwrap_or_else(|| match navigation.role {
            NativeRole::Tabs | NativeRole::TabList => Orientation::Horizontal,
            _ => Orientation::Vertical,
        });
    let should_focus_wrap = optional_boolean_attribute(
        &navigation.props,
        &["shouldFocusWrap", "data-should-focus-wrap"],
    )
    .or_else(|| {
        (navigation.node != owner.node)
            .then(|| {
                optional_boolean_attribute(
                    &owner.props,
                    &["shouldFocusWrap", "data-should-focus-wrap"],
                )
            })
            .flatten()
    })
    .unwrap_or(matches!(
        navigation.role,
        NativeRole::RadioGroup | NativeRole::Tabs | NativeRole::TabList
    ));
    let keyboard_activation_manual = attribute(
        &owner.props,
        &["keyboardActivation", "data-keyboard-activation"],
    )
    .or_else(|| {
        attribute(
            &navigation.props,
            &["keyboardActivation", "data-keyboard-activation"],
        )
    })
    .is_some_and(|activation| activation.trim().eq_ignore_ascii_case("manual"));
    let escape_key_behavior = attribute(
        &navigation.props,
        &["escapeKeyBehavior", "data-escape-key-behavior"],
    )
    .or_else(|| {
        (navigation.node != owner.node)
            .then(|| {
                attribute(
                    &owner.props,
                    &["escapeKeyBehavior", "data-escape-key-behavior"],
                )
            })
            .flatten()
    });

    CollectionNavigationConfig {
        owner_role: owner.role,
        navigation_role: navigation.role,
        orientation,
        should_focus_wrap,
        keyboard_activation_manual,
        escape_key_behavior: EscapeKeyBehavior::from_name(escape_key_behavior),
    }
}

fn keyboard_selection_update(
    collection: &MountedCollection,
    event: &NativeEvent,
    key: &str,
) -> Option<MountedKeyboardSelectionUpdate> {
    if !matches!(
        collection.navigation.navigation_role,
        NativeRole::ListBox | NativeRole::Tree
    ) || matches!(
        collection.navigation.owner_role,
        NativeRole::Select | NativeRole::ComboBox | NativeRole::Menu
    ) {
        return None;
    }

    let selection = if key == "Escape"
        && collection.navigation.escape_key_behavior == EscapeKeyBehavior::ClearSelection
    {
        Selection::empty()
    } else if key.eq_ignore_ascii_case("a")
        && (event.context.modifiers.control || event.context.modifiers.meta)
        && !event.context.modifiers.alt
        && collection.manager.mode() == SelectionMode::Multiple
    {
        Selection::All
    } else {
        return None;
    };

    Some(MountedKeyboardSelectionUpdate {
        collection: collection.node,
        selection,
    })
}

fn navigation_step(
    config: &CollectionNavigationConfig,
    direction: TextDirection,
    key: &str,
) -> Option<CollectionNavigationStep> {
    match key {
        "Home" | "PageUp" => return Some(CollectionNavigationStep::First),
        "End" | "PageDown" => return Some(CollectionNavigationStep::Last),
        _ => {}
    }

    if config.navigation_role == NativeRole::RadioGroup {
        return match key {
            "ArrowUp" => Some(CollectionNavigationStep::Previous),
            "ArrowDown" => Some(CollectionNavigationStep::Next),
            "ArrowLeft" if direction == TextDirection::Rtl => Some(CollectionNavigationStep::Next),
            "ArrowLeft" => Some(CollectionNavigationStep::Previous),
            "ArrowRight" if direction == TextDirection::Rtl => {
                Some(CollectionNavigationStep::Previous)
            }
            "ArrowRight" => Some(CollectionNavigationStep::Next),
            _ => None,
        };
    }

    match config.orientation {
        Orientation::Vertical => match key {
            "ArrowUp" => Some(CollectionNavigationStep::Previous),
            "ArrowDown" => Some(CollectionNavigationStep::Next),
            _ => None,
        },
        Orientation::Horizontal => match key {
            "ArrowLeft" if direction == TextDirection::Rtl => Some(CollectionNavigationStep::Next),
            "ArrowLeft" => Some(CollectionNavigationStep::Previous),
            "ArrowRight" if direction == TextDirection::Rtl => {
                Some(CollectionNavigationStep::Previous)
            }
            "ArrowRight" => Some(CollectionNavigationStep::Next),
            _ => None,
        },
    }
}

fn typeahead_target(
    collection: &mut MountedCollection,
    event: &NativeEvent,
    key: &str,
    current_key: Option<&CollectionKey>,
    locale: Option<&str>,
    now: Instant,
) -> Option<CollectionKey> {
    if event.context.modifiers.control
        || event.context.modifiers.meta
        || !matches!(
            collection.navigation.navigation_role,
            NativeRole::ListBox | NativeRole::Menu | NativeRole::Tree
        )
    {
        return None;
    }
    let search = collection.typeahead.push(key, now)?.to_string();
    let candidates = collection
        .items
        .iter()
        .map(|item| TypeaheadCandidate {
            key: &item.key,
            text: &item.text_value,
            disabled: collection.manager.is_disabled(&item.key)
                || (collection.navigation.navigation_role == NativeRole::Tree
                    && !tree_navigation::is_visible(collection, item)),
        })
        .collect::<Vec<_>>();
    find_match(&candidates, &search, current_key, locale)
}

fn should_select_during_navigation(
    collection: &MountedCollection,
    event: &NativeEvent,
    target: &CollectionKey,
) -> bool {
    if !collection.manager.can_select_item(target) {
        return false;
    }
    if event.context.modifiers.control || event.context.modifiers.meta {
        return false;
    }
    if event.context.modifiers.shift {
        return collection.manager.mode() == SelectionMode::Multiple
            && !matches!(
                collection.navigation.owner_role,
                NativeRole::Select | NativeRole::ComboBox | NativeRole::Menu
            );
    }

    match collection.navigation.navigation_role {
        NativeRole::Tabs | NativeRole::TabList => {
            collection.manager.mode() != SelectionMode::None
                && !collection.navigation.keyboard_activation_manual
        }
        NativeRole::RadioGroup => collection.manager.mode() != SelectionMode::None,
        NativeRole::ListBox | NativeRole::Tree => {
            !matches!(
                collection.navigation.owner_role,
                NativeRole::Select | NativeRole::ComboBox
            ) && collection.manager.mode() != SelectionMode::None
                && collection.manager.behavior() == SelectionBehavior::Replace
        }
        _ => false,
    }
}

fn configure_manager(
    manager: &mut SelectionManager,
    props: &NativeProps,
    items: &[MountedSelectionItem],
    collection: &KeyedCollection<HostNodeId>,
    was_mounted: bool,
) -> GuiResult<()> {
    let mode = selection_mode(props);
    manager.set_mode(mode);
    manager.set_selection_behavior(selection_behavior(props, mode));
    manager.set_disabled_behavior(disabled_behavior(props));
    manager.set_disallow_empty_selection(false);
    manager.sync_collection(collection);
    manager.set_disabled_keys(
        attribute(props, &["disabledKeys", "data-disabled-keys"])
            .and_then(decode_selection)
            .and_then(|selection| selection.explicit_keys().cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|key| resolve_key_alias(&key, items)),
    );

    let controlled = attribute(props, &["selectedKeys", "data-selected-keys"])
        .and_then(decode_selection)
        .map(|selection| resolve_selection_aliases(selection, items))
        .or_else(|| {
            props
                .value
                .as_deref()
                .filter(|value| !value.is_empty())
                .map(|value| {
                    Selection::keys([
                        resolve_item_key(value, items).unwrap_or_else(|| CollectionKey::new(value))
                    ])
                })
        });
    let initial = (!was_mounted)
        .then(|| {
            attribute(
                props,
                &["defaultSelectedKeys", "data-default-selected-keys"],
            )
            .and_then(decode_selection)
            .map(|selection| resolve_selection_aliases(selection, items))
            .or_else(|| {
                attribute(props, &["defaultValue", "data-default-value"])
                    .filter(|value| !value.is_empty())
                    .map(|value| {
                        Selection::keys([resolve_item_key(value, items)
                            .unwrap_or_else(|| CollectionKey::new(value))])
                    })
            })
            .or_else(|| {
                let keys = items
                    .iter()
                    .filter(|item| item.selected)
                    .map(|item| item.key.clone())
                    .collect::<BTreeSet<_>>();
                (!keys.is_empty()).then_some(Selection::Keys(keys))
            })
        })
        .flatten();
    if let Some(selection) = controlled.or(initial) {
        manager.set_selection(normalize_for_mode(mode, selection, items))?;
    }
    manager.set_disallow_empty_selection(boolean_attribute(
        props,
        &["disallowEmptySelection", "data-disallow-empty-selection"],
    ));
    Ok(())
}

fn apply_key_event(
    manager: &mut SelectionManager,
    key: &CollectionKey,
    event: &NativeEvent,
) -> bool {
    manager.set_focused_key(Some(key.clone()));
    manager.set_focused(true);
    if event.context.modifiers.shift {
        manager.extend_selection(key)
    } else if event.context.modifiers.control || event.context.modifiers.meta {
        manager.toggle_selection(key)
    } else {
        manager.select(key)
    }
}

fn aggregate_selection(value: Option<&str>, items: &[MountedSelectionItem]) -> Option<Selection> {
    let value = value?.trim();
    if !(value.starts_with('[')
        || value.eq_ignore_ascii_case("all")
        || value.eq_ignore_ascii_case("\"all\""))
    {
        return None;
    }
    decode_selection(value).map(|selection| resolve_selection_aliases(selection, items))
}

fn decode_selection(value: &str) -> Option<Selection> {
    serde_json::from_str(value)
        .ok()
        .or_else(|| value.eq_ignore_ascii_case("all").then_some(Selection::All))
}

fn resolve_selection_aliases(selection: Selection, items: &[MountedSelectionItem]) -> Selection {
    match selection {
        Selection::All => Selection::All,
        Selection::Keys(keys) => {
            Selection::keys(keys.into_iter().map(|key| resolve_key_alias(&key, items)))
        }
    }
}

fn resolve_key_alias(key: &CollectionKey, items: &[MountedSelectionItem]) -> CollectionKey {
    resolve_item_key(key.as_str(), items).unwrap_or_else(|| key.clone())
}

fn resolve_item_key(value: &str, items: &[MountedSelectionItem]) -> Option<CollectionKey> {
    items
        .iter()
        .find(|item| item.key.as_str() == value)
        .or_else(|| {
            items
                .iter()
                .find(|item| item.value.as_deref() == Some(value))
        })
        .or_else(|| {
            items
                .iter()
                .find(|item| item.label.as_deref() == Some(value))
        })
        .map(|item| item.key.clone())
}

fn collection_item_text(props: &NativeProps) -> String {
    attribute(props, &["textValue", "data-text-value"])
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .or_else(|| props.label.clone().filter(|value| !value.trim().is_empty()))
        .or_else(|| props.value.clone().filter(|value| !value.trim().is_empty()))
        .unwrap_or_default()
}

fn normalize_for_mode(
    mode: SelectionMode,
    selection: Selection,
    items: &[MountedSelectionItem],
) -> Selection {
    match mode {
        SelectionMode::None => Selection::empty(),
        SelectionMode::Multiple => selection,
        SelectionMode::Single => {
            let key = match selection {
                Selection::All => items.first().map(|item| item.key.clone()),
                Selection::Keys(keys) => items
                    .iter()
                    .find(|item| keys.contains(&item.key))
                    .map(|item| item.key.clone())
                    .or_else(|| keys.into_iter().next()),
            };
            key.map(|key| Selection::keys([key]))
                .unwrap_or_else(Selection::empty)
        }
    }
}

fn selection_owner(
    mut parent: Option<HostNodeId>,
    nodes: &BTreeMap<HostNodeId, &MountedNodeSnapshot>,
) -> Option<HostNodeId> {
    let mut nearest = None;
    while let Some(node) = parent.and_then(|node| nodes.get(&node).copied()) {
        if is_selection_container(node.role) {
            if nearest.is_none() {
                nearest = Some((node.node, node.role));
            }
            if nearest.is_some_and(|(_, role)| role == NativeRole::TabList)
                && node.role == NativeRole::Tabs
            {
                return Some(node.node);
            }
            if nearest.is_some_and(|(_, role)| role == NativeRole::ListBox)
                && matches!(node.role, NativeRole::ComboBox | NativeRole::Select)
            {
                return Some(node.node);
            }
        }
        parent = node.parent;
    }
    nearest.map(|(node, _)| node)
}

fn collection_action_owner(
    mut parent: Option<HostNodeId>,
    nodes: &BTreeMap<HostNodeId, &MountedNodeSnapshot>,
) -> Option<HostNodeId> {
    while let Some(node) = parent.and_then(|node| nodes.get(&node).copied()) {
        if is_selection_container(node.role)
            && node
                .props
                .web
                .events
                .get("onAction")
                .is_some_and(|action| !action.is_empty())
        {
            return Some(node.node);
        }
        parent = node.parent;
    }
    None
}

fn suppresses_native_selection(press: &MountedActionPress) -> bool {
    match press.modality {
        NativeInputModality::Keyboard | NativeInputModality::Virtual => true,
        NativeInputModality::Touch | NativeInputModality::Pen => !press.touch_selection_mode,
        NativeInputModality::Mouse | NativeInputModality::Unknown => press.primary,
    }
}

fn tree_parent_key(
    item: &MountedNodeSnapshot,
    nodes: &BTreeMap<HostNodeId, &MountedNodeSnapshot>,
) -> Option<CollectionKey> {
    if let Some(parent) = attribute(&item.props, &["data-tree-parent-key"]) {
        return Some(CollectionKey::new(parent));
    }
    let mut parent = item.parent;
    while let Some(snapshot) = parent.and_then(|node| nodes.get(&node).copied()) {
        if snapshot.role == NativeRole::TreeItem {
            return Some(collection_key(snapshot.key.as_str(), &snapshot.props));
        }
        if is_selection_container(snapshot.role) {
            break;
        }
        parent = snapshot.parent;
    }
    None
}

fn tree_level(
    item: &MountedNodeSnapshot,
    nodes: &BTreeMap<HostNodeId, &MountedNodeSnapshot>,
) -> u32 {
    if let Some(level) = attribute(&item.props, &["data-tree-level"])
        .and_then(|level| level.parse::<u32>().ok())
        .filter(|level| *level > 0)
    {
        return level;
    }
    let mut level = 1_u32;
    let mut parent = item.parent;
    while let Some(snapshot) = parent.and_then(|node| nodes.get(&node).copied()) {
        if snapshot.role == NativeRole::TreeItem {
            level = level.saturating_add(1);
        }
        if is_selection_container(snapshot.role) {
            break;
        }
        parent = snapshot.parent;
    }
    level
}

fn selection_mode(props: &NativeProps) -> SelectionMode {
    if props.multiple {
        return SelectionMode::Multiple;
    }
    SelectionMode::from_name(attribute(props, &["selectionMode", "data-selection-mode"]))
}

fn selection_behavior(props: &NativeProps, mode: SelectionMode) -> SelectionBehavior {
    match attribute(props, &["selectionBehavior", "data-selection-behavior"]).map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("toggle") => SelectionBehavior::Toggle,
        Some(value) if value.eq_ignore_ascii_case("replace") => SelectionBehavior::Replace,
        _ if mode == SelectionMode::Multiple => SelectionBehavior::Toggle,
        _ => SelectionBehavior::Replace,
    }
}

fn disabled_behavior(props: &NativeProps) -> DisabledBehavior {
    match attribute(props, &["disabledBehavior", "data-disabled-behavior"]).map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("selection") => DisabledBehavior::Selection,
        _ => DisabledBehavior::All,
    }
}

fn boolean_attribute(props: &NativeProps, names: &[&str]) -> bool {
    attribute(props, names).is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "" | "true" | "1" | "yes"
        )
    })
}

fn optional_boolean_attribute(props: &NativeProps, names: &[&str]) -> Option<bool> {
    attribute(props, names).map(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "" | "true" | "1" | "yes"
        )
    })
}

fn attribute<'a>(props: &'a NativeProps, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|name| {
        props
            .web
            .attributes
            .get(*name)
            .or_else(|| props.metadata.get(*name))
            .map(String::as_str)
    })
}

fn collection_key(element_key: &str, props: &NativeProps) -> CollectionKey {
    attribute(props, &["data-collection-key"])
        .filter(|key| !key.is_empty())
        .map(CollectionKey::new)
        .unwrap_or_else(|| CollectionKey::new(element_key))
}

pub(crate) fn is_selection_item(role: NativeRole) -> bool {
    matches!(
        role,
        NativeRole::ListBoxItem
            | NativeRole::TreeItem
            | NativeRole::MenuItem
            | NativeRole::Radio
            | NativeRole::Tab
    )
}

pub(crate) fn is_selection_container(role: NativeRole) -> bool {
    matches!(
        role,
        NativeRole::Select
            | NativeRole::ComboBox
            | NativeRole::ListBox
            | NativeRole::Tree
            | NativeRole::Menu
            | NativeRole::RadioGroup
            | NativeRole::Tabs
            | NativeRole::TabList
    )
}
