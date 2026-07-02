use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::native::NativeRole;
use crate::style::{DisplayMode, PortableStyle};

use super::types::{NativeBackendKind, NativeWidgetBlueprint};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetConfig {
    pub backend: NativeBackendKind,
    pub widget_class: String,
    pub role: NativeRole,
    pub accessibility_role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub class_name: Option<String>,
    pub placeholder: Option<String>,
    pub enabled: bool,
    pub visible: bool,
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
    pub web_style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

impl NativeWidgetConfig {
    pub fn from_blueprint(blueprint: &NativeWidgetBlueprint) -> Self {
        let state = &blueprint.control_state;
        Self {
            backend: blueprint.backend,
            widget_class: blueprint.widget_class.clone(),
            role: blueprint.role,
            accessibility_role: blueprint.accessibility_role,
            label: blueprint.label.clone(),
            value: blueprint.value.clone(),
            action: blueprint.action.clone(),
            class_name: blueprint.class_name.clone(),
            placeholder: state.placeholder.clone(),
            enabled: !state.disabled,
            visible: blueprint.portable_style.display != Some(DisplayMode::None),
            required: state.required,
            invalid: state.invalid,
            selected: state.selected,
            checked: state.checked,
            expanded: state.expanded,
            orientation: state.orientation,
            min: state.min,
            max: state.max,
            current: state.current,
            step: state.step,
            web_style: blueprint.style.clone(),
            portable_style: blueprint.portable_style.clone(),
            events: blueprint.events.clone(),
            metadata: blueprint.metadata.clone(),
        }
    }

    pub fn diff(&self, after: &Self) -> NativeWidgetConfigPatch {
        NativeWidgetConfigPatch::between(self, after)
    }

    pub fn create_setters(&self) -> Vec<NativeWidgetSetter> {
        vec![
            NativeWidgetSetter::SetAccessibilityRole(self.accessibility_role),
            NativeWidgetSetter::SetLabel(self.label.clone()),
            NativeWidgetSetter::SetValue(self.value.clone()),
            NativeWidgetSetter::SetAction(self.action.clone()),
            NativeWidgetSetter::SetClassName(self.class_name.clone()),
            NativeWidgetSetter::SetPlaceholder(self.placeholder.clone()),
            NativeWidgetSetter::SetEnabled(self.enabled),
            NativeWidgetSetter::SetVisible(self.visible),
            NativeWidgetSetter::SetRequired(self.required),
            NativeWidgetSetter::SetInvalid(self.invalid),
            NativeWidgetSetter::SetSelected(self.selected),
            NativeWidgetSetter::SetChecked(self.checked),
            NativeWidgetSetter::SetExpanded(self.expanded),
            NativeWidgetSetter::SetOrientation(self.orientation),
            NativeWidgetSetter::SetMinimum(self.min),
            NativeWidgetSetter::SetMaximum(self.max),
            NativeWidgetSetter::SetCurrent(self.current),
            NativeWidgetSetter::SetStep(self.step),
            NativeWidgetSetter::SetWebStyle(self.web_style.clone()),
            NativeWidgetSetter::SetPortableStyle(self.portable_style.clone()),
            NativeWidgetSetter::SetEvents(self.events.clone()),
            NativeWidgetSetter::SetMetadata(self.metadata.clone()),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeConfigValueChange<T> {
    pub before: T,
    pub after: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetConfigPatch {
    pub backend: Option<NativeConfigValueChange<NativeBackendKind>>,
    pub widget_class: Option<NativeConfigValueChange<String>>,
    pub role: Option<NativeConfigValueChange<NativeRole>>,
    pub accessibility_role: Option<NativeConfigValueChange<AccessibilityRole>>,
    pub label: Option<NativeConfigValueChange<Option<String>>>,
    pub value: Option<NativeConfigValueChange<Option<String>>>,
    pub action: Option<NativeConfigValueChange<Option<String>>>,
    pub class_name: Option<NativeConfigValueChange<Option<String>>>,
    pub placeholder: Option<NativeConfigValueChange<Option<String>>>,
    pub enabled: Option<NativeConfigValueChange<bool>>,
    pub visible: Option<NativeConfigValueChange<bool>>,
    pub required: Option<NativeConfigValueChange<bool>>,
    pub invalid: Option<NativeConfigValueChange<bool>>,
    pub selected: Option<NativeConfigValueChange<bool>>,
    pub checked: Option<NativeConfigValueChange<Option<bool>>>,
    pub expanded: Option<NativeConfigValueChange<Option<bool>>>,
    pub orientation: Option<NativeConfigValueChange<Option<Orientation>>>,
    pub min: Option<NativeConfigValueChange<Option<f64>>>,
    pub max: Option<NativeConfigValueChange<Option<f64>>>,
    pub current: Option<NativeConfigValueChange<Option<f64>>>,
    pub step: Option<NativeConfigValueChange<Option<f64>>>,
    pub web_style: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
    pub portable_style: Option<NativeConfigValueChange<PortableStyle>>,
    pub events: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
    pub metadata: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
}

impl NativeWidgetConfigPatch {
    pub fn between(before: &NativeWidgetConfig, after: &NativeWidgetConfig) -> Self {
        Self {
            backend: diff_value(&before.backend, &after.backend),
            widget_class: diff_value(&before.widget_class, &after.widget_class),
            role: diff_value(&before.role, &after.role),
            accessibility_role: diff_value(&before.accessibility_role, &after.accessibility_role),
            label: diff_value(&before.label, &after.label),
            value: diff_value(&before.value, &after.value),
            action: diff_value(&before.action, &after.action),
            class_name: diff_value(&before.class_name, &after.class_name),
            placeholder: diff_value(&before.placeholder, &after.placeholder),
            enabled: diff_value(&before.enabled, &after.enabled),
            visible: diff_value(&before.visible, &after.visible),
            required: diff_value(&before.required, &after.required),
            invalid: diff_value(&before.invalid, &after.invalid),
            selected: diff_value(&before.selected, &after.selected),
            checked: diff_value(&before.checked, &after.checked),
            expanded: diff_value(&before.expanded, &after.expanded),
            orientation: diff_value(&before.orientation, &after.orientation),
            min: diff_value(&before.min, &after.min),
            max: diff_value(&before.max, &after.max),
            current: diff_value(&before.current, &after.current),
            step: diff_value(&before.step, &after.step),
            web_style: diff_value(&before.web_style, &after.web_style),
            portable_style: diff_value(&before.portable_style, &after.portable_style),
            events: diff_value(&before.events, &after.events),
            metadata: diff_value(&before.metadata, &after.metadata),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    pub fn setters(&self) -> Vec<NativeWidgetSetter> {
        let mut setters = Vec::new();
        push_setter(
            &mut setters,
            &self.accessibility_role,
            NativeWidgetSetter::SetAccessibilityRole,
        );
        push_setter(&mut setters, &self.label, NativeWidgetSetter::SetLabel);
        push_setter(&mut setters, &self.value, NativeWidgetSetter::SetValue);
        push_setter(&mut setters, &self.action, NativeWidgetSetter::SetAction);
        push_setter(
            &mut setters,
            &self.class_name,
            NativeWidgetSetter::SetClassName,
        );
        push_setter(
            &mut setters,
            &self.placeholder,
            NativeWidgetSetter::SetPlaceholder,
        );
        push_setter(&mut setters, &self.enabled, NativeWidgetSetter::SetEnabled);
        push_setter(&mut setters, &self.visible, NativeWidgetSetter::SetVisible);
        push_setter(
            &mut setters,
            &self.required,
            NativeWidgetSetter::SetRequired,
        );
        push_setter(&mut setters, &self.invalid, NativeWidgetSetter::SetInvalid);
        push_setter(
            &mut setters,
            &self.selected,
            NativeWidgetSetter::SetSelected,
        );
        push_setter(&mut setters, &self.checked, NativeWidgetSetter::SetChecked);
        push_setter(
            &mut setters,
            &self.expanded,
            NativeWidgetSetter::SetExpanded,
        );
        push_setter(
            &mut setters,
            &self.orientation,
            NativeWidgetSetter::SetOrientation,
        );
        push_setter(&mut setters, &self.min, NativeWidgetSetter::SetMinimum);
        push_setter(&mut setters, &self.max, NativeWidgetSetter::SetMaximum);
        push_setter(&mut setters, &self.current, NativeWidgetSetter::SetCurrent);
        push_setter(&mut setters, &self.step, NativeWidgetSetter::SetStep);
        push_setter(
            &mut setters,
            &self.web_style,
            NativeWidgetSetter::SetWebStyle,
        );
        push_setter(
            &mut setters,
            &self.portable_style,
            NativeWidgetSetter::SetPortableStyle,
        );
        push_setter(&mut setters, &self.events, NativeWidgetSetter::SetEvents);
        push_setter(
            &mut setters,
            &self.metadata,
            NativeWidgetSetter::SetMetadata,
        );
        setters
    }
}

fn diff_value<T: Clone + PartialEq>(before: &T, after: &T) -> Option<NativeConfigValueChange<T>> {
    (before != after).then(|| NativeConfigValueChange {
        before: before.clone(),
        after: after.clone(),
    })
}

fn push_setter<T: Clone>(
    setters: &mut Vec<NativeWidgetSetter>,
    change: &Option<NativeConfigValueChange<T>>,
    setter: impl FnOnce(T) -> NativeWidgetSetter,
) {
    if let Some(change) = change {
        setters.push(setter(change.after.clone()));
    }
}

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
    SetSelected(bool),
    SetChecked(Option<bool>),
    SetExpanded(Option<bool>),
    SetOrientation(Option<Orientation>),
    SetMinimum(Option<f64>),
    SetMaximum(Option<f64>),
    SetCurrent(Option<f64>),
    SetStep(Option<f64>),
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
        NativeWidgetSetter::SetSelected(value) => config.selected = *value,
        NativeWidgetSetter::SetChecked(value) => config.checked = *value,
        NativeWidgetSetter::SetExpanded(value) => config.expanded = *value,
        NativeWidgetSetter::SetOrientation(value) => config.orientation = *value,
        NativeWidgetSetter::SetMinimum(value) => config.min = *value,
        NativeWidgetSetter::SetMaximum(value) => config.max = *value,
        NativeWidgetSetter::SetCurrent(value) => config.current = *value,
        NativeWidgetSetter::SetStep(value) => config.step = *value,
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
