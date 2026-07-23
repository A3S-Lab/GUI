use super::helpers::*;
use super::*;

impl WinUiNativeSurface {
    pub(super) fn insert_native_child_impl(
        &mut self,
        parent: HostNodeId,
        parent_handle: &WinUiOsHandle,
        child: HostNodeId,
        child_handle: &WinUiOsHandle,
        index: usize,
    ) -> GuiResult<()> {
        if let WinUiOsWidget::ContentDialog(dialog) = &child_handle.widget {
            self.show_content_dialog_if_marked_visible(child, dialog)?;
            return Ok(());
        }

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
            WinUiOsWidget::ScrollViewer { content, .. } => {
                let child_element = child_handle.widget.ui_element().ok_or_else(|| {
                    GuiError::host("WinUI scroll viewer child must be a UIElement-backed widget")
                })?;
                self.insert_panel_child(
                    parent,
                    map_winui(
                        "failed to read WinUI scroll viewer content children",
                        content.Children(),
                    )?,
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
                let items = map_winui("failed to read WinUI list box items", list_box.Items())?;
                let events_suppressed = Arc::clone(&self.events_suppressed);
                let previous = events_suppressed.swap(true, Ordering::SeqCst);
                let insert_result = Self::insert_items_child(
                    &mut self.list_children,
                    parent,
                    items,
                    child,
                    child_object,
                    index,
                );
                events_suppressed.store(previous, Ordering::SeqCst);
                insert_result?;
                self.list_item_parents.insert(child, parent);
                self.sync_list_values(parent);
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
            | WinUiOsWidget::PasswordBox(_)
            | WinUiOsWidget::Slider(_)
            | WinUiOsWidget::ProgressBar(_) => {}
        }
        Ok(())
    }

    pub(super) fn remove_native_widget_impl(
        &mut self,
        id: HostNodeId,
        handle: WinUiOsHandle,
    ) -> GuiResult<()> {
        self.detach_child(id)?;
        if let Ok(mut contexts) = self.activation_contexts.lock() {
            contexts.remove(&id);
        }
        if let Ok(mut pending) = self.pending_activation_cleanup.lock() {
            pending.remove(&id);
        }
        self.interaction_nodes.remove(&id);
        if let Ok(mut presses) = self.keyboard_presses.lock() {
            presses.remove(id);
        }
        match &handle.widget {
            WinUiOsWidget::Window(window) => {
                map_winui("failed to close WinUI window", window.Close())?;
            }
            WinUiOsWidget::ContentDialog(dialog) => {
                self.hide_content_dialog(id, dialog)?;
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
        self.overlay_positions
            .retain(|overlay, (anchor, _)| *overlay != id && *anchor != id);
        self.dialog_visible.remove(&id);
        self.mark_content_dialog_closed(id);
        self.dialog_operations.remove(&id);
        self.combo_boxes.remove(&id);
        self.combo_items.remove(&id);
        self.combo_children.remove(&id);
        self.combo_selected_values.remove(&id);
        if let Ok(mut combo_values) = self.combo_values.lock() {
            combo_values.remove(&id);
        }
        self.container_children.remove(&id);
        self.list_children.remove(&id);
        self.list_item_parents.remove(&id);
        if let Ok(mut list_values) = self.list_values.lock() {
            list_values.remove(&id);
        }
        self.tab_children.remove(&id);
        self.tab_items.remove(&id);
        self.tab_selected_values.remove(&id);
        if let Ok(mut tab_values) = self.tab_values.lock() {
            tab_values.remove(&id);
        }
        self.ranges.remove(&id);
        self.text_inputs.remove(&id);
        self.text_input_configs.remove(&id);
        if let Ok(mut max_lengths) = self.text_input_max_lengths.lock() {
            max_lengths.remove(&id);
        }
        if let Ok(mut read_only) = self.text_input_read_only.lock() {
            read_only.remove(&id);
        }
        if let Ok(mut values) = self.text_input_values.lock() {
            values.remove(&id);
        }
        if let Ok(mut focused_node) = self.focused_node.lock() {
            if *focused_node == Some(id) {
                *focused_node = None;
            }
        }
        if self.root == Some(id) {
            self.root = None;
        }
        Ok(())
    }
}
