#![allow(unsafe_code)]

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{
    define_class, msg_send, sel, AnyThread, ClassType, DefinedClass, MainThreadMarker,
    MainThreadOnly,
};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSBorderType, NSBox, NSBoxType, NSButton, NSComboBox,
    NSComboBoxDelegate, NSControl, NSControlStateValue, NSControlStateValueOff,
    NSControlStateValueOn, NSControlTextEditingDelegate, NSEvent, NSEventMask, NSEventType, NSMenu,
    NSMenuItem, NSPanel, NSPopover, NSPopoverBehavior, NSProgressIndicator,
    NSProgressIndicatorStyle, NSResponder, NSScrollView, NSSearchField, NSSearchFieldDelegate,
    NSSecureTextField, NSSlider, NSStackView, NSStackViewDistribution, NSSwitch, NSTabView,
    NSTabViewDelegate, NSTabViewItem, NSTextField, NSTextFieldDelegate,
    NSUserInterfaceLayoutOrientation, NSView, NSViewController, NSWindow, NSWindowDelegate,
    NSWindowStyleMask,
};
use objc2_foundation::{
    NSDate, NSDefaultRunLoopMode, NSInteger, NSNotification, NSObject, NSObjectProtocol, NSPoint,
    NSRect, NSSize, NSString,
};

use crate::app::{NativeRuntimeApp, NativeRuntimeEventResponse};
use crate::appkit::{
    appkit_text_input_hints, AppKitTextInputHints, AppKitTextInputTrait, AppKitWidgetKind,
};
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation;
use crate::host::HostNodeId;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native_backends::appkit::menu::AppKitMenuRegistry;
use crate::platform::{
    apply_widget_setter, AppKitAdapter, NativeBackendKind, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetSetter,
};
use crate::protocol::UiFrame;
use crate::style::{OverflowMode, PortableStyle, StyleLength};

mod surface;

pub type AppKitNativeSurfaceAdapter = SurfaceHandleAdapter<AppKitNativeSurface>;
pub type AppKitNativeSurfaceDriver = HandleWidgetDriver<AppKitNativeSurfaceAdapter>;
pub type AppKitNativeSurfaceCommandExecutor = DriverCommandExecutor<AppKitNativeSurfaceDriver>;
pub type AppKitRuntimeHost =
    CommandExecutingHost<AppKitAdapter, AppKitNativeSurfaceCommandExecutor>;
pub type AppKitRuntimeApp<S, F, R> = NativeRuntimeApp<AppKitRuntimeHost, S, F, R>;

const MAX_APPKIT_SLIDER_TICK_MARKS: NSInteger = 101;
const APPKIT_TEXT_INPUT_DEFAULT_WIDTH: f64 = 120.0;
const APPKIT_TEXT_INPUT_DEFAULT_HEIGHT: f64 = 24.0;
const APPKIT_TEXT_INPUT_MIN_WIDTH: f64 = 80.0;

#[derive(Debug)]
pub struct AppKitNativeSurface {
    mtm: MainThreadMarker,
    _application: Retained<NSApplication>,
    root: Option<HostNodeId>,
    pending_auto_focus: Option<HostNodeId>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    focused_node: Rc<Cell<Option<HostNodeId>>>,
    closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
    widgets: BTreeMap<HostNodeId, AppKitOsWidget>,
    action_targets: BTreeMap<HostNodeId, Retained<AppKitActionTarget>>,
    window_delegates: BTreeMap<HostNodeId, Retained<AppKitWindowDelegate>>,
    responder_nodes: BTreeMap<usize, HostNodeId>,
    combo_boxes: BTreeMap<HostNodeId, Retained<NSComboBox>>,
    combo_items: BTreeMap<HostNodeId, AppKitComboBoxItem>,
    combo_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    combo_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    list_views: BTreeMap<HostNodeId, AppKitListViewState>,
    list_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    list_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    ranges: BTreeMap<HostNodeId, AppKitRangeState>,
    text_inputs: BTreeMap<HostNodeId, AppKitTextInputSizing>,
    text_input_configs: BTreeMap<HostNodeId, NativeWidgetConfig>,
    menus: AppKitMenuRegistry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppKitEventWait {
    Poll,
    Wait,
}

impl AppKitNativeSurface {
    pub fn new() -> GuiResult<Self> {
        let mtm = MainThreadMarker::new().ok_or_else(|| {
            GuiError::host("AppKit native surface must be created on main thread")
        })?;
        let application = NSApplication::sharedApplication(mtm);
        application.finishLaunching();
        Ok(Self {
            mtm,
            _application: application,
            root: None,
            pending_auto_focus: None,
            events: Rc::new(RefCell::new(Vec::new())),
            focused_node: Rc::new(Cell::new(None)),
            closed_windows: Rc::new(RefCell::new(BTreeSet::new())),
            widgets: BTreeMap::new(),
            action_targets: BTreeMap::new(),
            window_delegates: BTreeMap::new(),
            responder_nodes: BTreeMap::new(),
            combo_boxes: BTreeMap::new(),
            combo_items: BTreeMap::new(),
            combo_children: BTreeMap::new(),
            combo_item_parents: BTreeMap::new(),
            list_views: BTreeMap::new(),
            list_children: BTreeMap::new(),
            list_item_parents: BTreeMap::new(),
            ranges: BTreeMap::new(),
            text_inputs: BTreeMap::new(),
            text_input_configs: BTreeMap::new(),
            menus: AppKitMenuRegistry::default(),
        })
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn application(&self) -> &NSApplication {
        &self._application
    }

    pub fn root_window_open(&self) -> bool {
        self.root
            .map(|root| !self.closed_windows.borrow().contains(&root))
            .unwrap_or(false)
    }

    pub fn into_driver(self) -> AppKitNativeSurfaceDriver {
        HandleWidgetDriver::new(SurfaceHandleAdapter::new(self))
    }

    pub fn into_executor(self) -> AppKitNativeSurfaceCommandExecutor {
        DriverCommandExecutor::new(self.into_driver())
    }

    pub fn into_host(
        self,
    ) -> CommandExecutingHost<AppKitAdapter, AppKitNativeSurfaceCommandExecutor> {
        CommandExecutingHost::new(AppKitAdapter, self.into_executor())
    }

    fn apply_text_input_size(&self, id: HostNodeId, text_field: &NSTextField) {
        let Some(sizing) = self.text_inputs.get(&id).copied() else {
            return;
        };
        if sizing.explicit_width.is_some() && sizing.explicit_height.is_some() {
            return;
        }
        let view = text_field.as_super().as_super();
        let current = view.frame().size;
        let width = if sizing.explicit_width.is_some() {
            current.width
        } else {
            sizing
                .hinted_width()
                .unwrap_or(APPKIT_TEXT_INPUT_DEFAULT_WIDTH)
        };
        let height = if sizing.explicit_height.is_some() {
            current.height
        } else if let Some(height) = sizing.hinted_height() {
            height
        } else if current.height > 0.0 {
            current.height
        } else {
            APPKIT_TEXT_INPUT_DEFAULT_HEIGHT
        };
        view.setFrameSize(NSSize::new(width, height));
    }

    fn apply_text_input_hints(&self, id: HostNodeId, widget: &AppKitOsWidget) {
        let Some(config) = self.text_input_configs.get(&id) else {
            return;
        };
        let hints = appkit_text_input_hints(config);
        match widget {
            AppKitOsWidget::TextField(text_field) => apply_text_field_hints(text_field, hints),
            AppKitOsWidget::SearchField(text_field) => {
                apply_text_field_hints(text_field.as_super(), hints);
            }
            AppKitOsWidget::SecureTextField(text_field) => {
                apply_text_field_hints(text_field.as_super(), hints);
            }
            _ => {}
        }
    }

    fn register_responder(&mut self, id: HostNodeId, widget: &AppKitOsWidget) {
        if let Some(responder) = widget.as_responder() {
            self.responder_nodes.insert(responder_key(responder), id);
        }
    }

    fn unregister_responder(&mut self, widget: &AppKitOsWidget) {
        if let Some(responder) = widget.as_responder() {
            self.responder_nodes.remove(&responder_key(responder));
        }
    }

    fn register_view_responder(&mut self, id: HostNodeId, view: &NSView) {
        self.responder_nodes
            .insert(responder_key(view.as_super()), id);
    }

    fn unregister_view_responder(&mut self, view: &NSView) {
        self.responder_nodes.remove(&responder_key(view.as_super()));
    }

    fn request_auto_focus(&mut self, id: HostNodeId, widget: &AppKitOsWidget) {
        if self.pending_auto_focus.is_none() {
            self.pending_auto_focus = Some(id);
        }
        self.focus_auto_focus_widget(id, widget);
    }

    fn focus_pending_auto_focus(&mut self) {
        let Some(id) = self.pending_auto_focus else {
            return;
        };
        let Some(widget) = self.widgets.get(&id).cloned() else {
            return;
        };
        self.focus_auto_focus_widget(id, &widget);
    }

    fn focus_auto_focus_widget(&mut self, id: HostNodeId, widget: &AppKitOsWidget) {
        if self.pending_auto_focus != Some(id) || !focus_appkit_widget(widget) {
            return;
        }
        self.pending_auto_focus = None;
        self.focused_node.set(Some(id));
    }

    fn node_for_key_event(&self, event: &NSEvent) -> Option<HostNodeId> {
        event
            .window(self.mtm)
            .and_then(|window| window.firstResponder())
            .and_then(|responder| {
                self.responder_nodes
                    .get(&responder_key(&responder))
                    .copied()
            })
            .or_else(|| self.focused_node.get())
            .or(self.root)
    }

    fn enqueue_key_event(&self, event: &NSEvent) {
        let kind = match event.r#type() {
            event_type if event_type == NSEventType::KeyDown => NativeEventKind::KeyDown,
            event_type if event_type == NSEventType::KeyUp => NativeEventKind::KeyUp,
            _ => return,
        };

        if let Some(node) = self.node_for_key_event(event) {
            self.events
                .borrow_mut()
                .push(NativeEvent::new(node, kind).value(appkit_key_value(event)));
        }
    }

    fn update_option_item_label(
        &mut self,
        id: HostNodeId,
        fallback: &AppKitComboBoxItem,
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
        self.rebuild_for_option_item(id)
    }

    fn update_option_item_value(
        &mut self,
        id: HostNodeId,
        fallback: &AppKitComboBoxItem,
        value: String,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
        self.rebuild_for_option_item(id)
    }

    fn update_option_item_selected(
        &mut self,
        id: HostNodeId,
        fallback: &AppKitComboBoxItem,
        selected: bool,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
        self.rebuild_for_option_item(id)
    }

    fn rebuild_for_option_item(&mut self, item: HostNodeId) -> GuiResult<()> {
        if let Some(parent) = self.combo_item_parents.get(&item).copied() {
            self.rebuild_combo_box(parent)?;
        }
        if let Some(parent) = self.list_item_parents.get(&item).copied() {
            self.rebuild_list_view(parent)?;
        }
        Ok(())
    }

    fn rebuild_combo_box(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(combo_box) = self.combo_boxes.get(&id).cloned() else {
            return Ok(());
        };
        let previous_value = combo_box_selected_value(&combo_box);
        let children = self.combo_children.get(&id).cloned().unwrap_or_default();
        combo_box.removeAllItems();

        let mut selected_value = None;
        for (index, child) in children.iter().enumerate() {
            let Some(item) = self.combo_items.get(child) else {
                continue;
            };
            let value = ns_string(&item.value);
            unsafe {
                combo_box.insertItemWithObjectValue_atIndex(
                    ns_string_as_any(&value),
                    index
                        .try_into()
                        .map_err(|_| GuiError::host("AppKit combo box item index overflow"))?,
                );
            }
            if item.selected && selected_value.is_none() {
                selected_value = Some(item.value.clone());
            }
        }

        let selected_value = selected_value.or_else(|| {
            if previous_value.is_empty() {
                None
            } else {
                Some(previous_value)
            }
        });
        set_combo_box_value(&combo_box, selected_value.as_deref());
        Ok(())
    }

    fn rebuild_list_view(&mut self, id: HostNodeId) -> GuiResult<()> {
        let Some(state) = self.list_views.get(&id).cloned() else {
            return Ok(());
        };
        let previous_value = list_view_selected_value(&state);
        let children = self.list_children.get(&id).cloned().unwrap_or_default();

        for row in state.rows.borrow_mut().drain(..) {
            state.stack_view.removeArrangedSubview(row.button_view());
            self.unregister_view_responder(row.button_view());
            row.button_view().removeFromSuperview();
        }

        let mut rows = Vec::new();
        for child in children {
            let Some(item) = self.combo_items.get(&child).cloned() else {
                continue;
            };
            let selected =
                item.selected || (!previous_value.is_empty() && item.value == previous_value);
            let row = AppKitListRow::new(
                id,
                item,
                selected,
                self.events.clone(),
                self.focused_node.clone(),
                self.mtm,
            );
            let index = rows
                .len()
                .try_into()
                .map_err(|_| GuiError::host("AppKit list row index overflow"))?;
            state
                .stack_view
                .insertArrangedSubview_atIndex(row.button_view(), index);
            self.register_view_responder(id, row.button_view());
            rows.push(row);
        }
        *state.rows.borrow_mut() = rows;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct AppKitWindowDelegateIvars {
    node: HostNodeId,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppKitWindowDelegateIvars]
    #[derive(Debug)]
    struct AppKitWindowDelegate;

    unsafe impl NSObjectProtocol for AppKitWindowDelegate {}

    unsafe impl NSWindowDelegate for AppKitWindowDelegate {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            true
        }

        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
            self.ivars()
                .events
                .borrow_mut()
                .push(NativeEvent::new(self.ivars().node, NativeEventKind::Close));
            self.ivars()
                .closed_windows
                .borrow_mut()
                .insert(self.ivars().node);
        }
    }
);

impl AppKitWindowDelegate {
    fn new(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitWindowDelegateIvars {
            node,
            events,
            closed_windows,
        });
        unsafe { msg_send![super(this), init] }
    }
}

impl AppKitEventWait {
    fn expiration(self) -> objc2::rc::Retained<NSDate> {
        match self {
            Self::Poll => NSDate::distantPast(),
            Self::Wait => NSDate::distantFuture(),
        }
    }
}

impl<S, F, R> NativeRuntimeApp<AppKitRuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<()>,
{
    pub fn appkit(state: S, frame_builder: F, action_reducer: R) -> GuiResult<Self> {
        Ok(Self::new(
            AppKitNativeSurface::new()?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_appkit_event(
        &mut self,
        wait: AppKitEventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_appkit_event_while(wait, |_| true)
    }

    pub fn pump_appkit_event_while(
        &mut self,
        wait: AppKitEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        let mut responses = self.handle_pending_native_events_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(responses);
        }
        let expiration = wait.expiration();
        let event = self
            .runtime()
            .host()
            .executor()
            .driver()
            .adapter()
            .surface()
            .application()
            .nextEventMatchingMask_untilDate_inMode_dequeue(
                NSEventMask::Any,
                Some(&expiration),
                unsafe { NSDefaultRunLoopMode },
                true,
            );

        if let Some(event) = event {
            let surface = self
                .runtime()
                .host()
                .executor()
                .driver()
                .adapter()
                .surface();
            surface.enqueue_key_event(&event);
            surface.application().sendEvent(&event);
            surface.application().updateWindows();
            responses.extend(self.handle_pending_native_events_while(&mut should_continue)?);
        }

        Ok(responses)
    }

    pub fn run_appkit(&mut self) -> GuiResult<()> {
        self.run_appkit_while(|_| true)
    }

    pub fn appkit_root_window_open(&self) -> bool {
        self.runtime()
            .host()
            .executor()
            .driver()
            .adapter()
            .surface()
            .root_window_open()
    }

    pub fn run_appkit_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.appkit_root_window_open() && should_continue(self.state()) {
            self.pump_appkit_event_while(AppKitEventWait::Wait, &mut should_continue)?;
        }
        Ok(())
    }
}

impl AppKitComboBoxItem {
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

#[derive(Debug, Clone)]
struct AppKitActionTargetIvars {
    node: HostNodeId,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    focused_node: Rc<Cell<Option<HostNodeId>>>,
    selection_value: Option<String>,
    max_length: Cell<Option<u32>>,
    suppress_text_change: Cell<bool>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppKitActionTargetIvars]
    #[derive(Debug)]
    struct AppKitActionTarget;

    impl AppKitActionTarget {
        #[unsafe(method(a3sGuiPress:))]
        fn press(&self, _sender: &AnyObject) {
            let kind = if self.ivars().selection_value.is_some() {
                NativeEventKind::SelectionChange
            } else {
                NativeEventKind::Press
            };
            let mut event = NativeEvent::new(self.ivars().node, kind);
            if let Some(value) = &self.ivars().selection_value {
                event = event.value(value.clone());
            }
            self.ivars().events.borrow_mut().push(event);
        }

        #[unsafe(method(a3sGuiToggle:))]
        fn toggle(&self, sender: &AnyObject) {
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Toggle)
                    .value(control_checked_value(sender).to_string()),
            );
        }

        #[unsafe(method(a3sGuiChange:))]
        fn change(&self, sender: &AnyObject) {
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Change)
                    .value(control_double_value(sender).to_string()),
            );
        }
    }

    unsafe impl NSObjectProtocol for AppKitActionTarget {}

    unsafe impl NSControlTextEditingDelegate for AppKitActionTarget {
        #[unsafe(method(controlTextDidBeginEditing:))]
        fn control_text_did_begin_editing(&self, _notification: &NSNotification) {
            self.ivars().focused_node.set(Some(self.ivars().node));
            self.ivars()
                .events
                .borrow_mut()
                .push(NativeEvent::new(self.ivars().node, NativeEventKind::Focus));
        }

        #[unsafe(method(controlTextDidChange:))]
        fn control_text_did_change(&self, notification: &NSNotification) {
            if self.ivars().suppress_text_change.get() {
                return;
            }

            let value = notification
                .object()
                .and_then(|object| object.downcast::<NSControl>().ok())
                .map(|control| {
                    let raw_value = control.stringValue().to_string();
                    let value = truncate_to_max_length(&raw_value, self.max_length());
                    if value != raw_value {
                        self.ivars().suppress_text_change.set(true);
                        control.setStringValue(&ns_string(&value));
                        self.ivars().suppress_text_change.set(false);
                    }
                    value
                })
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Change).value(value),
            );
        }

        #[unsafe(method(controlTextDidEndEditing:))]
        fn control_text_did_end_editing(&self, _notification: &NSNotification) {
            if self.ivars().focused_node.get() == Some(self.ivars().node) {
                self.ivars().focused_node.set(None);
            }
            self.ivars()
                .events
                .borrow_mut()
                .push(NativeEvent::new(self.ivars().node, NativeEventKind::Blur));
        }
    }

    unsafe impl NSTextFieldDelegate for AppKitActionTarget {}

    unsafe impl NSSearchFieldDelegate for AppKitActionTarget {}

    unsafe impl NSComboBoxDelegate for AppKitActionTarget {
        #[unsafe(method(comboBoxSelectionDidChange:))]
        fn combo_box_selection_did_change(&self, notification: &NSNotification) {
            let value = notification
                .object()
                .and_then(|object| object.downcast::<NSComboBox>().ok())
                .map(|combo_box| combo_box_selected_value(&combo_box))
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::SelectionChange)
                    .value(value),
            );
        }
    }

    unsafe impl NSTabViewDelegate for AppKitActionTarget {
        #[unsafe(method(tabView:didSelectTabViewItem:))]
        fn tab_view_did_select_tab_view_item(
            &self,
            _tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        ) {
            let value = tab_view_item
                .map(|item| item.label().to_string())
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::SelectionChange)
                    .value(value),
            );
        }
    }
);

impl AppKitActionTarget {
    fn new(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            focused_node,
            selection_value: None,
            max_length: Cell::new(None),
            suppress_text_change: Cell::new(false),
        });
        unsafe { msg_send![super(this), init] }
    }

    fn new_selection(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        selection_value: String,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            focused_node,
            selection_value: Some(selection_value),
            max_length: Cell::new(None),
            suppress_text_change: Cell::new(false),
        });
        unsafe { msg_send![super(this), init] }
    }

    fn as_any_object(&self) -> &AnyObject {
        self.as_super().as_super()
    }

    fn max_length(&self) -> Option<u32> {
        self.ivars().max_length.get()
    }

    fn set_max_length(&self, max_length: Option<u32>) {
        self.ivars().max_length.set(max_length);
    }
}

#[derive(Debug, Clone)]
pub struct AppKitOsHandle {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub selected: bool,
    pub widget: AppKitOsWidget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppKitComboBoxItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

#[derive(Debug, Clone)]
struct AppKitListViewState {
    stack_view: Retained<NSStackView>,
    rows: Rc<RefCell<Vec<AppKitListRow>>>,
}

#[derive(Debug, Clone)]
struct AppKitListRow {
    button: Retained<NSButton>,
    _target: Retained<AppKitActionTarget>,
    value: String,
}

impl AppKitListRow {
    fn new(
        parent: HostNodeId,
        item: AppKitComboBoxItem,
        selected: bool,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Self {
        let title = ns_string(&item.label);
        let target = AppKitActionTarget::new_selection(
            parent,
            events,
            focused_node,
            item.value.clone(),
            mtm,
        );
        let button = unsafe {
            NSButton::buttonWithTitle_target_action(
                &title,
                Some(target.as_any_object()),
                Some(sel!(a3sGuiPress:)),
                mtm,
            )
        };
        button.setBordered(false);
        button.setState(appkit_state(selected));
        Self {
            button,
            _target: target,
            value: item.value,
        }
    }

    fn button_view(&self) -> &NSView {
        self.button.as_super().as_super()
    }
}

#[derive(Debug, Clone)]
pub struct AppKitPopoverState {
    popover: Retained<NSPopover>,
    content_view_controller: Retained<NSViewController>,
    content_view: Retained<NSView>,
}

#[derive(Debug, Clone)]
pub struct AppKitScrollViewState {
    scroll_view: Retained<NSScrollView>,
    stack_view: Retained<NSStackView>,
}

#[derive(Debug, Clone)]
pub enum AppKitOsWidget {
    Window(Retained<NSWindow>),
    Panel(Retained<NSPanel>),
    Popover(AppKitPopoverState),
    Menu(Retained<NSMenu>),
    MenuItem(Retained<NSMenuItem>),
    View(Retained<NSView>),
    StackView(Retained<NSStackView>),
    Button(Retained<NSButton>),
    Switch(Retained<NSSwitch>),
    ComboBox(Retained<NSComboBox>),
    ComboBoxItem(AppKitComboBoxItem),
    ListView(Retained<NSScrollView>),
    ScrollView(AppKitScrollViewState),
    Slider(Retained<NSSlider>),
    ProgressIndicator(Retained<NSProgressIndicator>),
    TabView(Retained<NSTabView>),
    TabViewItem(Retained<NSTabViewItem>),
    Box(Retained<NSBox>),
    TextField(Retained<NSTextField>),
    SearchField(Retained<NSSearchField>),
    SecureTextField(Retained<NSSecureTextField>),
}

impl AppKitOsWidget {
    fn as_responder(&self) -> Option<&NSResponder> {
        match self {
            AppKitOsWidget::Window(window) => Some(window.as_super()),
            AppKitOsWidget::Panel(panel) => Some(panel.as_super().as_super()),
            _ => self.as_view().map(NSView::as_super),
        }
    }

    fn as_view(&self) -> Option<&NSView> {
        match self {
            AppKitOsWidget::Window(_)
            | AppKitOsWidget::Panel(_)
            | AppKitOsWidget::Popover(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_) => None,
            AppKitOsWidget::View(view) => Some(view),
            AppKitOsWidget::StackView(stack_view) => Some(stack_view.as_super()),
            AppKitOsWidget::Button(button) => Some(button.as_super().as_super()),
            AppKitOsWidget::Switch(switch) => Some(switch.as_super().as_super()),
            AppKitOsWidget::ComboBox(combo_box) => Some(combo_box.as_super().as_super().as_super()),
            AppKitOsWidget::ListView(scroll_view) => Some(scroll_view.as_super()),
            AppKitOsWidget::ScrollView(state) => Some(state.scroll_view.as_super()),
            AppKitOsWidget::Slider(slider) => Some(slider.as_super().as_super()),
            AppKitOsWidget::ProgressIndicator(progress) => Some(progress.as_super()),
            AppKitOsWidget::TabView(tab_view) => Some(tab_view.as_super()),
            AppKitOsWidget::Box(box_) => Some(box_.as_super()),
            AppKitOsWidget::TextField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::SearchField(text_field) => {
                Some(text_field.as_super().as_super().as_super())
            }
            AppKitOsWidget::SecureTextField(text_field) => {
                Some(text_field.as_super().as_super().as_super())
            }
            AppKitOsWidget::ComboBoxItem(_) | AppKitOsWidget::TabViewItem(_) => None,
        }
    }

    fn as_control(&self) -> Option<&objc2_app_kit::NSControl> {
        match self {
            AppKitOsWidget::Button(button) => Some(button.as_super()),
            AppKitOsWidget::Switch(switch) => Some(switch.as_super()),
            AppKitOsWidget::ComboBox(combo_box) => Some(combo_box.as_super().as_super()),
            AppKitOsWidget::Slider(slider) => Some(slider.as_super()),
            AppKitOsWidget::TextField(text_field) => Some(text_field.as_super()),
            AppKitOsWidget::SearchField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::SecureTextField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::Window(_)
            | AppKitOsWidget::Panel(_)
            | AppKitOsWidget::Popover(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::View(_)
            | AppKitOsWidget::StackView(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::ListView(_)
            | AppKitOsWidget::ScrollView(_)
            | AppKitOsWidget::TabView(_)
            | AppKitOsWidget::TabViewItem(_)
            | AppKitOsWidget::Box(_)
            | AppKitOsWidget::ProgressIndicator(_) => None,
        }
    }
}

fn focus_appkit_widget(widget: &AppKitOsWidget) -> bool {
    let Some(responder) = widget.as_responder() else {
        return false;
    };

    if let Some(view) = widget.as_view() {
        return view
            .window()
            .is_some_and(|window| window.makeFirstResponder(Some(responder)));
    }

    match widget {
        AppKitOsWidget::Window(window) => window.makeFirstResponder(Some(responder)),
        AppKitOsWidget::Panel(panel) => panel.as_super().makeFirstResponder(Some(responder)),
        _ => false,
    }
}

fn config_rect(config: &NativeWidgetConfig, default_width: f64, default_height: f64) -> NSRect {
    NSRect::new(
        NSPoint::new(0.0, 0.0),
        config_size(config, default_width, default_height),
    )
}

fn config_rect_for_orientation(
    config: &NativeWidgetConfig,
    orientation: Orientation,
    horizontal_width: f64,
    horizontal_height: f64,
    vertical_width: f64,
    vertical_height: f64,
) -> NSRect {
    let (width, height) = match orientation {
        Orientation::Horizontal => (horizontal_width, horizontal_height),
        Orientation::Vertical => (vertical_width, vertical_height),
    };
    config_rect(config, width, height)
}

fn separator_size(orientation: Orientation) -> NSSize {
    match orientation {
        Orientation::Horizontal => NSSize::new(160.0, 1.0),
        Orientation::Vertical => NSSize::new(1.0, 160.0),
    }
}

fn slider_size_for_orientation(config: &NativeWidgetConfig, orientation: Orientation) -> NSSize {
    config_rect_for_orientation(config, orientation, 180.0, 24.0, 24.0, 180.0).size
}

fn apply_slider_orientation(slider: &NSSlider, orientation: Orientation) {
    slider.setVertical(matches!(orientation, Orientation::Vertical));
}

fn config_size(config: &NativeWidgetConfig, default_width: f64, default_height: f64) -> NSSize {
    let size = config.portable_style.native_size_constraints();
    let width = size.width.unwrap_or(default_width);
    let height = size.height.unwrap_or(default_height);
    NSSize::new(width, height)
}

fn config_text_input_size(config: &NativeWidgetConfig) -> NSSize {
    let sizing = AppKitTextInputSizing::from_config(config);
    let width = sizing
        .explicit_width
        .or_else(|| sizing.hinted_width())
        .unwrap_or(APPKIT_TEXT_INPUT_DEFAULT_WIDTH);
    let height = sizing
        .explicit_height
        .or_else(|| sizing.hinted_height())
        .unwrap_or(APPKIT_TEXT_INPUT_DEFAULT_HEIGHT);
    NSSize::new(width, height)
}

fn apply_text_field_hints(text_field: &NSTextField, hints: AppKitTextInputHints) {
    if let Some(enabled) = hints.automatic_text_completion_enabled {
        text_field.setAutomaticTextCompletionEnabled(enabled);
    }
    text_field.setAllowsCharacterPickerTouchBarItem(hints.character_picker_enabled);
    set_spell_checking_trait(text_field, hints.spell_checking);
    set_autocorrection_trait(text_field, hints.autocorrection);
    set_text_replacement_trait(text_field, hints.text_replacement);
    set_text_completion_trait(text_field, hints.text_completion);
    set_inline_prediction_trait(text_field, hints.inline_prediction);
}

fn set_spell_checking_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setSpellCheckingType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setSpellCheckingType: value];
        }
    }
}

fn set_autocorrection_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setAutocorrectionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setAutocorrectionType: value];
        }
    }
}

fn set_text_replacement_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setTextReplacementType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setTextReplacementType: value];
        }
    }
}

fn set_text_completion_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setTextCompletionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setTextCompletionType: value];
        }
    }
}

fn set_inline_prediction_trait(text_field: &NSTextField, value: AppKitTextInputTrait) {
    if text_field.respondsToSelector(sel!(setInlinePredictionType:)) {
        let value = appkit_text_input_trait_value(value);
        unsafe {
            let _: () = msg_send![text_field, setInlinePredictionType: value];
        }
    }
}

fn appkit_text_input_trait_value(value: AppKitTextInputTrait) -> NSInteger {
    match value {
        AppKitTextInputTrait::Default => 0,
        AppKitTextInputTrait::No => 1,
        AppKitTextInputTrait::Yes => 2,
    }
}

fn apply_window_portable_style(window: &NSWindow, style: &crate::style::PortableStyle) {
    let size = style.native_size_constraints();
    if size.width.is_some() || size.height.is_some() {
        let current = window
            .contentView()
            .map(|view| view.frame().size)
            .unwrap_or_else(|| window.contentLayoutRect().size);
        window.setContentSize(NSSize::new(
            size.width.unwrap_or(current.width),
            size.height.unwrap_or(current.height),
        ));
    }

    if size.min_width.is_some() || size.min_height.is_some() {
        let current = window.minSize();
        window.setMinSize(NSSize::new(
            size.min_width.unwrap_or(current.width),
            size.min_height.unwrap_or(current.height),
        ));
    }

    if size.max_width.is_some() || size.max_height.is_some() {
        let current = window.maxSize();
        window.setMaxSize(NSSize::new(
            size.max_width.unwrap_or(current.width),
            size.max_height.unwrap_or(current.height),
        ));
    }
}

fn apply_scroll_view_layout(state: &AppKitScrollViewState, style: &PortableStyle) {
    state
        .scroll_view
        .setHasVerticalScroller(appkit_vertical_scroll_enabled_for_style(style));
    state
        .scroll_view
        .setHasHorizontalScroller(appkit_horizontal_scroll_enabled_for_style(style));
    apply_stack_view_layout(&state.stack_view, style, None);

    let size = style.native_size_constraints();
    if size.width.is_some() || size.height.is_some() {
        let current = state.stack_view.frame().size;
        state.stack_view.setFrameSize(NSSize::new(
            size.width.unwrap_or(current.width.max(120.0)),
            size.height.unwrap_or(current.height.max(32.0)),
        ));
    }
}

fn apply_stack_view_layout(
    stack_view: &NSStackView,
    style: &PortableStyle,
    orientation: Option<Orientation>,
) {
    if let Some(orientation) = orientation.or(style.flex_direction) {
        stack_view.setOrientation(appkit_stack_orientation(orientation));
    }
    let gap = style
        .gap
        .as_ref()
        .and_then(StyleLength::points)
        .unwrap_or(0.0);
    unsafe {
        let _: () = msg_send![stack_view, setSpacing: gap];
    }
}

fn appkit_vertical_scroll_enabled_for_style(style: &PortableStyle) -> bool {
    scroll_enabled(style.overflow_y)
        || scroll_enabled(style.overflow_block)
        || (!scroll_enabled(style.overflow_x) && !scroll_enabled(style.overflow_inline))
}

fn appkit_horizontal_scroll_enabled_for_style(style: &PortableStyle) -> bool {
    scroll_enabled(style.overflow_x) || scroll_enabled(style.overflow_inline)
}

fn scroll_enabled(value: Option<OverflowMode>) -> bool {
    matches!(value, Some(OverflowMode::Auto | OverflowMode::Scroll))
}

fn appkit_stack_orientation(orientation: Orientation) -> NSUserInterfaceLayoutOrientation {
    match orientation {
        Orientation::Horizontal => NSUserInterfaceLayoutOrientation::Horizontal,
        Orientation::Vertical => NSUserInterfaceLayoutOrientation::Vertical,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct AppKitRangeState {
    min: Option<f64>,
    max: Option<f64>,
    current: Option<f64>,
    step: Option<f64>,
}

impl AppKitRangeState {
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

    fn step(self) -> Option<f64> {
        self.step.filter(|value| value.is_finite() && *value > 0.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct AppKitTextInputSizing {
    rows: Option<u32>,
    cols: Option<u32>,
    size: Option<u32>,
    explicit_width: Option<f64>,
    explicit_height: Option<f64>,
}

impl AppKitTextInputSizing {
    fn from_config(config: &NativeWidgetConfig) -> Self {
        let size = config.portable_style.native_size_constraints();
        Self {
            rows: config.rows,
            cols: config.cols,
            size: config.size,
            explicit_width: size.width,
            explicit_height: size.height,
        }
    }

    fn hinted_width(self) -> Option<f64> {
        if self.explicit_width.is_some() {
            return None;
        }
        self.size
            .or(self.cols)
            .filter(|value| *value > 0)
            .map(|columns| APPKIT_TEXT_INPUT_MIN_WIDTH.max(columns as f64 * 8.0 + 28.0))
    }

    fn hinted_height(self) -> Option<f64> {
        if self.explicit_height.is_some() {
            return None;
        }
        self.rows
            .filter(|value| *value > 0)
            .map(|rows| (rows as f64 * 20.0 + 18.0).max(64.0))
    }
}

fn config_is_textarea(config: &NativeWidgetConfig) -> bool {
    config
        .metadata
        .get(HTML_TAG_METADATA_KEY)
        .is_some_and(|tag| tag == "textarea")
}

fn config_is_password(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("password"))
}

fn config_is_search(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("search"))
}

fn apply_progress_range(progress: &NSProgressIndicator, range: AppKitRangeState) {
    progress.setMinValue(range.lower());
    progress.setMaxValue(range.upper());
    progress.setDoubleValue(range.current());
}

fn apply_slider_step(slider: &NSSlider, range: AppKitRangeState) {
    let Some(step) = range.step() else {
        slider.setAllowsTickMarkValuesOnly(false);
        slider.setNumberOfTickMarks(0);
        slider.setAltIncrementValue(0.0);
        return;
    };

    slider.setAltIncrementValue(step);
    let span = range.upper() - range.lower();
    let ticks = (span / step).round() + 1.0;
    if span.is_finite()
        && span > 0.0
        && ticks >= 2.0
        && ticks <= MAX_APPKIT_SLIDER_TICK_MARKS as f64
    {
        slider.setNumberOfTickMarks(ticks as NSInteger);
        slider.setAllowsTickMarkValuesOnly(true);
    } else {
        slider.setNumberOfTickMarks(0);
        slider.setAllowsTickMarkValuesOnly(false);
    }
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

fn set_control_string_value(control: &NSControl, value: &str, max_length: Option<u32>) {
    control.setStringValue(&ns_string(&truncate_to_max_length(value, max_length)));
}

fn apply_control_max_length(control: &NSControl, max_length: Option<u32>) {
    let value = control.stringValue().to_string();
    set_control_string_value(control, &value, max_length);
}

fn parse_f64(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
}

fn ns_string(value: &str) -> Retained<NSString> {
    NSString::from_str(value)
}

fn ns_string_as_any(value: &NSString) -> &AnyObject {
    value.as_super().as_super()
}

fn responder_key(responder: &NSResponder) -> usize {
    responder as *const NSResponder as usize
}

fn appkit_key_value(event: &NSEvent) -> String {
    let characters = event
        .charactersIgnoringModifiers()
        .or_else(|| event.characters())
        .map(|characters| characters.to_string());
    appkit_key_value_from_parts(event.keyCode(), characters.as_deref())
}

fn appkit_key_value_from_parts(key_code: u16, characters: Option<&str>) -> String {
    match key_code {
        36 | 76 => return "Enter".to_string(),
        48 => return "Tab".to_string(),
        49 => return " ".to_string(),
        51 => return "Backspace".to_string(),
        53 => return "Escape".to_string(),
        115 => return "Home".to_string(),
        116 => return "PageUp".to_string(),
        117 => return "Delete".to_string(),
        119 => return "End".to_string(),
        121 => return "PageDown".to_string(),
        123 => return "ArrowLeft".to_string(),
        124 => return "ArrowRight".to_string(),
        125 => return "ArrowDown".to_string(),
        126 => return "ArrowUp".to_string(),
        _ => {}
    }

    let raw = characters.unwrap_or_default();
    match raw {
        "\r" | "\n" => "Enter".to_string(),
        "\t" | "\u{19}" => "Tab".to_string(),
        "\u{1b}" => "Escape".to_string(),
        "\u{7f}" | "\u{8}" => "Backspace".to_string(),
        " " => " ".to_string(),
        value => crate::event::native_key_value(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_to_max_length_limits_unicode_scalar_values() {
        let unicode_value = format!("a{}{}b", '\u{e9}', '\u{4e2d}');
        let unicode_prefix = format!("a{}{}", '\u{e9}', '\u{4e2d}');
        assert_eq!(truncate_to_max_length("abcdef", Some(3)), "abc");
        assert_eq!(
            truncate_to_max_length(&unicode_value, Some(3)),
            unicode_prefix
        );
        assert_eq!(truncate_to_max_length("abc", None), "abc");
        assert_eq!(truncate_to_max_length("abc", Some(0)), "");
    }

    #[test]
    fn appkit_key_value_normalizes_special_keys() {
        assert_eq!(appkit_key_value_from_parts(36, Some("\r")), "Enter");
        assert_eq!(appkit_key_value_from_parts(48, Some("\t")), "Tab");
        assert_eq!(appkit_key_value_from_parts(49, Some(" ")), " ");
        assert_eq!(appkit_key_value_from_parts(51, Some("\u{7f}")), "Backspace");
        assert_eq!(appkit_key_value_from_parts(53, Some("\u{1b}")), "Escape");
        assert_eq!(appkit_key_value_from_parts(123, None), "ArrowLeft");
        assert_eq!(appkit_key_value_from_parts(124, None), "ArrowRight");
        assert_eq!(appkit_key_value_from_parts(125, None), "ArrowDown");
        assert_eq!(appkit_key_value_from_parts(126, None), "ArrowUp");
        assert_eq!(appkit_key_value_from_parts(0, Some("a")), "a");
    }
}

fn set_combo_box_value(combo_box: &NSComboBox, value: Option<&str>) {
    let value = ns_string(value.unwrap_or(""));
    unsafe {
        let object = ns_string_as_any(&value);
        if combo_box.indexOfItemWithObjectValue(object) >= 0 {
            combo_box.selectItemWithObjectValue(Some(object));
        }
    }
    combo_box.as_super().as_super().setStringValue(&value);
}

fn combo_box_selected_value(combo_box: &NSComboBox) -> String {
    combo_box
        .objectValueOfSelectedItem()
        .and_then(|value| value.downcast::<NSString>().ok())
        .map(|value| value.to_string())
        .unwrap_or_else(|| combo_box.as_super().as_super().stringValue().to_string())
}

fn list_view_selected_value(state: &AppKitListViewState) -> String {
    state
        .rows
        .borrow()
        .iter()
        .find(|row| row.button.state() == NSControlStateValueOn)
        .map(|row| row.value.clone())
        .unwrap_or_default()
}

fn appkit_state(value: bool) -> NSControlStateValue {
    if value {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    }
}

fn set_button_checked(button: &NSButton, value: bool) {
    button.setState(appkit_state(value));
}

fn set_switch_checked(switch: &NSSwitch, value: bool) {
    switch.setState(appkit_state(value));
}

fn control_checked_value(sender: &AnyObject) -> bool {
    sender
        .downcast_ref::<NSButton>()
        .map(|button| button.state() == NSControlStateValueOn)
        .or_else(|| {
            sender
                .downcast_ref::<NSSwitch>()
                .map(|switch| switch.state() == NSControlStateValueOn)
        })
        .unwrap_or(false)
}

fn control_double_value(sender: &AnyObject) -> f64 {
    sender
        .downcast_ref::<NSControl>()
        .map(NSControl::doubleValue)
        .unwrap_or_default()
}
