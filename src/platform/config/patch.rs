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
    pub enter_key_hint: Option<NativeConfigValueChange<Option<String>>>,
    pub auto_capitalize: Option<NativeConfigValueChange<Option<String>>>,
    pub auto_correct: Option<NativeConfigValueChange<Option<String>>>,
    pub virtual_keyboard_policy: Option<NativeConfigValueChange<Option<String>>>,
    pub pattern: Option<NativeConfigValueChange<Option<String>>>,
    pub min_length: Option<NativeConfigValueChange<Option<u32>>>,
    pub max_length: Option<NativeConfigValueChange<Option<u32>>>,
    pub rows: Option<NativeConfigValueChange<Option<u32>>>,
    pub cols: Option<NativeConfigValueChange<Option<u32>>>,
    pub size: Option<NativeConfigValueChange<Option<u32>>>,
    pub title: Option<NativeConfigValueChange<Option<String>>>,
    pub window_resizable: Option<NativeConfigValueChange<Option<bool>>>,
    pub hidden: Option<NativeConfigValueChange<bool>>,
    pub lang: Option<NativeConfigValueChange<Option<String>>>,
    pub dir: Option<NativeConfigValueChange<Option<String>>>,
    pub tab_index: Option<NativeConfigValueChange<Option<i32>>>,
    pub explicit_role: Option<NativeConfigValueChange<Option<String>>>,
    pub access_key: Option<NativeConfigValueChange<Option<String>>>,
    pub content_editable: Option<NativeConfigValueChange<Option<String>>>,
    pub draggable: Option<NativeConfigValueChange<Option<String>>>,
    pub spell_check: Option<NativeConfigValueChange<Option<bool>>>,
    pub translate: Option<NativeConfigValueChange<Option<bool>>>,
    pub inert: Option<NativeConfigValueChange<bool>>,
    pub popover: Option<NativeConfigValueChange<Option<String>>>,
    pub anchor: Option<NativeConfigValueChange<Option<String>>>,
    pub custom_element_is: Option<NativeConfigValueChange<Option<String>>>,
    pub nonce: Option<NativeConfigValueChange<Option<String>>>,
    pub name: Option<NativeConfigValueChange<Option<String>>>,
    pub form: Option<NativeConfigValueChange<Option<String>>>,
    pub input_type: Option<NativeConfigValueChange<Option<String>>>,
    pub accept: Option<NativeConfigValueChange<Option<String>>>,
    pub capture: Option<NativeConfigValueChange<Option<String>>>,
    pub alt: Option<NativeConfigValueChange<Option<String>>>,
    pub href: Option<NativeConfigValueChange<Option<String>>>,
    pub src: Option<NativeConfigValueChange<Option<String>>>,
    pub srcset: Option<NativeConfigValueChange<Option<String>>>,
    pub sizes: Option<NativeConfigValueChange<Option<String>>>,
    pub media: Option<NativeConfigValueChange<Option<String>>>,
    pub resource_type: Option<NativeConfigValueChange<Option<String>>>,
    pub intrinsic_width: Option<NativeConfigValueChange<Option<u32>>>,
    pub intrinsic_height: Option<NativeConfigValueChange<Option<u32>>>,
    pub loading: Option<NativeConfigValueChange<Option<String>>>,
    pub decoding: Option<NativeConfigValueChange<Option<String>>>,
    pub fetch_priority: Option<NativeConfigValueChange<Option<String>>>,
    pub cross_origin: Option<NativeConfigValueChange<Option<String>>>,
    pub referrer_policy: Option<NativeConfigValueChange<Option<String>>>,
    pub poster: Option<NativeConfigValueChange<Option<String>>>,
    pub controls: Option<NativeConfigValueChange<bool>>,
    pub autoplay: Option<NativeConfigValueChange<bool>>,
    pub loop_playback: Option<NativeConfigValueChange<bool>>,
    pub muted: Option<NativeConfigValueChange<bool>>,
    pub plays_inline: Option<NativeConfigValueChange<bool>>,
    pub preload: Option<NativeConfigValueChange<Option<String>>>,
    pub track_kind: Option<NativeConfigValueChange<Option<String>>>,
    pub srclang: Option<NativeConfigValueChange<Option<String>>>,
    pub track_label: Option<NativeConfigValueChange<Option<String>>>,
    pub default_track: Option<NativeConfigValueChange<bool>>,
    pub list: Option<NativeConfigValueChange<Option<String>>>,
    pub dirname: Option<NativeConfigValueChange<Option<String>>>,
    pub form_action: Option<NativeConfigValueChange<Option<String>>>,
    pub form_enctype: Option<NativeConfigValueChange<Option<String>>>,
    pub form_method: Option<NativeConfigValueChange<Option<String>>>,
    pub form_target: Option<NativeConfigValueChange<Option<String>>>,
    pub form_no_validate: Option<NativeConfigValueChange<bool>>,
    pub html_resource_policy: Option<NativeConfigValueChange<HtmlResourcePolicyProps>>,
    pub html_activation: Option<NativeConfigValueChange<HtmlActivationProps>>,
    pub html_text_annotation: Option<NativeConfigValueChange<HtmlTextAnnotationProps>>,
    pub html_dialog: Option<NativeConfigValueChange<HtmlDialogProps>>,
    pub html_shadow: Option<NativeConfigValueChange<HtmlShadowProps>>,
    pub html_microdata: Option<NativeConfigValueChange<HtmlMicrodataProps>>,
    pub html_form_association: Option<NativeConfigValueChange<HtmlFormAssociationProps>>,
    pub html_collection: Option<NativeConfigValueChange<HtmlCollectionProps>>,
    pub accessibility_relationships:
        Option<NativeConfigValueChange<AccessibilityRelationshipProps>>,
    pub accessibility_description: Option<NativeConfigValueChange<AccessibilityDescriptionProps>>,
    pub accessibility_structure: Option<NativeConfigValueChange<AccessibilityStructureProps>>,
    pub accessibility_state: Option<NativeConfigValueChange<AccessibilityStateProps>>,
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
            enter_key_hint: diff_value(&before.enter_key_hint, &after.enter_key_hint),
            auto_capitalize: diff_value(&before.auto_capitalize, &after.auto_capitalize),
            auto_correct: diff_value(&before.auto_correct, &after.auto_correct),
            virtual_keyboard_policy: diff_value(
                &before.virtual_keyboard_policy,
                &after.virtual_keyboard_policy,
            ),
            pattern: diff_value(&before.pattern, &after.pattern),
            min_length: diff_value(&before.min_length, &after.min_length),
            max_length: diff_value(&before.max_length, &after.max_length),
            rows: diff_value(&before.rows, &after.rows),
            cols: diff_value(&before.cols, &after.cols),
            size: diff_value(&before.size, &after.size),
            title: diff_value(&before.title, &after.title),
            window_resizable: diff_value(&before.window_resizable, &after.window_resizable),
            hidden: diff_value(&before.hidden, &after.hidden),
            lang: diff_value(&before.lang, &after.lang),
            dir: diff_value(&before.dir, &after.dir),
            tab_index: diff_value(&before.tab_index, &after.tab_index),
            explicit_role: diff_value(&before.explicit_role, &after.explicit_role),
            access_key: diff_value(&before.access_key, &after.access_key),
            content_editable: diff_value(&before.content_editable, &after.content_editable),
            draggable: diff_value(&before.draggable, &after.draggable),
            spell_check: diff_value(&before.spell_check, &after.spell_check),
            translate: diff_value(&before.translate, &after.translate),
            inert: diff_value(&before.inert, &after.inert),
            popover: diff_value(&before.popover, &after.popover),
            anchor: diff_value(&before.anchor, &after.anchor),
            custom_element_is: diff_value(&before.custom_element_is, &after.custom_element_is),
            nonce: diff_value(&before.nonce, &after.nonce),
            name: diff_value(&before.name, &after.name),
            form: diff_value(&before.form, &after.form),
            input_type: diff_value(&before.input_type, &after.input_type),
            accept: diff_value(&before.accept, &after.accept),
            capture: diff_value(&before.capture, &after.capture),
            alt: diff_value(&before.alt, &after.alt),
            href: diff_value(&before.href, &after.href),
            src: diff_value(&before.src, &after.src),
            srcset: diff_value(&before.srcset, &after.srcset),
            sizes: diff_value(&before.sizes, &after.sizes),
            media: diff_value(&before.media, &after.media),
            resource_type: diff_value(&before.resource_type, &after.resource_type),
            intrinsic_width: diff_value(&before.intrinsic_width, &after.intrinsic_width),
            intrinsic_height: diff_value(&before.intrinsic_height, &after.intrinsic_height),
            loading: diff_value(&before.loading, &after.loading),
            decoding: diff_value(&before.decoding, &after.decoding),
            fetch_priority: diff_value(&before.fetch_priority, &after.fetch_priority),
            cross_origin: diff_value(&before.cross_origin, &after.cross_origin),
            referrer_policy: diff_value(&before.referrer_policy, &after.referrer_policy),
            poster: diff_value(&before.poster, &after.poster),
            controls: diff_value(&before.controls, &after.controls),
            autoplay: diff_value(&before.autoplay, &after.autoplay),
            loop_playback: diff_value(&before.loop_playback, &after.loop_playback),
            muted: diff_value(&before.muted, &after.muted),
            plays_inline: diff_value(&before.plays_inline, &after.plays_inline),
            preload: diff_value(&before.preload, &after.preload),
            track_kind: diff_value(&before.track_kind, &after.track_kind),
            srclang: diff_value(&before.srclang, &after.srclang),
            track_label: diff_value(&before.track_label, &after.track_label),
            default_track: diff_value(&before.default_track, &after.default_track),
            list: diff_value(&before.list, &after.list),
            dirname: diff_value(&before.dirname, &after.dirname),
            form_action: diff_value(&before.form_action, &after.form_action),
            form_enctype: diff_value(&before.form_enctype, &after.form_enctype),
            form_method: diff_value(&before.form_method, &after.form_method),
            form_target: diff_value(&before.form_target, &after.form_target),
            form_no_validate: diff_value(&before.form_no_validate, &after.form_no_validate),
            html_resource_policy: diff_value(
                &before.html_resource_policy,
                &after.html_resource_policy,
            ),
            html_activation: diff_value(&before.html_activation, &after.html_activation),
            html_text_annotation: diff_value(
                &before.html_text_annotation,
                &after.html_text_annotation,
            ),
            html_dialog: diff_value(&before.html_dialog, &after.html_dialog),
            html_shadow: diff_value(&before.html_shadow, &after.html_shadow),
            html_microdata: diff_value(&before.html_microdata, &after.html_microdata),
            html_form_association: diff_value(
                &before.html_form_association,
                &after.html_form_association,
            ),
            html_collection: diff_value(&before.html_collection, &after.html_collection),
            accessibility_relationships: diff_value(
                &before.accessibility_relationships,
                &after.accessibility_relationships,
            ),
            accessibility_description: diff_value(
                &before.accessibility_description,
                &after.accessibility_description,
            ),
            accessibility_structure: diff_value(
                &before.accessibility_structure,
                &after.accessibility_structure,
            ),
            accessibility_state: diff_value(
                &before.accessibility_state,
                &after.accessibility_state,
            ),
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
        push_setter(
            &mut setters,
            &self.enter_key_hint,
            NativeWidgetSetter::SetEnterKeyHint,
        );
        push_setter(
            &mut setters,
            &self.auto_capitalize,
            NativeWidgetSetter::SetAutoCapitalize,
        );
        push_setter(
            &mut setters,
            &self.auto_correct,
            NativeWidgetSetter::SetAutoCorrect,
        );
        push_setter(
            &mut setters,
            &self.virtual_keyboard_policy,
            NativeWidgetSetter::SetVirtualKeyboardPolicy,
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
        push_setter(&mut setters, &self.title, NativeWidgetSetter::SetTitle);
        push_setter(
            &mut setters,
            &self.window_resizable,
            NativeWidgetSetter::SetWindowResizable,
        );
        push_setter(&mut setters, &self.hidden, NativeWidgetSetter::SetHidden);
        push_setter(&mut setters, &self.lang, NativeWidgetSetter::SetLang);
        push_setter(&mut setters, &self.dir, NativeWidgetSetter::SetDir);
        push_setter(
            &mut setters,
            &self.tab_index,
            NativeWidgetSetter::SetTabIndex,
        );
        push_setter(
            &mut setters,
            &self.explicit_role,
            NativeWidgetSetter::SetExplicitRole,
        );
        push_setter(
            &mut setters,
            &self.access_key,
            NativeWidgetSetter::SetAccessKey,
        );
        push_setter(
            &mut setters,
            &self.content_editable,
            NativeWidgetSetter::SetContentEditable,
        );
        push_setter(
            &mut setters,
            &self.draggable,
            NativeWidgetSetter::SetDraggable,
        );
        push_setter(
            &mut setters,
            &self.spell_check,
            NativeWidgetSetter::SetSpellCheck,
        );
        push_setter(
            &mut setters,
            &self.translate,
            NativeWidgetSetter::SetTranslate,
        );
        push_setter(&mut setters, &self.inert, NativeWidgetSetter::SetInert);
        push_setter(&mut setters, &self.popover, NativeWidgetSetter::SetPopover);
        push_setter(&mut setters, &self.anchor, NativeWidgetSetter::SetAnchor);
        push_setter(
            &mut setters,
            &self.custom_element_is,
            NativeWidgetSetter::SetCustomElementIs,
        );
        push_setter(&mut setters, &self.nonce, NativeWidgetSetter::SetNonce);
        push_setter(&mut setters, &self.name, NativeWidgetSetter::SetName);
        push_setter(&mut setters, &self.form, NativeWidgetSetter::SetForm);
        push_setter(
            &mut setters,
            &self.input_type,
            NativeWidgetSetter::SetInputType,
        );
        push_setter(&mut setters, &self.accept, NativeWidgetSetter::SetAccept);
        push_setter(&mut setters, &self.capture, NativeWidgetSetter::SetCapture);
        push_setter(&mut setters, &self.alt, NativeWidgetSetter::SetAlt);
        push_setter(&mut setters, &self.href, NativeWidgetSetter::SetHref);
        push_setter(&mut setters, &self.src, NativeWidgetSetter::SetSrc);
        push_setter(&mut setters, &self.srcset, NativeWidgetSetter::SetSrcset);
        push_setter(&mut setters, &self.sizes, NativeWidgetSetter::SetSizes);
        push_setter(&mut setters, &self.media, NativeWidgetSetter::SetMedia);
        push_setter(
            &mut setters,
            &self.resource_type,
            NativeWidgetSetter::SetResourceType,
        );
        push_setter(
            &mut setters,
            &self.intrinsic_width,
            NativeWidgetSetter::SetIntrinsicWidth,
        );
        push_setter(
            &mut setters,
            &self.intrinsic_height,
            NativeWidgetSetter::SetIntrinsicHeight,
        );
        push_setter(&mut setters, &self.loading, NativeWidgetSetter::SetLoading);
        push_setter(
            &mut setters,
            &self.decoding,
            NativeWidgetSetter::SetDecoding,
        );
        push_setter(
            &mut setters,
            &self.fetch_priority,
            NativeWidgetSetter::SetFetchPriority,
        );
        push_setter(
            &mut setters,
            &self.cross_origin,
            NativeWidgetSetter::SetCrossOrigin,
        );
        push_setter(
            &mut setters,
            &self.referrer_policy,
            NativeWidgetSetter::SetReferrerPolicy,
        );
        push_setter(&mut setters, &self.poster, NativeWidgetSetter::SetPoster);
        push_setter(
            &mut setters,
            &self.controls,
            NativeWidgetSetter::SetControls,
        );
        push_setter(
            &mut setters,
            &self.autoplay,
            NativeWidgetSetter::SetAutoplay,
        );
        push_setter(
            &mut setters,
            &self.loop_playback,
            NativeWidgetSetter::SetLoopPlayback,
        );
        push_setter(&mut setters, &self.muted, NativeWidgetSetter::SetMuted);
        push_setter(
            &mut setters,
            &self.plays_inline,
            NativeWidgetSetter::SetPlaysInline,
        );
        push_setter(&mut setters, &self.preload, NativeWidgetSetter::SetPreload);
        push_setter(
            &mut setters,
            &self.track_kind,
            NativeWidgetSetter::SetTrackKind,
        );
        push_setter(&mut setters, &self.srclang, NativeWidgetSetter::SetSrclang);
        push_setter(
            &mut setters,
            &self.track_label,
            NativeWidgetSetter::SetTrackLabel,
        );
        push_setter(
            &mut setters,
            &self.default_track,
            NativeWidgetSetter::SetDefaultTrack,
        );
        push_setter(&mut setters, &self.list, NativeWidgetSetter::SetList);
        push_setter(&mut setters, &self.dirname, NativeWidgetSetter::SetDirname);
        push_setter(
            &mut setters,
            &self.form_action,
            NativeWidgetSetter::SetFormAction,
        );
        push_setter(
            &mut setters,
            &self.form_enctype,
            NativeWidgetSetter::SetFormEnctype,
        );
        push_setter(
            &mut setters,
            &self.form_method,
            NativeWidgetSetter::SetFormMethod,
        );
        push_setter(
            &mut setters,
            &self.form_target,
            NativeWidgetSetter::SetFormTarget,
        );
        push_setter(
            &mut setters,
            &self.form_no_validate,
            NativeWidgetSetter::SetFormNoValidate,
        );
        push_setter(
            &mut setters,
            &self.html_resource_policy,
            NativeWidgetSetter::SetHtmlResourcePolicy,
        );
        push_setter(
            &mut setters,
            &self.html_activation,
            NativeWidgetSetter::SetHtmlActivation,
        );
        push_setter(
            &mut setters,
            &self.html_text_annotation,
            NativeWidgetSetter::SetHtmlTextAnnotation,
        );
        push_setter(
            &mut setters,
            &self.html_dialog,
            NativeWidgetSetter::SetHtmlDialog,
        );
        push_setter(
            &mut setters,
            &self.html_shadow,
            NativeWidgetSetter::SetHtmlShadow,
        );
        push_setter(
            &mut setters,
            &self.html_microdata,
            NativeWidgetSetter::SetHtmlMicrodata,
        );
        push_setter(
            &mut setters,
            &self.html_form_association,
            NativeWidgetSetter::SetHtmlFormAssociation,
        );
        push_setter(
            &mut setters,
            &self.html_collection,
            NativeWidgetSetter::SetHtmlCollection,
        );
        push_setter(
            &mut setters,
            &self.accessibility_relationships,
            NativeWidgetSetter::SetAccessibilityRelationships,
        );
        push_setter(
            &mut setters,
            &self.accessibility_description,
            NativeWidgetSetter::SetAccessibilityDescription,
        );
        push_setter(
            &mut setters,
            &self.accessibility_structure,
            NativeWidgetSetter::SetAccessibilityStructure,
        );
        push_setter(
            &mut setters,
            &self.accessibility_state,
            NativeWidgetSetter::SetAccessibilityState,
        );
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
