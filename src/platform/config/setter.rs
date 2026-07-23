use std::collections::BTreeMap;
use std::fmt;

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityRole,
    AccessibilityStateProps, AccessibilityStructureProps,
};
use crate::geometry::Orientation;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};
use crate::native::ValueSensitivity;
use crate::style::PortableStyle;

use super::NativeWidgetConfig;

pub const DEFAULT_NATIVE_SETTER_HISTORY_LIMIT: usize = 256;

#[derive(Clone, PartialEq)]
pub enum NativeWidgetSetter {
    SetAccessibilityRole(AccessibilityRole),
    SetLabel(Option<String>),
    SetAccessibilityLabel(Option<String>),
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
    SetEnterKeyHint(Option<String>),
    SetAutoCapitalize(Option<String>),
    SetAutoCorrect(Option<String>),
    SetVirtualKeyboardPolicy(Option<String>),
    SetPattern(Option<String>),
    SetMinLength(Option<u32>),
    SetMaxLength(Option<u32>),
    SetRows(Option<u32>),
    SetCols(Option<u32>),
    SetSize(Option<u32>),
    SetTitle(Option<String>),
    SetWindowResizable(Option<bool>),
    SetHidden(bool),
    SetLang(Option<String>),
    SetDir(Option<String>),
    SetTabIndex(Option<i32>),
    SetExplicitRole(Option<String>),
    SetAccessKey(Option<String>),
    SetContentEditable(Option<String>),
    SetDraggable(Option<String>),
    SetSpellCheck(Option<bool>),
    SetTranslate(Option<bool>),
    SetInert(bool),
    SetPopover(Option<String>),
    SetAnchor(Option<String>),
    SetCustomElementIs(Option<String>),
    SetNonce(Option<String>),
    SetName(Option<String>),
    SetForm(Option<String>),
    SetInputType(Option<String>),
    SetAccept(Option<String>),
    SetCapture(Option<String>),
    SetAlt(Option<String>),
    SetHref(Option<String>),
    SetSrc(Option<String>),
    SetSrcset(Option<String>),
    SetSizes(Option<String>),
    SetMedia(Option<String>),
    SetResourceType(Option<String>),
    SetIntrinsicWidth(Option<u32>),
    SetIntrinsicHeight(Option<u32>),
    SetLoading(Option<String>),
    SetDecoding(Option<String>),
    SetFetchPriority(Option<String>),
    SetCrossOrigin(Option<String>),
    SetReferrerPolicy(Option<String>),
    SetPoster(Option<String>),
    SetControls(bool),
    SetAutoplay(bool),
    SetLoopPlayback(bool),
    SetMuted(bool),
    SetPlaysInline(bool),
    SetPreload(Option<String>),
    SetTrackKind(Option<String>),
    SetSrclang(Option<String>),
    SetTrackLabel(Option<String>),
    SetDefaultTrack(bool),
    SetList(Option<String>),
    SetDirname(Option<String>),
    SetFormAction(Option<String>),
    SetFormEnctype(Option<String>),
    SetFormMethod(Option<String>),
    SetFormTarget(Option<String>),
    SetFormNoValidate(bool),
    SetHtmlResourcePolicy(HtmlResourcePolicyProps),
    SetHtmlActivation(HtmlActivationProps),
    SetHtmlTextAnnotation(HtmlTextAnnotationProps),
    SetHtmlDialog(HtmlDialogProps),
    SetHtmlShadow(HtmlShadowProps),
    SetHtmlMicrodata(HtmlMicrodataProps),
    SetHtmlFormAssociation(HtmlFormAssociationProps),
    SetHtmlCollection(HtmlCollectionProps),
    SetAccessibilityRelationships(AccessibilityRelationshipProps),
    SetAccessibilityDescription(AccessibilityDescriptionProps),
    SetAccessibilityStructure(AccessibilityStructureProps),
    SetAccessibilityState(AccessibilityStateProps),
    SetWebStyle(BTreeMap<String, String>),
    // Keep the setter enum compact as the complete portable style schema grows.
    SetPortableStyle(Box<PortableStyle>),
    SetEvents(BTreeMap<String, String>),
    SetMetadata(BTreeMap<String, String>),
}

impl fmt::Debug for NativeWidgetSetter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! debug_payload_variants {
            ($($variant:ident),+ $(,)?) => {
                match self {
                    Self::SetValue(value) => formatter
                        .debug_struct("SetValue")
                        .field("has_value", &value.is_some())
                        .finish(),
                    Self::SetNonce(value) => formatter
                        .debug_struct("SetNonce")
                        .field("has_value", &value.is_some())
                        .finish(),
                    Self::SetAccessibilityDescription(description) => {
                        let mut redacted = description.clone();
                        let has_value_text = redacted.value_text.is_some();
                        redacted.value_text = None;
                        formatter
                            .debug_struct("SetAccessibilityDescription")
                            .field("description", &redacted)
                            .field("has_value_text", &has_value_text)
                            .finish()
                    }
                    Self::SetMetadata(metadata) => {
                        let keys = metadata.keys().collect::<Vec<_>>();
                        formatter
                            .debug_struct("SetMetadata")
                            .field("entry_count", &metadata.len())
                            .field("keys", &keys)
                            .finish()
                    }
                    $(Self::$variant(value) => formatter
                        .debug_tuple(stringify!($variant))
                        .field(value)
                        .finish(),)+
                }
            };
        }

        debug_payload_variants!(
            SetAccessibilityRole,
            SetLabel,
            SetAccessibilityLabel,
            SetAction,
            SetClassName,
            SetPlaceholder,
            SetEnabled,
            SetVisible,
            SetRequired,
            SetInvalid,
            SetReadOnly,
            SetMultiple,
            SetAutoFocus,
            SetSelected,
            SetChecked,
            SetExpanded,
            SetOrientation,
            SetMinimum,
            SetMaximum,
            SetCurrent,
            SetStep,
            SetAutocomplete,
            SetInputMode,
            SetEnterKeyHint,
            SetAutoCapitalize,
            SetAutoCorrect,
            SetVirtualKeyboardPolicy,
            SetPattern,
            SetMinLength,
            SetMaxLength,
            SetRows,
            SetCols,
            SetSize,
            SetTitle,
            SetWindowResizable,
            SetHidden,
            SetLang,
            SetDir,
            SetTabIndex,
            SetExplicitRole,
            SetAccessKey,
            SetContentEditable,
            SetDraggable,
            SetSpellCheck,
            SetTranslate,
            SetInert,
            SetPopover,
            SetAnchor,
            SetCustomElementIs,
            SetName,
            SetForm,
            SetInputType,
            SetAccept,
            SetCapture,
            SetAlt,
            SetHref,
            SetSrc,
            SetSrcset,
            SetSizes,
            SetMedia,
            SetResourceType,
            SetIntrinsicWidth,
            SetIntrinsicHeight,
            SetLoading,
            SetDecoding,
            SetFetchPriority,
            SetCrossOrigin,
            SetReferrerPolicy,
            SetPoster,
            SetControls,
            SetAutoplay,
            SetLoopPlayback,
            SetMuted,
            SetPlaysInline,
            SetPreload,
            SetTrackKind,
            SetSrclang,
            SetTrackLabel,
            SetDefaultTrack,
            SetList,
            SetDirname,
            SetFormAction,
            SetFormEnctype,
            SetFormMethod,
            SetFormTarget,
            SetFormNoValidate,
            SetHtmlResourcePolicy,
            SetHtmlActivation,
            SetHtmlTextAnnotation,
            SetHtmlDialog,
            SetHtmlShadow,
            SetHtmlMicrodata,
            SetHtmlFormAssociation,
            SetHtmlCollection,
            SetAccessibilityRelationships,
            SetAccessibilityStructure,
            SetAccessibilityState,
            SetWebStyle,
            SetPortableStyle,
            SetEvents,
        )
    }
}

impl NativeWidgetSetter {
    pub fn redacted_for_diagnostics(&self, sensitivity: ValueSensitivity) -> Self {
        match self {
            Self::SetValue(_) if sensitivity.is_sensitive() => Self::SetValue(None),
            Self::SetNonce(_) => Self::SetNonce(None),
            Self::SetAccessibilityDescription(description) if sensitivity.is_sensitive() => {
                let mut description = description.clone();
                description.value_text = None;
                Self::SetAccessibilityDescription(description)
            }
            Self::SetHtmlResourcePolicy(policy) => {
                Self::SetHtmlResourcePolicy(policy.redacted_for_diagnostics())
            }
            Self::SetMetadata(metadata) => {
                let mut metadata = metadata.clone();
                sensitivity.redact_metadata_for_diagnostics(&mut metadata);
                Self::SetMetadata(metadata)
            }
            _ => self.clone(),
        }
    }

    /// Compare only the config field owned by this setter.
    ///
    /// This keeps diff construction linear in the number of setters and avoids
    /// cloning the full config (including style and metadata maps) per field.
    pub(crate) fn differs_from(&self, config: &NativeWidgetConfig) -> bool {
        match self {
            Self::SetAccessibilityRole(value) => &config.accessibility_role != value,
            Self::SetLabel(value) => &config.label != value,
            Self::SetAccessibilityLabel(value) => &config.accessibility_label != value,
            Self::SetValue(value) => &config.value != value,
            Self::SetAction(value) => &config.action != value,
            Self::SetClassName(value) => &config.class_name != value,
            Self::SetPlaceholder(value) => &config.placeholder != value,
            Self::SetEnabled(value) => &config.enabled != value,
            Self::SetVisible(value) => &config.visible != value,
            Self::SetRequired(value) => &config.required != value,
            Self::SetInvalid(value) => &config.invalid != value,
            Self::SetReadOnly(value) => &config.read_only != value,
            Self::SetMultiple(value) => &config.multiple != value,
            Self::SetAutoFocus(value) => &config.auto_focus != value,
            Self::SetSelected(value) => &config.selected != value,
            Self::SetChecked(value) => &config.checked != value,
            Self::SetExpanded(value) => &config.expanded != value,
            Self::SetOrientation(value) => &config.orientation != value,
            Self::SetMinimum(value) => &config.min != value,
            Self::SetMaximum(value) => &config.max != value,
            Self::SetCurrent(value) => &config.current != value,
            Self::SetStep(value) => &config.step != value,
            Self::SetAutocomplete(value) => &config.autocomplete != value,
            Self::SetInputMode(value) => &config.input_mode != value,
            Self::SetEnterKeyHint(value) => &config.enter_key_hint != value,
            Self::SetAutoCapitalize(value) => &config.auto_capitalize != value,
            Self::SetAutoCorrect(value) => &config.auto_correct != value,
            Self::SetVirtualKeyboardPolicy(value) => &config.virtual_keyboard_policy != value,
            Self::SetPattern(value) => &config.pattern != value,
            Self::SetMinLength(value) => &config.min_length != value,
            Self::SetMaxLength(value) => &config.max_length != value,
            Self::SetRows(value) => &config.rows != value,
            Self::SetCols(value) => &config.cols != value,
            Self::SetSize(value) => &config.size != value,
            Self::SetTitle(value) => &config.title != value,
            Self::SetWindowResizable(value) => &config.window_resizable != value,
            Self::SetHidden(value) => &config.hidden != value,
            Self::SetLang(value) => &config.lang != value,
            Self::SetDir(value) => &config.dir != value,
            Self::SetTabIndex(value) => &config.tab_index != value,
            Self::SetExplicitRole(value) => &config.explicit_role != value,
            Self::SetAccessKey(value) => &config.access_key != value,
            Self::SetContentEditable(value) => &config.content_editable != value,
            Self::SetDraggable(value) => &config.draggable != value,
            Self::SetSpellCheck(value) => &config.spell_check != value,
            Self::SetTranslate(value) => &config.translate != value,
            Self::SetInert(value) => &config.inert != value,
            Self::SetPopover(value) => &config.popover != value,
            Self::SetAnchor(value) => &config.anchor != value,
            Self::SetCustomElementIs(value) => &config.custom_element_is != value,
            Self::SetNonce(value) => &config.nonce != value,
            Self::SetName(value) => &config.name != value,
            Self::SetForm(value) => &config.form != value,
            Self::SetInputType(value) => &config.input_type != value,
            Self::SetAccept(value) => &config.accept != value,
            Self::SetCapture(value) => &config.capture != value,
            Self::SetAlt(value) => &config.alt != value,
            Self::SetHref(value) => &config.href != value,
            Self::SetSrc(value) => &config.src != value,
            Self::SetSrcset(value) => &config.srcset != value,
            Self::SetSizes(value) => &config.sizes != value,
            Self::SetMedia(value) => &config.media != value,
            Self::SetResourceType(value) => &config.resource_type != value,
            Self::SetIntrinsicWidth(value) => &config.intrinsic_width != value,
            Self::SetIntrinsicHeight(value) => &config.intrinsic_height != value,
            Self::SetLoading(value) => &config.loading != value,
            Self::SetDecoding(value) => &config.decoding != value,
            Self::SetFetchPriority(value) => &config.fetch_priority != value,
            Self::SetCrossOrigin(value) => &config.cross_origin != value,
            Self::SetReferrerPolicy(value) => &config.referrer_policy != value,
            Self::SetPoster(value) => &config.poster != value,
            Self::SetControls(value) => &config.controls != value,
            Self::SetAutoplay(value) => &config.autoplay != value,
            Self::SetLoopPlayback(value) => &config.loop_playback != value,
            Self::SetMuted(value) => &config.muted != value,
            Self::SetPlaysInline(value) => &config.plays_inline != value,
            Self::SetPreload(value) => &config.preload != value,
            Self::SetTrackKind(value) => &config.track_kind != value,
            Self::SetSrclang(value) => &config.srclang != value,
            Self::SetTrackLabel(value) => &config.track_label != value,
            Self::SetDefaultTrack(value) => &config.default_track != value,
            Self::SetList(value) => &config.list != value,
            Self::SetDirname(value) => &config.dirname != value,
            Self::SetFormAction(value) => &config.form_action != value,
            Self::SetFormEnctype(value) => &config.form_enctype != value,
            Self::SetFormMethod(value) => &config.form_method != value,
            Self::SetFormTarget(value) => &config.form_target != value,
            Self::SetFormNoValidate(value) => &config.form_no_validate != value,
            Self::SetHtmlResourcePolicy(value) => &config.html_resource_policy != value,
            Self::SetHtmlActivation(value) => &config.html_activation != value,
            Self::SetHtmlTextAnnotation(value) => &config.html_text_annotation != value,
            Self::SetHtmlDialog(value) => &config.html_dialog != value,
            Self::SetHtmlShadow(value) => &config.html_shadow != value,
            Self::SetHtmlMicrodata(value) => &config.html_microdata != value,
            Self::SetHtmlFormAssociation(value) => &config.html_form_association != value,
            Self::SetHtmlCollection(value) => &config.html_collection != value,
            Self::SetAccessibilityRelationships(value) => {
                &config.accessibility_relationships != value
            }
            Self::SetAccessibilityDescription(value) => &config.accessibility_description != value,
            Self::SetAccessibilityStructure(value) => &config.accessibility_structure != value,
            Self::SetAccessibilityState(value) => &config.accessibility_state != value,
            Self::SetWebStyle(value) => &config.web_style != value,
            Self::SetPortableStyle(value) => config.portable_style != **value,
            Self::SetEvents(value) => &config.events != value,
            Self::SetMetadata(value) => &config.metadata != value,
        }
    }
}

pub fn push_widget_setter_history(
    history: &mut Vec<NativeWidgetSetter>,
    setters: &[NativeWidgetSetter],
    sensitivity: ValueSensitivity,
    limit: usize,
) {
    if limit == 0 {
        history.clear();
        return;
    }
    history.extend(
        setters
            .iter()
            .map(|setter| setter.redacted_for_diagnostics(sensitivity)),
    );
    let excess = history.len().saturating_sub(limit);
    if excess > 0 {
        history.drain(..excess);
    }
}

pub fn apply_widget_setter(config: &mut NativeWidgetConfig, setter: &NativeWidgetSetter) {
    match setter {
        NativeWidgetSetter::SetAccessibilityRole(value) => config.accessibility_role = *value,
        NativeWidgetSetter::SetLabel(value) => config.label = value.clone(),
        NativeWidgetSetter::SetAccessibilityLabel(value) => {
            config.accessibility_label = value.clone();
        }
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
        NativeWidgetSetter::SetEnterKeyHint(value) => config.enter_key_hint = value.clone(),
        NativeWidgetSetter::SetAutoCapitalize(value) => config.auto_capitalize = value.clone(),
        NativeWidgetSetter::SetAutoCorrect(value) => config.auto_correct = value.clone(),
        NativeWidgetSetter::SetVirtualKeyboardPolicy(value) => {
            config.virtual_keyboard_policy = value.clone();
        }
        NativeWidgetSetter::SetPattern(value) => config.pattern = value.clone(),
        NativeWidgetSetter::SetMinLength(value) => config.min_length = *value,
        NativeWidgetSetter::SetMaxLength(value) => config.max_length = *value,
        NativeWidgetSetter::SetRows(value) => config.rows = *value,
        NativeWidgetSetter::SetCols(value) => config.cols = *value,
        NativeWidgetSetter::SetSize(value) => config.size = *value,
        NativeWidgetSetter::SetTitle(value) => config.title = value.clone(),
        NativeWidgetSetter::SetWindowResizable(value) => config.window_resizable = *value,
        NativeWidgetSetter::SetHidden(value) => config.hidden = *value,
        NativeWidgetSetter::SetLang(value) => config.lang = value.clone(),
        NativeWidgetSetter::SetDir(value) => config.dir = value.clone(),
        NativeWidgetSetter::SetTabIndex(value) => config.tab_index = *value,
        NativeWidgetSetter::SetExplicitRole(value) => config.explicit_role = value.clone(),
        NativeWidgetSetter::SetAccessKey(value) => config.access_key = value.clone(),
        NativeWidgetSetter::SetContentEditable(value) => config.content_editable = value.clone(),
        NativeWidgetSetter::SetDraggable(value) => config.draggable = value.clone(),
        NativeWidgetSetter::SetSpellCheck(value) => config.spell_check = *value,
        NativeWidgetSetter::SetTranslate(value) => config.translate = *value,
        NativeWidgetSetter::SetInert(value) => config.inert = *value,
        NativeWidgetSetter::SetPopover(value) => config.popover = value.clone(),
        NativeWidgetSetter::SetAnchor(value) => config.anchor = value.clone(),
        NativeWidgetSetter::SetCustomElementIs(value) => config.custom_element_is = value.clone(),
        NativeWidgetSetter::SetNonce(value) => config.nonce = value.clone(),
        NativeWidgetSetter::SetName(value) => config.name = value.clone(),
        NativeWidgetSetter::SetForm(value) => config.form = value.clone(),
        NativeWidgetSetter::SetInputType(value) => config.input_type = value.clone(),
        NativeWidgetSetter::SetAccept(value) => config.accept = value.clone(),
        NativeWidgetSetter::SetCapture(value) => config.capture = value.clone(),
        NativeWidgetSetter::SetAlt(value) => config.alt = value.clone(),
        NativeWidgetSetter::SetHref(value) => config.href = value.clone(),
        NativeWidgetSetter::SetSrc(value) => config.src = value.clone(),
        NativeWidgetSetter::SetSrcset(value) => config.srcset = value.clone(),
        NativeWidgetSetter::SetSizes(value) => config.sizes = value.clone(),
        NativeWidgetSetter::SetMedia(value) => config.media = value.clone(),
        NativeWidgetSetter::SetResourceType(value) => config.resource_type = value.clone(),
        NativeWidgetSetter::SetIntrinsicWidth(value) => config.intrinsic_width = *value,
        NativeWidgetSetter::SetIntrinsicHeight(value) => config.intrinsic_height = *value,
        NativeWidgetSetter::SetLoading(value) => config.loading = value.clone(),
        NativeWidgetSetter::SetDecoding(value) => config.decoding = value.clone(),
        NativeWidgetSetter::SetFetchPriority(value) => config.fetch_priority = value.clone(),
        NativeWidgetSetter::SetCrossOrigin(value) => config.cross_origin = value.clone(),
        NativeWidgetSetter::SetReferrerPolicy(value) => config.referrer_policy = value.clone(),
        NativeWidgetSetter::SetPoster(value) => config.poster = value.clone(),
        NativeWidgetSetter::SetControls(value) => config.controls = *value,
        NativeWidgetSetter::SetAutoplay(value) => config.autoplay = *value,
        NativeWidgetSetter::SetLoopPlayback(value) => config.loop_playback = *value,
        NativeWidgetSetter::SetMuted(value) => config.muted = *value,
        NativeWidgetSetter::SetPlaysInline(value) => config.plays_inline = *value,
        NativeWidgetSetter::SetPreload(value) => config.preload = value.clone(),
        NativeWidgetSetter::SetTrackKind(value) => config.track_kind = value.clone(),
        NativeWidgetSetter::SetSrclang(value) => config.srclang = value.clone(),
        NativeWidgetSetter::SetTrackLabel(value) => config.track_label = value.clone(),
        NativeWidgetSetter::SetDefaultTrack(value) => config.default_track = *value,
        NativeWidgetSetter::SetList(value) => config.list = value.clone(),
        NativeWidgetSetter::SetDirname(value) => config.dirname = value.clone(),
        NativeWidgetSetter::SetFormAction(value) => config.form_action = value.clone(),
        NativeWidgetSetter::SetFormEnctype(value) => config.form_enctype = value.clone(),
        NativeWidgetSetter::SetFormMethod(value) => config.form_method = value.clone(),
        NativeWidgetSetter::SetFormTarget(value) => config.form_target = value.clone(),
        NativeWidgetSetter::SetFormNoValidate(value) => config.form_no_validate = *value,
        NativeWidgetSetter::SetHtmlResourcePolicy(value) => {
            config.html_resource_policy = value.clone();
        }
        NativeWidgetSetter::SetHtmlActivation(value) => {
            config.html_activation = value.clone();
        }
        NativeWidgetSetter::SetHtmlTextAnnotation(value) => {
            config.html_text_annotation = value.clone();
        }
        NativeWidgetSetter::SetHtmlDialog(value) => {
            config.html_dialog = value.clone();
        }
        NativeWidgetSetter::SetHtmlShadow(value) => {
            config.html_shadow = value.clone();
        }
        NativeWidgetSetter::SetHtmlMicrodata(value) => {
            config.html_microdata = value.clone();
        }
        NativeWidgetSetter::SetHtmlFormAssociation(value) => {
            config.html_form_association = value.clone();
        }
        NativeWidgetSetter::SetHtmlCollection(value) => {
            config.html_collection = value.clone();
        }
        NativeWidgetSetter::SetAccessibilityRelationships(value) => {
            config.accessibility_relationships = value.clone();
        }
        NativeWidgetSetter::SetAccessibilityDescription(value) => {
            config.accessibility_description = value.clone();
        }
        NativeWidgetSetter::SetAccessibilityStructure(value) => {
            config.accessibility_structure = value.clone();
        }
        NativeWidgetSetter::SetAccessibilityState(value) => {
            config.accessibility_state = value.clone();
        }
        NativeWidgetSetter::SetWebStyle(value) => config.web_style = value.clone(),
        NativeWidgetSetter::SetPortableStyle(value) => config.portable_style = (**value).clone(),
        NativeWidgetSetter::SetEvents(value) => config.events = value.clone(),
        NativeWidgetSetter::SetMetadata(value) => config.metadata = value.clone(),
    }
}

pub fn apply_widget_setters(config: &mut NativeWidgetConfig, setters: &[NativeWidgetSetter]) {
    for setter in setters {
        apply_widget_setter(config, setter);
    }
}

#[cfg(test)]
mod tests {
    use super::NativeWidgetSetter;

    #[test]
    fn native_widget_setter_stays_compact() {
        let size = std::mem::size_of::<NativeWidgetSetter>();
        assert!(
            size <= 1024,
            "NativeWidgetSetter grew to {size} bytes and can exhaust small UI-thread stacks"
        );
    }
}
