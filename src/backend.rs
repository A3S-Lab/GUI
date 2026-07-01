use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::{HostNodeId, NativeHost};
use crate::native::{NativeElement, NativeProps};
use crate::platform::{
    BlueprintHost, NativeBackendKind, NativeControlState, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetSetter, PlatformAdapter,
    PlatformCommand, PlatformPlanningHost,
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
        self.apply_setters(id, handle, &patch.setters())
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

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.surface.take_native_events()
    }
}

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

#[derive(Debug)]
pub struct DriverCommandExecutor<D: NativeWidgetDriver> {
    driver: D,
    commands: Vec<PlatformCommand>,
}

impl<D: NativeWidgetDriver> DriverCommandExecutor<D> {
    pub fn new(driver: D) -> Self {
        Self {
            driver,
            commands: Vec::new(),
        }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }

    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }

    pub fn commands(&self) -> &[PlatformCommand] {
        &self.commands
    }

    pub fn into_driver(self) -> D {
        self.driver
    }

    fn ensure_backend(&self, blueprint: &NativeWidgetBlueprint) -> GuiResult<()> {
        if blueprint.backend == self.driver.backend() {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "{:?} driver received {:?} blueprint",
                self.driver.backend(),
                blueprint.backend
            )))
        }
    }
}

impl<D: NativeWidgetDriver + Default> Default for DriverCommandExecutor<D> {
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<D: NativeWidgetDriver> PlatformCommandExecutor for DriverCommandExecutor<D> {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        match command {
            PlatformCommand::Create { id, blueprint } => {
                self.ensure_backend(blueprint)?;
                self.driver.create_widget(*id, blueprint)?;
            }
            PlatformCommand::Update { id, blueprint } => {
                self.ensure_backend(blueprint)?;
                self.driver.update_widget(*id, blueprint)?;
            }
            PlatformCommand::InsertChild {
                parent,
                child,
                index,
            } => {
                self.driver.insert_child(*parent, *child, *index)?;
            }
            PlatformCommand::Remove { id } => {
                self.driver.remove_widget(*id)?;
            }
            PlatformCommand::SetRoot { id } => {
                self.driver.set_root_widget(*id)?;
            }
        }
        self.commands.push(command.clone());
        Ok(())
    }
}

impl<D: NativeWidgetDriver + NativeEventSource> NativeEventSource for DriverCommandExecutor<D> {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.driver.take_native_events()
    }
}

#[derive(Debug)]
pub struct CommandExecutingHost<A: PlatformAdapter, E: PlatformCommandExecutor> {
    planning: PlatformPlanningHost<A>,
    executor: E,
    executed_commands: usize,
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> CommandExecutingHost<A, E> {
    pub fn new(adapter: A, executor: E) -> Self {
        Self {
            planning: PlatformPlanningHost::new(adapter),
            executor,
            executed_commands: 0,
        }
    }

    pub fn planning(&self) -> &PlatformPlanningHost<A> {
        &self.planning
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn executor_mut(&mut self) -> &mut E {
        &mut self.executor
    }

    pub fn into_parts(self) -> (PlatformPlanningHost<A>, E) {
        (self.planning, self.executor)
    }

    fn flush_commands(&mut self) -> GuiResult<()> {
        for command in &self.planning.commands()[self.executed_commands..] {
            self.executor.execute(command)?;
        }
        self.executed_commands = self.planning.commands().len();
        Ok(())
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor + NativeEventSource> NativeEventHost
    for CommandExecutingHost<A, E>
{
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.executor.take_native_events()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> BlueprintHost for CommandExecutingHost<A, E> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.planning.blueprint(id)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> NativeHost for CommandExecutingHost<A, E> {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let id = self.planning.create(element)?;
        self.flush_commands()?;
        Ok(id)
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        self.planning.update(id, props)?;
        self.flush_commands()
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.planning.insert_child(parent, child, index)?;
        self.flush_commands()
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.planning.remove(id)?;
        self.flush_commands()
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.planning.set_root(id)?;
        self.flush_commands()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::CompiledJsxNode;
    use crate::event::NativeEventKind;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{Gtk4Adapter, PlatformAdapter, WinUiAdapter};
    use crate::runtime::GuiRuntime;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Default)]
    struct TestWidgetDriver {
        calls: Vec<String>,
        events: Vec<NativeEvent>,
    }

    #[derive(Debug, Clone)]
    struct ThreadBoundHandle {
        widget_class: String,
        _thread_affine: Rc<()>,
    }

    #[derive(Debug, Clone)]
    struct SurfaceHandle {
        widget_class: String,
        _thread_affine: Rc<()>,
    }

    #[derive(Debug, Default)]
    struct ThreadBoundHandleAdapter {
        calls: Rc<RefCell<Vec<String>>>,
    }

    #[derive(Debug, Default)]
    struct TestNativeSurface {
        calls: Rc<RefCell<Vec<String>>>,
        events: Rc<RefCell<Vec<NativeEvent>>>,
    }

    impl NativeWidgetSurface for TestNativeSurface {
        type Handle = SurfaceHandle;

        fn backend(&self) -> NativeBackendKind {
            NativeBackendKind::Gtk4
        }

        fn create_native_widget(
            &mut self,
            id: HostNodeId,
            blueprint: &NativeWidgetBlueprint,
        ) -> GuiResult<Self::Handle> {
            self.calls
                .borrow_mut()
                .push(format!("create:{}:{}", id.get(), blueprint.widget_class));
            Ok(SurfaceHandle {
                widget_class: blueprint.widget_class.clone(),
                _thread_affine: Rc::new(()),
            })
        }

        fn apply_native_setter(
            &mut self,
            id: HostNodeId,
            handle: &Self::Handle,
            setter: &NativeWidgetSetter,
        ) -> GuiResult<()> {
            let call = match setter {
                NativeWidgetSetter::SetLabel(value) => {
                    format!(
                        "setter:{}:{}:label={}",
                        id.get(),
                        handle.widget_class,
                        value.as_deref().unwrap_or("<none>")
                    )
                }
                NativeWidgetSetter::SetEnabled(value) => {
                    format!(
                        "setter:{}:{}:enabled={value}",
                        id.get(),
                        handle.widget_class
                    )
                }
                NativeWidgetSetter::SetPlaceholder(value) => {
                    format!(
                        "setter:{}:{}:placeholder={}",
                        id.get(),
                        handle.widget_class,
                        value.as_deref().unwrap_or("<none>")
                    )
                }
                other => {
                    format!("setter:{}:{}:{other:?}", id.get(), handle.widget_class)
                }
            };
            self.calls.borrow_mut().push(call);
            Ok(())
        }

        fn insert_native_child(
            &mut self,
            parent: HostNodeId,
            parent_handle: &Self::Handle,
            child: HostNodeId,
            child_handle: &Self::Handle,
            index: usize,
        ) -> GuiResult<()> {
            self.calls.borrow_mut().push(format!(
                "insert:{}:{}:{}:{}:{index}",
                parent.get(),
                parent_handle.widget_class,
                child.get(),
                child_handle.widget_class
            ));
            Ok(())
        }

        fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
            self.calls
                .borrow_mut()
                .push(format!("remove:{}:{}", id.get(), handle.widget_class));
            Ok(())
        }

        fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
            self.calls
                .borrow_mut()
                .push(format!("root:{}:{}", id.get(), handle.widget_class));
            Ok(())
        }

        fn take_native_events(&mut self) -> Vec<NativeEvent> {
            std::mem::take(&mut self.events.borrow_mut())
        }
    }

    impl NativeHandleAdapter for ThreadBoundHandleAdapter {
        type Handle = ThreadBoundHandle;

        fn backend(&self) -> NativeBackendKind {
            NativeBackendKind::Gtk4
        }

        fn create_handle(
            &mut self,
            id: HostNodeId,
            blueprint: &NativeWidgetBlueprint,
        ) -> GuiResult<Self::Handle> {
            self.calls
                .borrow_mut()
                .push(format!("create:{}:{}", id.get(), blueprint.widget_class));
            Ok(ThreadBoundHandle {
                widget_class: blueprint.widget_class.clone(),
                _thread_affine: Rc::new(()),
            })
        }

        fn update_handle(
            &mut self,
            id: HostNodeId,
            handle: &Self::Handle,
            blueprint: &NativeWidgetBlueprint,
        ) -> GuiResult<()> {
            self.calls.borrow_mut().push(format!(
                "update:{}:{}->{}",
                id.get(),
                handle.widget_class,
                blueprint.widget_class
            ));
            Ok(())
        }

        fn update_handle_config(
            &mut self,
            id: HostNodeId,
            _handle: &Self::Handle,
            _blueprint: &NativeWidgetBlueprint,
            patch: &NativeWidgetConfigPatch,
        ) -> GuiResult<()> {
            let label = patch
                .label
                .as_ref()
                .and_then(|change| change.after.as_deref())
                .unwrap_or("<unchanged>");
            let enabled = patch
                .enabled
                .as_ref()
                .map(|change| change.after.to_string())
                .unwrap_or_else(|| "unchanged".to_string());
            self.calls.borrow_mut().push(format!(
                "patch:{}:label={label}:enabled={enabled}",
                id.get()
            ));
            Ok(())
        }

        fn insert_child_handle(
            &mut self,
            parent: HostNodeId,
            parent_handle: &Self::Handle,
            child: HostNodeId,
            child_handle: &Self::Handle,
            index: usize,
        ) -> GuiResult<()> {
            self.calls.borrow_mut().push(format!(
                "insert:{}:{}:{}:{}:{index}",
                parent.get(),
                parent_handle.widget_class,
                child.get(),
                child_handle.widget_class
            ));
            Ok(())
        }

        fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
            self.calls
                .borrow_mut()
                .push(format!("remove:{}:{}", id.get(), handle.widget_class));
            Ok(())
        }

        fn set_root_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
            self.calls
                .borrow_mut()
                .push(format!("root:{}:{}", id.get(), handle.widget_class));
            Ok(())
        }
    }

    impl NativeWidgetDriver for TestWidgetDriver {
        fn backend(&self) -> NativeBackendKind {
            NativeBackendKind::Gtk4
        }

        fn create_widget(
            &mut self,
            id: HostNodeId,
            blueprint: &NativeWidgetBlueprint,
        ) -> GuiResult<()> {
            self.calls
                .push(format!("create:{}:{}", id.get(), blueprint.widget_class));
            Ok(())
        }

        fn update_widget(
            &mut self,
            id: HostNodeId,
            blueprint: &NativeWidgetBlueprint,
        ) -> GuiResult<()> {
            self.calls
                .push(format!("update:{}:{}", id.get(), blueprint.widget_class));
            Ok(())
        }

        fn insert_child(
            &mut self,
            parent: HostNodeId,
            child: HostNodeId,
            index: usize,
        ) -> GuiResult<()> {
            self.calls
                .push(format!("insert:{}:{}:{index}", parent.get(), child.get()));
            Ok(())
        }

        fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
            self.calls.push(format!("remove:{}", id.get()));
            Ok(())
        }

        fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
            self.calls.push(format!("root:{}", id.get()));
            Ok(())
        }
    }

    impl NativeEventSource for TestWidgetDriver {
        fn take_native_events(&mut self) -> Vec<NativeEvent> {
            std::mem::take(&mut self.events)
        }
    }

    #[test]
    fn driver_command_executor_delegates_native_commands_to_driver() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut executor = DriverCommandExecutor::new(TestWidgetDriver::default());

        executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(1),
                blueprint: blueprint.clone(),
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::SetRoot {
                id: HostNodeId::new(1),
            })
            .unwrap();

        assert_eq!(
            executor.driver().calls,
            vec!["create:1:gtk::Button", "root:1"]
        );
        assert_eq!(executor.commands().len(), 2);
    }

    #[test]
    fn driver_command_executor_rejects_wrong_backend_blueprint() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = WinUiAdapter.blueprint(&element);
        let mut executor = DriverCommandExecutor::new(TestWidgetDriver::default());

        let error = executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(1),
                blueprint,
            })
            .unwrap_err();

        assert!(error
            .to_string()
            .contains("driver received WinUI blueprint"));
        assert!(executor.commands().is_empty());
        assert!(executor.driver().calls.is_empty());
    }

    #[test]
    fn handle_widget_driver_accepts_thread_bound_native_handles() {
        let adapter = ThreadBoundHandleAdapter::default();
        let calls = adapter.calls.clone();
        let driver = HandleWidgetDriver::new(adapter);
        let mut executor = DriverCommandExecutor::new(driver);
        let root = Gtk4Adapter.blueprint(&NativeElement::new("root", NativeRole::View));
        let button = Gtk4Adapter.blueprint(&NativeElement::new("save", NativeRole::Button));

        executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(1),
                blueprint: root,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(2),
                blueprint: button,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::InsertChild {
                parent: HostNodeId::new(1),
                child: HostNodeId::new(2),
                index: 0,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::SetRoot {
                id: HostNodeId::new(1),
            })
            .unwrap();

        assert_eq!(
            calls.borrow().as_slice(),
            [
                "create:1:gtk::Box",
                "create:2:gtk::Button",
                "insert:1:gtk::Box:2:gtk::Button:0",
                "root:1:gtk::Box",
            ]
        );
        assert_eq!(executor.driver().root(), Some(HostNodeId::new(1)));
        assert_eq!(executor.driver().handles().len(), 2);
        assert_eq!(executor.driver().configs().len(), 2);
    }

    #[test]
    fn handle_widget_driver_passes_config_patch_to_native_adapter() {
        let adapter = ThreadBoundHandleAdapter::default();
        let calls = adapter.calls.clone();
        let driver = HandleWidgetDriver::new(adapter);
        let mut executor = DriverCommandExecutor::new(driver);
        let first = Gtk4Adapter.blueprint(
            &NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        );
        let second = Gtk4Adapter.blueprint(
            &NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Saved").disabled(true)),
        );

        executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(1),
                blueprint: first,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::Update {
                id: HostNodeId::new(1),
                blueprint: second,
            })
            .unwrap();

        assert_eq!(
            calls.borrow().as_slice(),
            ["create:1:gtk::Button", "patch:1:label=Saved:enabled=false",]
        );
        let config = executor.driver().config(HostNodeId::new(1)).unwrap();
        assert_eq!(config.label.as_deref(), Some("Saved"));
        assert!(!config.enabled);
    }

    #[test]
    fn surface_handle_adapter_applies_native_setters_to_surface() {
        let surface = TestNativeSurface::default();
        let calls = surface.calls.clone();
        let adapter = SurfaceHandleAdapter::new(surface);
        let driver = HandleWidgetDriver::new(adapter);
        let mut executor = DriverCommandExecutor::new(driver);
        let first = Gtk4Adapter.blueprint(
            &NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        );
        let second = Gtk4Adapter.blueprint(
            &NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Saved").disabled(true)),
        );

        executor
            .execute(&PlatformCommand::Create {
                id: HostNodeId::new(1),
                blueprint: first,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::Update {
                id: HostNodeId::new(1),
                blueprint: second,
            })
            .unwrap();
        executor
            .execute(&PlatformCommand::SetRoot {
                id: HostNodeId::new(1),
            })
            .unwrap();

        let calls = calls.borrow();
        assert!(calls.contains(&"create:1:gtk::Button".to_string()));
        assert!(calls.contains(&"setter:1:gtk::Button:label=Save".to_string()));
        assert!(calls.contains(&"setter:1:gtk::Button:enabled=true".to_string()));
        assert!(calls.contains(&"setter:1:gtk::Button:label=Saved".to_string()));
        assert!(calls.contains(&"setter:1:gtk::Button:enabled=false".to_string()));
        assert!(calls.contains(&"root:1:gtk::Button".to_string()));
        let config = executor.driver().config(HostNodeId::new(1)).unwrap();
        assert_eq!(config.label.as_deref(), Some("Saved"));
        assert!(!config.enabled);
    }

    #[test]
    fn surface_handle_adapter_drains_native_surface_events() {
        let surface = TestNativeSurface::default();
        let events = surface.events.clone();
        let mut driver = HandleWidgetDriver::new(SurfaceHandleAdapter::new(surface));

        events
            .borrow_mut()
            .push(NativeEvent::new(HostNodeId::new(3), NativeEventKind::Press));
        driver.push_native_event(NativeEvent::new(HostNodeId::new(4), NativeEventKind::Focus));

        let drained = driver.take_native_events();

        assert_eq!(
            drained,
            vec![
                NativeEvent::new(HostNodeId::new(4), NativeEventKind::Focus),
                NativeEvent::new(HostNodeId::new(3), NativeEventKind::Press),
            ]
        );
        assert!(driver.take_native_events().is_empty());
    }

    #[test]
    fn command_executing_host_dispatches_driver_native_events_to_actions() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let executor = DriverCommandExecutor::new(TestWidgetDriver::default());
        let host = CommandExecutingHost::new(Gtk4Adapter, executor);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveProfile");

        let root_id = runtime.render_compiled(&compiled).unwrap();
        runtime
            .host_mut()
            .executor_mut()
            .driver_mut()
            .events
            .push(NativeEvent::new(root_id, NativeEventKind::Press));
        let invocations = runtime.dispatch_pending_native_events().unwrap();

        assert_eq!(invocations.len(), 1);
        assert_eq!(invocations[0].action, "saveProfile");
        assert_eq!(runtime.actions().invocations().len(), 1);
        assert!(runtime
            .host_mut()
            .executor_mut()
            .take_native_events()
            .is_empty());
    }

    #[test]
    fn command_executing_host_creates_backend_object_tree_from_compiled_jsx() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "form",
              "tag": "form",
              "children": [
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveProfile"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(WinUiAdapter, RecordingBackend::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let backend = runtime.host().executor();
        let root = backend.object(root_id).unwrap();
        let child = backend.object(root.children[0]).unwrap();

        assert_eq!(root.widget_class, "Microsoft.UI.Xaml.Controls.StackPanel");
        assert_eq!(child.widget_class, "Microsoft.UI.Xaml.Controls.Button");
        assert_eq!(child.label.as_deref(), Some("Save"));
        assert_eq!(child.action.as_deref(), Some("saveProfile"));
    }

    #[test]
    fn command_executing_host_applies_updates_and_removes_to_backend_objects() {
        let first: CompiledJsxNode = serde_json::from_str(
            r#"
            {"kind": "element", "key": "root", "tag": "div", "children": [
              {"kind": "element", "key": "a", "tag": "Button", "children": [
                {"kind": "text", "key": "a-text", "value": "A"}
              ]},
              {"kind": "element", "key": "b", "tag": "Button", "children": [
                {"kind": "text", "key": "b-text", "value": "B"}
              ]}
            ]}
            "#,
        )
        .unwrap();
        let second: CompiledJsxNode = serde_json::from_str(
            r#"
            {"kind": "element", "key": "root", "tag": "div", "children": [
              {"kind": "element", "key": "b", "tag": "Button", "children": [
                {"kind": "text", "key": "b-text", "value": "B+"}
              ]}
            ]}
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&first).unwrap();
        runtime.render_compiled(&second).unwrap();

        let backend = runtime.host().executor();
        let root = backend.object(root_id).unwrap();
        assert_eq!(root.children.len(), 1);
        let only_child = backend.object(root.children[0]).unwrap();
        assert_eq!(only_child.label.as_deref(), Some("B+"));
        assert!(backend
            .commands()
            .iter()
            .any(|command| matches!(command, PlatformCommand::Remove { .. })));
        assert!(backend
            .commands()
            .iter()
            .any(|command| matches!(command, PlatformCommand::Update { .. })));
    }
}
