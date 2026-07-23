use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityRole,
    AccessibilityStateProps, AccessibilityStructureProps,
};
use crate::geometry::Orientation;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};
use crate::native::{NativeProps, NativeRole, ValueSensitivity};
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

/// Backend-independent native primitive selected during platform planning.
///
/// Platform drivers map this typed value to an OS widget. `widget_class` remains
/// on [`NativeWidgetBlueprint`] for diagnostics and legacy protocol consumers;
/// it must not be parsed to decide which widget to create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeWidgetKind {
    Window,
    Container(NativeContainerKind),
    ScrollContainer,
    Label,
    Button,
    TextInput(NativeTextInputKind),
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    ComboBox,
    List,
    ListItem,
    Tree,
    TreeItem,
    Table,
    Dialog,
    Popover,
    Tabs,
    Tab,
    Menu,
    MenuItem,
    Separator,
    Slider,
    Progress,
    Toolbar,
    Image,
    Media,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeContainerKind {
    Linear,
    Grid,
    Canvas,
    Embedded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeTextInputKind {
    SingleLine,
    Search,
    Number,
    Password,
    Multiline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeTextInputPurpose {
    FreeForm,
    Alpha,
    Digits,
    Number,
    Phone,
    Url,
    Email,
    Name,
    Password,
    Pin,
    Terminal,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeTextInputHints {
    pub spellcheck: Option<bool>,
    pub word_completion: bool,
    pub lowercase: bool,
    pub uppercase_chars: bool,
    pub uppercase_words: bool,
    pub uppercase_sentences: bool,
    pub inhibit_osk: bool,
    pub emoji: Option<bool>,
    pub private: bool,
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetBlueprint {
    pub backend: NativeBackendKind,
    pub widget_kind: NativeWidgetKind,
    /// Platform class name retained for diagnostics and legacy wire formats.
    pub widget_class: String,
    pub role: NativeRole,
    pub accessibility_role: AccessibilityRole,
    /// Text rendered by the native widget.
    pub label: Option<String>,
    /// Computed assistive-technology name; legacy payloads may omit it.
    #[serde(default)]
    pub accessibility_label: Option<String>,
    pub value: Option<String>,
    #[serde(default)]
    pub value_sensitivity: ValueSensitivity,
    pub action: Option<String>,
    pub class_name: Option<String>,
    pub control_state: NativeControlState,
    pub style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeWidgetBlueprintWire<'a> {
    backend: NativeBackendKind,
    widget_kind: NativeWidgetKind,
    widget_class: &'a str,
    role: NativeRole,
    accessibility_role: AccessibilityRole,
    label: Option<&'a str>,
    accessibility_label: Option<&'a str>,
    value: Option<&'a str>,
    #[serde(skip_serializing_if = "ValueSensitivity::is_public")]
    value_sensitivity: ValueSensitivity,
    action: Option<&'a str>,
    class_name: Option<&'a str>,
    control_state: &'a NativeControlState,
    style: &'a BTreeMap<String, String>,
    portable_style: &'a PortableStyle,
    events: &'a BTreeMap<String, String>,
    metadata: &'a BTreeMap<String, String>,
}

impl Serialize for NativeWidgetBlueprint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value_sensitivity = self.effective_value_sensitivity();
        let mut control_state = self.control_state.clone();
        if value_sensitivity.is_sensitive() {
            control_state.accessibility_description.value_text = None;
        }
        let mut metadata = self.metadata.clone();
        value_sensitivity.redact_metadata(&mut metadata);
        NativeWidgetBlueprintWire {
            backend: self.backend,
            widget_kind: self.widget_kind,
            widget_class: &self.widget_class,
            role: self.role,
            accessibility_role: self.accessibility_role,
            label: self.label.as_deref(),
            accessibility_label: self.accessibility_label.as_deref(),
            value: value_sensitivity.redact(self.value.as_deref()),
            value_sensitivity,
            action: self.action.as_deref(),
            class_name: self.class_name.as_deref(),
            control_state: &control_state,
            style: &self.style,
            portable_style: &self.portable_style,
            events: &self.events,
            metadata: &metadata,
        }
        .serialize(serializer)
    }
}

impl NativeWidgetBlueprint {
    /// Resolves sensitivity defensively from every supported typed and legacy
    /// input-type channel. This protects manually assembled low-level values,
    /// not only blueprints created by a platform adapter.
    pub fn effective_value_sensitivity(&self) -> ValueSensitivity {
        if self.value_sensitivity.is_sensitive()
            || matches!(
                self.widget_kind,
                NativeWidgetKind::TextInput(NativeTextInputKind::Password)
            )
            || ValueSensitivity::from_input_type(self.control_state.input_type.as_deref())
                .is_sensitive()
            || ValueSensitivity::from_input_type(self.metadata.get("type").map(String::as_str))
                .is_sensitive()
        {
            ValueSensitivity::Sensitive
        } else {
            ValueSensitivity::Public
        }
    }

    pub fn config(&self) -> NativeWidgetConfig {
        NativeWidgetConfig::from_blueprint(self)
    }

    /// Clone a blueprint for retained logs, histories, or failure reports.
    /// Runtime/native state keeps the original value; diagnostics never do.
    pub fn redacted_for_diagnostics(&self) -> Self {
        let mut redacted = self.clone();
        redacted.value_sensitivity = redacted.effective_value_sensitivity();
        if redacted.value_sensitivity.is_sensitive() {
            redacted.value = None;
            redacted.control_state.accessibility_description.value_text = None;
        }
        redacted.control_state.nonce = None;
        redacted.control_state.html_resource_policy = redacted
            .control_state
            .html_resource_policy
            .redacted_for_diagnostics();
        redacted
            .value_sensitivity
            .redact_metadata_for_diagnostics(&mut redacted.metadata);
        redacted
    }
}

impl std::fmt::Debug for NativeWidgetBlueprint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let redacted = self.redacted_for_diagnostics();
        formatter
            .debug_struct("NativeWidgetBlueprint")
            .field("backend", &redacted.backend)
            .field("widget_kind", &redacted.widget_kind)
            .field("widget_class", &redacted.widget_class)
            .field("role", &redacted.role)
            .field("accessibility_role", &redacted.accessibility_role)
            .field("label", &redacted.label)
            .field("accessibility_label", &redacted.accessibility_label)
            .field("value", &redacted.value)
            .field("value_sensitivity", &redacted.value_sensitivity)
            .field("action", &redacted.action)
            .field("class_name", &redacted.class_name)
            .field("control_state", &redacted.control_state)
            .field("style", &redacted.style)
            .field("portable_style", &redacted.portable_style)
            .field("events", &redacted.events)
            .field("metadata", &redacted.metadata)
            .finish()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
    pub enter_key_hint: Option<String>,
    pub auto_capitalize: Option<String>,
    pub auto_correct: Option<String>,
    pub virtual_keyboard_policy: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub size: Option<u32>,
    pub title: Option<String>,
    pub hidden: bool,
    pub lang: Option<String>,
    pub dir: Option<String>,
    pub tab_index: Option<i32>,
    pub explicit_role: Option<String>,
    pub access_key: Option<String>,
    pub content_editable: Option<String>,
    pub draggable: Option<String>,
    pub spell_check: Option<bool>,
    pub translate: Option<bool>,
    pub inert: bool,
    pub popover: Option<String>,
    pub anchor: Option<String>,
    pub custom_element_is: Option<String>,
    pub nonce: Option<String>,
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
    pub html_resource_policy: HtmlResourcePolicyProps,
    pub html_activation: HtmlActivationProps,
    pub html_text_annotation: HtmlTextAnnotationProps,
    pub html_dialog: HtmlDialogProps,
    pub html_shadow: HtmlShadowProps,
    pub html_microdata: HtmlMicrodataProps,
    pub html_form_association: HtmlFormAssociationProps,
    pub html_collection: HtmlCollectionProps,
    pub accessibility_relationships: AccessibilityRelationshipProps,
    pub accessibility_description: AccessibilityDescriptionProps,
    pub accessibility_structure: AccessibilityStructureProps,
    pub accessibility_state: AccessibilityStateProps,
}

impl std::fmt::Debug for NativeControlState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut accessibility_description = self.accessibility_description.clone();
        accessibility_description.value_text = None;
        formatter
            .debug_struct("NativeControlState")
            .field("placeholder", &self.placeholder)
            .field("disabled", &self.disabled)
            .field("required", &self.required)
            .field("invalid", &self.invalid)
            .field("read_only", &self.read_only)
            .field("multiple", &self.multiple)
            .field("auto_focus", &self.auto_focus)
            .field("selected", &self.selected)
            .field("checked", &self.checked)
            .field("expanded", &self.expanded)
            .field("orientation", &self.orientation)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("current", &self.current)
            .field("step", &self.step)
            .field("input_type", &self.input_type)
            .field("hidden", &self.hidden)
            .field("inert", &self.inert)
            .field("accessibility_role", &self.explicit_role)
            .field("accessibility_description", &accessibility_description)
            .finish_non_exhaustive()
    }
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
            enter_key_hint: props.enter_key_hint.clone(),
            auto_capitalize: props.auto_capitalize.clone(),
            auto_correct: props.auto_correct.clone(),
            virtual_keyboard_policy: props.virtual_keyboard_policy.clone(),
            pattern: props.pattern.clone(),
            min_length: props.min_length,
            max_length: props.max_length,
            rows: props.rows,
            cols: props.cols,
            size: props.size,
            title: props.title.clone(),
            hidden: props.hidden,
            lang: props.lang.clone(),
            dir: props.dir.clone(),
            tab_index: props.tab_index,
            explicit_role: props.explicit_role.clone(),
            access_key: props.access_key.clone(),
            content_editable: props.content_editable.clone(),
            draggable: props.draggable.clone(),
            spell_check: props.spell_check,
            translate: props.translate,
            inert: props.inert,
            popover: props.popover.clone(),
            anchor: props.anchor.clone(),
            custom_element_is: props.custom_element_is.clone(),
            nonce: props.nonce.clone(),
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
            html_resource_policy: props.html_resource_policy.clone(),
            html_activation: props.html_activation.clone(),
            html_text_annotation: props.html_text_annotation.clone(),
            html_dialog: props.html_dialog.clone(),
            html_shadow: props.html_shadow.clone(),
            html_microdata: props.html_microdata.clone(),
            html_form_association: props.html_form_association.clone(),
            html_collection: props.html_collection.clone(),
            accessibility_relationships: props.accessibility_relationships.clone(),
            accessibility_description: props.accessibility_description.clone(),
            accessibility_structure: props.accessibility_structure.clone(),
            accessibility_state: props.accessibility_state.clone(),
        }
    }
}
