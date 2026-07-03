#[cfg(target_os = "linux")]
#[path = "support/controls_smoke.rs"]
mod controls_smoke;

#[cfg(target_os = "linux")]
mod gtk4_controls {
    use a3s_gui::{Gtk4RuntimeApp, GuiResult, UiFrame};

    use crate::controls_smoke::{controls_frame, controls_reduce, ControlsState};

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = Gtk4RuntimeApp::gtk4(ControlsState::default(), gtk4_frame, controls_reduce)?;
        app.render()?;
        app.run_gtk4()?;
        println!("controls smoke closed with state: {:?}", app.state());
        Ok(())
    }

    fn gtk4_frame(state: &ControlsState) -> GuiResult<UiFrame> {
        controls_frame(state, "gtk4-controls", "A3S GTK4 Controls")
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    gtk4_controls::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("gtk4_controls requires Linux.");
}
