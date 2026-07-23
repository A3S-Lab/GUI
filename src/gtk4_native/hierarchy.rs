use super::*;

impl Gtk4NativeSurface {
    pub(super) fn insert_native_child_impl(
        &mut self,
        parent: HostNodeId,
        parent_handle: &Gtk4OsHandle,
        child: HostNodeId,
        child_handle: &Gtk4OsHandle,
        index: usize,
    ) -> GuiResult<()> {
        if let Gtk4OsWidget::Dialog(dialog) = &child_handle.widget {
            self.show_dialog_if_marked_visible(child, dialog);
            return Ok(());
        }

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
            return Ok(());
        }

        if self.notebook_tabs.contains_key(&parent) {
            self.update_notebook_tab_panel(parent, Some(child))?;
            return Ok(());
        }

        if let (Gtk4OsWidget::Menu(_), Gtk4OsWidget::MenuItem(item)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            self.menus.insert_item(parent, child, item, index);
            return Ok(());
        }

        if let (Gtk4OsWidget::MenuItem(item), Gtk4OsWidget::Menu(menu)) =
            (&parent_handle.widget, &child_handle.widget)
        {
            item.item.set_submenu(Some(&menu.model));
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
                let children = self.container_children.entry(parent).or_default();
                children.retain(|existing| *existing != child);
                let index = index.min(children.len());
                let native_index = index_to_i32(index)?;
                let events_suppressed = self.events_suppressed.clone();
                let previous = events_suppressed.replace(true);
                list_box.insert(&child_widget, native_index);
                events_suppressed.replace(previous);
                children.insert(index, child);
                self.list_item_parents.insert(child, parent);
                self.sync_list_values(parent);
                if let Gtk4OsWidget::ListBoxRow { row, item, .. } = &child_handle.widget {
                    self.update_list_item_selected(child, row, item.selected);
                }
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
        Ok(())
    }

    pub(super) fn remove_native_widget_impl(
        &mut self,
        id: HostNodeId,
        handle: Gtk4OsHandle,
    ) -> GuiResult<()> {
        if self.root == Some(id) {
            self.root = None;
        }
        interaction::forget_activation_context(&self.activation_contexts, id);
        self.interaction_nodes.borrow_mut().remove(&id);
        self.keyboard_presses.borrow_mut().remove(id);
        self.closed_windows.borrow_mut().remove(&id);
        self.dialog_visible.remove(&id);
        self.popover_positions
            .retain(|overlay, (anchor, _)| *overlay != id && *anchor != id);
        self.widgets.remove(&id);
        if let Some(parent) = self.list_item_parents.remove(&id) {
            if let Some(children) = self.container_children.get_mut(&parent) {
                children.retain(|child| *child != id);
            }
            self.sync_list_values(parent);
        }
        for children in self.container_children.values_mut() {
            children.retain(|child| *child != id);
        }
        self.container_children.remove(&id);
        self.list_values.borrow_mut().remove(&id);

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
            Gtk4OsWidget::ListBoxRow { row, .. } => {
                if row.parent().is_some() {
                    self.suppress_events(|| row.unparent());
                }
            }
            other => {
                if let Some(widget) = other.as_widget() {
                    if widget.parent().is_some() {
                        widget.unparent();
                    }
                }
            }
        }
        self.forget_accessibility_relationship_node(id)?;
        self.forget_accessibility_structure_node(id);
        Ok(())
    }
}
