#![allow(unsafe_code)]

use std::cell::RefCell;
use std::collections::BTreeMap;
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
    NSControlStateValueOn, NSControlTextEditingDelegate, NSMenu, NSMenuItem, NSPanel, NSPopover,
    NSPopoverBehavior, NSProgressIndicator, NSProgressIndicatorStyle, NSScrollView, NSSlider,
    NSStackView, NSStackViewDistribution, NSSwitch, NSTabView, NSTabViewDelegate, NSTabViewItem,
    NSTextField, NSTextFieldDelegate, NSUserInterfaceLayoutOrientation, NSView, NSViewController,
    NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use crate::appkit::AppKitWidgetKind;
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeWidgetSurface,
    SurfaceHandleAdapter,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{NativeEvent, NativeEventKind};
use crate::geometry::Orientation;
use crate::host::HostNodeId;
use crate::native_backends::appkit::menu::AppKitMenuRegistry;
use crate::platform::{
    AppKitAdapter, NativeBackendKind, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetSetter,
};
use crate::style::StyleLength;

pub type AppKitNativeSurfaceAdapter = SurfaceHandleAdapter<AppKitNativeSurface>;
pub type AppKitNativeSurfaceDriver = HandleWidgetDriver<AppKitNativeSurfaceAdapter>;
pub type AppKitNativeSurfaceCommandExecutor = DriverCommandExecutor<AppKitNativeSurfaceDriver>;

#[derive(Debug)]
pub struct AppKitNativeSurface {
    mtm: MainThreadMarker,
    _application: Retained<NSApplication>,
    root: Option<HostNodeId>,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    action_targets: BTreeMap<HostNodeId, Retained<AppKitActionTarget>>,
    combo_boxes: BTreeMap<HostNodeId, Retained<NSComboBox>>,
    combo_items: BTreeMap<HostNodeId, AppKitComboBoxItem>,
    combo_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    combo_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    list_views: BTreeMap<HostNodeId, AppKitListViewState>,
    list_children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    list_item_parents: BTreeMap<HostNodeId, HostNodeId>,
    menus: AppKitMenuRegistry,
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
            events: Rc::new(RefCell::new(Vec::new())),
            action_targets: BTreeMap::new(),
            combo_boxes: BTreeMap::new(),
            combo_items: BTreeMap::new(),
            combo_children: BTreeMap::new(),
            combo_item_parents: BTreeMap::new(),
            list_views: BTreeMap::new(),
            list_children: BTreeMap::new(),
            list_item_parents: BTreeMap::new(),
            menus: AppKitMenuRegistry::default(),
        })
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
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
            row.button_view().removeFromSuperview();
        }

        let mut rows = Vec::new();
        for child in children {
            let Some(item) = self.combo_items.get(&child).cloned() else {
                continue;
            };
            let selected =
                item.selected || (!previous_value.is_empty() && item.value == previous_value);
            let row = AppKitListRow::new(id, item, selected, self.events.clone(), self.mtm);
            let index = rows
                .len()
                .try_into()
                .map_err(|_| GuiError::host("AppKit list row index overflow"))?;
            state
                .stack_view
                .insertArrangedSubview_atIndex(row.button_view(), index);
            rows.push(row);
        }
        *state.rows.borrow_mut() = rows;
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
    selection_value: Option<String>,
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
            self.ivars()
                .events
                .borrow_mut()
                .push(NativeEvent::new(self.ivars().node, NativeEventKind::Focus));
        }

        #[unsafe(method(controlTextDidChange:))]
        fn control_text_did_change(&self, notification: &NSNotification) {
            let value = notification
                .object()
                .and_then(|object| object.downcast::<NSControl>().ok())
                .map(|control| control.stringValue().to_string())
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Change).value(value),
            );
        }

        #[unsafe(method(controlTextDidEndEditing:))]
        fn control_text_did_end_editing(&self, _notification: &NSNotification) {
            self.ivars()
                .events
                .borrow_mut()
                .push(NativeEvent::new(self.ivars().node, NativeEventKind::Blur));
        }
    }

    unsafe impl NSTextFieldDelegate for AppKitActionTarget {}

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
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            selection_value: None,
        });
        unsafe { msg_send![super(this), init] }
    }

    fn new_selection(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        selection_value: String,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            selection_value: Some(selection_value),
        });
        unsafe { msg_send![super(this), init] }
    }

    fn as_any_object(&self) -> &AnyObject {
        self.as_super().as_super()
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
        mtm: MainThreadMarker,
    ) -> Self {
        let title = ns_string(&item.label);
        let target = AppKitActionTarget::new_selection(parent, events, item.value.clone(), mtm);
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
    Slider(Retained<NSSlider>),
    ProgressIndicator(Retained<NSProgressIndicator>),
    TabView(Retained<NSTabView>),
    TabViewItem(Retained<NSTabViewItem>),
    Box(Retained<NSBox>),
    TextField(Retained<NSTextField>),
}

impl AppKitOsWidget {
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
            AppKitOsWidget::Slider(slider) => Some(slider.as_super().as_super()),
            AppKitOsWidget::ProgressIndicator(progress) => Some(progress.as_super()),
            AppKitOsWidget::TabView(tab_view) => Some(tab_view.as_super()),
            AppKitOsWidget::Box(box_) => Some(box_.as_super()),
            AppKitOsWidget::TextField(text_field) => Some(text_field.as_super().as_super()),
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
            AppKitOsWidget::Window(_)
            | AppKitOsWidget::Panel(_)
            | AppKitOsWidget::Popover(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::View(_)
            | AppKitOsWidget::StackView(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::ListView(_)
            | AppKitOsWidget::TabView(_)
            | AppKitOsWidget::TabViewItem(_)
            | AppKitOsWidget::Box(_)
            | AppKitOsWidget::ProgressIndicator(_) => None,
        }
    }
}

impl NativeWidgetSurface for AppKitNativeSurface {
    type Handle = AppKitOsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let kind = AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let config = blueprint.config();
        let widget = match kind {
            AppKitWidgetKind::Window => {
                let rect = config_rect(&config, 640.0, 480.0);
                let style = if config.enabled {
                    NSWindowStyleMask::Titled
                        | NSWindowStyleMask::Closable
                        | NSWindowStyleMask::Miniaturizable
                        | NSWindowStyleMask::Resizable
                } else {
                    NSWindowStyleMask::Titled
                };
                let window = unsafe {
                    NSWindow::initWithContentRect_styleMask_backing_defer(
                        NSWindow::alloc(self.mtm),
                        rect,
                        style,
                        NSBackingStoreType::Buffered,
                        false,
                    )
                };
                AppKitOsWidget::Window(window)
            }
            AppKitWidgetKind::Panel => {
                let rect = config_rect(&config, 420.0, 280.0);
                let style = NSWindowStyleMask::Titled
                    | NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Resizable;
                let panel = NSPanel::initWithContentRect_styleMask_backing_defer(
                    NSPanel::alloc(self.mtm),
                    rect,
                    style,
                    NSBackingStoreType::Buffered,
                    false,
                );
                panel.setTitle(&ns_string(config.label.as_deref().unwrap_or("")));
                AppKitOsWidget::Panel(panel)
            }
            AppKitWidgetKind::Popover => {
                let size = config_size(&config, 320.0, 220.0);
                let content_view = NSView::initWithFrame(
                    NSView::alloc(self.mtm),
                    NSRect::new(NSPoint::new(0.0, 0.0), size),
                );
                let content_view_controller = NSViewController::new(self.mtm);
                content_view_controller.setView(&content_view);
                if let Some(label) = config.label.as_deref() {
                    content_view_controller.setTitle(Some(&ns_string(label)));
                }

                let popover = NSPopover::new(self.mtm);
                popover.setBehavior(NSPopoverBehavior::Transient);
                popover.setAnimates(true);
                popover.setContentSize(size);
                popover.setContentViewController(Some(&content_view_controller));
                AppKitOsWidget::Popover(AppKitPopoverState {
                    popover,
                    content_view_controller,
                    content_view,
                })
            }
            AppKitWidgetKind::Menu => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let menu = NSMenu::initWithTitle(NSMenu::alloc(self.mtm), &title);
                self.menus.register_menu(id, menu.clone());
                AppKitOsWidget::Menu(menu)
            }
            AppKitWidgetKind::MenuItem => {
                let title = ns_string(
                    config
                        .label
                        .as_deref()
                        .or(config.value.as_deref())
                        .unwrap_or(""),
                );
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let menu_item = unsafe {
                    NSMenuItem::initWithTitle_action_keyEquivalent(
                        NSMenuItem::alloc(self.mtm),
                        &title,
                        Some(sel!(a3sGuiPress:)),
                        &ns_string(""),
                    )
                };
                unsafe {
                    menu_item.setTarget(Some(target.as_any_object()));
                }
                menu_item.setEnabled(config.enabled);
                menu_item.setState(appkit_state(config.selected));
                self.action_targets.insert(id, target);
                AppKitOsWidget::MenuItem(menu_item)
            }
            AppKitWidgetKind::View => {
                let view = NSView::initWithFrame(
                    NSView::alloc(self.mtm),
                    config_rect(&config, 320.0, 240.0),
                );
                AppKitOsWidget::View(view)
            }
            AppKitWidgetKind::Button => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let button = unsafe {
                    NSButton::buttonWithTitle_target_action(
                        &title,
                        Some(target.as_any_object()),
                        Some(sel!(a3sGuiPress:)),
                        self.mtm,
                    )
                };
                self.action_targets.insert(id, target);
                AppKitOsWidget::Button(button)
            }
            AppKitWidgetKind::Checkbox => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let checkbox = unsafe {
                    NSButton::checkboxWithTitle_target_action(
                        &title,
                        Some(target.as_any_object()),
                        Some(sel!(a3sGuiToggle:)),
                        self.mtm,
                    )
                };
                set_button_checked(&checkbox, config.checked.unwrap_or(false));
                self.action_targets.insert(id, target);
                AppKitOsWidget::Button(checkbox)
            }
            AppKitWidgetKind::Switch => {
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let switch = NSSwitch::initWithFrame(
                    NSSwitch::alloc(self.mtm),
                    config_rect(&config, 48.0, 28.0),
                );
                unsafe {
                    switch.as_super().setTarget(Some(target.as_any_object()));
                    switch.as_super().setAction(Some(sel!(a3sGuiToggle:)));
                }
                set_switch_checked(&switch, config.checked.unwrap_or(false));
                self.action_targets.insert(id, target);
                AppKitOsWidget::Switch(switch)
            }
            AppKitWidgetKind::RadioGroup => {
                let stack_view = NSStackView::initWithFrame(
                    NSStackView::alloc(self.mtm),
                    config_rect(&config, 180.0, 96.0),
                );
                stack_view.setDistribution(NSStackViewDistribution::GravityAreas);
                stack_view.setOrientation(appkit_stack_orientation(
                    config.orientation.unwrap_or(Orientation::Vertical),
                ));
                AppKitOsWidget::StackView(stack_view)
            }
            AppKitWidgetKind::Toolbar => {
                let stack_view = NSStackView::initWithFrame(
                    NSStackView::alloc(self.mtm),
                    config_rect(&config, 320.0, 44.0),
                );
                stack_view.setDistribution(NSStackViewDistribution::GravityAreas);
                stack_view.setOrientation(appkit_stack_orientation(
                    config.orientation.unwrap_or(Orientation::Horizontal),
                ));
                AppKitOsWidget::StackView(stack_view)
            }
            AppKitWidgetKind::Radio => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let radio = unsafe {
                    NSButton::radioButtonWithTitle_target_action(
                        &title,
                        Some(target.as_any_object()),
                        Some(sel!(a3sGuiToggle:)),
                        self.mtm,
                    )
                };
                set_button_checked(&radio, config.checked.unwrap_or(config.selected));
                self.action_targets.insert(id, target);
                AppKitOsWidget::Button(radio)
            }
            AppKitWidgetKind::Tabs => {
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let tab_view = NSTabView::initWithFrame(
                    NSTabView::alloc(self.mtm),
                    config_rect(&config, 320.0, 240.0),
                );
                let delegate: &ProtocolObject<dyn NSTabViewDelegate> =
                    ProtocolObject::from_ref(&*target);
                tab_view.setDelegate(Some(delegate));
                self.action_targets.insert(id, target);
                AppKitOsWidget::TabView(tab_view)
            }
            AppKitWidgetKind::Tab => {
                let tab_item =
                    unsafe { NSTabViewItem::initWithIdentifier(NSTabViewItem::alloc(), None) };
                let label = ns_string(
                    config
                        .label
                        .as_deref()
                        .or(config.value.as_deref())
                        .unwrap_or(""),
                );
                tab_item.setLabel(&label);
                AppKitOsWidget::TabViewItem(tab_item)
            }
            AppKitWidgetKind::ComboBox => {
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let combo_box = NSComboBox::initWithFrame(
                    NSComboBox::alloc(self.mtm),
                    config_rect(&config, 180.0, 32.0),
                );
                combo_box.setCompletes(true);
                combo_box.setNumberOfVisibleItems(8);
                if let Some(value) = config.value.as_deref() {
                    set_combo_box_value(&combo_box, Some(value));
                }
                let delegate: &ProtocolObject<dyn NSComboBoxDelegate> =
                    ProtocolObject::from_ref(&*target);
                unsafe {
                    combo_box.setDelegate(Some(delegate));
                }
                self.action_targets.insert(id, target);
                self.combo_boxes.insert(id, combo_box.clone());
                self.combo_children.entry(id).or_default();
                AppKitOsWidget::ComboBox(combo_box)
            }
            AppKitWidgetKind::ListView => {
                let rect = config_rect(&config, 240.0, 160.0);
                let scroll_view = NSScrollView::initWithFrame(NSScrollView::alloc(self.mtm), rect);
                scroll_view.setBorderType(NSBorderType::BezelBorder);
                scroll_view.setHasVerticalScroller(true);
                scroll_view.setAutohidesScrollers(true);

                let stack_view = NSStackView::initWithFrame(
                    NSStackView::alloc(self.mtm),
                    NSRect::new(NSPoint::new(0.0, 0.0), rect.size),
                );
                stack_view.setDistribution(NSStackViewDistribution::Fill);
                stack_view.setOrientation(appkit_stack_orientation(
                    config.orientation.unwrap_or(Orientation::Vertical),
                ));
                scroll_view.setDocumentView(Some(stack_view.as_super()));

                self.list_views.insert(
                    id,
                    AppKitListViewState {
                        stack_view,
                        rows: Rc::new(RefCell::new(Vec::new())),
                    },
                );
                self.list_children.entry(id).or_default();
                AppKitOsWidget::ListView(scroll_view)
            }
            AppKitWidgetKind::ListItem => {
                let item = AppKitComboBoxItem::from_config(&config);
                self.combo_items.insert(id, item.clone());
                AppKitOsWidget::ComboBoxItem(item)
            }
            AppKitWidgetKind::Slider => {
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let range = AppKitRangeState::from_config(&config);
                let slider = unsafe {
                    NSSlider::sliderWithValue_minValue_maxValue_target_action(
                        range.current(),
                        range.lower(),
                        range.upper(),
                        Some(target.as_any_object()),
                        Some(sel!(a3sGuiChange:)),
                        self.mtm,
                    )
                };
                slider
                    .as_super()
                    .as_super()
                    .setFrameSize(config_size(&config, 180.0, 24.0));
                self.action_targets.insert(id, target);
                AppKitOsWidget::Slider(slider)
            }
            AppKitWidgetKind::ProgressIndicator => {
                let progress = NSProgressIndicator::initWithFrame(
                    NSProgressIndicator::alloc(self.mtm),
                    config_rect(&config, 180.0, 16.0),
                );
                progress.setStyle(NSProgressIndicatorStyle::Bar);
                progress.setIndeterminate(false);
                apply_progress_range(&progress, AppKitRangeState::from_config(&config));
                AppKitOsWidget::ProgressIndicator(progress)
            }
            AppKitWidgetKind::Separator => {
                let orientation = config.orientation.unwrap_or(Orientation::Horizontal);
                let separator = NSBox::initWithFrame(
                    NSBox::alloc(self.mtm),
                    config_rect_for_orientation(&config, orientation, 160.0, 1.0, 1.0, 160.0),
                );
                separator.setBoxType(NSBoxType::Separator);
                AppKitOsWidget::Box(separator)
            }
            AppKitWidgetKind::Label => {
                let label = ns_string(
                    config
                        .label
                        .as_deref()
                        .or(config.value.as_deref())
                        .unwrap_or(""),
                );
                let text_field = NSTextField::labelWithString(&label, self.mtm);
                AppKitOsWidget::TextField(text_field)
            }
            AppKitWidgetKind::TextField => {
                let value = ns_string(config.value.as_deref().unwrap_or(""));
                let text_field = NSTextField::textFieldWithString(&value, self.mtm);
                let target = AppKitActionTarget::new(id, self.events.clone(), self.mtm);
                let delegate: &ProtocolObject<dyn NSTextFieldDelegate> =
                    ProtocolObject::from_ref(&*target);
                unsafe {
                    text_field.setDelegate(Some(delegate));
                }
                self.action_targets.insert(id, target);
                AppKitOsWidget::TextField(text_field)
            }
        };
        Ok(AppKitOsHandle {
            id,
            kind,
            selected: config.selected,
            widget,
        })
    }

    fn apply_native_setter(
        &mut self,
        _id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        match setter {
            NativeWidgetSetter::SetLabel(value) => {
                let label = value.as_deref().unwrap_or("");
                let native_label = ns_string(label);
                match &handle.widget {
                    AppKitOsWidget::Window(window) => window.setTitle(&native_label),
                    AppKitOsWidget::Panel(panel) => panel.setTitle(&native_label),
                    AppKitOsWidget::Popover(state) => {
                        state.content_view_controller.setTitle(Some(&native_label));
                    }
                    AppKitOsWidget::Menu(menu) => menu.setTitle(&native_label),
                    AppKitOsWidget::MenuItem(menu_item) => menu_item.setTitle(&native_label),
                    AppKitOsWidget::Button(button) => button.setTitle(&native_label),
                    AppKitOsWidget::TextField(text_field) => {
                        text_field.as_super().setStringValue(&native_label)
                    }
                    AppKitOsWidget::ComboBoxItem(item) => {
                        if let Some(label) = value {
                            self.update_option_item_label(handle.id, item, label.clone())?;
                        }
                    }
                    AppKitOsWidget::TabViewItem(tab_item) => {
                        tab_item.setLabel(&native_label);
                    }
                    AppKitOsWidget::Box(_) => {}
                    AppKitOsWidget::Switch(_)
                    | AppKitOsWidget::ComboBox(_)
                    | AppKitOsWidget::ListView(_)
                    | AppKitOsWidget::Slider(_)
                    | AppKitOsWidget::ProgressIndicator(_)
                    | AppKitOsWidget::TabView(_)
                    | AppKitOsWidget::StackView(_)
                    | AppKitOsWidget::View(_) => {}
                }
            }
            NativeWidgetSetter::SetValue(value) => match &handle.widget {
                AppKitOsWidget::TextField(text_field) => {
                    text_field
                        .as_super()
                        .setStringValue(&ns_string(value.as_deref().unwrap_or("")));
                }
                AppKitOsWidget::ComboBox(combo_box) => {
                    set_combo_box_value(combo_box, value.as_deref());
                }
                AppKitOsWidget::ComboBoxItem(item) => {
                    self.update_option_item_value(
                        handle.id,
                        item,
                        value.clone().unwrap_or_else(|| item.label.clone()),
                    )?;
                }
                AppKitOsWidget::Slider(slider) => {
                    if let Some(value) = value.as_deref().and_then(parse_f64) {
                        slider.as_super().setDoubleValue(value);
                    }
                }
                AppKitOsWidget::ProgressIndicator(progress) => {
                    if let Some(value) = value.as_deref().and_then(parse_f64) {
                        progress.setDoubleValue(value);
                    }
                }
                AppKitOsWidget::Box(_) => {}
                AppKitOsWidget::Window(_)
                | AppKitOsWidget::Panel(_)
                | AppKitOsWidget::Popover(_)
                | AppKitOsWidget::Menu(_)
                | AppKitOsWidget::MenuItem(_)
                | AppKitOsWidget::View(_)
                | AppKitOsWidget::StackView(_)
                | AppKitOsWidget::ListView(_)
                | AppKitOsWidget::TabView(_)
                | AppKitOsWidget::TabViewItem(_)
                | AppKitOsWidget::Button(_)
                | AppKitOsWidget::Switch(_) => {}
            },
            NativeWidgetSetter::SetPlaceholder(value) => {
                let placeholder = value.as_deref().map(ns_string);
                match &handle.widget {
                    AppKitOsWidget::TextField(text_field) => {
                        text_field.setPlaceholderString(placeholder.as_deref());
                    }
                    AppKitOsWidget::ComboBox(combo_box) => {
                        combo_box
                            .as_super()
                            .setPlaceholderString(placeholder.as_deref());
                    }
                    AppKitOsWidget::Window(_)
                    | AppKitOsWidget::Panel(_)
                    | AppKitOsWidget::Popover(_)
                    | AppKitOsWidget::Menu(_)
                    | AppKitOsWidget::MenuItem(_)
                    | AppKitOsWidget::View(_)
                    | AppKitOsWidget::StackView(_)
                    | AppKitOsWidget::Button(_)
                    | AppKitOsWidget::Switch(_)
                    | AppKitOsWidget::ListView(_)
                    | AppKitOsWidget::Slider(_)
                    | AppKitOsWidget::ProgressIndicator(_)
                    | AppKitOsWidget::TabView(_)
                    | AppKitOsWidget::TabViewItem(_)
                    | AppKitOsWidget::Box(_)
                    | AppKitOsWidget::ComboBoxItem(_) => {}
                }
            }
            NativeWidgetSetter::SetEnabled(value) => {
                if let Some(control) = handle.widget.as_control() {
                    control.setEnabled(*value);
                }
                if let AppKitOsWidget::MenuItem(menu_item) = &handle.widget {
                    menu_item.setEnabled(*value);
                }
            }
            NativeWidgetSetter::SetVisible(value) => {
                if let Some(view) = handle.widget.as_view() {
                    view.setHidden(!*value);
                }
                if let AppKitOsWidget::Popover(state) = &handle.widget {
                    if !*value {
                        state.popover.close();
                    }
                }
                if let AppKitOsWidget::MenuItem(menu_item) = &handle.widget {
                    menu_item.setHidden(!*value);
                }
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                if let Some(view) = handle.widget.as_view() {
                    let width = style.width.and_then(StyleLength::points);
                    let height = style.height.and_then(StyleLength::points);
                    if width.is_some() || height.is_some() {
                        view.setFrameSize(NSSize::new(
                            width.unwrap_or(120.0),
                            height.unwrap_or(32.0),
                        ));
                    }
                }
                if let AppKitOsWidget::Popover(state) = &handle.widget {
                    let width = style.width.and_then(StyleLength::points);
                    let height = style.height.and_then(StyleLength::points);
                    if width.is_some() || height.is_some() {
                        let size = NSSize::new(width.unwrap_or(320.0), height.unwrap_or(220.0));
                        state.popover.setContentSize(size);
                        state.content_view.setFrameSize(size);
                    }
                }
            }
            NativeWidgetSetter::SetChecked(value) => match &handle.widget {
                AppKitOsWidget::Button(button) => {
                    set_button_checked(button, value.unwrap_or(false))
                }
                AppKitOsWidget::Switch(switch) => {
                    set_switch_checked(switch, value.unwrap_or(false))
                }
                AppKitOsWidget::MenuItem(menu_item) => {
                    menu_item.setState(appkit_state(value.unwrap_or(false)));
                }
                AppKitOsWidget::Window(_)
                | AppKitOsWidget::Panel(_)
                | AppKitOsWidget::Popover(_)
                | AppKitOsWidget::Menu(_)
                | AppKitOsWidget::View(_)
                | AppKitOsWidget::StackView(_)
                | AppKitOsWidget::ComboBox(_)
                | AppKitOsWidget::ComboBoxItem(_)
                | AppKitOsWidget::ListView(_)
                | AppKitOsWidget::Slider(_)
                | AppKitOsWidget::ProgressIndicator(_)
                | AppKitOsWidget::TabView(_)
                | AppKitOsWidget::TabViewItem(_)
                | AppKitOsWidget::Box(_)
                | AppKitOsWidget::TextField(_) => {}
            },
            NativeWidgetSetter::SetSelected(value) => match &handle.widget {
                AppKitOsWidget::ComboBoxItem(item) => {
                    self.update_option_item_selected(handle.id, item, *value)?;
                }
                AppKitOsWidget::MenuItem(menu_item) => {
                    menu_item.setState(appkit_state(*value));
                }
                AppKitOsWidget::Button(button) if handle.kind == AppKitWidgetKind::Radio => {
                    set_button_checked(button, *value);
                }
                AppKitOsWidget::TabViewItem(tab_item) if *value => {
                    if let Some(tab_view) = tab_item.tabView(self.mtm) {
                        tab_view.selectTabViewItem(Some(tab_item));
                    }
                }
                _ => {}
            },
            NativeWidgetSetter::SetMinimum(value) => match &handle.widget {
                AppKitOsWidget::Slider(slider) => {
                    slider.setMinValue(value.unwrap_or(0.0));
                }
                AppKitOsWidget::ProgressIndicator(progress) => {
                    progress.setMinValue(value.unwrap_or(0.0));
                }
                _ => {}
            },
            NativeWidgetSetter::SetMaximum(value) => match &handle.widget {
                AppKitOsWidget::Slider(slider) => {
                    slider.setMaxValue(value.unwrap_or(100.0));
                }
                AppKitOsWidget::ProgressIndicator(progress) => {
                    progress.setMaxValue(value.unwrap_or(100.0));
                }
                _ => {}
            },
            NativeWidgetSetter::SetCurrent(value) => match &handle.widget {
                AppKitOsWidget::Slider(slider) => {
                    slider
                        .as_super()
                        .setDoubleValue(value.unwrap_or_else(|| slider.minValue()));
                }
                AppKitOsWidget::ProgressIndicator(progress) => {
                    progress.setDoubleValue(value.unwrap_or_else(|| progress.minValue()));
                }
                _ => {}
            },
            NativeWidgetSetter::SetOrientation(value) => {
                if let (AppKitOsWidget::StackView(stack_view), Some(orientation)) =
                    (&handle.widget, value)
                {
                    stack_view.setOrientation(appkit_stack_orientation(*orientation));
                }
                if let (AppKitOsWidget::Box(separator), Some(orientation)) = (&handle.widget, value)
                {
                    if handle.kind == AppKitWidgetKind::Separator {
                        separator
                            .as_super()
                            .setFrameSize(separator_size(*orientation));
                    }
                }
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
        if let (AppKitOsWidget::ComboBox(_), AppKitOsWidget::ComboBoxItem(item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.combo_items
                .entry(child)
                .or_insert_with(|| item.clone());
            if let Some(old_parent) = self.list_item_parents.remove(&child) {
                if let Some(children) = self.list_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_list_view(old_parent)?;
            }
            if let Some(old_parent) = self.combo_item_parents.insert(child, parent) {
                if let Some(children) = self.combo_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_combo_box(old_parent)?;
            }
            let children = self.combo_children.entry(parent).or_default();
            children.retain(|existing| *existing != child);
            let index = index.min(children.len());
            children.insert(index, child);
            self.rebuild_combo_box(parent)?;
            return Ok(());
        }

        if let (AppKitOsWidget::ListView(_), AppKitOsWidget::ComboBoxItem(item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.combo_items
                .entry(child)
                .or_insert_with(|| item.clone());
            if let Some(old_parent) = self.combo_item_parents.remove(&child) {
                if let Some(children) = self.combo_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_combo_box(old_parent)?;
            }
            if let Some(old_parent) = self.list_item_parents.insert(child, parent) {
                if let Some(children) = self.list_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_list_view(old_parent)?;
            }
            let children = self.list_children.entry(parent).or_default();
            children.retain(|existing| *existing != child);
            let index = index.min(children.len());
            children.insert(index, child);
            self.rebuild_list_view(parent)?;
            return Ok(());
        }

        if let (AppKitOsWidget::TabView(tab_view), AppKitOsWidget::TabViewItem(tab_item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            tab_view.insertTabViewItem_atIndex(
                tab_item,
                index
                    .try_into()
                    .map_err(|_| GuiError::host("AppKit tab view item insertion index overflow"))?,
            );
            if child_handle.selected {
                tab_view.selectTabViewItem(Some(tab_item));
            }
            return Ok(());
        }

        if let AppKitOsWidget::TabViewItem(tab_item) = &parent_handle.widget {
            let child = child_handle.widget.as_view().ok_or_else(|| {
                GuiError::host("AppKit tab item insertion requires an NSView child")
            })?;
            tab_item.setView(Some(child));
            return Ok(());
        }

        if let (AppKitOsWidget::Menu(menu), AppKitOsWidget::MenuItem(menu_item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.menus
                .insert_item(parent, menu, child, menu_item, index)?;
            return Ok(());
        }

        if let (AppKitOsWidget::MenuItem(menu_item), AppKitOsWidget::Menu(menu)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            menu_item.setSubmenu(Some(menu));
            return Ok(());
        }

        let child = child_handle.widget.as_view().ok_or_else(|| {
            GuiError::host("AppKit native child insertion requires an NSView child")
        })?;
        match &parent_handle.widget {
            AppKitOsWidget::Window(window) => window.setContentView(Some(child)),
            AppKitOsWidget::Panel(panel) => panel.setContentView(Some(child)),
            AppKitOsWidget::Popover(state) => state.content_view.addSubview(child),
            AppKitOsWidget::View(view) => view.addSubview(child),
            AppKitOsWidget::StackView(stack_view) => stack_view.insertArrangedSubview_atIndex(
                child,
                index.try_into().map_err(|_| {
                    GuiError::host("AppKit stack view child insertion index overflow")
                })?,
            ),
            AppKitOsWidget::Button(button) => button.as_super().as_super().addSubview(child),
            AppKitOsWidget::Switch(switch) => switch.as_super().as_super().addSubview(child),
            AppKitOsWidget::Slider(slider) => slider.as_super().as_super().addSubview(child),
            AppKitOsWidget::ProgressIndicator(progress) => progress.as_super().addSubview(child),
            AppKitOsWidget::TabView(tab_view) => tab_view.addSubview(child),
            AppKitOsWidget::Box(box_) => box_.as_super().addSubview(child),
            AppKitOsWidget::ComboBox(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::ListView(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::TabViewItem(_) => {}
            AppKitOsWidget::TextField(text_field) => {
                text_field.as_super().as_super().addSubview(child)
            }
        }
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        let was_root = self.root == Some(id);
        if was_root {
            self.root = None;
        }
        self.action_targets.remove(&id);
        if let AppKitOsWidget::ComboBox(_) = &handle.widget {
            self.combo_boxes.remove(&id);
            if let Some(children) = self.combo_children.remove(&id) {
                for child in children {
                    self.combo_item_parents.remove(&child);
                }
            }
        }
        if let AppKitOsWidget::ListView(_) = &handle.widget {
            self.list_views.remove(&id);
            if let Some(children) = self.list_children.remove(&id) {
                for child in children {
                    self.list_item_parents.remove(&child);
                }
            }
        }
        if let AppKitOsWidget::ComboBoxItem(_) = &handle.widget {
            self.combo_items.remove(&id);
            if let Some(parent) = self.combo_item_parents.remove(&id) {
                if let Some(children) = self.combo_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_combo_box(parent)?;
            }
            if let Some(parent) = self.list_item_parents.remove(&id) {
                if let Some(children) = self.list_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_list_view(parent)?;
            }
        }
        if let AppKitOsWidget::Menu(_) = &handle.widget {
            if was_root {
                self._application.setMainMenu(None);
            }
            self.menus.remove_menu(id);
        }
        if let AppKitOsWidget::MenuItem(_) = &handle.widget {
            self.menus.remove_item(id)?;
        }
        if let AppKitOsWidget::TabViewItem(tab_item) = &handle.widget {
            if let Some(tab_view) = tab_item.tabView(self.mtm) {
                tab_view.removeTabViewItem(tab_item);
            }
        }
        if let Some(view) = handle.widget.as_view() {
            view.removeFromSuperview();
        }
        if let AppKitOsWidget::Panel(panel) = &handle.widget {
            panel.as_super().close();
        }
        if let AppKitOsWidget::Popover(state) = &handle.widget {
            state.popover.close();
        }
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.root = Some(id);
        match &handle.widget {
            AppKitOsWidget::Window(window) => window.makeKeyAndOrderFront(None),
            AppKitOsWidget::Panel(panel) => panel.as_super().makeKeyAndOrderFront(None),
            AppKitOsWidget::Menu(menu) => self._application.setMainMenu(Some(menu)),
            _ => {}
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
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

fn config_size(config: &NativeWidgetConfig, default_width: f64, default_height: f64) -> NSSize {
    let width = config
        .portable_style
        .width
        .and_then(StyleLength::points)
        .unwrap_or(default_width);
    let height = config
        .portable_style
        .height
        .and_then(StyleLength::points)
        .unwrap_or(default_height);
    NSSize::new(width, height)
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
}

impl AppKitRangeState {
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

fn apply_progress_range(progress: &NSProgressIndicator, range: AppKitRangeState) {
    progress.setMinValue(range.lower());
    progress.setMaxValue(range.upper());
    progress.setDoubleValue(range.current());
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
