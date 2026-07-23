#[cfg(target_os = "windows")]
#[path = "support/dogfood_app.rs"]
mod dogfood_app;

#[cfg(target_os = "windows")]
mod winui_dogfood {
    use a3s_gui::{run_winui_application_staged_async, GuiResult, UiFrame, WinUiRuntimeApp};

    use crate::dogfood_app::{
        dogfood_frame, dogfood_reduce, dogfood_should_continue, DogfoodState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        run_winui_application_staged_async(
            || {
                let mut app =
                    WinUiRuntimeApp::winui(DogfoodState::default(), winui_frame, dogfood_reduce)?;
                app.render()?;
                Ok(app)
            },
            |mut app| async move {
                app.run_winui_while_async(dogfood_should_continue).await?;
                println!("dogfood app closed with state: {:?}", app.state());
                Ok(())
            },
        )?;
        Ok(())
    }

    fn winui_frame(state: &DogfoodState) -> GuiResult<UiFrame> {
        dogfood_frame(state, "winui-dogfood", "A3S WinUI Dogfood")
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    winui_dogfood::run()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("winui_dogfood requires Windows.");
}
