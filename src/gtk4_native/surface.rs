use super::*;

impl NativeWidgetSurface for Gtk4NativeSurface {
    type Handle = Gtk4OsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let kind = Gtk4WidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let config = blueprint.config();
        let widget = match kind {
            Gtk4WidgetKind::ApplicationWindow => {
                let size = config.portable_style.native_size_constraints();
                let window = gtk::ApplicationWindow::builder()
                    .application(&self.application)
                    .title(config.label.as_deref().unwrap_or(""))
                    .default_width(config_dimension(size.width, 640))
                    .default_height(config_dimension(size.height, 480))
                    .resizable(config.window_resizable.unwrap_or(true))
                    .build();
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let closed_windows = self.closed_windows.clone();
                window.connect_close_request(move |_| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Close),
                    );
                    closed_windows.borrow_mut().insert(id);
                    gtk::glib::Propagation::Proceed
                });
                Gtk4OsWidget::ApplicationWindow(window)
            }
            Gtk4WidgetKind::Box | Gtk4WidgetKind::ToolbarBox => {
                let box_ = gtk::Box::new(
                    config_orientation(&config).unwrap_or(gtk::Orientation::Vertical),
                    config
                        .portable_style
                        .gap
                        .as_ref()
                        .and_then(StyleLength::points)
                        .map(points_to_i32)
                        .unwrap_or(0),
                );
                Gtk4OsWidget::Box(box_)
            }
            Gtk4WidgetKind::ScrolledWindow => {
                let scrolled_window = gtk::ScrolledWindow::new();
                scrolled_window.set_policy(
                    gtk4_scroll_policy(config.portable_style.overflow_x),
                    gtk4_scroll_policy(config.portable_style.overflow_y),
                );
                let box_ = gtk::Box::new(
                    config_orientation(&config).unwrap_or(gtk::Orientation::Vertical),
                    config
                        .portable_style
                        .gap
                        .as_ref()
                        .and_then(StyleLength::points)
                        .map(points_to_i32)
                        .unwrap_or(0),
                );
                scrolled_window.set_child(Some(&box_));
                apply_widget_size(scrolled_window.as_ref(), &config.portable_style);
                Gtk4OsWidget::ScrolledWindow {
                    scrolled_window,
                    content: box_,
                }
            }
            Gtk4WidgetKind::Label => Gtk4OsWidget::Label(gtk::Label::new(Some(
                config
                    .label
                    .as_deref()
                    .or(config.value.as_deref())
                    .unwrap_or(""),
            ))),
            Gtk4WidgetKind::Button => {
                let button = gtk::Button::with_label(config.label.as_deref().unwrap_or(""));
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                button.connect_clicked(move |_| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Press),
                    );
                });
                Gtk4OsWidget::Button(button)
            }
            Gtk4WidgetKind::Entry => {
                if config_is_search(&config) {
                    let entry = gtk::SearchEntry::new();
                    let max_length = config.max_length;
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            config.value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                    if let Some(placeholder) = config.placeholder.as_deref() {
                        entry.set_placeholder_text(Some(placeholder));
                    }
                    entry.set_editable(!config.read_only);
                    self.text_inputs
                        .insert(id, Gtk4TextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    self.text_input_max_lengths
                        .borrow_mut()
                        .insert(id, config.max_length);
                    self.apply_search_entry_width_hint(id, &entry);

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    let text_input_max_lengths = self.text_input_max_lengths.clone();
                    entry.connect_search_changed(move |entry| {
                        if *events_suppressed.borrow() {
                            return;
                        }

                        let raw_value = entry.text().to_string();
                        let max_length =
                            text_input_max_lengths.borrow().get(&id).copied().flatten();
                        let value = truncate_to_max_length(&raw_value, max_length);
                        if value != raw_value {
                            let previous = events_suppressed.replace(true);
                            entry.set_text(&value);
                            events_suppressed.replace(previous);
                        }
                        events
                            .borrow_mut()
                            .push(NativeEvent::new(id, NativeEventKind::Change).value(value));
                    });

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    entry.connect_has_focus_notify(move |entry| {
                        let kind = if entry.has_focus() {
                            NativeEventKind::Focus
                        } else {
                            NativeEventKind::Blur
                        };
                        push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                    });

                    Gtk4OsWidget::SearchEntry(entry)
                } else if config_is_password(&config) {
                    let entry = gtk::PasswordEntry::new();
                    let max_length = config.max_length;
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            config.value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                    if let Some(placeholder) = config.placeholder.as_deref() {
                        entry.set_placeholder_text(Some(placeholder));
                    }
                    entry.set_editable(!config.read_only);
                    self.text_inputs
                        .insert(id, Gtk4TextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    self.text_input_max_lengths
                        .borrow_mut()
                        .insert(id, config.max_length);
                    self.apply_password_entry_width_hint(id, &entry);

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    let text_input_max_lengths = self.text_input_max_lengths.clone();
                    entry.connect_changed(move |entry| {
                        if *events_suppressed.borrow() {
                            return;
                        }

                        let raw_value = entry.text().to_string();
                        let max_length =
                            text_input_max_lengths.borrow().get(&id).copied().flatten();
                        let value = truncate_to_max_length(&raw_value, max_length);
                        if value != raw_value {
                            let previous = events_suppressed.replace(true);
                            entry.set_text(&value);
                            events_suppressed.replace(previous);
                        }
                        events
                            .borrow_mut()
                            .push(NativeEvent::new(id, NativeEventKind::Change).value(value));
                    });

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    entry.connect_has_focus_notify(move |entry| {
                        let kind = if entry.has_focus() {
                            NativeEventKind::Focus
                        } else {
                            NativeEventKind::Blur
                        };
                        push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                    });

                    Gtk4OsWidget::PasswordEntry(entry)
                } else {
                    let entry = gtk::Entry::new();
                    self.suppress_events(|| {
                        entry.set_text(config.value.as_deref().unwrap_or(""));
                    });
                    if let Some(placeholder) = config.placeholder.as_deref() {
                        entry.set_placeholder_text(Some(placeholder));
                    }
                    self.text_inputs
                        .insert(id, Gtk4TextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    self.apply_entry_width_hint(id, &entry);

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    entry.connect_changed(move |entry| {
                        push_event(
                            &events,
                            &events_suppressed,
                            NativeEvent::new(id, NativeEventKind::Change)
                                .value(entry.text().to_string()),
                        );
                    });

                    let events = self.events.clone();
                    let events_suppressed = self.events_suppressed.clone();
                    entry.connect_has_focus_notify(move |entry| {
                        let kind = if entry.has_focus() {
                            NativeEventKind::Focus
                        } else {
                            NativeEventKind::Blur
                        };
                        push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                    });

                    Gtk4OsWidget::Entry(entry)
                }
            }
            Gtk4WidgetKind::SpinButton => {
                let range = Gtk4RangeState::from_config(&config);
                let spin_button =
                    gtk::SpinButton::with_range(range.lower(), range.upper(), range.step());
                spin_button.set_numeric(true);
                spin_button.set_update_policy(gtk::SpinButtonUpdatePolicy::IfValid);
                spin_button.set_digits(range.spin_button_digits());
                spin_button.set_increments(range.step(), range.step() * 10.0);
                spin_button.set_value(range.current());
                spin_button.set_editable(!config.read_only);
                self.ranges.insert(id, range);
                self.text_inputs
                    .insert(id, Gtk4TextInputSizing::from_config(&config));
                self.apply_spin_button_width_hint(id, &spin_button);

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                spin_button.connect_value_changed(move |spin_button| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Change)
                            .value(spin_button.value().to_string()),
                    );
                });

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                spin_button.connect_has_focus_notify(move |spin_button| {
                    let kind = if spin_button.has_focus() {
                        NativeEventKind::Focus
                    } else {
                        NativeEventKind::Blur
                    };
                    push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                });

                Gtk4OsWidget::SpinButton(spin_button)
            }
            Gtk4WidgetKind::TextView => {
                let text_view = gtk::TextView::new();
                text_view.set_wrap_mode(gtk::WrapMode::WordChar);
                text_view.set_editable(!config.read_only);
                if let Some(placeholder) = config.placeholder.as_deref() {
                    text_view.set_placeholder_text(Some(placeholder));
                }
                let buffer = text_view.buffer();
                let max_length = config.max_length;
                self.suppress_events(|| {
                    set_text_buffer_text(
                        &buffer,
                        config.value.as_deref().unwrap_or(""),
                        max_length,
                    );
                });
                self.text_inputs
                    .insert(id, Gtk4TextInputSizing::from_config(&config));
                self.text_input_configs.insert(id, config.clone());
                self.text_input_max_lengths
                    .borrow_mut()
                    .insert(id, config.max_length);
                self.apply_text_view_size_hint(id, &text_view);

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let text_input_max_lengths = self.text_input_max_lengths.clone();
                buffer.connect_changed(move |buffer| {
                    if *events_suppressed.borrow() {
                        return;
                    }

                    let raw_value = text_buffer_text(buffer);
                    let max_length = text_input_max_lengths.borrow().get(&id).copied().flatten();
                    let value = truncate_to_max_length(&raw_value, max_length);
                    if value != raw_value {
                        let previous = events_suppressed.replace(true);
                        buffer.set_text(&value);
                        events_suppressed.replace(previous);
                    }
                    events
                        .borrow_mut()
                        .push(NativeEvent::new(id, NativeEventKind::Change).value(value));
                });

                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                text_view.connect_has_focus_notify(move |text_view| {
                    let kind = if text_view.has_focus() {
                        NativeEventKind::Focus
                    } else {
                        NativeEventKind::Blur
                    };
                    push_event(&events, &events_suppressed, NativeEvent::new(id, kind));
                });

                Gtk4OsWidget::TextView(text_view)
            }
            Gtk4WidgetKind::CheckButton => {
                let check_button =
                    gtk::CheckButton::with_label(config.label.as_deref().unwrap_or(""));
                self.suppress_events(|| {
                    check_button.set_active(config.checked.unwrap_or(false));
                });
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                check_button.connect_toggled(move |check_button| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Toggle)
                            .value(check_button.is_active().to_string()),
                    );
                });
                Gtk4OsWidget::CheckButton(check_button)
            }
            Gtk4WidgetKind::Switch => {
                let switch = gtk::Switch::new();
                self.suppress_events(|| {
                    switch.set_active(config.checked.unwrap_or(false));
                });
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                switch.connect_active_notify(move |switch| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Toggle)
                            .value(switch.is_active().to_string()),
                    );
                });
                Gtk4OsWidget::Switch(switch)
            }
            Gtk4WidgetKind::DropDown => {
                let model = gtk::StringList::new(&[]);
                let drop_down = gtk::DropDown::from_strings(&[]);
                drop_down.set_model(Some(&model));
                if let Some(value) = config.value.clone() {
                    self.drop_down_selected_values.insert(id, Some(value));
                }
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let drop_down_values = self.drop_down_values.clone();
                drop_down.connect_selected_notify(move |drop_down| {
                    let selected = drop_down.selected();
                    let value = if selected == gtk::INVALID_LIST_POSITION {
                        String::new()
                    } else {
                        drop_down_values
                            .borrow()
                            .get(&id)
                            .and_then(|values| values.get(selected as usize).cloned())
                            .unwrap_or_default()
                    };
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
                    );
                });
                self.drop_downs.insert(
                    id,
                    Gtk4DropDownState {
                        drop_down: drop_down.clone(),
                        model,
                    },
                );
                self.drop_down_children.entry(id).or_default();
                Gtk4OsWidget::DropDown(drop_down)
            }
            Gtk4WidgetKind::ListBox => Gtk4OsWidget::ListBox(gtk::ListBox::new()),
            Gtk4WidgetKind::Dialog => {
                let dialog = gtk::Dialog::builder()
                    .application(&self.application)
                    .title(config.label.as_deref().unwrap_or(""))
                    .default_width(config_dimension(config.portable_style.width, 420))
                    .default_height(config_dimension(config.portable_style.height, 280))
                    .build();
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let closed_windows = self.closed_windows.clone();
                dialog.connect_close_request(move |_| {
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::Close),
                    );
                    closed_windows.borrow_mut().insert(id);
                    gtk::glib::Propagation::Proceed
                });
                Gtk4OsWidget::Dialog(dialog)
            }
            Gtk4WidgetKind::Popover => {
                let popover = gtk::Popover::builder()
                    .autohide(true)
                    .has_arrow(true)
                    .build();
                Gtk4OsWidget::Popover(popover)
            }
            Gtk4WidgetKind::Menu => {
                let menu = Gtk4Menu::new();
                self.menus.register_menu(id, menu.clone());
                Gtk4OsWidget::Menu(menu)
            }
            Gtk4WidgetKind::MenuItem => {
                let item = Gtk4MenuItem::from_config(
                    id,
                    &config,
                    &self.application,
                    self.events.clone(),
                    self.events_suppressed.clone(),
                );
                self.menus.register_item(id, item.clone());
                Gtk4OsWidget::MenuItem(item)
            }
            Gtk4WidgetKind::ListBoxRow => {
                let item = Gtk4DropDownItem::from_config(&config);
                let label = gtk::Label::new(Some(&item.label));
                let row = gtk::ListBoxRow::new();
                row.set_child(Some(&label));
                self.drop_down_items.insert(id, item.clone());
                Gtk4OsWidget::ListBoxRow { row, label, item }
            }
            Gtk4WidgetKind::Notebook => {
                let notebook = gtk::Notebook::new();
                if let Some(value) = config.value.clone() {
                    self.notebook_selected_values.insert(id, Some(value));
                }
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let notebook_values = self.notebook_values.clone();
                notebook.connect_switch_page(move |_, _, page_num| {
                    let value = notebook_values
                        .borrow()
                        .get(&id)
                        .and_then(|values| values.get(page_num as usize).cloned())
                        .unwrap_or_else(|| page_num.to_string());
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
                    );
                });
                self.notebooks.insert(id, notebook.clone());
                self.notebook_children.entry(id).or_default();
                Gtk4OsWidget::Notebook(notebook)
            }
            Gtk4WidgetKind::Separator => Gtk4OsWidget::Separator(gtk::Separator::new(
                config_orientation(&config).unwrap_or(gtk::Orientation::Horizontal),
            )),
            Gtk4WidgetKind::Scale => {
                let range = Gtk4RangeState::from_config(&config);
                let min = range.lower();
                let max = range.upper();
                let scale = gtk::Scale::with_range(
                    config_orientation(&config).unwrap_or(gtk::Orientation::Horizontal),
                    min,
                    max,
                    range.step(),
                );
                scale.set_increments(range.step(), range.step() * 10.0);
                scale.set_value(range.current());
                self.ranges.insert(id, range);
                Gtk4OsWidget::Scale(scale)
            }
            Gtk4WidgetKind::ProgressBar => {
                let range = Gtk4RangeState::from_config(&config);
                let progress_bar = gtk::ProgressBar::new();
                set_progress_bar_fraction(&progress_bar, range);
                self.ranges.insert(id, range);
                Gtk4OsWidget::ProgressBar(progress_bar)
            }
            other => {
                return Err(GuiError::host(format!(
                    "GTK4 native surface does not support {other:?} yet"
                )));
            }
        };

        if kind == Gtk4WidgetKind::Label && blueprint.widget_class == "gtk::Label(tab)" {
            self.notebook_tabs
                .insert(id, Gtk4NotebookTab::from_config(id, &config));
        }

        let handle = Gtk4OsHandle { id, kind, widget };
        set_widget_title(&handle.widget, config.title.as_deref());
        self.install_key_events(id, &handle.widget);
        if let Some(widget) = handle.widget.as_widget() {
            self.widgets.insert(id, widget);
        }
        self.apply_text_input_hints(id, &handle.widget);
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
                    | Gtk4OsWidget::Dialog(_)
                    | Gtk4OsWidget::Popover(_)
                    | Gtk4OsWidget::Menu(_)
                    | Gtk4OsWidget::Switch(_)
                    | Gtk4OsWidget::DropDown(_)
                    | Gtk4OsWidget::ListBox(_)
                    | Gtk4OsWidget::Notebook(_)
                    | Gtk4OsWidget::Separator(_)
                    | Gtk4OsWidget::Scale(_)
                    | Gtk4OsWidget::ProgressBar(_)
                    | Gtk4OsWidget::Box(_) => {}
                }
            }
            NativeWidgetSetter::SetWindowResizable(value) => {
                if let Gtk4OsWidget::ApplicationWindow(window) = &handle.widget {
                    window.set_resizable(value.unwrap_or(true));
                }
            }
            NativeWidgetSetter::SetValue(value) => match &handle.widget {
                Gtk4OsWidget::Entry(entry) => {
                    self.suppress_events(|| entry.set_text(value.as_deref().unwrap_or("")));
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
                Gtk4OsWidget::TextView(text_view) => {
                    text_view.set_placeholder_text(value.as_deref());
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
                | Gtk4OsWidget::ListBoxRow { .. }
                | Gtk4OsWidget::Notebook(_)
                | Gtk4OsWidget::Separator(_)
                | Gtk4OsWidget::Scale(_)
                | Gtk4OsWidget::ProgressBar(_) => {}
            },
            NativeWidgetSetter::SetSelected(value) => {
                if let Gtk4OsWidget::ListBoxRow { item, .. } = &handle.widget {
                    self.update_drop_down_item_selected(id, item, *value)?;
                }
                if let Gtk4OsWidget::MenuItem(item) = &handle.widget {
                    self.menus.update_item_selected(id, item, *value);
                }
                if let Some(tab) = self.notebook_tabs.get(&id).cloned() {
                    self.update_notebook_tab_selected(id, &tab, *value)?;
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
                    if let Some(gap) = style.gap.as_ref().and_then(StyleLength::points) {
                        box_.set_spacing(points_to_i32(gap));
                    }
                }
            }
            NativeWidgetSetter::SetOrientation(value) => match &handle.widget {
                Gtk4OsWidget::Box(box_) => {
                    if let Some(value) = value {
                        box_.set_orientation(gtk_orientation(*value));
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
                    let range = self.ranges.entry(id).or_default();
                    range.current = *value;
                    spin_button.set_digits(range.spin_button_digits());
                    self.suppress_events(|| spin_button.set_value(range.current()));
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
                    entry.set_max_length(value.map(u32_to_i32).unwrap_or(0));
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
            NativeWidgetSetter::SetAutoFocus(true) => {
                self.request_auto_focus(id, &handle.widget);
            }
            NativeWidgetSetter::SetAutoFocus(false) => {}
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
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
        if let (Gtk4OsWidget::DropDown(_), Gtk4OsWidget::ListBoxRow { item, .. }) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.drop_down_items
                .entry(child)
                .or_insert_with(|| item.clone());
            if let Some(old_parent) = self.drop_down_item_parents.insert(child, parent) {
                if let Some(children) = self.drop_down_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_drop_down(old_parent)?;
            }
            let children = self.drop_down_children.entry(parent).or_default();
            children.retain(|existing| *existing != child);
            let index = index.min(children.len());
            children.insert(index, child);
            self.rebuild_drop_down(parent)?;
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let Gtk4OsWidget::Notebook(_) = &parent_handle.widget {
            self.notebook_tabs
                .entry(child)
                .or_insert_with(|| Gtk4NotebookTab::fallback(child));
            if let Some(old_parent) = self.notebook_tab_parents.insert(child, parent) {
                if let Some(children) = self.notebook_children.get_mut(&old_parent) {
                    children.retain(|existing| *existing != child);
                }
                self.rebuild_notebook(old_parent)?;
            }
            {
                let children = self.notebook_children.entry(parent).or_default();
                children.retain(|existing| *existing != child);
                let index = index.min(children.len());
                children.insert(index, child);
            }
            self.rebuild_notebook(parent)?;
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if self.notebook_tabs.contains_key(&parent) {
            self.update_notebook_tab_panel(parent, Some(child))?;
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let (Gtk4OsWidget::Menu(_), Gtk4OsWidget::MenuItem(item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.menus.insert_item(parent, child, item, index);
            self.focus_pending_auto_focus();
            return Ok(());
        }

        if let (Gtk4OsWidget::MenuItem(item), Gtk4OsWidget::Menu(menu)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            item.item.set_submenu(Some(&menu.model));
            self.focus_pending_auto_focus();
            return Ok(());
        }

        let child_widget = child_handle
            .widget
            .as_widget()
            .ok_or_else(|| GuiError::host("GTK4 native child insertion requires a widget child"))?;
        match &parent_handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => {
                window.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::Box(box_) => {
                self.insert_box_child(parent, box_, child, &child_widget, index);
            }
            Gtk4OsWidget::ScrolledWindow { content, .. } => {
                self.insert_box_child(parent, content, child, &child_widget, index);
            }
            Gtk4OsWidget::Button(button) => {
                button.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::ListBox(list_box) => {
                list_box.insert(&child_widget, index_to_i32(index)?);
            }
            Gtk4OsWidget::ListBoxRow { row, .. } => {
                row.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::Dialog(dialog) => {
                let content_area = dialog.content_area();
                self.insert_box_child(parent, &content_area, child, &child_widget, index);
            }
            Gtk4OsWidget::Popover(popover) => {
                popover.set_child(Some(&child_widget));
            }
            Gtk4OsWidget::DropDown(_)
            | Gtk4OsWidget::Notebook(_)
            | Gtk4OsWidget::Menu(_)
            | Gtk4OsWidget::MenuItem(_)
            | Gtk4OsWidget::Label(_)
            | Gtk4OsWidget::Entry(_)
            | Gtk4OsWidget::SearchEntry(_)
            | Gtk4OsWidget::PasswordEntry(_)
            | Gtk4OsWidget::SpinButton(_)
            | Gtk4OsWidget::TextView(_)
            | Gtk4OsWidget::CheckButton(_)
            | Gtk4OsWidget::Switch(_)
            | Gtk4OsWidget::Separator(_)
            | Gtk4OsWidget::Scale(_)
            | Gtk4OsWidget::ProgressBar(_) => {}
        }
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        if self.root == Some(id) {
            self.root = None;
        }
        if self.pending_auto_focus == Some(id) {
            self.pending_auto_focus = None;
        }
        self.closed_windows.borrow_mut().remove(&id);
        self.widgets.remove(&id);
        for children in self.container_children.values_mut() {
            children.retain(|child| *child != id);
        }
        self.container_children.remove(&id);

        let tabs_with_removed_panel = self
            .notebook_tabs
            .iter()
            .filter_map(|(tab, item)| (item.panel == Some(id)).then_some(*tab))
            .collect::<Vec<_>>();
        for tab in tabs_with_removed_panel {
            self.update_notebook_tab_panel(tab, None)?;
        }

        if let Gtk4OsWidget::Notebook(_) = &handle.widget {
            self.notebooks.remove(&id);
            self.notebook_selected_values.remove(&id);
            self.notebook_values.borrow_mut().remove(&id);
            if let Some(children) = self.notebook_children.remove(&id) {
                for child in children {
                    self.notebook_tab_parents.remove(&child);
                }
            }
        }

        if self.notebook_tabs.contains_key(&id) {
            if let Some(parent) = self.notebook_tab_parents.remove(&id) {
                if let Some(children) = self.notebook_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_notebook(parent)?;
            }
            self.notebook_tabs.remove(&id);
        }

        if let Gtk4OsWidget::DropDown(_) = &handle.widget {
            self.drop_downs.remove(&id);
            self.drop_down_selected_values.remove(&id);
            self.drop_down_values.borrow_mut().remove(&id);
            if let Some(children) = self.drop_down_children.remove(&id) {
                for child in children {
                    self.drop_down_item_parents.remove(&child);
                }
            }
        }
        if let Gtk4OsWidget::ListBoxRow { .. } = &handle.widget {
            self.drop_down_items.remove(&id);
            if let Some(parent) = self.drop_down_item_parents.remove(&id) {
                if let Some(children) = self.drop_down_children.get_mut(&parent) {
                    children.retain(|child| *child != id);
                }
                self.rebuild_drop_down(parent)?;
            }
        }
        if let Gtk4OsWidget::Menu(_) = &handle.widget {
            self.menus.remove_menu(id);
        }
        if let Gtk4OsWidget::MenuItem(_) = &handle.widget {
            self.menus.remove_item(id, &self.application);
        }
        self.ranges.remove(&id);
        self.text_inputs.remove(&id);
        self.text_input_configs.remove(&id);
        self.text_input_max_lengths.borrow_mut().remove(&id);
        match &handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => {
                window.close();
                self.closed_windows.borrow_mut().remove(&id);
            }
            Gtk4OsWidget::Dialog(dialog) => {
                dialog.close();
                self.closed_windows.borrow_mut().remove(&id);
            }
            Gtk4OsWidget::Popover(popover) => popover.popdown(),
            other => {
                if let Some(widget) = other.as_widget() {
                    if widget.parent().is_some() {
                        widget.unparent();
                    }
                }
            }
        }
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.root = Some(id);
        match &handle.widget {
            Gtk4OsWidget::ApplicationWindow(window) => {
                self.closed_windows.borrow_mut().remove(&id);
                window.present();
            }
            Gtk4OsWidget::Dialog(dialog) => {
                self.closed_windows.borrow_mut().remove(&id);
                dialog.present();
            }
            Gtk4OsWidget::Popover(popover) => popover.popup(),
            other => {
                if let Some(widget) = other.as_widget() {
                    widget.set_visible(true);
                }
            }
        }
        self.focus_pending_auto_focus();
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
}

fn set_widget_title(widget: &Gtk4OsWidget, title: Option<&str>) {
    if let Some(widget) = widget.as_widget() {
        widget.set_tooltip_text(title);
    }
}
