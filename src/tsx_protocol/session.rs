use crate::compiler::CompiledRsxNode;
use crate::error::{GuiError, GuiResult};
use crate::protocol::UiFrame;

use super::{
    encode_tsx_json_frame_v1, TsxClientMessageV1, TsxCommittedPayloadV1, TsxEventPayloadV1,
    TsxFrameLimitsV1, TsxHostMessageV1, TsxLivenessPayloadV1, TsxMessageSequenceV1,
    TsxNegotiatedSessionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsxPendingRenderV1 {
    render_revision: u64,
    frame_id: String,
    root_key: String,
}

impl TsxPendingRenderV1 {
    pub const fn render_revision(&self) -> u64 {
        self.render_revision
    }

    pub fn frame_id(&self) -> &str {
        &self.frame_id
    }

    pub fn root_key(&self) -> &str {
        &self.root_key
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsxAcceptedRenderV1 {
    render_revision: u64,
    frame: UiFrame,
}

impl TsxAcceptedRenderV1 {
    pub const fn render_revision(&self) -> u64 {
        self.render_revision
    }

    pub fn frame(&self) -> &UiFrame {
        &self.frame
    }

    pub fn into_frame(self) -> UiFrame {
        self.frame
    }
}

/// Transactional ordering state for one negotiated TSX application session.
///
/// This type owns protocol ordering only. The caller still owns semantic
/// lowering and the self-drawn runtime's prepare/commit transaction. A render
/// becomes active here only after that caller supplies a successful committed
/// payload.
#[derive(Debug, Clone, PartialEq)]
pub struct TsxHostApplicationSessionV1 {
    session_id: String,
    limits: TsxFrameLimitsV1,
    client_messages: TsxMessageSequenceV1,
    host_messages: TsxMessageSequenceV1,
    committed_render_revision: u64,
    committed_host_revision: Option<u64>,
    last_event_sequence: u64,
    pending_render: Option<TsxPendingRenderV1>,
    pending_host_ping_nonce: Option<u64>,
}

impl TsxHostApplicationSessionV1 {
    pub fn new(negotiated: &TsxNegotiatedSessionV1) -> GuiResult<Self> {
        let limits = TsxFrameLimitsV1::new(negotiated.limits().maximum_frame_bytes)?;
        Ok(Self {
            session_id: negotiated.session_id().to_string(),
            limits,
            // The accepted hello and emitted welcome both have id 1.
            client_messages: TsxMessageSequenceV1::from_last_message_id(1),
            host_messages: TsxMessageSequenceV1::from_last_message_id(1),
            committed_render_revision: 0,
            committed_host_revision: None,
            last_event_sequence: 0,
            pending_render: None,
            pending_host_ping_nonce: None,
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub const fn committed_render_revision(&self) -> u64 {
        self.committed_render_revision
    }

    pub const fn committed_host_revision(&self) -> Option<u64> {
        self.committed_host_revision
    }

    pub const fn last_event_sequence(&self) -> u64 {
        self.last_event_sequence
    }

    pub fn pending_render(&self) -> Option<&TsxPendingRenderV1> {
        self.pending_render.as_ref()
    }

    pub const fn last_client_message_id(&self) -> u64 {
        self.client_messages.last_message_id()
    }

    pub const fn last_host_message_id(&self) -> u64 {
        self.host_messages.last_message_id()
    }

    pub const fn pending_host_ping_nonce(&self) -> Option<u64> {
        self.pending_host_ping_nonce
    }

    /// Starts one host-originated liveness probe without changing UI state.
    pub fn begin_host_ping(&mut self, nonce: u64) -> GuiResult<TsxHostMessageV1> {
        if let Some(pending) = self.pending_host_ping_nonce {
            return Err(GuiError::host(format!(
                "TSX host liveness nonce {pending} is already pending"
            )));
        }
        let payload = TsxLivenessPayloadV1 { nonce };
        payload.validate()?;
        let message_id = self.host_messages.expected_message_id()?;
        let ping = TsxHostMessageV1::ping(
            self.session_id.clone(),
            message_id,
            self.committed_render_revision,
            payload,
        );
        ping.validate()?;
        encode_tsx_json_frame_v1(&ping, self.limits)?;

        self.host_messages.accept(message_id)?;
        self.pending_host_ping_nonce = Some(nonce);
        Ok(ping)
    }

    /// Accepts the exact next full-frame render without changing the active
    /// committed revision.
    pub fn accept_render(
        &mut self,
        message: &TsxClientMessageV1,
    ) -> GuiResult<TsxAcceptedRenderV1> {
        message.validate()?;
        let TsxClientMessageV1::Render {
            session_id,
            message_id,
            render_revision,
            payload,
            ..
        } = message
        else {
            return Err(GuiError::host(
                "TSX application session expected a render message",
            ));
        };
        self.validate_client_identity(session_id, *message_id)?;
        if self.pending_render.is_some() {
            return Err(GuiError::host(
                "TSX protocol v1 permits only one in-flight render",
            ));
        }
        let expected_revision = self
            .committed_render_revision
            .checked_add(1)
            .ok_or_else(|| GuiError::host("TSX render revision sequence overflowed"))?;
        if *render_revision != expected_revision {
            return Err(GuiError::host(format!(
                "TSX render revision {render_revision} is invalid; expected {expected_revision}"
            )));
        }

        let frame: UiFrame = payload.clone().try_into()?;
        let root_key = compiled_key(&frame.root).to_string();
        let pending = TsxPendingRenderV1 {
            render_revision: *render_revision,
            frame_id: frame.frame_id.clone(),
            root_key,
        };

        // Nothing below this point can fail after the exact id check.
        self.client_messages.accept(*message_id)?;
        self.pending_render = Some(pending);
        Ok(TsxAcceptedRenderV1 {
            render_revision: *render_revision,
            frame,
        })
    }

    /// Atomically accepts a control message and sequences its required reply.
    ///
    /// Ping receives pong, close receives close, and pong needs no reply. The
    /// complete reply must fit the negotiated frame before either independent
    /// message sequence advances.
    pub fn accept_control(
        &mut self,
        message: &TsxClientMessageV1,
    ) -> GuiResult<Option<TsxHostMessageV1>> {
        self.validate_control(message)?;
        let metadata = message.metadata();
        let response = match message {
            TsxClientMessageV1::Ping { payload, .. } => Some(TsxHostMessageV1::pong(
                self.session_id.clone(),
                self.host_messages.expected_message_id()?,
                self.committed_render_revision,
                *payload,
            )),
            TsxClientMessageV1::Pong { payload, .. } => {
                let pending = self.pending_host_ping_nonce.ok_or_else(|| {
                    GuiError::host("TSX client pong has no pending host liveness probe")
                })?;
                if payload.nonce != pending {
                    return Err(GuiError::host(format!(
                        "TSX client pong nonce {} does not match pending host nonce {pending}",
                        payload.nonce
                    )));
                }
                None
            }
            TsxClientMessageV1::Close { payload, .. } => Some(TsxHostMessageV1::close(
                self.session_id.clone(),
                self.host_messages.expected_message_id()?,
                self.committed_render_revision,
                payload.clone(),
            )),
            TsxClientMessageV1::Hello { .. } | TsxClientMessageV1::Render { .. } => {
                unreachable!("validated TSX control message")
            }
        };
        if let Some(response) = &response {
            response.validate()?;
            encode_tsx_json_frame_v1(response, self.limits)?;
        }

        self.client_messages.accept(metadata.message_id)?;
        if matches!(
            message,
            TsxClientMessageV1::Pong { .. } | TsxClientMessageV1::Close { .. }
        ) {
            self.pending_host_ping_nonce = None;
        }
        if let Some(response) = &response {
            self.host_messages.accept(response.metadata().message_id)?;
        }
        Ok(response)
    }

    /// Abandons a prepared render while retaining the previous committed
    /// revision. The next client message may retry that same revision.
    pub fn reject_pending_render(&mut self, render_revision: u64) -> GuiResult<()> {
        let pending = self.pending_render.as_ref().ok_or_else(|| {
            GuiError::host("TSX application session has no pending render to reject")
        })?;
        if pending.render_revision != render_revision {
            return Err(GuiError::host(format!(
                "cannot reject TSX render revision {render_revision}; revision {} is pending",
                pending.render_revision
            )));
        }
        self.pending_render = None;
        Ok(())
    }

    /// Promotes the pending TSX revision only after the self-drawn host has
    /// committed and after the complete response fits the negotiated frame.
    pub fn commit_pending_render(
        &mut self,
        payload: TsxCommittedPayloadV1,
    ) -> GuiResult<TsxHostMessageV1> {
        payload.validate()?;
        if payload.contains_error() {
            return Err(GuiError::host(
                "TSX renders with error diagnostics cannot be committed",
            ));
        }
        let pending = self.pending_render.as_ref().cloned().ok_or_else(|| {
            GuiError::host("TSX application session has no pending render to commit")
        })?;
        if payload.frame_id != pending.frame_id {
            return Err(GuiError::host(format!(
                "TSX committed frame id {:?} does not match pending frame {:?}",
                payload.frame_id, pending.frame_id
            )));
        }
        if self
            .committed_host_revision
            .is_some_and(|revision| payload.host_revision < revision)
        {
            return Err(GuiError::host(format!(
                "TSX committed host revision {} is older than active host revision {}",
                payload.host_revision,
                self.committed_host_revision.unwrap_or_default()
            )));
        }

        let message_id = self.host_messages.expected_message_id()?;
        let message = TsxHostMessageV1::committed(
            self.session_id.clone(),
            message_id,
            pending.render_revision,
            payload.clone(),
        );
        message.validate()?;
        encode_tsx_json_frame_v1(&message, self.limits)?;

        self.host_messages.accept(message_id)?;
        self.committed_render_revision = pending.render_revision;
        self.committed_host_revision = Some(payload.host_revision);
        self.pending_render = None;
        Ok(message)
    }

    /// Emits one complete ordered self-drawn dispatch against the currently
    /// active callback scope.
    pub fn emit_event(&mut self, payload: TsxEventPayloadV1) -> GuiResult<TsxHostMessageV1> {
        payload.validate()?;
        if self.committed_render_revision == 0 {
            return Err(GuiError::host(
                "TSX events require a committed render revision",
            ));
        }
        let active_host_revision = self.committed_host_revision.ok_or_else(|| {
            GuiError::host("TSX application session has no committed host revision")
        })?;
        if payload.host_revision != active_host_revision {
            return Err(GuiError::host(format!(
                "TSX event host revision {} is stale; active host revision is {active_host_revision}",
                payload.host_revision
            )));
        }
        let expected_event_sequence = self
            .last_event_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("TSX event sequence overflowed"))?;
        if payload.event_sequence != expected_event_sequence {
            return Err(GuiError::host(format!(
                "TSX event sequence {} is invalid; expected {expected_event_sequence}",
                payload.event_sequence
            )));
        }

        let message_id = self.host_messages.expected_message_id()?;
        let message = TsxHostMessageV1::event(
            self.session_id.clone(),
            message_id,
            self.committed_render_revision,
            payload.clone(),
        );
        message.validate()?;
        encode_tsx_json_frame_v1(&message, self.limits)?;

        self.host_messages.accept(message_id)?;
        self.last_event_sequence = payload.event_sequence;
        Ok(message)
    }

    fn validate_client_identity(&self, session_id: &str, message_id: u64) -> GuiResult<()> {
        if session_id != self.session_id {
            return Err(GuiError::host(format!(
                "TSX message session id {session_id:?} does not match negotiated session {:?}",
                self.session_id
            )));
        }
        let expected = self.client_messages.expected_message_id()?;
        if message_id != expected {
            return Err(GuiError::host(format!(
                "TSX protocol message id {message_id} is invalid; expected {expected}"
            )));
        }
        Ok(())
    }

    fn validate_control(&self, message: &TsxClientMessageV1) -> GuiResult<()> {
        message.validate()?;
        if matches!(
            message,
            TsxClientMessageV1::Hello { .. } | TsxClientMessageV1::Render { .. }
        ) {
            return Err(GuiError::host(
                "TSX application control path accepts only ping, pong, or close",
            ));
        }
        let metadata = message.metadata();
        self.validate_client_identity(metadata.session_id, metadata.message_id)?;
        if metadata.render_revision != self.committed_render_revision {
            return Err(GuiError::host(format!(
                "TSX control message render revision {} is stale; active revision is {}",
                metadata.render_revision, self.committed_render_revision
            )));
        }
        Ok(())
    }
}

fn compiled_key(node: &CompiledRsxNode) -> &str {
    match node {
        CompiledRsxNode::Element { key, .. } | CompiledRsxNode::Text { key, .. } => key,
    }
}

#[cfg(feature = "platform-runtime")]
impl TsxHostApplicationSessionV1 {
    pub fn commit_self_drawn_snapshot(
        &mut self,
        frame_id: impl Into<String>,
        snapshot: &crate::platform_runtime::SelfDrawnFrameSnapshot,
        diagnostics: Vec<super::TsxDiagnosticV1>,
    ) -> GuiResult<TsxHostMessageV1> {
        let payload =
            TsxCommittedPayloadV1::from_self_drawn_snapshot(frame_id, snapshot, diagnostics)?;
        self.commit_pending_render(payload)
    }

    pub fn emit_self_drawn_event(
        &mut self,
        dispatch: &crate::platform_runtime::SelfDrawnInputDispatch,
    ) -> GuiResult<TsxHostMessageV1> {
        self.emit_event(dispatch.try_into()?)
    }
}
