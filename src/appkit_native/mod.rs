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
    NSApplication, NSApplicationActivationOptions, NSApplicationActivationPolicy,
    NSAutoresizingMaskOptions, NSBackingStoreType, NSBorderType, NSBox, NSBoxType, NSButton,
    NSColor, NSComboBox, NSComboBoxDelegate, NSControl, NSControlStateValue,
    NSControlStateValueOff, NSControlStateValueOn, NSControlTextEditingDelegate, NSEvent,
    NSEventMask, NSEventModifierFlags, NSEventType, NSFont, NSLayoutAttribute, NSLayoutConstraint,
    NSLayoutConstraintOrientation, NSLayoutPriorityDefaultLow, NSLayoutPriorityRequired,
    NSLayoutRelation, NSMenu, NSMenuItem, NSPanel, NSPointingDeviceType, NSPopover,
    NSPopoverBehavior, NSProgressIndicator, NSProgressIndicatorStyle, NSResponder,
    NSRunningApplication, NSScrollView, NSSearchField, NSSearchFieldDelegate, NSSecureTextField,
    NSSlider, NSStackView, NSStackViewDistribution, NSSwitch, NSTabView, NSTabViewDelegate,
    NSTabViewItem, NSTextField, NSTextFieldDelegate, NSTrackingArea, NSTrackingAreaOptions,
    NSUserInterfaceLayoutOrientation, NSView, NSViewController, NSWindow, NSWindowDelegate,
    NSWindowStyleMask,
};
use objc2_foundation::{
    NSDate, NSDefaultRunLoopMode, NSEdgeInsets, NSInteger, NSNotification, NSObject,
    NSObjectProtocol, NSPoint, NSRect, NSRectEdge, NSRunLoop, NSRunLoopCommonModes, NSSize,
    NSString, NSTimer,
};

use crate::app::{
    ActionPropagation, NativeRuntimeApp, NativeRuntimeEventBatch, NativeRuntimeEventResponse,
};
use crate::appkit::{
    appkit_text_input_hints, AppKitTextInputHints, AppKitTextInputTrait, AppKitWidgetKind,
};
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{virtual_press_events, NativeEvent, NativeEventKind};
use crate::geometry::Orientation;
use crate::host::HostNodeId;
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native_backends::appkit::menu::AppKitMenuRegistry;
use crate::overlay_position::{
    OverlayCrossAlignment, OverlayPlacementAxis, OverlayPositionRequest,
};
use crate::platform::{
    apply_widget_setter, AppKitAdapter, NativeBackendKind, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetSetter,
};
use crate::protocol::UiFrame;
use crate::style::{
    EdgeInsets, FontWeight, OverflowMode, PortableStyle, StyleColor, StyleLength, TextAlign,
};

mod action;
mod controls;
mod hierarchy;
mod interaction;
mod mount;
mod propagation;
mod style;
mod surface;
mod types;
mod update;
mod window;

use action::AppKitActionTarget;
use controls::*;
use style::*;
use surface::{appkit_horizontal_scroll_enabled, appkit_vertical_scroll_enabled, set_widget_title};
use types::{
    focus_appkit_view, focus_appkit_widget, install_window_content_view, AppKitListRow,
    AppKitListViewState, AppKitSizeConstraints,
};
pub use types::{
    AppKitComboBoxItem, AppKitOsHandle, AppKitOsWidget, AppKitPopoverState, AppKitScrollViewState,
};
#[cfg(test)]
use window::push_window_close_event_once;
use window::{flipped_stack_view, flipped_view, AppKitWindowDelegate};

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
const APPKIT_LIST_ROW_HEIGHT: f64 = 28.0;
const APPKIT_LIST_MIN_HEIGHT: f64 = 32.0;
const APPKIT_LIST_DEFAULT_WIDTH: f64 = 240.0;

#[derive(Debug)]
pub struct AppKitNativeSurface {
    mtm: MainThreadMarker,
    _application: Retained<NSApplication>,
    root: Option<HostNodeId>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    focused_node: Rc<Cell<Option<HostNodeId>>>,
    activation_contexts: interaction::AppKitActivationContexts,
    interaction_nodes: BTreeMap<HostNodeId, interaction::AppKitInteractionRegistration>,
    keyboard_presses: RefCell<crate::event::KeyboardPressState>,
    pointer_press: Rc<RefCell<Option<interaction::AppKitPointerPress>>>,
    hovered_pointer: RefCell<Option<interaction::AppKitHoverTarget>>,
    closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
    dialog_visible: BTreeMap<HostNodeId, bool>,
    popover_visible: BTreeMap<HostNodeId, bool>,
    popover_anchors: BTreeMap<HostNodeId, HostNodeId>,
    popover_positions: BTreeMap<HostNodeId, OverlayPositionRequest>,
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
    size_constraints: BTreeMap<HostNodeId, AppKitSizeConstraints>,
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
        application.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        application.finishLaunching();
        #[allow(deprecated)]
        {
            application.activateIgnoringOtherApps(true);
        }
        Ok(Self {
            mtm,
            _application: application,
            root: None,
            events: Rc::new(RefCell::new(Vec::new())),
            focused_node: Rc::new(Cell::new(None)),
            activation_contexts: Rc::new(RefCell::new(BTreeMap::new())),
            interaction_nodes: BTreeMap::new(),
            keyboard_presses: RefCell::new(crate::event::KeyboardPressState::default()),
            pointer_press: Rc::new(RefCell::new(None)),
            hovered_pointer: RefCell::new(None),
            closed_windows: Rc::new(RefCell::new(BTreeSet::new())),
            dialog_visible: BTreeMap::new(),
            popover_visible: BTreeMap::new(),
            popover_anchors: BTreeMap::new(),
            popover_positions: BTreeMap::new(),
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
            size_constraints: BTreeMap::new(),
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
        } else {
            APPKIT_TEXT_INPUT_DEFAULT_HEIGHT
        };
        view.setFrameSize(NSSize::new(width, height));
    }

    fn apply_native_size_constraints(
        &mut self,
        id: HostNodeId,
        view: &NSView,
        style: &PortableStyle,
    ) {
        let constraints = style.native_size_constraints();
        if constraints.width.is_none()
            && constraints.height.is_none()
            && constraints.min_width.is_none()
            && constraints.min_height.is_none()
            && constraints.max_width.is_none()
            && constraints.max_height.is_none()
        {
            self.clear_native_size_constraints(id);
            return;
        }

        self.clear_native_size_constraints(id);

        let current = view.frame().size;
        let width = constraints.width.unwrap_or(current.width.max(120.0));
        let height = constraints.height.unwrap_or(current.height.max(32.0));
        view.setFrameSize(NSSize::new(width, height));
        view.setTranslatesAutoresizingMaskIntoConstraints(false);
        view.setContentCompressionResistancePriority_forOrientation(
            NSLayoutPriorityRequired,
            NSLayoutConstraintOrientation::Horizontal,
        );
        view.setContentCompressionResistancePriority_forOrientation(
            NSLayoutPriorityRequired,
            NSLayoutConstraintOrientation::Vertical,
        );
        view.setContentHuggingPriority_forOrientation(
            if constraints.width.is_some() {
                NSLayoutPriorityRequired
            } else {
                NSLayoutPriorityDefaultLow
            },
            NSLayoutConstraintOrientation::Horizontal,
        );
        view.setContentHuggingPriority_forOrientation(
            if constraints.height.is_some() {
                NSLayoutPriorityRequired
            } else {
                NSLayoutPriorityDefaultLow
            },
            NSLayoutConstraintOrientation::Vertical,
        );

        let mut active_constraints = Vec::new();
        if let Some(width) = constraints.width {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Width,
                NSLayoutRelation::Equal,
                width,
            ));
        }
        if let Some(height) = constraints.height {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Height,
                NSLayoutRelation::Equal,
                height,
            ));
        }
        if let Some(width) = constraints.min_width {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Width,
                NSLayoutRelation::GreaterThanOrEqual,
                width,
            ));
        }
        if let Some(height) = constraints.min_height {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Height,
                NSLayoutRelation::GreaterThanOrEqual,
                height,
            ));
        }
        if let Some(width) = constraints.max_width {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Width,
                NSLayoutRelation::LessThanOrEqual,
                width,
            ));
        }
        if let Some(height) = constraints.max_height {
            active_constraints.push(size_constraint(
                view,
                NSLayoutAttribute::Height,
                NSLayoutRelation::LessThanOrEqual,
                height,
            ));
        }

        if !active_constraints.is_empty() {
            for constraint in &active_constraints {
                constraint.setActive(true);
            }
            self.size_constraints
                .insert(id, AppKitSizeConstraints { active_constraints });
        }
        view.invalidateIntrinsicContentSize();
        view.setNeedsLayout(true);
    }

    fn apply_native_minimum_size(
        &mut self,
        id: HostNodeId,
        view: &NSView,
        min_width: f64,
        min_height: f64,
    ) {
        self.clear_native_size_constraints(id);
        view.setFrameSize(NSSize::new(
            view.frame().size.width.max(min_width),
            view.frame().size.height.max(min_height),
        ));
        view.setTranslatesAutoresizingMaskIntoConstraints(false);
        let active_constraints = vec![
            size_constraint(
                view,
                NSLayoutAttribute::Width,
                NSLayoutRelation::GreaterThanOrEqual,
                min_width,
            ),
            size_constraint(
                view,
                NSLayoutAttribute::Height,
                NSLayoutRelation::GreaterThanOrEqual,
                min_height,
            ),
        ];
        for constraint in &active_constraints {
            constraint.setActive(true);
        }
        self.size_constraints
            .insert(id, AppKitSizeConstraints { active_constraints });
        view.invalidateIntrinsicContentSize();
        view.setNeedsLayout(true);
    }

    fn clear_native_size_constraints(&mut self, id: HostNodeId) {
        let Some(previous) = self.size_constraints.remove(&id) else {
            return;
        };
        for constraint in previous.active_constraints {
            constraint.setActive(false);
        }
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

    fn node_for_key_event(&self, event: &NSEvent) -> Option<HostNodeId> {
        event
            .window(self.mtm)
            .and_then(|window| window.firstResponder())
            .and_then(|responder| self.node_for_responder(&responder))
            .or_else(|| self.focused_node.get())
            .or(self.root)
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
        let label = if label.trim().is_empty() {
            item.value.clone()
        } else {
            label
        };
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

    fn update_option_item_style(
        &mut self,
        id: HostNodeId,
        fallback: &AppKitComboBoxItem,
        style: PortableStyle,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .style = style;
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

    fn update_option_item_visible(
        &mut self,
        id: HostNodeId,
        fallback: &AppKitComboBoxItem,
        visible: bool,
    ) -> GuiResult<()> {
        self.combo_items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .visible = visible;
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
        let mut index = 0_usize;
        for child in &children {
            let Some(item) = self.combo_items.get(child) else {
                continue;
            };
            if !item.visible {
                continue;
            }
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
            index = index.saturating_add(1);
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
            if !item.visible {
                continue;
            }
            let selected =
                item.selected || (!previous_value.is_empty() && item.value == previous_value);
            let row = AppKitListRow::new(
                child,
                id,
                item,
                selected,
                self.events.clone(),
                self.activation_contexts.clone(),
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
            self.register_view_responder(child, row.button_view());
            rows.push(row);
        }
        *state.rows.borrow_mut() = rows;
        if let Some(focused) = self
            .focused_node
            .get()
            .filter(|focused| self.list_item_parents.get(focused).copied() == Some(id))
        {
            if let Some(row) = state.rows.borrow().iter().find(|row| row.node == focused) {
                focus_appkit_view(row.button_view());
            }
        }
        if let Some(AppKitOsWidget::ListView(scroll_view)) = self.widgets.get(&id) {
            apply_list_view_layout(scroll_view, &state, &state.style);
        }
        Ok(())
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

fn appkit_runtime_root_window_open<S, F, R>(
    app: &NativeRuntimeApp<AppKitRuntimeHost, S, F, R>,
) -> bool {
    app.runtime()
        .host()
        .executor()
        .driver()
        .adapter()
        .surface()
        .root_window_open()
}

fn pump_appkit_os_event<S, F, R>(
    app: &mut NativeRuntimeApp<AppKitRuntimeHost, S, F, R>,
    wait: AppKitEventWait,
) -> GuiResult<bool> {
    let expiration = wait.expiration();
    let event = app
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

    let Some(event) = event else {
        return Ok(false);
    };
    let surface = app.runtime().host().executor().driver().adapter().surface();
    let previous_focus = surface.focus_before_event(&event);
    let prevent_default = surface.enqueue_interaction_event(&event);
    if !prevent_default {
        surface.application().sendEvent(&event);
    }
    surface.finish_interaction_event(&event, previous_focus);
    surface.application().updateWindows();
    Ok(true)
}

impl<S, F, R> NativeRuntimeApp<AppKitRuntimeHost, S, F, R> {
    pub fn appkit_root_window_open(&self) -> bool {
        appkit_runtime_root_window_open(self)
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
        self.pump_appkit_event_batch(wait)
            .map(|batch| batch.responses)
    }

    pub fn pump_appkit_event_while(
        &mut self,
        wait: AppKitEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_appkit_event_batch_while(wait, &mut should_continue)
            .map(|batch| batch.responses)
    }

    pub fn pump_appkit_event_batch(
        &mut self,
        wait: AppKitEventWait,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        self.pump_appkit_event_batch_while(wait, |_| true)
    }

    pub fn pump_appkit_event_batch_while(
        &mut self,
        wait: AppKitEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        let mut batch = self.handle_pending_native_event_batch_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(batch);
        }
        if pump_appkit_os_event(self, wait)? {
            batch.extend(self.handle_pending_native_event_batch_while(&mut should_continue)?);
        }

        Ok(batch)
    }

    pub fn run_appkit(&mut self) -> GuiResult<()> {
        self.run_appkit_while(|_| true)
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
        let label = non_empty_config_string(config.label.as_ref())
            .or_else(|| non_empty_config_string(config.value.as_ref()))
            .unwrap_or_default();
        let value = non_empty_config_string(config.value.as_ref()).unwrap_or_else(|| label.clone());
        Self {
            label,
            value,
            selected: config.selected,
            visible: config.visible,
            style: config.portable_style.clone(),
        }
    }
}

fn non_empty_config_string(value: Option<&String>) -> Option<String> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
}
