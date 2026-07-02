use std::collections::{BTreeMap, BTreeSet};

use crate::accessibility::{accessibility_role, AccessibilityNode, AccessibilityTreeHost};
use crate::error::{GuiError, GuiResult};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::style::PortableStyle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HostNodeId(u64);

impl HostNodeId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

pub trait NativeHost {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId>;
    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()>;
    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()>;
    /// Remove a host node and its complete descendant subtree.
    fn remove(&mut self, id: HostNodeId) -> GuiResult<()>;
    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HostOperation {
    Create {
        id: HostNodeId,
        role: NativeRole,
        label: Option<String>,
    },
    Update {
        id: HostNodeId,
        label: Option<String>,
        value: Option<String>,
    },
    InsertChild {
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    },
    Remove {
        id: HostNodeId,
    },
    SetRoot {
        id: HostNodeId,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadlessNode {
    pub id: HostNodeId,
    pub role: NativeRole,
    pub props: NativeProps,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct HeadlessHost {
    next_id: u64,
    root: Option<HostNodeId>,
    nodes: BTreeMap<HostNodeId, HeadlessNode>,
    operations: Vec<HostOperation>,
}

impl HeadlessHost {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn node(&self, id: HostNodeId) -> Option<&HeadlessNode> {
        self.nodes.get(&id)
    }

    pub fn nodes(&self) -> &BTreeMap<HostNodeId, HeadlessNode> {
        &self.nodes
    }

    pub fn operations(&self) -> &[HostOperation] {
        &self.operations
    }

    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }

    fn allocate_id(&mut self) -> HostNodeId {
        self.next_id += 1;
        HostNodeId::new(self.next_id)
    }

    fn ensure_node(&self, id: HostNodeId) -> GuiResult<()> {
        if self.nodes.contains_key(&id) {
            Ok(())
        } else {
            Err(GuiError::host(format!("unknown host node id {}", id.get())))
        }
    }

    fn subtree_contains(&self, root: HostNodeId, target: HostNodeId) -> bool {
        let Some(root) = self.nodes.get(&root) else {
            return false;
        };
        let mut stack = root.children.clone();
        let mut visited = BTreeSet::new();

        while let Some(id) = stack.pop() {
            if id == target {
                return true;
            }
            if !visited.insert(id) {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                stack.extend(node.children.iter().copied());
            }
        }

        false
    }

    fn subtree_ids(&self, root: HostNodeId) -> BTreeSet<HostNodeId> {
        let mut ids = BTreeSet::new();
        let mut stack = vec![root];

        while let Some(id) = stack.pop() {
            if !ids.insert(id) {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                stack.extend(node.children.iter().copied());
            }
        }

        ids
    }

    fn accessibility_subtree(&self, id: HostNodeId) -> Option<AccessibilityNode> {
        let node = self.nodes.get(&id)?;
        if !is_accessibility_visible(&node.props)
            || is_accessibility_inert(&node.props)
            || node.props.accessibility_state.hidden == Some(true)
        {
            return None;
        }
        let children = node
            .children
            .iter()
            .filter_map(|child| self.accessibility_subtree(*child))
            .collect::<Vec<_>>();

        Some(AccessibilityNode {
            node: Some(id),
            role: accessibility_role(node.role),
            label: node.props.label.clone(),
            value: node.props.value.clone(),
            relationships: node.props.accessibility_relationships.clone(),
            description: node.props.accessibility_description.clone(),
            structure: node.props.accessibility_structure.clone(),
            state: node.props.accessibility_state.clone(),
            disabled: node.props.disabled,
            required: node.props.required,
            invalid: node.props.invalid,
            read_only: node.props.read_only,
            multiple: node.props.multiple,
            focused: false,
            selected: node.props.selected,
            checked: node.props.checked,
            expanded: node.props.expanded,
            children,
        })
    }
}

fn is_accessibility_visible(props: &NativeProps) -> bool {
    !props.hidden
        && PortableStyle::from_web(&props.web).renders_native_widget()
        && props.html_dialog.open.unwrap_or(true)
}

fn is_accessibility_inert(props: &NativeProps) -> bool {
    props.inert || PortableStyle::from_web(&props.web).makes_native_widget_inert()
}

impl AccessibilityTreeHost for HeadlessHost {
    fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        self.root.and_then(|root| self.accessibility_subtree(root))
    }
}

impl NativeHost for HeadlessHost {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let id = self.allocate_id();
        self.nodes.insert(
            id,
            HeadlessNode {
                id,
                role: element.role,
                props: element.props.clone(),
                children: Vec::new(),
            },
        );
        self.operations.push(HostOperation::Create {
            id,
            role: element.role,
            label: element.props.label.clone(),
        });
        Ok(id)
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        let node = self
            .nodes
            .get_mut(&id)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", id.get())))?;
        node.props = props.clone();
        self.operations.push(HostOperation::Update {
            id,
            label: props.label.clone(),
            value: props.value.clone(),
        });
        Ok(())
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.ensure_node(parent)?;
        self.ensure_node(child)?;
        if parent == child {
            return Err(GuiError::host(format!(
                "cannot insert host node {} into itself",
                child.get()
            )));
        }
        if self.subtree_contains(child, parent) {
            return Err(GuiError::host(format!(
                "inserting host node {} under {} would create a cycle",
                child.get(),
                parent.get()
            )));
        }

        for node in self.nodes.values_mut() {
            node.children.retain(|existing| *existing != child);
        }
        let parent_node = self
            .nodes
            .get_mut(&parent)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", parent.get())))?;
        let index = index.min(parent_node.children.len());
        parent_node.children.insert(index, child);
        self.operations.push(HostOperation::InsertChild {
            parent,
            child,
            index,
        });
        Ok(())
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        let removed_ids = self.subtree_ids(id);
        for node in self.nodes.values_mut() {
            node.children.retain(|child| !removed_ids.contains(child));
        }
        for removed_id in &removed_ids {
            self.nodes.remove(removed_id);
        }
        if self
            .root
            .map(|root| removed_ids.contains(&root))
            .unwrap_or(false)
        {
            self.root = None;
        }
        self.operations.push(HostOperation::Remove { id });
        Ok(())
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        self.root = Some(id);
        self.operations.push(HostOperation::SetRoot { id });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_host_reparents_child_without_duplicate_parent_links() {
        let mut host = HeadlessHost::default();
        let first = host
            .create(&NativeElement::new("first", NativeRole::View))
            .unwrap();
        let second = host
            .create(&NativeElement::new("second", NativeRole::View))
            .unwrap();
        let child = host
            .create(&NativeElement::new("child", NativeRole::Button))
            .unwrap();

        host.insert_child(first, child, 0).unwrap();
        host.insert_child(second, child, 0).unwrap();

        assert!(host.node(first).unwrap().children.is_empty());
        assert_eq!(host.node(second).unwrap().children, vec![child]);
    }

    #[test]
    fn headless_host_rejects_cyclic_child_insertions() {
        let mut host = HeadlessHost::default();
        let parent = host
            .create(&NativeElement::new("parent", NativeRole::View))
            .unwrap();
        let child = host
            .create(&NativeElement::new("child", NativeRole::Button))
            .unwrap();

        let operation_count = host.operations().len();
        let error = host.insert_child(parent, parent, 0).unwrap_err();

        assert!(error.to_string().contains("cannot insert host node"));
        assert_eq!(host.operations().len(), operation_count);
        assert!(host.node(parent).unwrap().children.is_empty());

        host.insert_child(parent, child, 0).unwrap();
        let operation_count = host.operations().len();
        let error = host.insert_child(child, parent, 0).unwrap_err();

        assert!(error.to_string().contains("would create a cycle"));
        assert_eq!(host.operations().len(), operation_count);
        assert_eq!(host.node(parent).unwrap().children, vec![child]);
        assert!(host.node(child).unwrap().children.is_empty());
    }

    #[test]
    fn headless_host_remove_deletes_entire_subtree() {
        let mut host = HeadlessHost::default();
        let root = host
            .create(&NativeElement::new("root", NativeRole::View))
            .unwrap();
        let child = host
            .create(&NativeElement::new("child", NativeRole::View))
            .unwrap();
        let grandchild = host
            .create(&NativeElement::new("grandchild", NativeRole::Button))
            .unwrap();
        host.insert_child(root, child, 0).unwrap();
        host.insert_child(child, grandchild, 0).unwrap();
        host.set_root(root).unwrap();
        let operation_count = host.operations().len();

        host.remove(root).unwrap();

        assert!(host.root().is_none());
        assert!(host.nodes().is_empty());
        assert_eq!(host.operations().len(), operation_count + 1);
        assert_eq!(
            host.operations().last(),
            Some(&HostOperation::Remove { id: root })
        );
    }
}
