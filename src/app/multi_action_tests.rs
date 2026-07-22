use serde_json::json;

use super::*;
use crate::backend::{CommandExecutingHost, RecordingBackend};
use crate::event::NativeEventKind;
use crate::input::NativeInputModality;
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};

#[derive(Debug, Default)]
struct EventLog {
    actions: Vec<String>,
}

fn frame(_state: &EventLog) -> GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "multi-action",
        "actions": [
            {"id": "targetHoverStart"},
            {"id": "targetHoverChange"},
            {"id": "parentHoverChange"}
        ],
        "root": {
            "kind": "element",
            "key": "parent",
            "tag": "Group",
            "props": {"events": {"onHoverChange": "parentHoverChange"}},
            "children": [{
                "kind": "element",
                "key": "target",
                "tag": "Button",
                "props": {"events": {
                    "onHoverStart": "targetHoverStart",
                    "onHoverChange": "targetHoverChange"
                }}
            }]
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid multi-action frame: {error}")))
}

fn reduce(state: &mut EventLog, invocation: &ActionInvocation) -> GuiResult<()> {
    state.actions.push(invocation.action.clone());
    Ok(())
}

fn reduce_with_propagation(
    state: &mut EventLog,
    invocation: &ActionInvocation,
) -> GuiResult<ActionPropagation> {
    state.actions.push(invocation.action.clone());
    Ok(if invocation.action == "targetHoverStart" {
        ActionPropagation::Stop
    } else {
        ActionPropagation::Continue
    })
}

fn reduce_and_continue(
    state: &mut EventLog,
    invocation: &ActionInvocation,
) -> GuiResult<ActionPropagation> {
    state.actions.push(invocation.action.clone());
    Ok(ActionPropagation::Continue)
}

fn reduce_with_error(
    state: &mut EventLog,
    invocation: &ActionInvocation,
) -> GuiResult<ActionPropagation> {
    state.actions.push(invocation.action.clone());
    if invocation.action == "targetHoverChange" {
        Err(GuiError::host("propagation reducer failed"))
    } else {
        Ok(ActionPropagation::Continue)
    }
}

#[test]
fn runtime_app_reduces_complete_event_batch_before_one_render() {
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut app = NativeRuntimeApp::new(host, EventLog::default(), frame, reduce);
    let rendered = app.render().unwrap();
    let target = app.runtime().host().node(rendered.root).unwrap().children[0];

    let response = app
        .handle_native_event(
            NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        )
        .unwrap();

    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange", "parentHoverChange"]
    );
    assert_eq!(response.invocations.len(), 3);
    assert_eq!(response.invocation, response.invocations.first().cloned());
    assert!(response.render.is_some());
}

#[test]
fn runtime_app_stops_before_ancestors_after_finishing_same_target_callbacks() {
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut app = NativeRuntimeApp::new_with_propagation(
        host,
        EventLog::default(),
        frame,
        reduce_with_propagation,
    );
    let rendered = app.render().unwrap();
    let target = app.runtime().host().node(rendered.root).unwrap().children[0];

    let response = app
        .handle_native_event_with_propagation(
            NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        )
        .unwrap();

    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange"]
    );
    assert_eq!(response.invocations.len(), 2);
    assert!(response
        .invocations
        .iter()
        .all(|invocation| invocation.current_target() == target));
    assert_eq!(response.propagation_stopped_at, Some(target));
    assert_eq!(
        serde_json::to_value(&response).unwrap()["propagationStoppedAt"],
        json!(target)
    );
    assert_eq!(
        app.runtime()
            .actions()
            .invocations()
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["targetHoverStart", "targetHoverChange"]
    );
    assert!(response.render.is_some());
}

#[test]
fn runtime_app_continues_through_all_ancestor_callbacks() {
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut app = NativeRuntimeApp::new_with_propagation(
        host,
        EventLog::default(),
        frame,
        reduce_and_continue,
    );
    let rendered = app.render().unwrap();
    let target = app.runtime().host().node(rendered.root).unwrap().children[0];

    let response = app
        .handle_native_event_with_propagation(
            NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        )
        .unwrap();

    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange", "parentHoverChange"]
    );
    assert_eq!(response.invocations.len(), 3);
    assert_eq!(response.propagation_stopped_at, None);
    assert_eq!(app.runtime().actions().invocations().len(), 3);
}

#[test]
fn runtime_app_discards_routed_history_when_propagation_reducer_fails() {
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut app =
        NativeRuntimeApp::new_with_propagation(host, EventLog::default(), frame, reduce_with_error);
    let rendered = app.render().unwrap();
    let target = app.runtime().host().node(rendered.root).unwrap().children[0];

    let error = app
        .handle_native_event_with_propagation(
            NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        )
        .unwrap_err();

    assert_eq!(
        error.to_string(),
        "native host error: propagation reducer failed"
    );
    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange"]
    );
    assert!(app.runtime().actions().invocations().is_empty());
}

#[test]
fn runtime_app_pending_event_drain_honors_propagation() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new_with_propagation(
        host,
        EventLog::default(),
        frame,
        reduce_with_propagation,
    );
    let rendered = app.render().unwrap();
    let target = app
        .runtime()
        .host()
        .planning()
        .node(rendered.root)
        .unwrap()
        .children[0];
    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(
            NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        );

    let batch = app
        .handle_pending_native_event_batch_with_propagation()
        .unwrap();

    assert_eq!(batch.host_events_drained, 1);
    assert_eq!(batch.handled_native_events, 1);
    assert_eq!(batch.responses[0].propagation_stopped_at, Some(target));
    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange"]
    );
}
