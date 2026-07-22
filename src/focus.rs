use std::collections::{BTreeMap, BTreeSet};

use crate::host::HostNodeId;
use crate::native::{NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::style::PortableStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusNavigationMode {
    /// Includes programmatically focusable nodes with a negative tab index.
    #[default]
    Focusable,
    /// Includes only nodes reachable through sequential keyboard navigation.
    Tabbable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFocusScope {
    pub node: HostNodeId,
    pub contain: bool,
    pub restore_focus: bool,
    pub auto_focus: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
struct FocusEntry {
    node: HostNodeId,
    parent: Option<HostNodeId>,
    tab_index: i32,
    auto_focus: bool,
    focusable: bool,
    tree_order: usize,
}

/// Portable focus navigation and scope semantics for the mounted native tree.
///
/// Methods return host node ids. Native adapters remain responsible for moving
/// platform focus to the selected node.
#[derive(Debug, Clone, Default)]
pub struct FocusManager {
    entries: Vec<FocusEntry>,
    entry_indices: BTreeMap<HostNodeId, usize>,
    scopes: BTreeMap<HostNodeId, NativeFocusScope>,
    restore_targets: BTreeMap<HostNodeId, Option<HostNodeId>>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_snapshot(snapshot: &[MountedNodeSnapshot]) -> Self {
        let mut manager = Self::new();
        manager.sync(snapshot);
        manager
    }

    pub fn sync(&mut self, snapshot: &[MountedNodeSnapshot]) {
        self.sync_with_focus(snapshot, None);
    }

    /// Synchronizes the mounted tree and returns a focus target when a
    /// restore-enabled scope was removed.
    pub fn sync_with_focus(
        &mut self,
        snapshot: &[MountedNodeSnapshot],
        current_focus: Option<HostNodeId>,
    ) -> Option<HostNodeId> {
        let previous_scopes = self.scopes.clone();
        let previous_restore_targets = std::mem::take(&mut self.restore_targets);
        let previous_scope_depths = previous_scopes
            .keys()
            .map(|scope| (*scope, self.ancestors(*scope).count()))
            .collect::<BTreeMap<_, _>>();
        let props_by_node = snapshot
            .iter()
            .map(|record| (record.node, &record.props))
            .collect::<BTreeMap<_, _>>();
        let parents = snapshot
            .iter()
            .map(|record| (record.node, record.parent))
            .collect::<BTreeMap<_, _>>();
        let mut availability = BTreeMap::new();
        let mut resolving = BTreeSet::new();
        for record in snapshot {
            resolve_availability(
                record.node,
                &props_by_node,
                &parents,
                &mut availability,
                &mut resolving,
            );
        }

        self.scopes.clear();
        for record in snapshot {
            if !is_focus_scope(&record.props) {
                continue;
            }
            self.scopes.insert(
                record.node,
                NativeFocusScope {
                    node: record.node,
                    contain: bool_marker(&record.props, "data-contain"),
                    restore_focus: bool_marker(&record.props, "data-restore-focus"),
                    auto_focus: record.props.auto_focus
                        || bool_marker(&record.props, "data-auto-focus"),
                    disabled: !availability.get(&record.node).copied().unwrap_or(false),
                },
            );
        }

        self.entries = snapshot
            .iter()
            .enumerate()
            .map(|(tree_order, record)| {
                let available = availability.get(&record.node).copied().unwrap_or(false);
                FocusEntry {
                    node: record.node,
                    parent: record.parent,
                    tab_index: record.props.tab_index.unwrap_or(0),
                    auto_focus: record.props.auto_focus,
                    focusable: available
                        && !is_focus_scope(&record.props)
                        && role_is_focusable(record.role, &record.props),
                    tree_order,
                }
            })
            .collect();
        self.entry_indices = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (entry.node, index))
            .collect();

        for (node, scope) in &self.scopes {
            if !scope.restore_focus || scope.disabled {
                continue;
            }
            let target = if previous_scopes
                .get(node)
                .is_some_and(|previous| previous.restore_focus && !previous.disabled)
            {
                previous_restore_targets.get(node).copied().flatten()
            } else {
                current_focus
                    .filter(|target| *target != *node && !self.is_descendant_of(*target, *node))
            };
            self.restore_targets.insert(*node, target);
        }

        let mut removed_scopes = previous_scopes
            .values()
            .filter(|scope| {
                scope.restore_focus && !scope.disabled && !self.scopes.contains_key(&scope.node)
            })
            .collect::<Vec<_>>();
        removed_scopes.sort_by_key(|scope| {
            std::cmp::Reverse(previous_scope_depths.get(&scope.node).copied().unwrap_or(0))
        });

        removed_scopes.into_iter().find_map(|scope| {
            previous_restore_targets
                .get(&scope.node)
                .copied()
                .flatten()
                .filter(|target| self.is_focusable(*target))
        })
    }

    pub fn scopes(&self) -> impl Iterator<Item = &NativeFocusScope> {
        self.scopes.values()
    }

    pub fn scope(&self, node: HostNodeId) -> Option<&NativeFocusScope> {
        self.scopes.get(&node)
    }

    pub fn is_focusable(&self, node: HostNodeId) -> bool {
        self.entry(node).is_some_and(|entry| entry.focusable)
    }

    pub fn is_tabbable(&self, node: HostNodeId) -> bool {
        self.entry(node)
            .is_some_and(|entry| entry.focusable && entry.tab_index >= 0)
    }

    pub fn containing_scope(&self, node: HostNodeId) -> Option<&NativeFocusScope> {
        self.ancestors(node)
            .find_map(|ancestor| self.scopes.get(&ancestor))
    }

    pub fn focusable_nodes(
        &self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
    ) -> Vec<HostNodeId> {
        let mut entries = self
            .entries
            .iter()
            .filter(|entry| {
                entry.focusable
                    && (mode == FocusNavigationMode::Focusable || entry.tab_index >= 0)
                    && scope.is_none_or(|scope| self.is_descendant_of(entry.node, scope))
            })
            .collect::<Vec<_>>();
        if mode == FocusNavigationMode::Tabbable {
            entries.sort_by_key(|entry| {
                if entry.tab_index > 0 {
                    (0, entry.tab_index, entry.tree_order)
                } else {
                    (1, 0, entry.tree_order)
                }
            });
        }
        entries.into_iter().map(|entry| entry.node).collect()
    }

    pub fn first(
        &self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
    ) -> Option<HostNodeId> {
        self.focusable_nodes(scope, mode).into_iter().next()
    }

    pub fn last(&self, scope: Option<HostNodeId>, mode: FocusNavigationMode) -> Option<HostNodeId> {
        self.focusable_nodes(scope, mode).into_iter().next_back()
    }

    pub fn next(
        &self,
        from: HostNodeId,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
        wrap: bool,
    ) -> Option<HostNodeId> {
        let nodes = self.focusable_nodes(scope, mode);
        let Some(index) = nodes.iter().position(|node| *node == from) else {
            return nodes.first().copied();
        };
        nodes
            .get(index + 1)
            .copied()
            .or_else(|| wrap.then(|| nodes.first().copied()).flatten())
    }

    pub fn previous(
        &self,
        from: HostNodeId,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
        wrap: bool,
    ) -> Option<HostNodeId> {
        let nodes = self.focusable_nodes(scope, mode);
        let Some(index) = nodes.iter().position(|node| *node == from) else {
            return nodes.last().copied();
        };
        index
            .checked_sub(1)
            .and_then(|index| nodes.get(index).copied())
            .or_else(|| wrap.then(|| nodes.last().copied()).flatten())
    }

    /// Constrains a requested target to the nearest active contained scope.
    pub fn constrain_focus(
        &self,
        current: HostNodeId,
        requested: HostNodeId,
    ) -> Option<HostNodeId> {
        let scope = self.ancestors(current).find(|ancestor| {
            self.scopes
                .get(ancestor)
                .is_some_and(|scope| scope.contain && !scope.disabled)
        });
        let Some(scope) = scope else {
            return self.is_focusable(requested).then_some(requested);
        };
        if self.is_focusable(requested) && self.is_descendant_of(requested, scope) {
            return Some(requested);
        }
        self.is_focusable(current)
            .then_some(current)
            .or_else(|| self.first(Some(scope), FocusNavigationMode::Tabbable))
            .or_else(|| self.first(Some(scope), FocusNavigationMode::Focusable))
    }

    /// Returns the first target requested by an autofocus scope or node.
    pub fn auto_focus_target(&self) -> Option<HostNodeId> {
        for entry in &self.entries {
            if let Some(scope) = self.scopes.get(&entry.node) {
                if scope.auto_focus && !scope.disabled {
                    if let Some(target) = self
                        .first(Some(scope.node), FocusNavigationMode::Tabbable)
                        .or_else(|| self.first(Some(scope.node), FocusNavigationMode::Focusable))
                    {
                        return Some(target);
                    }
                }
            } else if entry.auto_focus && entry.focusable {
                return Some(entry.node);
            }
        }
        None
    }

    fn entry(&self, node: HostNodeId) -> Option<&FocusEntry> {
        self.entry_indices
            .get(&node)
            .and_then(|index| self.entries.get(*index))
    }

    fn ancestors(&self, node: HostNodeId) -> Ancestors<'_> {
        Ancestors {
            manager: self,
            next: self.entry(node).and_then(|entry| entry.parent),
            visited: BTreeSet::new(),
        }
    }

    fn is_descendant_of(&self, node: HostNodeId, ancestor: HostNodeId) -> bool {
        self.ancestors(node).any(|candidate| candidate == ancestor)
    }
}

struct Ancestors<'a> {
    manager: &'a FocusManager,
    next: Option<HostNodeId>,
    visited: BTreeSet<HostNodeId>,
}

impl Iterator for Ancestors<'_> {
    type Item = HostNodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        if !self.visited.insert(current) {
            self.next = None;
            return None;
        }
        self.next = self.manager.entry(current).and_then(|entry| entry.parent);
        Some(current)
    }
}

fn resolve_availability(
    node: HostNodeId,
    props_by_node: &BTreeMap<HostNodeId, &NativeProps>,
    parents: &BTreeMap<HostNodeId, Option<HostNodeId>>,
    availability: &mut BTreeMap<HostNodeId, bool>,
    resolving: &mut BTreeSet<HostNodeId>,
) -> bool {
    if let Some(available) = availability.get(&node) {
        return *available;
    }
    let Some(props) = props_by_node.get(&node) else {
        return false;
    };
    if !resolving.insert(node) {
        return false;
    }

    let parent_available = match parents.get(&node).copied().flatten() {
        Some(parent) => {
            resolve_availability(parent, props_by_node, parents, availability, resolving)
        }
        None => true,
    };
    let available = parent_available && props_are_available(props);
    resolving.remove(&node);
    availability.insert(node, available);
    available
}

fn is_focus_scope(props: &NativeProps) -> bool {
    bool_marker(props, "data-focus-scope")
}

fn bool_marker(props: &NativeProps, name: &str) -> bool {
    props
        .metadata
        .get(name)
        .or_else(|| props.web.attributes.get(name))
        .is_some_and(|value| {
            value.is_empty()
                || value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case(name)
        })
}

fn props_are_available(props: &NativeProps) -> bool {
    let style = PortableStyle::from_web(&props.web);
    !props.disabled
        && !props.hidden
        && !props.inert
        && props.html_dialog.open.unwrap_or(true)
        && style.renders_native_widget()
        && !style.makes_native_widget_inert()
}

fn role_is_focusable(role: NativeRole, props: &NativeProps) -> bool {
    if props.tab_index.is_some() {
        return true;
    }
    if props
        .content_editable
        .as_deref()
        .is_some_and(|value| !value.eq_ignore_ascii_case("false"))
    {
        return true;
    }
    matches!(
        role,
        NativeRole::Button
            | NativeRole::Link
            | NativeRole::ImageMapArea
            | NativeRole::TextField
            | NativeRole::Checkbox
            | NativeRole::Switch
            | NativeRole::Radio
            | NativeRole::Select
            | NativeRole::ComboBox
            | NativeRole::ListBox
            | NativeRole::ListBoxItem
            | NativeRole::TreeItem
            | NativeRole::DisclosureSummary
            | NativeRole::Tab
            | NativeRole::MenuItem
            | NativeRole::Slider
    )
}

#[cfg(test)]
mod tests;
