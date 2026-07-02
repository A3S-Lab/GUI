use super::*;

pub(super) fn set_label(widget: &WinUiOsWidget, value: Option<&str>) -> GuiResult<()> {
    let value = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::Window(window) => {
            map_winui(
                "failed to set WinUI window title",
                window.SetTitle(&hstr(value)),
            )?;
        }
        WinUiOsWidget::ContentDialog(dialog) => {
            let title = text_content(value)?;
            map_winui("failed to set WinUI dialog title", dialog.SetTitle(&title))?;
        }
        WinUiOsWidget::ToolTip(tool_tip) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI tooltip popover content",
                tool_tip.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TextBlock(text) => {
            map_winui(
                "failed to set WinUI text block text",
                text.SetText(&hstr(value)),
            )?;
        }
        WinUiOsWidget::Separator(_) => {}
        WinUiOsWidget::Button(button) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI button content",
                button.SetContent(&content),
            )?;
        }
        WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI checkbox content",
                check_box.SetContent(&content),
            )?;
        }
        WinUiOsWidget::RadioButton(radio) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI radio button content",
                radio.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TextBox(text_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI text box header",
                text_box.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::PasswordBox(password_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI password box header",
                password_box.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI combo box header",
                combo_box.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::ComboBoxItem(item) => set_combo_box_item_content(item, value)?,
        WinUiOsWidget::ListBoxItem(item) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI list box item content",
                item.SetContent(&content),
            )?;
        }
        WinUiOsWidget::TabViewItem(item) => {
            let content = text_content(value)?;
            map_winui(
                "failed to set WinUI tab view item header",
                item.SetHeader(&content),
            )?;
        }
        WinUiOsWidget::StackPanel(_)
        | WinUiOsWidget::ListBox(_)
        | WinUiOsWidget::TabView(_)
        | WinUiOsWidget::Grid(_)
        | WinUiOsWidget::Slider(_)
        | WinUiOsWidget::ProgressBar(_) => {}
    }
    Ok(())
}

pub(super) fn set_title(widget: &WinUiOsWidget, value: Option<&str>) -> GuiResult<()> {
    let Some(element) = widget.framework_element() else {
        return Ok(());
    };
    let content = text_content(value.unwrap_or_default())?;
    map_winui(
        "failed to set WinUI element tooltip title",
        Controls::ToolTipService::SetToolTip(&element, &content),
    )
}

pub(super) fn set_value(
    surface: &mut WinUiNativeSurface,
    id: HostNodeId,
    widget: &WinUiOsWidget,
    value: Option<&str>,
) -> GuiResult<()> {
    let value_text = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::TextBlock(text) => {
            if let Some(value) = value {
                map_winui(
                    "failed to set WinUI text block value",
                    text.SetText(&hstr(value)),
                )?;
            }
        }
        WinUiOsWidget::Separator(_) => {}
        WinUiOsWidget::TextBox(text_box) => {
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI text box value",
                    text_box.SetText(&hstr(value_text)),
                )
            })?;
        }
        WinUiOsWidget::PasswordBox(password_box) => {
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI password box value",
                    password_box.SetPassword(&hstr(value_text)),
                )
            })?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            surface.set_combo_value(id, combo_box, Some(value_text))?;
        }
        WinUiOsWidget::TabView(tab_view) => {
            surface.set_tab_value(id, tab_view, value)?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_placeholder(widget: &WinUiOsWidget, value: Option<&str>) -> GuiResult<()> {
    let value = value.unwrap_or_default();
    match widget {
        WinUiOsWidget::TextBox(text_box) => {
            map_winui(
                "failed to set WinUI text box placeholder",
                text_box.SetPlaceholderText(&hstr(value)),
            )?;
        }
        WinUiOsWidget::PasswordBox(password_box) => {
            map_winui(
                "failed to set WinUI password box placeholder",
                password_box.SetPlaceholderText(&hstr(value)),
            )?;
        }
        WinUiOsWidget::ComboBox(combo_box) => {
            map_winui(
                "failed to set WinUI combo box placeholder",
                combo_box.SetPlaceholderText(&hstr(value)),
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_checked(
    surface: &WinUiNativeSurface,
    widget: &WinUiOsWidget,
    checked: bool,
) -> GuiResult<()> {
    match widget {
        WinUiOsWidget::CheckBox(check_box) | WinUiOsWidget::ToggleSwitch(check_box) => {
            let value = bool_reference(checked)?;
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI checkbox checked state",
                    check_box.SetIsChecked(&value),
                )
            })?;
        }
        WinUiOsWidget::RadioButton(radio) => {
            let value = bool_reference(checked)?;
            surface.suppress_events(|| {
                map_winui(
                    "failed to set WinUI radio button checked state",
                    radio.SetIsChecked(&value),
                )
            })?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_selected(widget: &WinUiOsWidget, selected: bool) -> GuiResult<()> {
    match widget {
        WinUiOsWidget::ComboBoxItem(item) => {
            map_winui(
                "failed to set WinUI combo box item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        WinUiOsWidget::ListBoxItem(item) => {
            map_winui(
                "failed to set WinUI list box item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        WinUiOsWidget::TabViewItem(item) => {
            map_winui(
                "failed to set WinUI tab view item selected state",
                item.SetIsSelected(selected),
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_orientation(
    widget: &WinUiOsWidget,
    orientation: Option<A3sOrientation>,
) -> GuiResult<()> {
    let Some(orientation) = orientation else {
        return Ok(());
    };
    let orientation = match orientation {
        A3sOrientation::Horizontal => Controls::Orientation::Horizontal,
        A3sOrientation::Vertical => Controls::Orientation::Vertical,
    };
    match widget {
        WinUiOsWidget::StackPanel(panel) => {
            map_winui(
                "failed to set WinUI stack panel orientation",
                panel.SetOrientation(orientation),
            )?;
        }
        WinUiOsWidget::Slider(slider) => {
            map_winui(
                "failed to set WinUI slider orientation",
                slider.SetOrientation(orientation),
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn create_winui_separator(
    orientation: Option<A3sOrientation>,
) -> GuiResult<xaml::FrameworkElement> {
    let xaml = r##"<Border xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Background="#767676" IsHitTestVisible="False" />"##;
    let object = map_winui(
        "failed to load WinUI separator XAML",
        Markup::XamlReader::Load(&hstr(xaml)),
    )?;
    let separator = map_winui(
        "failed to cast WinUI separator to framework element",
        object.cast::<xaml::FrameworkElement>(),
    )?;
    set_winui_separator_orientation(&separator, orientation)?;
    Ok(separator)
}

pub(super) fn set_winui_separator_orientation(
    separator: &xaml::FrameworkElement,
    orientation: Option<A3sOrientation>,
) -> GuiResult<()> {
    match orientation.unwrap_or(A3sOrientation::Horizontal) {
        A3sOrientation::Horizontal => {
            map_winui(
                "failed to reset WinUI separator width",
                separator.SetWidth(f64::NAN),
            )?;
            map_winui(
                "failed to set WinUI separator height",
                separator.SetHeight(1.0),
            )?;
            map_winui(
                "failed to set WinUI separator minimum width",
                separator.SetMinWidth(160.0),
            )?;
            map_winui(
                "failed to reset WinUI separator minimum height",
                separator.SetMinHeight(0.0),
            )?;
        }
        A3sOrientation::Vertical => {
            map_winui(
                "failed to set WinUI separator width",
                separator.SetWidth(1.0),
            )?;
            map_winui(
                "failed to reset WinUI separator height",
                separator.SetHeight(f64::NAN),
            )?;
            map_winui(
                "failed to reset WinUI separator minimum width",
                separator.SetMinWidth(0.0),
            )?;
            map_winui(
                "failed to set WinUI separator minimum height",
                separator.SetMinHeight(160.0),
            )?;
        }
    }
    Ok(())
}

pub(super) fn apply_portable_style(widget: &WinUiOsWidget, style: &PortableStyle) -> GuiResult<()> {
    let Some(element) = widget.framework_element() else {
        return Ok(());
    };
    if let Some(value) = style.width.as_ref().and_then(StyleLength::points) {
        map_winui("failed to set WinUI element width", element.SetWidth(value))?;
    }
    if let Some(value) = style.height.as_ref().and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element height",
            element.SetHeight(value),
        )?;
    }
    if let Some(value) = style.min_width.as_ref().and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element minimum width",
            element.SetMinWidth(value),
        )?;
    }
    if let Some(value) = style.min_height.as_ref().and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element minimum height",
            element.SetMinHeight(value),
        )?;
    }
    if let Some(value) = style.max_width.as_ref().and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element maximum width",
            element.SetMaxWidth(value),
        )?;
    }
    if let Some(value) = style.max_height.as_ref().and_then(StyleLength::points) {
        map_winui(
            "failed to set WinUI element maximum height",
            element.SetMaxHeight(value),
        )?;
    }
    Ok(())
}

pub(super) fn register_press(
    id: HostNodeId,
    button: &Controls::Button,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let handler = RoutedEventHandler::new(move |_, _| {
        push_event(&events, NativeEvent::new(id, NativeEventKind::Press));
        Ok(())
    });
    map_winui(
        "failed to register WinUI button press handler",
        button.Click(&handler),
    )?;
    Ok(())
}

pub(super) fn register_text_change(
    id: HostNodeId,
    text_box: &Controls::TextBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_text_box = text_box.clone();
    let handler = Controls::TextChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let value = event_text_box.Text()?.to_string();
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Change).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI text change handler",
        text_box.TextChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_password_change(
    id: HostNodeId,
    password_box: &Controls::PasswordBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_password_box = password_box.clone();
    let handler = RoutedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let value = event_password_box.Password()?.to_string();
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Change).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI password change handler",
        password_box.PasswordChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_toggle(
    id: HostNodeId,
    check_box: &Controls::CheckBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let checked_events = Arc::clone(events);
    let checked_suppressed = Arc::clone(&suppressed);
    let checked = RoutedEventHandler::new(move |_, _| {
        if !checked_suppressed.load(Ordering::SeqCst) {
            push_event(
                &checked_events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("true"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI checked handler",
        check_box.Checked(&checked),
    )?;

    let unchecked_events = Arc::clone(events);
    let unchecked = RoutedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            push_event(
                &unchecked_events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("false"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI unchecked handler",
        check_box.Unchecked(&unchecked),
    )?;
    Ok(())
}

pub(super) fn register_radio_toggle(
    id: HostNodeId,
    radio: &Controls::RadioButton,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let handler = RoutedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Toggle).value("true"),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI radio handler",
        radio.Checked(&handler),
    )?;
    Ok(())
}

pub(super) fn register_combo_selection(
    id: HostNodeId,
    combo_box: &Controls::ComboBox,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
    values_by_combo: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_combo_box = combo_box.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let index = event_combo_box.SelectedIndex()?;
            let value = if index < 0 {
                String::new()
            } else {
                values_by_combo
                    .lock()
                    .ok()
                    .and_then(|values| values.get(&id).cloned())
                    .and_then(|values| values.get(index as usize).cloned())
                    .unwrap_or_default()
            };
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI combo box selection handler",
        combo_box.SelectionChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_list_selection(
    id: HostNodeId,
    list_box: &Controls::ListBox,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_list_box = list_box.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        let value = event_list_box
            .SelectedIndex()
            .map(|index| index.to_string())
            .unwrap_or_default();
        push_event(
            &events,
            NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
        );
        Ok(())
    });
    map_winui(
        "failed to register WinUI list selection handler",
        list_box.SelectionChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_tab_selection(
    id: HostNodeId,
    tab_view: &Controls::TabView,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
    values_by_tab_view: Arc<Mutex<BTreeMap<HostNodeId, Vec<String>>>>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_tab_view = tab_view.clone();
    let handler = Controls::SelectionChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let index = event_tab_view.SelectedIndex()?;
            let value = if index < 0 {
                String::new()
            } else {
                values_by_tab_view
                    .lock()
                    .ok()
                    .and_then(|values| values.get(&id).cloned())
                    .and_then(|values| values.get(index as usize).cloned())
                    .unwrap_or_else(|| index.to_string())
            };
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::SelectionChange).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI tab view selection handler",
        tab_view.SelectionChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_range_change(
    id: HostNodeId,
    slider: &Controls::Slider,
    events: &WinUiEventQueue,
    suppressed: Arc<AtomicBool>,
) -> GuiResult<()> {
    let events = Arc::clone(events);
    let event_slider = slider.clone();
    let handler = Primitives::RangeBaseValueChangedEventHandler::new(move |_, _| {
        if !suppressed.load(Ordering::SeqCst) {
            let value = event_slider.Value()?.to_string();
            push_event(
                &events,
                NativeEvent::new(id, NativeEventKind::Change).value(value),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI slider value handler",
        slider.ValueChanged(&handler),
    )?;
    Ok(())
}

pub(super) fn register_focus_events(
    id: HostNodeId,
    widget: &WinUiOsWidget,
    events: &WinUiEventQueue,
) -> GuiResult<()> {
    let Some(element) = widget.ui_element() else {
        return Ok(());
    };
    let focus_events = Arc::clone(events);
    let focus_handler = RoutedEventHandler::new(move |_, _| {
        push_event(&focus_events, NativeEvent::new(id, NativeEventKind::Focus));
        Ok(())
    });
    map_winui(
        "failed to register WinUI focus handler",
        element.GotFocus(&focus_handler),
    )?;

    let blur_events = Arc::clone(events);
    let blur_handler = RoutedEventHandler::new(move |_, _| {
        push_event(&blur_events, NativeEvent::new(id, NativeEventKind::Blur));
        Ok(())
    });
    map_winui(
        "failed to register WinUI blur handler",
        element.LostFocus(&blur_handler),
    )?;
    Ok(())
}

pub(super) fn set_combo_box_item_content(
    item: &Controls::ComboBoxItem,
    value: &str,
) -> GuiResult<()> {
    let content = text_content(value)?;
    map_winui(
        "failed to set WinUI combo box item content",
        item.SetContent(&content),
    )
}

pub(super) fn text_content(value: &str) -> GuiResult<Controls::TextBlock> {
    let text = map_winui(
        "failed to create WinUI text content",
        Controls::TextBlock::new(),
    )?;
    map_winui(
        "failed to set WinUI text content",
        text.SetText(&hstr(value)),
    )?;
    Ok(text)
}

pub(super) fn bool_reference(value: bool) -> GuiResult<windows::Foundation::IReference<bool>> {
    let value = map_winui(
        "failed to box WinUI boolean value",
        PropertyValue::CreateBoolean(value),
    )?;
    map_winui("failed to cast WinUI boolean value", value.cast())
}

pub(super) fn child_position(
    children_by_parent: &BTreeMap<HostNodeId, Vec<HostNodeId>>,
    child: HostNodeId,
) -> Option<(HostNodeId, usize)> {
    children_by_parent.iter().find_map(|(parent, children)| {
        children
            .iter()
            .position(|existing| *existing == child)
            .map(|index| (*parent, index))
    })
}

pub(super) fn hstr(value: &str) -> HSTRING {
    HSTRING::from(value)
}

pub(super) fn to_u32(value: usize) -> GuiResult<u32> {
    value
        .try_into()
        .map_err(|_| GuiError::host("WinUI collection index overflow"))
}

pub(super) fn push_event(events: &WinUiEventQueue, event: NativeEvent) {
    if let Ok(mut events) = events.lock() {
        events.push(event);
    }
}

pub(super) fn map_winui<T>(context: &str, result: windows_core::Result<T>) -> GuiResult<T> {
    result.map_err(|error| GuiError::host(format!("{context}: {error}")))
}
