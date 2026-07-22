use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::error::{GuiError, GuiResult};
use crate::host::{HostNodeId, NativeHost, ProgrammaticFocusHost};
use crate::native::{NativeElement, NativeProps};

use super::adapters::{BlueprintHost, PlatformAdapter};
use super::types::NativeWidgetBlueprint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformPlannedNode {
    pub id: HostNodeId,
    pub blueprint: NativeWidgetBlueprint,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PlatformCommand {
    Create {
        id: HostNodeId,
        blueprint: NativeWidgetBlueprint,
    },
    Update {
        id: HostNodeId,
        blueprint: NativeWidgetBlueprint,
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
    RequestFocus {
        id: HostNodeId,
    },
}

#[derive(Debug)]
pub struct PlatformPlanningHost<A: PlatformAdapter> {
    adapter: A,
    next_id: u64,
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    commands: Vec<PlatformCommand>,
}

#[derive(Debug, Clone)]
pub(crate) struct PlatformPlanningCheckpoint {
    next_id: u64,
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    command_len: usize,
}

impl<A: PlatformAdapter> PlatformPlanningHost<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            next_id: 0,
            root: None,
            focused: None,
            nodes: BTreeMap::new(),
            commands: Vec::new(),
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
    }

    pub fn node(&self, id: HostNodeId) -> Option<&PlatformPlannedNode> {
        self.nodes.get(&id)
    }

    pub fn nodes(&self) -> &BTreeMap<HostNodeId, PlatformPlannedNode> {
        &self.nodes
    }

    pub fn commands(&self) -> &[PlatformCommand] {
        &self.commands
    }

    pub fn capabilities(&self) -> NativeCapabilities {
        self.adapter.capabilities()
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    pub(crate) fn checkpoint(&self) -> PlatformPlanningCheckpoint {
        PlatformPlanningCheckpoint {
            next_id: self.next_id,
            root: self.root,
            focused: self.focused,
            nodes: self.nodes.clone(),
            command_len: self.commands.len(),
        }
    }

    pub(crate) fn restore(&mut self, checkpoint: PlatformPlanningCheckpoint) {
        self.next_id = checkpoint.next_id;
        self.root = checkpoint.root;
        self.focused = checkpoint.focused;
        self.nodes = checkpoint.nodes;
        self.commands.truncate(checkpoint.command_len);
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
        let state = &node.blueprint.control_state;
        if !node.blueprint.config().visible
            || state.inert
            || node.blueprint.portable_style.makes_native_widget_inert()
            || state.accessibility_state.hidden == Some(true)
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
            role: node.blueprint.accessibility_role,
            label: node.blueprint.label.clone(),
            value: node.blueprint.value.clone(),
            relationships: state.accessibility_relationships.clone(),
            description: state.accessibility_description.clone(),
            structure: state.accessibility_structure.clone(),
            state: state.accessibility_state.clone(),
            disabled: state.disabled,
            required: state.required,
            invalid: state.invalid,
            read_only: state.read_only,
            multiple: state.multiple,
            focused: false,
            selected: state.selected,
            checked: state.checked,
            expanded: state.expanded,
            children,
        })
    }
}

impl<A: PlatformAdapter> CapabilityHost for PlatformPlanningHost<A> {
    fn native_capabilities(&self) -> NativeCapabilities {
        self.capabilities()
    }
}

impl<A: PlatformAdapter> AccessibilityTreeHost for PlatformPlanningHost<A> {
    fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        self.root.and_then(|root| self.accessibility_subtree(root))
    }
}

impl<A: PlatformAdapter> BlueprintHost for PlatformPlanningHost<A> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.node(id).map(|node| &node.blueprint)
    }
}

impl<A: PlatformAdapter> NativeHost for PlatformPlanningHost<A> {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let id = self.allocate_id();
        let blueprint = self.adapter.blueprint(element);
        self.nodes.insert(
            id,
            PlatformPlannedNode {
                id,
                blueprint: blueprint.clone(),
                children: Vec::new(),
            },
        );
        self.commands
            .push(PlatformCommand::Create { id, blueprint });
        Ok(id)
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        let node = self
            .nodes
            .get_mut(&id)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", id.get())))?;
        let element = NativeElement::new(format!("host-{}", id.get()), node.blueprint.role)
            .with_props(props.clone());
        let blueprint = self.adapter.blueprint(&element);
        node.blueprint = blueprint.clone();
        self.commands
            .push(PlatformCommand::Update { id, blueprint });
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
        self.commands.push(PlatformCommand::InsertChild {
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
        if self
            .focused
            .is_some_and(|focused| removed_ids.contains(&focused))
        {
            self.focused = None;
        }
        self.commands.push(PlatformCommand::Remove { id });
        Ok(())
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        self.root = Some(id);
        self.commands.push(PlatformCommand::SetRoot { id });
        Ok(())
    }

    fn programmatic_focus_host(&mut self) -> Option<&mut dyn ProgrammaticFocusHost> {
        Some(self)
    }
}

impl<A: PlatformAdapter> ProgrammaticFocusHost for PlatformPlanningHost<A> {
    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        self.focused = Some(id);
        self.commands.push(PlatformCommand::RequestFocus { id });
        Ok(())
    }
}
