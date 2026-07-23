use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use gtk::prelude::*;
use gtk4_crate as gtk;

use crate::accessibility::relationship_registry::AccessibilityRelationshipRegistry;
use crate::accessibility::structure_registry::AccessibilityStructureRegistry;
use crate::accessibility::{AccessibilityAnnouncement, AccessibilityAnnouncementPriority};
use crate::app::{
    ActionPropagation, NativeRuntimeApp, NativeRuntimeEventBatch, NativeRuntimeEventResponse,
};
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{native_key_value, NativeEvent, NativeEventKind};
use crate::geometry::{Orientation, Rect, Size};
use crate::gtk4::Gtk4WidgetKind;
use crate::host::HostNodeId;
use crate::native_backends::gtk4::menu::Gtk4MenuRegistry;
pub use crate::native_backends::gtk4::menu::{Gtk4Menu, Gtk4MenuItem};
use crate::overlay_position::{
    OverlayCrossAlignment, OverlayPlacementAxis, OverlayPositionRequest,
};
use crate::platform::{
    apply_widget_setter, Gtk4Adapter, NativeBackendKind, NativeTextInputHints,
    NativeTextInputPurpose, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetSetter,
};
use crate::protocol::UiFrame;
use crate::selection::{CollectionKey, CollectionLayoutSnapshot};
use crate::style::{OverflowMode, StyleLength};

mod hierarchy;
mod interaction;
mod mount;
mod propagation;
mod relationships;
mod structure;
mod surface;
mod types;
mod update;

use surface::set_widget_title;
use types::*;
pub use types::{Gtk4DropDownItem, Gtk4NotebookTab, Gtk4OsHandle, Gtk4OsWidget};

pub type Gtk4NativeSurfaceAdapter = SurfaceHandleAdapter<Gtk4NativeSurface>;
pub type Gtk4NativeSurfaceDriver = HandleWidgetDriver<Gtk4NativeSurfaceAdapter>;
pub type Gtk4NativeSurfaceCommandExecutor = DriverCommandExecutor<Gtk4NativeSurfaceDriver>;
pub type Gtk4RuntimeHost = CommandExecutingHost<Gtk4Adapter, Gtk4NativeSurfaceCommandExecutor>;
pub type Gtk4RuntimeApp<S, F, R> = NativeRuntimeApp<Gtk4RuntimeHost, S, F, R>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gtk4EventWait {
    Poll,
    Wait,
}

#[derive(Debug)]
pub struct Gtk4NativeSurface {
    application: gtk::Application,
    root: Option<HostNodeId>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    events_suppressed: Rc<RefCell<bool>>,
    activation_contexts: Rc<RefCell<BTreeMap<HostNodeId, crate::input::NativeEventContext>>>,
    interaction_nodes: interaction::Gtk4InteractionNodes,
    keyboard_presses: interaction::Gtk4KeyboardPresses,
    closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
    dialog_visible: BTreeMap<HostNodeId, bool>,
    popover_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
    widgets: BTreeMap<HostNodeId, gtk::Widget>,
    accessibility_relationships: AccessibilityRelationshipRegistry,
    accessibility_structures: AccessibilityStructureRegistry,
    container_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    drop_downs: BTreeMap<HostNodeId, Gtk4DropDownState>,
    drop_down_items: BTreeMap<HostNodeId, Gtk4DropDownItem>,
    drop_down_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    drop_down_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    drop_down_selected_values: BTreeMap<HostNodeId, Option<String>>,
    drop_down_values: Rc<RefCell<BTreeMap<HostNodeId, Vec<String>>>>,
    list_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    list_values: Rc<RefCell<BTreeMap<HostNodeId, Vec<String>>>>,
    notebooks: BTreeMap<HostNodeId, gtk::Notebook>,
    notebook_tabs: BTreeMap<HostNodeId, Gtk4NotebookTab>,
    notebook_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    notebook_tab_parents: BTreeMap<HostNodeId, HostNodeId>,
    notebook_selected_values: BTreeMap<HostNodeId, Option<String>>,
    notebook_values: Rc<RefCell<BTreeMap<HostNodeId, Vec<String>>>>,
    menus: Gtk4MenuRegistry,
    ranges: BTreeMap<HostNodeId, Gtk4RangeState>,
    text_inputs: BTreeMap<HostNodeId, Gtk4TextInputSizing>,
    text_input_configs: BTreeMap<HostNodeId, NativeWidgetConfig>,
    text_input_max_lengths: Rc<RefCell<BTreeMap<HostNodeId, Option<u32>>>>,
}

impl Gtk4NativeSurface {
    pub fn new() -> GuiResult<Self> {
        Self::with_application_id("lab.a3s.gui")
    }

    pub fn with_application_id(application_id: &str) -> GuiResult<Self> {
        gtk::init().map_err(|error| {
            GuiError::host(format!("failed to initialize GTK4 native surface: {error}"))
        })?;
        let application = gtk::Application::builder()
            .application_id(application_id)
            .build();
        if !application.is_registered() {
            application
                .register(None::<&gtk::gio::Cancellable>)
                .map_err(|error| {
                    GuiError::host(format!("failed to register GTK4 application: {error}"))
                })?;
        }
        Ok(Self::with_application(application))
    }

    pub fn with_application(application: gtk::Application) -> Self {
        Self {
            application,
            root: None,
            events: Rc::new(RefCell::new(Vec::new())),
            events_suppressed: Rc::new(RefCell::new(false)),
            activation_contexts: Rc::new(RefCell::new(BTreeMap::new())),
            interaction_nodes: Rc::new(RefCell::new(BTreeMap::new())),
            keyboard_presses: Rc::new(RefCell::new(crate::event::KeyboardPressState::default())),
            closed_windows: Rc::new(RefCell::new(BTreeSet::new())),
            dialog_visible: BTreeMap::new(),
            popover_positions: BTreeMap::new(),
            widgets: BTreeMap::new(),
            accessibility_relationships: AccessibilityRelationshipRegistry::default(),
            accessibility_structures: AccessibilityStructureRegistry::default(),
            container_children: BTreeMap::new(),
            drop_downs: BTreeMap::new(),
            drop_down_items: BTreeMap::new(),
            drop_down_children: BTreeMap::new(),
            drop_down_item_parents: BTreeMap::new(),
            drop_down_selected_values: BTreeMap::new(),
            drop_down_values: Rc::new(RefCell::new(BTreeMap::new())),
            list_item_parents: BTreeMap::new(),
            list_values: Rc::new(RefCell::new(BTreeMap::new())),
            notebooks: BTreeMap::new(),
            notebook_tabs: BTreeMap::new(),
            notebook_children: BTreeMap::new(),
            notebook_tab_parents: BTreeMap::new(),
            notebook_selected_values: BTreeMap::new(),
            notebook_values: Rc::new(RefCell::new(BTreeMap::new())),
            menus: Gtk4MenuRegistry::default(),
            ranges: BTreeMap::new(),
            text_inputs: BTreeMap::new(),
            text_input_configs: BTreeMap::new(),
            text_input_max_lengths: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    pub fn application(&self) -> &gtk::Application {
        &self.application
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn root_window_open(&self) -> bool {
        self.root
            .map(|root| !self.closed_windows.borrow().contains(&root))
            .unwrap_or(false)
    }

    pub fn into_driver(self) -> Gtk4NativeSurfaceDriver {
        HandleWidgetDriver::new(SurfaceHandleAdapter::new(self))
    }

    pub fn into_executor(self) -> Gtk4NativeSurfaceCommandExecutor {
        DriverCommandExecutor::new(self.into_driver())
    }

    pub fn into_host(self) -> CommandExecutingHost<Gtk4Adapter, Gtk4NativeSurfaceCommandExecutor> {
        CommandExecutingHost::new(Gtk4Adapter, self.into_executor())
    }

    fn apply_entry_width_hint(&self, id: HostNodeId, entry: &gtk::Entry) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let width_chars = if sizing.has_explicit_width {
            -1
        } else {
            sizing.hinted_width_chars().unwrap_or(-1)
        };
        entry.set_width_chars(width_chars);
    }

    fn apply_search_entry_width_hint(&self, id: HostNodeId, entry: &gtk::SearchEntry) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let width_chars = if sizing.has_explicit_width {
            -1
        } else {
            sizing.hinted_width_chars().unwrap_or(-1)
        };
        entry.set_width_chars(width_chars);
    }

    fn apply_password_entry_width_hint(&self, id: HostNodeId, entry: &gtk::PasswordEntry) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let width_chars = if sizing.has_explicit_width {
            -1
        } else {
            sizing.hinted_width_chars().unwrap_or(-1)
        };
        entry.set_width_chars(width_chars);
    }

    fn apply_spin_button_width_hint(&self, id: HostNodeId, spin_button: &gtk::SpinButton) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let width_chars = if sizing.has_explicit_width {
            -1
        } else {
            sizing.hinted_width_chars().unwrap_or(-1)
        };
        spin_button.set_width_chars(width_chars);
    }

    fn apply_text_view_size_hint(&self, id: HostNodeId, text_view: &gtk::TextView) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let (width, height) =
            sizing.text_view_size_request(text_view.width_request(), text_view.height_request());
        text_view.set_size_request(width, height);
    }

    fn apply_text_input_hints(&self, id: HostNodeId, widget: &Gtk4OsWidget) {
        let Some(config) = self.text_input_configs.get(&id) else {
            return;
        };
        let purpose = gtk_input_purpose(config.text_input_purpose());
        let hints = gtk_input_hints(config.text_input_hints());
        match widget {
            Gtk4OsWidget::Entry(entry) => {
                entry.set_input_purpose(purpose);
                entry.set_input_hints(hints);
            }
            Gtk4OsWidget::TextView(text_view) => {
                text_view.set_input_purpose(purpose);
                text_view.set_input_hints(hints);
            }
            _ => {}
        }
    }

    fn suppress_events<T>(&self, apply: impl FnOnce() -> T) -> T {
        let previous = self.events_suppressed.replace(true);
        let result = apply();
        self.events_suppressed.replace(previous);
        result
    }

    fn update_drop_down_item_label(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4DropDownItem,
        label: String,
    ) -> GuiResult<()> {
        let item = self
            .drop_down_items
            .entry(id)
            .or_insert_with(|| fallback.clone());
        if item.value == item.label {
            item.value = label.clone();
        }
        item.label = label;
        self.rebuild_drop_down_for_item(id)?;
        self.sync_list_values_for_item(id);
        Ok(())
    }

    fn update_drop_down_item_value(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4DropDownItem,
        value: String,
    ) -> GuiResult<()> {
        self.drop_down_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
        self.rebuild_drop_down_for_item(id)?;
        self.sync_list_values_for_item(id);
        Ok(())
    }

    fn update_drop_down_item_selected(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4DropDownItem,
        selected: bool,
    ) -> GuiResult<()> {
        self.drop_down_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
        self.rebuild_drop_down_for_item(id)
    }

    fn rebuild_drop_down_for_item(&mut self, item: HostNodeId) -> GuiResult<()> {
        if let Some(parent) = self.drop_down_item_parents.get(&item).copied() {
            self.rebuild_drop_down(parent)?;
        }
        Ok(())
    }

    fn sync_list_values_for_item(&self, item: HostNodeId) {
        if let Some(parent) = self.list_item_parents.get(&item).copied() {
            self.sync_list_values(parent);
        }
    }

    fn sync_list_values(&self, list: HostNodeId) {
        let values = self
            .container_children
            .get(&list)
            .into_iter()
            .flatten()
            .filter_map(|child| self.drop_down_items.get(child))
            .map(|item| item.value.clone())
            .collect();
        self.list_values.borrow_mut().insert(list, values);
    }

    fn update_list_item_selected(&self, item: HostNodeId, row: &gtk::ListBoxRow, selected: bool) {
        let Some(parent) = self.list_item_parents.get(&item) else {
            return;
        };
        let Some(list_box) = self
            .widgets
            .get(parent)
            .and_then(|widget| widget.downcast_ref::<gtk::ListBox>())
        else {
            return;
        };
        self.suppress_events(|| {
            if selected {
                list_box.select_row(Some(row));
            } else {
                list_box.unselect_row(row);
            }
        });
    }

    fn rebuild_drop_down(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(state) = self.drop_downs.get(&id).cloned() else {
            return Ok(());
        };
        let previous_value = self.selected_drop_down_value(id, &state.drop_down);
        let children = self
            .drop_down_children
            .get(&id)
            .cloned()
            .unwrap_or_default();

        let mut labels = Vec::new();
        let mut values = Vec::new();
        let mut selected_value = None;
        for child in children {
            let Some(item) = self.drop_down_items.get(&child) else {
                continue;
            };
            labels.push(item.label.clone());
            values.push(item.value.clone());
            if item.selected && selected_value.is_none() {
                selected_value = Some(item.value.clone());
            }
        }

        let selected_value = selected_value
            .or_else(|| self.drop_down_selected_values.get(&id).cloned().flatten())
            .or_else(|| (!previous_value.is_empty()).then_some(previous_value));
        let selected_index = selected_value
            .as_ref()
            .and_then(|value| values.iter().position(|item_value| item_value == value))
            .map(|index| index as u32)
            .unwrap_or(gtk::INVALID_LIST_POSITION);

        self.drop_down_values
            .borrow_mut()
            .insert(id, values.clone());
        let label_refs = labels.iter().map(String::as_str).collect::<Vec<_>>();
        self.suppress_events(|| {
            state
                .model
                .splice(0, state.model.n_items(), label_refs.as_slice());
            state.drop_down.set_selected(selected_index);
        });
        Ok(())
    }

    fn set_drop_down_value(&mut self, id: HostNodeId, value: Option<&str>) {
        self.drop_down_selected_values
            .insert(id, value.map(str::to_string));
        let Some(state) = self.drop_downs.get(&id) else {
            return;
        };
        let selected_index = value
            .and_then(|value| {
                self.drop_down_values
                    .borrow()
                    .get(&id)
                    .and_then(|values| values.iter().position(|item_value| item_value == value))
            })
            .map(|index| index as u32)
            .unwrap_or(gtk::INVALID_LIST_POSITION);
        self.suppress_events(|| state.drop_down.set_selected(selected_index));
    }

    fn selected_drop_down_value(&self, id: HostNodeId, drop_down: &gtk::DropDown) -> String {
        let selected = drop_down.selected();
        if selected == gtk::INVALID_LIST_POSITION {
            return String::new();
        }
        self.drop_down_values
            .borrow()
            .get(&id)
            .and_then(|values| values.get(selected as usize).cloned())
            .unwrap_or_default()
    }

    fn update_notebook_tab_label(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4NotebookTab,
        label: String,
    ) -> GuiResult<()> {
        let tab = self
            .notebook_tabs
            .entry(id)
            .or_insert_with(|| fallback.clone());
        if tab.value == tab.label {
            tab.value = label.clone();
        }
        tab.label = label;
        self.rebuild_notebook_for_tab(id)
    }

    fn update_notebook_tab_value(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4NotebookTab,
        value: String,
    ) -> GuiResult<()> {
        self.notebook_tabs
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
        self.rebuild_notebook_for_tab(id)
    }

    fn update_notebook_tab_selected(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4NotebookTab,
        selected: bool,
    ) -> GuiResult<()> {
        self.notebook_tabs
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
        self.rebuild_notebook_for_tab(id)
    }

    fn update_notebook_tab_panel(
        &mut self,
        tab: HostNodeId,
        panel: Option<HostNodeId>,
    ) -> GuiResult<()> {
        self.notebook_tabs
            .entry(tab)
            .or_insert_with(|| Gtk4NotebookTab::fallback(tab))
            .panel = panel;
        self.rebuild_notebook_for_tab(tab)
    }

    fn rebuild_notebook_for_tab(&mut self, tab: HostNodeId) -> GuiResult<()> {
        if let Some(parent) = self.notebook_tab_parents.get(&tab).copied() {
            self.rebuild_notebook(parent)?;
        }
        Ok(())
    }

    fn rebuild_notebook(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(notebook) = self.notebooks.get(&id).cloned() else {
            return Ok(());
        };
        let previous_page = notebook.current_page();
        let children = self.notebook_children.get(&id).cloned().unwrap_or_default();
        let mut pages = Vec::with_capacity(children.len());
        let mut values = Vec::with_capacity(children.len());
        let mut selected_value = None;

        for tab_id in children {
            let tab = self
                .notebook_tabs
                .get(&tab_id)
                .cloned()
                .unwrap_or_else(|| Gtk4NotebookTab::fallback(tab_id));
            let page = tab
                .panel
                .and_then(|panel| self.widgets.get(&panel).cloned())
                .unwrap_or_else(|| gtk::Box::new(gtk::Orientation::Vertical, 0).upcast());
            let label = self
                .widgets
                .get(&tab_id)
                .cloned()
                .unwrap_or_else(|| gtk::Label::new(Some(&tab.label)).upcast());
            if tab.selected && selected_value.is_none() {
                selected_value = Some(tab.value.clone());
            }
            values.push(tab.value);
            pages.push((page, label));
        }

        let selected_value =
            selected_value.or_else(|| self.notebook_selected_values.get(&id).cloned().flatten());
        let selected_index = selected_value
            .as_ref()
            .and_then(|value| values.iter().position(|item_value| item_value == value))
            .map(|index| index as u32)
            .or_else(|| previous_page.filter(|index| (*index as usize) < values.len()));

        self.notebook_values.borrow_mut().insert(id, values);
        self.suppress_events(|| {
            while notebook.n_pages() > 0 {
                notebook.remove_page(Some(0));
            }
            for (index, (page, label)) in pages.iter().enumerate() {
                notebook.insert_page(page, Some(label), Some(index as u32));
            }
            if let Some(selected_index) = selected_index {
                notebook.set_current_page(Some(selected_index));
            }
        });
        Ok(())
    }

    fn set_notebook_value(&mut self, id: HostNodeId, value: Option<&str>) {
        self.notebook_selected_values
            .insert(id, value.map(str::to_string));
        let Some(value) = value else {
            return;
        };
        let Some(notebook) = self.notebooks.get(&id) else {
            return;
        };
        let selected_index = self
            .notebook_values
            .borrow()
            .get(&id)
            .and_then(|values| values.iter().position(|item_value| item_value == value))
            .map(|index| index as u32);
        if let Some(selected_index) = selected_index {
            self.suppress_events(|| notebook.set_current_page(Some(selected_index)));
        }
    }

    fn insert_box_child(
        &mut self,
        parent: HostNodeId,
        box_: &gtk::Box,
        child: HostNodeId,
        child_widget: &gtk::Widget,
        index: usize,
    ) {
        let children = self.container_children.entry(parent).or_default();
        children.retain(|existing| *existing != child);
        let index = index.min(children.len());
        let previous_sibling = index
            .checked_sub(1)
            .and_then(|previous_index| children.get(previous_index))
            .and_then(|previous_child| self.widgets.get(previous_child));
        box_.insert_child_after(child_widget, previous_sibling);
        children.insert(index, child);
    }
}

impl Gtk4EventWait {
    fn may_block(self) -> bool {
        matches!(self, Self::Wait)
    }
}

fn gtk4_runtime_root_window_open<S, F, R>(
    app: &NativeRuntimeApp<Gtk4RuntimeHost, S, F, R>,
) -> bool {
    app.runtime()
        .host()
        .executor()
        .driver()
        .adapter()
        .surface()
        .root_window_open()
}

fn pump_gtk4_os_event(wait: Gtk4EventWait) -> bool {
    let context = gtk::glib::MainContext::default();
    if wait.may_block() || context.pending() {
        context.iteration(wait.may_block());
        true
    } else {
        false
    }
}

impl<S, F, R> NativeRuntimeApp<Gtk4RuntimeHost, S, F, R> {
    pub fn gtk4_root_window_open(&self) -> bool {
        gtk4_runtime_root_window_open(self)
    }
}

impl<S, F, R> NativeRuntimeApp<Gtk4RuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<()>,
{
    pub fn gtk4(state: S, frame_builder: F, action_reducer: R) -> GuiResult<Self> {
        Self::gtk4_with_application_id("lab.a3s.gui", state, frame_builder, action_reducer)
    }

    pub fn gtk4_with_application_id(
        application_id: &str,
        state: S,
        frame_builder: F,
        action_reducer: R,
    ) -> GuiResult<Self> {
        Ok(Self::new(
            Gtk4NativeSurface::with_application_id(application_id)?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_gtk4_event(
        &mut self,
        wait: Gtk4EventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_gtk4_event_batch(wait)
            .map(|batch| batch.responses)
    }

    pub fn pump_gtk4_event_while(
        &mut self,
        wait: Gtk4EventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_gtk4_event_batch_while(wait, &mut should_continue)
            .map(|batch| batch.responses)
    }

    pub fn pump_gtk4_event_batch(
        &mut self,
        wait: Gtk4EventWait,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        self.pump_gtk4_event_batch_while(wait, |_| true)
    }

    pub fn pump_gtk4_event_batch_while(
        &mut self,
        wait: Gtk4EventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        let mut batch = self.handle_pending_native_event_batch_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(batch);
        }
        if pump_gtk4_os_event(wait) {
            batch.extend(self.handle_pending_native_event_batch_while(&mut should_continue)?);
        }
        Ok(batch)
    }

    pub fn run_gtk4(&mut self) -> GuiResult<()> {
        self.run_gtk4_while(|_| true)
    }

    pub fn run_gtk4_while(&mut self, mut should_continue: impl FnMut(&S) -> bool) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.gtk4_root_window_open() && should_continue(self.state()) {
            self.poll_background_updates()?;
            if !self.gtk4_root_window_open() || !should_continue(self.state()) {
                break;
            }
            let wait = if self.has_pending_background_work() {
                Gtk4EventWait::Poll
            } else {
                Gtk4EventWait::Wait
            };
            self.pump_gtk4_event_while(wait, &mut should_continue)?;
            if self.has_pending_background_work() {
                std::thread::park_timeout(std::time::Duration::from_millis(16));
            }
        }
        Ok(())
    }
}
