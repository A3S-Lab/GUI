use super::*;
use crate::accessibility::AccessibilityRole;
use crate::compiler::CompiledJsxNode;
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{
    Gtk4Adapter, NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfigPatch,
    NativeWidgetSetter, PlatformAdapter, PlatformCommand, WinUiAdapter,
};
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
    fail_removes: bool,
}

#[derive(Debug, Default)]
struct TestNativeSurface {
    calls: Rc<RefCell<Vec<String>>>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
}

#[derive(Debug, Default)]
struct FailingCommandExecutor {
    fail_creates: bool,
    fail_updates: bool,
}

impl PlatformCommandExecutor for FailingCommandExecutor {
    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        match command {
            PlatformCommand::Create { .. } if self.fail_creates => {
                Err(GuiError::host("forced backend create failure"))
            }
            PlatformCommand::Update { .. } if self.fail_updates => {
                Err(GuiError::host("forced backend update failure"))
            }
            _ => Ok(()),
        }
    }
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
        if self.fail_removes {
            self.calls.borrow_mut().push(format!(
                "remove:{}:{}:failed",
                id.get(),
                handle.widget_class
            ));
            return Err(GuiError::host("forced handle remove failure"));
        }
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
fn handle_widget_driver_rejects_duplicate_creates_without_replacing_handle() {
    let adapter = ThreadBoundHandleAdapter::default();
    let calls = adapter.calls.clone();
    let driver = HandleWidgetDriver::new(adapter);
    let mut executor = DriverCommandExecutor::new(driver);
    let id = HostNodeId::new(1);
    let first = Gtk4Adapter.blueprint(
        &NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save")),
    );
    let second = Gtk4Adapter.blueprint(
        &NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().label("Email")),
    );

    executor
        .execute(&PlatformCommand::Create {
            id,
            blueprint: first,
        })
        .unwrap();
    let error = executor
        .execute(&PlatformCommand::Create {
            id,
            blueprint: second,
        })
        .unwrap_err();

    assert!(error.to_string().contains("native handle 1 already exists"));
    assert_eq!(executor.commands().len(), 1);
    assert_eq!(executor.driver().handles().len(), 1);
    assert_eq!(
        executor.driver().config(id).unwrap().label.as_deref(),
        Some("Save")
    );
    assert_eq!(calls.borrow().as_slice(), ["create:1:gtk::Button"]);
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
fn handle_widget_driver_preserves_state_when_remove_handle_fails() {
    let adapter = ThreadBoundHandleAdapter {
        fail_removes: true,
        ..Default::default()
    };
    let calls = adapter.calls.clone();
    let driver = HandleWidgetDriver::new(adapter);
    let mut executor = DriverCommandExecutor::new(driver);
    let id = HostNodeId::new(1);
    let root = Gtk4Adapter.blueprint(&NativeElement::new("root", NativeRole::View));

    executor
        .execute(&PlatformCommand::Create {
            id,
            blueprint: root,
        })
        .unwrap();
    executor.execute(&PlatformCommand::SetRoot { id }).unwrap();
    let command_count = executor.commands().len();

    let error = executor
        .execute(&PlatformCommand::Remove { id })
        .unwrap_err();

    assert!(error.to_string().contains("forced handle remove failure"));
    assert_eq!(executor.commands().len(), command_count);
    assert_eq!(executor.driver().root(), Some(id));
    assert!(executor.driver().handle(id).is_some());
    assert!(executor.driver().config(id).is_some());
    assert_eq!(
        calls.borrow().as_slice(),
        [
            "create:1:gtk::Box",
            "root:1:gtk::Box",
            "remove:1:gtk::Box:failed",
        ]
    );
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
fn recording_backend_reparents_children_and_rejects_cycles() {
    let mut backend = RecordingBackend::default();
    let first = HostNodeId::new(1);
    let second = HostNodeId::new(2);
    let child = HostNodeId::new(3);
    let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = Gtk4Adapter.blueprint(&NativeElement::new("child", NativeRole::Button));

    backend
        .execute(&PlatformCommand::Create {
            id: first,
            blueprint: container.clone(),
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::Create {
            id: second,
            blueprint: container,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::Create {
            id: child,
            blueprint: button,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::InsertChild {
            parent: first,
            child,
            index: 0,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::InsertChild {
            parent: second,
            child,
            index: 0,
        })
        .unwrap();

    assert!(backend.object(first).unwrap().children.is_empty());
    assert_eq!(backend.object(second).unwrap().children, vec![child]);

    let command_count = backend.commands().len();
    let error = backend
        .execute(&PlatformCommand::InsertChild {
            parent: child,
            child,
            index: 0,
        })
        .unwrap_err();

    assert!(error.to_string().contains("cannot insert backend object"));
    assert_eq!(backend.commands().len(), command_count);

    let error = backend
        .execute(&PlatformCommand::InsertChild {
            parent: child,
            child: second,
            index: 0,
        })
        .unwrap_err();

    assert!(error.to_string().contains("would create a cycle"));
    assert_eq!(backend.commands().len(), command_count);
    assert_eq!(backend.object(second).unwrap().children, vec![child]);
    assert!(backend.object(child).unwrap().children.is_empty());
}

#[test]
fn recording_backend_rejects_duplicate_creates_without_overwriting_object() {
    let mut backend = RecordingBackend::default();
    let id = HostNodeId::new(1);
    let first = Gtk4Adapter.blueprint(
        &NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save")),
    );
    let second = Gtk4Adapter.blueprint(
        &NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().label("Email")),
    );

    backend
        .execute(&PlatformCommand::Create {
            id,
            blueprint: first,
        })
        .unwrap();
    let error = backend
        .execute(&PlatformCommand::Create {
            id,
            blueprint: second,
        })
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("backend object 1 already exists"));
    assert_eq!(backend.commands().len(), 1);
    assert_eq!(backend.objects().len(), 1);
    assert_eq!(backend.object(id).unwrap().label.as_deref(), Some("Save"));
    assert_eq!(backend.object(id).unwrap().widget_class, "gtk::Button");
}

#[test]
fn recording_backend_remove_deletes_entire_subtree() {
    let mut backend = RecordingBackend::default();
    let root = HostNodeId::new(1);
    let child = HostNodeId::new(2);
    let grandchild = HostNodeId::new(3);
    let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
    let button = Gtk4Adapter.blueprint(&NativeElement::new("child", NativeRole::Button));

    backend
        .execute(&PlatformCommand::Create {
            id: root,
            blueprint: container.clone(),
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::Create {
            id: child,
            blueprint: container,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::Create {
            id: grandchild,
            blueprint: button,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::InsertChild {
            parent: root,
            child,
            index: 0,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::InsertChild {
            parent: child,
            child: grandchild,
            index: 0,
        })
        .unwrap();
    backend
        .execute(&PlatformCommand::SetRoot { id: root })
        .unwrap();
    let command_count = backend.commands().len();

    backend
        .execute(&PlatformCommand::Remove { id: root })
        .unwrap();

    assert!(backend.root().is_none());
    assert!(backend.objects().is_empty());
    assert_eq!(backend.commands().len(), command_count + 1);
    assert_eq!(
        backend.commands().last(),
        Some(&PlatformCommand::Remove { id: root })
    );
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
fn command_executing_host_rolls_back_planning_after_backend_create_failure() {
    let host = CommandExecutingHost::new(
        Gtk4Adapter,
        FailingCommandExecutor {
            fail_creates: true,
            ..FailingCommandExecutor::default()
        },
    );
    let mut runtime = GuiRuntime::new(host);

    let error = runtime
        .render_native(&NativeElement::new("save", NativeRole::Button))
        .unwrap_err();

    assert!(error.to_string().contains("forced backend create failure"));
    assert!(runtime.host().planning().nodes().is_empty());
    assert!(runtime.host().planning().commands().is_empty());
    assert!(runtime.host().planning().root().is_none());
}

#[test]
fn command_executing_host_rolls_back_planning_after_backend_update_failure() {
    let host = CommandExecutingHost::new(Gtk4Adapter, FailingCommandExecutor::default());
    let mut runtime = GuiRuntime::new(host);
    let first =
        NativeElement::new("save", NativeRole::Button).with_props(NativeProps::new().label("Save"));
    let second = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().label("Saved"));

    let root_id = runtime.render_native(&first).unwrap();
    runtime.host_mut().executor_mut().fail_updates = true;
    let error = runtime.render_native(&second).unwrap_err();

    assert!(error.to_string().contains("forced backend update failure"));
    let planned = runtime.host().planning().node(root_id).unwrap();
    assert_eq!(planned.blueprint.label.as_deref(), Some("Save"));
    assert_eq!(runtime.host().planning().root(), Some(root_id));
    assert!(!runtime
        .host()
        .planning()
        .commands()
        .iter()
        .any(|command| matches!(command, PlatformCommand::Update { .. })));
}

#[test]
fn command_executing_host_dispatches_pending_state_events_without_invocation() {
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
        .push(NativeEvent::new(root_id, NativeEventKind::Focus));
    runtime
        .host_mut()
        .executor_mut()
        .driver_mut()
        .events
        .push(NativeEvent::new(root_id, NativeEventKind::Press));

    let invocations = runtime.dispatch_pending_native_events().unwrap();

    assert_eq!(invocations.len(), 1);
    assert_eq!(invocations[0].action, "saveProfile");
    assert!(runtime.accessibility_tree().unwrap().focused);
    assert!(runtime
        .host_mut()
        .executor_mut()
        .take_native_events()
        .is_empty());
}

#[test]
fn command_executing_host_handles_unbound_native_events_without_invocation() {
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
        .push(NativeEvent::new(root_id, NativeEventKind::Focus));
    runtime
        .host_mut()
        .executor_mut()
        .driver_mut()
        .events
        .push(NativeEvent::new(root_id, NativeEventKind::Press));

    let invocations = runtime.handle_pending_native_events().unwrap();

    assert_eq!(invocations.len(), 1);
    assert_eq!(invocations[0].action, "saveProfile");
    assert!(runtime.accessibility_tree().unwrap().focused);
}

#[test]
fn command_executing_host_reports_pending_native_event_results() {
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
        .push(NativeEvent::new(root_id, NativeEventKind::Focus));
    runtime
        .host_mut()
        .executor_mut()
        .driver_mut()
        .events
        .push(NativeEvent::new(root_id, NativeEventKind::Press));

    let events = runtime.handle_pending_native_event_results().unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event.kind, NativeEventKind::Focus);
    assert!(events[0].invocation.is_none());
    assert_eq!(events[0].interaction_changes.len(), 1);
    assert!(events[0].interaction_changes[0].after.focused);
    let json = serde_json::to_string(&events[0]).unwrap();
    assert!(json.contains(r#""interactionChanges""#));
    let decoded: crate::runtime::HandledNativeEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, events[0]);
    assert_eq!(events[1].event.kind, NativeEventKind::Press);
    assert_eq!(
        events[1]
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("saveProfile")
    );
    assert!(events[1].interaction_changes.is_empty());
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

    assert_eq!(
        root.widget_class,
        "Microsoft.UI.Xaml.Controls.StackPanel(form)"
    );
    assert_eq!(child.widget_class, "Microsoft.UI.Xaml.Controls.Button");
    assert_eq!(child.label.as_deref(), Some("Save"));
    assert_eq!(child.action.as_deref(), Some("saveProfile"));
}

#[test]
fn command_executing_host_exposes_rendered_accessibility_tree() {
    let compiled: CompiledJsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {
                "attributes": {
                  "aria-label": "Save profile",
                  "aria-describedby": "save-help",
                  "aria-description": "Writes profile changes",
                  "aria-pressed": "false"
                }
              }
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(WinUiAdapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);

    runtime.render_compiled(&compiled).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.role, AccessibilityRole::Button);
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
    assert_eq!(
        accessibility.relationships.described_by.as_deref(),
        Some("save-help")
    );
    assert_eq!(
        accessibility.description.description.as_deref(),
        Some("Writes profile changes")
    );
    assert_eq!(accessibility.state.pressed.as_deref(), Some("false"));
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
