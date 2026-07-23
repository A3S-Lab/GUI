#[cfg(target_os = "windows")]
mod winui_counter {
    use a3s_gui::{run_winui_application_staged_async, ActionInvocation, UiFrame, WinUiRuntimeApp};
    use serde_json::json;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct CounterState {
        count: u32,
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        run_winui_application_staged_async(
            || {
                let mut app =
                    WinUiRuntimeApp::winui(CounterState::default(), counter_frame, counter_reduce)?;
                app.render()?;
                Ok(app)
            },
            |mut app| async move {
                app.run_winui_while_async(|state| state.count < 5).await?;
                println!("counter finished at {}", app.state().count);
                Ok(())
            },
        )?;
        Ok(())
    }

    fn counter_frame(state: &CounterState) -> a3s_gui::GuiResult<UiFrame> {
        serde_json::from_value(json!({
            "frameId": "winui-counter",
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
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    winui_counter::run()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("winui_counter requires Windows.");
}
