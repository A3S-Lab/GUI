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

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events.borrow_mut())
    }
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
