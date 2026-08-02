use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::protocol::NATIVE_PROTOCOL_VERSION_V1;

pub const TSX_PROTOCOL_NAME: &str = "a3s.gui.tsx";
pub const TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES: usize = 128;
pub const TSX_PROTOCOL_V1_MAX_VERSION_BYTES: usize = 128;
pub const TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES: usize = 4 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxRendererV1 {
    Auto,
    Software,
    Gpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxHostPlatformV1 {
    Headless,
    Macos,
    Windows,
    Linux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxDebugCapabilityV1 {
    ProtocolTrace,
    StructuredDiagnostics,
    Inspector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxHostCapabilityV1 {
    HeadlessRendering,
    SelfDrawnRendering,
    DropPolicyQueries,
    StructuredDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxHelloPayloadV1 {
    pub sdk_version: String,
    pub minimum_protocol_version: u32,
    pub maximum_protocol_version: u32,
    pub requested_renderer: TsxRendererV1,
    pub maximum_frame_bytes: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub debug_capabilities: Vec<TsxDebugCapabilityV1>,
}

impl TsxHelloPayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_bounded_text(
            "TSX SDK version",
            &self.sdk_version,
            TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
        )?;
        if self.minimum_protocol_version == 0 {
            return Err(GuiError::host(
                "TSX protocol minimum version must be non-zero",
            ));
        }
        if self.minimum_protocol_version > self.maximum_protocol_version {
            return Err(GuiError::host(
                "TSX protocol minimum version cannot exceed its maximum version",
            ));
        }
        super::TsxFrameLimitsV1::new(self.maximum_frame_bytes)?;
        validate_unique(
            "TSX debug capability",
            self.debug_capabilities.iter().copied(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxProtocolLimitsV1 {
    pub maximum_frame_bytes: u32,
    pub maximum_in_flight_renders: u16,
}

impl TsxProtocolLimitsV1 {
    pub fn validate(&self) -> GuiResult<()> {
        super::TsxFrameLimitsV1::new(self.maximum_frame_bytes)?;
        if self.maximum_in_flight_renders != 1 {
            return Err(GuiError::host(
                "TSX protocol v1 requires exactly one in-flight render",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxWelcomePayloadV1 {
    pub selected_protocol_version: u32,
    pub host_version: String,
    pub host_build_id: String,
    pub platform: TsxHostPlatformV1,
    pub renderer: TsxRendererV1,
    pub limits: TsxProtocolLimitsV1,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<TsxHostCapabilityV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub debug_capabilities: Vec<TsxDebugCapabilityV1>,
}

impl TsxWelcomePayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        if self.selected_protocol_version != NATIVE_PROTOCOL_VERSION_V1 {
            return Err(GuiError::host(format!(
                "TSX welcome selected unsupported protocol version {}; expected {}",
                self.selected_protocol_version, NATIVE_PROTOCOL_VERSION_V1
            )));
        }
        validate_bounded_text(
            "TSX host version",
            &self.host_version,
            TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
        )?;
        validate_bounded_text(
            "TSX host build id",
            &self.host_build_id,
            TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
        )?;
        if self.renderer == TsxRendererV1::Auto {
            return Err(GuiError::host(
                "TSX welcome must select a concrete renderer",
            ));
        }
        self.limits.validate()?;
        validate_unique("TSX host capability", self.capabilities.iter().copied())?;
        validate_unique(
            "TSX debug capability",
            self.debug_capabilities.iter().copied(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxLivenessPayloadV1 {
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TsxCloseReasonV1 {
    Normal,
    Requested,
    ProtocolError,
    HostShutdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxClosePayloadV1 {
    pub reason: TsxCloseReasonV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl TsxClosePayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_optional_diagnostic("TSX close message", self.message.as_deref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TsxFatalPayloadV1 {
    pub code: String,
    pub message: String,
}

impl TsxFatalPayloadV1 {
    pub fn validate(&self) -> GuiResult<()> {
        validate_bounded_text(
            "TSX fatal code",
            &self.code,
            TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
        )?;
        validate_bounded_text(
            "TSX fatal message",
            &self.message,
            TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsxMessageMetadataRefV1<'a> {
    pub protocol: &'a str,
    pub protocol_version: u32,
    pub session_id: &'a str,
    pub message_id: u64,
    pub render_revision: u64,
}

impl TsxMessageMetadataRefV1<'_> {
    pub fn validate(&self) -> GuiResult<()> {
        if self.protocol != TSX_PROTOCOL_NAME {
            return Err(GuiError::host(format!(
                "unsupported TSX protocol identifier {:?}; expected {:?}",
                self.protocol, TSX_PROTOCOL_NAME
            )));
        }
        if self.protocol_version != NATIVE_PROTOCOL_VERSION_V1 {
            return Err(GuiError::host(format!(
                "unsupported TSX protocol version {}; expected {}",
                self.protocol_version, NATIVE_PROTOCOL_VERSION_V1
            )));
        }
        validate_bounded_text(
            "TSX session id",
            self.session_id,
            TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
        )?;
        if self.message_id == 0 {
            return Err(GuiError::host("TSX protocol message ids must be non-zero"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
#[non_exhaustive]
pub enum TsxClientMessageV1 {
    Hello {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxHelloPayloadV1,
    },
    Render {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxRenderPayloadV1,
    },
    Ping {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxLivenessPayloadV1,
    },
    Pong {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxLivenessPayloadV1,
    },
    Close {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxClosePayloadV1,
    },
}

impl TsxClientMessageV1 {
    pub fn hello(
        session_id: impl Into<String>,
        message_id: u64,
        payload: TsxHelloPayloadV1,
    ) -> Self {
        Self::Hello {
            protocol: TSX_PROTOCOL_NAME.to_string(),
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            message_id,
            render_revision: 0,
            payload,
        }
    }

    pub fn render(
        session_id: impl Into<String>,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxRenderPayloadV1,
    ) -> Self {
        Self::Render {
            protocol: TSX_PROTOCOL_NAME.to_string(),
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            message_id,
            render_revision,
            payload,
        }
    }

    pub fn metadata(&self) -> TsxMessageMetadataRefV1<'_> {
        match self {
            Self::Hello {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Render {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Ping {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Pong {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Close {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            } => TsxMessageMetadataRefV1 {
                protocol,
                protocol_version: *protocol_version,
                session_id,
                message_id: *message_id,
                render_revision: *render_revision,
            },
        }
    }

    pub fn validate(&self) -> GuiResult<()> {
        let metadata = self.metadata();
        metadata.validate()?;
        match self {
            Self::Hello { payload, .. } => {
                if metadata.render_revision != 0 {
                    return Err(GuiError::host(
                        "TSX hello messages require render revision zero",
                    ));
                }
                payload.validate()
            }
            Self::Render { payload, .. } => {
                if metadata.render_revision == 0 {
                    return Err(GuiError::host(
                        "TSX render messages require a non-zero render revision",
                    ));
                }
                let _: crate::protocol::UiFrame = payload.clone().try_into()?;
                Ok(())
            }
            Self::Ping { .. } | Self::Pong { .. } => Ok(()),
            Self::Close { payload, .. } => payload.validate(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
#[non_exhaustive]
pub enum TsxHostMessageV1 {
    Welcome {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxWelcomePayloadV1,
    },
    Committed {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxCommittedPayloadV1,
    },
    Event {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxEventPayloadV1,
    },
    Ping {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxLivenessPayloadV1,
    },
    Pong {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxLivenessPayloadV1,
    },
    Close {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxClosePayloadV1,
    },
    Fatal {
        protocol: String,
        protocol_version: u32,
        session_id: String,
        message_id: u64,
        render_revision: u64,
        payload: TsxFatalPayloadV1,
    },
}

impl TsxHostMessageV1 {
    pub fn welcome(
        session_id: impl Into<String>,
        message_id: u64,
        payload: TsxWelcomePayloadV1,
    ) -> Self {
        Self::Welcome {
            protocol: TSX_PROTOCOL_NAME.to_string(),
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            message_id,
            render_revision: 0,
            payload,
        }
    }

    pub fn committed(
        session_id: impl Into<String>,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxCommittedPayloadV1,
    ) -> Self {
        Self::Committed {
            protocol: TSX_PROTOCOL_NAME.to_string(),
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            message_id,
            render_revision,
            payload,
        }
    }

    pub fn event(
        session_id: impl Into<String>,
        message_id: u64,
        render_revision: u64,
        payload: super::TsxEventPayloadV1,
    ) -> Self {
        Self::Event {
            protocol: TSX_PROTOCOL_NAME.to_string(),
            protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            session_id: session_id.into(),
            message_id,
            render_revision,
            payload,
        }
    }

    pub fn metadata(&self) -> TsxMessageMetadataRefV1<'_> {
        match self {
            Self::Welcome {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Committed {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Event {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Ping {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Pong {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Close {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            }
            | Self::Fatal {
                protocol,
                protocol_version,
                session_id,
                message_id,
                render_revision,
                ..
            } => TsxMessageMetadataRefV1 {
                protocol,
                protocol_version: *protocol_version,
                session_id,
                message_id: *message_id,
                render_revision: *render_revision,
            },
        }
    }

    pub fn validate(&self) -> GuiResult<()> {
        let metadata = self.metadata();
        metadata.validate()?;
        match self {
            Self::Welcome { payload, .. } => {
                if metadata.render_revision != 0 {
                    return Err(GuiError::host(
                        "TSX welcome messages require render revision zero",
                    ));
                }
                payload.validate()
            }
            Self::Committed { payload, .. } => {
                if metadata.render_revision == 0 {
                    return Err(GuiError::host(
                        "TSX committed messages require a non-zero render revision",
                    ));
                }
                payload.validate()
            }
            Self::Event { payload, .. } => {
                if metadata.render_revision == 0 {
                    return Err(GuiError::host(
                        "TSX event messages require a non-zero render revision",
                    ));
                }
                payload.validate()
            }
            Self::Ping { .. } | Self::Pong { .. } => Ok(()),
            Self::Close { payload, .. } => payload.validate(),
            Self::Fatal { payload, .. } => payload.validate(),
        }
    }
}

pub(super) fn validate_bounded_text(
    field: &str,
    value: &str,
    maximum_bytes: usize,
) -> GuiResult<()> {
    if value.trim().is_empty() {
        return Err(GuiError::host(format!("{field} must be non-empty")));
    }
    if value.len() > maximum_bytes {
        return Err(GuiError::host(format!(
            "{field} exceeds its {maximum_bytes}-byte limit"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(GuiError::host(format!(
            "{field} cannot contain control characters"
        )));
    }
    Ok(())
}

fn validate_optional_diagnostic(field: &str, value: Option<&str>) -> GuiResult<()> {
    match value {
        Some(value) => validate_bounded_text(field, value, TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES),
        None => Ok(()),
    }
}

pub(super) fn validate_unique<T>(field: &str, values: impl IntoIterator<Item = T>) -> GuiResult<()>
where
    T: Copy + Ord + std::fmt::Debug,
{
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(GuiError::host(format!(
                "{field} lists cannot contain duplicate value {value:?}"
            )));
        }
    }
    Ok(())
}
