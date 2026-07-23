use crate::accessibility::AccessibilityAnnouncement;
use crate::error::GuiResult;
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{
    NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfigPatch, NativeWidgetSetter,
};
use crate::selection::{CollectionKey, CollectionLayoutSnapshot};

use super::traits::{NativeHandleAdapter, NativeWidgetSurface};

#[derive(Debug)]
pub struct SurfaceHandleAdapter<S> {
    surface: S,
}

impl<S> SurfaceHandleAdapter<S> {
    pub fn new(surface: S) -> Self {
        Self { surface }
    }

    pub fn surface(&self) -> &S {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut S {
        &mut self.surface
    }

    pub fn into_surface(self) -> S {
        self.surface
    }

    fn apply_setters(
        &mut self,
        id: HostNodeId,
        handle: &S::Handle,
        setters: &[NativeWidgetSetter],
    ) -> GuiResult<()>
    where
        S: NativeWidgetSurface,
    {
        for setter in setters {
            self.surface.apply_native_setter(id, handle, setter)?;
        }
        Ok(())
    }
}

impl<S: Default> Default for SurfaceHandleAdapter<S> {
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S: NativeWidgetSurface> NativeHandleAdapter for SurfaceHandleAdapter<S> {
    type Handle = S::Handle;

    fn backend(&self) -> NativeBackendKind {
        self.surface.backend()
    }

    fn create_handle(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let handle = self.surface.create_native_widget(id, blueprint)?;
        self.apply_setters(id, &handle, &blueprint.config().create_setters())?;
        Ok(handle)
    }

    fn update_handle(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        self.apply_setters(id, handle, &blueprint.config().create_setters())
    }

    fn update_handle_config(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        _blueprint: &NativeWidgetBlueprint,
        patch: &NativeWidgetConfigPatch,
    ) -> GuiResult<()> {
        self.apply_setters(id, handle, patch.as_setters())
    }

    fn insert_child_handle(
        &mut self,
        parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()> {
        self.surface
            .insert_native_child(parent, parent_handle, child, child_handle, index)
    }

    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        self.surface.remove_native_widget(id, handle)
    }

    fn set_root_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.surface.set_native_root(id, handle)
    }

    fn request_focus_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.surface.request_native_focus(id, handle)
    }

    fn announce_accessibility_handle(
        &mut self,
        announcement: &AccessibilityAnnouncement,
        handle: &Self::Handle,
    ) -> GuiResult<()> {
        self.surface
            .announce_native_accessibility(announcement, handle)
    }

    fn position_overlay_handle(
        &mut self,
        overlay: HostNodeId,
        overlay_handle: &Self::Handle,
        anchor: HostNodeId,
        anchor_handle: &Self::Handle,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        self.surface.position_native_overlay(
            overlay,
            overlay_handle,
            anchor,
            anchor_handle,
            request,
        )
    }

    fn measure_collection_layout_handles(
        &mut self,
        collection: HostNodeId,
        collection_handle: &Self::Handle,
        items: &[(HostNodeId, CollectionKey, Self::Handle)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        self.surface
            .measure_native_collection_layout(collection, collection_handle, items)
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.surface.take_native_events()
    }
}
