use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use gtk::prelude::*;
use gtk4_crate as gtk;

use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation;
use crate::gtk4::Gtk4WidgetKind;
use crate::host::HostNodeId;
use crate::native_backends::gtk4::menu::{Gtk4Menu, Gtk4MenuItem, Gtk4MenuRegistry};
use crate::platform::{
    Gtk4Adapter, NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetSetter,
};
use crate::style::StyleLength;

pub type Gtk4NativeSurfaceAdapter = SurfaceHandleAdapter<Gtk4NativeSurface>;
pub type Gtk4NativeSurfaceDriver = HandleWidgetDriver<Gtk4NativeSurfaceAdapter>;
pub type Gtk4NativeSurfaceCommandExecutor = DriverCommandExecutor<Gtk4NativeSurfaceDriver>;

#[derive(Debug)]
pub struct Gtk4NativeSurface {
    application: gtk::Application,
    root: Option<HostNodeId>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    events_suppressed: Rc<RefCell<bool>>,
    widgets: BTreeMap<HostNodeId, gtk::Widget>,
    container_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    drop_downs: BTreeMap<HostNodeId, Gtk4DropDownState>,
    drop_down_items: BTreeMap<HostNodeId, Gtk4DropDownItem>,
    drop_down_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    drop_down_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    drop_down_selected_values: BTreeMap<HostNodeId, Option<String>>,
    drop_down_values: Rc<RefCell<BTreeMap<HostNodeId, Vec<String>>>>,
    notebooks: BTreeMap<HostNodeId, gtk::Notebook>,
    notebook_tabs: BTreeMap<HostNodeId, Gtk4NotebookTab>,
    notebook_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    notebook_tab_parents: BTreeMap<HostNodeId, HostNodeId>,
    notebook_selected_values: BTreeMap<HostNodeId, Option<String>>,
    notebook_values: Rc<RefCell<BTreeMap<HostNodeId, Vec<String>>>>,
    menus: Gtk4MenuRegistry,
    ranges: BTreeMap<HostNodeId, Gtk4RangeState>,
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
        Ok(Self::with_application(application))
    }

    pub fn with_application(application: gtk::Application) -> Self {
        Self {
            application,
            root: None,
            events: Rc::new(RefCell::new(Vec::new())),
            events_suppressed: Rc::new(RefCell::new(false)),
            widgets: BTreeMap::new(),
            container_children: BTreeMap::new(),
            drop_downs: BTreeMap::new(),
            drop_down_items: BTreeMap::new(),
            drop_down_children: BTreeMap::new(),
            drop_down_item_parents: BTreeMap::new(),
            drop_down_selected_values: BTreeMap::new(),
            drop_down_values: Rc::new(RefCell::new(BTreeMap::new())),
            notebooks: BTreeMap::new(),
            notebook_tabs: BTreeMap::new(),
            notebook_children: BTreeMap::new(),
            notebook_tab_parents: BTreeMap::new(),
            notebook_selected_values: BTreeMap::new(),
            notebook_values: Rc::new(RefCell::new(BTreeMap::new())),
            menus: Gtk4MenuRegistry::default(),
            ranges: BTreeMap::new(),
        }
    }

    pub fn application(&self) -> &gtk::Application {
        &self.application
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
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
        self.rebuild_drop_down_for_item(id)
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
        self.rebuild_drop_down_for_item(id)
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

#[derive(Debug, Clone)]
struct Gtk4DropDownState {
    drop_down: gtk::DropDown,
    model: gtk::StringList,
}

#[derive(Debug, Clone, Copy, Default)]
struct Gtk4RangeState {
    min: Option<f64>,
    max: Option<f64>,
    current: Option<f64>,
}

impl Gtk4RangeState {
    fn from_config(config: &NativeWidgetConfig) -> Self {
        Self {
            min: config.min,
            max: config.max,
            current: config.current,
        }
    }

    fn lower(self) -> f64 {
        self.min.unwrap_or(0.0)
    }

    fn upper(self) -> f64 {
        self.max.unwrap_or(100.0)
    }

    fn current(self) -> f64 {
        self.current.unwrap_or_else(|| self.lower())
    }
}

#[derive(Debug, Clone)]
pub struct Gtk4OsHandle {
    pub id: HostNodeId,
    pub kind: Gtk4WidgetKind,
    pub widget: Gtk4OsWidget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtk4DropDownItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl Gtk4DropDownItem {
    fn from_config(config: &NativeWidgetConfig) -> Self {
        let label = config
            .label
            .clone()
            .or_else(|| config.value.clone())
            .unwrap_or_default();
        let value = config.value.clone().unwrap_or_else(|| label.clone());
        Self {
            label,
            value,
            selected: config.selected,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtk4NotebookTab {
    pub label: String,
    pub value: String,
    pub selected: bool,
    pub panel: Option<HostNodeId>,
}

impl Gtk4NotebookTab {
    fn from_config(id: HostNodeId, config: &NativeWidgetConfig) -> Self {
        let label = config
            .label
            .clone()
            .or_else(|| config.value.clone())
            .unwrap_or_else(|| id.get().to_string());
        let value = config.value.clone().unwrap_or_else(|| label.clone());
        Self {
            label,
            value,
            selected: config.selected,
            panel: None,
        }
    }

    fn fallback(id: HostNodeId) -> Self {
        let label = id.get().to_string();
        Self {
            label: label.clone(),
            value: label,
            selected: false,
            panel: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Gtk4OsWidget {
    ApplicationWindow(gtk::ApplicationWindow),
    Box(gtk::Box),
    Label(gtk::Label),
    Button(gtk::Button),
    Entry(gtk::Entry),
    CheckButton(gtk::CheckButton),
    Switch(gtk::Switch),
    DropDown(gtk::DropDown),
    ListBox(gtk::ListBox),
    ListBoxRow {
        row: gtk::ListBoxRow,
        label: gtk::Label,
        item: Gtk4DropDownItem,
    },
    Dialog(gtk::Dialog),
    Popover(gtk::Popover),
    Menu(Gtk4Menu),
    MenuItem(Gtk4MenuItem),
    Notebook(gtk::Notebook),
    Separator(gtk::Separator),
    Scale(gtk::Scale),
    ProgressBar(gtk::ProgressBar),
}

impl Gtk4OsWidget {
    fn as_widget(&self) -> Option<gtk::Widget> {
        match self {
            Gtk4OsWidget::ApplicationWindow(window) => Some(window.clone().upcast()),
            Gtk4OsWidget::Box(box_) => Some(box_.clone().upcast()),
            Gtk4OsWidget::Label(label) => Some(label.clone().upcast()),
            Gtk4OsWidget::Button(button) => Some(button.clone().upcast()),
            Gtk4OsWidget::Entry(entry) => Some(entry.clone().upcast()),
            Gtk4OsWidget::CheckButton(check_button) => Some(check_button.clone().upcast()),
            Gtk4OsWidget::Switch(switch) => Some(switch.clone().upcast()),
            Gtk4OsWidget::DropDown(drop_down) => Some(drop_down.clone().upcast()),
            Gtk4OsWidget::ListBox(list_box) => Some(list_box.clone().upcast()),
            Gtk4OsWidget::ListBoxRow { row, .. } => Some(row.clone().upcast()),
            Gtk4OsWidget::Dialog(dialog) => Some(dialog.clone().upcast()),
            Gtk4OsWidget::Popover(popover) => Some(popover.clone().upcast()),
            Gtk4OsWidget::Menu(menu) => Some(menu.bar.clone().upcast()),
            Gtk4OsWidget::Notebook(notebook) => Some(notebook.clone().upcast()),
            Gtk4OsWidget::Separator(separator) => Some(separator.clone().upcast()),
            Gtk4OsWidget::Scale(scale) => Some(scale.clone().upcast()),
            Gtk4OsWidget::ProgressBar(progress_bar) => Some(progress_bar.clone().upcast()),
            Gtk4OsWidget::MenuItem(_) => None,
        }
    }
}

impl NativeWidgetSurface for Gtk4NativeSurface {
    type Handle = Gtk4OsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let kind = Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let config = blueprint.config();
        let widget = match kind {
            Gtk4WidgetKind::ApplicationWindow => {
                let window = gtk::ApplicationWindow::builder()
                    .application(&self.application)
                    .title(config.label.as_deref().unwrap_or(""))
                    .default_width(config_dimension(config.portable_style.width, 640))
                    .default_height(config_dimension(config.portable_style.height, 480))
                    .build();
                Gtk4OsWidget::ApplicationWindow(window)
            }
            Gtk4WidgetKind::Box | Gtk4WidgetKind::ToolbarBox => {
                let box_ = gtk::Box::new(
                    config_orientation(&config).unwrap_or(gtk::Orientation::Vertical),
                    config
                        .portable_style
                        .gap
                        .and_then(StyleLength::points)
                        .map(points_to_i32)
                        .unwrap_or(0),
                );
                Gtk4OsWidget::Box(box_)
            }
            Gtk4WidgetKind::Label => Gtk4OsWidget::Label(gtk::Label::new(Some(
                config
                    .label
                    .as_deref()
                    .or(config.value.as_deref())
                    .unwrap_or(""),
            ))),
            Gtk4WidgetKind::Button => {
                let button = gtk::Button::with_label(config.label.as_deref().unwrap_or(""));
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                button.connect_clicked(move |_| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Press),
                    );
                });
                Gtk4OsWidget::Button(button)
            }
            Gtk4WidgetKind::Entry => {
                let entry = gtk::Entry::new();
                self.suppress_events(|| {
                    entry.set_text(config.value.as_deref().unwrap_or(""));
                });
                if let Some(placeholder) = config.placeholder.as_deref() {
                    entry.set_placeholder_text(Some(placeholder));
                }

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                entry.connect_changed(move |entry| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Change)
                            .value(entry.text().to_string()),
                    );
                });

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                entry.connect_has_focus_notify(move |entry| {
                    let kind = if entry.has_focus() {
                        NativeEventKind::Focus
                    } else {
                        NativeEventKind::Blur
                    };
                    push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                });

                Gtk4OsWidget::Entry(entry)
            }
            Gtk4WidgetKind::CheckButton => {
                let check_button =
                    gtk::CheckButton::with_label(config.label.as_deref().unwrap_or(""));
                self.suppress_events(|| {
                    check_button.set_active(config.checked.unwrap_or(false));
                });
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                check_button.connect_toggled(move |check_button| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Toggle)
                            .value(check_button.is_active().to_string()),
                    );
                });
                Gtk4OsWidget::CheckButton(check_button)
            }
            Gtk4WidgetKind::Switch => {
                let switch = gtk::Switch::new();
                self.suppress_events(|| {
                    switch.set_active(config.checked.unwrap_or(false));
                });
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                switch.connect_active_notify(move |switch| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Toggle)
                            .value(switch.is_active().to_string()),
                    );
                });
                Gtk4OsWidget::Switch(switch)
            }
            Gtk4WidgetKind::DropDown => {
                let model = gtk::StringList::new(&[]);
                let drop_down = gtk::DropDown::from_strings(&[]);
                drop_down.set_model(Some(&model));
                if let Some(value) = config.value.clone() {
                    self.drop_down_selected_values.insert(id, Some(value));
                }
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let drop_down_values = self.drop_down_values.clone();
                drop_down.connect_selected_notify(move |drop_down| {
                    let selected = drop_down.selected();
                    let value = if selected == gtk::INVALID_LIST_POSITION {
                        String::new()
                    } else {
                        drop_down_values
                            .borrow()
                            .get(&id)
                            .and_then(|values| values.get(selected as usize).cloned())
                            .unwrap_or_default()
                    };
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
                    );
                });
                self.drop_downs.insert(
                    id,
                    Gtk4DropDownState {
                        drop_down: drop_down.clone(),
                        model,
                    },
                );
                self.drop_down_children.entry(id).or_default();
                Gtk4OsWidget::DropDown(drop_down)
            }
            Gtk4WidgetKind::ListBox => Gtk4OsWidget::ListBox(gtk::ListBox::new()),
            Gtk4WidgetKind::Dialog => {
                let dialog = gtk::Dialog::builder()
                    .application(&self.application)
                    .title(config.label.as_deref().unwrap_or(""))
                    .default_width(config_dimension(config.portable_style.width, 420))
                    .default_height(config_dimension(config.portable_style.height, 280))
                    .build();
                Gtk4OsWidget::Dialog(dialog)
            }
            Gtk4WidgetKind::Popover => {
                let popover = gtk::Popover::builder()
                    .autohide(true)
                    .has_arrow(true)
                    .build();
                Gtk4OsWidget::Popover(popover)
            }
            Gtk4WidgetKind::Menu => {
                let menu = Gtk4Menu::new();
                self.menus.register_menu(id, menu.clone());
                Gtk4OsWidget::Menu(menu)
            }
            Gtk4WidgetKind::MenuItem => {
                let item = Gtk4MenuItem::from_config(
                    id,
                    &config,
                    &self.application,
                    self.events.clone(),
                    self.events_suppressed.clone(),
                );
                self.menus.register_item(id, item.clone());
                Gtk4OsWidget::MenuItem(item)
            }
            Gtk4WidgetKind::ListBoxRow => {
                let item = Gtk4DropDownItem::from_config(&config);
                let label = gtk::Label::new(Some(&item.label));
                let row = gtk::ListBoxRow::new();
                row.set_child(Some(&label));
                self.drop_down_items.insert(id, item.clone());
                Gtk4OsWidget::ListBoxRow { row, label, item }
            }
            Gtk4WidgetKind::Notebook => {
                let notebook = gtk::Notebook::new();
                if let Some(value) = config.value.clone() {
                    self.notebook_selected_values.insert(id, Some(value));
                }
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let notebook_values = self.notebook_values.clone();
                notebook.connect_switch_page(move |_, _, page_num| {
                    let value = notebook_values
                        .borrow()
                        .get(&id)
                        .and_then(|values| values.get(page_num as usize).cloned())
                        .unwrap_or_else(|| page_num.to_string());
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
                    );
                });
                self.notebooks.insert(id, notebook.clone());
                self.notebook_children.entry(id).or_default();
                Gtk4OsWidget::Notebook(notebook)
            }
            Gtk4WidgetKind::Separator => Gtk4OsWidget::Separator(gtk::Separator::new(
                config_orientation(&config).unwrap_or(gtk::Orientation::Horizontal),
            )),
            Gtk4WidgetKind::Scale => {
                let range = Gtk4RangeState::from_config(&config);
                let min = range.lower();
                let max = range.upper();
                let scale = gtk::Scale::with_range(
                    config_orientation(&config).unwrap_or(gtk::Orientation::Horizontal),
                    min,
                    max,
                    1.0,
                );
                scale.set_value(range.current());
                self.ranges.insert(id, range);
                Gtk4OsWidget::Scale(scale)
            }
            Gtk4WidgetKind::ProgressBar => {
                let range = Gtk4RangeState::from_config(&config);
                let progress_bar = gtk::ProgressBar::new();
                set_progress_bar_fraction(&progress_bar, range);
                self.ranges.insert(id, range);
                Gtk4OsWidget::ProgressBar(progress_bar)
            }
            other => {
                return Err(GuiError::host(format!(
                    "GTK4 native surface does not support {other:?} yet"
                )));
            }
        };

        if kind == Gtk4WidgetKind::Label && blueprint.widget_class == "gtk::Label(tab)" {
            self.notebook_tabs
                .insert(id, Gtk4NotebookTab::from_config(id, &config));
        }

        let handle = Gtk4OsHandle { id, kind, widget };
        if let Some(widget) = handle.widget.as_widget() {
            self.widgets.insert(id, widget);
        }
        Ok(handle)
    }

    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        match setter {
            NativeWidgetSetter::SetLabel(value) => {
                let label = value.as_deref().unwrap_or("");
                match &handle.widget {
                    Gtk4OsWidget::ApplicationWindow(window) => window.set_title(Some(label)),
                    Gtk4OsWidget::Dialog(dialog) => dialog.set_title(Some(label)),
                    Gtk4OsWidget::Label(widget) => {
                        widget.set_text(label);
                        if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                            self.update_notebook_tab_label(id, &tab, label.to_string())?;
                        }
                    }
                    Gtk4OsWidget::Button(button) => button.set_label(label),
                    Gtk4OsWidget::CheckButton(check_button) => {
                        check_button.set_label(Some(label));
                    }
                    Gtk4OsWidget::ListBoxRow {
                        label: label_widget,
                        item,
                        ..
                    } => {
                        label_widget.set_text(label);
                        if let Some(label) = value {
                            self.update_drop_down_item_label(id, item, label.clone())?;
                        }
                    }
                    Gtk4OsWidget::MenuItem(item) => {
                        self.menus.update_item_label(id, item, label.to_string());
                    }
                    Gtk4OsWidget::Entry(_)
                    | Gtk4OsWidget::Dialog(_)
                    | Gtk4OsWidget::Popover(_)
                    | Gtk4OsWidget::Menu(_)
                    | Gtk4OsWidget::Switch(_)
                    | Gtk4OsWidget::DropDown(_)
                    | Gtk4OsWidget::ListBox(_)
                    | Gtk4OsWidget::Notebook(_)
                    | Gtk4OsWidget::Separator(_)
                    | Gtk4OsWidget::Scale(_)
                    | Gtk4OsWidget::ProgressBar(_)
                    | Gtk4OsWidget::Box(_) => {}
                }
            }
            NativeWidgetSetter::SetValue(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    self.suppress_events(|| entry.set_text(value.as_deref().unwrap_or("")));
                }
                Gtk4OsWidget::Label(label) => {
                    if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                        if let Some(value) = value {
                            self.update_notebook_tab_value(id, &tab, value.clone())?;
                        }
                    } else {
                        label.set_text(value.as_deref().unwrap_or(""));
                    }
                }
                Gtk4OsWidget::DropDown(_) => {
                    self.set_drop_down_value(id, value.as_deref());
                }
                Gtk4OsWidget::Notebook(_) => {
                    self.set_notebook_value(id, value.as_deref());
                }
                Gtk4OsWidget::ListBoxRow { item, .. } => {
                    self.update_drop_down_item_value(
                        id,
                        item,
                        value.clone().unwrap_or_else(|| item.label.clone()),
                    )?;
                }
                Gtk4OsWidget::MenuItem(item) => {
                    self.menus.update_item_value(
                        id,
                        item,
                        value.clone().unwrap_or_else(|| item.label.clone()),
                    );
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_)
                | Gtk4OsWidget::Scale(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetPlaceholder(value) => {
                if let Gtk4OsWidget::Entry(entry) = &handle.widget {
                    entry.set_placeholder_text(value.as_deref());
                }
            }
            NativeWidgetSetter::SetEnabled(value) => {
                if let Some(widget) = handle.widget.as_widget() {
                    widget.set_sensitive(*value);
                }
            }
            NativeWidgetSetter::SetVisible(value) => {
                if let Some(widget) = handle.widget.as_widget() {
                    widget.set_visible(*value);
                }
            }
            NativeWidgetSetter::SetChecked(value) => match &handle.widget {
                Gtk4OsWidget::CheckButton(check_button) => {
                    self.suppress_events(|| check_button.set_active(value.unwrap_or(false)));
                }
                Gtk4OsWidget::Switch(switch) => {
                    self.suppress_events(|| switch.set_active(value.unwrap_or(false)));
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_)
                | Gtk4OsWidget::Scale(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetSelected(value) => {
                if let Gtk4OsWidget::ListBoxRow { item, .. } = &handle.widget {
                    self.update_drop_down_item_selected(id, item, *value)?;
                }
                if let Gtk4OsWidget::MenuItem(item) = &handle.widget {
                    self.menus.update_item_selected(id, item, *value);
                }
                if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                    self.update_notebook_tab_selected(id, &tab, *value)?;
                }
            }
            NativeWidgetSetter::SetClassName(value) => {
                if let (Some(widget), Some(class_name)) = (handle.widget.as_widget(), value) {
                    for class_name in class_name.split_whitespace() {
                        widget.add_css_class(class_name);
                    }
                }
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                if let Some(widget) = handle.widget.as_widget() {
                    apply_widget_size(&widget, style);
                }
                if let Gtk4OsWidget::Box(box_) = &handle.widget {
                    if let Some(orientation) = style.flex_direction {
                        box_.set_orientation(gtk_orientation(orientation));
                    }
                    if let Some(gap) = style.gap.and_then(StyleLength::points) {
                        box_.set_spacing(points_to_i32(gap));
                    }
                }
            }
            NativeWidgetSetter::SetOrientation(value) => match &handle.widget {
                Gtk4OsWidget::Box(box_) => {
                    if let Some(value) = value {
                        box_.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::Separator(separator) => {
                    if let Some(value) = value {
                        separator.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::Scale(scale) => {
                    if let Some(value) = value {
                        scale.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetMinimum(value) => {
                let range = self.ranges.entry(id).or_default();
                range.min = *value;
                match &handle.widget {
                    Gtk4OsWidget::Scale(scale) => {
                        scale.set_range(range.lower(), range.upper());
                    }
                    Gtk4OsWidget::ProgressBar(progress_bar) => {
                        set_progress_bar_fraction(progress_bar, *range);
                    }
                    _ => {}
                }
            }
            NativeWidgetSetter::SetMaximum(value) => {
                let range = self.ranges.entry(id).or_default();
                range.max = *value;
                match &handle.widget {
                    Gtk4OsWidget::Scale(scale) => {
                        scale.set_range(range.lower(), range.upper());
                    }
                    Gtk4OsWidget::ProgressBar(progress_bar) => {
                        set_progress_bar_fraction(progress_bar, *range);
                    }
                    _ => {}
                }
            }
            NativeWidgetSetter::SetCurrent(value) => match &handle.widget {
                Gtk4OsWidget::Scale(scale) => {
                    let range = self.ranges.entry(id).or_default();
                    range.current = *value;
                    scale.set_value(range.current());
                }
                Gtk4OsWidget::ProgressBar(progress_bar) => {
                    let range = self.ranges.entry(id).or_default();
                    range.current = *value;
                    set_progress_bar_fraction(progress_bar, *range);
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_) => {}
            },
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetRequired(_)
            | NativeWidgetSetter::SetInvalid(_)
            | NativeWidgetSetter::SetExpanded(_)
            | NativeWidgetSetter::SetWebStyle(_)
            | NativeWidgetSetter::SetEvents(_)
            | NativeWidgetSetter::SetMetadata(_) => {}
        }
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
        if let (Gtk4OsWidget::DropDown(_), Gtk4OsWidget::ListBoxRow { item, .. }) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.drop_down_items
                .entry(child)
                .or_insert_with(|| item.clone());
            if let Some(old_parent) = self.drop_down_item_parents.insert(child, parent) {
                if let Some(children) = self.drop_down_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_drop_down(old_parent)?;
            }
            let children = self.drop_down_children.entry(parent).or_default();
            children.retain(|existing| *existing != child);
            let index = index.min(children.len());
            children.insert(index, child);
            self.rebuild_drop_down(parent)?;
            return Ok(());
        }

        if let Gtk4OsWidget::Notebook(_) = &parent_handle.widget {
            self.notebook_tabs
                .entry(child)
                .or_insert_with(|| Gtk4NotebookTab::fallback(child));
            if let Some(old_parent) = self.notebook_tab_parents.insert(child, parent) {
                if let Some(children) = self.notebook_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_notebook(old_parent)?;
            }
            {
                let children = self.notebook_children.entry(parent).or_default();
                children.retain(|existing| *existing != child);
                let index = index.min(children.len());
                children.insert(index, child);
            }
            self.rebuild_notebook(parent)?;
            return Ok(());
        }

        if self.notebook_tabs.contains_key(&parent) {
            self.update_notebook_tab_panel(parent, Some(child))?;
            return Ok(());
        }

        if let (Gtk4OsWidget::Menu(_), Gtk4OsWidget::MenuItem(item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.menus.insert_item(parent, child, item, index);
            return Ok(());
        }

        if let (Gtk4OsWidget::MenuItem(item), Gtk4OsWidget::Menu(menu)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            item.item.set_submenu(Some(&menu.model));
            return Ok(());
        }

        let child_widget = child_handle
            .widget
            .as_widget()
            .ok_or_else(|| GuiError::host("GTK4 native child insertion requires a widget child"))?;
        match &parent_handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => {
                window.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::Box(box_) => {
                self.insert_box_child(parent, box_, child, &child_widget, index);
            }
            Gtk4OsWidget::Button(button) => {
                button.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::ListBox(list_box) => {
                list_box.insert(&child_widget, index_to_i32(index)?);
            }
            Gtk4OsWidget::ListBoxRow { row, .. } => {
                row.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::Dialog(dialog) => {
                let content_area = dialog.content_area();
                self.insert_box_child(parent, &content_area, child, &child_widget, index);
            }
            Gtk4OsWidget::Popover(popover) => {
                popover.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::DropDown(_)
            | Gtk4OsWidget::Notebook(_)
            | Gtk4OsWidget::Menu(_)
            | Gtk4OsWidget::MenuItem(_)
            | Gtk4OsWidget::Label(_)
            | Gtk4OsWidget::Entry(_)
            | Gtk4OsWidget::CheckButton(_)
            | Gtk4OsWidget::Switch(_)
            | Gtk4OsWidget::Separator(_)
            | Gtk4OsWidget::Scale(_)
            | Gtk4OsWidget::ProgressBar(_) => {}
        }
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        if self.root == Some(id) {
            self.root = None;
        }
        self.widgets.remove(&id);
        for children in self.container_children.values_mut() {
            children.retain(|child| *child != id);
        }
        self.container_children.remove(&id);

        let tabs_with_removed_panel = self
            .notebook_tabs
            .iter()
            .filter_map(|(tab, item)| (item.panel == Some(id)).then_some(*tab))
            .collect::<Vec<_>>();
        for tab in tabs_with_removed_panel {
            self.update_notebook_tab_panel(tab, None)?;
        }

        if let Gtk4OsWidget::Notebook(_) = &handle.widget {
            self.notebooks.remove(&id);
            self.notebook_selected_values.remove(&id);
            self.notebook_values.borrow_mut().remove(&id);
            if let Some(children) = self.notebook_children.remove(&id) {
                for child in children {
                    self.notebook_tab_parents.remove(&child);
                }
            }
        }

        if self.notebook_tabs.contains_key(&id) {
            if let Some(parent) = self.notebook_tab_parents.remove(&id) {
                if let Some(children) = self.notebook_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_notebook(parent)?;
            }
            self.notebook_tabs.remove(&id);
        }

        if let Gtk4OsWidget::DropDown(_) = &handle.widget {
            self.drop_downs.remove(&id);
            self.drop_down_selected_values.remove(&id);
            self.drop_down_values.borrow_mut().remove(&id);
            if let Some(children) = self.drop_down_children.remove(&id) {
                for child in children {
                    self.drop_down_item_parents.remove(&child);
                }
            }
        }
        if let Gtk4OsWidget::ListBoxRow { .. } = &handle.widget {
            self.drop_down_items.remove(&id);
            if let Some(parent) = self.drop_down_item_parents.remove(&id) {
                if let Some(children) = self.drop_down_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_drop_down(parent)?;
            }
        }
        if let Gtk4OsWidget::Menu(_) = &handle.widget {
            self.menus.remove_menu(id);
        }
        if let Gtk4OsWidget::MenuItem(_) = &handle.widget {
            self.menus.remove_item(id, &self.application);
        }
        self.ranges.remove(&id);
        match &handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => window.close(),
            Gtk4OsWidget::Dialog(dialog) => dialog.close(),
            Gtk4OsWidget::Popover(popover) => popover.popdown(),
            other => {
                if let Some(widget) = other.as_widget() {
                    if widget.parent().is_some() {
                        widget.unparent();
                    }
                }
            }
        }
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.root = Some(id);
        match &handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => window.present(),
            Gtk4OsWidget::Dialog(dialog) => dialog.present(),
            Gtk4OsWidget::Popover(popover) => popover.popup(),
            other => {
                if let Some(widget) = other.as_widget() {
                    widget.set_visible(true);
                }
            }
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
}

fn push_event(
    events: &Rc<RefCell<Vec<NativeEvent>>>,
    events_suppressed: &Rc<RefCell<bool>>,
    event: NativeEvent,
) {
    if !*events_suppressed.borrow() {
        events.borrow_mut().push(event);
    }
}

fn config_orientation(config: &NativeWidgetConfig) -> Option<gtk::Orientation> {
    config
        .orientation
        .or(config.portable_style.flex_direction)
        .map(gtk_orientation)
}

fn gtk_orientation(orientation: Orientation) -> gtk::Orientation {
    match orientation {
        Orientation::Horizontal => gtk::Orientation::Horizontal,
        Orientation::Vertical => gtk::Orientation::Vertical,
    }
}

fn config_dimension(value: Option<StyleLength>, default: i32) -> i32 {
    value
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(default)
}

fn apply_widget_size(widget: &gtk::Widget, style: &crate::style::PortableStyle) {
    let width = style
        .width
        .or(style.min_width)
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(-1);
    let height = style
        .height
        .or(style.min_height)
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(-1);
    if width >= 0 || height >= 0 {
        widget.set_size_request(width, height);
    }
}

fn set_progress_bar_fraction(progress_bar: &gtk::ProgressBar, range: Gtk4RangeState) {
    let min = range.lower();
    let max = range.upper();
    let current = range.current();
    let range = max - min;
    let fraction = if range.abs() < f64::EPSILON {
        0.0
    } else {
        ((current - min) / range).clamp(0.0, 1.0)
    };
    progress_bar.set_fraction(fraction);
}

fn points_to_i32(value: f64) -> i32 {
    if value.is_finite() {
        value.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32
    } else {
        -1
    }
}

fn index_to_i32(index: usize) -> GuiResult<i32> {
    index
        .try_into()
        .map_err(|_| GuiError::host("GTK4 child index overflow"))
}
