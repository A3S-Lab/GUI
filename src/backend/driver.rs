use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::accessibility::AccessibilityAnnouncement;
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig};
use crate::selection::{CollectionKey, CollectionLayoutSnapshot};

use super::traits::{NativeEventSource, NativeHandleAdapter, NativeWidgetDriver};

#[derive(Debug)]
pub struct HandleWidgetDriver<A: NativeHandleAdapter> {
    adapter: A,
    handles: BTreeMap<HostNodeId, A::Handle>,
    configs: BTreeMap<HostNodeId, NativeWidgetConfig>,
    children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    events: Vec<NativeEvent>,
}

impl<A: NativeHandleAdapter> HandleWidgetDriver<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            handles: BTreeMap::new(),
            configs: BTreeMap::new(),
            children: BTreeMap::new(),
            root: None,
            focused: None,
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

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
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

    pub fn children(&self, id: HostNodeId) -> Option<&[HostNodeId]> {
        self.children.get(&id).map(Vec::as_slice)
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

    fn ensure_handle_absent(&self, id: HostNodeId) -> GuiResult<()> {
        if self.handles.contains_key(&id) {
            Err(GuiError::host(format!(
                "native handle {} already exists",
                id.get()
            )))
        } else {
            Ok(())
        }
    }

    fn subtree_contains(&self, root: HostNodeId, target: HostNodeId) -> bool {
        let mut stack = self.children.get(&root).cloned().unwrap_or_default();
        let mut visited = BTreeSet::new();

        while let Some(id) = stack.pop() {
            if id == target {
                return true;
            }
            if !visited.insert(id) {
                continue;
            }
            if let Some(children) = self.children.get(&id) {
                stack.extend(children.iter().copied());
            }
        }

        false
    }

    fn subtree_postorder(&self, root: HostNodeId) -> Vec<HostNodeId> {
        fn visit(
            driver_children: &BTreeMap<HostNodeId, Vec<HostNodeId>>,
            id: HostNodeId,
            visited: &mut BTreeSet<HostNodeId>,
            ordered: &mut Vec<HostNodeId>,
        ) {
            if !visited.insert(id) {
                return;
            }
            if let Some(children) = driver_children.get(&id) {
                for child in children {
                    visit(driver_children, *child, visited, ordered);
                }
            }
            ordered.push(id);
        }

        let mut visited = BTreeSet::new();
        let mut ordered = Vec::new();
        visit(&self.children, root, &mut visited, &mut ordered);
        ordered
    }

    fn forget_removed_handles(&mut self, removed_ids: &BTreeSet<HostNodeId>) {
        if removed_ids.is_empty() {
            return;
        }
        for children in self.children.values_mut() {
            children.retain(|child| !removed_ids.contains(child));
        }
        for removed_id in removed_ids {
            self.handles.remove(removed_id);
            self.configs.remove(removed_id);
            self.children.remove(removed_id);
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

    fn forget_parent_link(&mut self, child: HostNodeId) {
        for children in self.children.values_mut() {
            children.retain(|existing| *existing != child);
        }
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
        self.ensure_handle_absent(id)?;
        let handle = self.adapter.create_handle(id, blueprint)?;
        self.handles.insert(id, handle);
        self.configs.insert(id, blueprint.config());
        self.children.insert(id, Vec::new());
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
        if let Some(replacement) = patch.replacement() {
            return Err(GuiError::host(format!(
                "native widget {} changed identity ({replacement:?}); recreate it instead of applying setters",
                id.get()
            )));
        }
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
        if parent == child {
            return Err(GuiError::host(format!(
                "cannot insert native handle {} into itself",
                child.get()
            )));
        }
        if self.subtree_contains(child, parent) {
            return Err(GuiError::host(format!(
                "inserting native handle {} under {} would create a cycle",
                child.get(),
                parent.get()
            )));
        }

        let parent_handle = self.cloned_handle(parent)?;
        let child_handle = self.cloned_handle(child)?;
        let previous_parent = self
            .children
            .iter()
            .find_map(|(candidate, children)| children.contains(&child).then_some(*candidate));
        let detached_parent =
            if let Some(previous_parent) = previous_parent.filter(|previous| *previous != parent) {
                let previous_parent_handle = self.cloned_handle(previous_parent)?;
                self.adapter.remove_child_handle(
                    previous_parent,
                    &previous_parent_handle,
                    child,
                    &child_handle,
                )?;
                Some(previous_parent)
            } else {
                None
            };
        if let Err(error) =
            self.adapter
                .insert_child_handle(parent, &parent_handle, child, &child_handle, index)
        {
            if detached_parent.is_some() {
                self.forget_parent_link(child);
            }
            return Err(error);
        }
        self.forget_parent_link(child);
        let children = self.children.get_mut(&parent).ok_or_else(|| {
            GuiError::host(format!("native handle {} does not exist", parent.get()))
        })?;
        let index = index.min(children.len());
        children.insert(index, child);
        Ok(())
    }

    fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        let removed = self.subtree_postorder(id);
        let mut removed_ids = BTreeSet::new();
        for removed_id in &removed {
            let handle = match self.cloned_handle(*removed_id) {
                Ok(handle) => handle,
                Err(error) => {
                    self.forget_removed_handles(&removed_ids);
                    return Err(error);
                }
            };
            if let Err(error) = self.adapter.remove_handle(*removed_id, handle) {
                self.forget_removed_handles(&removed_ids);
                return Err(error);
            }
            removed_ids.insert(*removed_id);
        }
        self.forget_removed_handles(&removed_ids);
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        let handle = self.cloned_handle(id)?;
        self.adapter.set_root_handle(id, &handle)?;
        self.root = Some(id);
        Ok(())
    }

    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        let handle = self.cloned_handle(id)?;
        self.adapter.request_focus_handle(id, &handle)?;
        self.focused = Some(id);
        Ok(())
    }

    fn announce_accessibility(
        &mut self,
        announcement: &AccessibilityAnnouncement,
    ) -> GuiResult<()> {
        let handle = self.cloned_handle(announcement.node)?;
        self.adapter
            .announce_accessibility_handle(announcement, &handle)
    }

    fn position_overlay(
        &mut self,
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        let overlay_handle = self.cloned_handle(overlay)?;
        let anchor_handle = self.cloned_handle(anchor)?;
        self.adapter.position_overlay_handle(
            overlay,
            &overlay_handle,
            anchor,
            &anchor_handle,
            request,
        )
    }

    fn measure_collection_layout(
        &mut self,
        collection: HostNodeId,
        items: &[(HostNodeId, CollectionKey)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        let collection_handle = self.cloned_handle(collection)?;
        let items = items
            .iter()
            .map(|(id, key)| Ok((*id, key.clone(), self.cloned_handle(*id)?)))
            .collect::<GuiResult<Vec<_>>>()?;
        self.adapter
            .measure_collection_layout_handles(collection, &collection_handle, &items)
    }
}
