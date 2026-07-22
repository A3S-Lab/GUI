use super::*;
use crate::style::OverflowMode;

impl AppKitNativeSurface {
    pub(super) fn apply_range(&mut self, id: HostNodeId, widget: &AppKitOsWidget) {
        let range = self.ranges.get(&id).copied().unwrap_or_default();
        match widget {
            AppKitOsWidget::Slider(slider) => {
                slider.setMinValue(range.lower());
                slider.setMaxValue(range.upper());
                apply_slider_step(slider, range);
                slider.as_super().setDoubleValue(range.current());
            }
            AppKitOsWidget::ProgressIndicator(progress) => {
                apply_progress_range(progress, range);
            }
            _ => {}
        }
    }
}

impl NativeWidgetSurface for AppKitNativeSurface {
    type Handle = AppKitOsHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }

    fn create_native_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        self.create_native_widget_impl(id, blueprint)
    }

    fn apply_native_setter(
        &mut self,
        id: HostNodeId,
        handle: &Self::Handle,
        setter: &NativeWidgetSetter,
    ) -> GuiResult<()> {
        self.apply_native_setter_impl(id, handle, setter)
    }

    fn insert_native_child(
        &mut self,
        parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()> {
        self.insert_native_child_impl(parent, parent_handle, child, child_handle, index)
    }

    fn remove_native_widget(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        self.remove_native_widget_impl(id, handle)
    }

    fn set_native_root(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        let root_changed = self.root != Some(id);
        self.root = Some(id);
        if root_changed {
            match &handle.widget {
                AppKitOsWidget::Window(window) => {
                    self.closed_windows.borrow_mut().remove(&id);
                    window.makeKeyAndOrderFront(None);
                }
                AppKitOsWidget::Panel(panel) => {
                    self.closed_windows.borrow_mut().remove(&id);
                    panel.as_super().makeKeyAndOrderFront(None);
                }
                AppKitOsWidget::Menu(menu) => self._application.setMainMenu(Some(menu)),
                _ => {}
            }
        }
        self.present_visible_panels();
        self.present_visible_popovers();
        if root_changed {
            activate_current_application();
        }
        Ok(())
    }

    fn request_native_focus(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        if handle.id != id {
            return Err(GuiError::host(format!(
                "AppKit handle id does not match focus target {}",
                id.get()
            )));
        }
        let accepted = match &handle.widget {
            AppKitOsWidget::ComboBoxItem(_) => self
                .list_item_parents
                .get(&id)
                .and_then(|parent| self.list_views.get(parent))
                .and_then(|state| {
                    state
                        .rows
                        .borrow()
                        .iter()
                        .find(|row| row.node == id)
                        .map(|row| focus_appkit_view(row.button_view()))
                })
                .unwrap_or(false),
            _ => focus_appkit_widget(&handle.widget),
        };
        if !accepted {
            return Err(GuiError::host(format!(
                "AppKit widget {} did not accept keyboard focus",
                id.get()
            )));
        }
        let previous = self.focused_node.replace(Some(id));
        if previous != Some(id) {
            let mut events = self.events.borrow_mut();
            if let Some(previous) = previous {
                events.push(NativeEvent::new(previous, NativeEventKind::Blur));
            }
            events.push(NativeEvent::new(id, NativeEventKind::Focus));
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
}

impl AppKitNativeSurface {
    pub(super) fn set_panel_visible(&mut self, id: HostNodeId, panel: &NSPanel, visible: bool) {
        self.dialog_visible.insert(id, visible);
        if visible {
            self.show_panel_if_marked_visible(id, panel);
        } else {
            panel.as_super().orderOut(None);
        }
    }

    pub(super) fn show_panel_if_marked_visible(&mut self, id: HostNodeId, panel: &NSPanel) {
        if self.root.is_some() && self.dialog_visible.get(&id).copied().unwrap_or(false) {
            self.closed_windows.borrow_mut().remove(&id);
            panel.as_super().makeKeyAndOrderFront(None);
        }
    }

    pub(super) fn present_visible_panels(&mut self) {
        let panels = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| match widget {
                AppKitOsWidget::Panel(panel)
                    if self.dialog_visible.get(id).copied().unwrap_or(false) =>
                {
                    Some((*id, panel.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (id, panel) in panels {
            self.show_panel_if_marked_visible(id, &panel);
        }
    }

    pub(super) fn set_popover_visible(
        &mut self,
        id: HostNodeId,
        state: &AppKitPopoverState,
        visible: bool,
    ) {
        self.popover_visible.insert(id, visible);
        if visible {
            self.show_popover_if_marked_visible(id, state);
        } else {
            state.popover.close();
        }
    }

    pub(super) fn show_popover_if_marked_visible(
        &mut self,
        id: HostNodeId,
        state: &AppKitPopoverState,
    ) {
        if self.root.is_none() || !self.popover_visible.get(&id).copied().unwrap_or(false) {
            return;
        }

        let Some(anchor_id) = self.popover_anchors.get(&id).copied() else {
            return;
        };
        let Some(anchor_widget) = self.widgets.get(&anchor_id).cloned() else {
            return;
        };
        let Some(anchor_view) = anchor_widget.as_view() else {
            return;
        };
        if anchor_view.window().is_none() || anchor_view.isHiddenOrHasHiddenAncestor() {
            return;
        }

        state.popover.showRelativeToRect_ofView_preferredEdge(
            anchor_view.bounds(),
            anchor_view,
            NSRectEdge::MaxY,
        );
    }

    pub(super) fn present_visible_popovers(&mut self) {
        let popovers = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| match widget {
                AppKitOsWidget::Popover(state)
                    if self.popover_visible.get(id).copied().unwrap_or(false) =>
                {
                    Some((*id, state.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (id, state) in popovers {
            self.show_popover_if_marked_visible(id, &state);
        }
    }
}

pub(super) fn set_widget_title(widget: &AppKitOsWidget, title: Option<&str>) {
    let title = title.map(ns_string);
    let title = title.as_deref();
    match widget {
        AppKitOsWidget::Window(window) => {
            if let Some(content_view) = window.contentView() {
                content_view.setToolTip(title);
            }
        }
        AppKitOsWidget::Panel(panel) => {
            if let Some(content_view) = panel.as_super().contentView() {
                content_view.setToolTip(title);
            }
        }
        AppKitOsWidget::Popover(state) => state.content_view.setToolTip(title),
        AppKitOsWidget::MenuItem(menu_item) => menu_item.setToolTip(title),
        AppKitOsWidget::TabViewItem(tab_item) => tab_item.setToolTip(title),
        AppKitOsWidget::Menu(_) | AppKitOsWidget::ComboBoxItem(_) => {}
        _ => {
            if let Some(view) = widget.as_view() {
                view.setToolTip(title);
            }
        }
    }
}

pub(super) fn appkit_vertical_scroll_enabled(config: &NativeWidgetConfig) -> bool {
    scroll_enabled(config.portable_style.overflow_y)
        || scroll_enabled(config.portable_style.overflow_block)
        || (!scroll_enabled(config.portable_style.overflow_x)
            && !scroll_enabled(config.portable_style.overflow_inline))
}

pub(super) fn appkit_horizontal_scroll_enabled(config: &NativeWidgetConfig) -> bool {
    scroll_enabled(config.portable_style.overflow_x)
        || scroll_enabled(config.portable_style.overflow_inline)
}

fn scroll_enabled(value: Option<OverflowMode>) -> bool {
    matches!(value, Some(OverflowMode::Auto | OverflowMode::Scroll))
}
