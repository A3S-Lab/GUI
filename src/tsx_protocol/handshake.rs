use crate::error::{GuiError, GuiResult};
use crate::protocol::NATIVE_PROTOCOL_VERSION_V1;

use super::{
    encode_tsx_json_frame_v1, validate_bounded_text, validate_unique, TsxClientMessageV1,
    TsxDebugCapabilityV1, TsxFrameLimitsV1, TsxHostCapabilityV1, TsxHostMessageV1,
    TsxHostPlatformV1, TsxProtocolLimitsV1, TsxRendererV1, TsxWelcomePayloadV1,
    TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsxHostHandshakeConfigV1 {
    host_version: String,
    host_build_id: String,
    platform: TsxHostPlatformV1,
    supported_renderers: Vec<TsxRendererV1>,
    maximum_frame_bytes: u32,
    capabilities: Vec<TsxHostCapabilityV1>,
    debug_capabilities: Vec<TsxDebugCapabilityV1>,
}

impl TsxHostHandshakeConfigV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        host_version: impl Into<String>,
        host_build_id: impl Into<String>,
        platform: TsxHostPlatformV1,
        supported_renderers: Vec<TsxRendererV1>,
        maximum_frame_bytes: u32,
        capabilities: Vec<TsxHostCapabilityV1>,
        debug_capabilities: Vec<TsxDebugCapabilityV1>,
    ) -> GuiResult<Self> {
        let config = Self {
            host_version: host_version.into(),
            host_build_id: host_build_id.into(),
            platform,
            supported_renderers,
            maximum_frame_bytes,
            capabilities,
            debug_capabilities,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn host_version(&self) -> &str {
        &self.host_version
    }

    pub fn host_build_id(&self) -> &str {
        &self.host_build_id
    }

    pub fn platform(&self) -> TsxHostPlatformV1 {
        self.platform
    }

    pub fn supported_renderers(&self) -> &[TsxRendererV1] {
        &self.supported_renderers
    }

    pub fn maximum_frame_bytes(&self) -> u32 {
        self.maximum_frame_bytes
    }

    pub fn capabilities(&self) -> &[TsxHostCapabilityV1] {
        &self.capabilities
    }

    pub fn debug_capabilities(&self) -> &[TsxDebugCapabilityV1] {
        &self.debug_capabilities
    }

    fn validate(&self) -> GuiResult<()> {
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
        TsxFrameLimitsV1::new(self.maximum_frame_bytes)?;
        if self.supported_renderers.is_empty() {
            return Err(GuiError::host(
                "TSX host handshake requires at least one concrete renderer",
            ));
        }
        if self.supported_renderers.contains(&TsxRendererV1::Auto) {
            return Err(GuiError::host(
                "TSX host renderer capabilities cannot contain auto",
            ));
        }
        validate_unique(
            "TSX host renderer",
            self.supported_renderers.iter().copied(),
        )?;
        validate_unique("TSX host capability", self.capabilities.iter().copied())?;
        validate_unique(
            "TSX debug capability",
            self.debug_capabilities.iter().copied(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsxNegotiatedSessionV1 {
    session_id: String,
    sdk_version: String,
    renderer: TsxRendererV1,
    limits: TsxProtocolLimitsV1,
    capabilities: Vec<TsxHostCapabilityV1>,
    debug_capabilities: Vec<TsxDebugCapabilityV1>,
}

impl TsxNegotiatedSessionV1 {
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn sdk_version(&self) -> &str {
        &self.sdk_version
    }

    pub fn renderer(&self) -> TsxRendererV1 {
        self.renderer
    }

    pub fn limits(&self) -> TsxProtocolLimitsV1 {
        self.limits
    }

    pub fn capabilities(&self) -> &[TsxHostCapabilityV1] {
        &self.capabilities
    }

    pub fn debug_capabilities(&self) -> &[TsxDebugCapabilityV1] {
        &self.debug_capabilities
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsxHostHandshakeV1 {
    config: TsxHostHandshakeConfigV1,
    negotiated: Option<TsxNegotiatedSessionV1>,
}

impl TsxHostHandshakeV1 {
    pub fn new(config: TsxHostHandshakeConfigV1) -> Self {
        Self {
            config,
            negotiated: None,
        }
    }

    pub fn config(&self) -> &TsxHostHandshakeConfigV1 {
        &self.config
    }

    pub fn negotiated(&self) -> Option<&TsxNegotiatedSessionV1> {
        self.negotiated.as_ref()
    }

    /// Validates and accepts the first client message atomically.
    ///
    /// Failed attempts leave the handshake unbound so a transport can report a
    /// structured fatal response or close without exposing a partial session.
    pub fn accept_hello(&mut self, message: &TsxClientMessageV1) -> GuiResult<TsxHostMessageV1> {
        if self.negotiated.is_some() {
            return Err(GuiError::host(
                "TSX protocol handshake has already completed",
            ));
        }
        message.validate()?;
        let TsxClientMessageV1::Hello {
            session_id,
            message_id,
            payload,
            ..
        } = message
        else {
            return Err(GuiError::host(
                "the first TSX protocol client message must be hello",
            ));
        };
        if *message_id != 1 {
            return Err(GuiError::host(format!(
                "TSX hello message id {message_id} is invalid; expected 1"
            )));
        }
        if payload.minimum_protocol_version > NATIVE_PROTOCOL_VERSION_V1
            || payload.maximum_protocol_version < NATIVE_PROTOCOL_VERSION_V1
        {
            return Err(GuiError::host(format!(
                "TSX SDK protocol range {}..={} does not include host version {}",
                payload.minimum_protocol_version,
                payload.maximum_protocol_version,
                NATIVE_PROTOCOL_VERSION_V1
            )));
        }

        let renderer = match payload.requested_renderer {
            TsxRendererV1::Auto => self.config.supported_renderers[0],
            requested if self.config.supported_renderers.contains(&requested) => requested,
            requested => {
                return Err(GuiError::host(format!(
                    "TSX requested renderer {requested:?} is not supported by this host"
                )))
            }
        };
        let maximum_frame_bytes = payload
            .maximum_frame_bytes
            .min(self.config.maximum_frame_bytes);
        let limits = TsxProtocolLimitsV1 {
            maximum_frame_bytes,
            maximum_in_flight_renders: 1,
        };
        limits.validate()?;
        let debug_capabilities = payload
            .debug_capabilities
            .iter()
            .copied()
            .filter(|capability| self.config.debug_capabilities.contains(capability))
            .collect::<Vec<_>>();
        let welcome_payload = TsxWelcomePayloadV1 {
            selected_protocol_version: NATIVE_PROTOCOL_VERSION_V1,
            host_version: self.config.host_version.clone(),
            host_build_id: self.config.host_build_id.clone(),
            platform: self.config.platform,
            renderer,
            limits,
            capabilities: self.config.capabilities.clone(),
            debug_capabilities: debug_capabilities.clone(),
        };
        let welcome = TsxHostMessageV1::welcome(session_id.clone(), 1, welcome_payload);
        welcome.validate()?;

        // A tiny client limit cannot negotiate a welcome it cannot receive.
        // Serialize before mutating handshake state so this failure is atomic.
        encode_tsx_json_frame_v1(&welcome, TsxFrameLimitsV1::new(maximum_frame_bytes)?)?;

        self.negotiated = Some(TsxNegotiatedSessionV1 {
            session_id: session_id.clone(),
            sdk_version: payload.sdk_version.clone(),
            renderer,
            limits,
            capabilities: self.config.capabilities.clone(),
            debug_capabilities,
        });
        Ok(welcome)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TsxMessageSequenceV1 {
    last_message_id: u64,
}

impl TsxMessageSequenceV1 {
    pub const fn new() -> Self {
        Self { last_message_id: 0 }
    }

    pub const fn from_last_message_id(last_message_id: u64) -> Self {
        Self { last_message_id }
    }

    pub const fn last_message_id(self) -> u64 {
        self.last_message_id
    }

    pub fn expected_message_id(self) -> GuiResult<u64> {
        let expected = self
            .last_message_id
            .checked_add(1)
            .ok_or_else(|| GuiError::host("TSX protocol message id sequence overflowed"))?;
        if expected > super::TSX_PROTOCOL_V1_MAX_SAFE_INTEGER {
            return Err(GuiError::host(
                "TSX protocol message id sequence exceeded JavaScript's maximum safe integer",
            ));
        }
        Ok(expected)
    }

    /// Accepts only the exact next id and advances only after validation.
    pub fn accept(&mut self, message_id: u64) -> GuiResult<()> {
        let expected = self.expected_message_id()?;
        if message_id != expected {
            return Err(GuiError::host(format!(
                "TSX protocol message id {message_id} is invalid; expected {expected}"
            )));
        }
        self.last_message_id = message_id;
        Ok(())
    }
}
