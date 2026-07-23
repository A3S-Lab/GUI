use std::cell::{Cell, Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::backend::{
    DriverCommandExecutor, HandleWidgetDriver, NativeEventSource, NativeHandleAdapter,
    NativeWidgetDriver,
};
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
use crate::platform::{
    apply_widget_setters, push_widget_setter_history, NativeBackendKind, NativeControlState,
    NativeTextInputKind, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetConfigPatch,
    NativeWidgetKind, NativeWidgetSetter, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
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
    ScrolledWindow,
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
    pub fn from_widget_kind(kind: NativeWidgetKind) -> Self {
        match kind {
            NativeWidgetKind::Window => Self::ApplicationWindow,
            NativeWidgetKind::Container(_)
            | NativeWidgetKind::RadioGroup
            | NativeWidgetKind::Tree
            | NativeWidgetKind::TreeItem
            | NativeWidgetKind::Table
            | NativeWidgetKind::Image
            | NativeWidgetKind::Media => Self::Box,
            NativeWidgetKind::ScrollContainer => Self::ScrolledWindow,
            NativeWidgetKind::Label | NativeWidgetKind::Tab => Self::Label,
            NativeWidgetKind::Button => Self::Button,
            NativeWidgetKind::TextInput(NativeTextInputKind::Number) => Self::SpinButton,
            NativeWidgetKind::TextInput(NativeTextInputKind::Multiline) => Self::TextView,
            NativeWidgetKind::TextInput(_) => Self::Entry,
            NativeWidgetKind::Checkbox | NativeWidgetKind::Radio => Self::CheckButton,
            NativeWidgetKind::Switch => Self::Switch,
            NativeWidgetKind::ComboBox => Self::DropDown,
            NativeWidgetKind::List => Self::ListBox,
            NativeWidgetKind::ListItem => Self::ListBoxRow,
            NativeWidgetKind::Dialog => Self::Dialog,
            NativeWidgetKind::Popover => Self::Popover,
            NativeWidgetKind::Tabs => Self::Notebook,
            NativeWidgetKind::Menu => Self::Menu,
            NativeWidgetKind::MenuItem => Self::MenuItem,
            NativeWidgetKind::Separator => Self::Separator,
            NativeWidgetKind::Slider => Self::Scale,
            NativeWidgetKind::Progress => Self::ProgressBar,
            NativeWidgetKind::Toolbar => Self::ToolbarBox,
        }
    }

    /// Legacy class-name compatibility. Runtime drivers use `from_widget_kind`.
    #[deprecated(note = "use Gtk4WidgetKind::from_widget_kind with typed NativeWidgetKind")]
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
            "gtk::ListBox" | "gtk::TreeListModel" => Ok(Gtk4WidgetKind::ListBox),
            "gtk::ScrolledWindow+Box" => Ok(Gtk4WidgetKind::ScrolledWindow),
            "gtk::ListBoxRow" | "gtk::TreeExpander" => Ok(Gtk4WidgetKind::ListBoxRow),
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

#[derive(Clone, PartialEq)]
pub struct Gtk4NativeObject {
    pub id: HostNodeId,
    pub kind: Gtk4WidgetKind,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

impl std::fmt::Debug for Gtk4NativeObject {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Gtk4NativeObject")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("label", &self.label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct Gtk4WidgetDriver {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
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

    pub fn take_applied_setters(&self) -> Vec<NativeWidgetSetter> {
        std::mem::take(&mut self.state.borrow_mut().applied_setters)
    }
}

#[derive(Clone, PartialEq)]
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

impl std::fmt::Debug for Gtk4NativeHandleState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Gtk4NativeHandleState")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("config", &self.config)
            .field("label", &self.label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("applied_setters", &self.applied_setters)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct Gtk4HandleAdapter {
    focused: Rc<Cell<Option<HostNodeId>>>,
    overlay_positions: Rc<RefCell<BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>>>,
}

impl Gtk4HandleAdapter {
    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused.get()
    }

    pub fn overlay_positions(
        &self,
    ) -> Ref<'_, BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>> {
        self.overlay_positions.borrow()
    }
}

pub type Gtk4HandleDriver = HandleWidgetDriver<Gtk4HandleAdapter>;
pub type Gtk4HandleCommandExecutor = DriverCommandExecutor<Gtk4HandleDriver>;

impl Gtk4WidgetDriver {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
    }

    pub fn overlay_positions(&self) -> &BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)> {
        &self.overlay_positions
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
        let setters = config.create_setters();
        let mut applied_setters = Vec::new();
        push_widget_setter_history(
            &mut applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
        Ok(Gtk4NativeHandle {
            state: Rc::new(RefCell::new(Gtk4NativeHandleState {
                id,
                kind: Gtk4WidgetKind::from_widget_kind(blueprint.widget_kind),
                label: config.label.clone(),
                value: config.value.clone(),
                action: config.action.clone(),
                applied_setters,
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
        state.kind = Gtk4WidgetKind::from_widget_kind(blueprint.widget_kind);
        state.config = blueprint.config();
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        let setters = state.config.create_setters();
        push_widget_setter_history(
            &mut state.applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
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
        state.kind = Gtk4WidgetKind::from_widget_kind(blueprint.widget_kind);
        let setters = patch.setters();
        apply_widget_setters(&mut state.config, &setters);
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        state.control_state = blueprint.control_state.clone();
        push_widget_setter_history(
            &mut state.applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
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

    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        handle.state.borrow_mut().children.clear();
        if self.focused.get() == Some(id) {
            self.focused.set(None);
        }
        self.overlay_positions
            .borrow_mut()
            .retain(|overlay, (anchor, _)| *overlay != id && *anchor != id);
        Ok(())
    }

    fn set_root_handle(&mut self, _id: HostNodeId, _handle: &Self::Handle) -> GuiResult<()> {
        Ok(())
    }

    fn request_focus_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        if handle.state.borrow().id != id {
            return Err(GuiError::host(format!(
                "GTK4 handle id does not match focus target {}",
                id.get()
            )));
        }
        self.focused.set(Some(id));
        Ok(())
    }

    fn position_overlay_handle(
        &mut self,
        overlay: HostNodeId,
        overlay_handle: &Self::Handle,
        anchor: HostNodeId,
        anchor_handle: &Self::Handle,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        if overlay_handle.state.borrow().id != overlay || anchor_handle.state.borrow().id != anchor
        {
            return Err(GuiError::host("GTK4 overlay or anchor handle id mismatch"));
        }
        if overlay_handle.state.borrow().kind != Gtk4WidgetKind::Popover {
            return Err(GuiError::host(format!(
                "GTK4 object {} is not a gtk::Popover",
                overlay.get()
            )));
        }
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        self.overlay_positions
            .borrow_mut()
            .insert(overlay, (anchor, request));
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
                kind: Gtk4WidgetKind::from_widget_kind(blueprint.widget_kind),
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
        object.kind = Gtk4WidgetKind::from_widget_kind(blueprint.widget_kind);
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
        self.overlay_positions.retain(|overlay, (anchor, _)| {
            !removed_ids.contains(overlay) && !removed_ids.contains(anchor)
        });
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
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.root = Some(id);
        Ok(())
    }

    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.focused = Some(id);
        Ok(())
    }

    fn position_overlay(
        &mut self,
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        self.ensure_object(overlay)?;
        self.ensure_object(anchor)?;
        if self.objects.get(&overlay).map(|object| object.kind) != Some(Gtk4WidgetKind::Popover) {
            return Err(GuiError::host(format!(
                "GTK4 object {} is not a gtk::Popover",
                overlay.get()
            )));
        }
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        self.overlay_positions.insert(overlay, (anchor, request));
        Ok(())
    }
}

#[cfg(test)]
mod tests;
