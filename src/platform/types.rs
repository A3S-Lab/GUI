use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::native::{NativeProps, NativeRole};
use crate::style::PortableStyle;

use super::config::NativeWidgetConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeBackendKind {
    AppKit,
    WinUI,
    Gtk4,
    Headless,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetBlueprint {
    pub backend: NativeBackendKind,
    pub widget_class: String,
    pub role: NativeRole,
    pub accessibility_role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub class_name: Option<String>,
    pub control_state: NativeControlState,
    pub style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

impl NativeWidgetBlueprint {
    pub fn config(&self) -> NativeWidgetConfig {
        NativeWidgetConfig::from_blueprint(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeControlState {
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    pub read_only: bool,
    pub multiple: bool,
    pub auto_focus: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
    pub step: Option<f64>,
    pub autocomplete: Option<String>,
    pub input_mode: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub size: Option<u32>,
}

impl NativeControlState {
    pub fn from_props(props: &NativeProps) -> Self {
        Self {
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            required: props.required,
            invalid: props.invalid,
            read_only: props.read_only,
            multiple: props.multiple,
            auto_focus: props.auto_focus,
            selected: props.selected,
            checked: props.checked,
            expanded: props.expanded,
            orientation: props.orientation,
            min: props.min,
            max: props.max,
            current: props.current,
            step: props.step,
            autocomplete: props.autocomplete.clone(),
            input_mode: props.input_mode.clone(),
            pattern: props.pattern.clone(),
            min_length: props.min_length,
            max_length: props.max_length,
            rows: props.rows,
            cols: props.cols,
            size: props.size,
        }
    }
}
