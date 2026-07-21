use std::collections::BTreeMap;

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityRole,
    AccessibilityStateProps, AccessibilityStructureProps,
};
use crate::geometry::Orientation;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};
use crate::native::NativeRole;
use crate::style::PortableStyle;

use super::types::{
    NativeBackendKind, NativeTextInputHints, NativeTextInputPurpose, NativeWidgetBlueprint,
    NativeWidgetKind,
};

mod patch;
mod setter;

pub use patch::{
    NativeConfigValueChange, NativeWidgetConfigPatch, NativeWidgetReplacement,
    NativeWidgetSetterBatch,
};
pub use setter::{
    apply_widget_setter, apply_widget_setters, push_widget_setter_history, NativeWidgetSetter,
    DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
};

#[derive(Clone, PartialEq)]
pub struct NativeWidgetConfig {
    pub backend: NativeBackendKind,
    pub widget_kind: NativeWidgetKind,
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
    pub window_resizable: Option<bool>,
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
    pub web_style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

impl std::fmt::Debug for NativeWidgetConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut accessibility_description = self.accessibility_description.clone();
        let has_accessibility_value_text = accessibility_description.value_text.is_some();
        accessibility_description.value_text = None;
        let metadata_keys = self.metadata.keys().collect::<Vec<_>>();
        formatter
            .debug_struct("NativeWidgetConfig")
            .field("backend", &self.backend)
            .field("widget_kind", &self.widget_kind)
            .field("role", &self.role)
            .field("accessibility_role", &self.accessibility_role)
            .field("label", &self.label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("class_name", &self.class_name)
            .field("placeholder", &self.placeholder)
            .field("enabled", &self.enabled)
            .field("visible", &self.visible)
            .field("selected", &self.selected)
            .field("checked", &self.checked)
            .field("expanded", &self.expanded)
            .field("input_type", &self.input_type)
            .field("accessibility_description", &accessibility_description)
            .field(
                "has_accessibility_value_text",
                &has_accessibility_value_text,
            )
            .field("style_declaration_count", &self.web_style.len())
            .field("event_binding_count", &self.events.len())
            .field("metadata_entry_count", &self.metadata.len())
            .field("metadata_keys", &metadata_keys)
            .finish_non_exhaustive()
    }
}

impl NativeWidgetConfig {
    pub fn from_blueprint(blueprint: &NativeWidgetBlueprint) -> Self {
        let state = &blueprint.control_state;
        Self {
            backend: blueprint.backend,
            widget_kind: blueprint.widget_kind,
            role: blueprint.role,
            accessibility_role: blueprint.accessibility_role,
            label: blueprint.label.clone(),
            value: blueprint.value.clone(),
            action: blueprint.action.clone(),
            class_name: blueprint.class_name.clone(),
            placeholder: state.placeholder.clone(),
            enabled: !state.disabled,
            visible: !state.hidden
                && blueprint.portable_style.renders_native_widget()
                && state.html_dialog.open.unwrap_or(true),
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
            enter_key_hint: state.enter_key_hint.clone(),
            auto_capitalize: state.auto_capitalize.clone(),
            auto_correct: state.auto_correct.clone(),
            virtual_keyboard_policy: state.virtual_keyboard_policy.clone(),
            pattern: state.pattern.clone(),
            min_length: state.min_length,
            max_length: state.max_length,
            rows: state.rows,
            cols: state.cols,
            size: state.size,
            title: state.title.clone(),
            window_resizable: window_resizable_from_blueprint(blueprint),
            hidden: state.hidden,
            lang: state.lang.clone(),
            dir: state.dir.clone(),
            tab_index: state.tab_index,
            explicit_role: state.explicit_role.clone(),
            access_key: state.access_key.clone(),
            content_editable: state.content_editable.clone(),
            draggable: state.draggable.clone(),
            spell_check: state.spell_check,
            translate: state.translate,
            inert: state.inert || blueprint.portable_style.makes_native_widget_inert(),
            popover: state.popover.clone(),
            anchor: state.anchor.clone(),
            custom_element_is: state.custom_element_is.clone(),
            nonce: state.nonce.clone(),
            name: state.name.clone(),
            form: state.form.clone(),
            input_type: state.input_type.clone(),
            accept: state.accept.clone(),
            capture: state.capture.clone(),
            alt: state.alt.clone(),
            href: state.href.clone(),
            src: state.src.clone(),
            srcset: state.srcset.clone(),
            sizes: state.sizes.clone(),
            media: state.media.clone(),
            resource_type: state.resource_type.clone(),
            intrinsic_width: state.intrinsic_width,
            intrinsic_height: state.intrinsic_height,
            loading: state.loading.clone(),
            decoding: state.decoding.clone(),
            fetch_priority: state.fetch_priority.clone(),
            cross_origin: state.cross_origin.clone(),
            referrer_policy: state.referrer_policy.clone(),
            poster: state.poster.clone(),
            controls: state.controls,
            autoplay: state.autoplay,
            loop_playback: state.loop_playback,
            muted: state.muted,
            plays_inline: state.plays_inline,
            preload: state.preload.clone(),
            track_kind: state.track_kind.clone(),
            srclang: state.srclang.clone(),
            track_label: state.track_label.clone(),
            default_track: state.default_track,
            list: state.list.clone(),
            dirname: state.dirname.clone(),
            form_action: state.form_action.clone(),
            form_enctype: state.form_enctype.clone(),
            form_method: state.form_method.clone(),
            form_target: state.form_target.clone(),
            form_no_validate: state.form_no_validate,
            html_resource_policy: state.html_resource_policy.clone(),
            html_activation: state.html_activation.clone(),
            html_text_annotation: state.html_text_annotation.clone(),
            html_dialog: state.html_dialog.clone(),
            html_shadow: state.html_shadow.clone(),
            html_microdata: state.html_microdata.clone(),
            html_form_association: state.html_form_association.clone(),
            html_collection: state.html_collection.clone(),
            accessibility_relationships: state.accessibility_relationships.clone(),
            accessibility_description: state.accessibility_description.clone(),
            accessibility_structure: state.accessibility_structure.clone(),
            accessibility_state: state.accessibility_state.clone(),
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
            NativeWidgetSetter::SetEnterKeyHint(self.enter_key_hint.clone()),
            NativeWidgetSetter::SetAutoCapitalize(self.auto_capitalize.clone()),
            NativeWidgetSetter::SetAutoCorrect(self.auto_correct.clone()),
            NativeWidgetSetter::SetVirtualKeyboardPolicy(self.virtual_keyboard_policy.clone()),
            NativeWidgetSetter::SetPattern(self.pattern.clone()),
            NativeWidgetSetter::SetMinLength(self.min_length),
            NativeWidgetSetter::SetMaxLength(self.max_length),
            NativeWidgetSetter::SetRows(self.rows),
            NativeWidgetSetter::SetCols(self.cols),
            NativeWidgetSetter::SetSize(self.size),
            NativeWidgetSetter::SetTitle(self.title.clone()),
            NativeWidgetSetter::SetWindowResizable(self.window_resizable),
            NativeWidgetSetter::SetHidden(self.hidden),
            NativeWidgetSetter::SetLang(self.lang.clone()),
            NativeWidgetSetter::SetDir(self.dir.clone()),
            NativeWidgetSetter::SetTabIndex(self.tab_index),
            NativeWidgetSetter::SetExplicitRole(self.explicit_role.clone()),
            NativeWidgetSetter::SetAccessKey(self.access_key.clone()),
            NativeWidgetSetter::SetContentEditable(self.content_editable.clone()),
            NativeWidgetSetter::SetDraggable(self.draggable.clone()),
            NativeWidgetSetter::SetSpellCheck(self.spell_check),
            NativeWidgetSetter::SetTranslate(self.translate),
            NativeWidgetSetter::SetInert(self.inert),
            NativeWidgetSetter::SetPopover(self.popover.clone()),
            NativeWidgetSetter::SetAnchor(self.anchor.clone()),
            NativeWidgetSetter::SetCustomElementIs(self.custom_element_is.clone()),
            NativeWidgetSetter::SetNonce(self.nonce.clone()),
            NativeWidgetSetter::SetName(self.name.clone()),
            NativeWidgetSetter::SetForm(self.form.clone()),
            NativeWidgetSetter::SetInputType(self.input_type.clone()),
            NativeWidgetSetter::SetAccept(self.accept.clone()),
            NativeWidgetSetter::SetCapture(self.capture.clone()),
            NativeWidgetSetter::SetAlt(self.alt.clone()),
            NativeWidgetSetter::SetHref(self.href.clone()),
            NativeWidgetSetter::SetSrc(self.src.clone()),
            NativeWidgetSetter::SetSrcset(self.srcset.clone()),
            NativeWidgetSetter::SetSizes(self.sizes.clone()),
            NativeWidgetSetter::SetMedia(self.media.clone()),
            NativeWidgetSetter::SetResourceType(self.resource_type.clone()),
            NativeWidgetSetter::SetIntrinsicWidth(self.intrinsic_width),
            NativeWidgetSetter::SetIntrinsicHeight(self.intrinsic_height),
            NativeWidgetSetter::SetLoading(self.loading.clone()),
            NativeWidgetSetter::SetDecoding(self.decoding.clone()),
            NativeWidgetSetter::SetFetchPriority(self.fetch_priority.clone()),
            NativeWidgetSetter::SetCrossOrigin(self.cross_origin.clone()),
            NativeWidgetSetter::SetReferrerPolicy(self.referrer_policy.clone()),
            NativeWidgetSetter::SetPoster(self.poster.clone()),
            NativeWidgetSetter::SetControls(self.controls),
            NativeWidgetSetter::SetAutoplay(self.autoplay),
            NativeWidgetSetter::SetLoopPlayback(self.loop_playback),
            NativeWidgetSetter::SetMuted(self.muted),
            NativeWidgetSetter::SetPlaysInline(self.plays_inline),
            NativeWidgetSetter::SetPreload(self.preload.clone()),
            NativeWidgetSetter::SetTrackKind(self.track_kind.clone()),
            NativeWidgetSetter::SetSrclang(self.srclang.clone()),
            NativeWidgetSetter::SetTrackLabel(self.track_label.clone()),
            NativeWidgetSetter::SetDefaultTrack(self.default_track),
            NativeWidgetSetter::SetList(self.list.clone()),
            NativeWidgetSetter::SetDirname(self.dirname.clone()),
            NativeWidgetSetter::SetFormAction(self.form_action.clone()),
            NativeWidgetSetter::SetFormEnctype(self.form_enctype.clone()),
            NativeWidgetSetter::SetFormMethod(self.form_method.clone()),
            NativeWidgetSetter::SetFormTarget(self.form_target.clone()),
            NativeWidgetSetter::SetFormNoValidate(self.form_no_validate),
            NativeWidgetSetter::SetHtmlResourcePolicy(self.html_resource_policy.clone()),
            NativeWidgetSetter::SetHtmlActivation(self.html_activation.clone()),
            NativeWidgetSetter::SetHtmlTextAnnotation(self.html_text_annotation.clone()),
            NativeWidgetSetter::SetHtmlDialog(self.html_dialog.clone()),
            NativeWidgetSetter::SetHtmlShadow(self.html_shadow.clone()),
            NativeWidgetSetter::SetHtmlMicrodata(self.html_microdata.clone()),
            NativeWidgetSetter::SetHtmlFormAssociation(self.html_form_association.clone()),
            NativeWidgetSetter::SetHtmlCollection(self.html_collection.clone()),
            NativeWidgetSetter::SetAccessibilityRelationships(
                self.accessibility_relationships.clone(),
            ),
            NativeWidgetSetter::SetAccessibilityDescription(self.accessibility_description.clone()),
            NativeWidgetSetter::SetAccessibilityStructure(self.accessibility_structure.clone()),
            NativeWidgetSetter::SetAccessibilityState(self.accessibility_state.clone()),
            NativeWidgetSetter::SetWebStyle(self.web_style.clone()),
            NativeWidgetSetter::SetPortableStyle(self.portable_style.clone()),
            NativeWidgetSetter::SetEvents(self.events.clone()),
            NativeWidgetSetter::SetMetadata(self.metadata.clone()),
        ]
    }

    pub fn text_input_purpose(&self) -> NativeTextInputPurpose {
        let input_type = normalized_token(self.input_type.as_deref());
        let input_mode = normalized_token(self.input_mode.as_deref());

        if input_type.as_deref() == Some("password") {
            return match input_mode.as_deref() {
                Some("numeric" | "decimal") => NativeTextInputPurpose::Pin,
                _ => NativeTextInputPurpose::Password,
            };
        }

        match input_mode.as_deref() {
            Some("numeric") => return NativeTextInputPurpose::Digits,
            Some("decimal") => return NativeTextInputPurpose::Number,
            Some("tel") => return NativeTextInputPurpose::Phone,
            Some("url") => return NativeTextInputPurpose::Url,
            Some("email") => return NativeTextInputPurpose::Email,
            Some("latin") => return NativeTextInputPurpose::Alpha,
            Some("none" | "text" | "search") => return NativeTextInputPurpose::FreeForm,
            Some(_) | None => {}
        }

        match input_type.as_deref() {
            Some("number") => NativeTextInputPurpose::Number,
            Some("tel") => NativeTextInputPurpose::Phone,
            Some("url") => NativeTextInputPurpose::Url,
            Some("email") => NativeTextInputPurpose::Email,
            Some("date" | "datetime-local" | "month" | "time" | "week") => {
                NativeTextInputPurpose::Digits
            }
            _ if is_name_autocomplete(self.autocomplete.as_deref()) => NativeTextInputPurpose::Name,
            _ => NativeTextInputPurpose::FreeForm,
        }
    }

    pub fn text_input_hints(&self) -> NativeTextInputHints {
        let mut hints = NativeTextInputHints {
            spellcheck: self
                .spell_check
                .or_else(|| auto_correct_spellcheck(self.auto_correct.as_deref())),
            word_completion: normalized_token(self.autocomplete.as_deref()).as_deref()
                == Some("on"),
            inhibit_osk: normalized_token(self.virtual_keyboard_policy.as_deref()).as_deref()
                == Some("manual")
                || normalized_token(self.input_mode.as_deref()).as_deref() == Some("none"),
            private: normalized_token(self.input_type.as_deref()).as_deref() == Some("password"),
            ..NativeTextInputHints::default()
        };

        match normalized_token(self.auto_capitalize.as_deref()).as_deref() {
            Some("characters") => hints.uppercase_chars = true,
            Some("words") => hints.uppercase_words = true,
            Some("sentences" | "on") => hints.uppercase_sentences = true,
            Some("lowercase") => hints.lowercase = true,
            _ => {}
        }

        match normalized_token(self.input_mode.as_deref()).as_deref() {
            Some("emoji") => hints.emoji = Some(true),
            Some("none") => hints.emoji = Some(false),
            _ => {}
        }

        hints
    }
}

fn window_resizable_from_blueprint(blueprint: &NativeWidgetBlueprint) -> Option<bool> {
    if blueprint.role != NativeRole::Window {
        return None;
    }

    Some(
        blueprint
            .metadata
            .get("data-a3s-window-resizable")
            .and_then(|value| value.parse::<bool>().ok())
            .unwrap_or(true),
    )
}

fn normalized_token(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
}

fn auto_correct_spellcheck(value: Option<&str>) -> Option<bool> {
    match normalized_token(value).as_deref() {
        Some("on" | "true") => Some(true),
        Some("off" | "false") => Some(false),
        _ => None,
    }
}

fn is_name_autocomplete(value: Option<&str>) -> bool {
    matches!(
        normalized_token(value).as_deref(),
        Some(
            "name"
                | "given-name"
                | "additional-name"
                | "family-name"
                | "nickname"
                | "honorific-prefix"
                | "honorific-suffix"
        )
    )
}
