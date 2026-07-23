use super::*;

impl AppKitNativeSurface {
    pub(super) fn insert_native_child_impl(
        &mut self,
        parent: HostNodeId,
        parent_handle: &AppKitOsHandle,
        child: HostNodeId,
        child_handle: &AppKitOsHandle,
        index: usize,
    ) -> GuiResult<()> {
        if let AppKitOsWidget::Panel(panel) = &child_handle.widget {
            self.show_panel_if_marked_visible(child, panel);
            return Ok(());
        }
        if let AppKitOsWidget::Popover(state) = &child_handle.widget {
            if parent_handle.widget.as_view().is_some() {
                self.popover_anchors.insert(child, parent);
                self.show_popover_if_marked_visible(child, state);
                return Ok(());
            }
            return Err(GuiError::host(
                "AppKit popover insertion requires an anchor NSView parent",
            ));
        }
        if matches!(child_handle.widget, AppKitOsWidget::Menu(_))
            && !matches!(parent_handle.widget, AppKitOsWidget::MenuItem(_))
        {
            if let AppKitOsWidget::Menu(menu) = &child_handle.widget {
                self._application.setMainMenu(Some(menu));
            }
            return Ok(());
        }

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
                GuiError::host(format!(
                    "AppKit tab item insertion requires an NSView child: parent={:?}({parent:?}) child={:?}({child:?})",
                    parent_handle.kind, child_handle.kind
                ))
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
            GuiError::host(format!(
                "AppKit native child insertion requires an NSView child: parent={:?}({parent:?}) child={:?}({child:?})",
                parent_handle.kind, child_handle.kind
            ))
        })?;
        match &parent_handle.widget {
            AppKitOsWidget::Window(window) => install_window_content_view(window, child),
            AppKitOsWidget::Panel(panel) => install_window_content_view(panel.as_super(), child),
            AppKitOsWidget::Popover(state) => state.content_view.addSubview(child),
            AppKitOsWidget::View(view) => view.addSubview(child),
            AppKitOsWidget::StackView(stack_view) => stack_view.insertArrangedSubview_atIndex(
                child,
                stack_arranged_insert_index(stack_view, index)?,
            ),
            AppKitOsWidget::ScrollView(state) => {
                state.stack_view.insertArrangedSubview_atIndex(
                    child,
                    stack_arranged_insert_index(&state.stack_view, index)?,
                );
                scroll_view_to_top(state);
            }
            AppKitOsWidget::ListView(scroll_view) => {
                if let Some(state) = self.list_views.get(&parent) {
                    state.stack_view.insertArrangedSubview_atIndex(
                        child,
                        stack_arranged_insert_index(&state.stack_view, index)?,
                    );
                    apply_list_view_layout(scroll_view, state, &state.style);
                }
            }
            AppKitOsWidget::Button(button) => button.as_super().as_super().addSubview(child),
            AppKitOsWidget::Switch(switch) => switch.as_super().as_super().addSubview(child),
            AppKitOsWidget::Slider(slider) => slider.as_super().as_super().addSubview(child),
            AppKitOsWidget::ProgressIndicator(progress) => progress.as_super().addSubview(child),
            AppKitOsWidget::TabView(tab_view) => tab_view.addSubview(child),
            AppKitOsWidget::Box(box_) => box_.as_super().addSubview(child),
            AppKitOsWidget::ComboBox(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::TabViewItem(_) => {}
            AppKitOsWidget::TextField(text_field) => {
                text_field.as_super().as_super().addSubview(child)
            }
            AppKitOsWidget::SearchField(text_field) => text_field
                .as_super()
                .as_super()
                .as_super()
                .addSubview(child),
            AppKitOsWidget::SecureTextField(text_field) => text_field
                .as_super()
                .as_super()
                .as_super()
                .addSubview(child),
        }
        Ok(())
    }

    pub(super) fn remove_native_widget_impl(
        &mut self,
        id: HostNodeId,
        handle: AppKitOsHandle,
    ) -> GuiResult<()> {
        let was_root = self.root == Some(id);
        if was_root {
            self.root = None;
        }
        self.widgets.remove(&id);
        self.action_targets.remove(&id);
        self.interaction_nodes.remove(&id);
        self.keyboard_presses.borrow_mut().remove(id);
        self.activation_contexts.borrow_mut().remove(&id);
        if self
            .pointer_press
            .borrow()
            .as_ref()
            .is_some_and(|active| active.node == id)
        {
            self.pointer_press.borrow_mut().take();
        }
        if self
            .hovered_pointer
            .borrow()
            .as_ref()
            .is_some_and(|hovered| hovered.node == id)
        {
            self.hovered_pointer.borrow_mut().take();
        }
        if self.focused_node.get() == Some(id) {
            self.focused_node.set(None);
        }
        self.unregister_responder(&handle.widget);
        self.ranges.remove(&id);
        self.text_inputs.remove(&id);
        self.text_input_configs.remove(&id);
        self.clear_native_size_constraints(id);
        self.closed_windows.borrow_mut().remove(&id);
        self.dialog_visible.remove(&id);
        self.popover_visible.remove(&id);
        self.popover_anchors.remove(&id);
        self.popover_anchors.retain(|_, anchor| *anchor != id);
        self.popover_positions.remove(&id);
        let popover_anchors = &self.popover_anchors;
        self.popover_positions
            .retain(|popover, _| popover_anchors.contains_key(popover));
        if let AppKitOsWidget::Window(window) = &handle.widget {
            window.setDelegate(None);
            self.window_delegates.remove(&id);
            window.close();
        }
        if let AppKitOsWidget::Panel(panel) = &handle.widget {
            panel.as_super().setDelegate(None);
            self.window_delegates.remove(&id);
        }
        if let AppKitOsWidget::ComboBox(_) = &handle.widget {
            self.combo_boxes.remove(&id);
            if let Some(children) = self.combo_children.remove(&id) {
                for child in children {
                    self.combo_item_parents.remove(&child);
                }
            }
        }
        if let AppKitOsWidget::ListView(_) = &handle.widget {
            if let Some(state) = self.list_views.remove(&id) {
                for row in state.rows.borrow().iter() {
                    self.unregister_view_responder(row.button_view());
                }
            }
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
}
