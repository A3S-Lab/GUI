use super::*;

macro_rules! define_accessibility_dto {
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

define_accessibility_dto! {
    ProtocolAccessibilityRelationshipPropsV1 => AccessibilityRelationshipProps {
        labelled_by: Option<String>,
        described_by: Option<String>,
        details: Option<String>,
        controls: Option<String>,
        owns: Option<String>,
        flow_to: Option<String>,
        error_message: Option<String>,
        active_descendant: Option<String>,
    }
}

define_accessibility_dto! {
    ProtocolAccessibilityDescriptionPropsV1 => AccessibilityDescriptionProps {
        description: Option<String>,
        role_description: Option<String>,
        key_shortcuts: Option<String>,
        value_text: Option<String>,
    }
}

define_accessibility_dto! {
    ProtocolAccessibilityStructurePropsV1 => AccessibilityStructureProps {
        level: Option<u32>,
        position_in_set: Option<i32>,
        set_size: Option<i32>,
        row_count: Option<i32>,
        row_index: Option<i32>,
        row_span: Option<u32>,
        column_count: Option<i32>,
        column_index: Option<i32>,
        column_span: Option<u32>,
        row_index_text: Option<String>,
        column_index_text: Option<String>,
        sort: Option<String>,
    }
}

define_accessibility_dto! {
    ProtocolAccessibilityStatePropsV1 => AccessibilityStateProps {
        hidden: Option<bool>,
        autocomplete: Option<String>,
        multiline: Option<bool>,
        current: Option<String>,
        has_popup: Option<String>,
        pressed: Option<String>,
        live: Option<String>,
        atomic: Option<bool>,
        busy: Option<bool>,
        relevant: Option<String>,
        modal: Option<bool>,
    }
}
