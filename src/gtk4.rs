use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::backend::{
    DriverCommandExecutor, HandleWidgetDriver, NativeEventSource, NativeHandleAdapter,
    NativeWidgetDriver,
};
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::platform::{
    apply_widget_setters, NativeBackendKind, NativeControlState, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetSetter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gtk4WidgetKind {
    ApplicationWindow,
    Box,
    Label,
    Button,
    Entry,
    SpinButton,
    TextView,
    CheckButton,
    Switch,
    DropDown,
    ListBox,
    ListBoxRow,
    Dialog,
    Popover,
    Notebook,
    Menu,
    MenuItem,
    Separator,
    Scale,
    ProgressBar,
    ToolbarBox,
}

impl Gtk4WidgetKind {
    pub fn from_widget_class(widget_class: &str) -> GuiResult<Self> {
        match widget_class {
            "gtk::ApplicationWindow" => Ok(Gtk4WidgetKind::ApplicationWindow),
            "gtk::Box"
            | "gtk::Box(document)"
            | "gtk::Box(document-head)"
            | "gtk::Box(document-body)"
            | "gtk::Box(metadata)"
            | "gtk::Box(resource-link)"
            | "gtk::Box(style-sheet)"
            | "gtk::Box(script)"
            | "gtk::Box(template)"
            | "gtk::Box(slot)"
            | "gtk::Box(paragraph)"
            | "gtk::Box(preformatted-text)"
            | "gtk::Box(block-quote)"
            | "gtk::Box(contact-address)"
            | "gtk::Box(no-break-text)"
            | "gtk::Box(centered-text)"
            | "gtk::Box(font-text)"
            | "gtk::Box(big-text)"
            | "gtk::Box(teletype-text)"
            | "gtk::Box(applet)"
            | "gtk::Box(background-sound)"
            | "gtk::Box(frame)"
            | "gtk::Box(frameset)"
            | "gtk::Box(noembed-fallback)"
            | "gtk::Box(noframes-fallback)"
            | "gtk::Box(marquee)"
            | "gtk::Box(math)"
            | "gtk::Box(nextid)"
            | "gtk::Box(selected-content)"
            | "gtk::Box(heading-group)"
            | "gtk::Box(ruby)"
            | "gtk::Box(ruby-text-container)"
            | "gtk::Box(main)"
            | "gtk::Box(navigation)"
            | "gtk::Box(header)"
            | "gtk::Box(footer)"
            | "gtk::Box(article)"
            | "gtk::Box(section)"
            | "gtk::Box(aside)"
            | "gtk::Box(search)"
            | "gtk::Box(disclosure)"
            | "gtk::Box(figure)"
            | "gtk::Box(description-list)"
            | "gtk::Box(description-details)"
            | "gtk::Box(form)"
            | "gtk::Box(fieldset)"
            | "gtk::Box(option-group)"
            | "gtk::Box(radio-group)"
            | "gtk::Box(embedded-content)"
            | "gtk::Grid(table)"
            | "gtk::Grid(row)"
            | "gtk::Grid(cell)"
            | "gtk::Box(table-section)"
            | "gtk::ColumnViewColumn"
            | "gtk::Picture"
            | "gtk::Video"
            | "gtk::DrawingArea"
            | "gtk::DrawingArea(image-map)" => Ok(Gtk4WidgetKind::Box),
            "gtk::Label"
            | "gtk::Label(abbreviation)"
            | "gtk::Label(citation)"
            | "gtk::Label(definition)"
            | "gtk::Label(data-value)"
            | "gtk::Label(inserted-text)"
            | "gtk::Label(deleted-text)"
            | "gtk::Label(marked-text)"
            | "gtk::Label(time)"
            | "gtk::Label(emphasis)"
            | "gtk::Label(strong-text)"
            | "gtk::Label(code)"
            | "gtk::Label(keyboard-input)"
            | "gtk::Label(sample-output)"
            | "gtk::Label(variable)"
            | "gtk::Label(inline-quote)"
            | "gtk::Label(subscript)"
            | "gtk::Label(superscript)"
            | "gtk::Label(small-text)"
            | "gtk::Label(bold-text)"
            | "gtk::Label(italic-text)"
            | "gtk::Label(struck-text)"
            | "gtk::Label(underlined-text)"
            | "gtk::Label(bidi-isolate)"
            | "gtk::Label(bidi-override)"
            | "gtk::Label(line-break)"
            | "gtk::Label(word-break-opportunity)"
            | "gtk::Label(document-title)"
            | "gtk::Label(heading)"
            | "gtk::Label(ruby-base)"
            | "gtk::Label(ruby-text)"
            | "gtk::Label(ruby-parenthesis)"
            | "gtk::Label(tab)"
            | "gtk::Label(figure-caption)"
            | "gtk::Label(description-term)"
            | "gtk::Label(legend)"
            | "gtk::Label(output)"
            | "gtk::Label(table-caption)" => Ok(Gtk4WidgetKind::Label),
            "gtk::Button"
            | "gtk::LinkButton"
            | "gtk::LinkButton(image-map-area)"
            | "gtk::Button(disclosure-summary)" => Ok(Gtk4WidgetKind::Button),
            "gtk::Entry" | "gtk::SearchEntry" | "gtk::PasswordEntry" => Ok(Gtk4WidgetKind::Entry),
            "gtk::SpinButton" => Ok(Gtk4WidgetKind::SpinButton),
            "gtk::TextView" => Ok(Gtk4WidgetKind::TextView),
            "gtk::CheckButton" | "gtk::CheckButton(radio)" => Ok(Gtk4WidgetKind::CheckButton),
            "gtk::Switch" => Ok(Gtk4WidgetKind::Switch),
            "gtk::DropDown" => Ok(Gtk4WidgetKind::DropDown),
            "gtk::ListBox" => Ok(Gtk4WidgetKind::ListBox),
            "gtk::ListBoxRow" => Ok(Gtk4WidgetKind::ListBoxRow),
            "gtk::Dialog" => Ok(Gtk4WidgetKind::Dialog),
            "gtk::Popover" => Ok(Gtk4WidgetKind::Popover),
            "gtk::Notebook" => Ok(Gtk4WidgetKind::Notebook),
            "gio::Menu" => Ok(Gtk4WidgetKind::Menu),
            "gio::MenuItem" => Ok(Gtk4WidgetKind::MenuItem),
            "gtk::Separator" => Ok(Gtk4WidgetKind::Separator),
            "gtk::Scale" => Ok(Gtk4WidgetKind::Scale),
            "gtk::ProgressBar" | "gtk::ProgressBar(meter)" => Ok(Gtk4WidgetKind::ProgressBar),
            "gtk::Box(toolbar)" => Ok(Gtk4WidgetKind::ToolbarBox),
            other => Err(GuiError::host(format!(
                "unsupported GTK4 widget class {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gtk4NativeObject {
    pub id: HostNodeId,
    pub kind: Gtk4WidgetKind,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct Gtk4WidgetDriver {
    root: Option<HostNodeId>,
    objects: BTreeMap<HostNodeId, Gtk4NativeObject>,
    events: Vec<NativeEvent>,
}

pub type Gtk4CommandExecutor = DriverCommandExecutor<Gtk4WidgetDriver>;

#[derive(Debug, Clone)]
pub struct Gtk4NativeHandle {
    state: Rc<RefCell<Gtk4NativeHandleState>>,
}

impl Gtk4NativeHandle {
    pub fn state(&self) -> Ref<'_, Gtk4NativeHandleState> {
        self.state.borrow()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gtk4NativeHandleState {
    pub id: HostNodeId,
    pub kind: Gtk4WidgetKind,
    pub config: NativeWidgetConfig,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub applied_setters: Vec<NativeWidgetSetter>,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct Gtk4HandleAdapter;

pub type Gtk4HandleDriver = HandleWidgetDriver<Gtk4HandleAdapter>;
pub type Gtk4HandleCommandExecutor = DriverCommandExecutor<Gtk4HandleDriver>;

impl Gtk4WidgetDriver {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn object(&self, id: HostNodeId) -> Option<&Gtk4NativeObject> {
        self.objects.get(&id)
    }

    pub fn objects(&self) -> &BTreeMap<HostNodeId, Gtk4NativeObject> {
        &self.objects
    }

    pub fn push_native_event(&mut self, event: NativeEvent) {
        self.events.push(event);
    }

    pub fn queued_native_events(&self) -> &[NativeEvent] {
        &self.events
    }

    fn ensure_object(&self, id: HostNodeId) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "GTK4 object {} does not exist",
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

impl NativeEventSource for Gtk4WidgetDriver {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events)
    }
}

impl NativeHandleAdapter for Gtk4HandleAdapter {
    type Handle = Gtk4NativeHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }

    fn create_handle(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let config = blueprint.config();
        Ok(Gtk4NativeHandle {
            state: Rc::new(RefCell::new(Gtk4NativeHandleState {
                id,
                kind: Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
                label: config.label.clone(),
                value: config.value.clone(),
                action: config.action.clone(),
                applied_setters: config.create_setters(),
                config,
                control_state: blueprint.control_state.clone(),
                children: Vec::new(),
            })),
        })
    }

    fn update_handle(
        &mut self,
        _id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let mut state = handle.state.borrow_mut();
        state.kind = Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        state.config = blueprint.config();
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        let setters = state.config.create_setters();
        state.applied_setters.extend(setters);
        state.control_state = blueprint.control_state.clone();
        Ok(())
    }

    fn update_handle_config(
        &mut self,
        _id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
        patch: &NativeWidgetConfigPatch,
    ) -> GuiResult<()> {
        let mut state = handle.state.borrow_mut();
        state.kind = Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let setters = patch.setters();
        apply_widget_setters(&mut state.config, &setters);
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        state.control_state = blueprint.control_state.clone();
        state.applied_setters.extend(setters);
        Ok(())
    }

    fn insert_child_handle(
        &mut self,
        _parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        _child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()> {
        let mut parent = parent_handle.state.borrow_mut();
        parent.children.retain(|existing| *existing != child);
        let index = index.min(parent.children.len());
        parent.children.insert(index, child);
        Ok(())
    }

    fn remove_child_handle(
        &mut self,
        _parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        _child_handle: &Self::Handle,
    ) -> GuiResult<()> {
        parent_handle
            .state
            .borrow_mut()
            .children
            .retain(|existing| *existing != child);
        Ok(())
    }

    fn remove_handle(&mut self, _id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        handle.state.borrow_mut().children.clear();
        Ok(())
    }

    fn set_root_handle(&mut self, _id: HostNodeId, _handle: &Self::Handle) -> GuiResult<()> {
        Ok(())
    }
}

impl NativeWidgetDriver for Gtk4WidgetDriver {
    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }

    fn create_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            return Err(GuiError::host(format!(
                "GTK4 object {} already exists",
                id.get()
            )));
        }
        self.objects.insert(
            id,
            Gtk4NativeObject {
                id,
                kind: Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
                label: blueprint.label.clone(),
                value: blueprint.value.clone(),
                action: blueprint.action.clone(),
                control_state: blueprint.control_state.clone(),
                children: Vec::new(),
            },
        );
        Ok(())
    }

    fn update_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let object = self
            .objects
            .get_mut(&id)
            .ok_or_else(|| GuiError::host(format!("GTK4 object {} missing", id.get())))?;
        object.kind = Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        object.label = blueprint.label.clone();
        object.value = blueprint.value.clone();
        object.action = blueprint.action.clone();
        object.control_state = blueprint.control_state.clone();
        Ok(())
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.ensure_object(parent)?;
        self.ensure_object(child)?;
        if parent == child {
            return Err(GuiError::host(format!(
                "cannot insert GTK4 object {} into itself",
                child.get()
            )));
        }
        if self.subtree_contains(child, parent) {
            return Err(GuiError::host(format!(
                "inserting GTK4 object {} under {} would create a cycle",
                child.get(),
                parent.get()
            )));
        }

        for object in self.objects.values_mut() {
            object.children.retain(|existing| *existing != child);
        }
        let parent_object = self.objects.get_mut(&parent).ok_or_else(|| {
            GuiError::host(format!("GTK4 parent object {} missing", parent.get()))
        })?;
        let index = index.min(parent_object.children.len());
        parent_object.children.insert(index, child);
        Ok(())
    }

    fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        let removed_ids = self.subtree_ids(id);
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
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.root = Some(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::CommandExecutingHost;
    use crate::compiler::CompiledJsxNode;
    use crate::native::{NativeElement, NativeRole};
    use crate::platform::{Gtk4Adapter, PlatformAdapter};
    use crate::runtime::GuiRuntime;

    #[test]
    fn gtk4_widget_driver_reparents_children_and_removes_subtrees() {
        let mut driver = Gtk4WidgetDriver::default();
        let root = HostNodeId::new(1);
        let child = HostNodeId::new(2);
        let grandchild = HostNodeId::new(3);
        let second = HostNodeId::new(4);
        let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = Gtk4Adapter.blueprint(&NativeElement::new("button", NativeRole::Button));

        driver.create_widget(root, &container).unwrap();
        driver.create_widget(child, &container).unwrap();
        driver.create_widget(grandchild, &button).unwrap();
        driver.create_widget(second, &container).unwrap();
        driver.insert_child(root, child, 0).unwrap();
        driver.insert_child(child, grandchild, 0).unwrap();
        driver.insert_child(second, child, 0).unwrap();

        assert!(driver.object(root).unwrap().children.is_empty());
        assert_eq!(driver.object(second).unwrap().children, vec![child]);
        let error = driver.insert_child(child, second, 0).unwrap_err();
        assert!(error.to_string().contains("would create a cycle"));

        driver.set_root_widget(second).unwrap();
        driver.remove_widget(second).unwrap();

        assert!(driver.root().is_none());
        assert!(driver.object(root).is_some());
        assert!(driver.object(second).is_none());
        assert!(driver.object(child).is_none());
        assert!(driver.object(grandchild).is_none());
        assert_eq!(driver.objects().len(), 1);
    }

    #[test]
    fn gtk4_handle_adapter_clears_previous_parent_on_reparent() {
        let mut driver = Gtk4HandleDriver::default();
        let first = HostNodeId::new(1);
        let second = HostNodeId::new(2);
        let child = HostNodeId::new(3);
        let container = Gtk4Adapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = Gtk4Adapter.blueprint(&NativeElement::new("button", NativeRole::Button));

        driver.create_widget(first, &container).unwrap();
        driver.create_widget(second, &container).unwrap();
        driver.create_widget(child, &button).unwrap();
        driver.insert_child(first, child, 0).unwrap();
        driver.insert_child(second, child, 0).unwrap();

        assert_eq!(driver.children(first), Some([].as_slice()));
        assert_eq!(driver.children(second), Some([child].as_slice()));
        assert!(driver.handle(first).unwrap().state().children.is_empty());
        assert_eq!(driver.handle(second).unwrap().state().children, vec![child]);
    }

    #[test]
    fn gtk4_executor_consumes_compiled_react_aria_commands() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();

        assert_eq!(object.kind, Gtk4WidgetKind::Button);
        assert_eq!(object.label.as_deref(), Some("Save"));
        assert_eq!(object.action.as_deref(), Some("saveProfile"));
        assert!(object.control_state.disabled);
    }

    #[test]
    fn gtk4_executor_consumes_compiled_react_aria_toolbar_commands() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "tools",
              "tag": "Toolbar",
              "props": {"aria-orientation": "horizontal"},
              "children": [
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveDocument"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, Gtk4WidgetKind::ToolbarBox);
        assert_eq!(
            object.control_state.orientation,
            Some(crate::geometry::Orientation::Horizontal)
        );
        assert_eq!(child.kind, Gtk4WidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("saveDocument"));
    }

    #[test]
    fn gtk4_executor_consumes_compiled_react_aria_dialog_commands() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "preferences",
              "tag": "Dialog",
              "props": {"aria-label": "Preferences"},
              "children": [
                {
                  "kind": "element",
                  "key": "close",
                  "tag": "Button",
                  "props": {"events": {"onPress": "closePreferences"}},
                  "children": [{"kind": "text", "key": "close-text", "value": "Close"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, Gtk4WidgetKind::Dialog);
        assert_eq!(object.label.as_deref(), Some("Preferences"));
        assert_eq!(child.kind, Gtk4WidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("closePreferences"));
    }

    #[test]
    fn gtk4_executor_consumes_compiled_react_aria_popover_commands() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "actions-popover",
              "tag": "Popover",
              "props": {"aria-label": "Actions"},
              "children": [
                {
                  "kind": "element",
                  "key": "archive",
                  "tag": "Button",
                  "props": {"events": {"onPress": "archiveItem"}},
                  "children": [{"kind": "text", "key": "archive-text", "value": "Archive"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, Gtk4WidgetKind::Popover);
        assert_eq!(object.label.as_deref(), Some("Actions"));
        assert_eq!(child.kind, Gtk4WidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("archiveItem"));
    }

    #[test]
    fn gtk4_executor_consumes_compiled_react_aria_menu_commands() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "file-menu",
              "tag": "Menu",
              "children": [
                {
                  "kind": "element",
                  "key": "open",
                  "tag": "MenuItem",
                  "props": {"value": "open", "events": {"onPress": "openFile"}},
                  "children": [{"kind": "text", "key": "open-text", "value": "Open"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4CommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let item = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, Gtk4WidgetKind::Menu);
        assert_eq!(item.kind, Gtk4WidgetKind::MenuItem);
        assert_eq!(item.label.as_deref(), Some("Open"));
        assert_eq!(item.value.as_deref(), Some("open"));
        assert_eq!(item.action.as_deref(), Some("openFile"));
    }

    #[test]
    fn gtk4_handle_adapter_stores_thread_bound_native_handles() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4HandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();

        assert_eq!(state.kind, Gtk4WidgetKind::Button);
        assert_eq!(state.label.as_deref(), Some("Save"));
        assert_eq!(state.action.as_deref(), Some("saveProfile"));
        assert!(state.control_state.disabled);
        assert!(!state.config.enabled);
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetEnabled(false)));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetLabel(Some("Save".to_string()))));
    }

    #[test]
    fn gtk4_handle_adapter_applies_update_setters() {
        let first: CompiledJsxNode = serde_json::from_str(
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
        let second: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveProfile"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Saved"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(Gtk4Adapter, Gtk4HandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&first).unwrap();
        runtime.render_compiled(&second).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();

        assert_eq!(state.label.as_deref(), Some("Saved"));
        assert!(!state.config.enabled);
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetLabel(Some("Saved".to_string()))));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetEnabled(false)));
    }
}
