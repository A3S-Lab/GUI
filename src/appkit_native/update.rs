use super::*;

impl AppKitNativeSurface {
    pub(super) fn apply_native_setter_impl(
        &mut self,
        id: HostNodeId,
        handle: &AppKitOsHandle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        let cancel_number_field_stepper =
            if let Some(registration) = self.interaction_nodes.get_mut(&id) {
                registration.apply_setter(setter);
                !registration.tracks_number_field_stepper()
            } else {
                false
            };
        if cancel_number_field_stepper {
            if let Some(active) = self
                .pointer_press
                .borrow_mut()
                .as_mut()
                .filter(|active| active.node == id)
            {
                active.cancel_number_field_stepper();
            }
        }
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
            NativeWidgetSetter::SetAccessibilityLabel(value) => {
                set_appkit_accessibility_label(&handle.widget, value.as_deref());
            }
            NativeWidgetSetter::SetAccessibilityDescription(value) => {
                set_appkit_accessibility_description(&handle.widget, value);
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
                if let AppKitOsWidget::ComboBoxItem(item) = &handle.widget {
                    self.update_option_item_visible(id, item, *value)?;
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
                    self.update_option_item_style(id, item, style.as_ref().clone())?;
                }
                if let AppKitOsWidget::ListView(scroll_view) = &handle.widget {
                    if let Some(state) = self.list_views.get_mut(&id) {
                        state.style = style.as_ref().clone();
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
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetAutoFocus(_)
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
            | NativeWidgetSetter::SetAccessibilityStructure(_)
            | NativeWidgetSetter::SetAccessibilityState(_)
            | NativeWidgetSetter::SetWebStyle(_)
            | NativeWidgetSetter::SetEvents(_)
            | NativeWidgetSetter::SetMetadata(_) => {}
        }
        Ok(())
    }
}

fn set_appkit_accessibility_label(widget: &AppKitOsWidget, value: Option<&str>) {
    let label = value.map(ns_string);
    let label = label.as_deref();
    match widget {
        AppKitOsWidget::Window(window) => window.setAccessibilityLabel(label),
        AppKitOsWidget::Panel(panel) => panel.setAccessibilityLabel(label),
        AppKitOsWidget::Popover(state) => state.popover.setAccessibilityLabel(label),
        AppKitOsWidget::Menu(menu) => menu.setAccessibilityLabel(label),
        AppKitOsWidget::MenuItem(item) => item.setAccessibilityLabel(label),
        AppKitOsWidget::ComboBoxItem(_) | AppKitOsWidget::TabViewItem(_) => {}
        _ => {
            if let Some(view) = widget.as_view() {
                view.setAccessibilityLabel(label);
            }
        }
    }
}

fn set_appkit_accessibility_description(
    widget: &AppKitOsWidget,
    value: &crate::accessibility::AccessibilityDescriptionProps,
) {
    let description = value.description.as_deref().map(ns_string);
    let role_description = value.role_description.as_deref().map(ns_string);
    let value_text = value.value_text.as_deref().map(ns_string);

    macro_rules! apply {
        ($target:expr) => {{
            let target = $target;
            target.setAccessibilityHelp(description.as_deref());
            target.setAccessibilityRoleDescription(role_description.as_deref());
            target.setAccessibilityValueDescription(value_text.as_deref());
        }};
    }

    match widget {
        AppKitOsWidget::Window(window) => apply!(window),
        AppKitOsWidget::Panel(panel) => apply!(panel),
        AppKitOsWidget::Popover(state) => apply!(&state.popover),
        AppKitOsWidget::Menu(menu) => apply!(menu),
        AppKitOsWidget::MenuItem(item) => apply!(item),
        AppKitOsWidget::ComboBoxItem(_) | AppKitOsWidget::TabViewItem(_) => {}
        _ => {
            if let Some(view) = widget.as_view() {
                apply!(view);
            }
        }
    }
}
