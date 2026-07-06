#[path = "autocomplete.rsx"]
mod autocomplete;
#[path = "badge.rsx"]
mod badge;
#[path = "breadcrumb.rsx"]
mod breadcrumb;
#[path = "breadcrumbs.rsx"]
mod breadcrumbs;
#[path = "button.rsx"]
mod button;
#[path = "card.rsx"]
mod card;
#[path = "card_content.rsx"]
mod card_content;
#[path = "card_description.rsx"]
mod card_description;
#[path = "card_footer.rsx"]
mod card_footer;
#[path = "card_header.rsx"]
mod card_header;
#[path = "card_title.rsx"]
mod card_title;
#[path = "checkbox.rsx"]
mod checkbox;
#[path = "checkbox_group.rsx"]
mod checkbox_group;
#[path = "combo_box.rsx"]
mod combo_box;
#[path = "dialog.rsx"]
mod dialog;
#[path = "disclosure.rsx"]
mod disclosure;
#[path = "disclosure_group.rsx"]
mod disclosure_group;
#[path = "disclosure_summary.rsx"]
mod disclosure_summary;
#[path = "drop_zone.rsx"]
mod drop_zone;
#[path = "field_set.rsx"]
mod field_set;
#[path = "file_trigger.rsx"]
mod file_trigger;
#[path = "form.rsx"]
mod form;
#[path = "grid_list.rsx"]
mod grid_list;
#[path = "grid_list_item.rsx"]
mod grid_list_item;
#[path = "group.rsx"]
mod group;
#[path = "heading.rsx"]
mod heading;
#[path = "input.rsx"]
mod input;
#[path = "label.rsx"]
mod label;
#[path = "legend.rsx"]
mod legend;
#[path = "link.rsx"]
mod link;
#[path = "list_box.rsx"]
mod list_box;
#[path = "list_box_item.rsx"]
mod list_box_item;
#[path = "menu.rsx"]
mod menu;
#[path = "menu_item.rsx"]
mod menu_item;
#[path = "meter.rsx"]
mod meter;
#[path = "modal.rsx"]
mod modal;
#[path = "number_field.rsx"]
mod number_field;
#[path = "popover.rsx"]
mod popover;
#[path = "progress_bar.rsx"]
mod progress_bar;
#[path = "radio.rsx"]
mod radio;
#[path = "radio_group.rsx"]
mod radio_group;
#[path = "search_field.rsx"]
mod search_field;
#[path = "select.rsx"]
mod select;
#[path = "select_value.rsx"]
mod select_value;
#[path = "separator.rsx"]
mod separator;
#[path = "slider.rsx"]
mod slider;
#[path = "switch.rsx"]
mod switch;
#[path = "table.rsx"]
mod table;
#[path = "table_body.rsx"]
mod table_body;
#[path = "table_caption.rsx"]
mod table_caption;
#[path = "table_cell.rsx"]
mod table_cell;
#[path = "table_column.rsx"]
mod table_column;
#[path = "table_header.rsx"]
mod table_header;
#[path = "table_row.rsx"]
mod table_row;
#[path = "tabs.rsx"]
mod tabs;
#[path = "tabs_content.rsx"]
mod tabs_content;
#[path = "tabs_list.rsx"]
mod tabs_list;
#[path = "tabs_trigger.rsx"]
mod tabs_trigger;
#[path = "tag.rsx"]
mod tag;
#[path = "tag_group.rsx"]
mod tag_group;
#[path = "text.rsx"]
mod text;
#[path = "text_field.rsx"]
mod text_field;
#[path = "textarea.rsx"]
mod textarea;
#[path = "toast.rsx"]
mod toast;
#[path = "toast_region.rsx"]
mod toast_region;
#[path = "toggle_button.rsx"]
mod toggle_button;
#[path = "toggle_button_group.rsx"]
mod toggle_button_group;
#[path = "toolbar.rsx"]
mod toolbar;
#[path = "tooltip.rsx"]
mod tooltip;
#[path = "tree.rsx"]
mod tree;
#[path = "tree_item.rsx"]
mod tree_item;
#[path = "tree_item_content.rsx"]
mod tree_item_content;
#[path = "virtualizer.rsx"]
mod virtualizer;

use serde_json::Value as JsonValue;

use crate::compiler::ComponentClassVariants;
use crate::error::GuiResult;
use crate::rsx_app::{ComponentCx, RsxComponent, RsxComponentContract, RSX};
use crate::rsx_ui::variants::{ui_badge_variants, ui_button_variants};

pub use autocomplete::{ui_autocomplete, UiAutocompleteProps};
pub use badge::{ui_badge, UiBadgeProps};
pub use breadcrumb::{ui_breadcrumb, UiBreadcrumbProps};
pub use breadcrumbs::{ui_breadcrumbs, UiBreadcrumbsProps};
pub use button::{ui_button, UiButtonProps};
pub use card::{ui_card, UiCardProps};
pub use card_content::{ui_card_content, UiCardContentProps};
pub use card_description::{ui_card_description, UiCardDescriptionProps};
pub use card_footer::{ui_card_footer, UiCardFooterProps};
pub use card_header::{ui_card_header, UiCardHeaderProps};
pub use card_title::{ui_card_title, UiCardTitleProps};
pub use checkbox::{ui_checkbox, UiCheckboxProps};
pub use checkbox_group::{ui_checkbox_group, UiCheckboxGroupProps};
pub use combo_box::{ui_combo_box, UiComboBoxProps};
pub use dialog::{ui_dialog, UiDialogProps};
pub use disclosure::{ui_disclosure, UiDisclosureProps};
pub use disclosure_group::{ui_disclosure_group, UiDisclosureGroupProps};
pub use disclosure_summary::{ui_disclosure_summary, UiDisclosureSummaryProps};
pub use drop_zone::{ui_drop_zone, UiDropZoneProps};
pub use field_set::{ui_field_set, UiFieldSetProps};
pub use file_trigger::{ui_file_trigger, UiFileTriggerProps};
pub use form::{ui_form, UiFormProps};
pub use grid_list::{ui_grid_list, UiGridListProps};
pub use grid_list_item::{ui_grid_list_item, UiGridListItemProps};
pub use group::{ui_group, UiGroupProps};
pub use heading::{ui_heading, UiHeadingProps};
pub use input::{ui_input, UiInputProps};
pub use label::{ui_label, UiLabelProps};
pub use legend::{ui_legend, UiLegendProps};
pub use link::{ui_link, UiLinkProps};
pub use list_box::{ui_list_box, UiListBoxProps};
pub use list_box_item::{ui_list_box_item, UiListBoxItemProps};
pub use menu::{ui_menu, UiMenuProps};
pub use menu_item::{ui_menu_item, UiMenuItemProps};
pub use meter::{ui_meter, UiMeterProps};
pub use modal::{ui_modal, UiModalProps};
pub use number_field::{ui_number_field, UiNumberFieldProps};
pub use popover::{ui_popover, UiPopoverProps};
pub use progress_bar::{ui_progress_bar, UiProgressBarProps};
pub use radio::{ui_radio, UiRadioProps};
pub use radio_group::{ui_radio_group, UiRadioGroupProps};
pub use search_field::{ui_search_field, UiSearchFieldProps};
pub use select::{ui_select, UiSelectProps};
pub use select_value::{ui_select_value, UiSelectValueProps};
pub use separator::{ui_separator, UiSeparatorProps};
pub use slider::{ui_slider, UiSliderProps};
pub use switch::{ui_switch, UiSwitchProps};
pub use table::{ui_table, UiTableProps};
pub use table_body::{ui_table_body, UiTableBodyProps};
pub use table_caption::{ui_table_caption, UiTableCaptionProps};
pub use table_cell::{ui_table_cell, UiTableCellProps};
pub use table_column::{ui_table_column, UiTableColumnProps};
pub use table_header::{ui_table_header, UiTableHeaderProps};
pub use table_row::{ui_table_row, UiTableRowProps};
pub use tabs::{ui_tabs, UiTabsProps};
pub use tabs_content::{ui_tabs_content, UiTabsContentProps};
pub use tabs_list::{ui_tabs_list, UiTabsListProps};
pub use tabs_trigger::{ui_tabs_trigger, UiTabsTriggerProps};
pub use tag::{ui_tag, UiTagProps};
pub use tag_group::{ui_tag_group, UiTagGroupProps};
pub use text::{ui_text, UiTextProps};
pub use text_field::{ui_text_field, UiTextFieldProps};
pub use textarea::{ui_textarea, UiTextareaProps};
pub use toast::{ui_toast, UiToastProps};
pub use toast_region::{ui_toast_region, UiToastRegionProps};
pub use toggle_button::{ui_toggle_button, UiToggleButtonProps};
pub use toggle_button_group::{ui_toggle_button_group, UiToggleButtonGroupProps};
pub use toolbar::{ui_toolbar, UiToolbarProps};
pub use tooltip::{ui_tooltip, UiTooltipProps};
pub use tree::{ui_tree, UiTreeProps};
pub use tree_item::{ui_tree_item, UiTreeItemProps};
pub use tree_item_content::{ui_tree_item_content, UiTreeItemContentProps};
pub use virtualizer::{ui_virtualizer, UiVirtualizerProps};

pub(crate) fn with_builtin_components<S>(component: RsxComponent<S>) -> GuiResult<RsxComponent<S>> {
    let component = with_builtin_template(
        component,
        "UiButton",
        ui_button,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        Some(ui_button_variants()?),
    )?;
    let component = with_builtin_template(
        component,
        "UiInput",
        ui_input,
        passthrough_contract()?
            .default_prop("type", "text")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTextField",
        ui_text_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSearchField",
        ui_search_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "Search")?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiNumberField",
        ui_number_field,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("placeholder", "")?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?
            .default_prop("stepValue", 1.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLabel",
        ui_label,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiText",
        ui_text,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiHeading",
        ui_heading,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?
            .default_prop("level", 2_u32)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLink",
        ui_link,
        passthrough_contract()?
            .default_prop("href", "")?
            .default_prop("onPress", "")?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBreadcrumbs",
        ui_breadcrumbs,
        passthrough_contract()?.default_prop("label", "Breadcrumbs")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBreadcrumb",
        ui_breadcrumb,
        passthrough_contract()?
            .default_prop("href", "")?
            .default_prop("onPress", "")?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTextarea",
        ui_textarea,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
            .default_prop("onChange", "")?
            .default_prop("rows", "")?
            .default_prop("cols", "")?
            .default_prop("maxLength", "")?,
        None,
    )?;
    let component =
        with_builtin_template(component, "UiCard", ui_card, passthrough_contract()?, None)?;
    let component = with_builtin_template(
        component,
        "UiCardHeader",
        ui_card_header,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardTitle",
        ui_card_title,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardDescription",
        ui_card_description,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardContent",
        ui_card_content,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCardFooter",
        ui_card_footer,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiBadge",
        ui_badge,
        passthrough_contract()?,
        Some(ui_badge_variants()?),
    )?;
    let component = with_builtin_template(
        component,
        "UiForm",
        ui_form,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("onSubmit", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiFieldSet",
        ui_field_set,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiLegend",
        ui_legend,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCheckbox",
        ui_checkbox,
        passthrough_contract()?
            .default_prop("onChange", "")?
            .default_prop("isChecked", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiCheckboxGroup",
        ui_checkbox_group,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSwitch",
        ui_switch,
        passthrough_contract()?
            .default_prop("onChange", "")?
            .default_prop("isChecked", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRadioGroup",
        ui_radio_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiRadio",
        ui_radio,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("textValue", "")?
            .default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSeparator",
        ui_separator,
        passthrough_contract()?.default_prop("orientation", "horizontal")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDialog",
        ui_dialog,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?
            .default_prop("onClose", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiModal",
        ui_modal,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?
            .default_prop("onClose", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosure",
        ui_disclosure,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isExpanded", false)?
            .default_prop("onExpandedChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosureGroup",
        ui_disclosure_group,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiDisclosureSummary",
        ui_disclosure_summary,
        passthrough_contract()?.default_prop("onPress", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSelect",
        ui_select,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("placeholder", "")?
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
        "UiPopover",
        ui_popover,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTooltip",
        ui_tooltip,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("isOpen", false)?,
        None,
    )?;
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
        "UiListBox",
        ui_list_box,
        passthrough_contract()?.default_prop("onSelectionChange", "")?,
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
        "UiGridList",
        ui_grid_list,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("onSelectionChange", "")?,
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
        "UiTagGroup",
        ui_tag_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
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
            .default_prop("onSelectionChange", "")?,
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
    let component =
        with_builtin_template(component, "UiMenu", ui_menu, passthrough_contract()?, None)?;
    let component = with_builtin_template(
        component,
        "UiMenuItem",
        ui_menu_item,
        passthrough_contract()?
            .default_prop("onAction", "")?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiSlider",
        ui_slider,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?
            .default_prop("stepValue", 1.0)?
            .default_prop("onChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiProgressBar",
        ui_progress_bar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiMeter",
        ui_meter,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("valueNumber", 0.0)?
            .default_prop("minValue", 0.0)?
            .default_prop("maxValue", 100.0)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToolbar",
        ui_toolbar,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("orientation", "horizontal")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToggleButton",
        ui_toggle_button,
        passthrough_contract()?
            .default_prop("onPress", "")?
            .default_prop("isSelected", false)?
            .default_prop("actionValue", "")?
            .default_prop_value("actionPayload", JsonValue::Null)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToggleButtonGroup",
        ui_toggle_button_group,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("value", "")?
            .default_prop("orientation", "horizontal")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiGroup",
        ui_group,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToastRegion",
        ui_toast_region,
        passthrough_contract()?.default_prop("label", "Notifications")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiToast",
        ui_toast,
        passthrough_contract()?
            .default_prop("title", "")?
            .default_prop("description", "")?
            .default_prop("onClose", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiVirtualizer",
        ui_virtualizer,
        passthrough_contract()?.default_prop("label", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTable",
        ui_table,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableHeader",
        ui_table_header,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableBody",
        ui_table_body,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableRow",
        ui_table_row,
        passthrough_contract()?.default_prop("isSelected", false)?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableColumn",
        ui_table_column,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableCell",
        ui_table_cell,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTableCaption",
        ui_table_caption,
        passthrough_contract()?
            .default_prop("label", "")?
            .default_prop("textValue", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabs",
        ui_tabs,
        passthrough_contract()?
            .default_prop("value", "")?
            .default_prop("onSelectionChange", "")?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabsList",
        ui_tabs_list,
        passthrough_contract()?,
        None,
    )?;
    let component = with_builtin_template(
        component,
        "UiTabsTrigger",
        ui_tabs_trigger,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )?;
    with_builtin_template(
        component,
        "UiTabsContent",
        ui_tabs_content,
        passthrough_contract()?.default_prop("value", "")?,
        None,
    )
}

fn with_builtin_template<S, P, F>(
    component: RsxComponent<S>,
    name: &str,
    render: F,
    contract: RsxComponentContract,
    variants: Option<ComponentClassVariants>,
) -> GuiResult<RsxComponent<S>>
where
    P: 'static,
    F: FnOnce(&mut ComponentCx<P>) -> RSX,
{
    let template = ComponentCx::compile_bare(name, render)?;
    let component = component.use_template_component_with_contract(
        name,
        template.template().clone(),
        contract,
    )?;
    match variants {
        Some(variants) => component.use_component_class_variants(name, variants),
        None => Ok(component),
    }
}

pub(super) fn passthrough_contract() -> GuiResult<RsxComponentContract> {
    RsxComponentContract::open()
        .optional([
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
            "isSelected",
            "isPressed",
            "press",
            "pressProps",
            "onPress",
            "onPressStart",
            "onPressEnd",
            "onChange",
            "onSelectionChange",
            "onAction",
            "onToggle",
            "onExpandedChange",
            "onSubmit",
            "onClose",
            "onRemove",
            "onSelect",
            "onDrop",
            "onDragEnter",
            "onDragLeave",
            "actionValue",
            "actionPayload",
            "label",
            "textValue",
            "title",
            "description",
            "aria-label",
            "href",
            "acceptedFileTypes",
            "allowsMultiple",
            "level",
            "type",
            "value",
            "inputValue",
            "placeholder",
            "orientation",
            "isChecked",
            "isExpanded",
            "isOpen",
            "minValue",
            "maxValue",
            "stepValue",
            "valueNumber",
            "rows",
            "cols",
            "maxLength",
            "variant",
            "size",
            "tone",
            "density",
        ])
        .default_prop("className", "")
}
