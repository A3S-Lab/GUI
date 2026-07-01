use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use windows::Foundation::PropertyValue;
use windows_core::{Interface, HSTRING};
use winui3::bootstrap::PackageDependency;
use winui3::Microsoft::UI::Xaml as xaml;
use xaml::Controls::{self, Primitives};
use xaml::{Markup, RoutedEventHandler, Visibility};

use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation as A3sOrientation;
use crate::host::HostNodeId;
use crate::native_backends::winui::menu as winui_menu;
use crate::platform::{
    NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetSetter, WinUiAdapter,
};
use crate::style::{PortableStyle, StyleLength};
use crate::winui::WinUiWidgetKind;

type WinUiEventQueue = Arc<Mutex<Vec<NativeEvent>>>;

pub type WinUiNativeSurfaceAdapter = SurfaceHandleAdapter<WinUiNativeSurface>;
pub type WinUiNativeSurfaceDriver = HandleWidgetDriver<WinUiNativeSurfaceAdapter>;
pub type WinUiNativeSurfaceCommandExecutor = DriverCommandExecutor<WinUiNativeSurfaceDriver>;

#[derive(Debug)]
pub struct WinUiNativeSurface {
    _package_dependency: PackageDependency,
    root: Option<HostNodeId>,
    events: WinUiEventQueue,
    events_suppressed: Arc<AtomicBool>,
    widgets: BTreeMap<HostNodeId, WinUiOsWidget>,
    container_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    combo_boxes: BTreeMap<HostNodeId, ControlsComboBox>,
    combo_items: BTreeMap<HostNodeId, WinUiComboBoxItem>,
    combo_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    combo_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    combo_selected_values: BTreeMap<HostNodeId, Option<String>>,
    combo_values: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
    list_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    tab_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    tab_items: BTreeMap<HostNodeId, WinUiTabItem>,
    tab_selected_values: BTreeMap<HostNodeId, Option<String>>,
    tab_values: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
    ranges: BTreeMap<HostNodeId, WinUiRangeState>,
}

type ControlsComboBox = Controls::ComboBox;

impl WinUiNativeSurface {
    pub fn new() -> GuiResult<Self> {
        map_winui(
            "failed to initialize WinRT single-threaded apartment",
            winui3::init_apartment(winui3::ApartmentType::SingleThreaded),
        )?;
        let package_dependency = map_winui(
            "failed to initialize Windows App SDK package dependency",
            PackageDependency::initialize(),
        )?;
        Ok(Self::with_package_dependency(package_dependency))
    }

    pub fn with_package_dependency(package_dependency: PackageDependency) -> Self {
        Self {
            _package_dependency: package_dependency,
            root: None,
            events: Arc::new(Mutex::new(Vec::new())),
            events_suppressed: Arc::new(AtomicBool::new(false)),
            widgets: BTreeMap::new(),
            container_children: BTreeMap::new(),
            combo_boxes: BTreeMap::new(),
            combo_items: BTreeMap::new(),
            combo_children: BTreeMap::new(),
            combo_item_parents: BTreeMap::new(),
            combo_selected_values: BTreeMap::new(),
            combo_values: Arc::new(Mutex::new(BTreeMap::new())),
            list_children: BTreeMap::new(),
            tab_children: BTreeMap::new(),
            tab_items: BTreeMap::new(),
            tab_selected_values: BTreeMap::new(),
            tab_values: Arc::new(Mutex::new(BTreeMap::new())),
            ranges: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn into_driver(self) -> WinUiNativeSurfaceDriver {
        HandleWidgetDriver::new(SurfaceHandleAdapter::new(self))
    }

    pub fn into_executor(self) -> WinUiNativeSurfaceCommandExecutor {
        DriverCommandExecutor::new(self.into_driver())
    }

    pub fn into_host(
        self,
    ) -> CommandExecutingHost<WinUiAdapter, WinUiNativeSurfaceCommandExecutor> {
        CommandExecutingHost::new(WinUiAdapter, self.into_executor())
    }

    fn suppress_events<T>(&self, apply: impl FnOnce() -> T) -> T {
        let previous = self.events_suppressed.swap(true, Ordering::SeqCst);
        let result = apply();
        self.events_suppressed.store(previous, Ordering::SeqCst);
        result
    }

    fn update_combo_item_label(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiComboBoxItem,
        label: String,
    ) -> GuiResult<()> {
        let item = self
            .combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone());
        if item.value == item.label {
            item.value = label.clone();
        }
        item.label = label;
        self.rebuild_combo_for_item(id)
    }

    fn update_combo_item_value(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiComboBoxItem,
        value: String,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
        self.rebuild_combo_for_item(id)
    }

    fn update_combo_item_selected(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiComboBoxItem,
        selected: bool,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
        self.rebuild_combo_for_item(id)
    }

    fn rebuild_combo_for_item(&mut self, item: HostNodeId) -> GuiResult<()> {
        if let Some(parent) = self.combo_item_parents.get(&item).copied() {
            self.rebuild_combo_box(parent)?;
        }
        Ok(())
    }

    fn rebuild_combo_box(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(combo_box) = self.combo_boxes.get(&id).cloned() else {
            return Ok(());
        };
        let previous_value = self.selected_combo_value(id, &combo_box);
        let children = self.combo_children.get(&id).cloned().unwrap_or_default();
        let mut values = Vec::new();
        let mut selected_value = None;
        let items = map_winui("failed to read WinUI combo box items", combo_box.Items())?;

        self.suppress_events(|| -> GuiResult<()> {
            map_winui("failed to clear WinUI combo box items", items.Clear())?;
            for child in children {
                let Some(item) = self.combo_items.get(&child) else {
                    continue;
                };
                let combo_item = map_winui(
                    "failed to create WinUI combo box item",
                    Controls::ComboBoxItem::new(),
                )?;
                set_combo_box_item_content(&combo_item, &item.label)?;
                map_winui(
                    "failed to append WinUI combo box item",
                    items.Append(&combo_item),
                )?;
                values.push(item.value.clone());
                if item.selected && selected_value.is_none() {
                    selected_value = Some(item.value.clone());
                }
            }
            Ok(())
        })?;

        let selected_value = selected_value
            .or_else(|| self.combo_selected_values.get(&id).cloned().flatten())
            .or_else(|| (!previous_value.is_empty()).then_some(previous_value));
        let selected_index = selected_value
            .as_ref()
            .and_then(|value| values.iter().position(|item_value| item_value == value))
            .map(|index| index as i32)
            .unwrap_or(-1);

        if let Ok(mut combo_values) = self.combo_values.lock() {
            combo_values.insert(id, values);
        }
        self.suppress_events(|| {
            map_winui(
                "failed to set WinUI combo box selected index",
                combo_box.SetSelectedIndex(selected_index),
            )
        })?;
        Ok(())
    }

    fn set_combo_value(
        &mut self,
        id: HostNodeId,
        combo_box: &Controls::ComboBox,
        value: Option<&str>,
    ) -> GuiResult<()> {
        self.combo_selected_values
            .insert(id, value.map(str::to_string));
        let selected_index = value
            .and_then(|value| {
                self.combo_values
                    .lock()
                    .ok()
                    .and_then(|values| values.get(&id).cloned())
                    .and_then(|values| values.iter().position(|item_value| item_value == value))
            })
            .map(|index| index as i32)
            .unwrap_or(-1);
        self.suppress_events(|| {
            map_winui(
                "failed to set WinUI combo box selected index",
                combo_box.SetSelectedIndex(selected_index),
            )
        })
    }

    fn selected_combo_value(&self, id: HostNodeId, combo_box: &Controls::ComboBox) -> String {
        let Ok(index) = combo_box.SelectedIndex() else {
            return String::new();
        };
        if index < 0 {
            return String::new();
        }
        self.combo_values
            .lock()
            .ok()
            .and_then(|values| values.get(&id).cloned())
            .and_then(|values| values.get(index as usize).cloned())
            .unwrap_or_default()
    }

    fn update_tab_item_label(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiTabItem,
        label: String,
    ) -> GuiResult<()> {
        let item = self.tab_items.entry(id).or_insert_with(|| fallback.clone());
        if item.value == item.label {
            item.value = label.clone();
        }
        item.label = label;
        self.rebuild_tab_for_item(id)
    }

    fn update_tab_item_value(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiTabItem,
        value: String,
    ) -> GuiResult<()> {
        self.tab_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
        self.rebuild_tab_for_item(id)
    }

    fn update_tab_item_selected(
        &mut self,
        id: HostNodeId,
        fallback: &WinUiTabItem,
        selected: bool,
    ) -> GuiResult<()> {
        self.tab_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
        self.rebuild_tab_for_item(id)
    }

    fn rebuild_tab_for_item(&mut self, item: HostNodeId) -> GuiResult<()> {
        if let Some((parent, _)) = child_position(&self.tab_children, item) {
            self.rebuild_tab_view(parent)?;
        }
        Ok(())
    }

    fn rebuild_tab_view(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(WinUiOsWidget::TabView(tab_view)) = self.widgets.get(&id).cloned() else {
            return Ok(());
        };
        let children = self.tab_children.get(&id).cloned().unwrap_or_default();
        let previous_index = tab_view
            .SelectedIndex()
            .ok()
            .and_then(|index| (index >= 0).then_some(index as usize));
        let mut values = Vec::with_capacity(children.len());
        let mut selected_item_value = None;

        for child in children {
            let item = self
                .tab_items
                .get(&child)
                .cloned()
                .unwrap_or_else(|| WinUiTabItem::fallback(child));
            if item.selected && selected_item_value.is_none() {
                selected_item_value = Some(item.value.clone());
            }
            values.push(item.value);
        }

        let selected_value = self
            .tab_selected_values
            .get(&id)
            .cloned()
            .flatten()
            .or(selected_item_value);
        let selected_index = selected_value
            .as_ref()
            .and_then(|value| values.iter().position(|item_value| item_value == value))
            .or_else(|| previous_index.filter(|index| *index < values.len()));

        if let Ok(mut tab_values) = self.tab_values.lock() {
            tab_values.insert(id, values);
        }

        if let Some(selected_index) = selected_index {
            self.suppress_events(|| {
                map_winui(
                    "failed to set WinUI tab view selected index",
                    tab_view.SetSelectedIndex(selected_index as i32),
                )
            })?;
        }
        Ok(())
    }

    fn set_tab_value(
        &mut self,
        id: HostNodeId,
        tab_view: &Controls::TabView,
        value: Option<&str>,
    ) -> GuiResult<()> {
        self.tab_selected_values
            .insert(id, value.map(str::to_string));
        let Some(value) = value else {
            return Ok(());
        };
        let selected_index = self
            .tab_values
            .lock()
            .ok()
            .and_then(|values| values.get(&id).cloned())
            .and_then(|values| values.iter().position(|item_value| item_value == value))
            .map(|index| index as i32)
            .unwrap_or(-1);
        self.suppress_events(|| {
            map_winui(
                "failed to set WinUI tab view selected index",
                tab_view.SetSelectedIndex(selected_index),
            )
        })
    }

    fn insert_panel_child(
        &mut self,
        parent: HostNodeId,
        collection: Controls::UIElementCollection,
        child: HostNodeId,
        child_element: xaml::UIElement,
        index: usize,
    ) -> GuiResult<()> {
        let children = self.container_children.entry(parent).or_default();
        if let Some(previous_index) = children.iter().position(|existing| *existing == child) {
            map_winui(
                "failed to move existing WinUI panel child",
                collection.RemoveAt(to_u32(previous_index)?),
            )?;
            children.remove(previous_index);
        }
        let index = index.min(children.len());
        map_winui(
            "failed to insert WinUI panel child",
            collection.InsertAt(to_u32(index)?, &child_element),
        )?;
        children.insert(index, child);
        Ok(())
    }

    fn insert_items_child(
        children_by_parent: &mut BTreeMap<HostNodeId, Vec<HostNodeId>>,
        parent: HostNodeId,
        collection: Controls::ItemCollection,
        child: HostNodeId,
        child_object: windows_core::IInspectable,
        index: usize,
    ) -> GuiResult<()> {
        let children = children_by_parent.entry(parent).or_default();
        if let Some(previous_index) = children.iter().position(|existing| *existing == child) {
            map_winui(
                "failed to move existing WinUI items child",
                collection.RemoveAt(to_u32(previous_index)?),
            )?;
            children.remove(previous_index);
        }
        let index = index.min(children.len());
        map_winui(
            "failed to insert WinUI items child",
            collection.InsertAt(to_u32(index)?, &child_object),
        )?;
        children.insert(index, child);
        Ok(())
    }

    fn detach_child(&mut self, id: HostNodeId) -> GuiResult<()> {
        if let Some((parent, index)) = child_position(&self.container_children, id) {
            if let Some(parent_widget) = self.widgets.get(&parent).cloned() {
                if let Some(collection) = parent_widget.children_collection()? {
                    map_winui(
                        "failed to remove WinUI panel child",
                        collection.RemoveAt(to_u32(index)?),
                    )?;
                }
            }
            if let Some(children) = self.container_children.get_mut(&parent) {
                children.remove(index);
            }
        }

        if let Some((parent, index)) = child_position(&self.list_children, id) {
            if let Some(parent_widget) = self.widgets.get(&parent).cloned() {
                if let Some(collection) = parent_widget.items_collection()? {
                    map_winui(
                        "failed to remove WinUI items child",
                        collection.RemoveAt(to_u32(index)?),
                    )?;
                }
            }
            if let Some(children) = self.list_children.get_mut(&parent) {
                children.remove(index);
            }
        }

        if let Some((parent, index)) = child_position(&self.tab_children, id) {
            if let Some(WinUiOsWidget::TabView(tab_view)) = self.widgets.get(&parent).cloned() {
                let items = map_winui("failed to read WinUI tab view items", tab_view.TabItems())?;
                map_winui(
                    "failed to remove WinUI tab view item",
                    items.RemoveAt(to_u32(index)?),
                )?;
            }
            if let Some(children) = self.tab_children.get_mut(&parent) {
                children.remove(index);
            }
            self.rebuild_tab_view(parent)?;
        }

        if let Some(parent) = self.combo_item_parents.remove(&id) {
            if let Some(children) = self.combo_children.get_mut(&parent) {
                children.retain(|child| *child != id);
            }
            self.rebuild_combo_box(parent)?;
        }

        Ok(())
    }

    fn apply_range(&mut self, id: HostNodeId, widget: &WinUiOsWidget) -> GuiResult<()> {
        let state = self.ranges.get(&id).copied().unwrap_or_default();
        match widget {
            WinUiOsWidget::Slider(slider) => {
                map_winui(
                    "failed to set WinUI slider minimum",
                    slider.SetMinimum(state.lower()),
                )?;
                map_winui(
                    "failed to set WinUI slider maximum",
                    slider.SetMaximum(state.upper()),
                )?;
                self.suppress_events(|| {
                    map_winui(
                        "failed to set WinUI slider value",
                        slider.SetValue(state.current()),
                    )
                })?;
            }
            WinUiOsWidget::ProgressBar(progress) => {
                map_winui(
                    "failed to set WinUI progress bar minimum",
                    progress.SetMinimum(state.lower()),
                )?;
                map_winui(
                    "failed to set WinUI progress bar maximum",
                    progress.SetMaximum(state.upper()),
                )?;
                map_winui(
                    "failed to set WinUI progress bar value",
                    progress.SetValue(state.current()),
                )?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WinUiOsHandle {
    pub id: HostNodeId,
    pub kind: WinUiWidgetKind,
    pub widget: WinUiOsWidget,
}

#[derive(Debug, Clone)]
pub enum WinUiOsWidget {
    Window(xaml::Window),
    StackPanel(Controls::StackPanel),
    TextBlock(Controls::TextBlock),
    Separator(xaml::FrameworkElement),
    Button(Controls::Button),
    TextBox(Controls::TextBox),
    CheckBox(Controls::CheckBox),
    ToggleSwitch(Controls::CheckBox),
    RadioButton(Controls::RadioButton),
    ComboBox(Controls::ComboBox),
    ComboBoxItem(Controls::ComboBoxItem),
    ListBox(Controls::ListBox),
    ListBoxItem(Controls::ListBoxItem),
    ContentDialog(Controls::ContentDialog),
    ToolTip(Controls::ToolTip),
    TabView(Controls::TabView),
    TabViewItem(Controls::TabViewItem),
    Grid(Controls::Grid),
    Slider(Controls::Slider),
    ProgressBar(Controls::ProgressBar),
}

impl WinUiOsWidget {
    fn ui_element(&self) -> Option<xaml::UIElement> {
        match self {
            WinUiOsWidget::Window(_) => None,
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => widget.cast().ok(),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    fn inspectable(&self) -> Option<windows_core::IInspectable> {
        match self {
            WinUiOsWidget::Window(widget) => widget.cast().ok(),
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => widget.cast().ok(),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    fn framework_element(&self) -> Option<xaml::FrameworkElement> {
        match self {
            WinUiOsWidget::Window(_) => None,
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => Some(widget.clone()),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    fn control(&self) -> Option<Controls::Control> {
        match self {
            WinUiOsWidget::Window(_)
            | WinUiOsWidget::StackPanel(_)
            | WinUiOsWidget::TextBlock(_)
            | WinUiOsWidget::Separator(_)
            | WinUiOsWidget::Grid(_) => None,
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    fn children_collection(&self) -> GuiResult<Option<Controls::UIElementCollection>> {
        match self {
            WinUiOsWidget::StackPanel(widget) => Ok(Some(map_winui(
                "failed to read WinUI stack panel children",
                widget.Children(),
            )?)),
            WinUiOsWidget::Grid(widget) => Ok(Some(map_winui(
                "failed to read WinUI grid children",
                widget.Children(),
            )?)),
            _ => Ok(None),
        }
    }

    fn items_collection(&self) -> GuiResult<Option<Controls::ItemCollection>> {
        match self {
            WinUiOsWidget::ComboBox(widget) => Ok(Some(map_winui(
                "failed to read WinUI combo box items",
                widget.Items(),
            )?)),
            WinUiOsWidget::ListBox(widget) => Ok(Some(map_winui(
                "failed to read WinUI list box items",
                widget.Items(),
            )?)),
            WinUiOsWidget::TabView(_) => Ok(None),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WinUiComboBoxItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl WinUiComboBoxItem {
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
pub struct WinUiTabItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl WinUiTabItem {
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
        }
    }

    fn fallback(id: HostNodeId) -> Self {
        let label = id.get().to_string();
        Self {
            label: label.clone(),
            value: label,
            selected: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct WinUiRangeState {
    min: Option<f64>,
    max: Option<f64>,
    current: Option<f64>,
}

impl WinUiRangeState {
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

impl NativeWidgetSurface for WinUiNativeSurface {
    type Handle = WinUiOsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let kind = WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let config = blueprint.config();
        let widget = match kind {
            WinUiWidgetKind::Window => {
                let window = map_winui("failed to create WinUI window", xaml::Window::new())?;
                if let Some(label) = config.label.as_deref() {
                    map_winui(
                        "failed to set WinUI window title",
                        window.SetTitle(&hstr(label)),
                    )?;
                }
                WinUiOsWidget::Window(window)
            }
            WinUiWidgetKind::StackPanel
            | WinUiWidgetKind::RadioButtons
            | WinUiWidgetKind::MenuPanel
            | WinUiWidgetKind::CommandBar => {
                let panel = map_winui(
                    "failed to create WinUI stack panel",
                    Controls::StackPanel::new(),
                )?;
                if let Some(orientation) =
                    winui_menu::stack_panel_orientation(kind, config.orientation)
                {
                    let orientation = match orientation {
                        A3sOrientation::Horizontal => Controls::Orientation::Horizontal,
                        A3sOrientation::Vertical => Controls::Orientation::Vertical,
                    };
                    map_winui(
                        "failed to set WinUI stack panel orientation",
                        panel.SetOrientation(orientation),
                    )?;
                }
                WinUiOsWidget::StackPanel(panel)
            }
            WinUiWidgetKind::TextBlock => {
                let text = map_winui(
                    "failed to create WinUI text block",
                    Controls::TextBlock::new(),
                )?;
                WinUiOsWidget::TextBlock(text)
            }
            WinUiWidgetKind::Separator => {
                let separator = create_winui_separator(config.orientation)?;
                WinUiOsWidget::Separator(separator)
            }
            WinUiWidgetKind::Button | WinUiWidgetKind::MenuItemButton => {
                let button = map_winui("failed to create WinUI button", Controls::Button::new())?;
                register_press(id, &button, &self.events)?;
                WinUiOsWidget::Button(button)
            }
            WinUiWidgetKind::TextBox => {
                let text_box =
                    map_winui("failed to create WinUI text box", Controls::TextBox::new())?;
                register_text_change(
                    id,
                    &text_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::TextBox(text_box)
            }
            WinUiWidgetKind::CheckBox => {
                let check_box =
                    map_winui("failed to create WinUI checkbox", Controls::CheckBox::new())?;
                register_toggle(
                    id,
                    &check_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::CheckBox(check_box)
            }
            WinUiWidgetKind::ToggleSwitch => {
                let check_box = map_winui(
                    "failed to create WinUI switch fallback checkbox",
                    Controls::CheckBox::new(),
                )?;
                register_toggle(
                    id,
                    &check_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::ToggleSwitch(check_box)
            }
            WinUiWidgetKind::RadioButton => {
                let radio = map_winui(
                    "failed to create WinUI radio button",
                    Controls::RadioButton::new(),
                )?;
                register_radio_toggle(
                    id,
                    &radio,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::RadioButton(radio)
            }
            WinUiWidgetKind::ComboBox => {
                let combo_box = map_winui(
                    "failed to create WinUI combo box",
                    Controls::ComboBox::new(),
                )?;
                register_combo_selection(
                    id,
                    &combo_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.combo_values),
                )?;
                self.combo_boxes.insert(id, combo_box.clone());
                WinUiOsWidget::ComboBox(combo_box)
            }
            WinUiWidgetKind::ListView => {
                let list_box =
                    map_winui("failed to create WinUI list box", Controls::ListBox::new())?;
                register_list_selection(id, &list_box, &self.events)?;
                WinUiOsWidget::ListBox(list_box)
            }
            WinUiWidgetKind::TabView => {
                let tab_view =
                    map_winui("failed to create WinUI tab view", Controls::TabView::new())?;
                register_tab_selection(
                    id,
                    &tab_view,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.tab_values),
                )?;
                WinUiOsWidget::TabView(tab_view)
            }
            WinUiWidgetKind::TabViewItem => {
                let item = map_winui(
                    "failed to create WinUI tab view item",
                    Controls::TabViewItem::new(),
                )?;
                self.tab_items
                    .insert(id, WinUiTabItem::from_config(id, &config));
                WinUiOsWidget::TabViewItem(item)
            }
            WinUiWidgetKind::ListViewItem => {
                let item = map_winui(
                    "failed to create WinUI list box item",
                    Controls::ListBoxItem::new(),
                )?;
                self.combo_items
                    .insert(id, WinUiComboBoxItem::from_config(&config));
                WinUiOsWidget::ListBoxItem(item)
            }
            WinUiWidgetKind::ContentDialog => {
                let dialog = map_winui(
                    "failed to create WinUI content dialog",
                    Controls::ContentDialog::new(),
                )?;
                if let Some(label) = config.label.as_deref() {
                    let title = text_content(label)?;
                    map_winui("failed to set WinUI dialog title", dialog.SetTitle(&title))?;
                }
                WinUiOsWidget::ContentDialog(dialog)
            }
            WinUiWidgetKind::ToolTip => {
                let tool_tip = map_winui(
                    "failed to create WinUI tooltip popover",
                    Controls::ToolTip::new(),
                )?;
                if let Some(label) = config.label.as_deref() {
                    let content = text_content(label)?;
                    map_winui(
                        "failed to set WinUI tooltip popover content",
                        tool_tip.SetContent(&content),
                    )?;
                }
                WinUiOsWidget::ToolTip(tool_tip)
            }
            WinUiWidgetKind::SelectorItem => {
                let item = map_winui(
                    "failed to create WinUI combo box item",
                    Controls::ComboBoxItem::new(),
                )?;
                self.combo_items
                    .insert(id, WinUiComboBoxItem::from_config(&config));
                WinUiOsWidget::ComboBoxItem(item)
            }
            WinUiWidgetKind::Grid => {
                let grid = map_winui("failed to create WinUI grid", Controls::Grid::new())?;
                WinUiOsWidget::Grid(grid)
            }
            WinUiWidgetKind::Slider => {
                let slider = map_winui("failed to create WinUI slider", Controls::Slider::new())?;
                register_range_change(
                    id,
                    &slider,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                self.ranges
                    .insert(id, WinUiRangeState::from_config(&config));
                WinUiOsWidget::Slider(slider)
            }
            WinUiWidgetKind::ProgressBar => {
                let progress = map_winui(
                    "failed to create WinUI progress bar",
                    Controls::ProgressBar::new(),
                )?;
                self.ranges
                    .insert(id, WinUiRangeState::from_config(&config));
                WinUiOsWidget::ProgressBar(progress)
            }
        };

        register_focus_events(id, &widget, &self.events)?;
        self.widgets.insert(id, widget.clone());
        Ok(WinUiOsHandle { id, kind, widget })
    }

    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        match setter {
            NativeWidgetSetter::SetLabel(value) => {
                set_label(&handle.widget, value.as_deref())?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_label(id, &item, value.clone().unwrap_or_default())?;
                }
                if let Some(item) = self.tab_items.get(&id).cloned() {
                    self.update_tab_item_label(id, &item, value.clone().unwrap_or_default())?;
                }
            }
            NativeWidgetSetter::SetValue(value) => {
                set_value(self, id, &handle.widget, value.as_deref())?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_value(id, &item, value.clone().unwrap_or_default())?;
                }
                if let (Some(item), Some(value)) = (self.tab_items.get(&id).cloned(), value) {
                    self.update_tab_item_value(id, &item, value.clone())?;
                }
            }
            NativeWidgetSetter::SetPlaceholder(value) => {
                set_placeholder(&handle.widget, value.as_deref())?;
            }
            NativeWidgetSetter::SetEnabled(enabled) => {
                if let Some(control) = handle.widget.control() {
                    map_winui(
                        "failed to set WinUI control enabled state",
                        control.SetIsEnabled(*enabled),
                    )?;
                }
            }
            NativeWidgetSetter::SetVisible(visible) => {
                if let WinUiOsWidget::ToolTip(tool_tip) = &handle.widget {
                    map_winui(
                        "failed to set WinUI tooltip popover open state",
                        tool_tip.SetIsOpen(*visible),
                    )?;
                }
                if let Some(element) = handle.widget.ui_element() {
                    let visibility = if *visible {
                        Visibility::Visible
                    } else {
                        Visibility::Collapsed
                    };
                    map_winui(
                        "failed to set WinUI element visibility",
                        element.SetVisibility(visibility),
                    )?;
                }
            }
            NativeWidgetSetter::SetSelected(selected) => {
                set_selected(&handle.widget, *selected)?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_selected(id, &item, *selected)?;
                }
                if let Some(item) = self.tab_items.get(&id).cloned() {
                    self.update_tab_item_selected(id, &item, *selected)?;
                }
            }
            NativeWidgetSetter::SetChecked(checked) => {
                if let Some(checked) = checked {
                    set_checked(self, &handle.widget, *checked)?;
                }
            }
            NativeWidgetSetter::SetOrientation(orientation) => {
                if let WinUiOsWidget::Separator(separator) = &handle.widget {
                    set_winui_separator_orientation(separator, *orientation)?;
                }
                set_orientation(&handle.widget, *orientation)?;
            }
            NativeWidgetSetter::SetMinimum(minimum) => {
                self.ranges.entry(id).or_default().min = *minimum;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetMaximum(maximum) => {
                self.ranges.entry(id).or_default().max = *maximum;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetCurrent(current) => {
                self.ranges.entry(id).or_default().current = *current;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                apply_portable_style(&handle.widget, style)?;
            }
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetClassName(_)
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
        match &parent_handle.widget {
            WinUiOsWidget::Window(window) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI window child must be a UIElement-backed widget")
                })?;
                map_winui(
                    "failed to set WinUI window content",
                    window.SetContent(&child_element),
                )?;
            }
            WinUiOsWidget::StackPanel(panel) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI stack panel child must be a UIElement-backed widget")
                })?;
                self.insert_panel_child(
                    parent,
                    map_winui(
                        "failed to read WinUI stack panel children",
                        panel.Children(),
                    )?,
                    child,
                    child_element,
                    index,
                )?;
            }
            WinUiOsWidget::Grid(grid) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI grid child must be a UIElement-backed widget")
                })?;
                self.insert_panel_child(
                    parent,
                    map_winui("failed to read WinUI grid children", grid.Children())?,
                    child,
                    child_element,
                    index,
                )?;
            }
            WinUiOsWidget::ContentDialog(dialog) => {
                let child = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI content dialog child must be an inspectable widget")
                })?;
                map_winui(
                    "failed to set WinUI content dialog content",
                    dialog.SetContent(&child),
                )?;
            }
            WinUiOsWidget::ToolTip(tool_tip) => {
                let child = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI tooltip popover child must be an inspectable widget")
                })?;
                map_winui(
                    "failed to set WinUI tooltip popover content",
                    tool_tip.SetContent(&child),
                )?;
            }
            WinUiOsWidget::ComboBox(_) => {
                self.combo_children
                    .entry(parent)
                    .or_default()
                    .retain(|existing| *existing != child);
                let children = self.combo_children.entry(parent).or_default();
                let index = index.min(children.len());
                children.insert(index, child);
                self.combo_item_parents.insert(child, parent);
                self.combo_items.entry(child).or_insert_with(|| {
                    let label = child_handle.id.get().to_string();
                    WinUiComboBoxItem {
                        label: label.clone(),
                        value: label,
                        selected: false,
                    }
                });
                self.rebuild_combo_box(parent)?;
            }
            WinUiOsWidget::ListBox(list_box) => {
                let child_object = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI list child must be an inspectable native widget")
                })?;
                Self::insert_items_child(
                    &mut self.list_children,
                    parent,
                    map_winui("failed to read WinUI list box items", list_box.Items())?,
                    child,
                    child_object,
                    index,
                )?;
            }
            WinUiOsWidget::TabView(tab_view) => {
                let child_object = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI tab view child must be an inspectable native widget")
                })?;
                let collection =
                    map_winui("failed to read WinUI tab view items", tab_view.TabItems())?;
                let children = self.tab_children.entry(parent).or_default();
                if let Some(previous_index) =
                    children.iter().position(|existing| *existing == child)
                {
                    map_winui(
                        "failed to move existing WinUI tab view item",
                        collection.RemoveAt(to_u32(previous_index)?),
                    )?;
                    children.remove(previous_index);
                }
                let index = index.min(children.len());
                map_winui(
                    "failed to insert WinUI tab view item",
                    collection.InsertAt(to_u32(index)?, &child_object),
                )?;
                children.insert(index, child);
                self.tab_items
                    .entry(child)
                    .or_insert_with(|| WinUiTabItem::fallback(child));
                self.rebuild_tab_view(parent)?;
            }
            WinUiOsWidget::Button(button) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI button content",
                        button.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI checkbox content",
                        check_box.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::RadioButton(radio) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI radio button content",
                        radio.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::ComboBoxItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI combo box item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::ListBoxItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI list box item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::TabViewItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI tab view item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::TextBlock(_)
            | WinUiOsWidget::Separator(_)
            | WinUiOsWidget::TextBox(_)
            | WinUiOsWidget::Slider(_)
            | WinUiOsWidget::ProgressBar(_) => {}
        }
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        self.detach_child(id)?;
        match &handle.widget {
            WinUiOsWidget::Window(window) => {
                map_winui("failed to close WinUI window", window.Close())?;
            }
            WinUiOsWidget::ContentDialog(dialog) => {
                map_winui("failed to hide WinUI content dialog", dialog.Hide())?;
            }
            WinUiOsWidget::ToolTip(tool_tip) => {
                map_winui(
                    "failed to close WinUI tooltip popover",
                    tool_tip.SetIsOpen(false),
                )?;
            }
            _ => {}
        }
        self.widgets.remove(&id);
        self.combo_boxes.remove(&id);
        self.combo_items.remove(&id);
        self.combo_children.remove(&id);
        self.combo_selected_values.remove(&id);
        if let Ok(mut combo_values) = self.combo_values.lock() {
            combo_values.remove(&id);
        }
        self.container_children.remove(&id);
        self.list_children.remove(&id);
        self.tab_children.remove(&id);
        self.tab_items.remove(&id);
        self.tab_selected_values.remove(&id);
        if let Ok(mut tab_values) = self.tab_values.lock() {
            tab_values.remove(&id);
        }
        self.ranges.remove(&id);
        if self.root == Some(id) {
            self.root = None;
        }
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.root = Some(id);
        if let WinUiOsWidget::Window(window) = &handle.widget {
            map_winui("failed to activate WinUI window", window.Activate())?;
        }
        if let WinUiOsWidget::ToolTip(tool_tip) = &handle.widget {
            map_winui(
                "failed to open WinUI tooltip popover",
                tool_tip.SetIsOpen(true),
            )?;
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.events
            .lock()
            .map(|mut events| std::mem::take(&mut *events))
            .unwrap_or_default()
    }
}

fn set_label(widget: &WinUiOsWidget, value: Option<&str>) -> GuiResult<()> {
    let value = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::Window(window) => {
            map_winui(
                "failed to set WinUI window title",
                window.SetTitle(&hstr(value)),
            )?;
        }
        WinUiOsWidget::ContentDialog(dialog) => {
            let title = text_content(value)?;
            map_winui("failed to set WinUI dialog title", dialog.SetTitle(&title))?;
        }
        WinUiOsWidget::ToolTip(tool_tip) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI tooltip popover content",
                tool_tip.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TextBlock(text) => {
            map_winui(
                "failed to set WinUI text block text",
                text.SetText(&hstr(value)),
            )?;
        }
        WinUiOsWidget::Separator(_) => {}
        WinUiOsWidget::Button(button) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI button content",
                button.SetContent(&content),
            )?;
        }
        WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI checkbox content",
                check_box.SetContent(&content),
            )?;
        }
        WinUiOsWidget::RadioButton(radio) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI radio button content",
                radio.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TextBox(text_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI text box header",
                text_box.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI combo box header",
                combo_box.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::ComboBoxItem(item) => set_combo_box_item_content(item, value)?,
        WinUiOsWidget::ListBoxItem(item) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI list box item content",
                item.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TabViewItem(item) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI tab view item header",
                item.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::StackPanel(_)
        | WinUiOsWidget::ListBox(_)
        | WinUiOsWidget::TabView(_)
        | WinUiOsWidget::Grid(_)
        | WinUiOsWidget::Slider(_)
        | WinUiOsWidget::ProgressBar(_) => {}
    }
    Ok(())
}

fn set_value(
    surface: &mut WinUiNativeSurface,
    id: HostNodeId,
    widget: &WinUiOsWidget,
    value: Option<&str>,
) -> GuiResult<()> {
    let value_text = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::TextBlock(text) => {
            map_winui(
                "failed to set WinUI text block value",
                text.SetText(&hstr(value_text)),
            )?;
        }
        WinUiOsWidget::Separator(_) => {}
        WinUiOsWidget::TextBox(text_box) => {
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI text box value",
                    text_box.SetText(&hstr(value_text)),
                )
            })?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            surface.set_combo_value(id, combo_box, Some(value_text))?;
        }
        WinUiOsWidget::TabView(tab_view) => {
            surface.set_tab_value(id, tab_view, value)?;
        }
        _ => {}
    }
    Ok(())
}

fn set_placeholder(widget: &WinUiOsWidget, value: Option<&str>) -> GuiResult<()> {
    let value = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::TextBox(text_box) => {
            map_winui(
                "failed to set WinUI text box placeholder",
                text_box.SetPlaceholderText(&hstr(value)),
            )?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            map_winui(
                "failed to set WinUI combo box placeholder",
                combo_box.SetPlaceholderText(&hstr(value)),
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn set_checked(
    surface: &WinUiNativeSurface,
    widget: &WinUiOsWidget,
    checked: bool,
) -> GuiResult<()> {
    match widget {
        WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
            let value = bool_reference(checked)?;
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI checkbox checked state",
                    check_box.SetIsChecked(&value),
                )
            })?;
        }
        WinUiOsWidget::RadioButton(radio) => {
            let value = bool_reference(checked)?;
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI radio button checked state",
                    radio.SetIsChecked(&value),
                )
            })?;
        }
        _ => {}
    }
    Ok(())
}

fn set_selected(widget: &WinUiOsWidget, selected: bool) -> GuiResult<()> {
    match widget {
        WinUiOsWidget::ComboBoxItem(item) => {
            map_winui(
                "failed to set WinUI combo box item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        WinUiOsWidget::ListBoxItem(item) => {
            map_winui(
                "failed to set WinUI list box item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        WinUiOsWidget::TabViewItem(item) => {
            map_winui(
                "failed to set WinUI tab view item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn set_orientation(widget: &WinUiOsWidget, orientation: Option<A3sOrientation>) -> GuiResult<()> {
    let Some(orientation) = orientation else {
        return Ok(());
    };
    let orientation = match orientation {
        A3sOrientation::Horizontal => Controls::Orientation::Horizontal,
        A3sOrientation::Vertical => Controls::Orientation::Vertical,
    };
    match widget {
        WinUiOsWidget::StackPanel(panel) => {
            map_winui(
                "failed to set WinUI stack panel orientation",
                panel.SetOrientation(orientation),
            )?;
        }
        WinUiOsWidget::Slider(slider) => {
            map_winui(
                "failed to set WinUI slider orientation",
                slider.SetOrientation(orientation),
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn create_winui_separator(
    orientation: Option<A3sOrientation>,
) -> GuiResult<xaml::FrameworkElement> {
    let xaml = r##"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Background="#767676" IsHitTestVisible="False" />"##;
    let object = map_winui(
        "failed to load WinUI separator XAML",
        Markup::XamlReader::Load(&hstr(xaml)),
    )?;
    let separator = map_winui(
        "failed to cast WinUI separator to framework element",
        object.cast::<xaml::FrameworkElement>(),
    )?;
    set_winui_separator_orientation(&separator, orientation)?;
    Ok(separator)
}

fn set_winui_separator_orientation(
    separator: &xaml::FrameworkElement,
    orientation: Option<A3sOrientation>,
) -> GuiResult<()> {
    match orientation.unwrap_or(A3sOrientation::Horizontal) {
        A3sOrientation::Horizontal => {
            map_winui(
                "failed to reset WinUI separator width",
                separator.SetWidth(f64::NAN),
            )?;
            map_winui(
                "failed to set WinUI separator height",
                separator.SetHeight(1.0),
            )?;
            map_winui(
                "failed to set WinUI separator minimum width",
                separator.SetMinWidth(160.0),
            )?;
            map_winui(
                "failed to reset WinUI separator minimum height",
                separator.SetMinHeight(0.0),
            )?;
        }
        A3sOrientation::Vertical => {
            map_winui(
                "failed to set WinUI separator width",
                separator.SetWidth(1.0),
            )?;
            map_winui(
                "failed to reset WinUI separator height",
                separator.SetHeight(f64::NAN),
            )?;
            map_winui(
                "failed to reset WinUI separator minimum width",
                separator.SetMinWidth(0.0),
            )?;
            map_winui(
                "failed to set WinUI separator minimum height",
                separator.SetMinHeight(160.0),
            )?;
        }
    }
    Ok(())
}

fn apply_portable_style(widget: &WinUiOsWidget, style: &PortableStyle) -> GuiResult<()> {
    let Some(element) = widget.framework_element() else {
        return Ok(());
    };
    if let Some(value) = style.width.and_then(StyleLength::points) {
        map_winui("failed to set WinUI element width", element.SetWidth(value))?;
    }
    if let Some(value) = style.height.and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element height",
            element.SetHeight(value),
        )?;
    }
    if let Some(value) = style.min_width.and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element minimum width",
            element.SetMinWidth(value),
        )?;
    }
    if let Some(value) = style.min_height.and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element minimum height",
            element.SetMinHeight(value),
        )?;
    }
    if let Some(value) = style.max_width.and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element maximum width",
            element.SetMaxWidth(value),
        )?;
    }
    if let Some(value) = style.max_height.and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element maximum height",
            element.SetMaxHeight(value),
        )?;
    }
    Ok(())
}

fn register_press(
    id: HostNodeId,
    button: &Controls::Button,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let handler = RoutedEventHandler::new(move |_, _| {
        push_event(&events, NativeEvent::new(id, NativeEventKind::Press));
        Ok(())
    });
    map_winui(
        "failed to register WinUI button press handler",
        button.Click(&handler),
    )?;
    Ok(())
}

fn register_text_change(
    id: HostNodeId,
    text_box: &Controls::TextBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_text_box = text_box.clone();
    let handler = Controls::TextChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let value = event_text_box.Text()?.to_string();
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Change).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI text change handler",
        text_box.TextChanged(&handler),
    )?;
    Ok(())
}

fn register_toggle(
    id: HostNodeId,
    check_box: &Controls::CheckBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let checked_events = Arc::clone(events);
    let checked_suppressed = Arc::clone(&suppressed);
    let checked = RoutedEventHandler::new(move |_, _| {
        if !checked_suppressed.load(Ordering::SeqCst) {
            push_event(
                &checked_events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("true"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI checked handler",
        check_box.Checked(&checked),
    )?;

    let unchecked_events = Arc::clone(events);
    let unchecked = RoutedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            push_event(
                &unchecked_events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("false"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI unchecked handler",
        check_box.Unchecked(&unchecked),
    )?;
    Ok(())
}

fn register_radio_toggle(
    id: HostNodeId,
    radio: &Controls::RadioButton,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let handler = RoutedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("true"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI radio handler",
        radio.Checked(&handler),
    )?;
    Ok(())
}

fn register_combo_selection(
    id: HostNodeId,
    combo_box: &Controls::ComboBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
    values_by_combo: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_combo_box = combo_box.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let index = event_combo_box.SelectedIndex()?;
            let value = if index < 0 {
                String::new()
            } else {
                values_by_combo
                    .lock()
                    .ok()
                    .and_then(|values| values.get(&id).cloned())
                    .and_then(|values| values.get(index as usize).cloned())
                    .unwrap_or_default()
            };
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI combo box selection handler",
        combo_box.SelectionChanged(&handler),
    )?;
    Ok(())
}

fn register_list_selection(
    id: HostNodeId,
    list_box: &Controls::ListBox,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_list_box = list_box.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        let value = event_list_box
            .SelectedIndex()
            .map(|index| index.to_string())
            .unwrap_or_default();
        push_event(
            &events,
            NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
        );
        Ok(())
    });
    map_winui(
        "failed to register WinUI list selection handler",
        list_box.SelectionChanged(&handler),
    )?;
    Ok(())
}

fn register_tab_selection(
    id: HostNodeId,
    tab_view: &Controls::TabView,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
    values_by_tab_view: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_tab_view = tab_view.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let index = event_tab_view.SelectedIndex()?;
            let value = if index < 0 {
                String::new()
            } else {
                values_by_tab_view
                    .lock()
                    .ok()
                    .and_then(|values| values.get(&id).cloned())
                    .and_then(|values| values.get(index as usize).cloned())
                    .unwrap_or_else(|| index.to_string())
            };
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI tab view selection handler",
        tab_view.SelectionChanged(&handler),
    )?;
    Ok(())
}

fn register_range_change(
    id: HostNodeId,
    slider: &Controls::Slider,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_slider = slider.clone();
    let handler = Primitives::RangeBaseValueChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let value = event_slider.Value()?.to_string();
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Change).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI slider value handler",
        slider.ValueChanged(&handler),
    )?;
    Ok(())
}

fn register_focus_events(
    id: HostNodeId,
    widget: &WinUiOsWidget,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let Some(element) = widget.ui_element() else {
        return Ok(());
    };
    let focus_events = Arc::clone(events);
    let focus_handler = RoutedEventHandler::new(move |_, _| {
        push_event(&focus_events, NativeEvent::new(id, NativeEventKind::Focus));
        Ok(())
    });
    map_winui(
        "failed to register WinUI focus handler",
        element.GotFocus(&focus_handler),
    )?;

    let blur_events = Arc::clone(events);
    let blur_handler = RoutedEventHandler::new(move |_, _| {
        push_event(&blur_events, NativeEvent::new(id, NativeEventKind::Blur));
        Ok(())
    });
    map_winui(
        "failed to register WinUI blur handler",
        element.LostFocus(&blur_handler),
    )?;
    Ok(())
}

fn set_combo_box_item_content(item: &Controls::ComboBoxItem, value: &str) -> GuiResult<()> {
    let content = text_content(value)?;
    map_winui(
        "failed to set WinUI combo box item content",
        item.SetContent(&content),
    )
}

fn text_content(value: &str) -> GuiResult<Controls::TextBlock> {
    let text = map_winui(
        "failed to create WinUI text content",
        Controls::TextBlock::new(),
    )?;
    map_winui(
        "failed to set WinUI text content",
        text.SetText(&hstr(value)),
    )?;
    Ok(text)
}

fn bool_reference(value: bool) -> GuiResult<windows::Foundation::IReference<bool>> {
    let value = map_winui(
        "failed to box WinUI boolean value",
        PropertyValue::CreateBoolean(value),
    )?;
    map_winui("failed to cast WinUI boolean value", value.cast())
}

fn child_position(
    children_by_parent: &BTreeMap<HostNodeId, Vec<HostNodeId>>,
    child: HostNodeId,
) -> Option<(HostNodeId, usize)> {
    children_by_parent.iter().find_map(|(parent, children)| {
        children
            .iter()
            .position(|existing| *existing == child)
            .map(|index| (*parent, index))
    })
}

fn hstr(value: &str) -> HSTRING {
    HSTRING::from(value)
}

fn to_u32(value: usize) -> GuiResult<u32> {
    value
        .try_into()
        .map_err(|_| GuiError::host("WinUI collection index overflow"))
}

fn push_event(events: &WinUiEventQueue, event: NativeEvent) {
    if let Ok(mut events) = events.lock() {
        events.push(event);
    }
}

fn map_winui<T>(context: &str, result: windows_core::Result<T>) -> GuiResult<T> {
    result.map_err(|error| GuiError::host(format!("{context}: {error}")))
}
