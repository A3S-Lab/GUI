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
        self.present_visible_dialogs();
        Ok(())
    }

    fn request_native_focus(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        if handle.id != id {
            return Err(GuiError::host(format!(
                "GTK4 handle id does not match focus target {}",
                id.get()
            )));
        }
        let widget = handle.widget.as_widget().ok_or_else(|| {
            GuiError::host(format!(
                "GTK4 widget {} cannot receive keyboard focus",
                id.get()
            ))
        })?;
        if !widget.grab_focus() {
            return Err(GuiError::host(format!(
                "GTK4 widget {} did not accept keyboard focus",
                id.get()
            )));
        }
        Ok(())
    }

    fn announce_native_accessibility(
        &mut self,
        announcement: &AccessibilityAnnouncement,
        handle: &Self::Handle,
    ) -> GuiResult<()> {
        if handle.id != announcement.node {
            return Err(GuiError::host(format!(
                "GTK4 handle id does not match accessibility announcement target {}",
                announcement.node.get()
            )));
        }
        if announcement.message.trim().is_empty() {
            return Ok(());
        }
        let widget = handle.widget.as_widget().ok_or_else(|| {
            GuiError::host(format!(
                "GTK4 widget {} cannot post an accessibility announcement",
                announcement.node.get()
            ))
        })?;
        widget.announce(
            &announcement.message,
            match announcement.priority {
                AccessibilityAnnouncementPriority::Polite => {
                    gtk::AccessibleAnnouncementPriority::Medium
                }
                AccessibilityAnnouncementPriority::Assertive => {
                    gtk::AccessibleAnnouncementPriority::High
                }
            },
        );
        Ok(())
    }

    fn measure_native_collection_layout(
        &mut self,
        collection: HostNodeId,
        collection_handle: &Self::Handle,
        items: &[(HostNodeId, CollectionKey, Self::Handle)],
    ) -> GuiResult<Option<CollectionLayoutSnapshot>> {
        if collection_handle.id != collection {
            return Err(GuiError::host(format!(
                "GTK4 handle id does not match collection {}",
                collection.get()
            )));
        }
        let Gtk4OsWidget::ListBox(list_box) = &collection_handle.widget else {
            return Ok(None);
        };

        let allocation = list_box.allocation();
        let mut content_width = f64::from(allocation.width().max(0));
        let mut content_height = f64::from(allocation.height().max(0));
        let mut item_rects = Vec::new();
        for (_, key, handle) in items {
            let Gtk4OsWidget::ListBoxRow { row, .. } = &handle.widget else {
                continue;
            };
            let allocation = row.allocation();
            let rect = Rect::new(
                f64::from(allocation.x()),
                f64::from(allocation.y()),
                f64::from(allocation.width().max(0)),
                f64::from(allocation.height().max(0)),
            );
            content_width = content_width.max(rect.x + rect.width);
            content_height = content_height.max(rect.y + rect.height);
            item_rects.push((key.clone(), rect));
        }

        let scroller = gtk_scrolled_window_ancestor(list_box);
        let (visible_rect, content_size) = if let Some(scroller) = scroller {
            let horizontal = scroller.hadjustment();
            let vertical = scroller.vadjustment();
            let visible_width = positive_extent_or(horizontal.page_size(), scroller.width());
            let visible_height = positive_extent_or(vertical.page_size(), scroller.height());
            (
                Rect::new(
                    horizontal.value(),
                    vertical.value(),
                    visible_width,
                    visible_height,
                ),
                Size::new(
                    horizontal.upper().max(content_width),
                    vertical.upper().max(content_height),
                ),
            )
        } else {
            (
                Rect::new(0.0, 0.0, content_width, content_height),
                Size::new(content_width, content_height),
            )
        };
        let mut layout = CollectionLayoutSnapshot::new(visible_rect, content_size);
        for (key, rect) in item_rects {
            layout.insert_item_rect(key, rect);
        }
        layout.validate()?;
        Ok(Some(layout))
    }

    fn position_native_overlay(
        &mut self,
        overlay: HostNodeId,
        overlay_handle: &Self::Handle,
        anchor: HostNodeId,
        anchor_handle: &Self::Handle,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        if overlay_handle.id != overlay || anchor_handle.id != anchor {
            return Err(GuiError::host(
                "GTK4 overlay or anchor handle id does not match the positioning command",
            ));
        }
        let Gtk4OsWidget::Popover(popover) = &overlay_handle.widget else {
            return Err(GuiError::host(format!(
                "GTK4 widget {} is not a gtk::Popover",
                overlay.get()
            )));
        };
        let anchor_widget = anchor_handle.widget.as_widget().ok_or_else(|| {
            GuiError::host(format!(
                "GTK4 overlay anchor {} is not a widget",
                anchor.get()
            ))
        })?;
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        if popover.parent().as_ref() != Some(&anchor_widget) {
            if popover.parent().is_some() {
                popover.unparent();
            }
            popover.set_parent(&anchor_widget);
        }

        let placement = request.resolved_placement();
        let position = match placement.axis {
            OverlayPlacementAxis::Top => gtk::PositionType::Top,
            OverlayPlacementAxis::Bottom => gtk::PositionType::Bottom,
            OverlayPlacementAxis::Left => gtk::PositionType::Left,
            OverlayPlacementAxis::Right => gtk::PositionType::Right,
        };
        popover.set_position(position);

        let width = anchor_widget.width().max(0);
        let height = anchor_widget.height().max(0);
        let (point_x, point_y) = if placement.axis.is_vertical() {
            (
                aligned_gtk_coordinate(width, placement.alignment),
                if matches!(placement.axis, OverlayPlacementAxis::Top) {
                    0
                } else {
                    height
                },
            )
        } else {
            (
                if matches!(placement.axis, OverlayPlacementAxis::Left) {
                    0
                } else {
                    width
                },
                aligned_gtk_coordinate(height, placement.alignment),
            )
        };
        popover.set_pointing_to(Some(&gtk::gdk::Rectangle::new(point_x, point_y, 1, 1)));

        let main_offset = overlay_offset_i32(request.options.offset);
        let cross_offset = overlay_offset_i32(request.options.cross_offset);
        let (x_offset, y_offset) = match placement.axis {
            OverlayPlacementAxis::Top => (cross_offset, -main_offset),
            OverlayPlacementAxis::Bottom => (cross_offset, main_offset),
            OverlayPlacementAxis::Left => (-main_offset, cross_offset),
            OverlayPlacementAxis::Right => (main_offset, cross_offset),
        };
        popover.set_offset(x_offset, y_offset);
        self.popover_positions.insert(overlay, (anchor, request));
        if popover.is_visible() {
            popover.popup();
        }
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
}

fn gtk_scrolled_window_ancestor(widget: &gtk::ListBox) -> Option<gtk::ScrolledWindow> {
    let mut ancestor = widget.parent();
    while let Some(widget) = ancestor {
        if let Ok(scroller) = widget.clone().downcast::<gtk::ScrolledWindow>() {
            return Some(scroller);
        }
        ancestor = widget.parent();
    }
    None
}

fn positive_extent_or(extent: f64, fallback: i32) -> f64 {
    if extent > 0.0 {
        extent
    } else {
        f64::from(fallback.max(0))
    }
}

fn aligned_gtk_coordinate(size: i32, alignment: OverlayCrossAlignment) -> i32 {
    match alignment {
        OverlayCrossAlignment::Near => 0,
        OverlayCrossAlignment::Center => size / 2,
        OverlayCrossAlignment::Far => size,
    }
}

fn overlay_offset_i32(value: f64) -> i32 {
    value
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
}

impl Gtk4NativeSurface {
    pub(super) fn set_dialog_visible(
        &mut self,
        id: HostNodeId,
        dialog: &gtk::Dialog,
        visible: bool,
    ) {
        self.dialog_visible.insert(id, visible);
        if visible {
            self.show_dialog_if_marked_visible(id, dialog);
        } else {
            dialog.hide();
        }
    }

    pub(super) fn show_dialog_if_marked_visible(&mut self, id: HostNodeId, dialog: &gtk::Dialog) {
        if self.root.is_some() && self.dialog_visible.get(&id).copied().unwrap_or(false) {
            self.closed_windows.borrow_mut().remove(&id);
            dialog.present();
        }
    }

    pub(super) fn present_visible_dialogs(&mut self) {
        let dialogs = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| {
                if self.dialog_visible.get(id).copied().unwrap_or(false) {
                    widget
                        .clone()
                        .downcast::<gtk::Dialog>()
                        .ok()
                        .map(|dialog| (*id, dialog))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (id, dialog) in dialogs {
            self.show_dialog_if_marked_visible(id, &dialog);
        }
    }
}

pub(super) fn set_widget_title(widget: &Gtk4OsWidget, title: Option<&str>) {
    if let Some(widget) = widget.as_widget() {
        widget.set_tooltip_text(title);
    }
}
