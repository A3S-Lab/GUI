use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::platform::{NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig};

use super::traits::{NativeEventSource, NativeHandleAdapter, NativeWidgetDriver};

#[derive(Debug)]
pub struct HandleWidgetDriver<A: NativeHandleAdapter> {
    adapter: A,
    handles: BTreeMap<HostNodeId, A::Handle>,
    configs: BTreeMap<HostNodeId, NativeWidgetConfig>,
    root: Option<HostNodeId>,
    events: Vec<NativeEvent>,
}

impl<A: NativeHandleAdapter> HandleWidgetDriver<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            handles: BTreeMap::new(),
            configs: BTreeMap::new(),
            root: None,
            events: Vec::new(),
        }
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    pub fn adapter_mut(&mut self) -> &mut A {
        &mut self.adapter
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn handle(&self, id: HostNodeId) -> Option<&A::Handle> {
        self.handles.get(&id)
    }

    pub fn handles(&self) -> &BTreeMap<HostNodeId, A::Handle> {
        &self.handles
    }

    pub fn config(&self, id: HostNodeId) -> Option<&NativeWidgetConfig> {
        self.configs.get(&id)
    }

    pub fn configs(&self) -> &BTreeMap<HostNodeId, NativeWidgetConfig> {
        &self.configs
    }

    pub fn push_native_event(&mut self, event: NativeEvent) {
        self.events.push(event);
    }

    pub fn queued_native_events(&self) -> &[NativeEvent] {
        &self.events
    }

    fn cloned_handle(&self, id: HostNodeId) -> GuiResult<A::Handle> {
        self.handles
            .get(&id)
            .cloned()
            .ok_or_else(|| GuiError::host(format!("native handle {} does not exist", id.get())))
    }
}

impl<A: NativeHandleAdapter + Default> Default for HandleWidgetDriver<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}

impl<A: NativeHandleAdapter> NativeEventSource for HandleWidgetDriver<A> {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        let mut events = std::mem::take(&mut self.events);
        events.extend(self.adapter.take_native_events());
        events
    }
}

impl<A: NativeHandleAdapter> NativeWidgetDriver for HandleWidgetDriver<A> {
    fn backend(&self) -> NativeBackendKind {
        self.adapter.backend()
    }

    fn create_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let handle = self.adapter.create_handle(id, blueprint)?;
        self.handles.insert(id, handle);
        self.configs.insert(id, blueprint.config());
        Ok(())
    }

    fn update_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let handle = self.cloned_handle(id)?;
        let before = self
            .configs
            .get(&id)
            .cloned()
            .ok_or_else(|| GuiError::host(format!("native config {} missing", id.get())))?;
        let after = blueprint.config();
        let patch = before.diff(&after);
        self.adapter
            .update_handle_config(id, &handle, blueprint, &patch)?;
        self.configs.insert(id, after);
        Ok(())
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        let parent_handle = self.cloned_handle(parent)?;
        let child_handle = self.cloned_handle(child)?;
        self.adapter
            .insert_child_handle(parent, &parent_handle, child, &child_handle, index)
    }

    fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        let handle = self
            .handles
            .remove(&id)
            .ok_or_else(|| GuiError::host(format!("native handle {} missing", id.get())))?;
        self.adapter.remove_handle(id, handle)?;
        self.configs.remove(&id);
        if self.root == Some(id) {
            self.root = None;
        }
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        let handle = self.cloned_handle(id)?;
        self.adapter.set_root_handle(id, &handle)?;
        self.root = Some(id);
        Ok(())
    }
}
