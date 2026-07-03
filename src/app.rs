use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::backend::NativeEventHost;
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, NativeEvent};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionChange;
use crate::platform::BlueprintHost;
use crate::protocol::{RenderedFrame, UiFrame};
use crate::runtime::GuiRuntime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Debug)]
pub struct NativeRuntimeApp<H: NativeHost, S, F, R> {
    runtime: GuiRuntime<H>,
    state: S,
    frame_builder: F,
    action_reducer: R,
    active_frame_id: Option<String>,
    root: Option<HostNodeId>,
    pending_native_events: VecDeque<NativeEvent>,
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
            active_frame_id: None,
            root: None,
            pending_native_events: VecDeque::new(),
        }
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

    pub fn render(&mut self) -> GuiResult<RenderedFrame> {
        let frame = (self.frame_builder)(&self.state)?;
        let rendered = frame.render_into(&mut self.runtime)?;
        self.active_frame_id = Some(rendered.frame_id.clone());
        self.root = Some(rendered.root);
        Ok(rendered)
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
            return Err(GuiError::host("native event has no registered Web action"));
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

        Ok(NativeRuntimeEventResponse {
            frame_id,
            event: handled.event,
            invocation: handled.invocation,
            accessibility_tree,
            interaction_changes: handled.interaction_changes,
            render,
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
        self.pending_native_events
            .extend(self.runtime.host_mut().take_native_events());
        let mut responses = Vec::with_capacity(self.pending_native_events.len());
        while let Some(event) = self.pending_native_events.pop_front() {
            responses.push(self.handle_native_event(event)?);
        }
        Ok(responses)
    }

    pub fn handle_pending_native_events_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        if !should_continue(&self.state) {
            return Ok(Vec::new());
        }
        self.pending_native_events
            .extend(self.runtime.host_mut().take_native_events());
        let mut responses = Vec::with_capacity(self.pending_native_events.len());
        while let Some(event) = self.pending_native_events.pop_front() {
            responses.push(self.handle_native_event(event)?);
            if !should_continue(&self.state) {
                break;
            }
        }
        Ok(responses)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::backend::{CommandExecutingHost, NativeEventSource, RecordingBackend};
    use crate::event::NativeEventKind;
    use crate::platform::Gtk4Adapter;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct CounterState {
        count: u32,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct ClosingState {
        closed: bool,
        increments: u32,
    }

    fn counter_frame(state: &CounterState) -> GuiResult<UiFrame> {
        serde_json::from_value(json!({
            "frameId": "counter",
            "actions": [{"id": "increment"}],
            "root": {
                "kind": "element",
                "key": "counter-button",
                "tag": "Button",
                "props": {
                    "label": format!("Count {}", state.count),
                    "events": {"onPress": "increment"}
                }
            }
        }))
        .map_err(|error| GuiError::invalid_tree(format!("invalid counter frame: {error}")))
    }

    fn counter_reduce(state: &mut CounterState, invocation: &ActionInvocation) -> GuiResult<()> {
        match invocation.action.as_str() {
            "increment" => {
                state.count += 1;
                Ok(())
            }
            other => Err(GuiError::host(format!("unexpected action {other}"))),
        }
    }

    fn closing_frame(state: &ClosingState) -> GuiResult<UiFrame> {
        serde_json::from_value(json!({
            "frameId": "closing",
            "window": {"title": "Closing", "onClose": "close"},
            "actions": [{"id": "close"}, {"id": "increment"}],
            "root": {
                "kind": "element",
                "key": "increment-button",
                "tag": "Button",
                "props": {
                    "label": format!("Increments {}", state.increments),
                    "events": {"onPress": "increment"}
                }
            }
        }))
        .map_err(|error| GuiError::invalid_tree(format!("invalid closing frame: {error}")))
    }

    fn closing_reduce(state: &mut ClosingState, invocation: &ActionInvocation) -> GuiResult<()> {
        match invocation.action.as_str() {
            "close" => {
                state.closed = true;
                Ok(())
            }
            "increment" => {
                state.increments += 1;
                Ok(())
            }
            other => Err(GuiError::host(format!("unexpected action {other}"))),
        }
    }

    #[test]
    fn native_runtime_app_drains_native_events_reduces_actions_and_renders() {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut app =
            NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce);
        let rendered = app.render().unwrap();

        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press));

        let responses = app.handle_pending_native_events().unwrap();

        assert_eq!(app.state().count, 1);
        assert_eq!(responses.len(), 1);
        assert_eq!(
            responses[0]
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("increment")
        );
        assert_eq!(
            responses[0].render.as_ref().map(|render| render.root),
            Some(rendered.root)
        );
        assert_eq!(
            app.runtime()
                .host()
                .executor()
                .object(rendered.root)
                .and_then(|object| object.label.as_deref()),
            Some("Count 1")
        );
    }

    #[test]
    fn native_runtime_app_handles_state_only_events_without_rerendering() {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut app =
            NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce);
        let rendered = app.render().unwrap();

        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Focus));

        let responses = app.handle_pending_native_events().unwrap();

        assert_eq!(app.state().count, 0);
        assert_eq!(responses.len(), 1);
        assert!(responses[0].invocation.is_none());
        assert!(responses[0].render.is_none());
        assert_eq!(responses[0].interaction_changes.len(), 1);
        assert!(responses[0].accessibility_tree.as_ref().unwrap().focused);
    }

    #[test]
    fn native_runtime_app_stops_draining_pending_events_when_predicate_fails() {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut app =
            NativeRuntimeApp::new(host, ClosingState::default(), closing_frame, closing_reduce);
        let rendered = app.render().unwrap();
        let increment = app
            .runtime()
            .host()
            .planning()
            .nodes()
            .iter()
            .find_map(|(id, node)| {
                (node.blueprint.events.get("onPress").map(String::as_str) == Some("increment"))
                    .then_some(*id)
            })
            .unwrap();

        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Close));
        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(increment, NativeEventKind::Press));

        let responses = app
            .handle_pending_native_events_while(|state| !state.closed)
            .unwrap();

        assert!(app.state().closed);
        assert_eq!(app.state().increments, 0);
        assert_eq!(responses.len(), 1);
        assert_eq!(
            responses[0]
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("close")
        );
    }

    #[test]
    fn native_runtime_app_buffers_events_after_predicate_stops() {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut app =
            NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce);
        let rendered = app.render().unwrap();

        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press));
        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press));

        let first = app
            .handle_pending_native_events_while(|state| state.count < 1)
            .unwrap();
        assert_eq!(app.state().count, 1);
        assert_eq!(first.len(), 1);

        let second = app.handle_pending_native_events().unwrap();

        assert_eq!(app.state().count, 2);
        assert_eq!(second.len(), 1);
        assert_eq!(
            second[0]
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("increment")
        );
    }

    #[test]
    fn native_runtime_app_keeps_pending_events_when_predicate_starts_false() {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        let mut app =
            NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce);
        let rendered = app.render().unwrap();

        app.runtime_mut()
            .host_mut()
            .executor_mut()
            .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press));

        let responses = app.handle_pending_native_events_while(|_| false).unwrap();

        assert!(responses.is_empty());
        assert_eq!(app.state().count, 0);
        let pending = app
            .runtime_mut()
            .host_mut()
            .executor_mut()
            .take_native_events();
        assert_eq!(
            pending,
            vec![NativeEvent::new(rendered.root, NativeEventKind::Press)]
        );
    }
}
