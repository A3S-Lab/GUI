use std::collections::BTreeMap;

use crate::accessibility::{accessibility_role, AccessibilityNode, AccessibilityTreeHost};
use crate::error::{GuiError, GuiResult};
use crate::native::{NativeElement, NativeProps, NativeRole};
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

    fn accessibility_subtree(&self, id: HostNodeId) -> Option<AccessibilityNode> {
        let node = self.nodes.get(&id)?;
        let children = node
            .children
            .iter()
            .map(|child| self.accessibility_subtree(*child))
            .collect::<Option<Vec<_>>>()?;

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
            focused: false,
            selected: node.props.selected,
            checked: node.props.checked,
            expanded: node.props.expanded,
            children,
        })
    }
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
        self.ensure_node(child)?;
        let parent_node = self
            .nodes
            .get_mut(&parent)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", parent.get())))?;
        parent_node.children.retain(|existing| *existing != child);
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
        for node in self.nodes.values_mut() {
            node.children.retain(|child| *child != id);
        }
        self.nodes.remove(&id);
        if self.root == Some(id) {
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
