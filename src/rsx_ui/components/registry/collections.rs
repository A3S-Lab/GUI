use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::rsx_app::RsxComponent;

use super::super::catalog::*;
use super::super::contract::passthrough_contract;
use super::with_builtin_template;

pub(super) fn with_selection_input_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiSelect",
        ui_select,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("isOpen", false)?
            .default_prop("onOpenChange", "")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiComboBox",
        ui_combo_box,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("inputValue", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("isOpen", false)?
            .default_prop("onOpenChange", "")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiAutocomplete",
        ui_autocomplete,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("inputValue", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSelectValue",
        ui_select_value,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiComboBoxValue",
        ui_combo_box_value,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_collection_components<S>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiCollection",
        ui_collection,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("itemCount", 0)?
            .default_prop("isEmpty", false)?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiListBox",
        ui_list_box,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiListBoxSection",
        ui_list_box_section,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiListBoxHeader",
        ui_list_box_header,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiListBoxItem",
        ui_list_box_item,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiListBoxLoadMoreItem",
        ui_list_box_load_more_item,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onPress", "")?
            .default_prop("isLoading", false)?
            .default_prop("isDisabled", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGridList",
        ui_grid_list,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGridListSection",
        ui_grid_list_section,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGridListHeader",
        ui_grid_list_header,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGridListItem",
        ui_grid_list_item,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGridListLoadMoreItem",
        ui_grid_list_load_more_item,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onPress", "")?
            .default_prop("isLoading", false)?
            .default_prop("isDisabled", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTagGroup",
        ui_tag_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTagList",
        ui_tag_list,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTag",
        ui_tag,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("onRemove", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTree",
        ui_tree,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("selectionMode", "single")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTreeSection",
        ui_tree_section,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTreeHeader",
        ui_tree_header,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTreeItem",
        ui_tree_item,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isExpanded", false)?
            .default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTreeItemContent",
        ui_tree_item_content,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTreeLoadMoreItem",
        ui_tree_load_more_item,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onPress", "")?
            .default_prop("isLoading", false)?
            .default_prop("isDisabled", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    Ok(component)
}

pub(super) fn with_menu_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiMenu",
        ui_menu,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isDisabled", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMenuTrigger",
        ui_menu_trigger,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("isOpen", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMenuSection",
        ui_menu_section,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSubmenuTrigger",
        ui_submenu_trigger,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("onPressStart", "")?
            .default_prop("onPressEnd", "")?
            .default_prop("isDisabled", false)?
            .default_prop("isPressed", false)?
            .default_prop("isOpen", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMenuItem",
        ui_menu_item,
        passthrough_contract()?
            .default_prop("onAction", "")?
            .default_prop("actionValue", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?
            .default_prop("isDisabled", false)?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    Ok(component)
}
