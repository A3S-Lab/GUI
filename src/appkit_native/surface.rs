use super::*;
use crate::style::OverflowMode;

impl AppKitNativeSurface {
    fn apply_range(&mut self, id: HostNodeId, widget: &AppKitOsWidget) {
        let range = self.ranges.get(&id).copied().unwrap_or_default();
        match widget {
            AppKitOsWidget::Slider(slider) => {
                slider.setMinValue(range.lower());
                slider.setMaxValue(range.upper());
                apply_slider_step(slider, range);
                slider.as_super().setDoubleValue(range.current());
            }
            AppKitOsWidget::ProgressIndicator(progress) => {
                apply_progress_range(progress, range);
            }
            _ => {}
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
                let style = if config.window_resizable.unwrap_or(true) {
                    NSWindowStyleMask::Titled
                        | NSWindowStyleMask::Closable
                        | NSWindowStyleMask::Miniaturizable
                        | NSWindowStyleMask::Resizable
                } else {
                    NSWindowStyleMask::Titled
                        | NSWindowStyleMask::Closable
                        | NSWindowStyleMask::Miniaturizable
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
                let delegate = AppKitWindowDelegate::new(
                    id,
                    self.events.clone(),
                    self.closed_windows.clone(),
                    self.mtm,
                );
                let delegate_ref: &ProtocolObject<dyn NSWindowDelegate> =
                    ProtocolObject::from_ref(&*delegate);
                window.setDelegate(Some(delegate_ref));
                self.window_delegates.insert(id, delegate);
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
                unsafe {
                    panel.as_super().setReleasedWhenClosed(false);
                }
                panel.setTitle(&ns_string(config.label.as_deref().unwrap_or("")));
                let delegate = AppKitWindowDelegate::new(
                    id,
                    self.events.clone(),
                    self.closed_windows.clone(),
                    self.mtm,
                );
                let delegate_ref: &ProtocolObject<dyn NSWindowDelegate> =
                    ProtocolObject::from_ref(&*delegate);
                panel.as_super().setDelegate(Some(delegate_ref));
                self.window_delegates.insert(id, delegate);
                self.dialog_visible.insert(id, config.visible);
                AppKitOsWidget::Panel(panel)
            }
            AppKitWidgetKind::Popover => {
                let size = config_size(&config, 320.0, 220.0);
                let content_view =
                    flipped_view(self.mtm, NSRect::new(NSPoint::new(0.0, 0.0), size));
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
                self.popover_visible.insert(id, config.visible);
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
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                if let Some(orientation) = table_stack_orientation(config.role) {
                    let stack_view =
                        flipped_stack_view(self.mtm, config_rect(&config, 320.0, 44.0));
                    apply_stack_view_layout(&stack_view, &config.portable_style, Some(orientation));
                    if let Some((min_width, min_height)) = table_stack_minimum_size(config.role) {
                        self.apply_native_minimum_size(
                            id,
                            stack_view.as_super(),
                            min_width,
                            min_height,
                        );
                    }
                    AppKitOsWidget::StackView(stack_view)
                } else {
                    let view = flipped_view(self.mtm, config_rect(&config, 320.0, 240.0));
                    AppKitOsWidget::View(view)
                }
            }
            AppKitWidgetKind::Button => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                let orientation = config.orientation.unwrap_or(Orientation::Vertical);
                let stack_view = flipped_stack_view(self.mtm, config_rect(&config, 180.0, 96.0));
                apply_stack_view_layout(&stack_view, &config.portable_style, Some(orientation));
                AppKitOsWidget::StackView(stack_view)
            }
            AppKitWidgetKind::Toolbar => {
                let orientation = config.orientation.unwrap_or(Orientation::Horizontal);
                let stack_view = flipped_stack_view(self.mtm, config_rect(&config, 320.0, 44.0));
                apply_stack_view_layout(&stack_view, &config.portable_style, Some(orientation));
                AppKitOsWidget::StackView(stack_view)
            }
            AppKitWidgetKind::Radio => {
                let title = ns_string(config.label.as_deref().unwrap_or(""));
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
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
                scroll_view.setBorderType(NSBorderType::NoBorder);
                scroll_view.setHasVerticalScroller(true);
                scroll_view.setHasHorizontalScroller(false);
                scroll_view.setAutohidesScrollers(true);

                let stack_view =
                    flipped_stack_view(self.mtm, NSRect::new(NSPoint::new(0.0, 0.0), rect.size));
                apply_stack_view_layout(
                    &stack_view,
                    &config.portable_style,
                    Some(Orientation::Vertical),
                );
                configure_scroll_document(&scroll_view, &stack_view);

                self.list_views.insert(
                    id,
                    AppKitListViewState {
                        stack_view,
                        rows: Rc::new(RefCell::new(Vec::new())),
                        style: config.portable_style.clone(),
                    },
                );
                self.list_children.entry(id).or_default();
                AppKitOsWidget::ListView(scroll_view)
            }
            AppKitWidgetKind::ScrollView => {
                let orientation = config.orientation.unwrap_or(Orientation::Vertical);
                let rect = config_rect(&config, 320.0, 240.0);
                let scroll_view = NSScrollView::initWithFrame(NSScrollView::alloc(self.mtm), rect);
                scroll_view.setHasVerticalScroller(appkit_vertical_scroll_enabled(&config));
                scroll_view.setHasHorizontalScroller(appkit_horizontal_scroll_enabled(&config));
                scroll_view.setAutohidesScrollers(true);

                let stack_view =
                    flipped_stack_view(self.mtm, NSRect::new(NSPoint::new(0.0, 0.0), rect.size));
                apply_stack_view_layout(&stack_view, &config.portable_style, Some(orientation));
                configure_scroll_document(&scroll_view, &stack_view);

                AppKitOsWidget::ScrollView(AppKitScrollViewState {
                    scroll_view,
                    stack_view,
                })
            }
            AppKitWidgetKind::ListItem => {
                let item = AppKitComboBoxItem::from_config(&config);
                self.combo_items.insert(id, item.clone());
                AppKitOsWidget::ComboBoxItem(item)
            }
            AppKitWidgetKind::Slider => {
                let target = AppKitActionTarget::new(
                    id,
                    self.events.clone(),
                    self.focused_node.clone(),
                    self.mtm,
                );
                let range = AppKitRangeState::from_config(&config);
                let orientation = config.orientation.unwrap_or(Orientation::Horizontal);
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
                apply_slider_orientation(&slider, orientation);
                slider
                    .as_super()
                    .as_super()
                    .setFrameSize(slider_size_for_orientation(&config, orientation));
                apply_slider_step(&slider, range);
                self.ranges.insert(id, range);
                self.action_targets.insert(id, target);
                AppKitOsWidget::Slider(slider)
            }
            AppKitWidgetKind::ProgressIndicator => {
                let range = AppKitRangeState::from_config(&config);
                let progress = NSProgressIndicator::initWithFrame(
                    NSProgressIndicator::alloc(self.mtm),
                    config_rect(&config, 180.0, 16.0),
                );
                progress.setStyle(NSProgressIndicatorStyle::Bar);
                progress.setIndeterminate(false);
                apply_progress_range(&progress, range);
                self.ranges.insert(id, range);
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
                if config_is_search(&config) {
                    let text_field = NSSearchField::new(self.mtm);
                    text_field.as_super().as_super().setStringValue(&value);
                    text_field.setSendsSearchStringImmediately(true);
                    text_field.setSendsWholeSearchString(false);
                    text_field
                        .as_super()
                        .as_super()
                        .as_super()
                        .setFrameSize(config_text_input_size(&config));
                    self.text_inputs
                        .insert(id, AppKitTextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    let target = AppKitActionTarget::new(
                        id,
                        self.events.clone(),
                        self.focused_node.clone(),
                        self.mtm,
                    );
                    let delegate: &ProtocolObject<dyn NSSearchFieldDelegate> =
                        ProtocolObject::from_ref(&*target);
                    unsafe {
                        text_field.setDelegate(Some(delegate));
                    }
                    self.action_targets.insert(id, target);
                    AppKitOsWidget::SearchField(text_field)
                } else if config_is_password(&config) {
                    let text_field = NSSecureTextField::new(self.mtm);
                    text_field.as_super().as_super().setStringValue(&value);
                    text_field
                        .as_super()
                        .as_super()
                        .as_super()
                        .setFrameSize(config_text_input_size(&config));
                    self.text_inputs
                        .insert(id, AppKitTextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    let target = AppKitActionTarget::new(
                        id,
                        self.events.clone(),
                        self.focused_node.clone(),
                        self.mtm,
                    );
                    let delegate: &ProtocolObject<dyn NSTextFieldDelegate> =
                        ProtocolObject::from_ref(&*target);
                    unsafe {
                        text_field.as_super().setDelegate(Some(delegate));
                    }
                    self.action_targets.insert(id, target);
                    AppKitOsWidget::SecureTextField(text_field)
                } else {
                    let text_field = NSTextField::textFieldWithString(&value, self.mtm);
                    if config_is_textarea(&config) {
                        text_field.as_super().setUsesSingleLineMode(false);
                        if let Some(cell) = text_field.as_super().cell() {
                            cell.setUsesSingleLineMode(false);
                        }
                    }
                    text_field
                        .as_super()
                        .as_super()
                        .setFrameSize(config_text_input_size(&config));
                    self.text_inputs
                        .insert(id, AppKitTextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    let target = AppKitActionTarget::new(
                        id,
                        self.events.clone(),
                        self.focused_node.clone(),
                        self.mtm,
                    );
                    let delegate: &ProtocolObject<dyn NSTextFieldDelegate> =
                        ProtocolObject::from_ref(&*target);
                    unsafe {
                        text_field.setDelegate(Some(delegate));
                    }
                    self.action_targets.insert(id, target);
                    AppKitOsWidget::TextField(text_field)
                }
            }
        };
        let handle = AppKitOsHandle {
            id,
            kind,
            selected: config.selected,
            widget,
        };
        self.apply_text_input_hints(id, &handle.widget);
        set_widget_title(&handle.widget, config.title.as_deref());
        self.register_responder(id, &handle.widget);
        self.widgets.insert(id, handle.widget.clone());
        Ok(handle)
    }

    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        if let Some(config) = self.text_input_configs.get_mut(&id) {
            apply_widget_setter(config, setter);
        }

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
                    AppKitOsWidget::TextField(text_field)
                        if handle.kind == AppKitWidgetKind::Label =>
                    {
                        text_field.as_super().setStringValue(&native_label)
                    }
                    AppKitOsWidget::SearchField(_) => {}
                    AppKitOsWidget::SecureTextField(_) => {}
                    AppKitOsWidget::TextField(_) => {}
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
                    | AppKitOsWidget::ScrollView(_)
                    | AppKitOsWidget::Slider(_)
                    | AppKitOsWidget::ProgressIndicator(_)
                    | AppKitOsWidget::TabView(_)
                    | AppKitOsWidget::StackView(_)
                    | AppKitOsWidget::View(_) => {}
                }
            }
            NativeWidgetSetter::SetWindowResizable(value) => {
                if let AppKitOsWidget::Window(window) = &handle.widget {
                    let mut style = window.styleMask();
                    if value.unwrap_or(true) {
                        style.insert(NSWindowStyleMask::Resizable);
                    } else {
                        style.remove(NSWindowStyleMask::Resizable);
                    }
                    window.setStyleMask(style);
                }
            }
            NativeWidgetSetter::SetValue(value) => match &handle.widget {
                AppKitOsWidget::TextField(text_field) => {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let max_length = self
                            .action_targets
                            .get(&id)
                            .and_then(|target| target.max_length());
                        set_control_string_value(
                            text_field.as_super(),
                            value.as_deref().unwrap_or(""),
                            max_length,
                        );
                    } else if let Some(value) = value.as_deref() {
                        text_field.as_super().setStringValue(&ns_string(value));
                    }
                }
                AppKitOsWidget::SearchField(text_field) => {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let max_length = self
                            .action_targets
                            .get(&id)
                            .and_then(|target| target.max_length());
                        set_control_string_value(
                            text_field.as_super().as_super(),
                            value.as_deref().unwrap_or(""),
                            max_length,
                        );
                    }
                }
                AppKitOsWidget::SecureTextField(text_field) => {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let max_length = self
                            .action_targets
                            .get(&id)
                            .and_then(|target| target.max_length());
                        set_control_string_value(
                            text_field.as_super().as_super(),
                            value.as_deref().unwrap_or(""),
                            max_length,
                        );
                    }
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
                AppKitOsWidget::Slider(_) => {
                    if let Some(value) = value.as_deref().and_then(parse_f64) {
                        self.ranges.entry(id).or_default().current = Some(value);
                        self.apply_range(id, &handle.widget);
                    }
                }
                AppKitOsWidget::ProgressIndicator(_) => {
                    if let Some(value) = value.as_deref().and_then(parse_f64) {
                        self.ranges.entry(id).or_default().current = Some(value);
                        self.apply_range(id, &handle.widget);
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
                | AppKitOsWidget::ScrollView(_)
                | AppKitOsWidget::TabView(_)
                | AppKitOsWidget::TabViewItem(_)
                | AppKitOsWidget::Button(_)
                | AppKitOsWidget::Switch(_) => {}
            },
            NativeWidgetSetter::SetPlaceholder(value) => {
                let placeholder = value.as_deref().map(ns_string);
                match &handle.widget {
                    AppKitOsWidget::TextField(text_field)
                        if handle.kind == AppKitWidgetKind::TextField =>
                    {
                        text_field.setPlaceholderString(placeholder.as_deref());
                    }
                    AppKitOsWidget::SecureTextField(text_field)
                        if handle.kind == AppKitWidgetKind::TextField =>
                    {
                        text_field
                            .as_super()
                            .setPlaceholderString(placeholder.as_deref());
                    }
                    AppKitOsWidget::SearchField(text_field)
                        if handle.kind == AppKitWidgetKind::TextField =>
                    {
                        text_field
                            .as_super()
                            .setPlaceholderString(placeholder.as_deref());
                    }
                    AppKitOsWidget::TextField(_) => {}
                    AppKitOsWidget::SearchField(_) => {}
                    AppKitOsWidget::SecureTextField(_) => {}
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
                    | AppKitOsWidget::ScrollView(_)
                    | AppKitOsWidget::Slider(_)
                    | AppKitOsWidget::ProgressIndicator(_)
                    | AppKitOsWidget::TabView(_)
                    | AppKitOsWidget::TabViewItem(_)
                    | AppKitOsWidget::Box(_)
                    | AppKitOsWidget::ComboBoxItem(_) => {}
                }
            }
            NativeWidgetSetter::SetTitle(value) => {
                set_widget_title(&handle.widget, value.as_deref());
            }
            NativeWidgetSetter::SetEnabled(value) => {
                if let Some(control) = handle.widget.as_control() {
                    control.setEnabled(*value);
                }
                if let AppKitOsWidget::MenuItem(menu_item) = &handle.widget {
                    menu_item.setEnabled(*value);
                }
            }
            NativeWidgetSetter::SetReadOnly(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        text_field.setEditable(!*value);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        text_field.as_super().setEditable(!*value);
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        text_field.as_super().setEditable(!*value);
                    }
                }
            }
            NativeWidgetSetter::SetVisible(value) => {
                if let AppKitOsWidget::Panel(panel) = &handle.widget {
                    self.set_panel_visible(id, panel, *value);
                }
                if let Some(view) = handle.widget.as_view() {
                    view.setHidden(!*value);
                }
                if let AppKitOsWidget::Popover(state) = &handle.widget {
                    self.set_popover_visible(id, state, *value);
                }
                if let AppKitOsWidget::MenuItem(menu_item) = &handle.widget {
                    menu_item.setHidden(!*value);
                }
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                apply_widget_portable_style(&handle.widget, handle.kind, style);
                match &handle.widget {
                    AppKitOsWidget::Window(window) => {
                        apply_window_portable_style(window, style);
                    }
                    AppKitOsWidget::Panel(panel) => {
                        apply_window_portable_style(panel.as_super(), style);
                    }
                    _ => {}
                }
                if let Some(view) = handle.widget.as_view() {
                    self.apply_native_size_constraints(id, view, style);
                }
                if let AppKitOsWidget::Popover(state) = &handle.widget {
                    let constraints = style.native_size_constraints();
                    if constraints.width.is_some() || constraints.height.is_some() {
                        let current = state.content_view.frame().size;
                        let content_size = NSSize::new(
                            constraints.width.unwrap_or(current.width.max(320.0)),
                            constraints.height.unwrap_or(current.height.max(220.0)),
                        );
                        state.popover.setContentSize(content_size);
                        state.content_view.setFrameSize(content_size);
                    }
                    self.apply_native_size_constraints(id, &state.content_view, style);
                }
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let sizing = self.text_inputs.entry(id).or_default();
                        let size = style.native_size_constraints();
                        sizing.explicit_width = size.width;
                        sizing.explicit_height = size.height;
                        self.apply_text_input_size(id, text_field);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let sizing = self.text_inputs.entry(id).or_default();
                        let size = style.native_size_constraints();
                        sizing.explicit_width = size.width;
                        sizing.explicit_height = size.height;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        let sizing = self.text_inputs.entry(id).or_default();
                        let size = style.native_size_constraints();
                        sizing.explicit_width = size.width;
                        sizing.explicit_height = size.height;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
                if let AppKitOsWidget::StackView(stack_view) = &handle.widget {
                    apply_stack_view_layout(stack_view, style, None);
                }
                if let AppKitOsWidget::ScrollView(state) = &handle.widget {
                    apply_scroll_view_layout(state, style);
                }
                if let AppKitOsWidget::ComboBoxItem(item) = &handle.widget {
                    self.update_option_item_style(id, item, style.clone())?;
                }
                if let AppKitOsWidget::ListView(scroll_view) = &handle.widget {
                    if let Some(state) = self.list_views.get_mut(&id) {
                        state.style = style.clone();
                        apply_list_view_layout(scroll_view, state, style);
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
                | AppKitOsWidget::ScrollView(_)
                | AppKitOsWidget::Slider(_)
                | AppKitOsWidget::ProgressIndicator(_)
                | AppKitOsWidget::TabView(_)
                | AppKitOsWidget::TabViewItem(_)
                | AppKitOsWidget::Box(_)
                | AppKitOsWidget::TextField(_)
                | AppKitOsWidget::SearchField(_)
                | AppKitOsWidget::SecureTextField(_) => {}
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
            NativeWidgetSetter::SetMinimum(value) => {
                if matches!(
                    &handle.widget,
                    AppKitOsWidget::Slider(_) | AppKitOsWidget::ProgressIndicator(_)
                ) {
                    self.ranges.entry(id).or_default().min = *value;
                    self.apply_range(id, &handle.widget);
                }
            }
            NativeWidgetSetter::SetMaximum(value) => {
                if matches!(
                    &handle.widget,
                    AppKitOsWidget::Slider(_) | AppKitOsWidget::ProgressIndicator(_)
                ) {
                    self.ranges.entry(id).or_default().max = *value;
                    self.apply_range(id, &handle.widget);
                }
            }
            NativeWidgetSetter::SetCurrent(value) => {
                if matches!(
                    &handle.widget,
                    AppKitOsWidget::Slider(_) | AppKitOsWidget::ProgressIndicator(_)
                ) {
                    self.ranges.entry(id).or_default().current = *value;
                    self.apply_range(id, &handle.widget);
                }
            }
            NativeWidgetSetter::SetStep(value) => {
                if matches!(&handle.widget, AppKitOsWidget::Slider(_)) {
                    self.ranges.entry(id).or_default().step = *value;
                    self.apply_range(id, &handle.widget);
                }
            }
            NativeWidgetSetter::SetMaxLength(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        if let Some(target) = self.action_targets.get(&id) {
                            target.set_max_length(*value);
                        }
                        apply_control_max_length(text_field.as_super(), *value);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        if let Some(target) = self.action_targets.get(&id) {
                            target.set_max_length(*value);
                        }
                        apply_control_max_length(text_field.as_super().as_super(), *value);
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        if let Some(target) = self.action_targets.get(&id) {
                            target.set_max_length(*value);
                        }
                        apply_control_max_length(text_field.as_super().as_super(), *value);
                    }
                }
            }
            NativeWidgetSetter::SetCols(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().cols = *value;
                        self.apply_text_input_size(id, text_field);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().cols = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().cols = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
            }
            NativeWidgetSetter::SetSize(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().size = *value;
                        self.apply_text_input_size(id, text_field);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().size = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().size = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
            }
            NativeWidgetSetter::SetRows(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().rows = *value;
                        self.apply_text_input_size(id, text_field);
                    }
                }
                if let AppKitOsWidget::SecureTextField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().rows = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
                if let AppKitOsWidget::SearchField(text_field) = &handle.widget {
                    if handle.kind == AppKitWidgetKind::TextField {
                        self.text_inputs.entry(id).or_default().rows = *value;
                        self.apply_text_input_size(id, text_field.as_super());
                    }
                }
            }
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
                if let (AppKitOsWidget::Slider(slider), Some(orientation)) = (&handle.widget, value)
                {
                    apply_slider_orientation(slider, *orientation);
                }
                if let (AppKitOsWidget::ScrollView(state), Some(orientation)) =
                    (&handle.widget, value)
                {
                    state
                        .stack_view
                        .setOrientation(appkit_stack_orientation(*orientation));
                }
                if let (AppKitOsWidget::ListView(scroll_view), Some(orientation)) =
                    (&handle.widget, value)
                {
                    if let Some(state) = self.list_views.get(&id) {
                        state
                            .stack_view
                            .setOrientation(appkit_stack_orientation(*orientation));
                        apply_list_view_layout(scroll_view, state, &state.style);
                    }
                }
            }
            NativeWidgetSetter::SetAutocomplete(_)
            | NativeWidgetSetter::SetInputMode(_)
            | NativeWidgetSetter::SetAutoCorrect(_)
            | NativeWidgetSetter::SetVirtualKeyboardPolicy(_)
            | NativeWidgetSetter::SetInputType(_)
            | NativeWidgetSetter::SetSpellCheck(_) => {
                self.apply_text_input_hints(id, &handle.widget);
            }
            NativeWidgetSetter::SetAutoFocus(true) => {
                self.request_auto_focus(id, &handle.widget);
            }
            NativeWidgetSetter::SetAutoFocus(false) => {
                clear_pending_auto_focus(&mut self.pending_auto_focus, id);
            }
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetClassName(_)
            | NativeWidgetSetter::SetRequired(_)
            | NativeWidgetSetter::SetInvalid(_)
            | NativeWidgetSetter::SetMultiple(_)
            | NativeWidgetSetter::SetExpanded(_)
            | NativeWidgetSetter::SetPattern(_)
            | NativeWidgetSetter::SetMinLength(_)
            | NativeWidgetSetter::SetName(_)
            | NativeWidgetSetter::SetForm(_)
            | NativeWidgetSetter::SetAccept(_)
            | NativeWidgetSetter::SetCapture(_)
            | NativeWidgetSetter::SetEnterKeyHint(_)
            | NativeWidgetSetter::SetAutoCapitalize(_)
            | NativeWidgetSetter::SetHidden(_)
            | NativeWidgetSetter::SetLang(_)
            | NativeWidgetSetter::SetDir(_)
            | NativeWidgetSetter::SetTabIndex(_)
            | NativeWidgetSetter::SetExplicitRole(_)
            | NativeWidgetSetter::SetAccessKey(_)
            | NativeWidgetSetter::SetContentEditable(_)
            | NativeWidgetSetter::SetDraggable(_)
            | NativeWidgetSetter::SetTranslate(_)
            | NativeWidgetSetter::SetInert(_)
            | NativeWidgetSetter::SetPopover(_)
            | NativeWidgetSetter::SetAnchor(_)
            | NativeWidgetSetter::SetCustomElementIs(_)
            | NativeWidgetSetter::SetNonce(_)
            | NativeWidgetSetter::SetAlt(_)
            | NativeWidgetSetter::SetHref(_)
            | NativeWidgetSetter::SetSrc(_)
            | NativeWidgetSetter::SetSrcset(_)
            | NativeWidgetSetter::SetSizes(_)
            | NativeWidgetSetter::SetMedia(_)
            | NativeWidgetSetter::SetResourceType(_)
            | NativeWidgetSetter::SetIntrinsicWidth(_)
            | NativeWidgetSetter::SetIntrinsicHeight(_)
            | NativeWidgetSetter::SetLoading(_)
            | NativeWidgetSetter::SetDecoding(_)
            | NativeWidgetSetter::SetFetchPriority(_)
            | NativeWidgetSetter::SetCrossOrigin(_)
            | NativeWidgetSetter::SetReferrerPolicy(_)
            | NativeWidgetSetter::SetPoster(_)
            | NativeWidgetSetter::SetControls(_)
            | NativeWidgetSetter::SetAutoplay(_)
            | NativeWidgetSetter::SetLoopPlayback(_)
            | NativeWidgetSetter::SetMuted(_)
            | NativeWidgetSetter::SetPlaysInline(_)
            | NativeWidgetSetter::SetPreload(_)
            | NativeWidgetSetter::SetTrackKind(_)
            | NativeWidgetSetter::SetSrclang(_)
            | NativeWidgetSetter::SetTrackLabel(_)
            | NativeWidgetSetter::SetDefaultTrack(_)
            | NativeWidgetSetter::SetList(_)
            | NativeWidgetSetter::SetDirname(_)
            | NativeWidgetSetter::SetFormAction(_)
            | NativeWidgetSetter::SetFormEnctype(_)
            | NativeWidgetSetter::SetFormMethod(_)
            | NativeWidgetSetter::SetFormTarget(_)
            | NativeWidgetSetter::SetFormNoValidate(_)
            | NativeWidgetSetter::SetHtmlResourcePolicy(_)
            | NativeWidgetSetter::SetHtmlActivation(_)
            | NativeWidgetSetter::SetHtmlTextAnnotation(_)
            | NativeWidgetSetter::SetHtmlDialog(_)
            | NativeWidgetSetter::SetHtmlShadow(_)
            | NativeWidgetSetter::SetHtmlMicrodata(_)
            | NativeWidgetSetter::SetHtmlFormAssociation(_)
            | NativeWidgetSetter::SetHtmlCollection(_)
            | NativeWidgetSetter::SetAccessibilityRelationships(_)
            | NativeWidgetSetter::SetAccessibilityDescription(_)
            | NativeWidgetSetter::SetAccessibilityStructure(_)
            | NativeWidgetSetter::SetAccessibilityState(_)
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
        if let AppKitOsWidget::Panel(panel) = &child_handle.widget {
            self.show_panel_if_marked_visible(child, panel);
            return Ok(());
        }
        if let AppKitOsWidget::Popover(state) = &child_handle.widget {
            if parent_handle.widget.as_view().is_some() {
                self.popover_anchors.insert(child, parent);
                self.show_popover_if_marked_visible(child, state);
                return Ok(());
            }
            return Err(GuiError::host(
                "AppKit popover insertion requires an anchor NSView parent",
            ));
        }
        if matches!(child_handle.widget, AppKitOsWidget::Menu(_))
            && !matches!(parent_handle.widget, AppKitOsWidget::MenuItem(_))
        {
            if let AppKitOsWidget::Menu(menu) = &child_handle.widget {
                self._application.setMainMenu(Some(menu));
            }
            return Ok(());
        }

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
            self.focus_pending_auto_focus();
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
            self.focus_pending_auto_focus();
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
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let AppKitOsWidget::TabViewItem(tab_item) = &parent_handle.widget {
            let child = child_handle.widget.as_view().ok_or_else(|| {
                GuiError::host(format!(
                    "AppKit tab item insertion requires an NSView child: parent={:?}({parent:?}) child={:?}({child:?})",
                    parent_handle.kind, child_handle.kind
                ))
            })?;
            tab_item.setView(Some(child));
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let (AppKitOsWidget::Menu(menu), AppKitOsWidget::MenuItem(menu_item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.menus
                .insert_item(parent, menu, child, menu_item, index)?;
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let (AppKitOsWidget::MenuItem(menu_item), AppKitOsWidget::Menu(menu)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            menu_item.setSubmenu(Some(menu));
            self.focus_pending_auto_focus();
            return Ok(());
        }

        let child = child_handle.widget.as_view().ok_or_else(|| {
            GuiError::host(format!(
                "AppKit native child insertion requires an NSView child: parent={:?}({parent:?}) child={:?}({child:?})",
                parent_handle.kind, child_handle.kind
            ))
        })?;
        match &parent_handle.widget {
            AppKitOsWidget::Window(window) => install_window_content_view(window, child),
            AppKitOsWidget::Panel(panel) => install_window_content_view(panel.as_super(), child),
            AppKitOsWidget::Popover(state) => state.content_view.addSubview(child),
            AppKitOsWidget::View(view) => view.addSubview(child),
            AppKitOsWidget::StackView(stack_view) => stack_view.insertArrangedSubview_atIndex(
                child,
                stack_arranged_insert_index(stack_view, index)?,
            ),
            AppKitOsWidget::ScrollView(state) => {
                state.stack_view.insertArrangedSubview_atIndex(
                    child,
                    stack_arranged_insert_index(&state.stack_view, index)?,
                );
                scroll_view_to_top(state);
            }
            AppKitOsWidget::ListView(scroll_view) => {
                if let Some(state) = self.list_views.get(&parent) {
                    state.stack_view.insertArrangedSubview_atIndex(
                        child,
                        stack_arranged_insert_index(&state.stack_view, index)?,
                    );
                    apply_list_view_layout(scroll_view, state, &state.style);
                }
            }
            AppKitOsWidget::Button(button) => button.as_super().as_super().addSubview(child),
            AppKitOsWidget::Switch(switch) => switch.as_super().as_super().addSubview(child),
            AppKitOsWidget::Slider(slider) => slider.as_super().as_super().addSubview(child),
            AppKitOsWidget::ProgressIndicator(progress) => progress.as_super().addSubview(child),
            AppKitOsWidget::TabView(tab_view) => tab_view.addSubview(child),
            AppKitOsWidget::Box(box_) => box_.as_super().addSubview(child),
            AppKitOsWidget::ComboBox(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::TabViewItem(_) => {}
            AppKitOsWidget::TextField(text_field) => {
                text_field.as_super().as_super().addSubview(child)
            }
            AppKitOsWidget::SearchField(text_field) => text_field
                .as_super()
                .as_super()
                .as_super()
                .addSubview(child),
            AppKitOsWidget::SecureTextField(text_field) => text_field
                .as_super()
                .as_super()
                .as_super()
                .addSubview(child),
        }
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        let was_root = self.root == Some(id);
        if was_root {
            self.root = None;
        }
        clear_pending_auto_focus(&mut self.pending_auto_focus, id);
        self.widgets.remove(&id);
        self.action_targets.remove(&id);
        if self.focused_node.get() == Some(id) {
            self.focused_node.set(None);
        }
        self.unregister_responder(&handle.widget);
        self.ranges.remove(&id);
        self.text_inputs.remove(&id);
        self.text_input_configs.remove(&id);
        self.clear_native_size_constraints(id);
        self.closed_windows.borrow_mut().remove(&id);
        self.dialog_visible.remove(&id);
        self.popover_visible.remove(&id);
        self.popover_anchors.remove(&id);
        self.popover_anchors.retain(|_, anchor| *anchor != id);
        if let AppKitOsWidget::Window(window) = &handle.widget {
            window.setDelegate(None);
            self.window_delegates.remove(&id);
            window.close();
        }
        if let AppKitOsWidget::Panel(panel) = &handle.widget {
            panel.as_super().setDelegate(None);
            self.window_delegates.remove(&id);
        }
        if let AppKitOsWidget::ComboBox(_) = &handle.widget {
            self.combo_boxes.remove(&id);
            if let Some(children) = self.combo_children.remove(&id) {
                for child in children {
                    self.combo_item_parents.remove(&child);
                }
            }
        }
        if let AppKitOsWidget::ListView(_) = &handle.widget {
            if let Some(state) = self.list_views.remove(&id) {
                for row in state.rows.borrow().iter() {
                    self.unregister_view_responder(row.button_view());
                }
            }
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
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        let root_changed = self.root != Some(id);
        self.root = Some(id);
        if root_changed {
            match &handle.widget {
                AppKitOsWidget::Window(window) => {
                    self.closed_windows.borrow_mut().remove(&id);
                    window.makeKeyAndOrderFront(None);
                }
                AppKitOsWidget::Panel(panel) => {
                    self.closed_windows.borrow_mut().remove(&id);
                    panel.as_super().makeKeyAndOrderFront(None);
                }
                AppKitOsWidget::Menu(menu) => self._application.setMainMenu(Some(menu)),
                _ => {}
            }
        }
        self.present_visible_panels();
        self.present_visible_popovers();
        if root_changed {
            activate_current_application();
        }
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
}

impl AppKitNativeSurface {
    fn set_panel_visible(&mut self, id: HostNodeId, panel: &NSPanel, visible: bool) {
        self.dialog_visible.insert(id, visible);
        if visible {
            self.show_panel_if_marked_visible(id, panel);
        } else {
            panel.as_super().orderOut(None);
        }
    }

    fn show_panel_if_marked_visible(&mut self, id: HostNodeId, panel: &NSPanel) {
        if self.root.is_some() && self.dialog_visible.get(&id).copied().unwrap_or(false) {
            self.closed_windows.borrow_mut().remove(&id);
            panel.as_super().makeKeyAndOrderFront(None);
        }
    }

    fn present_visible_panels(&mut self) {
        let panels = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| match widget {
                AppKitOsWidget::Panel(panel)
                    if self.dialog_visible.get(id).copied().unwrap_or(false) =>
                {
                    Some((*id, panel.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (id, panel) in panels {
            self.show_panel_if_marked_visible(id, &panel);
        }
    }

    fn set_popover_visible(&mut self, id: HostNodeId, state: &AppKitPopoverState, visible: bool) {
        self.popover_visible.insert(id, visible);
        if visible {
            self.show_popover_if_marked_visible(id, state);
        } else {
            state.popover.close();
        }
    }

    fn show_popover_if_marked_visible(&mut self, id: HostNodeId, state: &AppKitPopoverState) {
        if self.root.is_none() || !self.popover_visible.get(&id).copied().unwrap_or(false) {
            return;
        }

        let Some(anchor_id) = self.popover_anchors.get(&id).copied() else {
            return;
        };
        let Some(anchor_widget) = self.widgets.get(&anchor_id).cloned() else {
            return;
        };
        let Some(anchor_view) = anchor_widget.as_view() else {
            return;
        };
        if anchor_view.window().is_none() || anchor_view.isHiddenOrHasHiddenAncestor() {
            return;
        }

        state.popover.showRelativeToRect_ofView_preferredEdge(
            anchor_view.bounds(),
            anchor_view,
            NSRectEdge::MaxY,
        );
    }

    fn present_visible_popovers(&mut self) {
        let popovers = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| match widget {
                AppKitOsWidget::Popover(state)
                    if self.popover_visible.get(id).copied().unwrap_or(false) =>
                {
                    Some((*id, state.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (id, state) in popovers {
            self.show_popover_if_marked_visible(id, &state);
        }
    }
}

fn set_widget_title(widget: &AppKitOsWidget, title: Option<&str>) {
    let title = title.map(ns_string);
    let title = title.as_deref();
    match widget {
        AppKitOsWidget::Window(window) => {
            if let Some(content_view) = window.contentView() {
                content_view.setToolTip(title);
            }
        }
        AppKitOsWidget::Panel(panel) => {
            if let Some(content_view) = panel.as_super().contentView() {
                content_view.setToolTip(title);
            }
        }
        AppKitOsWidget::Popover(state) => state.content_view.setToolTip(title),
        AppKitOsWidget::MenuItem(menu_item) => menu_item.setToolTip(title),
        AppKitOsWidget::TabViewItem(tab_item) => tab_item.setToolTip(title),
        AppKitOsWidget::Menu(_) | AppKitOsWidget::ComboBoxItem(_) => {}
        _ => {
            if let Some(view) = widget.as_view() {
                view.setToolTip(title);
            }
        }
    }
}

fn appkit_vertical_scroll_enabled(config: &NativeWidgetConfig) -> bool {
    scroll_enabled(config.portable_style.overflow_y)
        || scroll_enabled(config.portable_style.overflow_block)
        || (!scroll_enabled(config.portable_style.overflow_x)
            && !scroll_enabled(config.portable_style.overflow_inline))
}

fn appkit_horizontal_scroll_enabled(config: &NativeWidgetConfig) -> bool {
    scroll_enabled(config.portable_style.overflow_x)
        || scroll_enabled(config.portable_style.overflow_inline)
}

fn scroll_enabled(value: Option<OverflowMode>) -> bool {
    matches!(value, Some(OverflowMode::Auto | OverflowMode::Scroll))
}
