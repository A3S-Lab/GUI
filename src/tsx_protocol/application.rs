use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::protocol::{
    ProtocolCollectionDropTargetV1, ProtocolDropOperationV1, ProtocolNativeEventKindV1,
    ProtocolUiFrameV1,
};

use super::{validate_bounded_text, TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES};

pub const TSX_PROTOCOL_V1_MAX_DIAGNOSTICS: usize = 1_024;
pub const TSX_PROTOCOL_V1_MAX_EVENT_ITEMS: usize = 65_536;
pub const TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES: usize = 1_024;
pub const TSX_PROTOCOL_V1_MAX_ACTION_ID_BYTES: usize = 1_024;
pub const TSX_PROTOCOL_V1_MAX_EVENT_VALUE_BYTES: usize = 1024 * 1024;

/// A fully resolved frame emitted by the TypeScript JSX runtime.
///
/// This is intentionally the existing input-only frame vocabulary. It does
/// not expose the legacy planned-widget render response to a TSX peer.
pub type TsxRenderPayloadV1 = ProtocolUiFrameV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxDiagnosticSeverityV1 {
    Information,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxDiagnosticV1 {
    pub severity: TsxDiagnosticSeverityV1,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
}

impl TsxDiagnosticV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_bounded_text(
            "TSX diagnostic code",
            &self.code,
            super::TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
        )?;
        validate_bounded_text(
            "TSX diagnostic message",
            &self.message,
            TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
        )?;
        if let Some(element_id) = &self.element_id {
            validate_element_id("TSX diagnostic element id", element_id)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxCommittedPayloadV1 {
    pub frame_id: String,
    /// Revision of the self-drawn host snapshot used for hit testing.
    ///
    /// It may remain unchanged when a TSX revision only changes callback
    /// ownership while producing identical Native IR.
    pub host_revision: u64,
    pub root_id: String,
    pub layout_fingerprint: u64,
    pub scene_fingerprint: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<TsxDiagnosticV1>,
}

impl TsxCommittedPayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_bounded_text(
            "TSX committed frame id",
            &self.frame_id,
            TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
        )?;
        if self.host_revision == 0 {
            return Err(GuiError::host(
                "TSX committed host revisions must be non-zero",
            ));
        }
        validate_element_id("TSX committed root id", &self.root_id)?;
        if self.diagnostics.len() > TSX_PROTOCOL_V1_MAX_DIAGNOSTICS {
            return Err(GuiError::host(format!(
                "TSX committed payload exceeds the {TSX_PROTOCOL_V1_MAX_DIAGNOSTICS}-diagnostic limit"
            )));
        }
        for diagnostic in &self.diagnostics {
            diagnostic.validate()?;
        }
        Ok(())
    }

    pub fn contains_error(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == TsxDiagnosticSeverityV1::Error)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxInputModalityV1 {
    #[default]
    Unknown,
    Keyboard,
    Mouse,
    Touch,
    Pen,
    Virtual,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxKeyModifiersV1 {
    pub alt: bool,
    pub control: bool,
    pub meta: bool,
    pub shift: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxPointV1 {
    pub x: f64,
    pub y: f64,
}

impl TsxPointV1 {
    fn validate(self, field: &str) -> GuiResult<()> {
        if !self.x.is_finite() || !self.y.is_finite() {
            return Err(GuiError::host(format!(
                "{field} coordinates must be finite"
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TsxPointerButtonV1 {
    Primary,
    Secondary,
    Auxiliary,
    Back,
    Forward,
    Other { code: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxWheelDeltaModeV1 {
    Pixels,
    Lines,
    Pages,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum TsxDropItemV1 {
    Text {
        types: Vec<String>,
        formats: BTreeMap<String, String>,
    },
}

impl TsxDropItemV1 {
    fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Text { types, formats } => {
                validate_non_empty_unique_texts("TSX drop item type", types)?;
                let format_types = formats.keys().cloned().collect::<Vec<_>>();
                if &format_types != types {
                    return Err(GuiError::host(
                        "TSX text drop item types must exactly match its sorted format keys",
                    ));
                }
                for value in formats.values() {
                    validate_event_value("TSX drop item format value", value)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxDragContextV1 {
    pub types: Vec<String>,
    pub value: Option<String>,
    pub items: Vec<TsxDropItemV1>,
    pub dragging_keys: Vec<String>,
    pub allowed_operations: Vec<ProtocolDropOperationV1>,
    pub drop_operation: ProtocolDropOperationV1,
    pub target: Option<ProtocolCollectionDropTargetV1>,
    pub is_internal: bool,
}

impl Default for TsxDragContextV1 {
    fn default() -> Self {
        Self {
            types: Vec::new(),
            value: None,
            items: Vec::new(),
            dragging_keys: Vec::new(),
            allowed_operations: Vec::new(),
            drop_operation: ProtocolDropOperationV1::Cancel,
            target: None,
            is_internal: false,
        }
    }
}

impl TsxDragContextV1 {
    fn validate(&self) -> GuiResult<()> {
        validate_unique_texts("TSX drag type", &self.types)?;
        if let Some(value) = &self.value {
            validate_event_value("TSX drag value", value)?;
        }
        if self.items.len() > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS {
            return Err(GuiError::host(format!(
                "TSX drag context exceeds the {TSX_PROTOCOL_V1_MAX_EVENT_ITEMS}-item limit"
            )));
        }
        for item in &self.items {
            item.validate()?;
        }
        validate_unique_texts("TSX dragging key", &self.dragging_keys)?;
        let mut operations = Vec::new();
        for operation in &self.allowed_operations {
            if operations.contains(operation) {
                return Err(GuiError::host(format!(
                    "TSX allowed drop operations contain duplicate {operation:?}"
                )));
            }
            operations.push(*operation);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxEventContextV1 {
    pub device: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<u64>,
    #[serde(default)]
    pub modality: TsxInputModalityV1,
    #[serde(default)]
    pub modifiers: TsxKeyModifiersV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<TsxPointV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<TsxPointV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button: Option<TsxPointerButtonV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wheel_delta_mode: Option<TsxWheelDeltaModeV1>,
    #[serde(default)]
    pub repeat: bool,
    #[serde(default)]
    pub click_count: u8,
    #[serde(default)]
    pub handled_activation: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag: Option<TsxDragContextV1>,
    pub timestamp_micros: u64,
}

impl TsxEventContextV1 {
    pub fn validate(&self) -> GuiResult<()> {
        if self.device == 0 {
            return Err(GuiError::host(
                "TSX event input device ids must be non-zero",
            ));
        }
        if self.pointer == Some(0) {
            return Err(GuiError::host("TSX event pointer ids must be non-zero"));
        }
        if let Some(position) = self.position {
            position.validate("TSX event position")?;
        }
        if let Some(delta) = self.delta {
            delta.validate("TSX event delta")?;
        }
        if self
            .pressure
            .is_some_and(|pressure| !pressure.is_finite() || !(0.0..=1.0).contains(&pressure))
        {
            return Err(GuiError::host(
                "TSX event pressure must be finite and between zero and one",
            ));
        }
        if let Some(related_target) = &self.related_target {
            validate_element_id("TSX related target", related_target)?;
        }
        if let Some(drag) = &self.drag {
            drag.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxActionInvocationV1 {
    pub node: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_target: Option<String>,
    pub action: String,
    pub event: ProtocolNativeEventKindV1,
    pub context: TsxEventContextV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TsxActionInvocationV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_element_id("TSX action node", &self.node)?;
        if let Some(current_target) = &self.current_target {
            validate_element_id("TSX action current target", current_target)?;
        }
        validate_bounded_text(
            "TSX action id",
            &self.action,
            TSX_PROTOCOL_V1_MAX_ACTION_ID_BYTES,
        )?;
        self.context.validate()?;
        if let Some(value) = &self.value {
            validate_event_value("TSX action value", value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxElementInteractionV1 {
    pub hovered: bool,
    pub pressed: bool,
    pub long_pressed: bool,
    pub moving: bool,
    pub dragging: bool,
    pub drop_target: bool,
    pub focused: bool,
    pub focus_visible: bool,
    pub focus_within: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxInteractionChangeV1 {
    pub node: String,
    pub before: TsxElementInteractionV1,
    pub after: TsxElementInteractionV1,
}

impl TsxInteractionChangeV1 {
    fn validate(&self) -> GuiResult<()> {
        validate_element_id("TSX interaction node", &self.node)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxEventPayloadV1 {
    /// Revision of the committed self-drawn snapshot that handled the input.
    pub host_revision: u64,
    pub event_sequence: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invocations: Vec<TsxActionInvocationV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<TsxInteractionChangeV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation_stopped_at: Option<String>,
}

impl TsxEventPayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        if self.host_revision == 0 {
            return Err(GuiError::host("TSX event host revisions must be non-zero"));
        }
        if self.event_sequence == 0 {
            return Err(GuiError::host("TSX event sequences must be non-zero"));
        }
        if let Some(target) = &self.target {
            validate_element_id("TSX event target", target)?;
        }
        if self.invocations.len() > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS {
            return Err(GuiError::host(format!(
                "TSX event exceeds the {TSX_PROTOCOL_V1_MAX_EVENT_ITEMS}-invocation limit"
            )));
        }
        for invocation in &self.invocations {
            invocation.validate()?;
        }
        if self.interaction_changes.len() > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS {
            return Err(GuiError::host(format!(
                "TSX event exceeds the {TSX_PROTOCOL_V1_MAX_EVENT_ITEMS}-interaction-change limit"
            )));
        }
        for change in &self.interaction_changes {
            change.validate()?;
        }
        if let Some(target) = &self.propagation_stopped_at {
            validate_element_id("TSX propagation stop target", target)?;
        }
        Ok(())
    }
}

fn validate_element_id(field: &str, value: &str) -> GuiResult<()> {
    validate_bounded_text(field, value, TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES)
}

fn validate_event_value(field: &str, value: &str) -> GuiResult<()> {
    if value.len() > TSX_PROTOCOL_V1_MAX_EVENT_VALUE_BYTES {
        return Err(GuiError::host(format!(
            "{field} exceeds its {TSX_PROTOCOL_V1_MAX_EVENT_VALUE_BYTES}-byte limit"
        )));
    }
    Ok(())
}

fn validate_non_empty_unique_texts(field: &str, values: &[String]) -> GuiResult<()> {
    if values.is_empty() {
        return Err(GuiError::host(format!("{field} lists must be non-empty")));
    }
    validate_unique_texts(field, values)
}

fn validate_unique_texts(field: &str, values: &[String]) -> GuiResult<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        validate_bounded_text(field, value, TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES)?;
        if !seen.insert(value) {
            return Err(GuiError::host(format!(
                "{field} lists cannot contain duplicate value {value:?}"
            )));
        }
    }
    Ok(())
}

#[cfg(feature = "platform-runtime")]
mod self_drawn {
    use crate::input::{NativeInputModality, NativeKeyModifiers};
    use crate::layout::LayoutElementId;
    use crate::platform_host::{PlatformPoint, PlatformPointerButton, PlatformWheelDeltaMode};
    use crate::platform_runtime::{
        SelfDrawnActionInvocation, SelfDrawnDragContext, SelfDrawnDropItem,
        SelfDrawnElementInteraction, SelfDrawnEventContext, SelfDrawnFrameSnapshot,
        SelfDrawnInputDispatch, SelfDrawnInteractionChange,
    };

    use super::*;

    impl TsxCommittedPayloadV1 {
        pub fn from_self_drawn_snapshot(
            frame_id: impl Into<String>,
            snapshot: &SelfDrawnFrameSnapshot,
            diagnostics: Vec<TsxDiagnosticV1>,
        ) -> GuiResult<Self> {
            let root_id = LayoutElementId::root(snapshot.native_root().key.as_str());
            let payload = Self {
                frame_id: frame_id.into(),
                host_revision: snapshot.revision().get(),
                root_id: root_id.as_str().to_string(),
                layout_fingerprint: snapshot.layout_fingerprint(),
                scene_fingerprint: snapshot.scene_fingerprint(),
                diagnostics,
            };
            payload.validate()?;
            Ok(payload)
        }
    }

    impl TryFrom<&SelfDrawnInputDispatch> for TsxEventPayloadV1 {
        type Error = GuiError;

        fn try_from(dispatch: &SelfDrawnInputDispatch) -> Result<Self, Self::Error> {
            if dispatch.invocations.iter().any(|invocation| {
                invocation.frame_revision != dispatch.frame_revision
                    || invocation.event_sequence != dispatch.event_sequence
            }) {
                return Err(GuiError::host(
                    "self-drawn action invocation metadata does not match its TSX event batch",
                ));
            }
            let payload = Self {
                host_revision: dispatch.frame_revision.get(),
                event_sequence: dispatch.event_sequence,
                target: dispatch
                    .target
                    .as_ref()
                    .map(|target| target.as_str().to_string()),
                invocations: dispatch
                    .invocations
                    .iter()
                    .map(TsxActionInvocationV1::try_from)
                    .collect::<GuiResult<Vec<_>>>()?,
                interaction_changes: dispatch
                    .interaction_changes
                    .iter()
                    .map(TsxInteractionChangeV1::from)
                    .collect(),
                propagation_stopped_at: dispatch
                    .propagation_stopped_at
                    .as_ref()
                    .map(|target| target.as_str().to_string()),
            };
            payload.validate()?;
            Ok(payload)
        }
    }

    impl TryFrom<&SelfDrawnActionInvocation> for TsxActionInvocationV1 {
        type Error = GuiError;

        fn try_from(invocation: &SelfDrawnActionInvocation) -> Result<Self, Self::Error> {
            let value = Self {
                node: invocation.node.as_str().to_string(),
                current_target: invocation
                    .current_target
                    .as_ref()
                    .map(|target| target.as_str().to_string()),
                action: invocation.action.clone(),
                event: invocation.event.into(),
                context: (&invocation.context).try_into()?,
                value: invocation.value.clone(),
            };
            value.validate()?;
            Ok(value)
        }
    }

    impl TryFrom<&SelfDrawnEventContext> for TsxEventContextV1 {
        type Error = GuiError;

        fn try_from(context: &SelfDrawnEventContext) -> Result<Self, Self::Error> {
            let value = Self {
                device: context.device.get(),
                pointer: context.pointer.map(|pointer| pointer.get()),
                modality: context.modality.into(),
                modifiers: context.modifiers.into(),
                position: context.position.map(Into::into),
                delta: context.delta.map(Into::into),
                button: context.button.map(Into::into),
                pressure: context.pressure,
                wheel_delta_mode: context.wheel_delta_mode.map(Into::into),
                repeat: context.repeat,
                click_count: context.click_count,
                handled_activation: context.handled_activation,
                related_target: context
                    .related_target
                    .as_ref()
                    .map(|target| target.as_str().to_string()),
                drag: context
                    .drag
                    .as_ref()
                    .map(TsxDragContextV1::try_from)
                    .transpose()?,
                timestamp_micros: context.timestamp_micros,
            };
            value.validate()?;
            Ok(value)
        }
    }

    impl TryFrom<&SelfDrawnDragContext> for TsxDragContextV1 {
        type Error = GuiError;

        fn try_from(context: &SelfDrawnDragContext) -> Result<Self, Self::Error> {
            let value = Self {
                types: context.types.clone(),
                value: context.value.clone(),
                items: context.items.iter().map(Into::into).collect(),
                dragging_keys: context.dragging_keys.clone(),
                allowed_operations: context
                    .allowed_operations
                    .iter()
                    .copied()
                    .map(Into::into)
                    .collect(),
                drop_operation: context.drop_operation.into(),
                target: context.target.as_ref().map(Into::into),
                is_internal: context.is_internal,
            };
            value.validate()?;
            Ok(value)
        }
    }

    impl From<&SelfDrawnDropItem> for TsxDropItemV1 {
        fn from(item: &SelfDrawnDropItem) -> Self {
            match item {
                SelfDrawnDropItem::Text { types, formats } => Self::Text {
                    types: types.clone(),
                    formats: formats.clone(),
                },
            }
        }
    }

    impl From<&SelfDrawnInteractionChange> for TsxInteractionChangeV1 {
        fn from(change: &SelfDrawnInteractionChange) -> Self {
            Self {
                node: change.node.as_str().to_string(),
                before: change.before.into(),
                after: change.after.into(),
            }
        }
    }

    impl From<SelfDrawnElementInteraction> for TsxElementInteractionV1 {
        fn from(value: SelfDrawnElementInteraction) -> Self {
            Self {
                hovered: value.hovered,
                pressed: value.pressed,
                long_pressed: value.long_pressed,
                moving: value.moving,
                dragging: value.dragging,
                drop_target: value.drop_target,
                focused: value.focused,
                focus_visible: value.focus_visible,
                focus_within: value.focus_within,
            }
        }
    }

    impl From<NativeInputModality> for TsxInputModalityV1 {
        fn from(value: NativeInputModality) -> Self {
            match value {
                NativeInputModality::Unknown => Self::Unknown,
                NativeInputModality::Keyboard => Self::Keyboard,
                NativeInputModality::Mouse => Self::Mouse,
                NativeInputModality::Touch => Self::Touch,
                NativeInputModality::Pen => Self::Pen,
                NativeInputModality::Virtual => Self::Virtual,
            }
        }
    }

    impl From<NativeKeyModifiers> for TsxKeyModifiersV1 {
        fn from(value: NativeKeyModifiers) -> Self {
            Self {
                alt: value.alt,
                control: value.control,
                meta: value.meta,
                shift: value.shift,
            }
        }
    }

    impl From<PlatformPoint> for TsxPointV1 {
        fn from(value: PlatformPoint) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl From<PlatformPointerButton> for TsxPointerButtonV1 {
        fn from(value: PlatformPointerButton) -> Self {
            match value {
                PlatformPointerButton::Primary => Self::Primary,
                PlatformPointerButton::Secondary => Self::Secondary,
                PlatformPointerButton::Auxiliary => Self::Auxiliary,
                PlatformPointerButton::Back => Self::Back,
                PlatformPointerButton::Forward => Self::Forward,
                PlatformPointerButton::Other(code) => Self::Other { code },
            }
        }
    }

    impl From<PlatformWheelDeltaMode> for TsxWheelDeltaModeV1 {
        fn from(value: PlatformWheelDeltaMode) -> Self {
            match value {
                PlatformWheelDeltaMode::Pixels => Self::Pixels,
                PlatformWheelDeltaMode::Lines => Self::Lines,
                PlatformWheelDeltaMode::Pages => Self::Pages,
            }
        }
    }
}
