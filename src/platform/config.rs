use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::AccessibilityRole;
use crate::geometry::Orientation;
use crate::native::NativeRole;
use crate::style::{DisplayMode, PortableStyle};

use super::types::{NativeBackendKind, NativeWidgetBlueprint};

mod patch;
mod setter;

pub use patch::{NativeConfigValueChange, NativeWidgetConfigPatch};
pub use setter::{apply_widget_setter, apply_widget_setters, NativeWidgetSetter};

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
    pub name: Option<String>,
    pub form: Option<String>,
    pub input_type: Option<String>,
    pub accept: Option<String>,
    pub capture: Option<String>,
    pub alt: Option<String>,
    pub src: Option<String>,
    pub list: Option<String>,
    pub dirname: Option<String>,
    pub form_action: Option<String>,
    pub form_enctype: Option<String>,
    pub form_method: Option<String>,
    pub form_target: Option<String>,
    pub form_no_validate: bool,
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
            read_only: state.read_only,
            multiple: state.multiple,
            auto_focus: state.auto_focus,
            selected: state.selected,
            checked: state.checked,
            expanded: state.expanded,
            orientation: state.orientation,
            min: state.min,
            max: state.max,
            current: state.current,
            step: state.step,
            autocomplete: state.autocomplete.clone(),
            input_mode: state.input_mode.clone(),
            pattern: state.pattern.clone(),
            min_length: state.min_length,
            max_length: state.max_length,
            rows: state.rows,
            cols: state.cols,
            size: state.size,
            name: state.name.clone(),
            form: state.form.clone(),
            input_type: state.input_type.clone(),
            accept: state.accept.clone(),
            capture: state.capture.clone(),
            alt: state.alt.clone(),
            src: state.src.clone(),
            list: state.list.clone(),
            dirname: state.dirname.clone(),
            form_action: state.form_action.clone(),
            form_enctype: state.form_enctype.clone(),
            form_method: state.form_method.clone(),
            form_target: state.form_target.clone(),
            form_no_validate: state.form_no_validate,
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
            NativeWidgetSetter::SetReadOnly(self.read_only),
            NativeWidgetSetter::SetMultiple(self.multiple),
            NativeWidgetSetter::SetAutoFocus(self.auto_focus),
            NativeWidgetSetter::SetSelected(self.selected),
            NativeWidgetSetter::SetChecked(self.checked),
            NativeWidgetSetter::SetExpanded(self.expanded),
            NativeWidgetSetter::SetOrientation(self.orientation),
            NativeWidgetSetter::SetMinimum(self.min),
            NativeWidgetSetter::SetMaximum(self.max),
            NativeWidgetSetter::SetCurrent(self.current),
            NativeWidgetSetter::SetStep(self.step),
            NativeWidgetSetter::SetAutocomplete(self.autocomplete.clone()),
            NativeWidgetSetter::SetInputMode(self.input_mode.clone()),
            NativeWidgetSetter::SetPattern(self.pattern.clone()),
            NativeWidgetSetter::SetMinLength(self.min_length),
            NativeWidgetSetter::SetMaxLength(self.max_length),
            NativeWidgetSetter::SetRows(self.rows),
            NativeWidgetSetter::SetCols(self.cols),
            NativeWidgetSetter::SetSize(self.size),
            NativeWidgetSetter::SetName(self.name.clone()),
            NativeWidgetSetter::SetForm(self.form.clone()),
            NativeWidgetSetter::SetInputType(self.input_type.clone()),
            NativeWidgetSetter::SetAccept(self.accept.clone()),
            NativeWidgetSetter::SetCapture(self.capture.clone()),
            NativeWidgetSetter::SetAlt(self.alt.clone()),
            NativeWidgetSetter::SetSrc(self.src.clone()),
            NativeWidgetSetter::SetList(self.list.clone()),
            NativeWidgetSetter::SetDirname(self.dirname.clone()),
            NativeWidgetSetter::SetFormAction(self.form_action.clone()),
            NativeWidgetSetter::SetFormEnctype(self.form_enctype.clone()),
            NativeWidgetSetter::SetFormMethod(self.form_method.clone()),
            NativeWidgetSetter::SetFormTarget(self.form_target.clone()),
            NativeWidgetSetter::SetFormNoValidate(self.form_no_validate),
            NativeWidgetSetter::SetWebStyle(self.web_style.clone()),
            NativeWidgetSetter::SetPortableStyle(self.portable_style.clone()),
            NativeWidgetSetter::SetEvents(self.events.clone()),
            NativeWidgetSetter::SetMetadata(self.metadata.clone()),
        ]
    }
}
