use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::platform::{NativeControlState, PlatformCommand};

use super::traits::{NativeEventSource, PlatformCommandExecutor};

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedNativeObject {
    pub id: HostNodeId,
    pub widget_class: String,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct RecordingBackend {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    objects: BTreeMap<HostNodeId, RecordedNativeObject>,
    commands: Vec<PlatformCommand>,
    events: Vec<NativeEvent>,
}

impl RecordingBackend {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
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
                self.objects.insert(
                    *id,
                    RecordedNativeObject {
                        id: *id,
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
                let object = self.objects.get_mut(id).ok_or_else(|| {
                    GuiError::host(format!("backend object {} missing", id.get()))
                })?;
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
            }
            PlatformCommand::SetRoot { id } => {
                self.ensure_object(*id)?;
                self.root = Some(*id);
            }
            PlatformCommand::RequestFocus { id } => {
                self.ensure_object(*id)?;
                self.focused = Some(*id);
            }
        }
        self.commands.push(command.clone());
        Ok(())
    }
}

impl NativeEventSource for RecordingBackend {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events)
    }
}
