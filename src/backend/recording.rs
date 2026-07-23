use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{NativeControlState, NativeWidgetKind, PlatformCommand};

use super::traits::{NativeEventSource, PlatformCommandExecutor};

pub const DEFAULT_RECORDING_COMMAND_HISTORY_LIMIT: usize = 256;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedNativeObject {
    pub id: HostNodeId,
    pub widget_kind: NativeWidgetKind,
    /// Diagnostic/legacy class name; execution is driven by `widget_kind`.
    pub widget_class: String,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug)]
pub struct RecordingBackend {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
    objects: BTreeMap<HostNodeId, RecordedNativeObject>,
    commands: Vec<PlatformCommand>,
    command_history_limit: usize,
    events: Vec<NativeEvent>,
}

impl Default for RecordingBackend {
    fn default() -> Self {
        Self::with_command_history_limit(DEFAULT_RECORDING_COMMAND_HISTORY_LIMIT)
    }
}

impl RecordingBackend {
    pub fn with_command_history_limit(command_history_limit: usize) -> Self {
        Self {
            root: None,
            focused: None,
            overlay_positions: BTreeMap::new(),
            objects: BTreeMap::new(),
            commands: Vec::new(),
            command_history_limit,
            events: Vec::new(),
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

    pub fn object(&self, id: HostNodeId) -> Option<&RecordedNativeObject> {
        self.objects.get(&id)
    }

    pub fn objects(&self) -> &BTreeMap<HostNodeId, RecordedNativeObject> {
        &self.objects
    }

    pub fn commands(&self) -> &[PlatformCommand] {
        &self.commands
    }

    pub fn command_history_limit(&self) -> usize {
        self.command_history_limit
    }

    pub fn take_commands(&mut self) -> Vec<PlatformCommand> {
        std::mem::take(&mut self.commands)
    }

    pub fn push_native_event(&mut self, event: NativeEvent) {
        self.events.push(event);
    }

    pub fn extend_native_events(&mut self, events: impl IntoIterator<Item = NativeEvent>) {
        self.events.extend(events);
    }

    fn ensure_object(&self, id: HostNodeId) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "backend object {} does not exist",
                id.get()
            )))
        }
    }

    fn subtree_contains(&self, root: HostNodeId, target: HostNodeId) -> bool {
        let Some(root) = self.objects.get(&root) else {
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
            if let Some(object) = self.objects.get(&id) {
                stack.extend(object.children.iter().copied());
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
            if let Some(object) = self.objects.get(&id) {
                stack.extend(object.children.iter().copied());
            }
        }

        ids
    }
}

impl PlatformCommandExecutor for RecordingBackend {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        match command {
            PlatformCommand::Create { id, blueprint } => {
                if self.objects.contains_key(id) {
                    return Err(GuiError::host(format!(
                        "backend object {} already exists",
                        id.get()
                    )));
                }
                let blueprint = blueprint.redacted_for_diagnostics();
                self.objects.insert(
                    *id,
                    RecordedNativeObject {
                        id: *id,
                        widget_kind: blueprint.widget_kind,
                        widget_class: blueprint.widget_class.clone(),
                        label: blueprint.label.clone(),
                        value: blueprint.value.clone(),
                        action: blueprint.action.clone(),
                        control_state: blueprint.control_state.clone(),
                        children: Vec::new(),
                    },
                );
            }
            PlatformCommand::Update { id, blueprint } => {
                let blueprint = blueprint.redacted_for_diagnostics();
                let object = self.objects.get_mut(id).ok_or_else(|| {
                    GuiError::host(format!("backend object {} missing", id.get()))
                })?;
                object.widget_kind = blueprint.widget_kind;
                object.widget_class = blueprint.widget_class.clone();
                object.label = blueprint.label.clone();
                object.value = blueprint.value.clone();
                object.action = blueprint.action.clone();
                object.control_state = blueprint.control_state.clone();
            }
            PlatformCommand::InsertChild {
                parent,
                child,
                index,
            } => {
                self.ensure_object(*parent)?;
                self.ensure_object(*child)?;
                if parent == child {
                    return Err(GuiError::host(format!(
                        "cannot insert backend object {} into itself",
                        child.get()
                    )));
                }
                if self.subtree_contains(*child, *parent) {
                    return Err(GuiError::host(format!(
                        "inserting backend object {} under {} would create a cycle",
                        child.get(),
                        parent.get()
                    )));
                }

                for object in self.objects.values_mut() {
                    object.children.retain(|existing| *existing != *child);
                }
                let parent_object = self.objects.get_mut(parent).ok_or_else(|| {
                    GuiError::host(format!("backend parent object {} missing", parent.get()))
                })?;
                let index = (*index).min(parent_object.children.len());
                parent_object.children.insert(index, *child);
            }
            PlatformCommand::Remove { id } => {
                self.ensure_object(*id)?;
                let removed_ids = self.subtree_ids(*id);
                for object in self.objects.values_mut() {
                    object.children.retain(|child| !removed_ids.contains(child));
                }
                for removed_id in &removed_ids {
                    self.objects.remove(removed_id);
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
            }
            PlatformCommand::SetRoot { id } => {
                self.ensure_object(*id)?;
                self.root = Some(*id);
            }
            PlatformCommand::RequestFocus { id } => {
                self.ensure_object(*id)?;
                self.focused = Some(*id);
            }
            PlatformCommand::PositionOverlay {
                overlay,
                anchor,
                request,
            } => {
                self.ensure_object(*overlay)?;
                self.ensure_object(*anchor)?;
                if overlay == anchor {
                    return Err(GuiError::host(format!(
                        "overlay {} cannot anchor to itself",
                        overlay.get()
                    )));
                }
                if self.objects.get(overlay).map(|object| object.widget_kind)
                    != Some(NativeWidgetKind::Popover)
                {
                    return Err(GuiError::host(format!(
                        "backend object {} is not an overlay",
                        overlay.get()
                    )));
                }
                let request = OverlayPositionRequest::new(request.options, request.direction)?;
                self.overlay_positions.insert(*overlay, (*anchor, request));
            }
        }
        if self.command_history_limit > 0 {
            if self.commands.len() == self.command_history_limit {
                self.commands.remove(0);
            }
            self.commands.push(command.redacted_for_diagnostics());
        }
        Ok(())
    }
}

impl NativeEventSource for RecordingBackend {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events)
    }
}
