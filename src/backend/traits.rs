use crate::error::GuiResult;
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::platform::{
    NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfigPatch, NativeWidgetSetter,
    PlatformCommand,
};

pub trait PlatformCommandExecutor {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()>;
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
    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()>;
    fn set_root_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()>;
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
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        Vec::new()
    }
}
