use super::*;

impl Gtk4NativeSurface {
    pub(super) fn apply_native_setter_impl(
        &mut self,
        id: HostNodeId,
        handle: &Gtk4OsHandle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        if let Some(registration) = self.interaction_nodes.borrow().get(&id) {
            let mut current = registration.get();
            current.apply_setter(setter);
            registration.set(current);
        }
        if let Some(config) = self.text_input_configs.get_mut(&id) {
            apply_widget_setter(config, setter);
        }

        match setter {
            NativeWidgetSetter::SetLabel(value) => {
                let label = value.as_deref().unwrap_or("");
                match &handle.widget {
                    Gtk4OsWidget::ApplicationWindow(window) => window.set_title(Some(label)),
                    Gtk4OsWidget::Dialog(dialog) => dialog.set_title(Some(label)),
                    Gtk4OsWidget::Label(widget) => {
                        widget.set_text(label);
                        if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                            self.update_notebook_tab_label(id, &tab, label.to_string())?;
                        }
                    }
                    Gtk4OsWidget::Button(button) => button.set_label(label),
                    Gtk4OsWidget::CheckButton(check_button) => {
                        check_button.set_label(Some(label));
                    }
                    Gtk4OsWidget::ListBoxRow {
                        label: label_widget,
                        item,
                        ..
                    } => {
                        label_widget.set_text(label);
                        if let Some(label) = value {
                            self.update_drop_down_item_label(id, item, label.clone())?;
                        }
                    }
                    Gtk4OsWidget::MenuItem(item) => {
                        self.menus.update_item_label(id, item, label.to_string());
                    }
                    Gtk4OsWidget::Entry(_)
                    | Gtk4OsWidget::SearchEntry(_)
                    | Gtk4OsWidget::PasswordEntry(_)
                    | Gtk4OsWidget::SpinButton(_)
                    | Gtk4OsWidget::TextView(_)
                    | Gtk4OsWidget::Popover(_)
                    | Gtk4OsWidget::Menu(_)
                    | Gtk4OsWidget::Switch(_)
                    | Gtk4OsWidget::DropDown(_)
                    | Gtk4OsWidget::ListBox(_)
                    | Gtk4OsWidget::ScrolledWindow { .. }
                    | Gtk4OsWidget::Notebook(_)
                    | Gtk4OsWidget::Separator(_)
                    | Gtk4OsWidget::Scale(_)
                    | Gtk4OsWidget::ProgressBar(_)
                    | Gtk4OsWidget::Box(_) => {}
                }
            }
            NativeWidgetSetter::SetAccessibilityLabel(value) => {
                if let Some(widget) = handle.widget.as_widget() {
                    if let Some(label) = value.as_deref() {
                        widget.update_property(&[gtk::accessible::Property::Label(label)]);
                    } else {
                        widget.reset_property(gtk::AccessibleProperty::Label);
                    }
                }
            }
            NativeWidgetSetter::SetWindowResizable(value) => {
                if let Gtk4OsWidget::ApplicationWindow(window) = &handle.widget {
                    window.set_resizable(value.unwrap_or(true));
                }
            }
            NativeWidgetSetter::SetValue(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    let max_length = self
                        .text_input_max_lengths
                        .borrow()
                        .get(&id)
                        .copied()
                        .flatten();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                }
                Gtk4OsWidget::SearchEntry(entry) => {
                    let max_length = self
                        .text_input_max_lengths
                        .borrow()
                        .get(&id)
                        .copied()
                        .flatten();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                }
                Gtk4OsWidget::PasswordEntry(entry) => {
                    let max_length = self
                        .text_input_max_lengths
                        .borrow()
                        .get(&id)
                        .copied()
                        .flatten();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                }
                Gtk4OsWidget::SpinButton(spin_button) => {
                    if let Some(value) = parse_gtk_number_value(value.as_deref()) {
                        let range = self.ranges.entry(id).or_default();
                        range.current = Some(value);
                        spin_button.set_digits(range.spin_button_digits());
                        self.suppress_events(|| spin_button.set_value(value));
                    }
                }
                Gtk4OsWidget::TextView(text_view) => {
                    let buffer = text_view.buffer();
                    let max_length = self
                        .text_input_max_lengths
                        .borrow()
                        .get(&id)
                        .copied()
                        .flatten();
                    self.suppress_events(|| {
                        set_text_buffer_text(&buffer, value.as_deref().unwrap_or(""), max_length);
                    });
                }
                Gtk4OsWidget::Label(label) => {
                    if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                        if let Some(value) = value {
                            self.update_notebook_tab_value(id, &tab, value.clone())?;
                        }
                    } else if let Some(value) = value.as_deref() {
                        label.set_text(value);
                    }
                }
                Gtk4OsWidget::DropDown(_) => {
                    self.set_drop_down_value(id, value.as_deref());
                }
                Gtk4OsWidget::Notebook(_) => {
                    self.set_notebook_value(id, value.as_deref());
                }
                Gtk4OsWidget::ListBoxRow { item, .. } => {
                    self.update_drop_down_item_value(
                        id,
                        item,
                        value.clone().unwrap_or_else(|| item.label.clone()),
                    )?;
                }
                Gtk4OsWidget::MenuItem(item) => {
                    self.menus.update_item_value(
                        id,
                        item,
                        value.clone().unwrap_or_else(|| item.label.clone()),
                    );
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::ScrolledWindow { .. }
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::Separator(_)
                | Gtk4OsWidget::Scale(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetPlaceholder(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    entry.set_placeholder_text(value.as_deref());
                }
                Gtk4OsWidget::PasswordEntry(entry) => {
                    entry.set_placeholder_text(value.as_deref());
                }
                Gtk4OsWidget::SearchEntry(entry) => {
                    entry.set_placeholder_text(value.as_deref());
                }
                _ => {}
            },
            NativeWidgetSetter::SetTitle(value) => {
                set_widget_title(&handle.widget, value.as_deref());
            }
            NativeWidgetSetter::SetEnabled(value) => {
                if let Some(widget) = handle.widget.as_widget() {
                    widget.set_sensitive(*value);
                }
            }
            NativeWidgetSetter::SetReadOnly(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    entry.set_editable(!*value);
                }
                Gtk4OsWidget::PasswordEntry(entry) => {
                    entry.set_editable(!*value);
                }
                Gtk4OsWidget::SearchEntry(entry) => {
                    entry.set_editable(!*value);
                }
                Gtk4OsWidget::SpinButton(spin_button) => {
                    spin_button.set_editable(!*value);
                }
                Gtk4OsWidget::TextView(text_view) => {
                    text_view.set_editable(!*value);
                }
                _ => {}
            },
            NativeWidgetSetter::SetVisible(value) => {
                if let Gtk4OsWidget::Dialog(dialog) = &handle.widget {
                    self.set_dialog_visible(id, dialog, *value);
                }
                if let Some(widget) = handle.widget.as_widget() {
                    widget.set_visible(*value);
                }
            }
            NativeWidgetSetter::SetChecked(value) => match &handle.widget {
                Gtk4OsWidget::CheckButton(check_button) => {
                    self.suppress_events(|| check_button.set_active(value.unwrap_or(false)));
                }
                Gtk4OsWidget::Switch(switch) => {
                    self.suppress_events(|| switch.set_active(value.unwrap_or(false)));
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::SearchEntry(_)
                | Gtk4OsWidget::PasswordEntry(_)
                | Gtk4OsWidget::SpinButton(_)
                | Gtk4OsWidget::TextView(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ScrolledWindow { .. }
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_)
                | Gtk4OsWidget::Scale(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetSelected(value) => {
                if let Gtk4OsWidget::ListBoxRow { row, item, .. } = &handle.widget {
                    self.update_drop_down_item_selected(id, item, *value)?;
                    self.update_list_item_selected(id, row, *value);
                }
                if let Gtk4OsWidget::MenuItem(item) = &handle.widget {
                    self.menus.update_item_selected(id, item, *value);
                }
                if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                    self.update_notebook_tab_selected(id, &tab, *value)?;
                }
            }
            NativeWidgetSetter::SetMultiple(multiple) => {
                if let Gtk4OsWidget::ListBox(list_box) = &handle.widget {
                    self.suppress_events(|| {
                        list_box.set_selection_mode(if *multiple {
                            gtk::SelectionMode::Multiple
                        } else {
                            gtk::SelectionMode::Single
                        });
                    });
                }
            }
            NativeWidgetSetter::SetClassName(value) => {
                if let (Some(widget), Some(class_name)) = (handle.widget.as_widget(), value) {
                    for class_name in class_name.split_whitespace() {
                        widget.add_css_class(class_name);
                    }
                }
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                if let Some(widget) = handle.widget.as_widget() {
                    apply_widget_size(&widget, style);
                }
                if let Gtk4OsWidget::Entry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().has_explicit_width =
                        style_sets_gtk_width(style);
                    self.apply_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SearchEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().has_explicit_width =
                        style_sets_gtk_width(style);
                    self.apply_search_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::PasswordEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().has_explicit_width =
                        style_sets_gtk_width(style);
                    self.apply_password_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SpinButton(spin_button) = &handle.widget {
                    self.text_inputs.entry(id).or_default().has_explicit_width =
                        style_sets_gtk_width(style);
                    self.apply_spin_button_width_hint(id, spin_button);
                }
                if let Gtk4OsWidget::TextView(text_view) = &handle.widget {
                    let sizing = self.text_inputs.entry(id).or_default();
                    sizing.has_explicit_width = style_sets_gtk_width(style);
                    sizing.has_explicit_height = style_sets_gtk_height(style);
                    self.apply_text_view_size_hint(id, text_view);
                }
                if let Gtk4OsWidget::Box(box_) = &handle.widget {
                    if let Some(orientation) = style.flex_direction {
                        box_.set_orientation(gtk_orientation(orientation));
                    }
                    let gap = style
                        .gap
                        .as_ref()
                        .and_then(StyleLength::points)
                        .unwrap_or(0.0);
                    box_.set_spacing(points_to_i32(gap));
                }
                if let Gtk4OsWidget::ScrolledWindow {
                    scrolled_window,
                    content,
                } = &handle.widget
                {
                    scrolled_window.set_policy(
                        gtk4_scroll_policy(style.overflow_x),
                        gtk4_scroll_policy(style.overflow_y),
                    );
                    if let Some(orientation) = style.flex_direction {
                        content.set_orientation(gtk_orientation(orientation));
                    }
                    let gap = style
                        .gap
                        .as_ref()
                        .and_then(StyleLength::points)
                        .unwrap_or(0.0);
                    content.set_spacing(points_to_i32(gap));
                }
            }
            NativeWidgetSetter::SetOrientation(value) => match &handle.widget {
                Gtk4OsWidget::Box(box_) => {
                    if let Some(value) = value {
                        box_.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::ScrolledWindow { content, .. } => {
                    if let Some(value) = value {
                        content.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::Separator(separator) => {
                    if let Some(value) = value {
                        separator.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::Scale(scale) => {
                    if let Some(value) = value {
                        scale.set_orientation(gtk_orientation(*value));
                    }
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::SearchEntry(_)
                | Gtk4OsWidget::PasswordEntry(_)
                | Gtk4OsWidget::SpinButton(_)
                | Gtk4OsWidget::TextView(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetMinimum(value) => {
                let range = self.ranges.entry(id).or_default();
                range.min = *value;
                match &handle.widget {
                    Gtk4OsWidget::Scale(scale) => {
                        scale.set_range(range.lower(), range.upper());
                    }
                    Gtk4OsWidget::SpinButton(spin_button) => {
                        spin_button.set_range(range.lower(), range.upper());
                        spin_button.set_digits(range.spin_button_digits());
                    }
                    Gtk4OsWidget::ProgressBar(progress_bar) => {
                        set_progress_bar_fraction(progress_bar, *range);
                    }
                    _ => {}
                }
            }
            NativeWidgetSetter::SetMaximum(value) => {
                let range = self.ranges.entry(id).or_default();
                range.max = *value;
                match &handle.widget {
                    Gtk4OsWidget::Scale(scale) => {
                        scale.set_range(range.lower(), range.upper());
                    }
                    Gtk4OsWidget::SpinButton(spin_button) => {
                        spin_button.set_range(range.lower(), range.upper());
                        spin_button.set_digits(range.spin_button_digits());
                    }
                    Gtk4OsWidget::ProgressBar(progress_bar) => {
                        set_progress_bar_fraction(progress_bar, *range);
                    }
                    _ => {}
                }
            }
            NativeWidgetSetter::SetCurrent(value) => match &handle.widget {
                Gtk4OsWidget::Scale(scale) => {
                    let range = self.ranges.entry(id).or_default();
                    range.current = *value;
                    scale.set_value(range.current());
                }
                Gtk4OsWidget::SpinButton(spin_button) => {
                    let current = {
                        let range = self.ranges.entry(id).or_default();
                        range.current = *value;
                        spin_button.set_digits(range.spin_button_digits());
                        range.current()
                    };
                    self.suppress_events(|| spin_button.set_value(current));
                }
                Gtk4OsWidget::ProgressBar(progress_bar) => {
                    let range = self.ranges.entry(id).or_default();
                    range.current = *value;
                    set_progress_bar_fraction(progress_bar, *range);
                }
                Gtk4OsWidget::ApplicationWindow(_)
                | Gtk4OsWidget::Dialog(_)
                | Gtk4OsWidget::Popover(_)
                | Gtk4OsWidget::Menu(_)
                | Gtk4OsWidget::MenuItem(_)
                | Gtk4OsWidget::Box(_)
                | Gtk4OsWidget::Label(_)
                | Gtk4OsWidget::Button(_)
                | Gtk4OsWidget::Entry(_)
                | Gtk4OsWidget::SearchEntry(_)
                | Gtk4OsWidget::PasswordEntry(_)
                | Gtk4OsWidget::TextView(_)
                | Gtk4OsWidget::CheckButton(_)
                | Gtk4OsWidget::Switch(_)
                | Gtk4OsWidget::DropDown(_)
                | Gtk4OsWidget::ListBox(_)
                | Gtk4OsWidget::ScrolledWindow { .. }
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_) => {}
            },
            NativeWidgetSetter::SetStep(value) => {
                let range = self.ranges.entry(id).or_default();
                range.step = *value;
                if let Gtk4OsWidget::Scale(scale) = &handle.widget {
                    scale.set_increments(range.step(), range.step() * 10.0);
                }
                if let Gtk4OsWidget::SpinButton(spin_button) = &handle.widget {
                    spin_button.set_increments(range.step(), range.step() * 10.0);
                    spin_button.set_digits(range.spin_button_digits());
                }
            }
            NativeWidgetSetter::SetMaxLength(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    self.text_input_max_lengths.borrow_mut().insert(id, *value);
                    entry.set_max_length(value.map(u32_to_i32).unwrap_or(0));
                    let current = entry.text().to_string();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(&current, *value));
                    });
                }
                Gtk4OsWidget::SearchEntry(entry) => {
                    self.text_input_max_lengths.borrow_mut().insert(id, *value);
                    let current = entry.text().to_string();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(&current, *value));
                    });
                }
                Gtk4OsWidget::PasswordEntry(entry) => {
                    self.text_input_max_lengths.borrow_mut().insert(id, *value);
                    let current = entry.text().to_string();
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(&current, *value));
                    });
                }
                Gtk4OsWidget::TextView(text_view) => {
                    self.text_input_max_lengths.borrow_mut().insert(id, *value);
                    let buffer = text_view.buffer();
                    let current = text_buffer_text(&buffer);
                    self.suppress_events(|| {
                        set_text_buffer_text(&buffer, &current, *value);
                    });
                }
                _ => {}
            },
            NativeWidgetSetter::SetCols(value) => {
                if let Gtk4OsWidget::Entry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SearchEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_search_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::PasswordEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_password_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SpinButton(spin_button) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_spin_button_width_hint(id, spin_button);
                }
                if let Gtk4OsWidget::TextView(text_view) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_text_view_size_hint(id, text_view);
                }
            }
            NativeWidgetSetter::SetSize(value) => {
                if let Gtk4OsWidget::Entry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SearchEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_search_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::PasswordEntry(entry) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_password_entry_width_hint(id, entry);
                }
                if let Gtk4OsWidget::SpinButton(spin_button) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_spin_button_width_hint(id, spin_button);
                }
            }
            NativeWidgetSetter::SetRows(value) => {
                if let Gtk4OsWidget::TextView(text_view) = &handle.widget {
                    self.text_inputs.entry(id).or_default().rows = *value;
                    self.apply_text_view_size_hint(id, text_view);
                }
            }
            NativeWidgetSetter::SetAutocomplete(_)
            | NativeWidgetSetter::SetInputMode(_)
            | NativeWidgetSetter::SetAutoCapitalize(_)
            | NativeWidgetSetter::SetAutoCorrect(_)
            | NativeWidgetSetter::SetVirtualKeyboardPolicy(_)
            | NativeWidgetSetter::SetInputType(_)
            | NativeWidgetSetter::SetSpellCheck(_) => {
                self.apply_text_input_hints(id, &handle.widget);
            }
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetAutoFocus(_)
            | NativeWidgetSetter::SetRequired(_)
            | NativeWidgetSetter::SetInvalid(_)
            | NativeWidgetSetter::SetExpanded(_)
            | NativeWidgetSetter::SetPattern(_)
            | NativeWidgetSetter::SetMinLength(_)
            | NativeWidgetSetter::SetName(_)
            | NativeWidgetSetter::SetForm(_)
            | NativeWidgetSetter::SetAccept(_)
            | NativeWidgetSetter::SetCapture(_)
            | NativeWidgetSetter::SetEnterKeyHint(_)
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
}
