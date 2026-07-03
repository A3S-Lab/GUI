#[cfg(target_os = "windows")]
#[path = "support/controls_smoke.rs"]
mod controls_smoke;

#[cfg(target_os = "windows")]
mod winui_controls {
    use a3s_gui::{GuiResult, UiFrame, WinUiRuntimeApp};

    use crate::controls_smoke::{controls_frame, controls_reduce, ControlsState};

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app =
            WinUiRuntimeApp::winui(ControlsState::default(), winui_frame, controls_reduce)?;
        app.render()?;
        app.run_winui()?;
        println!("controls smoke closed with state: {:?}", app.state());
        Ok(())
    }

    fn winui_frame(state: &ControlsState) -> GuiResult<UiFrame> {
        controls_frame(state, "winui-controls", "A3S WinUI Controls")
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    winui_controls::run()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("winui_controls requires Windows.");
}
