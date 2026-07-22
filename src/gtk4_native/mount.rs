use super::*;

impl Gtk4NativeSurface {
    pub(super) fn create_native_widget_impl(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Gtk4OsHandle> {
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

                    Gtk4OsWidget::PasswordEntry(entry)
                } else {
                    let entry = gtk::Entry::new();
                    let max_length = config.max_length;
                    self.suppress_events(|| {
                        entry.set_text(&truncate_to_max_length(
                            config.value.as_deref().unwrap_or(""),
                            max_length,
                        ));
                    });
                    entry.set_max_length(max_length.map(u32_to_i32).unwrap_or(0));
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
                    self.apply_entry_width_hint(id, &entry);

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

                Gtk4OsWidget::SpinButton(spin_button)
            }
            Gtk4WidgetKind::TextView => {
                let text_view = gtk::TextView::new();
                text_view.set_wrap_mode(gtk::WrapMode::WordChar);
                text_view.set_editable(!config.read_only);
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
            Gtk4WidgetKind::ListBox => {
                let list_box = gtk::ListBox::new();
                list_box.set_selection_mode(if config.multiple {
                    gtk::SelectionMode::Multiple
                } else {
                    gtk::SelectionMode::Single
                });
                let events = self.events.clone();
                let events_suppressed = self.events_suppressed.clone();
                let list_values = self.list_values.clone();
                list_box.connect_selected_rows_changed(move |list_box| {
                    let values = list_values.borrow().get(&id).cloned().unwrap_or_default();
                    let mut indices = list_box
                        .selected_rows()
                        .into_iter()
                        .filter_map(|row| usize::try_from(row.index()).ok())
                        .collect::<Vec<_>>();
                    indices.sort_unstable();
                    let selected_values = indices
                        .into_iter()
                        .filter_map(|index| values.get(index).cloned())
                        .collect::<Vec<_>>();
                    let value = serde_json::to_string(&selected_values)
                        .unwrap_or_else(|_| "[]".to_string());
                    push_event(
                        &events,
                        &events_suppressed,
                        NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
                    );
                });
                Gtk4OsWidget::ListBox(list_box)
            }
            Gtk4WidgetKind::Dialog => {
                let dialog = gtk::Dialog::builder()
                    .application(&self.application)
                    .title(config.label.as_deref().unwrap_or(""))
                    .default_width(config_dimension(
                        config
                            .portable_style
                            .width
                            .as_ref()
                            .and_then(StyleLength::points),
                        420,
                    ))
                    .default_height(config_dimension(
                        config
                            .portable_style
                            .height
                            .as_ref()
                            .and_then(StyleLength::points),
                        280,
                    ))
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
                self.dialog_visible.insert(id, config.visible);
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
                row.set_focusable(true);
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
        };

        if kind == Gtk4WidgetKind::Label && blueprint.widget_class == "gtk::Label(tab)" {
            self.notebook_tabs
                .insert(id, Gtk4NotebookTab::from_config(id, &config));
        }

        let handle = Gtk4OsHandle { id, kind, widget };
        set_widget_title(&handle.widget, config.title.as_deref());
        let interaction_registration = Rc::new(std::cell::Cell::new(
            interaction::Gtk4InteractionRegistration::new(&handle.widget, blueprint),
        ));
        self.interaction_nodes
            .borrow_mut()
            .insert(id, Rc::clone(&interaction_registration));
        interaction::register_interaction_events(
            id,
            &handle.widget,
            &self.events,
            &self.events_suppressed,
            &self.activation_contexts,
            &self.interaction_nodes,
            &self.keyboard_presses,
            interaction_registration,
        );
        if let Some(widget) = handle.widget.as_widget() {
            self.widgets.insert(id, widget);
        }
        self.apply_text_input_hints(id, &handle.widget);
        Ok(handle)
    }
}
