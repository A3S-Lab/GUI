#![recursion_limit = "4096"]

#[cfg(target_os = "linux")]
#[path = "support/component_playground/mod.rs"]
mod component_playground;

#[cfg(target_os = "linux")]
mod gtk4_component_playground {
    use a3s_gui::Gtk4RuntimeApp;

    use crate::component_playground::{
        component_playground_frame, component_playground_reduce,
        shared_component_playground_component, ComponentPlaygroundState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let component = shared_component_playground_component(
            "gtk4-component-playground",
            "A3S Component Playground",
        )?;
        let render_component = component.clone();
        let reduce_component = component.clone();
        let mut app = Gtk4RuntimeApp::gtk4(
            ComponentPlaygroundState::default(),
            move |state| component_playground_frame(&render_component, state),
            move |state, invocation| {
                component_playground_reduce(&reduce_component, state, invocation)
            },
        )?;
        app.render()?;
        app.run_gtk4()?;
        println!("component playground closed with state: {:?}", app.state());
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    gtk4_component_playground::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("gtk4_component_playground requires Linux.");
}
