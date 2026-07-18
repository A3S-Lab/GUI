use std::cell::Cell;
use std::rc::Rc;

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

#[derive(Debug, Clone, PartialEq, Default)]
struct QueueState {
    handled: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct RemovingState {
    removed: bool,
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

fn queue_frame(_state: &QueueState) -> GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "queue",
        "actions": [{"id": "first"}, {"id": "second"}, {"id": "third"}],
        "root": {
            "kind": "element",
            "key": "queue-toolbar",
            "tag": "Toolbar",
            "children": [
                {
                    "kind": "element",
                    "key": "first",
                    "tag": "Button",
                    "props": {"label": "First", "events": {"onPress": "first"}}
                },
                {
                    "kind": "element",
                    "key": "second",
                    "tag": "Button",
                    "props": {"label": "Second", "events": {"onPress": "second"}}
                },
                {
                    "kind": "element",
                    "key": "third",
                    "tag": "Button",
                    "props": {"label": "Third", "events": {"onPress": "third"}}
                }
            ]
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid queue frame: {error}")))
}

fn queue_reduce(state: &mut QueueState, invocation: &ActionInvocation) -> GuiResult<()> {
    state.handled.push(invocation.action.clone());
    Ok(())
}

fn removing_frame(state: &RemovingState) -> GuiResult<UiFrame> {
    let children = if state.removed {
        json!([
            {
                "kind": "element",
                "key": "done",
                "tag": "Button",
                "props": {"label": "Done"}
            }
        ])
    } else {
        json!([
            {
                "kind": "element",
                "key": "remove",
                "tag": "Button",
                "props": {"label": "Remove stale", "events": {"onPress": "remove"}}
            },
            {
                "kind": "element",
                "key": "stale",
                "tag": "Button",
                "props": {"label": "Stale action", "events": {"onPress": "stale"}}
            }
        ])
    };

    serde_json::from_value(json!({
        "frameId": "removing",
        "actions": [{"id": "remove"}, {"id": "stale"}],
        "root": {
            "kind": "element",
            "key": "removing-toolbar",
            "tag": "Toolbar",
            "children": children
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid removing frame: {error}")))
}

fn removing_reduce(state: &mut RemovingState, invocation: &ActionInvocation) -> GuiResult<()> {
    match invocation.action.as_str() {
        "remove" => {
            state.removed = true;
            Ok(())
        }
        "stale" => Err(GuiError::host("stale action should not dispatch")),
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
fn native_runtime_event_batch_reports_buffered_events_when_predicate_stops() {
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

    let batch = app
        .handle_pending_native_event_batch_while(|state| state.count < 1)
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(batch.host_events_drained, 2);
    assert_eq!(batch.queued_native_events, 2);
    assert_eq!(batch.handled_native_events, 1);
    assert_eq!(batch.responses.len(), 1);
    assert_eq!(batch.buffered_native_events, 1);
    assert!(batch.stopped_by_predicate);
    assert!(!batch.host_queue_preserved);
    assert_eq!(app.pending_native_event_count(), 1);
    assert!(app.has_pending_native_events());

    let second_batch = app.handle_pending_native_event_batch().unwrap();

    assert_eq!(app.state().count, 2);
    assert_eq!(second_batch.host_events_drained, 0);
    assert_eq!(second_batch.queued_native_events, 1);
    assert_eq!(second_batch.handled_native_events, 1);
    assert_eq!(second_batch.buffered_native_events, 0);
    assert!(!second_batch.stopped_by_predicate);
    assert!(!app.has_pending_native_events());
}

#[test]
fn native_runtime_event_batch_merges_sequential_drains() {
    let mut first = NativeRuntimeEventBatch {
        responses: vec![NativeRuntimeEventResponse {
            frame_id: "queue".to_string(),
            event: NativeEvent::new(HostNodeId::new(1), NativeEventKind::Press),
            invocation: None,
            accessibility_tree: None,
            interaction_changes: Vec::new(),
            render: None,
            value_sensitivity: ValueSensitivity::Public,
        }],
        host_events_drained: 1,
        queued_native_events: 1,
        handled_native_events: 1,
        buffered_native_events: 0,
        stopped_by_predicate: false,
        host_queue_preserved: false,
    };
    let second = NativeRuntimeEventBatch {
        responses: Vec::new(),
        host_events_drained: 2,
        queued_native_events: 2,
        handled_native_events: 1,
        buffered_native_events: 1,
        stopped_by_predicate: true,
        host_queue_preserved: true,
    };

    first.extend(second);

    assert_eq!(first.responses.len(), 1);
    assert_eq!(first.host_events_drained, 3);
    assert_eq!(first.queued_native_events, 3);
    assert_eq!(first.handled_native_events, 2);
    assert_eq!(first.buffered_native_events, 1);
    assert!(first.stopped_by_predicate);
    assert!(first.host_queue_preserved);
}

#[test]
fn background_update_batch_renders_once_and_tracks_pending_work() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let poll_count = Rc::new(Cell::new(0_u32));
    let poll_count_for_callback = Rc::clone(&poll_count);
    let mut app =
        NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce)
            .with_background_updates(move |state| {
                let count = poll_count_for_callback.get();
                poll_count_for_callback.set(count + 1);
                if count == 0 {
                    state.count = 7;
                    Ok(BackgroundUpdate::changed(true))
                } else {
                    Ok(BackgroundUpdate::idle(false))
                }
            });
    app.render().unwrap();

    let rendered = app.poll_background_updates().unwrap();

    assert!(rendered.is_some());
    assert_eq!(app.state().count, 7);
    assert!(app.has_pending_background_work());

    assert!(app.poll_background_updates().unwrap().is_none());
    assert!(!app.has_pending_background_work());
    assert_eq!(poll_count.get(), 2);
}

#[test]
fn native_runtime_app_handles_buffered_events_before_new_host_events() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(host, QueueState::default(), queue_frame, queue_reduce);
    app.render().unwrap();

    let first = action_node(&app, "first");
    let second = action_node(&app, "second");
    let third = action_node(&app, "third");

    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(first, NativeEventKind::Press));
    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(second, NativeEventKind::Press));

    let first_batch = app
        .handle_pending_native_events_while(|state| state.handled.is_empty())
        .unwrap();
    assert_eq!(invocation_actions(&first_batch), vec!["first"]);
    assert_eq!(app.state().handled, vec!["first"]);

    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(third, NativeEventKind::Press));

    let second_batch = app.handle_pending_native_events().unwrap();

    assert_eq!(invocation_actions(&second_batch), vec!["second", "third"]);
    assert_eq!(app.state().handled, vec!["first", "second", "third"]);
}

#[test]
fn native_runtime_app_ignores_stale_pending_events_after_rerender_removes_node() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(
        host,
        RemovingState::default(),
        removing_frame,
        removing_reduce,
    );
    app.render().unwrap();

    let remove = action_node(&app, "remove");
    let stale = action_node(&app, "stale");

    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(remove, NativeEventKind::Press));
    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(stale, NativeEventKind::Press));

    let responses = app.handle_pending_native_events().unwrap();

    assert!(app.state().removed);
    assert_eq!(responses.len(), 2);
    assert_eq!(invocation_actions(&responses), vec!["remove"]);
    assert!(responses[1].invocation.is_none());
    assert!(responses[1].render.is_none());
    assert!(responses[1].interaction_changes.is_empty());
    assert!(app.runtime().host().planning().node(stale).is_none());
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

#[test]
fn native_runtime_event_batch_reports_preserved_host_queue_when_predicate_starts_false() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app =
        NativeRuntimeApp::new(host, CounterState::default(), counter_frame, counter_reduce);
    let rendered = app.render().unwrap();

    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press));

    let batch = app
        .handle_pending_native_event_batch_while(|_| false)
        .unwrap();

    assert!(batch.responses.is_empty());
    assert_eq!(batch.host_events_drained, 0);
    assert_eq!(batch.queued_native_events, 0);
    assert_eq!(batch.handled_native_events, 0);
    assert_eq!(batch.buffered_native_events, 0);
    assert!(batch.stopped_by_predicate);
    assert!(batch.host_queue_preserved);
    assert_eq!(app.pending_native_event_count(), 0);

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

fn action_node<S, F, R>(
    app: &NativeRuntimeApp<CommandExecutingHost<Gtk4Adapter, RecordingBackend>, S, F, R>,
    action: &str,
) -> HostNodeId
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    app.runtime()
        .host()
        .planning()
        .nodes()
        .iter()
        .find_map(|(id, node)| {
            (node.blueprint.events.get("onPress").map(String::as_str) == Some(action))
                .then_some(*id)
        })
        .unwrap_or_else(|| panic!("missing queue action node {action}"))
}

fn invocation_actions(responses: &[NativeRuntimeEventResponse]) -> Vec<&str> {
    responses
        .iter()
        .filter_map(|response| response.invocation.as_ref())
        .map(|invocation| invocation.action.as_str())
        .collect()
}

#[test]
fn native_runtime_event_response_redacts_password_but_reducer_keeps_value() {
    #[derive(Debug, Default)]
    struct PasswordState {
        reduced_value: Option<String>,
    }

    let frame_builder = |_state: &PasswordState| {
        serde_json::from_value(json!({
            "frameId": "password",
            "root": {
                "kind": "element",
                "key": "password",
                "tag": "TextField",
                "props": {
                    "inputType": "password",
                    "value": "initial-password-secret",
                    "events": {"onChange": "setPassword"}
                }
            }
        }))
        .map_err(|error| GuiError::invalid_tree(format!("invalid password frame: {error}")))
    };
    let reducer = |state: &mut PasswordState, invocation: &ActionInvocation| {
        state.reduced_value = invocation.value.clone();
        Ok(())
    };
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(host, PasswordState::default(), frame_builder, reducer);
    let rendered = app.render().unwrap();

    assert_eq!(
        app.runtime()
            .host()
            .planning()
            .node(rendered.root)
            .and_then(|node| node.blueprint.value.as_deref()),
        Some("initial-password-secret")
    );

    let response = app
        .handle_native_event(
            NativeEvent::new(rendered.root, NativeEventKind::Change)
                .value("changed-password-secret"),
        )
        .unwrap();

    assert_eq!(
        app.state().reduced_value.as_deref(),
        Some("changed-password-secret")
    );
    assert_eq!(
        app.runtime()
            .interactions()
            .node(rendered.root)
            .and_then(|state| state.value.as_deref()),
        Some("changed-password-secret")
    );
    assert_eq!(
        app.runtime()
            .actions()
            .invocations()
            .last()
            .and_then(|invocation| invocation.value.as_deref()),
        None
    );
    assert!(response
        .accessibility_tree
        .as_ref()
        .is_some_and(|tree| tree.value.is_none()));

    let wire = serde_json::to_string(&response).unwrap();
    let response_debug = format!("{response:?}");
    assert!(!wire.contains("initial-password-secret"));
    assert!(!wire.contains("changed-password-secret"));
    assert!(!wire.contains("valueSensitivity"));
    assert!(!response_debug.contains("initial-password-secret"));
    assert!(!response_debug.contains("changed-password-secret"));

    let app_debug = format!("{app:?}");
    let runtime_debug = format!("{:?}", app.runtime());
    assert!(!app_debug.contains("initial-password-secret"));
    assert!(!app_debug.contains("changed-password-secret"));
    assert!(!runtime_debug.contains("initial-password-secret"));
    assert!(!runtime_debug.contains("changed-password-secret"));
}
