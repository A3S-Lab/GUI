use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityAnnouncement, AccessibilityNode, AccessibilityTreeHost};
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::error::{GuiError, GuiResult};
use crate::host::{
    AccessibilityAnnouncementHost, HostFrameAck, HostNodeId, NativeHost, OverlayPositionHost,
    ProgrammaticFocusHost,
};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::overlay_position::OverlayPositionRequest;
use crate::style::PortableStyle;

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
    PositionOverlay {
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    },
    AccessibilityAnnouncement {
        announcement: AccessibilityAnnouncement,
    },
}

impl PlatformCommand {
    pub fn redacted_for_diagnostics(&self) -> Self {
        match self {
            Self::Create { id, blueprint } => Self::Create {
                id: *id,
                blueprint: blueprint.redacted_for_diagnostics(),
            },
            Self::Update { id, blueprint } => Self::Update {
                id: *id,
                blueprint: blueprint.redacted_for_diagnostics(),
            },
            Self::InsertChild {
                parent,
                child,
                index,
            } => Self::InsertChild {
                parent: *parent,
                child: *child,
                index: *index,
            },
            Self::Remove { id } => Self::Remove { id: *id },
            Self::SetRoot { id } => Self::SetRoot { id: *id },
            Self::RequestFocus { id } => Self::RequestFocus { id: *id },
            Self::PositionOverlay {
                overlay,
                anchor,
                request,
            } => Self::PositionOverlay {
                overlay: *overlay,
                anchor: *anchor,
                request: *request,
            },
            Self::AccessibilityAnnouncement { announcement } => Self::AccessibilityAnnouncement {
                announcement: announcement.clone(),
            },
        }
    }
}

#[derive(Debug)]
pub struct PlatformPlanningHost<A: PlatformAdapter> {
    adapter: A,
    next_id: u64,
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    commands: Vec<PlatformCommand>,
    active_frame: Option<PlatformPlanningCheckpoint>,
}

#[derive(Debug, Clone)]
pub(crate) struct PlatformPlanningCheckpoint {
    next_id: u64,
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    commands: Vec<PlatformCommand>,
}

impl<A: PlatformAdapter> PlatformPlanningHost<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            next_id: 0,
            root: None,
            focused: None,
            overlay_positions: BTreeMap::new(),
            nodes: BTreeMap::new(),
            commands: Vec::new(),
            active_frame: None,
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
    }

    pub fn overlay_positions(&self) -> &BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)> {
        &self.overlay_positions
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

    pub fn project_portable_style(
        &mut self,
        id: HostNodeId,
        style: &PortableStyle,
    ) -> GuiResult<()> {
        let blueprint = {
            let node = self
                .nodes
                .get_mut(&id)
                .ok_or_else(|| GuiError::host(format!("unknown host node id {}", id.get())))?;
            if node.blueprint.portable_style == *style {
                return Ok(());
            }
            node.blueprint.portable_style = style.clone();
            node.blueprint.clone()
        };
        self.commands
            .push(PlatformCommand::Update { id, blueprint });
        Ok(())
    }

    /// Takes all pending commands produced since the previous drain.
    pub fn take_commands(&mut self) -> Vec<PlatformCommand> {
        std::mem::take(&mut self.commands)
    }

    /// Acknowledge a successfully committed command prefix.
    ///
    /// Commands remain queued throughout prepare/commit and are removed only
    /// after a matching native acknowledgement.
    pub(crate) fn acknowledge_commands(&mut self, commands: &[PlatformCommand]) -> GuiResult<()> {
        if self.commands.len() < commands.len() || self.commands[..commands.len()] != *commands {
            return Err(GuiError::host(
                "native command acknowledgement does not match the planning queue",
            ));
        }
        self.commands.drain(..commands.len());
        Ok(())
    }

    /// Build a complete replay for a fresh native executor.
    pub(crate) fn replay_commands(&self) -> Vec<PlatformCommand> {
        let mut commands = Vec::new();
        for (id, node) in &self.nodes {
            commands.push(PlatformCommand::Create {
                id: *id,
                blueprint: node.blueprint.clone(),
            });
        }
        for (parent, node) in &self.nodes {
            for (index, child) in node.children.iter().enumerate() {
                commands.push(PlatformCommand::InsertChild {
                    parent: *parent,
                    child: *child,
                    index,
                });
            }
        }
        if let Some(id) = self.root {
            commands.push(PlatformCommand::SetRoot { id });
        }
        for (overlay, (anchor, request)) in &self.overlay_positions {
            commands.push(PlatformCommand::PositionOverlay {
                overlay: *overlay,
                anchor: *anchor,
                request: *request,
            });
        }
        commands
    }

    pub fn capabilities(&self) -> NativeCapabilities {
        self.adapter.capabilities()
    }

    pub fn clear_commands(&mut self) {
        self.take_commands();
    }

    pub(crate) fn checkpoint(&self) -> PlatformPlanningCheckpoint {
        PlatformPlanningCheckpoint {
            next_id: self.next_id,
            root: self.root,
            focused: self.focused,
            overlay_positions: self.overlay_positions.clone(),
            nodes: self.nodes.clone(),
            commands: self.commands.clone(),
        }
    }

    pub(crate) fn restore(&mut self, checkpoint: PlatformPlanningCheckpoint) {
        self.next_id = checkpoint.next_id;
        self.root = checkpoint.root;
        self.focused = checkpoint.focused;
        self.overlay_positions = checkpoint.overlay_positions;
        self.nodes = checkpoint.nodes;
        self.commands = checkpoint.commands;
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
        let value_sensitivity = node.blueprint.effective_value_sensitivity();
        let mut description = state.accessibility_description.clone();
        if value_sensitivity.is_sensitive() {
            description.value_text = None;
        }

        Some(AccessibilityNode {
            node: Some(id),
            role: node.blueprint.accessibility_role,
            label: node
                .blueprint
                .accessibility_label
                .clone()
                .or_else(|| node.blueprint.label.clone()),
            value: value_sensitivity
                .redact(node.blueprint.value.as_deref())
                .map(ToOwned::to_owned),
            value_sensitivity,
            relationships: state.accessibility_relationships.clone(),
            description,
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
    fn begin_frame(&mut self) -> GuiResult<()> {
        if self.active_frame.is_some() {
            return Err(GuiError::host(
                "a platform planning frame is already active",
            ));
        }
        self.active_frame = Some(self.checkpoint());
        Ok(())
    }

    fn commit_frame(&mut self) -> GuiResult<HostFrameAck> {
        let checkpoint = self
            .active_frame
            .take()
            .ok_or_else(|| GuiError::host("no platform planning frame is active"))?;
        Ok(HostFrameAck {
            batch_id: None,
            applied_operations: self
                .commands
                .len()
                .saturating_sub(checkpoint.commands.len()),
        })
    }

    fn rollback_frame(&mut self) -> GuiResult<()> {
        if let Some(checkpoint) = self.active_frame.take() {
            self.restore(checkpoint);
        }
        Ok(())
    }

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
        self.overlay_positions.retain(|overlay, (anchor, _)| {
            !removed_ids.contains(overlay) && !removed_ids.contains(anchor)
        });
        self.commands.push(PlatformCommand::Remove { id });
        Ok(())
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        self.root = Some(id);
        self.commands.push(PlatformCommand::SetRoot { id });
        Ok(())
    }

    fn update_portable_style(&mut self, id: HostNodeId, style: &PortableStyle) -> GuiResult<()> {
        self.project_portable_style(id, style)
    }

    fn programmatic_focus_host(&mut self) -> Option<&mut dyn ProgrammaticFocusHost> {
        Some(self)
    }

    fn overlay_position_host(&mut self) -> Option<&mut dyn OverlayPositionHost> {
        Some(self)
    }

    fn accessibility_announcement_host(
        &mut self,
    ) -> Option<&mut dyn AccessibilityAnnouncementHost> {
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

impl<A: PlatformAdapter> OverlayPositionHost for PlatformPlanningHost<A> {
    fn position_overlay(
        &mut self,
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        self.ensure_node(overlay)?;
        self.ensure_node(anchor)?;
        if overlay == anchor {
            return Err(GuiError::host(format!(
                "overlay {} cannot anchor to itself",
                overlay.get()
            )));
        }
        if self.nodes.get(&overlay).map(|node| node.blueprint.role) != Some(NativeRole::Popover) {
            return Err(GuiError::host(format!(
                "host node {} is not an overlay",
                overlay.get()
            )));
        }
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        let position = (anchor, request);
        let changed = self.overlay_positions.get(&overlay) != Some(&position);
        self.overlay_positions.insert(overlay, position);
        if changed || request.options.should_update_position {
            self.commands.push(PlatformCommand::PositionOverlay {
                overlay,
                anchor,
                request,
            });
        }
        Ok(())
    }
}

impl<A: PlatformAdapter> AccessibilityAnnouncementHost for PlatformPlanningHost<A> {
    fn announce(&mut self, announcement: AccessibilityAnnouncement) -> GuiResult<()> {
        self.ensure_node(announcement.node)?;
        if announcement.message.trim().is_empty() {
            return Ok(());
        }
        self.commands
            .push(PlatformCommand::AccessibilityAnnouncement { announcement });
        Ok(())
    }
}
