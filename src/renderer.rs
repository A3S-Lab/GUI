use std::collections::{BTreeMap, BTreeSet};

use crate::error::GuiResult;
use crate::host::{HostNodeId, NativeHost};
use crate::native::{ElementKey, NativeElement, NativeProps, NativeRole};

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

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render<H: NativeHost>(
        &mut self,
        element: &NativeElement,
        host: &mut H,
    ) -> GuiResult<HostNodeId> {
        let mounted = match self.root.take() {
            Some(old) => reconcile_node(None, 0, old, element, host)?,
            None => mount_node(None, 0, element, host)?,
        };
        host.set_root(mounted.id)?;
        let root_id = mounted.id;
        self.root = Some(mounted);
        Ok(root_id)
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
        children.push(mount_node(Some(id), child_index, child, host)?);
    }
    if let Some(parent) = parent {
        host.insert_child(parent, id, index)?;
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
) -> GuiResult<MountedNode> {
    if old.key != new.key || old.role != new.role {
        unmount_node(old, host)?;
        return mount_node(parent, index, new, host);
    }

    if old.props != new.props {
        host.update(old.id, &new.props)?;
    }

    let children = reconcile_children(old.id, old.children, &new.children, host)?;
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
) -> GuiResult<Vec<MountedNode>> {
    let mut mounted_children = Vec::with_capacity(new_children.len());
    let mut old_by_key: BTreeMap<ElementKey, (usize, MountedNode)> = old_children
        .into_iter()
        .enumerate()
        .map(|(index, child)| (child.key.clone(), (index, child)))
        .collect();

    for (index, new_child) in new_children.iter().enumerate() {
        match old_by_key.remove(&new_child.key) {
            Some((old_index, old_child)) => {
                let old_id = old_child.id;
                let mounted = reconcile_node(Some(parent), index, old_child, new_child, host)?;
                if mounted.id == old_id && old_index != index {
                    host.insert_child(parent, mounted.id, index)?;
                }
                mounted_children.push(mounted);
            }
            None => mounted_children.push(mount_node(Some(parent), index, new_child, host)?),
        }
    }

    for (_, old_child) in old_by_key.into_values() {
        unmount_node(old_child, host)?;
    }

    Ok(mounted_children)
}

fn unmount_node<H: NativeHost>(node: MountedNode, host: &mut H) -> GuiResult<()> {
    for child in node.children {
        unmount_node(child, host)?;
    }
    host.remove(node.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{HeadlessHost, HostOperation};
    use crate::native::{NativeElement, NativeProps, NativeRole};

    #[test]
    fn keyed_children_are_reordered_without_remounting() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            );
        let second = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            )
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.clear_operations();
        let second_root_id = renderer.render(&second, &mut host).unwrap();

        assert_eq!(root_id, second_root_id);
        assert!(!host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { .. } | HostOperation::Remove { .. }
        )));
        assert!(host
            .operations()
            .iter()
            .any(|operation| matches!(operation, HostOperation::InsertChild { parent, index, .. } if *parent == root_id && *index == 0)));

        let labels: Vec<_> = host
            .node(root_id)
            .unwrap()
            .children
            .iter()
            .map(|id| host.node(*id).unwrap().props.label.as_deref().unwrap())
            .collect();
        assert_eq!(labels, vec!["B", "A"]);
    }

    #[test]
    fn mounted_node_ids_follow_reconciled_tree() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button));
        let second = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let first_children = host.node(root_id).unwrap().children.clone();
        let removed = first_children[0];
        renderer.render(&second, &mut host).unwrap();
        let mounted = renderer.mounted_node_ids();

        assert!(mounted.contains(&root_id));
        assert!(!mounted.contains(&removed));
        assert_eq!(mounted.len(), 3);
    }

    #[test]
    fn mounted_node_props_follow_tree_order() {
        let tree = NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().label("Root"))
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        renderer.render(&tree, &mut host).unwrap();

        let labels = renderer
            .mounted_node_props()
            .into_iter()
            .map(|(_, props)| props.label)
            .collect::<Vec<_>>();
        assert_eq!(
            labels,
            vec![
                Some("Root".to_string()),
                Some("A".to_string()),
                Some("B".to_string())
            ]
        );
    }

    #[test]
    fn ancestor_ids_return_nearest_parent_first() {
        let tree = NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("group", NativeRole::View)
                .child(NativeElement::new("save", NativeRole::Button)),
        );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&tree, &mut host).unwrap();
        let group_id = host.node(root_id).unwrap().children[0];
        let save_id = host.node(group_id).unwrap().children[0];

        assert_eq!(renderer.ancestor_ids(save_id), vec![group_id, root_id]);
        assert!(renderer.ancestor_ids(root_id).is_empty());
    }
}
