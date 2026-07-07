#[cfg(target_os = "windows")]
#[path = "support/calculator/mod.rs"]
mod calculator;

#[cfg(target_os = "windows")]
mod winui_calculator {
    use a3s_gui::WinUiRuntimeApp;

    use crate::calculator::{
        calculator_frame, calculator_reduce, shared_calculator_component, CalculatorState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let component = shared_calculator_component("winui-calculator", "A3S Calculator")?;
        let render_component = component.clone();
        let reduce_component = component.clone();
        let mut app = WinUiRuntimeApp::winui(
            CalculatorState::default(),
            move |state| calculator_frame(&render_component, state),
            move |state, invocation| calculator_reduce(&reduce_component, state, invocation),
        )?;
        app.render()?;
        app.run_winui()?;
        println!("calculator closed with state: {:?}", app.state());
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    winui_calculator::run()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("winui_calculator requires Windows.");
}
