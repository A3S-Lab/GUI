use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::style::PortableStyle;

use super::NativeWidgetConfig;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum NativeWidgetSetter {
    SetAccessibilityRole(AccessibilityRole),
    SetLabel(Option<String>),
    SetValue(Option<String>),
    SetAction(Option<String>),
    SetClassName(Option<String>),
    SetPlaceholder(Option<String>),
    SetEnabled(bool),
    SetVisible(bool),
    SetRequired(bool),
    SetInvalid(bool),
    SetReadOnly(bool),
    SetMultiple(bool),
    SetAutoFocus(bool),
    SetSelected(bool),
    SetChecked(Option<bool>),
    SetExpanded(Option<bool>),
    SetOrientation(Option<Orientation>),
    SetMinimum(Option<f64>),
    SetMaximum(Option<f64>),
    SetCurrent(Option<f64>),
    SetStep(Option<f64>),
    SetAutocomplete(Option<String>),
    SetInputMode(Option<String>),
    SetPattern(Option<String>),
    SetMinLength(Option<u32>),
    SetMaxLength(Option<u32>),
    SetRows(Option<u32>),
    SetCols(Option<u32>),
    SetSize(Option<u32>),
    SetName(Option<String>),
    SetForm(Option<String>),
    SetInputType(Option<String>),
    SetAccept(Option<String>),
    SetCapture(Option<String>),
    SetAlt(Option<String>),
    SetSrc(Option<String>),
    SetList(Option<String>),
    SetDirname(Option<String>),
    SetFormAction(Option<String>),
    SetFormEnctype(Option<String>),
    SetFormMethod(Option<String>),
    SetFormTarget(Option<String>),
    SetFormNoValidate(bool),
    SetWebStyle(BTreeMap<String, String>),
    SetPortableStyle(PortableStyle),
    SetEvents(BTreeMap<String, String>),
    SetMetadata(BTreeMap<String, String>),
}

pub fn apply_widget_setter(config: &mut NativeWidgetConfig, setter: &NativeWidgetSetter) {
    match setter {
        NativeWidgetSetter::SetAccessibilityRole(value) => config.accessibility_role = *value,
        NativeWidgetSetter::SetLabel(value) => config.label = value.clone(),
        NativeWidgetSetter::SetValue(value) => config.value = value.clone(),
        NativeWidgetSetter::SetAction(value) => config.action = value.clone(),
        NativeWidgetSetter::SetClassName(value) => config.class_name = value.clone(),
        NativeWidgetSetter::SetPlaceholder(value) => config.placeholder = value.clone(),
        NativeWidgetSetter::SetEnabled(value) => config.enabled = *value,
        NativeWidgetSetter::SetVisible(value) => config.visible = *value,
        NativeWidgetSetter::SetRequired(value) => config.required = *value,
        NativeWidgetSetter::SetInvalid(value) => config.invalid = *value,
        NativeWidgetSetter::SetReadOnly(value) => config.read_only = *value,
        NativeWidgetSetter::SetMultiple(value) => config.multiple = *value,
        NativeWidgetSetter::SetAutoFocus(value) => config.auto_focus = *value,
        NativeWidgetSetter::SetSelected(value) => config.selected = *value,
        NativeWidgetSetter::SetChecked(value) => config.checked = *value,
        NativeWidgetSetter::SetExpanded(value) => config.expanded = *value,
        NativeWidgetSetter::SetOrientation(value) => config.orientation = *value,
        NativeWidgetSetter::SetMinimum(value) => config.min = *value,
        NativeWidgetSetter::SetMaximum(value) => config.max = *value,
        NativeWidgetSetter::SetCurrent(value) => config.current = *value,
        NativeWidgetSetter::SetStep(value) => config.step = *value,
        NativeWidgetSetter::SetAutocomplete(value) => config.autocomplete = value.clone(),
        NativeWidgetSetter::SetInputMode(value) => config.input_mode = value.clone(),
        NativeWidgetSetter::SetPattern(value) => config.pattern = value.clone(),
        NativeWidgetSetter::SetMinLength(value) => config.min_length = *value,
        NativeWidgetSetter::SetMaxLength(value) => config.max_length = *value,
        NativeWidgetSetter::SetRows(value) => config.rows = *value,
        NativeWidgetSetter::SetCols(value) => config.cols = *value,
        NativeWidgetSetter::SetSize(value) => config.size = *value,
        NativeWidgetSetter::SetName(value) => config.name = value.clone(),
        NativeWidgetSetter::SetForm(value) => config.form = value.clone(),
        NativeWidgetSetter::SetInputType(value) => config.input_type = value.clone(),
        NativeWidgetSetter::SetAccept(value) => config.accept = value.clone(),
        NativeWidgetSetter::SetCapture(value) => config.capture = value.clone(),
        NativeWidgetSetter::SetAlt(value) => config.alt = value.clone(),
        NativeWidgetSetter::SetSrc(value) => config.src = value.clone(),
        NativeWidgetSetter::SetList(value) => config.list = value.clone(),
        NativeWidgetSetter::SetDirname(value) => config.dirname = value.clone(),
        NativeWidgetSetter::SetFormAction(value) => config.form_action = value.clone(),
        NativeWidgetSetter::SetFormEnctype(value) => config.form_enctype = value.clone(),
        NativeWidgetSetter::SetFormMethod(value) => config.form_method = value.clone(),
        NativeWidgetSetter::SetFormTarget(value) => config.form_target = value.clone(),
        NativeWidgetSetter::SetFormNoValidate(value) => config.form_no_validate = *value,
        NativeWidgetSetter::SetWebStyle(value) => config.web_style = value.clone(),
        NativeWidgetSetter::SetPortableStyle(value) => config.portable_style = value.clone(),
        NativeWidgetSetter::SetEvents(value) => config.events = value.clone(),
        NativeWidgetSetter::SetMetadata(value) => config.metadata = value.clone(),
    }
}

pub fn apply_widget_setters(config: &mut NativeWidgetConfig, setters: &[NativeWidgetSetter]) {
    for setter in setters {
        apply_widget_setter(config, setter);
    }
}
