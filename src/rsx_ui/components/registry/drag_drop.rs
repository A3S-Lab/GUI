use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_drag_drop_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiFileTrigger",
        ui_file_trigger,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onSelect", "")?
            .default_prop("acceptedFileTypes", "")?
            .default_prop("allowsMultiple", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDropZone",
        ui_drop_zone,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onDrop", "")?
            .default_prop("onDragEnter", "")?
            .default_prop("onDragLeave", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDraggable",
        ui_draggable,
        passthrough_contract()?
            .default_prop("onDragStart", "")?
            .default_prop("onDragMove", "")?
            .default_prop("onDragEnd", "")?
            .default_prop("dragType", "")?
            .default_prop("dragValue", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isDragging", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDroppable",
        ui_droppable,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onDrop", "")?
            .default_prop("onDropEnter", "")?
            .default_prop("onDropExit", "")?
            .default_prop("onDropMove", "")?
            .default_prop("acceptedDragTypes", "")?
            .default_prop("dropOperation", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isDropTarget", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDropIndicator",
        ui_drop_indicator,
        passthrough_contract()?
            .default_prop("orientation", "horizontal")?
            .default_prop("isTarget", false)?,
        None,
    )?;
    Ok(component)
}
