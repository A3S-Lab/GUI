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

mod surface;

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
    text_inputs: BTreeMap<HostNodeId, Gtk4TextInputSizing>,
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
            text_inputs: BTreeMap::new(),
            text_input_max_lengths: Rc::new(RefCell::new(BTreeMap::new())),
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

    fn apply_text_view_size_hint(&self, id: HostNodeId, text_view: &gtk::TextView) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        let width = if sizing.has_explicit_width {
            -1
        } else {
            sizing.hinted_width_points().unwrap_or(-1)
        };
        let height = if sizing.has_explicit_height {
            -1
        } else {
            sizing.hinted_height_points().unwrap_or(-1)
        };
        if width >= 0 || height >= 0 {
            text_view.set_size_request(width, height);
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
    step: Option<f64>,
}

impl Gtk4RangeState {
    fn from_config(config: &NativeWidgetConfig) -> Self {
        Self {
            min: config.min,
            max: config.max,
            current: config.current,
            step: config.step,
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

    fn step(self) -> f64 {
        self.step.filter(|value| *value > 0.0).unwrap_or(1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Gtk4TextInputSizing {
    rows: Option<u32>,
    cols: Option<u32>,
    size: Option<u32>,
    has_explicit_width: bool,
    has_explicit_height: bool,
}

impl Gtk4TextInputSizing {
    fn from_config(config: &NativeWidgetConfig) -> Self {
        Self {
            rows: config.rows,
            cols: config.cols,
            size: config.size,
            has_explicit_width: style_sets_gtk_width(&config.portable_style),
            has_explicit_height: style_sets_gtk_height(&config.portable_style),
        }
    }

    fn hinted_width_chars(self) -> Option<i32> {
        self.size
            .or(self.cols)
            .filter(|value| *value > 0)
            .map(u32_to_i32)
    }

    fn hinted_width_points(self) -> Option<i32> {
        self.cols
            .filter(|value| *value > 0)
            .map(|columns| (columns as f64 * 8.0 + 28.0).max(80.0))
            .map(points_to_i32)
    }

    fn hinted_height_points(self) -> Option<i32> {
        self.rows
            .filter(|value| *value > 0)
            .map(|rows| (rows as f64 * 20.0 + 18.0).max(64.0))
            .map(points_to_i32)
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
    PasswordEntry(gtk::PasswordEntry),
    TextView(gtk::TextView),
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
            Gtk4OsWidget::PasswordEntry(entry) => Some(entry.clone().upcast()),
            Gtk4OsWidget::TextView(text_view) => Some(text_view.clone().upcast()),
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
        .as_ref()
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(default)
}

fn apply_widget_size(widget: &gtk::Widget, style: &crate::style::PortableStyle) {
    let width = style
        .width
        .as_ref()
        .or(style.min_width.as_ref())
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(-1);
    let height = style
        .height
        .as_ref()
        .or(style.min_height.as_ref())
        .and_then(StyleLength::points)
        .map(points_to_i32)
        .unwrap_or(-1);
    if width >= 0 || height >= 0 {
        widget.set_size_request(width, height);
    }
}

fn style_sets_gtk_width(style: &crate::style::PortableStyle) -> bool {
    style
        .width
        .as_ref()
        .or(style.min_width.as_ref())
        .and_then(StyleLength::points)
        .is_some()
}

fn style_sets_gtk_height(style: &crate::style::PortableStyle) -> bool {
    style
        .height
        .as_ref()
        .or(style.min_height.as_ref())
        .and_then(StyleLength::points)
        .is_some()
}

fn config_is_password(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("password"))
}

fn u32_to_i32(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

fn text_buffer_text(buffer: &gtk::TextBuffer) -> String {
    let (start, end) = buffer.bounds();
    buffer.text(&start, &end, true).to_string()
}

fn truncate_to_max_length(value: &str, max_length: Option<u32>) -> String {
    let Some(max_length) = max_length else {
        return value.to_string();
    };
    let max_length = max_length as usize;
    if value.chars().count() <= max_length {
        value.to_string()
    } else {
        value.chars().take(max_length).collect()
    }
}

fn set_text_buffer_text(buffer: &gtk::TextBuffer, value: &str, max_length: Option<u32>) {
    buffer.set_text(&truncate_to_max_length(value, max_length));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_to_max_length_limits_unicode_scalar_values() {
        assert_eq!(truncate_to_max_length("abcdef", Some(3)), "abc");
        assert_eq!(truncate_to_max_length("aé日b", Some(3)), "aé日");
        assert_eq!(truncate_to_max_length("abc", None), "abc");
        assert_eq!(truncate_to_max_length("abc", Some(0)), "");
    }
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
