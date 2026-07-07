#[cfg(target_os = "linux")]
#[path = "support/calculator/mod.rs"]
mod calculator;

#[cfg(target_os = "linux")]
mod gtk4_calculator {
    use a3s_gui::Gtk4RuntimeApp;

    use crate::calculator::{
        calculator_frame, calculator_reduce, shared_calculator_component, CalculatorState,
    };

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let component = shared_calculator_component("gtk4-calculator", "A3S Calculator")?;
        let render_component = component.clone();
        let reduce_component = component.clone();
        let mut app = Gtk4RuntimeApp::gtk4(
            CalculatorState::default(),
            move |state| calculator_frame(&render_component, state),
            move |state, invocation| calculator_reduce(&reduce_component, state, invocation),
        )?;
        app.render()?;
        app.run_gtk4()?;
        println!("calculator closed with state: {:?}", app.state());
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    gtk4_calculator::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("gtk4_calculator requires Linux.");
}
