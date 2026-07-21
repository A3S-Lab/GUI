use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::backend::NativeEventHost;
use crate::effect::EffectWaker;
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, NativeEvent};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionChange;
use crate::native::ValueSensitivity;
use crate::platform::BlueprintHost;
use crate::protocol::{RenderedFrame, UiFrame};
use crate::runtime::{redact_event_output, GuiRuntime};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeRuntimeEventResponse {
    pub frame_id: String,
    pub event: NativeEvent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ActionInvocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<RenderedFrame>,
    #[serde(default, skip)]
    pub value_sensitivity: ValueSensitivity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeRuntimeEventResponseWire {
    frame_id: String,
    event: NativeEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation: Option<ActionInvocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_tree: Option<AccessibilityNode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    interaction_changes: Vec<InteractionChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render: Option<RenderedFrame>,
}

impl Serialize for NativeRuntimeEventResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut event = self.event.clone();
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_event_output(
            &mut event,
            invocation.as_mut(),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        let mut accessibility_tree = self.accessibility_tree.clone();
        redact_accessibility_value(
            accessibility_tree.as_mut(),
            self.event.node,
            self.value_sensitivity,
        );
        NativeRuntimeEventResponseWire {
            frame_id: self.frame_id.clone(),
            event,
            invocation,
            accessibility_tree,
            interaction_changes,
            render: self.render.clone(),
        }
        .serialize(serializer)
    }
}

fn redact_accessibility_value(
    node: Option<&mut AccessibilityNode>,
    target: HostNodeId,
    sensitivity: ValueSensitivity,
) {
    let Some(node) = node else {
        return;
    };
    if node.node == Some(target) && sensitivity.is_sensitive() {
        node.value = None;
        node.description.value_text = None;
        node.value_sensitivity = ValueSensitivity::Sensitive;
    }
    for child in &mut node.children {
        redact_accessibility_value(Some(child), target, sensitivity);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeRuntimeEventBatch {
    pub responses: Vec<NativeRuntimeEventResponse>,
    pub host_events_drained: usize,
    pub queued_native_events: usize,
    pub handled_native_events: usize,
    pub buffered_native_events: usize,
    pub stopped_by_predicate: bool,
    pub host_queue_preserved: bool,
}

impl NativeRuntimeEventBatch {
    pub fn extend(&mut self, next: Self) {
        self.host_events_drained += next.host_events_drained;
        self.queued_native_events += next.queued_native_events;
        self.handled_native_events += next.handled_native_events;
        self.buffered_native_events = next.buffered_native_events;
        self.stopped_by_predicate |= next.stopped_by_predicate;
        self.host_queue_preserved |= next.host_queue_preserved;
        self.responses.extend(next.responses);
    }
}

/// Outcome of polling application-owned background work on the UI thread.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BackgroundUpdate {
    pub state_changed: bool,
    pub work_pending: bool,
}

impl BackgroundUpdate {
    pub const fn idle(work_pending: bool) -> Self {
        Self {
            state_changed: false,
            work_pending,
        }
    }

    pub const fn changed(work_pending: bool) -> Self {
        Self {
            state_changed: true,
            work_pending,
        }
    }
}

type BackgroundUpdatePoller<S> = Box<dyn FnMut(&mut S) -> GuiResult<BackgroundUpdate>>;

pub struct NativeRuntimeApp<H: NativeHost, S, F, R> {
    runtime: GuiRuntime<H>,
    state: S,
    frame_builder: F,
    action_reducer: R,
    render_effect: Option<Box<dyn FnMut(&mut S) -> GuiResult<()>>>,
    cleanup_effect: Option<Box<dyn FnMut(&mut S) -> GuiResult<()>>>,
    background_updates: Option<BackgroundUpdatePoller<S>>,
    background_work_pending: bool,
    active_frame_id: Option<String>,
    root: Option<HostNodeId>,
    pending_native_events: VecDeque<NativeEvent>,
}

impl<H, S, F, R> std::fmt::Debug for NativeRuntimeApp<H, S, F, R>
where
    H: NativeHost,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeRuntimeApp")
            .field("state_type", &std::any::type_name::<S>())
            .field("active_frame_id", &self.active_frame_id)
            .field("root", &self.root)
            .field(
                "pending_native_event_count",
                &self.pending_native_events.len(),
            )
            .field("has_render_effect", &self.render_effect.is_some())
            .field("has_cleanup_effect", &self.cleanup_effect.is_some())
            .field("has_background_updates", &self.background_updates.is_some())
            .field("background_work_pending", &self.background_work_pending)
            .finish_non_exhaustive()
    }
}

impl<H, S, F, R> NativeRuntimeApp<H, S, F, R>
where
    H: NativeHost,
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    pub fn new(host: H, state: S, frame_builder: F, action_reducer: R) -> Self {
        Self::from_runtime(GuiRuntime::new(host), state, frame_builder, action_reducer)
    }

    pub fn from_runtime(
        runtime: GuiRuntime<H>,
        state: S,
        frame_builder: F,
        action_reducer: R,
    ) -> Self {
        Self {
            runtime,
            state,
            frame_builder,
            action_reducer,
            render_effect: None,
            cleanup_effect: None,
            background_updates: None,
            background_work_pending: false,
            active_frame_id: None,
            root: None,
            pending_native_events: VecDeque::new(),
        }
    }

    pub fn with_render_effect(
        mut self,
        effect: impl FnMut(&mut S) -> GuiResult<()> + 'static,
    ) -> Self {
        self.render_effect = Some(Box::new(effect));
        self
    }

    pub fn with_cleanup_effect(
        mut self,
        cleanup: impl FnMut(&mut S) -> GuiResult<()> + 'static,
    ) -> Self {
        self.cleanup_effect = Some(Box::new(cleanup));
        self
    }

    /// Installs a UI-thread poller that merges completed background work into state.
    ///
    /// The poller should drain every currently available completion before returning.
    /// A changed batch is rendered exactly once.
    pub fn with_background_updates(
        mut self,
        poller: impl FnMut(&mut S) -> GuiResult<BackgroundUpdate> + 'static,
    ) -> Self {
        self.background_updates = Some(Box::new(poller));
        self
    }

    pub fn runtime(&self) -> &GuiRuntime<H> {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut GuiRuntime<H> {
        &mut self.runtime
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn active_frame_id(&self) -> Option<&str> {
        self.active_frame_id.as_deref()
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn pending_native_event_count(&self) -> usize {
        self.pending_native_events.len()
    }

    pub fn has_pending_native_events(&self) -> bool {
        !self.pending_native_events.is_empty()
    }

    pub fn has_pending_background_work(&self) -> bool {
        self.background_work_pending
    }

    /// Builds a waker for effects owned by this app's UI thread.
    ///
    /// Native pending-work loops park briefly between non-blocking OS polls;
    /// completion wakes that parked thread immediately. Call this from the UI
    /// thread before moving the waker into an executor.
    pub fn background_effect_waker(&self) -> EffectWaker {
        let ui_thread = std::thread::current();
        EffectWaker::new(move || ui_thread.unpark())
    }

    /// Drains background completions and renders one frame when the state changed.
    pub fn poll_background_updates(&mut self) -> GuiResult<Option<RenderedFrame>> {
        let Some(poller) = self.background_updates.as_mut() else {
            self.background_work_pending = false;
            return Ok(None);
        };
        let update = poller(&mut self.state)?;
        self.background_work_pending = update.work_pending;
        if update.state_changed {
            self.render().map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn render(&mut self) -> GuiResult<RenderedFrame> {
        let frame = (self.frame_builder)(&self.state)?;
        let rendered = frame.render_into(&mut self.runtime)?;
        self.active_frame_id = Some(rendered.frame_id.clone());
        self.root = Some(rendered.root);
        if let Some(effect) = self.render_effect.as_mut() {
            effect(&mut self.state)?;
        }
        Ok(rendered)
    }

    pub fn cleanup_effects(&mut self) -> GuiResult<()> {
        if let Some(cleanup) = self.cleanup_effect.as_mut() {
            cleanup(&mut self.state)?;
        }
        Ok(())
    }

    pub fn into_parts(self) -> (GuiRuntime<H>, S, F, R) {
        (
            self.runtime,
            self.state,
            self.frame_builder,
            self.action_reducer,
        )
    }
}

impl<H, S, F, R> NativeRuntimeApp<H, S, F, R>
where
    H: NativeHost + BlueprintHost + AccessibilityTreeHost,
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    pub fn dispatch_native_event(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<NativeRuntimeEventResponse> {
        let response = self.handle_native_event(event)?;
        if response.invocation.is_none() {
            return Err(GuiError::host("native event has no registered RSX action"));
        }
        Ok(response)
    }

    pub fn handle_native_event(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<NativeRuntimeEventResponse> {
        let frame_id = self
            .active_frame_id
            .clone()
            .ok_or_else(|| GuiError::host("no active native frame"))?;
        let handled = self.runtime.handle_native_event_with_changes(event)?;
        let mut render = None;
        let mut accessibility_tree = self.runtime.accessibility_tree();

        if let Some(invocation) = handled.invocation.as_ref() {
            (self.action_reducer)(&mut self.state, invocation)?;
            let rendered = self.render()?;
            accessibility_tree = self.runtime.accessibility_tree();
            render = Some(rendered);
        }

        redact_accessibility_value(
            accessibility_tree.as_mut(),
            handled.event.node,
            handled.value_sensitivity,
        );

        Ok(NativeRuntimeEventResponse {
            frame_id,
            event: handled.event,
            invocation: handled.invocation,
            accessibility_tree,
            interaction_changes: handled.interaction_changes,
            render,
            value_sensitivity: handled.value_sensitivity,
        })
    }
}

impl<H, S, F, R> NativeRuntimeApp<H, S, F, R>
where
    H: NativeHost + BlueprintHost + AccessibilityTreeHost + NativeEventHost,
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    pub fn handle_pending_native_events(&mut self) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.handle_pending_native_event_batch()
            .map(|batch| batch.responses)
    }

    pub fn handle_pending_native_events_while(
        &mut self,
        should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.handle_pending_native_event_batch_while(should_continue)
            .map(|batch| batch.responses)
    }

    pub fn handle_pending_native_event_batch(&mut self) -> GuiResult<NativeRuntimeEventBatch> {
        self.handle_pending_native_event_batch_while(|_| true)
    }

    pub fn handle_pending_native_event_batch_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        if !should_continue(&self.state) {
            let buffered_native_events = self.pending_native_events.len();
            return Ok(NativeRuntimeEventBatch {
                responses: Vec::new(),
                host_events_drained: 0,
                queued_native_events: buffered_native_events,
                handled_native_events: 0,
                buffered_native_events,
                stopped_by_predicate: true,
                host_queue_preserved: true,
            });
        }

        let host_events = self.runtime.host_mut().take_native_events();
        let host_events_drained = host_events.len();
        self.pending_native_events.extend(host_events);
        let queued_native_events = self.pending_native_events.len();
        let mut responses = Vec::with_capacity(queued_native_events);
        let mut stopped_by_predicate = false;

        while let Some(event) = self.pending_native_events.pop_front() {
            responses.push(self.handle_native_event(event)?);
            if !should_continue(&self.state) {
                stopped_by_predicate = true;
                break;
            }
        }

        Ok(NativeRuntimeEventBatch {
            handled_native_events: responses.len(),
            buffered_native_events: self.pending_native_events.len(),
            responses,
            host_events_drained,
            queued_native_events,
            stopped_by_predicate,
            host_queue_preserved: false,
        })
    }
}

#[cfg(test)]
#[path = "app/tests.rs"]
mod tests;
