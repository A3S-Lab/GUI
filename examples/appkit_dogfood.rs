#[cfg(target_os = "macos")]
#[path = "support/dogfood_app.rs"]
mod dogfood_app;

#[cfg(target_os = "macos")]
mod appkit_dogfood {
    use a3s_gui::{AppKitRuntimeApp, GuiResult, UiFrame};

    use crate::dogfood_app::{
        dogfood_frame, dogfood_reduce, dogfood_should_continue, DogfoodState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app =
            AppKitRuntimeApp::appkit(DogfoodState::default(), appkit_frame, dogfood_reduce)?;
        app.render()?;
        app.run_appkit_while(dogfood_should_continue)?;
        println!("dogfood app closed with state: {:?}", app.state());
        Ok(())
    }

    fn appkit_frame(state: &DogfoodState) -> GuiResult<UiFrame> {
        dogfood_frame(state, "appkit-dogfood", "A3S AppKit Dogfood")
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    appkit_dogfood::run()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("appkit_dogfood requires macOS.");
}
