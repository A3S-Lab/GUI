use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProtocolSessionMode {
    #[default]
    Unbound,
    Legacy,
    StrictV1,
}

/// Transport-owned native session.
///
/// Once v1 is selected, the caller must acknowledge each retained render batch
/// before dispatching ordered events or producing the next render revision.
/// Reducer/effect orchestration remains application-owned.
pub struct NativeProtocolSession<A: PlatformAdapter> {
    runtime: GuiRuntime<PlatformPlanningHost<A>>,
    mode: ProtocolSessionMode,
    session_id: String,
    render_revision: u64,
    last_event_sequence: u64,
    next_command_sequence: u64,
    pending_render: Option<ProtocolRenderResponseV1>,
    active_frame_id: Option<String>,
    root: Option<HostNodeId>,
}

impl<A: PlatformAdapter> std::fmt::Debug for NativeProtocolSession<A> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NativeProtocolSession")
            .field("mode", &self.mode)
            .field("session_id", &self.session_id)
            .field("render_revision", &self.render_revision)
            .field("last_event_sequence", &self.last_event_sequence)
            .field("next_command_sequence", &self.next_command_sequence)
            .field("pending_command_ack", &self.pending_command_ack())
            .field("active_frame_id", &self.active_frame_id)
            .field("root", &self.root)
            .finish_non_exhaustive()
    }
}

impl<A: PlatformAdapter> NativeProtocolSession<A> {
    pub fn new(adapter: A) -> Self {
        let id = NEXT_PROTOCOL_SESSION_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            runtime: GuiRuntime::new(PlatformPlanningHost::new(adapter)),
            mode: ProtocolSessionMode::Unbound,
            session_id: format!("a3s-native-{id}"),
            render_revision: 0,
            last_event_sequence: 0,
            next_command_sequence: 0,
            pending_render: None,
            active_frame_id: None,
            root: None,
        }
    }

    pub fn new_with_session_id(adapter: A, session_id: impl Into<String>) -> GuiResult<Self> {
        let session_id = session_id.into();
        if session_id.trim().is_empty() {
            return Err(GuiError::host(
                "version-1 protocol sessions require a non-empty session id",
            ));
        }
        Ok(Self {
            runtime: GuiRuntime::new(PlatformPlanningHost::new(adapter)),
            mode: ProtocolSessionMode::Unbound,
            session_id,
            render_revision: 0,
            last_event_sequence: 0,
            next_command_sequence: 0,
            pending_render: None,
            active_frame_id: None,
            root: None,
        })
    }

    pub fn runtime(&self) -> &GuiRuntime<PlatformPlanningHost<A>> {
        &self.runtime
    }

    pub fn active_frame_id(&self) -> Option<&str> {
        self.active_frame_id.as_deref()
    }

    pub fn mode(&self) -> ProtocolSessionMode {
        self.mode
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn render_revision(&self) -> u64 {
        self.render_revision
    }

    pub fn last_event_sequence(&self) -> u64 {
        self.last_event_sequence
    }

    pub fn pending_command_ack(&self) -> Option<(u64, u64)> {
        self.pending_render.as_ref().map(|pending| {
            (
                pending.metadata.render_revision,
                pending.payload.command_sequence,
            )
        })
    }

    /// Returns the retained command batch until its exact acknowledgement arrives.
    pub fn pending_render_v1(&self) -> Option<&ProtocolRenderResponseV1> {
        self.pending_render.as_ref()
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        let mut tree = self.runtime.accessibility_tree()?;
        redact_internal_accessibility_nodes(&mut tree, &self.sensitive_node_ids());
        Some(tree)
    }

    pub fn render_frame(&mut self, frame: &UiFrame) -> GuiResult<NativeRenderResponse> {
        self.enter_legacy_mode()?;
        if self.pending_render.is_some() {
            return Err(GuiError::host(
                "cannot use the legacy render API while a version-1 command batch awaits acknowledgement",
            ));
        }
        let revision = self
            .render_revision
            .checked_add(1)
            .ok_or_else(|| GuiError::host("native render revision overflow"))?;
        let response = self.render_frame_internal(frame)?;
        self.render_revision = revision;
        Ok(response)
    }

    fn render_frame_internal(&mut self, frame: &UiFrame) -> GuiResult<NativeRenderResponse> {
        let rendered = frame.render_into(&mut self.runtime)?;
        self.active_frame_id = Some(rendered.frame_id.clone());
        self.root = Some(rendered.root);
        let commands = self.take_pending_commands_internal();
        let accessibility_tree = self.accessibility_tree();
        Ok(NativeRenderResponse {
            frame_id: rendered.frame_id,
            root: rendered.root,
            commands,
            accessibility_tree,
        })
    }

    pub fn render_v1(
        &mut self,
        request: &ProtocolRenderRequestV1,
    ) -> GuiResult<ProtocolRenderResponseV1> {
        self.validate_v1_metadata(&request.metadata)?;
        self.enter_v1_mode()?;
        if request.metadata.event_sequence.is_some() {
            return Err(GuiError::host(
                "version-1 render messages cannot carry an event sequence",
            ));
        }
        if let Some(pending) = &self.pending_render {
            if pending.metadata.render_revision == request.metadata.render_revision {
                // The render revision is the idempotency key. While its batch is
                // unacknowledged, retries always receive the original response;
                // a changed payload cannot overwrite the committed revision.
                return Ok(pending.clone());
            }
            let revision = pending.metadata.render_revision;
            let sequence = pending.payload.command_sequence;
            return Err(GuiError::host(format!(
                "render revision {revision} command batch {sequence} still awaits acknowledgement"
            )));
        }
        let expected_revision = self
            .render_revision
            .checked_add(1)
            .ok_or_else(|| GuiError::host("native render revision overflow"))?;
        if request.metadata.render_revision != expected_revision {
            return Err(GuiError::host(format!(
                "version-1 render revision {} is invalid; expected {}",
                request.metadata.render_revision, expected_revision
            )));
        }

        let command_sequence = self
            .next_command_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("native command sequence overflow"))?;
        let frame = UiFrame::try_from(request.payload.clone())?;
        let rendered = self.render_frame_internal(&frame)?;
        let commands = rendered
            .commands
            .iter()
            .map(ProtocolCommandV1::from)
            .collect();
        let sensitive_nodes = self.sensitive_node_ids();
        let accessibility_tree = rendered.accessibility_tree.as_ref().map(|tree| {
            let mut tree: ProtocolAccessibilityNodeV1 = tree.into();
            redact_sensitive_accessibility_nodes(&mut tree, &sensitive_nodes);
            tree
        });

        let response = ProtocolEnvelopeV1::new(
            ProtocolMetadataV1::render(&self.session_id, expected_revision),
            ProtocolRenderPayloadV1 {
                frame_id: rendered.frame_id,
                root: rendered.root.get(),
                command_sequence,
                commands,
                accessibility_tree,
            },
        );

        self.render_revision = expected_revision;
        self.next_command_sequence = command_sequence;
        self.pending_render = Some(response.clone());

        Ok(response)
    }

    pub fn acknowledge_render_v1(&mut self, ack: &ProtocolRenderAckV1) -> GuiResult<()> {
        self.validate_v1_metadata(&ack.metadata)?;
        self.enter_v1_mode()?;
        if ack.metadata.event_sequence.is_some() {
            return Err(GuiError::host(
                "version-1 render acknowledgements cannot carry an event sequence",
            ));
        }
        let Some(pending) = self.pending_render.as_ref() else {
            return Err(GuiError::host(
                "no version-1 render command batch awaits acknowledgement",
            ));
        };
        let pending_revision = pending.metadata.render_revision;
        let pending_sequence = pending.payload.command_sequence;
        if ack.metadata.render_revision != pending_revision {
            return Err(GuiError::host(format!(
                "render acknowledgement revision {} does not match pending revision {}",
                ack.metadata.render_revision, pending_revision
            )));
        }
        if ack.payload.command_sequence != pending_sequence {
            return Err(GuiError::host(format!(
                "render acknowledgement command sequence {} does not match pending sequence {}",
                ack.payload.command_sequence, pending_sequence
            )));
        }
        self.pending_render = None;
        Ok(())
    }

    pub fn handle_host_event_v1(
        &mut self,
        request: &ProtocolHostEventV1,
    ) -> GuiResult<ProtocolHostEventResponseV1> {
        self.validate_v1_metadata(&request.metadata)?;
        self.enter_v1_mode()?;
        if let Some(pending) = &self.pending_render {
            let revision = pending.metadata.render_revision;
            let sequence = pending.payload.command_sequence;
            return Err(GuiError::host(format!(
                "cannot accept events before render revision {revision} command batch {sequence} is acknowledged"
            )));
        }
        if request.metadata.render_revision != self.render_revision {
            return Err(GuiError::host(format!(
                "event render revision {} is stale; active revision is {}",
                request.metadata.render_revision, self.render_revision
            )));
        }
        let event_sequence = request
            .metadata
            .event_sequence
            .ok_or_else(|| GuiError::host("version-1 host events require an event sequence"))?;
        let expected_sequence = self
            .last_event_sequence
            .checked_add(1)
            .ok_or_else(|| GuiError::host("native event sequence overflow"))?;
        if event_sequence != expected_sequence {
            return Err(GuiError::host(format!(
                "version-1 event sequence {event_sequence} is invalid; expected {expected_sequence}"
            )));
        }

        let event = HostEvent::try_from(request.payload.clone())?;
        let response = self.handle_host_event_internal(&event)?;
        let payload = protocol_event_payload(&response, &self.sensitive_node_ids())?;
        self.last_event_sequence = event_sequence;
        Ok(ProtocolEnvelopeV1::new(
            ProtocolMetadataV1::event(&self.session_id, self.render_revision, event_sequence),
            payload,
        ))
    }

    pub fn dispatch_host_event(&mut self, event: &HostEvent) -> GuiResult<HostEventResponse> {
        self.enter_legacy_mode()?;
        self.dispatch_host_event_internal(event)
    }

    fn dispatch_host_event_internal(&mut self, event: &HostEvent) -> GuiResult<HostEventResponse> {
        event.validate()?;
        self.ensure_active_frame(event)?;
        event.dispatch_into(&mut self.runtime)
    }

    pub fn handle_host_event(&mut self, event: &HostEvent) -> GuiResult<NativeHostEventResponse> {
        self.enter_legacy_mode()?;
        let mut response = self.handle_host_event_internal(event)?;
        if let Some(tree) = response.accessibility_tree.as_mut() {
            redact_internal_accessibility_nodes(tree, &self.sensitive_node_ids());
        }
        Ok(response)
    }

    fn handle_host_event_internal(
        &mut self,
        event: &HostEvent,
    ) -> GuiResult<NativeHostEventResponse> {
        event.validate()?;
        self.ensure_active_frame(event)?;
        event.handle_into(&mut self.runtime)
    }

    pub fn pending_commands(&mut self) -> Vec<PlatformCommand> {
        if self.mode == ProtocolSessionMode::StrictV1 {
            return Vec::new();
        }
        self.mode = ProtocolSessionMode::Legacy;
        self.take_pending_commands_internal()
    }

    fn take_pending_commands_internal(&mut self) -> Vec<PlatformCommand> {
        self.runtime.host_mut().take_commands()
    }

    fn ensure_active_frame(&self, event: &HostEvent) -> GuiResult<()> {
        let active_frame_id = self
            .active_frame_id
            .as_deref()
            .ok_or_else(|| crate::error::GuiError::host("no active native frame"))?;
        if event.frame_id != active_frame_id {
            return Err(crate::error::GuiError::host(format!(
                "native event for frame {} cannot be dispatched into active frame {}",
                event.frame_id, active_frame_id
            )));
        }
        Ok(())
    }

    fn validate_v1_metadata(&self, metadata: &ProtocolMetadataV1) -> GuiResult<()> {
        if metadata.protocol_version != NATIVE_PROTOCOL_VERSION_V1 {
            return Err(GuiError::host(format!(
                "unsupported native protocol version {}; expected {}",
                metadata.protocol_version, NATIVE_PROTOCOL_VERSION_V1
            )));
        }
        if metadata.session_id != self.session_id {
            return Err(GuiError::host(format!(
                "native protocol session {:?} does not match active session {:?}",
                metadata.session_id, self.session_id
            )));
        }
        Ok(())
    }

    fn sensitive_node_ids(&self) -> BTreeSet<u64> {
        self.runtime
            .host()
            .nodes()
            .iter()
            .filter_map(|(id, node)| {
                effective_blueprint_value_sensitivity(&node.blueprint)
                    .is_sensitive()
                    .then_some(id.get())
            })
            .collect()
    }

    fn enter_legacy_mode(&mut self) -> GuiResult<()> {
        match self.mode {
            ProtocolSessionMode::Unbound => {
                self.mode = ProtocolSessionMode::Legacy;
                Ok(())
            }
            ProtocolSessionMode::Legacy => Ok(()),
            ProtocolSessionMode::StrictV1 => Err(GuiError::host(
                "legacy native protocol APIs are disabled after protocol v1 is selected",
            )),
        }
    }

    fn enter_v1_mode(&mut self) -> GuiResult<()> {
        match self.mode {
            ProtocolSessionMode::Unbound => {
                self.mode = ProtocolSessionMode::StrictV1;
                Ok(())
            }
            ProtocolSessionMode::StrictV1 => Ok(()),
            ProtocolSessionMode::Legacy => Err(GuiError::host(
                "protocol v1 APIs are disabled after the legacy protocol is selected",
            )),
        }
    }
}

fn protocol_event_payload(
    response: &NativeHostEventResponse,
    sensitive_nodes: &BTreeSet<u64>,
) -> GuiResult<ProtocolEventPayloadV1> {
    let mut invocation = response.invocation.clone();
    let mut interaction_changes = response.interaction_changes.clone();
    redact_response_values(
        invocation.as_mut(),
        &mut interaction_changes,
        response.value_sensitivity,
    );
    let invocation = invocation.map(|invocation| ProtocolActionInvocationV1 {
        node: invocation.node.get(),
        action: invocation.action,
        event: invocation.event.into(),
        value: invocation.value,
    });
    let interaction_changes = interaction_changes
        .into_iter()
        .map(|change| ProtocolInteractionChangeV1 {
            node: change.node.get(),
            before: protocol_interaction_state(change.before),
            after: protocol_interaction_state(change.after),
        })
        .collect();
    let accessibility_tree = response.accessibility_tree.as_ref().map(|tree| {
        let mut tree: ProtocolAccessibilityNodeV1 = tree.into();
        redact_sensitive_accessibility_nodes(&mut tree, sensitive_nodes);
        tree
    });
    Ok(ProtocolEventPayloadV1 {
        frame_id: response.frame_id.clone(),
        invocation,
        interaction_changes,
        accessibility_tree,
    })
}

fn redact_sensitive_accessibility_nodes(
    node: &mut ProtocolAccessibilityNodeV1,
    sensitive_nodes: &BTreeSet<u64>,
) {
    if node.node.is_some_and(|id| sensitive_nodes.contains(&id)) {
        node.value = None;
        node.description.value_text = None;
    }
    for child in &mut node.children {
        redact_sensitive_accessibility_nodes(child, sensitive_nodes);
    }
}

fn redact_internal_accessibility_nodes(
    node: &mut AccessibilityNode,
    sensitive_nodes: &BTreeSet<u64>,
) {
    if node
        .node
        .is_some_and(|id| sensitive_nodes.contains(&id.get()))
    {
        node.value = None;
        node.description.value_text = None;
        node.value_sensitivity = ValueSensitivity::Sensitive;
    }
    for child in &mut node.children {
        redact_internal_accessibility_nodes(child, sensitive_nodes);
    }
}

fn protocol_interaction_state(
    state: crate::interaction::InteractionNodeState,
) -> ProtocolInteractionStateV1 {
    ProtocolInteractionStateV1 {
        focused: state.focused,
        pressed: state.pressed,
        value: state.value,
        selected: state.selected,
        checked: state.checked,
        expanded: state.expanded,
    }
}

impl<A: PlatformAdapter + Default> Default for NativeProtocolSession<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}
