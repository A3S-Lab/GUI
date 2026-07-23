use super::*;
use crate::style::OverflowMode;

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
        if let WinUiOsWidget::Window(window) = &handle.widget {
            map_winui("failed to activate WinUI window", window.Activate())?;
        }
        if let WinUiOsWidget::ToolTip(tool_tip) = &handle.widget {
            map_winui(
                "failed to open WinUI tooltip popover",
                tool_tip.SetIsOpen(true),
            )?;
        }
        self.present_visible_content_dialogs()?;
        Ok(())
    }

    fn request_native_focus(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        if handle.id != id {
            return Err(GuiError::host(format!(
                "WinUI handle id does not match focus target {}",
                id.get()
            )));
        }
        let element = handle.widget.ui_element().ok_or_else(|| {
            GuiError::host(format!(
                "WinUI widget {} cannot receive keyboard focus",
                id.get()
            ))
        })?;
        if !focus_winui_element(&element)? {
            return Err(GuiError::host(format!(
                "WinUI widget {} did not accept keyboard focus",
                id.get()
            )));
        }
        if let Ok(mut focused_node) = self.focused_node.lock() {
            *focused_node = Some(id);
        }
        Ok(())
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
                "WinUI overlay or anchor handle id does not match the positioning command",
            ));
        }
        let WinUiOsWidget::ToolTip(tool_tip) = &overlay_handle.widget else {
            return Err(GuiError::host(format!(
                "WinUI widget {} is not a ToolTip",
                overlay.get()
            )));
        };
        let anchor_element = anchor_handle.widget.ui_element().ok_or_else(|| {
            GuiError::host(format!(
                "WinUI overlay anchor {} is not a UIElement",
                anchor.get()
            ))
        })?;
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        map_winui(
            "failed to set WinUI tooltip placement target",
            tool_tip.SetPlacementTarget(&anchor_element),
        )?;

        if let Some(anchor_framework) = anchor_handle.widget.framework_element() {
            let width = anchor_framework.ActualWidth().unwrap_or(0.0).max(0.0);
            let height = anchor_framework.ActualHeight().unwrap_or(0.0).max(0.0);
            let placement = request.resolved_placement();
            let (x, y) = if placement.axis.is_vertical() {
                (
                    aligned_winui_coordinate(width, placement.alignment),
                    if matches!(placement.axis, OverlayPlacementAxis::Top) {
                        0.0
                    } else {
                        height
                    },
                )
            } else {
                (
                    if matches!(placement.axis, OverlayPlacementAxis::Left) {
                        0.0
                    } else {
                        width
                    },
                    aligned_winui_coordinate(height, placement.alignment),
                )
            };
            let placement_rect = winui_rect_reference(windows::Foundation::Rect {
                X: overlay_coordinate_f32(x),
                Y: overlay_coordinate_f32(y),
                Width: 0.0,
                Height: 0.0,
            })?;
            map_winui(
                "failed to set WinUI tooltip placement rectangle",
                tool_tip.SetPlacementRect(&placement_rect),
            )?;
        }

        let placement = request.resolved_placement();
        let (horizontal_offset, vertical_offset) = match placement.axis {
            OverlayPlacementAxis::Top => (request.options.cross_offset, -request.options.offset),
            OverlayPlacementAxis::Bottom => (request.options.cross_offset, request.options.offset),
            OverlayPlacementAxis::Left => (-request.options.offset, request.options.cross_offset),
            OverlayPlacementAxis::Right => (request.options.offset, request.options.cross_offset),
        };
        map_winui(
            "failed to set WinUI tooltip horizontal offset",
            tool_tip.SetHorizontalOffset(horizontal_offset),
        )?;
        map_winui(
            "failed to set WinUI tooltip vertical offset",
            tool_tip.SetVerticalOffset(vertical_offset),
        )?;
        if let (Some(max_height), Some(framework)) = (
            request.options.max_height,
            overlay_handle.widget.framework_element(),
        ) {
            map_winui(
                "failed to set WinUI tooltip maximum height",
                framework.SetMaxHeight(max_height),
            )?;
        }
        self.overlay_positions.insert(overlay, (anchor, request));
        Ok(())
    }

    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        let events = self
            .events
            .lock()
            .map(|mut events| std::mem::take(&mut *events))
            .unwrap_or_default();
        self.cleanup_closed_content_dialogs(&events);
        events
    }
}

fn aligned_winui_coordinate(size: f64, alignment: OverlayCrossAlignment) -> f64 {
    match alignment {
        OverlayCrossAlignment::Near => 0.0,
        OverlayCrossAlignment::Center => size / 2.0,
        OverlayCrossAlignment::Far => size,
    }
}

fn overlay_coordinate_f32(value: f64) -> f32 {
    value.clamp(f64::from(f32::MIN), f64::from(f32::MAX)) as f32
}

fn winui_rect_reference(
    value: windows::Foundation::Rect,
) -> GuiResult<windows::Foundation::IReference<windows::Foundation::Rect>> {
    let value = map_winui(
        "failed to box WinUI placement rectangle",
        PropertyValue::CreateRect(value),
    )?;
    map_winui("failed to cast WinUI placement rectangle", value.cast())
}

impl WinUiNativeSurface {
    pub(super) fn set_content_dialog_visible(
        &mut self,
        id: HostNodeId,
        dialog: &Controls::ContentDialog,
        visible: bool,
    ) -> GuiResult<()> {
        self.dialog_visible.insert(id, visible);
        if visible {
            self.show_content_dialog_if_marked_visible(id, dialog)
        } else {
            self.hide_content_dialog(id, dialog)
        }
    }

    pub(super) fn show_content_dialog_if_marked_visible(
        &mut self,
        id: HostNodeId,
        dialog: &Controls::ContentDialog,
    ) -> GuiResult<()> {
        if !self.dialog_visible.get(&id).copied().unwrap_or(false)
            || self.content_dialog_is_open(id)
        {
            return Ok(());
        }
        let Some(xaml_root) = self.root_xaml_root()? else {
            return Ok(());
        };

        let element: xaml::UIElement = map_winui(
            "failed to inspect WinUI content dialog as UI element",
            dialog.cast(),
        )?;
        map_winui(
            "failed to bind WinUI content dialog to root XamlRoot",
            element.SetXamlRoot(&xaml_root),
        )?;
        let operation = map_winui("failed to show WinUI content dialog", dialog.ShowAsync())?;
        let operation = map_winui(
            "failed to retain WinUI content dialog operation",
            operation.cast::<windows_core::IInspectable>(),
        )?;
        self.dialog_operations.insert(id, operation);
        self.mark_content_dialog_open(id);
        Ok(())
    }

    pub(super) fn hide_content_dialog(
        &mut self,
        id: HostNodeId,
        dialog: &Controls::ContentDialog,
    ) -> GuiResult<()> {
        self.dialog_operations.remove(&id);
        if self.mark_content_dialog_closed(id) {
            self.suppress_events(|| {
                map_winui("failed to hide WinUI content dialog", dialog.Hide())
            })?;
        }
        Ok(())
    }

    pub(super) fn present_visible_content_dialogs(&mut self) -> GuiResult<()> {
        let dialogs = self
            .widgets
            .iter()
            .filter_map(|(id, widget)| match widget {
                WinUiOsWidget::ContentDialog(dialog)
                    if self.dialog_visible.get(id).copied().unwrap_or(false) =>
                {
                    Some((*id, dialog.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        for (id, dialog) in dialogs {
            self.show_content_dialog_if_marked_visible(id, &dialog)?;
        }
        Ok(())
    }

    pub(super) fn content_dialog_is_open(&self, id: HostNodeId) -> bool {
        self.open_dialogs
            .lock()
            .map(|open_dialogs| open_dialogs.contains(&id))
            .unwrap_or(false)
    }

    pub(super) fn mark_content_dialog_open(&self, id: HostNodeId) {
        if let Ok(mut open_dialogs) = self.open_dialogs.lock() {
            open_dialogs.insert(id);
        }
    }

    pub(super) fn mark_content_dialog_closed(&self, id: HostNodeId) -> bool {
        self.open_dialogs
            .lock()
            .map(|mut open_dialogs| open_dialogs.remove(&id))
            .unwrap_or(false)
    }

    pub(super) fn cleanup_closed_content_dialogs(&mut self, events: &[NativeEvent]) {
        let closed_dialogs = events
            .iter()
            .filter_map(|event| {
                if event.kind == NativeEventKind::Close
                    && matches!(
                        self.widgets.get(&event.node),
                        Some(WinUiOsWidget::ContentDialog(_))
                    )
                {
                    Some(event.node)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for id in closed_dialogs {
            self.mark_content_dialog_closed(id);
            self.dialog_operations.remove(&id);
        }
    }

    pub(super) fn root_xaml_root(&self) -> GuiResult<Option<xaml::XamlRoot>> {
        let Some(root) = self.root else {
            return Ok(None);
        };
        let Some(WinUiOsWidget::Window(window)) = self.widgets.get(&root) else {
            return Ok(None);
        };
        let content = match window.Content() {
            Ok(content) => content,
            Err(_) => return Ok(None),
        };
        Ok(Some(map_winui(
            "failed to read WinUI root content XamlRoot",
            content.XamlRoot(),
        )?))
    }
}

pub(super) fn winui_scroll_visibility(
    value: Option<OverflowMode>,
) -> Controls::ScrollBarVisibility {
    match value {
        Some(OverflowMode::Scroll) => Controls::ScrollBarVisibility::Visible,
        Some(OverflowMode::Hidden | OverflowMode::Clip) => Controls::ScrollBarVisibility::Disabled,
        Some(OverflowMode::Visible | OverflowMode::Auto) | None => {
            Controls::ScrollBarVisibility::Auto
        }
    }
}
