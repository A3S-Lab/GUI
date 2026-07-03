use a3s_gui::{ActionInvocation, Gtk4RuntimeApp, UiFrame};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Default)]
struct CounterState {
    count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Gtk4RuntimeApp::gtk4(CounterState::default(), counter_frame, counter_reduce)?;
    app.render()?;
    app.run_gtk4_while(|state| state.count < 5)?;
    println!("counter finished at {}", app.state().count);
    Ok(())
}

fn counter_frame(state: &CounterState) -> a3s_gui::GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "gtk4-counter",
        "window": {"title": "A3S Counter", "width": 320, "height": 180},
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
