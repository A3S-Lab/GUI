use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;
use crate::platform::{NativeControlState, PlatformCommand};

use super::traits::PlatformCommandExecutor;

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
    objects: BTreeMap<HostNodeId, RecordedNativeObject>,
    commands: Vec<PlatformCommand>,
}

impl RecordingBackend {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
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
}

impl PlatformCommandExecutor for RecordingBackend {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        match command {
            PlatformCommand::Create { id, blueprint } => {
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
                self.ensure_object(*child)?;
                let parent_object = self.objects.get_mut(parent).ok_or_else(|| {
                    GuiError::host(format!("backend parent object {} missing", parent.get()))
                })?;
                parent_object
                    .children
                    .retain(|existing| *existing != *child);
                let index = (*index).min(parent_object.children.len());
                parent_object.children.insert(index, *child);
            }
            PlatformCommand::Remove { id } => {
                self.ensure_object(*id)?;
                for object in self.objects.values_mut() {
                    object.children.retain(|child| *child != *id);
                }
                self.objects.remove(id);
                if self.root == Some(*id) {
                    self.root = None;
                }
            }
            PlatformCommand::SetRoot { id } => {
                self.ensure_object(*id)?;
                self.root = Some(*id);
            }
        }
        self.commands.push(command.clone());
        Ok(())
    }
}
