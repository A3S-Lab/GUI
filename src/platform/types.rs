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
    pub name: Option<String>,
    pub form: Option<String>,
    pub input_type: Option<String>,
    pub accept: Option<String>,
    pub capture: Option<String>,
    pub alt: Option<String>,
    pub href: Option<String>,
    pub src: Option<String>,
    pub srcset: Option<String>,
    pub sizes: Option<String>,
    pub media: Option<String>,
    pub resource_type: Option<String>,
    pub intrinsic_width: Option<u32>,
    pub intrinsic_height: Option<u32>,
    pub loading: Option<String>,
    pub decoding: Option<String>,
    pub fetch_priority: Option<String>,
    pub cross_origin: Option<String>,
    pub referrer_policy: Option<String>,
    pub poster: Option<String>,
    pub controls: bool,
    pub autoplay: bool,
    pub loop_playback: bool,
    pub muted: bool,
    pub plays_inline: bool,
    pub preload: Option<String>,
    pub track_kind: Option<String>,
    pub srclang: Option<String>,
    pub track_label: Option<String>,
    pub default_track: bool,
    pub list: Option<String>,
    pub dirname: Option<String>,
    pub form_action: Option<String>,
    pub form_enctype: Option<String>,
    pub form_method: Option<String>,
    pub form_target: Option<String>,
    pub form_no_validate: bool,
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
            name: props.name.clone(),
            form: props.form.clone(),
            input_type: props.input_type.clone(),
            accept: props.accept.clone(),
            capture: props.capture.clone(),
            alt: props.alt.clone(),
            href: props.href.clone(),
            src: props.src.clone(),
            srcset: props.srcset.clone(),
            sizes: props.sizes.clone(),
            media: props.media.clone(),
            resource_type: props.resource_type.clone(),
            intrinsic_width: props.intrinsic_width,
            intrinsic_height: props.intrinsic_height,
            loading: props.loading.clone(),
            decoding: props.decoding.clone(),
            fetch_priority: props.fetch_priority.clone(),
            cross_origin: props.cross_origin.clone(),
            referrer_policy: props.referrer_policy.clone(),
            poster: props.poster.clone(),
            controls: props.controls,
            autoplay: props.autoplay,
            loop_playback: props.loop_playback,
            muted: props.muted,
            plays_inline: props.plays_inline,
            preload: props.preload.clone(),
            track_kind: props.track_kind.clone(),
            srclang: props.srclang.clone(),
            track_label: props.track_label.clone(),
            default_track: props.default_track,
            list: props.list.clone(),
            dirname: props.dirname.clone(),
            form_action: props.form_action.clone(),
            form_enctype: props.form_enctype.clone(),
            form_method: props.form_method.clone(),
            form_target: props.form_target.clone(),
            form_no_validate: props.form_no_validate,
        }
    }
}
