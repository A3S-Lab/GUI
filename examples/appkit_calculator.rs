#[cfg(target_os = "macos")]
#[path = "support/calculator/mod.rs"]
mod calculator;

#[cfg(target_os = "macos")]
mod appkit_calculator {
    use a3s_gui::AppKitRuntimeApp;

    use crate::calculator::{
        calculator_frame, calculator_reduce, shared_calculator_component, CalculatorState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let component = shared_calculator_component("appkit-calculator", "A3S Calculator")?;
        let render_component = component.clone();
        let reduce_component = component.clone();
        let mut app = AppKitRuntimeApp::appkit(
            CalculatorState::default(),
            move |state| calculator_frame(&render_component, state),
            move |state, invocation| calculator_reduce(&reduce_component, state, invocation),
        )?;
        app.render()?;
        app.run_appkit()?;
        println!("calculator closed with state: {:?}", app.state());
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    appkit_calculator::run()
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("appkit_calculator requires macOS.");
}
