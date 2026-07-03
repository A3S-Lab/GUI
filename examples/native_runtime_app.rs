use a3s_gui::{
    ActionInvocation, CommandExecutingHost, Gtk4Adapter, NativeEvent, NativeEventKind,
    NativeRuntimeApp, RecordingBackend, UiFrame,
};
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
struct CounterState {
    count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(
        host,
        CounterState { count: 0 },
        counter_frame,
        counter_reduce,
    );

    let rendered = app.render()?;
    let button = rendered.root;

    app.runtime_mut()
        .host_mut()
        .executor_mut()
        .push_native_event(NativeEvent::new(button, NativeEventKind::Press));
    let responses = app.handle_pending_native_events()?;

    assert_eq!(app.state().count, 1);
    assert_eq!(responses.len(), 1);
    println!("counter advanced to {}", app.state().count);
    Ok(())
}

fn counter_frame(state: &CounterState) -> a3s_gui::GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "counter",
        "actions": [{"id": "increment", "label": "Increment counter"}],
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
    .map_err(|error| a3s_gui::GuiError::invalid_tree(format!("invalid counter frame: {error}")))
}

fn counter_reduce(
    state: &mut CounterState,
    invocation: &ActionInvocation,
) -> a3s_gui::GuiResult<()> {
    match invocation.action.as_str() {
        "increment" => {
            state.count += 1;
            Ok(())
        }
        other => Err(a3s_gui::GuiError::host(format!(
            "unexpected action {other}"
        ))),
    }
}
