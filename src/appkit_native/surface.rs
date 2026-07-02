use super::*;

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
            NativeWidgetSetter::SetReadOnly(value) => {
                if let AppKitOsWidget::TextField(text_field) = &handle.widget {
                    text_field.setEditable(!*value);
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
                match &handle.widget {
                    AppKitOsWidget::Window(window) => {
                        apply_window_size_constraints(window, style);
                    }
                    AppKitOsWidget::Panel(panel) => {
                        apply_window_size_constraints(panel.as_super(), style);
                    }
                    _ => {}
                }
                if let Some(view) = handle.widget.as_view() {
                    let width = style.width.as_ref().and_then(StyleLength::points);
                    let height = style.height.as_ref().and_then(StyleLength::points);
                    if width.is_some() || height.is_some() {
                        view.setFrameSize(NSSize::new(
                            width.unwrap_or(120.0),
                            height.unwrap_or(32.0),
                        ));
                    }
                }
                if let AppKitOsWidget::Popover(state) = &handle.widget {
                    let width = style.width.as_ref().and_then(StyleLength::points);
                    let height = style.height.as_ref().and_then(StyleLength::points);
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
            NativeWidgetSetter::SetStep(_) => {}
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
            | NativeWidgetSetter::SetMultiple(_)
            | NativeWidgetSetter::SetAutoFocus(_)
            | NativeWidgetSetter::SetExpanded(_)
            | NativeWidgetSetter::SetAutocomplete(_)
            | NativeWidgetSetter::SetInputMode(_)
            | NativeWidgetSetter::SetPattern(_)
            | NativeWidgetSetter::SetMinLength(_)
            | NativeWidgetSetter::SetMaxLength(_)
            | NativeWidgetSetter::SetRows(_)
            | NativeWidgetSetter::SetCols(_)
            | NativeWidgetSetter::SetSize(_)
            | NativeWidgetSetter::SetName(_)
            | NativeWidgetSetter::SetForm(_)
            | NativeWidgetSetter::SetInputType(_)
            | NativeWidgetSetter::SetAccept(_)
            | NativeWidgetSetter::SetCapture(_)
            | NativeWidgetSetter::SetEnterKeyHint(_)
            | NativeWidgetSetter::SetAutoCapitalize(_)
            | NativeWidgetSetter::SetAutoCorrect(_)
            | NativeWidgetSetter::SetVirtualKeyboardPolicy(_)
            | NativeWidgetSetter::SetTitle(_)
            | NativeWidgetSetter::SetHidden(_)
            | NativeWidgetSetter::SetLang(_)
            | NativeWidgetSetter::SetDir(_)
            | NativeWidgetSetter::SetTabIndex(_)
            | NativeWidgetSetter::SetExplicitRole(_)
            | NativeWidgetSetter::SetAccessKey(_)
            | NativeWidgetSetter::SetContentEditable(_)
            | NativeWidgetSetter::SetDraggable(_)
            | NativeWidgetSetter::SetSpellCheck(_)
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
