use crate::accessibility::AccessibilityAnnouncement;
use crate::error::GuiResult;
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{
    NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfigPatch, NativeWidgetSetter,
    PlatformCommand,
};
use crate::selection::{CollectionKey, CollectionLayoutSnapshot};

#[derive(Debug, Clone, PartialEq)]
pub struct PlatformCommandBatch {
    pub id: u64,
    pub commands: Vec<PlatformCommand>,
}

impl PlatformCommandBatch {
    pub fn new(id: u64, commands: Vec<PlatformCommand>) -> Self {
        Self { id, commands }
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformBatchAck {
    pub batch_id: u64,
    pub prepared_commands: usize,
    pub applied_commands: usize,
}

impl PlatformBatchAck {
    pub fn committed(batch: &PlatformCommandBatch) -> Self {
        Self {
            batch_id: batch.id,
            prepared_commands: batch.commands.len(),
            applied_commands: batch.commands.len(),
        }
    }
}

#[derive(Debug)]
pub struct PlatformBatchFailure {
    pub batch_id: u64,
    pub applied_commands: usize,
    pub failed_command: usize,
    /// A command may mutate an OS object before returning an error. Commit
    /// failures are therefore conservatively treated as potentially partial.
    pub native_state_may_be_partial: bool,
    pub error: crate::error::GuiError,
}

/// Executes native command batches and owns the native resources they create.
///
/// Implementations must release all owned OS windows, widgets, registrations,
/// and callbacks from `Drop`. Degraded recovery drops the old executor before
/// preparing its replacement so two native surfaces never coexist.
pub trait PlatformCommandExecutor {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()>;

    fn measure_collection_layout(
        &mut self,
        _collection: HostNodeId,
        _items: &[(HostNodeId, CollectionKey)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        Ok(None)
    }

    /// Validate an entire frame before mutating native state.
    fn prepare_batch(&mut self, _batch: &PlatformCommandBatch) -> GuiResult<()> {
        Ok(())
    }

    /// Commit a prepared frame and return an explicit acknowledgement.
    fn commit_batch(
        &mut self,
        batch: &PlatformCommandBatch,
    ) -> Result<PlatformBatchAck, PlatformBatchFailure> {
        for (index, command) in batch.commands.iter().enumerate() {
            if let Err(error) = self.execute(command) {
                return Err(PlatformBatchFailure {
                    batch_id: batch.id,
                    applied_commands: index,
                    failed_command: index,
                    native_state_may_be_partial: true,
                    error,
                });
            }
        }
        Ok(PlatformBatchAck::committed(batch))
    }
}

pub trait NativeEventSource {
    fn take_native_events(&mut self) -> Vec<NativeEvent>;
}

pub trait NativeEventHost {
    fn take_native_events(&mut self) -> Vec<NativeEvent>;
}

pub trait NativeWidgetDriver {
    fn backend(&self) -> NativeBackendKind;
    fn create_widget(&mut self, id: HostNodeId, blueprint: &NativeWidgetBlueprint)
        -> GuiResult<()>;
    fn update_widget(&mut self, id: HostNodeId, blueprint: &NativeWidgetBlueprint)
        -> GuiResult<()>;
    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()>;
    fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()>;
    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()>;
    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()>;
    fn announce_accessibility(
        &mut self,
        _announcement: &AccessibilityAnnouncement,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native driver does not support accessibility announcements",
            self.backend()
        )))
    }
    fn position_overlay(
        &mut self,
        _overlay: HostNodeId,
        _anchor: HostNodeId,
        _request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native driver does not support anchored overlay positioning",
            self.backend()
        )))
    }

    fn measure_collection_layout(
        &mut self,
        _collection: HostNodeId,
        _items: &[(HostNodeId, CollectionKey)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        Ok(None)
    }
}

pub trait NativeHandleAdapter {
    type Handle: Clone;

    fn backend(&self) -> NativeBackendKind;
    fn create_handle(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle>;
    fn update_handle(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()>;
    fn update_handle_config(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
        _patch: &NativeWidgetConfigPatch,
    ) -> GuiResult<()> {
        self.update_handle(id, handle, blueprint)
    }
    fn insert_child_handle(
        &mut self,
        parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()>;
    fn remove_child_handle(
        &mut self,
        _parent: HostNodeId,
        _parent_handle: &Self::Handle,
        _child: HostNodeId,
        _child_handle: &Self::Handle,
    ) -> GuiResult<()> {
        Ok(())
    }
    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()>;
    fn set_root_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()>;
    fn request_focus_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()>;
    fn announce_accessibility_handle(
        &mut self,
        _announcement: &AccessibilityAnnouncement,
        _handle: &Self::Handle,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native handle adapter does not support accessibility announcements",
            self.backend()
        )))
    }
    fn position_overlay_handle(
        &mut self,
        _overlay: HostNodeId,
        _overlay_handle: &Self::Handle,
        _anchor: HostNodeId,
        _anchor_handle: &Self::Handle,
        _request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native handle adapter does not support anchored overlay positioning",
            self.backend()
        )))
    }
    fn measure_collection_layout_handles(
        &mut self,
        _collection: HostNodeId,
        _collection_handle: &Self::Handle,
        _items: &[(HostNodeId, CollectionKey, Self::Handle)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        Ok(None)
    }
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        Vec::new()
    }
}

pub trait NativeWidgetSurface {
    type Handle: Clone;

    fn backend(&self) -> NativeBackendKind;
    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle>;
    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()>;
    fn insert_native_child(
        &mut self,
        parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()>;
    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()>;
    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()>;
    fn request_native_focus(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()>;
    fn announce_native_accessibility(
        &mut self,
        _announcement: &AccessibilityAnnouncement,
        _handle: &Self::Handle,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native surface does not support accessibility announcements",
            self.backend()
        )))
    }
    fn position_native_overlay(
        &mut self,
        _overlay: HostNodeId,
        _overlay_handle: &Self::Handle,
        _anchor: HostNodeId,
        _anchor_handle: &Self::Handle,
        _request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        Err(crate::error::GuiError::host(format!(
            "{:?} native surface does not support anchored overlay positioning",
            self.backend()
        )))
    }
    fn measure_native_collection_layout(
        &mut self,
        _collection: HostNodeId,
        _collection_handle: &Self::Handle,
        _items: &[(HostNodeId, CollectionKey, Self::Handle)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        Ok(None)
    }
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        Vec::new()
    }
}
