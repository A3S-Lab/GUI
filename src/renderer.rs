use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};
use crate::host::{HostNodeId, NativeHost};
use crate::native::{
    normalize_props_for_native_role, ElementKey, NativeElement, NativeProps, NativeRole,
};
use crate::platform::native_widget_kind;

mod snapshot;
pub use snapshot::MountedNodeSnapshot;

#[derive(Debug, Clone, PartialEq)]
struct MountedNode {
    id: HostNodeId,
    key: ElementKey,
    role: NativeRole,
    props: NativeProps,
    children: Vec<MountedNode>,
}

#[derive(Debug, Default)]
pub struct Renderer {
    root: Option<MountedNode>,
}

#[derive(Debug, Default)]
struct ReconcileRollback {
    new_mounts: Vec<MountedNode>,
    updated_props: Vec<(HostNodeId, NativeProps)>,
    child_orders: Vec<(HostNodeId, Vec<HostNodeId>)>,
    deferred_unmounts: Vec<MountedNode>,
}

impl ReconcileRollback {
    fn record_new_mount(&mut self, mounted: MountedNode) {
        self.new_mounts.push(mounted);
    }

    fn record_update(&mut self, id: HostNodeId, previous_props: NativeProps) {
        self.updated_props.push((id, previous_props));
    }

    fn record_child_order(&mut self, parent: HostNodeId, children: Vec<HostNodeId>) {
        if self
            .child_orders
            .iter()
            .any(|(recorded_parent, _)| *recorded_parent == parent)
        {
            return;
        }
        self.child_orders.push((parent, children));
    }

    fn record_deferred_unmount(&mut self, mounted: MountedNode) {
        self.deferred_unmounts.push(mounted);
    }

    fn commit_deferred_unmounts<H: NativeHost>(&mut self, host: &mut H) -> GuiResult<()> {
        for mounted in std::mem::take(&mut self.deferred_unmounts) {
            unmount_node(mounted, host)?;
        }
        Ok(())
    }

    fn rollback<H: NativeHost>(&mut self, host: &mut H) {
        for mounted in std::mem::take(&mut self.new_mounts).into_iter().rev() {
            best_effort_unmount_node(mounted, host);
        }
        for (parent, children) in std::mem::take(&mut self.child_orders).into_iter().rev() {
            restore_child_order(parent, children, host);
        }
        for (id, props) in std::mem::take(&mut self.updated_props).into_iter().rev() {
            let _ = host.update(id, &props);
        }
        self.deferred_unmounts.clear();
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render<H: NativeHost>(
        &mut self,
        element: &NativeElement,
        host: &mut H,
    ) -> GuiResult<HostNodeId> {
        let element = normalize_native_element(element);
        validate_native_tree(&element)?;
        let previous_root = self.root.clone();
        host.begin_frame()?;
        let render_result = (|| {
            let mut rollback = ReconcileRollback::default();
            let mounted = match self.root.take() {
                Some(old) if needs_replacement(&old, &element) => {
                    let mounted = match mount_node(None, 0, &element, host) {
                        Ok(mounted) => mounted,
                        Err(error) => {
                            self.root = Some(old);
                            return Err(error);
                        }
                    };
                    rollback.record_new_mount(mounted.clone());
                    rollback.record_deferred_unmount(old);
                    mounted
                }
                Some(old) => match reconcile_node(None, 0, old, &element, host, &mut rollback) {
                    Ok(mounted) => mounted,
                    Err(error) => {
                        rollback.rollback(host);
                        self.root = previous_root.clone();
                        return Err(error);
                    }
                },
                None => mount_node(None, 0, &element, host)?,
            };
            if let Err(error) = host.set_root(mounted.id) {
                if previous_root.is_none() {
                    best_effort_unmount_node(mounted, host);
                } else {
                    rollback.rollback(host);
                }
                self.root = previous_root.clone();
                return Err(error);
            }
            if let Err(error) = rollback.commit_deferred_unmounts(host) {
                rollback.rollback(host);
                if let Some(previous_root) = &previous_root {
                    let _ = host.set_root(previous_root.id);
                }
                self.root = previous_root.clone();
                return Err(error);
            }
            let root_id = mounted.id;
            self.root = Some(mounted);
            Ok(root_id)
        })();

        match render_result {
            Ok(root_id) => match host.commit_frame() {
                Ok(_) => Ok(root_id),
                Err(error) => {
                    self.root = previous_root;
                    rollback_host_frame(host, error)
                }
            },
            Err(error) => {
                self.root = previous_root;
                rollback_host_frame(host, error)
            }
        }
    }

    pub fn mounted_node_ids(&self) -> BTreeSet<HostNodeId> {
        let mut ids = BTreeSet::new();
        if let Some(root) = &self.root {
            collect_mounted_node_ids(root, &mut ids);
        }
        ids
    }

    pub fn mounted_node_props(&self) -> Vec<(HostNodeId, NativeProps)> {
        let mut props = Vec::new();
        if let Some(root) = &self.root {
            collect_mounted_node_props(root, &mut props);
        }
        props
    }

    pub fn ancestor_ids(&self, node: HostNodeId) -> Vec<HostNodeId> {
        let mut ancestors = Vec::new();
        if let Some(root) = &self.root {
            collect_ancestor_ids(root, node, &mut ancestors);
        }
        ancestors
    }

    pub fn child_ids(&self, node: HostNodeId) -> Vec<HostNodeId> {
        self.root
            .as_ref()
            .and_then(|root| find_mounted_node(root, node))
            .map(|mounted| mounted.children.iter().map(|child| child.id).collect())
            .unwrap_or_default()
    }

    pub(crate) fn update_mounted_props<H: NativeHost>(
        &mut self,
        updates: &BTreeMap<HostNodeId, NativeProps>,
        host: &mut H,
    ) -> GuiResult<()> {
        let Some(root) = self.root.as_ref() else {
            return if updates.is_empty() {
                Ok(())
            } else {
                Err(GuiError::host(
                    "cannot update props without a mounted native tree",
                ))
            };
        };
        let mut previous = Vec::new();
        for (node, props) in updates {
            let mounted = find_mounted_node(root, *node).ok_or_else(|| {
                GuiError::host(format!("unknown mounted native node id {}", node.get()))
            })?;
            if native_widget_kind(mounted.role, &mounted.props)
                != native_widget_kind(mounted.role, props)
            {
                return Err(GuiError::host(format!(
                    "cannot update native widget shape in place for node {}",
                    node.get()
                )));
            }
            if mounted.props != *props {
                previous.push((*node, mounted.props.clone(), props.clone()));
            }
        }

        let mut applied = 0;
        for (node, _, props) in &previous {
            if let Err(error) = host.update(*node, props) {
                for (rollback_node, rollback_props, _) in previous[..applied].iter().rev() {
                    let _ = host.update(*rollback_node, rollback_props);
                }
                return Err(error);
            }
            applied += 1;
        }
        if let Some(root) = self.root.as_mut() {
            for (node, _, props) in previous {
                if let Some(mounted) = find_mounted_node_mut(root, node) {
                    mounted.props = props;
                }
            }
        }
        Ok(())
    }
}

fn rollback_host_frame<H: NativeHost, T>(host: &mut H, error: GuiError) -> GuiResult<T> {
    match host.rollback_frame() {
        Ok(()) => Err(error),
        Err(rollback_error) => Err(GuiError::host(format!(
            "{error}; frame rollback also failed: {rollback_error}"
        ))),
    }
}

fn normalize_native_element(element: &NativeElement) -> NativeElement {
    NativeElement {
        key: element.key.clone(),
        role: element.role,
        props: normalize_props_for_native_role(element.role, &element.props),
        children: element
            .children
            .iter()
            .map(normalize_native_element)
            .collect(),
    }
}

fn validate_native_tree(element: &NativeElement) -> GuiResult<()> {
    if element.key.as_str().is_empty() {
        return Err(GuiError::invalid_tree(
            "a3s-gui native elements need non-empty keys",
        ));
    }

    let mut sibling_keys = BTreeSet::new();
    for child in &element.children {
        validate_native_tree(child)?;
        let key = child.key.as_str();
        if !sibling_keys.insert(key) {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui native sibling elements need unique keys; duplicate key {key:?}"
            )));
        }
    }
    Ok(())
}

fn needs_replacement(old: &MountedNode, new: &NativeElement) -> bool {
    old.key != new.key
        || old.role != new.role
        || native_widget_kind(old.role, &old.props) != native_widget_kind(new.role, &new.props)
}

fn collect_mounted_node_ids(node: &MountedNode, ids: &mut BTreeSet<HostNodeId>) {
    ids.insert(node.id);
    for child in &node.children {
        collect_mounted_node_ids(child, ids);
    }
}

fn collect_mounted_node_props(node: &MountedNode, props: &mut Vec<(HostNodeId, NativeProps)>) {
    props.push((node.id, node.props.clone()));
    for child in &node.children {
        collect_mounted_node_props(child, props);
    }
}

fn find_mounted_node(node: &MountedNode, target: HostNodeId) -> Option<&MountedNode> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_mounted_node(child, target))
}

fn find_mounted_node_mut(node: &mut MountedNode, target: HostNodeId) -> Option<&mut MountedNode> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_mounted_node_mut(child, target))
}

fn collect_ancestor_ids(
    node: &MountedNode,
    target: HostNodeId,
    ancestors: &mut Vec<HostNodeId>,
) -> bool {
    if node.id == target {
        return true;
    }

    for child in &node.children {
        if collect_ancestor_ids(child, target, ancestors) {
            ancestors.push(node.id);
            return true;
        }
    }

    false
}

fn mount_node<H: NativeHost>(
    parent: Option<HostNodeId>,
    index: usize,
    element: &NativeElement,
    host: &mut H,
) -> GuiResult<MountedNode> {
    let id = host.create(element)?;
    let mut children = Vec::with_capacity(element.children.len());
    for (child_index, child) in element.children.iter().enumerate() {
        match mount_node(Some(id), child_index, child, host) {
            Ok(child) => children.push(child),
            Err(error) => {
                best_effort_unmount_node(
                    MountedNode {
                        id,
                        key: element.key.clone(),
                        role: element.role,
                        props: element.props.clone(),
                        children,
                    },
                    host,
                );
                return Err(error);
            }
        }
    }
    if let Some(parent) = parent {
        if let Err(error) = host.insert_child(parent, id, index) {
            best_effort_unmount_node(
                MountedNode {
                    id,
                    key: element.key.clone(),
                    role: element.role,
                    props: element.props.clone(),
                    children,
                },
                host,
            );
            return Err(error);
        }
    }
    Ok(MountedNode {
        id,
        key: element.key.clone(),
        role: element.role,
        props: element.props.clone(),
        children,
    })
}

fn reconcile_node<H: NativeHost>(
    parent: Option<HostNodeId>,
    index: usize,
    old: MountedNode,
    new: &NativeElement,
    host: &mut H,
    rollback: &mut ReconcileRollback,
) -> GuiResult<MountedNode> {
    if needs_replacement(&old, new) {
        let mounted = mount_node(parent, index, new, host)?;
        rollback.record_new_mount(mounted.clone());
        rollback.record_deferred_unmount(old);
        return Ok(mounted);
    }

    if old.props != new.props {
        let previous_props = old.props.clone();
        host.update(old.id, &new.props)?;
        rollback.record_update(old.id, previous_props);
    }

    let children = reconcile_children(old.id, old.children, &new.children, host, rollback)?;
    Ok(MountedNode {
        id: old.id,
        key: old.key,
        role: old.role,
        props: new.props.clone(),
        children,
    })
}

fn reconcile_children<H: NativeHost>(
    parent: HostNodeId,
    old_children: Vec<MountedNode>,
    new_children: &[NativeElement],
    host: &mut H,
    rollback: &mut ReconcileRollback,
) -> GuiResult<Vec<MountedNode>> {
    let mut mounted_children = Vec::with_capacity(new_children.len());
    let previous_child_order = old_children
        .iter()
        .map(|child| child.id)
        .collect::<Vec<_>>();
    let mut old_by_key: BTreeMap<ElementKey, (usize, MountedNode)> = old_children
        .into_iter()
        .enumerate()
        .map(|(index, child)| (child.key.clone(), (index, child)))
        .collect();

    for (index, new_child) in new_children.iter().enumerate() {
        match old_by_key.remove(&new_child.key) {
            Some((old_index, old_child)) => {
                let old_id = old_child.id;
                let mounted =
                    reconcile_node(Some(parent), index, old_child, new_child, host, rollback)?;
                if mounted.id == old_id && old_index != index {
                    rollback.record_child_order(parent, previous_child_order.clone());
                    host.insert_child(parent, mounted.id, index)?;
                }
                mounted_children.push(mounted);
            }
            None => match mount_node(Some(parent), index, new_child, host) {
                Ok(mounted) => {
                    rollback.record_new_mount(mounted.clone());
                    mounted_children.push(mounted);
                }
                Err(error) => return Err(error),
            },
        }
    }

    for (_, old_child) in old_by_key.into_values() {
        rollback.record_deferred_unmount(old_child);
    }

    Ok(mounted_children)
}

fn restore_child_order<H: NativeHost>(parent: HostNodeId, children: Vec<HostNodeId>, host: &mut H) {
    for (index, child) in children.into_iter().enumerate() {
        let _ = host.insert_child(parent, child, index);
    }
}

fn unmount_node<H: NativeHost>(node: MountedNode, host: &mut H) -> GuiResult<()> {
    host.remove(node.id)
}

fn best_effort_unmount_node<H: NativeHost>(node: MountedNode, host: &mut H) {
    let _ = host.remove(node.id);
}

#[cfg(test)]
mod tests;
