use super::*;

impl AppKitNativeSurface {
    pub(super) fn create_native_widget_impl(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<AppKitOsHandle> {
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                    self.activation_contexts.clone(),
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
                        self.activation_contexts.clone(),
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
                        self.activation_contexts.clone(),
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
                        self.activation_contexts.clone(),
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
        self.interaction_nodes.insert(
            id,
            interaction::AppKitInteractionRegistration::new(&handle.widget, blueprint),
        );
        self.register_responder(id, &handle.widget);
        self.widgets.insert(id, handle.widget.clone());
        Ok(handle)
    }
}
