use super::helpers::*;
use super::*;

impl NativeWidgetSurface for WinUiNativeSurface {
    type Handle = WinUiOsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let kind = WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
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
                register_press(id, &button, &self.events)?;
                WinUiOsWidget::Button(button)
            }
            WinUiWidgetKind::TextBox => {
                let text_box =
                    map_winui("failed to create WinUI text box", Controls::TextBox::new())?;
                register_text_change(
                    id,
                    &text_box,
                    &self.events,
                    Arc::clone(&self.events_suppressed),
                )?;
                WinUiOsWidget::TextBox(text_box)
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
                register_list_selection(id, &list_box, &self.events)?;
                WinUiOsWidget::ListBox(list_box)
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
                if let Some(label) = config.label.as_deref() {
                    let title = text_content(label)?;
                    map_winui("failed to set WinUI dialog title", dialog.SetTitle(&title))?;
                }
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

        register_focus_events(id, &widget, &self.events)?;
        self.widgets.insert(id, widget.clone());
        Ok(WinUiOsHandle { id, kind, widget })
    }

    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        match setter {
            NativeWidgetSetter::SetLabel(value) => {
                set_label(&handle.widget, value.as_deref())?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_label(id, &item, value.clone().unwrap_or_default())?;
                }
                if let Some(item) = self.tab_items.get(&id).cloned() {
                    self.update_tab_item_label(id, &item, value.clone().unwrap_or_default())?;
                }
            }
            NativeWidgetSetter::SetValue(value) => {
                set_value(self, id, &handle.widget, value.as_deref())?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_value(id, &item, value.clone().unwrap_or_default())?;
                }
                if let (Some(item), Some(value)) = (self.tab_items.get(&id).cloned(), value) {
                    self.update_tab_item_value(id, &item, value.clone())?;
                }
            }
            NativeWidgetSetter::SetPlaceholder(value) => {
                set_placeholder(&handle.widget, value.as_deref())?;
            }
            NativeWidgetSetter::SetEnabled(enabled) => {
                if let Some(control) = handle.widget.control() {
                    map_winui(
                        "failed to set WinUI control enabled state",
                        control.SetIsEnabled(*enabled),
                    )?;
                }
            }
            NativeWidgetSetter::SetReadOnly(read_only) => {
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    map_winui(
                        "failed to set WinUI text box read-only state",
                        text_box.SetIsReadOnly(*read_only),
                    )?;
                }
            }
            NativeWidgetSetter::SetVisible(visible) => {
                if let WinUiOsWidget::ToolTip(tool_tip) = &handle.widget {
                    map_winui(
                        "failed to set WinUI tooltip popover open state",
                        tool_tip.SetIsOpen(*visible),
                    )?;
                }
                if let Some(element) = handle.widget.ui_element() {
                    let visibility = if *visible {
                        Visibility::Visible
                    } else {
                        Visibility::Collapsed
                    };
                    map_winui(
                        "failed to set WinUI element visibility",
                        element.SetVisibility(visibility),
                    )?;
                }
            }
            NativeWidgetSetter::SetSelected(selected) => {
                set_selected(&handle.widget, *selected)?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_selected(id, &item, *selected)?;
                }
                if let Some(item) = self.tab_items.get(&id).cloned() {
                    self.update_tab_item_selected(id, &item, *selected)?;
                }
            }
            NativeWidgetSetter::SetChecked(checked) => {
                if let Some(checked) = checked {
                    set_checked(self, &handle.widget, *checked)?;
                }
            }
            NativeWidgetSetter::SetOrientation(orientation) => {
                if let WinUiOsWidget::Separator(separator) = &handle.widget {
                    set_winui_separator_orientation(separator, *orientation)?;
                }
                set_orientation(&handle.widget, *orientation)?;
            }
            NativeWidgetSetter::SetMinimum(minimum) => {
                self.ranges.entry(id).or_default().min = *minimum;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetMaximum(maximum) => {
                self.ranges.entry(id).or_default().max = *maximum;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetCurrent(current) => {
                self.ranges.entry(id).or_default().current = *current;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetStep(step) => {
                self.ranges.entry(id).or_default().step = *step;
                self.apply_range(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                apply_portable_style(&handle.widget, style)?;
            }
            NativeWidgetSetter::SetMaxLength(max_length) => {
                if let (WinUiOsWidget::TextBox(text_box), Some(max_length)) =
                    (&handle.widget, max_length)
                {
                    map_winui(
                        "failed to set WinUI text box max length",
                        text_box.SetMaxLength(*max_length as i32),
                    )?;
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
        match &parent_handle.widget {
            WinUiOsWidget::Window(window) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI window child must be a UIElement-backed widget")
                })?;
                map_winui(
                    "failed to set WinUI window content",
                    window.SetContent(&child_element),
                )?;
            }
            WinUiOsWidget::StackPanel(panel) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI stack panel child must be a UIElement-backed widget")
                })?;
                self.insert_panel_child(
                    parent,
                    map_winui(
                        "failed to read WinUI stack panel children",
                        panel.Children(),
                    )?,
                    child,
                    child_element,
                    index,
                )?;
            }
            WinUiOsWidget::Grid(grid) => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI grid child must be a UIElement-backed widget")
                })?;
                self.insert_panel_child(
                    parent,
                    map_winui("failed to read WinUI grid children", grid.Children())?,
                    child,
                    child_element,
                    index,
                )?;
            }
            WinUiOsWidget::ContentDialog(dialog) => {
                let child = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI content dialog child must be an inspectable widget")
                })?;
                map_winui(
                    "failed to set WinUI content dialog content",
                    dialog.SetContent(&child),
                )?;
            }
            WinUiOsWidget::ToolTip(tool_tip) => {
                let child = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI tooltip popover child must be an inspectable widget")
                })?;
                map_winui(
                    "failed to set WinUI tooltip popover content",
                    tool_tip.SetContent(&child),
                )?;
            }
            WinUiOsWidget::ComboBox(_) => {
                self.combo_children
                    .entry(parent)
                    .or_default()
                    .retain(|existing| *existing != child);
                let children = self.combo_children.entry(parent).or_default();
                let index = index.min(children.len());
                children.insert(index, child);
                self.combo_item_parents.insert(child, parent);
                self.combo_items.entry(child).or_insert_with(|| {
                    let label = child_handle.id.get().to_string();
                    WinUiComboBoxItem {
                        label: label.clone(),
                        value: label,
                        selected: false,
                    }
                });
                self.rebuild_combo_box(parent)?;
            }
            WinUiOsWidget::ListBox(list_box) => {
                let child_object = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI list child must be an inspectable native widget")
                })?;
                Self::insert_items_child(
                    &mut self.list_children,
                    parent,
                    map_winui("failed to read WinUI list box items", list_box.Items())?,
                    child,
                    child_object,
                    index,
                )?;
            }
            WinUiOsWidget::TabView(tab_view) => {
                let child_object = child_handle.widget.inspectable().ok_or_else(|| {
                    GuiError::host("WinUI tab view child must be an inspectable native widget")
                })?;
                let collection =
                    map_winui("failed to read WinUI tab view items", tab_view.TabItems())?;
                let children = self.tab_children.entry(parent).or_default();
                if let Some(previous_index) =
                    children.iter().position(|existing| *existing == child)
                {
                    map_winui(
                        "failed to move existing WinUI tab view item",
                        collection.RemoveAt(to_u32(previous_index)?),
                    )?;
                    children.remove(previous_index);
                }
                let index = index.min(children.len());
                map_winui(
                    "failed to insert WinUI tab view item",
                    collection.InsertAt(to_u32(index)?, &child_object),
                )?;
                children.insert(index, child);
                self.tab_items
                    .entry(child)
                    .or_insert_with(|| WinUiTabItem::fallback(child));
                self.rebuild_tab_view(parent)?;
            }
            WinUiOsWidget::Button(button) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI button content",
                        button.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI checkbox content",
                        check_box.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::RadioButton(radio) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI radio button content",
                        radio.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::ComboBoxItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI combo box item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::ListBoxItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI list box item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::TabViewItem(item) => {
                if let Some(content) = child_handle.widget.inspectable() {
                    map_winui(
                        "failed to set WinUI tab view item content",
                        item.SetContent(&content),
                    )?;
                }
            }
            WinUiOsWidget::TextBlock(_)
            | WinUiOsWidget::Separator(_)
            | WinUiOsWidget::TextBox(_)
            | WinUiOsWidget::Slider(_)
            | WinUiOsWidget::ProgressBar(_) => {}
        }
        Ok(())
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        self.detach_child(id)?;
        match &handle.widget {
            WinUiOsWidget::Window(window) => {
                map_winui("failed to close WinUI window", window.Close())?;
            }
            WinUiOsWidget::ContentDialog(dialog) => {
                map_winui("failed to hide WinUI content dialog", dialog.Hide())?;
            }
            WinUiOsWidget::ToolTip(tool_tip) => {
                map_winui(
                    "failed to close WinUI tooltip popover",
                    tool_tip.SetIsOpen(false),
                )?;
            }
            _ => {}
        }
        self.widgets.remove(&id);
        self.combo_boxes.remove(&id);
        self.combo_items.remove(&id);
        self.combo_children.remove(&id);
        self.combo_selected_values.remove(&id);
        if let Ok(mut combo_values) = self.combo_values.lock() {
            combo_values.remove(&id);
        }
        self.container_children.remove(&id);
        self.list_children.remove(&id);
        self.tab_children.remove(&id);
        self.tab_items.remove(&id);
        self.tab_selected_values.remove(&id);
        if let Ok(mut tab_values) = self.tab_values.lock() {
            tab_values.remove(&id);
        }
        self.ranges.remove(&id);
        if self.root == Some(id) {
            self.root = None;
        }
        Ok(())
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        self.root = Some(id);
        if let WinUiOsWidget::Window(window) = &handle.widget {
            map_winui("failed to activate WinUI window", window.Activate())?;
        }
        if let WinUiOsWidget::ToolTip(tool_tip) = &handle.widget {
            map_winui(
                "failed to open WinUI tooltip popover",
                tool_tip.SetIsOpen(true),
            )?;
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.events
            .lock()
            .map(|mut events| std::mem::take(&mut *events))
            .unwrap_or_default()
    }
}
