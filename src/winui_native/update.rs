use super::helpers::*;
use super::*;

impl WinUiNativeSurface {
    pub(super) fn apply_native_setter_impl(
        &mut self,
        id: HostNodeId,
        handle: &WinUiOsHandle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        if let Some(registration) = self.interaction_nodes.get(&id) {
            if let Ok(mut registration) = registration.lock() {
                registration.apply_setter(setter);
            }
        }
        if let Some(config) = self.text_input_configs.get_mut(&id) {
            apply_widget_setter(config, setter);
        }

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
            NativeWidgetSetter::SetAccessibilityLabel(value) => {
                if let Some(element) = handle.widget.ui_element() {
                    map_winui(
                        "failed to set WinUI accessibility name",
                        automation::set_name(&element, value.as_deref()),
                    )?;
                }
            }
            NativeWidgetSetter::SetAccessibilityDescription(value) => {
                if let Some(element) = handle.widget.ui_element() {
                    map_winui(
                        "failed to set WinUI accessibility description",
                        automation::set_help_text(&element, value.description.as_deref()),
                    )?;
                    map_winui(
                        "failed to set WinUI accessibility key shortcuts",
                        automation::set_accelerator_key(&element, value.key_shortcuts.as_deref()),
                    )?;
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
            NativeWidgetSetter::SetTitle(value) => {
                set_title(&handle.widget, value.as_deref())?;
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
                if let Ok(mut inputs) = self.text_input_read_only.lock() {
                    inputs.insert(id, *read_only);
                }
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    map_winui(
                        "failed to set WinUI text box read-only state",
                        text_box.SetIsReadOnly(*read_only),
                    )?;
                }
            }
            NativeWidgetSetter::SetVisible(visible) => {
                if let WinUiOsWidget::ContentDialog(dialog) = &handle.widget {
                    self.set_content_dialog_visible(id, dialog, *visible)?;
                }
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
                self.suppress_events(|| set_selected(&handle.widget, *selected))?;
                if let Some(item) = self.combo_items.get(&id).cloned() {
                    self.update_combo_item_selected(id, &item, *selected)?;
                }
                if let Some(item) = self.tab_items.get(&id).cloned() {
                    self.update_tab_item_selected(id, &item, *selected)?;
                }
            }
            NativeWidgetSetter::SetMultiple(multiple) => {
                if let WinUiOsWidget::ListBox(list_box) = &handle.widget {
                    self.suppress_events(|| {
                        map_winui(
                            "failed to update WinUI list box selection mode",
                            list_box.SetSelectionMode(if *multiple {
                                Controls::SelectionMode::Multiple
                            } else {
                                Controls::SelectionMode::Single
                            }),
                        )
                    })?;
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
                if let WinUiOsWidget::ScrollViewer { viewer, .. } = &handle.widget {
                    map_winui(
                        "failed to set WinUI scroll viewer horizontal policy",
                        viewer.SetHorizontalScrollBarVisibility(winui_scroll_visibility(
                            style.overflow_x,
                        )),
                    )?;
                    map_winui(
                        "failed to set WinUI scroll viewer vertical policy",
                        viewer.SetVerticalScrollBarVisibility(winui_scroll_visibility(
                            style.overflow_y,
                        )),
                    )?;
                }
                if let WinUiOsWidget::Window(window) = &handle.widget {
                    apply_winui_window_portable_style(window, style)?;
                }
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    let size = style.native_size_constraints();
                    let sizing = self.text_inputs.entry(id).or_default();
                    sizing.explicit_width = size.width;
                    sizing.explicit_height = size.height;
                    self.apply_text_box_size_hint(id, text_box)?;
                }
                if let WinUiOsWidget::PasswordBox(password_box) = &handle.widget {
                    let size = style.native_size_constraints();
                    let sizing = self.text_inputs.entry(id).or_default();
                    sizing.explicit_width = size.width;
                    sizing.explicit_height = size.height;
                    self.apply_password_box_size_hint(id, password_box)?;
                }
            }
            NativeWidgetSetter::SetMaxLength(max_length) => {
                if let Ok(mut max_lengths) = self.text_input_max_lengths.lock() {
                    max_lengths.insert(id, *max_length);
                }
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    map_winui(
                        "failed to set WinUI text box max length",
                        text_box.SetMaxLength(winui_max_length_value(*max_length)),
                    )?;
                    let current =
                        map_winui("failed to read WinUI text box value", text_box.Text())?
                            .to_string();
                    let value = winui_truncate_to_max_length(&current, *max_length);
                    self.suppress_events(|| {
                        map_winui(
                            "failed to truncate WinUI text box value",
                            text_box.SetText(&hstr(&value)),
                        )
                    })?;
                    if let Ok(mut values) = self.text_input_values.lock() {
                        values.insert(id, value);
                    }
                }
                if let WinUiOsWidget::PasswordBox(password_box) = &handle.widget {
                    map_winui(
                        "failed to set WinUI password box max length",
                        password_box.SetMaxLength(winui_max_length_value(*max_length)),
                    )?;
                    let current = map_winui(
                        "failed to read WinUI password box value",
                        password_box.Password(),
                    )?
                    .to_string();
                    let value = winui_truncate_to_max_length(&current, *max_length);
                    self.suppress_events(|| {
                        map_winui(
                            "failed to truncate WinUI password box value",
                            password_box.SetPassword(&hstr(&value)),
                        )
                    })?;
                    if let Ok(mut values) = self.text_input_values.lock() {
                        values.insert(id, value);
                    }
                }
            }
            NativeWidgetSetter::SetAccessibilityRelationships(value) => {
                self.set_accessibility_relationships(id, value.clone())?;
            }
            NativeWidgetSetter::SetMetadata(value) => {
                self.set_accessibility_relationship_metadata(id, value)?;
            }
            NativeWidgetSetter::SetCols(value) => {
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_text_box_size_hint(id, text_box)?;
                }
                if let WinUiOsWidget::PasswordBox(password_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().cols = *value;
                    self.apply_password_box_size_hint(id, password_box)?;
                }
            }
            NativeWidgetSetter::SetSize(value) => {
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_text_box_size_hint(id, text_box)?;
                }
                if let WinUiOsWidget::PasswordBox(password_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().size = *value;
                    self.apply_password_box_size_hint(id, password_box)?;
                }
            }
            NativeWidgetSetter::SetRows(value) => {
                if let WinUiOsWidget::TextBox(text_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().rows = *value;
                    self.apply_text_box_size_hint(id, text_box)?;
                }
                if let WinUiOsWidget::PasswordBox(password_box) = &handle.widget {
                    self.text_inputs.entry(id).or_default().rows = *value;
                    self.apply_password_box_size_hint(id, password_box)?;
                }
            }
            NativeWidgetSetter::SetAutocomplete(_)
            | NativeWidgetSetter::SetInputMode(_)
            | NativeWidgetSetter::SetAutoCorrect(_)
            | NativeWidgetSetter::SetVirtualKeyboardPolicy(_)
            | NativeWidgetSetter::SetInputType(_)
            | NativeWidgetSetter::SetSpellCheck(_) => {
                self.apply_text_input_hints(id, &handle.widget)?;
            }
            NativeWidgetSetter::SetWindowResizable(value) => {
                if let WinUiOsWidget::Window(window) = &handle.widget {
                    set_winui_window_resizable(window, *value)?;
                }
            }
            NativeWidgetSetter::SetAccessibilityRole(_)
            | NativeWidgetSetter::SetAction(_)
            | NativeWidgetSetter::SetClassName(_)
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
            | NativeWidgetSetter::SetAccessibilityStructure(_)
            | NativeWidgetSetter::SetAccessibilityState(_)
            | NativeWidgetSetter::SetWebStyle(_)
            | NativeWidgetSetter::SetEvents(_) => {}
        }
        Ok(())
    }
}
