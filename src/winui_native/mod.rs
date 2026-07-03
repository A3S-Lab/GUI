#![allow(unsafe_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use windows::Foundation::PropertyValue;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_KEYDOWN,
    WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
};
use windows_core::{Interface, HSTRING};
use winui3::bootstrap::PackageDependency;
use winui3::Microsoft::UI::Xaml as xaml;
use xaml::Controls::{self, Primitives};
use xaml::{Markup, RoutedEventHandler, Visibility};

use crate::app::{NativeRuntimeApp, NativeRuntimeEventResponse};
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation as A3sOrientation;
use crate::host::HostNodeId;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native_backends::winui::menu as winui_menu;
use crate::platform::{
    apply_widget_setter, NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig,
    NativeWidgetSetter, WinUiAdapter,
};
use crate::protocol::UiFrame;
use crate::winui::{winui_text_input_hints, WinUiWidgetKind};
use helpers::{child_position, map_winui, push_event, set_combo_box_item_content, to_u32};
use types::{config_is_password, config_is_textarea, WinUiRangeState, WinUiTextInputSizing};
pub use types::{WinUiComboBoxItem, WinUiOsHandle, WinUiOsWidget, WinUiTabItem};
use window::{
    apply_winui_window_portable_style, install_winui_window_close_event,
    set_winui_window_resizable, winui_window_is_open,
};

mod helpers;
mod surface;
mod types;
mod window;

const WINUI_TEXT_INPUT_DEFAULT_WIDTH: f64 = f64::NAN;
const WINUI_TEXT_INPUT_DEFAULT_HEIGHT: f64 = f64::NAN;
const WINUI_TEXT_INPUT_MIN_WIDTH: f64 = 80.0;
const WINUI_TEXT_INPUT_MIN_HEIGHT: f64 = 64.0;

type WinUiEventQueue = Arc<Mutex<Vec<NativeEvent>>>;
type WinUiFocusedNode = Arc<Mutex<Option<HostNodeId>>>;

pub type WinUiNativeSurfaceAdapter = SurfaceHandleAdapter<WinUiNativeSurface>;
pub type WinUiNativeSurfaceDriver = HandleWidgetDriver<WinUiNativeSurfaceAdapter>;
pub type WinUiNativeSurfaceCommandExecutor = DriverCommandExecutor<WinUiNativeSurfaceDriver>;
pub type WinUiRuntimeHost = CommandExecutingHost<WinUiAdapter, WinUiNativeSurfaceCommandExecutor>;
pub type WinUiRuntimeApp<S, F, R> = NativeRuntimeApp<WinUiRuntimeHost, S, F, R>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinUiEventWait {
    Poll,
    Wait,
}

#[derive(Debug)]
pub struct WinUiNativeSurface {
    _package_dependency: PackageDependency,
    root: Option<HostNodeId>,
    pending_auto_focus: Option<HostNodeId>,
    events: WinUiEventQueue,
    events_suppressed: Arc<AtomicBool>,
    focused_node: WinUiFocusedNode,
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
    text_inputs: BTreeMap<HostNodeId, WinUiTextInputSizing>,
    text_input_configs: BTreeMap<HostNodeId, NativeWidgetConfig>,
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
            pending_auto_focus: None,
            events: Arc::new(Mutex::new(Vec::new())),
            events_suppressed: Arc::new(AtomicBool::new(false)),
            focused_node: Arc::new(Mutex::new(None)),
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
            text_inputs: BTreeMap::new(),
            text_input_configs: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn root_window_open(&self) -> bool {
        let Some(root) = self.root else {
            return false;
        };
        let Some(widget) = self.widgets.get(&root) else {
            return false;
        };
        match widget {
            WinUiOsWidget::Window(window) => {
                winui_window_is_open(window).unwrap_or_else(|_| window.Visible().unwrap_or(false))
            }
            _ => true,
        }
    }

    fn key_event_target(&self) -> Option<HostNodeId> {
        self.focused_node
            .lock()
            .ok()
            .and_then(|focused| *focused)
            .filter(|node| self.widgets.contains_key(node))
            .or(self.root)
    }

    fn request_auto_focus(&mut self, id: HostNodeId) {
        if self.pending_auto_focus.is_none() {
            self.pending_auto_focus = Some(id);
        }
        self.clear_satisfied_auto_focus();
    }

    fn clear_satisfied_auto_focus(&mut self) {
        let Some(id) = self.pending_auto_focus else {
            return;
        };
        let is_focused = self.focused_node.lock().ok().and_then(|focused| *focused) == Some(id);
        if is_focused {
            self.pending_auto_focus = None;
        }
    }

    pub(crate) fn enqueue_key_message(&self, message: &MSG) {
        let Some(kind) = winui_key_event_kind(message.message) else {
            return;
        };
        let Some(node) = self.key_event_target() else {
            return;
        };
        push_event(
            &self.events,
            NativeEvent::new(node, kind).value(winui_key_value_from_virtual_key(message.wParam.0)),
        );
    }

    pub fn into_driver(self) -> WinUiNativeSurfaceDriver {
        HandleWidgetDriver::new(SurfaceHandleAdapter::new(self))
    }

    pub fn into_executor(self) -> WinUiNativeSurfaceCommandExecutor {
        DriverCommandExecutor::new(self.into_driver())
    }

    pub fn into_host(self) -> WinUiRuntimeHost {
        CommandExecutingHost::new(WinUiAdapter, self.into_executor())
    }

    fn apply_text_box_size_hint(
        &self,
        id: HostNodeId,
        text_box: &Controls::TextBox,
    ) -> GuiResult<()> {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return Ok(());
        };
        if sizing.explicit_width.is_some() && sizing.explicit_height.is_some() {
            return Ok(());
        }
        let element: xaml::FrameworkElement = map_winui(
            "failed to read WinUI text box framework element",
            text_box.cast(),
        )?;
        if sizing.explicit_width.is_none() {
            let width = sizing
                .hinted_width()
                .unwrap_or(WINUI_TEXT_INPUT_DEFAULT_WIDTH);
            map_winui(
                "failed to set WinUI text box hinted width",
                element.SetWidth(width),
            )?;
        }
        if sizing.explicit_height.is_none() {
            let height = sizing
                .hinted_height()
                .unwrap_or(WINUI_TEXT_INPUT_DEFAULT_HEIGHT);
            map_winui(
                "failed to set WinUI text box hinted height",
                element.SetHeight(height),
            )?;
        }
        Ok(())
    }

    fn apply_password_box_size_hint(
        &self,
        id: HostNodeId,
        password_box: &Controls::PasswordBox,
    ) -> GuiResult<()> {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return Ok(());
        };
        if sizing.explicit_width.is_some() && sizing.explicit_height.is_some() {
            return Ok(());
        }
        let element: xaml::FrameworkElement = map_winui(
            "failed to read WinUI password box framework element",
            password_box.cast(),
        )?;
        if sizing.explicit_width.is_none() {
            let width = sizing
                .hinted_width()
                .unwrap_or(WINUI_TEXT_INPUT_DEFAULT_WIDTH);
            map_winui(
                "failed to set WinUI password box hinted width",
                element.SetWidth(width),
            )?;
        }
        if sizing.explicit_height.is_none() {
            let height = sizing
                .hinted_height()
                .unwrap_or(WINUI_TEXT_INPUT_DEFAULT_HEIGHT);
            map_winui(
                "failed to set WinUI password box hinted height",
                element.SetHeight(height),
            )?;
        }
        Ok(())
    }

    fn apply_text_input_hints(&self, id: HostNodeId, widget: &WinUiOsWidget) -> GuiResult<()> {
        let Some(config) = self.text_input_configs.get(&id) else {
            return Ok(());
        };
        let hints = winui_text_input_hints(config);
        match widget {
            WinUiOsWidget::TextBox(text_box) => {
                if let Some(spellcheck) = hints.spellcheck_enabled {
                    map_winui(
                        "failed to set WinUI text box spell check hint",
                        text_box.SetIsSpellCheckEnabled(spellcheck),
                    )?;
                }
                if let Some(text_prediction) = hints.text_prediction_enabled {
                    map_winui(
                        "failed to set WinUI text box prediction hint",
                        text_box.SetIsTextPredictionEnabled(text_prediction),
                    )?;
                }
                map_winui(
                    "failed to set WinUI text box keyboard display hint",
                    text_box.SetPreventKeyboardDisplayOnProgrammaticFocus(
                        hints.prevent_keyboard_display_on_programmatic_focus,
                    ),
                )?;
                map_winui(
                    "failed to set WinUI text box color font hint",
                    text_box.SetIsColorFontEnabled(hints.color_font_enabled),
                )?;
            }
            WinUiOsWidget::PasswordBox(password_box) => {
                map_winui(
                    "failed to set WinUI password box keyboard display hint",
                    password_box.SetPreventKeyboardDisplayOnProgrammaticFocus(
                        hints.prevent_keyboard_display_on_programmatic_focus,
                    ),
                )?;
            }
            _ => {}
        }
        Ok(())
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
                map_winui(
                    "failed to set WinUI slider step frequency",
                    slider.SetStepFrequency(state.step()),
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

impl<S, F, R> NativeRuntimeApp<WinUiRuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<()>,
{
    pub fn winui(state: S, frame_builder: F, action_reducer: R) -> GuiResult<Self> {
        Ok(Self::new(
            WinUiNativeSurface::new()?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_winui_event(
        &mut self,
        wait: WinUiEventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        let mut responses = self.handle_pending_native_events()?;
        let pumped = {
            let surface = self
                .runtime_mut()
                .host_mut()
                .executor_mut()
                .driver_mut()
                .adapter_mut()
                .surface_mut();
            pump_winui_message(surface, wait)?
        };
        if pumped {
            responses.extend(self.handle_pending_native_events()?);
        }
        Ok(responses)
    }

    pub fn run_winui(&mut self) -> GuiResult<()> {
        self.run_winui_while(|_| true)
    }

    pub fn winui_root_window_open(&self) -> bool {
        self.runtime()
            .host()
            .executor()
            .driver()
            .adapter()
            .surface()
            .root_window_open()
    }

    pub fn run_winui_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.winui_root_window_open() && should_continue(self.state()) {
            self.pump_winui_event(WinUiEventWait::Wait)?;
        }
        Ok(())
    }
}

fn pump_winui_message(surface: &WinUiNativeSurface, wait: WinUiEventWait) -> GuiResult<bool> {
    let mut message = MSG::default();
    let received = match wait {
        WinUiEventWait::Poll => unsafe {
            PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool()
        },
        WinUiEventWait::Wait => {
            let result = unsafe { GetMessageW(&mut message, None, 0, 0) };
            if result.0 == -1 {
                return Err(GuiError::host("failed to read WinUI window message"));
            }
            result.as_bool()
        }
    };
    if !received {
        return Ok(false);
    }
    if message.message != WM_QUIT {
        surface.enqueue_key_message(&message);
        unsafe {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    Ok(true)
}

fn winui_key_event_kind(message: u32) -> Option<NativeEventKind> {
    match message {
        WM_KEYDOWN | WM_SYSKEYDOWN => Some(NativeEventKind::KeyDown),
        WM_KEYUP | WM_SYSKEYUP => Some(NativeEventKind::KeyUp),
        _ => None,
    }
}

fn winui_key_value_from_virtual_key(virtual_key: usize) -> String {
    match virtual_key {
        0x08 => "Backspace".to_string(),
        0x09 => "Tab".to_string(),
        0x0D => "Enter".to_string(),
        0x10 => "Shift".to_string(),
        0x11 => "Control".to_string(),
        0x12 => "Alt".to_string(),
        0x1B => "Escape".to_string(),
        0x20 => " ".to_string(),
        0x21 => "PageUp".to_string(),
        0x22 => "PageDown".to_string(),
        0x23 => "End".to_string(),
        0x24 => "Home".to_string(),
        0x25 => "ArrowLeft".to_string(),
        0x26 => "ArrowUp".to_string(),
        0x27 => "ArrowRight".to_string(),
        0x28 => "ArrowDown".to_string(),
        0x2D => "Insert".to_string(),
        0x2E => "Delete".to_string(),
        0x30..=0x39 | 0x41..=0x5A => char::from_u32(virtual_key as u32)
            .map(|value| value.to_string())
            .unwrap_or_else(|| format!("VirtualKey:{virtual_key}")),
        0x5B | 0x5C => "Meta".to_string(),
        0x60..=0x69 => (virtual_key - 0x60).to_string(),
        0x70..=0x87 => format!("F{}", virtual_key - 0x6F),
        _ => format!("VirtualKey:{virtual_key}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winui_key_event_kind_maps_key_messages() {
        assert_eq!(
            winui_key_event_kind(WM_KEYDOWN),
            Some(NativeEventKind::KeyDown)
        );
        assert_eq!(winui_key_event_kind(WM_KEYUP), Some(NativeEventKind::KeyUp));
        assert_eq!(
            winui_key_event_kind(WM_SYSKEYDOWN),
            Some(NativeEventKind::KeyDown)
        );
        assert_eq!(
            winui_key_event_kind(WM_SYSKEYUP),
            Some(NativeEventKind::KeyUp)
        );
        assert_eq!(winui_key_event_kind(WM_QUIT), None);
    }

    #[test]
    fn winui_key_value_normalizes_common_virtual_keys() {
        assert_eq!(winui_key_value_from_virtual_key(0x0D), "Enter");
        assert_eq!(winui_key_value_from_virtual_key(0x09), "Tab");
        assert_eq!(winui_key_value_from_virtual_key(0x20), " ");
        assert_eq!(winui_key_value_from_virtual_key(0x08), "Backspace");
        assert_eq!(winui_key_value_from_virtual_key(0x1B), "Escape");
        assert_eq!(winui_key_value_from_virtual_key(0x24), "Home");
        assert_eq!(winui_key_value_from_virtual_key(0x23), "End");
        assert_eq!(winui_key_value_from_virtual_key(0x25), "ArrowLeft");
        assert_eq!(winui_key_value_from_virtual_key(0x26), "ArrowUp");
        assert_eq!(winui_key_value_from_virtual_key(0x27), "ArrowRight");
        assert_eq!(winui_key_value_from_virtual_key(0x28), "ArrowDown");
        assert_eq!(winui_key_value_from_virtual_key(0x41), "A");
        assert_eq!(winui_key_value_from_virtual_key(0x31), "1");
        assert_eq!(winui_key_value_from_virtual_key(0x70), "F1");
        assert_eq!(winui_key_value_from_virtual_key(0xFF), "VirtualKey:255");
    }
}
