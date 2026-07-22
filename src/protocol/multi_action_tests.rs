use serde_json::json;

use super::*;
use crate::event::NativeEventKind;
use crate::input::NativeInputModality;
use crate::platform::Gtk4Adapter;

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

#[test]
fn protocol_app_preserves_and_reduces_complete_event_batch() {
    let mut app = NativeProtocolApp::new(Gtk4Adapter, EventLog::default(), frame, reduce);
    let rendered = app.render().unwrap();
    let target = app
        .session()
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let response = app
        .handle_host_event(&HostEvent {
            frame_id: "multi-action".to_string(),
            event: NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        })
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
fn protocol_app_stops_ancestors_after_same_target_callbacks() {
    let mut app = NativeProtocolApp::new_with_propagation(
        Gtk4Adapter,
        EventLog::default(),
        frame,
        reduce_with_propagation,
    );
    let rendered = app.render().unwrap();
    let target = app
        .session()
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let response = app
        .handle_host_event_with_propagation(&HostEvent {
            frame_id: "multi-action".to_string(),
            event: NativeEvent::new(target, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        })
        .unwrap();

    assert_eq!(
        app.state().actions,
        vec!["targetHoverStart", "targetHoverChange"]
    );
    assert_eq!(response.invocations.len(), 2);
    assert_eq!(response.propagation_stopped_at, Some(target));
    assert_eq!(app.session().runtime().actions().invocations().len(), 2);
    assert!(response.render.is_some());
}
