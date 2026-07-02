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
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
    pub step: Option<f64>,
}

impl NativeControlState {
    pub fn from_props(props: &NativeProps) -> Self {
        Self {
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            required: props.required,
            invalid: props.invalid,
            selected: props.selected,
            checked: props.checked,
            expanded: props.expanded,
            orientation: props.orientation,
            min: props.min,
            max: props.max,
            current: props.current,
            step: props.step,
        }
    }
}
