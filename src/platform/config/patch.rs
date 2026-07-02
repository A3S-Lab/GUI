use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::native::NativeRole;
use crate::platform::types::NativeBackendKind;
use crate::style::PortableStyle;

use super::{NativeWidgetConfig, NativeWidgetSetter};

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
    pub read_only: Option<NativeConfigValueChange<bool>>,
    pub multiple: Option<NativeConfigValueChange<bool>>,
    pub auto_focus: Option<NativeConfigValueChange<bool>>,
    pub selected: Option<NativeConfigValueChange<bool>>,
    pub checked: Option<NativeConfigValueChange<Option<bool>>>,
    pub expanded: Option<NativeConfigValueChange<Option<bool>>>,
    pub orientation: Option<NativeConfigValueChange<Option<Orientation>>>,
    pub min: Option<NativeConfigValueChange<Option<f64>>>,
    pub max: Option<NativeConfigValueChange<Option<f64>>>,
    pub current: Option<NativeConfigValueChange<Option<f64>>>,
    pub step: Option<NativeConfigValueChange<Option<f64>>>,
    pub autocomplete: Option<NativeConfigValueChange<Option<String>>>,
    pub input_mode: Option<NativeConfigValueChange<Option<String>>>,
    pub pattern: Option<NativeConfigValueChange<Option<String>>>,
    pub min_length: Option<NativeConfigValueChange<Option<u32>>>,
    pub max_length: Option<NativeConfigValueChange<Option<u32>>>,
    pub rows: Option<NativeConfigValueChange<Option<u32>>>,
    pub cols: Option<NativeConfigValueChange<Option<u32>>>,
    pub size: Option<NativeConfigValueChange<Option<u32>>>,
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
            read_only: diff_value(&before.read_only, &after.read_only),
            multiple: diff_value(&before.multiple, &after.multiple),
            auto_focus: diff_value(&before.auto_focus, &after.auto_focus),
            selected: diff_value(&before.selected, &after.selected),
            checked: diff_value(&before.checked, &after.checked),
            expanded: diff_value(&before.expanded, &after.expanded),
            orientation: diff_value(&before.orientation, &after.orientation),
            min: diff_value(&before.min, &after.min),
            max: diff_value(&before.max, &after.max),
            current: diff_value(&before.current, &after.current),
            step: diff_value(&before.step, &after.step),
            autocomplete: diff_value(&before.autocomplete, &after.autocomplete),
            input_mode: diff_value(&before.input_mode, &after.input_mode),
            pattern: diff_value(&before.pattern, &after.pattern),
            min_length: diff_value(&before.min_length, &after.min_length),
            max_length: diff_value(&before.max_length, &after.max_length),
            rows: diff_value(&before.rows, &after.rows),
            cols: diff_value(&before.cols, &after.cols),
            size: diff_value(&before.size, &after.size),
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
            &self.read_only,
            NativeWidgetSetter::SetReadOnly,
        );
        push_setter(
            &mut setters,
            &self.multiple,
            NativeWidgetSetter::SetMultiple,
        );
        push_setter(
            &mut setters,
            &self.auto_focus,
            NativeWidgetSetter::SetAutoFocus,
        );
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
            &self.autocomplete,
            NativeWidgetSetter::SetAutocomplete,
        );
        push_setter(
            &mut setters,
            &self.input_mode,
            NativeWidgetSetter::SetInputMode,
        );
        push_setter(&mut setters, &self.pattern, NativeWidgetSetter::SetPattern);
        push_setter(
            &mut setters,
            &self.min_length,
            NativeWidgetSetter::SetMinLength,
        );
        push_setter(
            &mut setters,
            &self.max_length,
            NativeWidgetSetter::SetMaxLength,
        );
        push_setter(&mut setters, &self.rows, NativeWidgetSetter::SetRows);
        push_setter(&mut setters, &self.cols, NativeWidgetSetter::SetCols);
        push_setter(&mut setters, &self.size, NativeWidgetSetter::SetSize);
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
