#[cfg(target_os = "linux")]
#[path = "support/dogfood_app.rs"]
mod dogfood_app;

#[cfg(target_os = "linux")]
mod gtk4_dogfood {
    use a3s_gui::{Gtk4RuntimeApp, GuiResult, UiFrame};

    use crate::dogfood_app::{dogfood_frame, dogfood_reduce, DogfoodState};

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = Gtk4RuntimeApp::gtk4(DogfoodState::default(), gtk4_frame, dogfood_reduce)?;
        app.render()?;
        app.run_gtk4()?;
        println!("dogfood app closed with state: {:?}", app.state());
        Ok(())
    }

    fn gtk4_frame(state: &DogfoodState) -> GuiResult<UiFrame> {
        dogfood_frame(state, "gtk4-dogfood", "A3S GTK4 Dogfood")
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    gtk4_dogfood::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("gtk4_dogfood requires Linux.");
}
