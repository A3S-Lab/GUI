use super::helpers::*;
use super::*;

impl WinUiNativeSurface {
    pub(super) fn create_native_widget_impl(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<WinUiOsHandle> {
        let kind = WinUiWidgetKind::from_widget_kind(blueprint.widget_kind);
        let config = blueprint.config();
        let widget = match kind {
            WinUiWidgetKind::Window => {
                let window = map_winui("failed to create WinUI window", xaml::Window::new())?;
                if let Some(label) = config.label.as_deref() {
                    map_winui(
                        "failed to set WinUI window title",
                        window.SetTitle(&hstr(label)),
                    )?;
                }
                install_winui_window_close_event(&window, id, self.events.clone())?;
                WinUiOsWidget::Window(window)
            }
            WinUiWidgetKind::StackPanel
            | WinUiWidgetKind::RadioButtons
            | WinUiWidgetKind::MenuPanel
            | WinUiWidgetKind::CommandBar => {
                let panel = map_winui(
                    "failed to create WinUI stack panel",
                    Controls::StackPanel::new(),
                )?;
                if let Some(orientation) =
                    winui_menu::stack_panel_orientation(kind, config.orientation)
                {
                    let orientation = match orientation {
                        A3sOrientation::Horizontal => Controls::Orientation::Horizontal,
                        A3sOrientation::Vertical => Controls::Orientation::Vertical,
                    };
                    map_winui(
                        "failed to set WinUI stack panel orientation",
                        panel.SetOrientation(orientation),
                    )?;
                }
                apply_stack_panel_spacing(
                    &panel,
                    &config.portable_style,
                    "failed to set WinUI stack panel spacing",
                )?;
                WinUiOsWidget::StackPanel(panel)
            }
            WinUiWidgetKind::TextBlock => {
                let text = map_winui(
                    "failed to create WinUI text block",
                    Controls::TextBlock::new(),
                )?;
                WinUiOsWidget::TextBlock(text)
            }
            WinUiWidgetKind::Separator => {
                let separator = create_winui_separator(config.orientation)?;
                WinUiOsWidget::Separator(separator)
            }
            WinUiWidgetKind::Button | WinUiWidgetKind::MenuItemButton => {
                let button = map_winui("failed to create WinUI button", Controls::Button::new())?;
                WinUiOsWidget::Button(button)
            }
            WinUiWidgetKind::TextBox => {
                if config_is_password(&config) {
                    let password_box = map_winui(
                        "failed to create WinUI password box",
                        Controls::PasswordBox::new(),
                    )?;
                    self.text_inputs
                        .insert(id, WinUiTextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    if let Ok(mut max_lengths) = self.text_input_max_lengths.lock() {
                        max_lengths.insert(id, config.max_length);
                    }
                    if let Ok(mut read_only) = self.text_input_read_only.lock() {
                        read_only.insert(id, config.read_only);
                    }
                    if let Ok(mut values) = self.text_input_values.lock() {
                        values.insert(
                            id,
                            winui_truncate_to_max_length(
                                config.value.as_deref().unwrap_or_default(),
                                config.max_length,
                            ),
                        );
                    }
                    self.apply_password_box_size_hint(id, &password_box)?;
                    register_password_change(
                        id,
                        &password_box,
                        &self.events,
                        Arc::clone(&self.events_suppressed),
                        Arc::clone(&self.text_input_max_lengths),
                        Arc::clone(&self.text_input_read_only),
                        Arc::clone(&self.text_input_values),
                    )?;
                    WinUiOsWidget::PasswordBox(password_box)
                } else {
                    let text_box =
                        map_winui("failed to create WinUI text box", Controls::TextBox::new())?;
                    if config_is_textarea(&config) {
                        map_winui(
                            "failed to enable WinUI text box return input",
                            text_box.SetAcceptsReturn(true),
                        )?;
                        map_winui(
                            "failed to enable WinUI text box wrapping",
                            text_box.SetTextWrapping(xaml::TextWrapping::Wrap),
                        )?;
                    }
                    self.text_inputs
                        .insert(id, WinUiTextInputSizing::from_config(&config));
                    self.text_input_configs.insert(id, config.clone());
                    if let Ok(mut max_lengths) = self.text_input_max_lengths.lock() {
                        max_lengths.insert(id, config.max_length);
                    }
                    if let Ok(mut read_only) = self.text_input_read_only.lock() {
                        read_only.insert(id, config.read_only);
                    }
                    if let Ok(mut values) = self.text_input_values.lock() {
                        values.insert(
                            id,
                            winui_truncate_to_max_length(
                                config.value.as_deref().unwrap_or_default(),
                                config.max_length,
                            ),
                        );
                    }
                    self.apply_text_box_size_hint(id, &text_box)?;
                    register_text_change(
                        id,
                        &text_box,
                        &self.events,
                        Arc::clone(&self.events_suppressed),
                        Arc::clone(&self.text_input_max_lengths),
                        Arc::clone(&self.text_input_read_only),
                        Arc::clone(&self.text_input_values),
                    )?;
                    WinUiOsWidget::TextBox(text_box)
                }
            }
            WinUiWidgetKind::CheckBox => {
                let check_box =
                    map_winui("failed to create WinUI checkbox", Controls::CheckBox::new())?;
                register_toggle(
                    id,
                    &check_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::CheckBox(check_box)
            }
            WinUiWidgetKind::ToggleSwitch => {
                let check_box = map_winui(
                    "failed to create WinUI switch fallback checkbox",
                    Controls::CheckBox::new(),
                )?;
                register_toggle(
                    id,
                    &check_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::ToggleSwitch(check_box)
            }
            WinUiWidgetKind::RadioButton => {
                let radio = map_winui(
                    "failed to create WinUI radio button",
                    Controls::RadioButton::new(),
                )?;
                register_radio_toggle(
                    id,
                    &radio,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::RadioButton(radio)
            }
            WinUiWidgetKind::ComboBox => {
                let combo_box = map_winui(
                    "failed to create WinUI combo box",
                    Controls::ComboBox::new(),
                )?;
                register_combo_selection(
                    id,
                    &combo_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.combo_values),
                )?;
                self.combo_boxes.insert(id, combo_box.clone());
                WinUiOsWidget::ComboBox(combo_box)
            }
            WinUiWidgetKind::ListView => {
                let list_box =
                    map_winui("failed to create WinUI list box", Controls::ListBox::new())?;
                map_winui(
                    "failed to set WinUI list box selection mode",
                    list_box.SetSelectionMode(if config.multiple {
                        Controls::SelectionMode::Multiple
                    } else {
                        Controls::SelectionMode::Single
                    }),
                )?;
                register_list_selection(
                    id,
                    &list_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.list_values),
                    Arc::clone(&self.list_nodes),
                    Arc::clone(&self.activation_contexts),
                )?;
                WinUiOsWidget::ListBox(list_box)
            }
            WinUiWidgetKind::ScrollViewer => {
                let viewer = map_winui(
                    "failed to create WinUI scroll viewer",
                    Controls::ScrollViewer::new(),
                )?;
                map_winui(
                    "failed to set WinUI scroll viewer horizontal policy",
                    viewer.SetHorizontalScrollBarVisibility(winui_scroll_visibility(
                        config.portable_style.overflow_x,
                    )),
                )?;
                map_winui(
                    "failed to set WinUI scroll viewer vertical policy",
                    viewer.SetVerticalScrollBarVisibility(winui_scroll_visibility(
                        config.portable_style.overflow_y,
                    )),
                )?;
                let content = map_winui(
                    "failed to create WinUI scroll viewer content panel",
                    Controls::StackPanel::new(),
                )?;
                if let Some(orientation) =
                    winui_menu::stack_panel_orientation(kind, config.orientation)
                {
                    let orientation = match orientation {
                        A3sOrientation::Horizontal => Controls::Orientation::Horizontal,
                        A3sOrientation::Vertical => Controls::Orientation::Vertical,
                    };
                    map_winui(
                        "failed to set WinUI scroll viewer content orientation",
                        content.SetOrientation(orientation),
                    )?;
                }
                apply_stack_panel_spacing(
                    &content,
                    &config.portable_style,
                    "failed to set WinUI scroll viewer content spacing",
                )?;
                let content_object = map_winui(
                    "failed to inspect WinUI scroll viewer content panel",
                    content.cast::<windows_core::IInspectable>(),
                )?;
                map_winui(
                    "failed to set WinUI scroll viewer content",
                    viewer.SetContent(&content_object),
                )?;
                WinUiOsWidget::ScrollViewer { viewer, content }
            }
            WinUiWidgetKind::TabView => {
                let tab_view =
                    map_winui("failed to create WinUI tab view", Controls::TabView::new())?;
                register_tab_selection(
                    id,
                    &tab_view,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.tab_values),
                )?;
                WinUiOsWidget::TabView(tab_view)
            }
            WinUiWidgetKind::TabViewItem => {
                let item = map_winui(
                    "failed to create WinUI tab view item",
                    Controls::TabViewItem::new(),
                )?;
                self.tab_items
                    .insert(id, WinUiTabItem::from_config(id, &config));
                WinUiOsWidget::TabViewItem(item)
            }
            WinUiWidgetKind::ListViewItem => {
                let item = map_winui(
                    "failed to create WinUI list box item",
                    Controls::ListBoxItem::new(),
                )?;
                self.combo_items
                    .insert(id, WinUiComboBoxItem::from_config(&config));
                WinUiOsWidget::ListBoxItem(item)
            }
            WinUiWidgetKind::ContentDialog => {
                let dialog = map_winui(
                    "failed to create WinUI content dialog",
                    Controls::ContentDialog::new(),
                )?;
                register_content_dialog_close(
                    id,
                    &dialog,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                    Arc::clone(&self.open_dialogs),
                )?;
                if let Some(label) = config.label.as_deref() {
                    let title = text_content(label)?;
                    map_winui("failed to set WinUI dialog title", dialog.SetTitle(&title))?;
                }
                self.dialog_visible.insert(id, config.visible);
                WinUiOsWidget::ContentDialog(dialog)
            }
            WinUiWidgetKind::ToolTip => {
                let tool_tip = map_winui(
                    "failed to create WinUI tooltip popover",
                    Controls::ToolTip::new(),
                )?;
                if let Some(label) = config.label.as_deref() {
                    let content = text_content(label)?;
                    map_winui(
                        "failed to set WinUI tooltip popover content",
                        tool_tip.SetContent(&content),
                    )?;
                }
                WinUiOsWidget::ToolTip(tool_tip)
            }
            WinUiWidgetKind::SelectorItem => {
                let item = map_winui(
                    "failed to create WinUI combo box item",
                    Controls::ComboBoxItem::new(),
                )?;
                self.combo_items
                    .insert(id, WinUiComboBoxItem::from_config(&config));
                WinUiOsWidget::ComboBoxItem(item)
            }
            WinUiWidgetKind::Grid => {
                let grid = map_winui("failed to create WinUI grid", Controls::Grid::new())?;
                WinUiOsWidget::Grid(grid)
            }
            WinUiWidgetKind::Slider => {
                let slider = map_winui("failed to create WinUI slider", Controls::Slider::new())?;
                register_range_change(
                    id,
                    &slider,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                self.ranges
                    .insert(id, WinUiRangeState::from_config(&config));
                WinUiOsWidget::Slider(slider)
            }
            WinUiWidgetKind::ProgressBar => {
                let progress = map_winui(
                    "failed to create WinUI progress bar",
                    Controls::ProgressBar::new(),
                )?;
                self.ranges
                    .insert(id, WinUiRangeState::from_config(&config));
                WinUiOsWidget::ProgressBar(progress)
            }
        };

        if config.title.is_some() {
            set_title(&widget, config.title.as_deref())?;
        }
        self.apply_text_input_hints(id, &widget)?;
        register_focus_events(id, &widget, &self.events, Arc::clone(&self.focused_node))?;
        let interaction_registration = Arc::new(Mutex::new(
            interaction::WinUiInteractionRegistration::new(&widget, blueprint),
        ));
        interaction::register_interaction_events(
            id,
            &widget,
            &self.events,
            Arc::clone(&self.activation_contexts),
            Arc::clone(&self.pending_activation_cleanup),
            Arc::clone(&self.forced_pointer_cancellations),
            Arc::clone(&interaction_registration),
            Arc::clone(&self.keyboard_presses),
        )?;
        self.interaction_nodes.insert(id, interaction_registration);
        self.widgets.insert(id, widget.clone());
        Ok(WinUiOsHandle { id, kind, widget })
    }
}
