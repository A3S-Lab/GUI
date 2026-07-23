#![recursion_limit = "4096"]

#[cfg(target_os = "windows")]
#[path = "support/component_playground/mod.rs"]
mod component_playground;

#[cfg(target_os = "windows")]
mod winui_component_playground {
    use a3s_gui::{run_winui_application_staged_async, WinUiRuntimeApp};

    use crate::component_playground::{
        component_playground_frame, component_playground_reduce,
        shared_component_playground_component, ComponentPlaygroundState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let component = shared_component_playground_component(
            "winui-component-playground",
            "A3S Component Playground",
        )?;
        let render_component = component.clone();
        let reduce_component = component.clone();
        run_winui_application_staged_async(
            move || {
                let mut app = WinUiRuntimeApp::winui(
                    ComponentPlaygroundState::default(),
                    move |state| component_playground_frame(&render_component, state),
                    move |state, invocation| {
                        component_playground_reduce(&reduce_component, state, invocation)
                    },
                )?;
                app.render()?;
                Ok(app)
            },
            |mut app| async move {
                app.run_winui_async().await?;
                println!("component playground closed with state: {:?}", app.state());
                Ok(())
            },
        )?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    winui_component_playground::run()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("winui_component_playground requires Windows.");
}
