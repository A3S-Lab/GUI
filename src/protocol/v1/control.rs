use crate::geometry::Orientation;
use crate::html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
};

use super::*;

macro_rules! define_html_dto {
    ($protocol:ident => $internal:ty { $( $field:ident: $ty:ty ),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct $protocol {
            $(#[serde(default)] pub $field: $ty,)+
        }

        impl From<&$internal> for $protocol {
            fn from(value: &$internal) -> Self {
                Self {
                    $($field: value.$field.clone(),)+
                }
            }
        }

        impl From<$protocol> for $internal {
            fn from(value: $protocol) -> Self {
                Self {
                    $($field: value.$field,)+
                }
            }
        }
    };
}

define_html_dto! {
    ProtocolHtmlActivationPropsV1 => HtmlActivationProps {
        command: Option<String>,
        command_for: Option<String>,
        popover_target: Option<String>,
        popover_target_action: Option<String>,
    }
}

define_html_dto! {
    ProtocolHtmlCollectionPropsV1 => HtmlCollectionProps {
        column_span: Option<u32>,
        row_span: Option<u32>,
        headers: Option<String>,
        scope: Option<String>,
        cell_abbr: Option<String>,
        list_start: Option<i32>,
        list_reversed: bool,
        list_type: Option<String>,
        list_item_value: Option<i32>,
    }
}

define_html_dto! {
    ProtocolHtmlDialogPropsV1 => HtmlDialogProps {
        open: Option<bool>,
    }
}

define_html_dto! {
    ProtocolHtmlFormAssociationPropsV1 => HtmlFormAssociationProps {
        label_for: Option<String>,
        output_for: Option<String>,
        meter_low: Option<f64>,
        meter_high: Option<f64>,
        meter_optimum: Option<f64>,
    }
}

define_html_dto! {
    ProtocolHtmlMicrodataPropsV1 => HtmlMicrodataProps {
        item_scope: bool,
        item_prop: Option<String>,
        item_type: Option<String>,
        item_id: Option<String>,
        item_ref: Option<String>,
    }
}

define_html_dto! {
    ProtocolHtmlResourcePolicyPropsV1 => HtmlResourcePolicyProps {
        target: Option<String>,
        download: Option<String>,
        ping: Option<String>,
        rel: Option<String>,
        href_lang: Option<String>,
        link_as: Option<String>,
        integrity: Option<String>,
        blocking: Option<String>,
        nonce: Option<String>,
        image_srcset: Option<String>,
        image_sizes: Option<String>,
        resource_disabled: bool,
        async_script: bool,
        defer_script: bool,
        no_module: bool,
        frame_name: Option<String>,
        frame_allow: Option<String>,
        frame_allow_fullscreen: bool,
        frame_sandbox: Option<String>,
        frame_srcdoc: Option<String>,
    }
}

define_html_dto! {
    ProtocolHtmlShadowPropsV1 => HtmlShadowProps {
        slot_name: Option<String>,
        part: Option<String>,
        export_parts: Option<String>,
    }
}

define_html_dto! {
    ProtocolHtmlTextAnnotationPropsV1 => HtmlTextAnnotationProps {
        cite: Option<String>,
        date_time: Option<String>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolOrientationV1 {
    Horizontal,
    Vertical,
}

impl From<Orientation> for ProtocolOrientationV1 {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => Self::Horizontal,
            Orientation::Vertical => Self::Vertical,
        }
    }
}

impl From<ProtocolOrientationV1> for Orientation {
    fn from(value: ProtocolOrientationV1) -> Self {
        match value {
            ProtocolOrientationV1::Horizontal => Self::Horizontal,
            ProtocolOrientationV1::Vertical => Self::Vertical,
        }
    }
}

macro_rules! define_control_state_v1 {
    ($( $field:ident: $ty:ty ),+ $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct ProtocolNativeControlStateV1 {
            $(#[serde(default)] pub $field: $ty,)+
            #[serde(default)]
            pub orientation: Option<ProtocolOrientationV1>,
            #[serde(default)]
            pub html_resource_policy: ProtocolHtmlResourcePolicyPropsV1,
            #[serde(default)]
            pub html_activation: ProtocolHtmlActivationPropsV1,
            #[serde(default)]
            pub html_text_annotation: ProtocolHtmlTextAnnotationPropsV1,
            #[serde(default)]
            pub html_dialog: ProtocolHtmlDialogPropsV1,
            #[serde(default)]
            pub html_shadow: ProtocolHtmlShadowPropsV1,
            #[serde(default)]
            pub html_microdata: ProtocolHtmlMicrodataPropsV1,
            #[serde(default)]
            pub html_form_association: ProtocolHtmlFormAssociationPropsV1,
            #[serde(default)]
            pub html_collection: ProtocolHtmlCollectionPropsV1,
            #[serde(default)]
            pub accessibility_relationships: ProtocolAccessibilityRelationshipPropsV1,
            #[serde(default)]
            pub accessibility_description: ProtocolAccessibilityDescriptionPropsV1,
            #[serde(default)]
            pub accessibility_structure: ProtocolAccessibilityStructurePropsV1,
            #[serde(default)]
            pub accessibility_state: ProtocolAccessibilityStatePropsV1,
        }

        impl From<&NativeControlState> for ProtocolNativeControlStateV1 {
            fn from(value: &NativeControlState) -> Self {
                Self {
                    $($field: value.$field.clone(),)+
                    orientation: value.orientation.map(Into::into),
                    html_resource_policy: (&value.html_resource_policy).into(),
                    html_activation: (&value.html_activation).into(),
                    html_text_annotation: (&value.html_text_annotation).into(),
                    html_dialog: (&value.html_dialog).into(),
                    html_shadow: (&value.html_shadow).into(),
                    html_microdata: (&value.html_microdata).into(),
                    html_form_association: (&value.html_form_association).into(),
                    html_collection: (&value.html_collection).into(),
                    accessibility_relationships: (&value.accessibility_relationships).into(),
                    accessibility_description: (&value.accessibility_description).into(),
                    accessibility_structure: (&value.accessibility_structure).into(),
                    accessibility_state: (&value.accessibility_state).into(),
                }
            }
        }

        impl From<ProtocolNativeControlStateV1> for NativeControlState {
            fn from(value: ProtocolNativeControlStateV1) -> Self {
                Self {
                    $($field: value.$field,)+
                    orientation: value.orientation.map(Into::into),
                    html_resource_policy: value.html_resource_policy.into(),
                    html_activation: value.html_activation.into(),
                    html_text_annotation: value.html_text_annotation.into(),
                    html_dialog: value.html_dialog.into(),
                    html_shadow: value.html_shadow.into(),
                    html_microdata: value.html_microdata.into(),
                    html_form_association: value.html_form_association.into(),
                    html_collection: value.html_collection.into(),
                    accessibility_relationships: value.accessibility_relationships.into(),
                    accessibility_description: value.accessibility_description.into(),
                    accessibility_structure: value.accessibility_structure.into(),
                    accessibility_state: value.accessibility_state.into(),
                }
            }
        }
    };
}

define_control_state_v1! {
    placeholder: Option<String>,
    disabled: bool,
    required: bool,
    invalid: bool,
    read_only: bool,
    multiple: bool,
    auto_focus: bool,
    selected: bool,
    checked: Option<bool>,
    expanded: Option<bool>,
    min: Option<f64>,
    max: Option<f64>,
    current: Option<f64>,
    step: Option<f64>,
    autocomplete: Option<String>,
    input_mode: Option<String>,
    enter_key_hint: Option<String>,
    auto_capitalize: Option<String>,
    auto_correct: Option<String>,
    virtual_keyboard_policy: Option<String>,
    pattern: Option<String>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    rows: Option<u32>,
    cols: Option<u32>,
    size: Option<u32>,
    title: Option<String>,
    hidden: bool,
    lang: Option<String>,
    dir: Option<String>,
    tab_index: Option<i32>,
    explicit_role: Option<String>,
    access_key: Option<String>,
    content_editable: Option<String>,
    draggable: Option<String>,
    spell_check: Option<bool>,
    translate: Option<bool>,
    inert: bool,
    popover: Option<String>,
    anchor: Option<String>,
    custom_element_is: Option<String>,
    nonce: Option<String>,
    name: Option<String>,
    form: Option<String>,
    input_type: Option<String>,
    accept: Option<String>,
    capture: Option<String>,
    alt: Option<String>,
    href: Option<String>,
    src: Option<String>,
    srcset: Option<String>,
    sizes: Option<String>,
    media: Option<String>,
    resource_type: Option<String>,
    intrinsic_width: Option<u32>,
    intrinsic_height: Option<u32>,
    loading: Option<String>,
    decoding: Option<String>,
    fetch_priority: Option<String>,
    cross_origin: Option<String>,
    referrer_policy: Option<String>,
    poster: Option<String>,
    controls: bool,
    autoplay: bool,
    loop_playback: bool,
    muted: bool,
    plays_inline: bool,
    preload: Option<String>,
    track_kind: Option<String>,
    srclang: Option<String>,
    track_label: Option<String>,
    default_track: bool,
    list: Option<String>,
    dirname: Option<String>,
    form_action: Option<String>,
    form_enctype: Option<String>,
    form_method: Option<String>,
    form_target: Option<String>,
    form_no_validate: bool,
}
