use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::error::{GuiError, GuiResult};
use crate::host::{HostNodeId, NativeHost};
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
}

#[derive(Debug)]
pub struct PlatformPlanningHost<A: PlatformAdapter> {
    adapter: A,
    next_id: u64,
    root: Option<HostNodeId>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    commands: Vec<PlatformCommand>,
}

impl<A: PlatformAdapter> PlatformPlanningHost<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            next_id: 0,
            root: None,
            nodes: BTreeMap::new(),
            commands: Vec::new(),
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
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

    pub fn clear_commands(&mut self) {
        self.commands.clear();
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
        let state = &node.blueprint.control_state;
        if state.hidden || state.inert {
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
        self.ensure_node(child)?;
        let parent_node = self
            .nodes
            .get_mut(&parent)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", parent.get())))?;
        parent_node.children.retain(|existing| *existing != child);
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
        for node in self.nodes.values_mut() {
            node.children.retain(|child| *child != id);
        }
        self.nodes.remove(&id);
        if self.root == Some(id) {
            self.root = None;
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
}
