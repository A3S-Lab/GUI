#[cfg(target_os = "macos")]
#[path = "support/controls_smoke.rs"]
mod controls_smoke;

#[cfg(target_os = "macos")]
mod appkit_controls {
    use a3s_gui::{AppKitRuntimeApp, GuiResult, UiFrame};

    use crate::controls_smoke::{controls_frame, controls_reduce, ControlsState};

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app =
            AppKitRuntimeApp::appkit(ControlsState::default(), appkit_frame, controls_reduce)?;
        app.render()?;
        app.run_appkit()?;
        println!("controls smoke closed with state: {:?}", app.state());
        Ok(())
    }

    fn appkit_frame(state: &ControlsState) -> GuiResult<UiFrame> {
        controls_frame(state, "appkit-controls", "A3S AppKit Controls")
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    appkit_controls::run()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("appkit_controls requires macOS.");
}
