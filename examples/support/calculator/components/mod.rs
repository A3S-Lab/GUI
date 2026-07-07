#[path = "button.rsx"]
mod button;
#[path = "calculator.rsx"]
mod calculator;
#[path = "display.rsx"]
mod display;
#[path = "edit_row.rsx"]
mod edit_row;
#[path = "four_row.rsx"]
mod four_row;
#[path = "function_row.rsx"]
mod function_row;
#[path = "keypad.rsx"]
mod keypad;
#[path = "keypad_row.rsx"]
mod keypad_row;
#[path = "memory_bar.rsx"]
mod memory_bar;
#[path = "one_row.rsx"]
mod one_row;
#[path = "seven_row.rsx"]
mod seven_row;
#[path = "shell.rsx"]
mod shell;
#[path = "title_bar.rsx"]
mod title_bar;
#[path = "zero_row.rsx"]
mod zero_row;

use a3s_gui::{ComponentCx, GuiResult, RsxComponent, RsxComponentContract, RSX};

pub use calculator::calculator;

#[cfg(test)]
pub(super) const CALCULATOR_RSX: &str = include_str!("calculator.rsx");
#[cfg(test)]
pub(super) const SHELL_RSX: &str = include_str!("shell.rsx");
#[cfg(test)]
pub(super) const KEYPAD_RSX: &str = include_str!("keypad.rsx");
#[cfg(test)]
pub(super) const SEVEN_ROW_RSX: &str = include_str!("seven_row.rsx");

pub fn with_calculator_components<S: 'static>(
    component: RsxComponent<S>,
) -> GuiResult<RsxComponent<S>> {
    let component = with_template(
        component,
        "CalculatorShell",
        shell::calculator_shell,
        RsxComponentContract::new().required([
            "display",
            "history",
            "hasError",
            "pressDigit",
            "pressOperator",
            "pressDecimal",
            "pressEquals",
            "clear",
            "clearEntry",
            "backspace",
            "percent",
            "reciprocal",
            "square",
            "squareRoot",
            "toggleSign",
        ]),
    )?;
    let component = with_template(
        component,
        "CalculatorTitleBar",
        title_bar::calculator_title_bar,
        RsxComponentContract::new(),
    )?;
    let component = with_template(
        component,
        "CalculatorDisplay",
        display::calculator_display,
        RsxComponentContract::new().required(["display", "history", "hasError"]),
    )?;
    let component = with_template(
        component,
        "CalculatorMemoryBar",
        memory_bar::calculator_memory_bar,
        RsxComponentContract::new(),
    )?;
    let component = with_template(
        component,
        "CalculatorKeypad",
        keypad::calculator_keypad,
        RsxComponentContract::new().required([
            "pressDigit",
            "pressOperator",
            "pressDecimal",
            "pressEquals",
            "clear",
            "clearEntry",
            "backspace",
            "percent",
            "reciprocal",
            "square",
            "squareRoot",
            "toggleSign",
        ]),
    )?;
    let component = with_template(
        component,
        "CalculatorKeypadRow",
        keypad_row::calculator_keypad_row,
        RsxComponentContract::new()
            .required(["label"])
            .default_prop("className", "")?,
    )?;
    let component = with_template(
        component,
        "CalculatorEditRow",
        edit_row::calculator_edit_row,
        RsxComponentContract::new().required(["percent", "clearEntry", "clear", "backspace"]),
    )?;
    let component = with_template(
        component,
        "CalculatorFunctionRow",
        function_row::calculator_function_row,
        RsxComponentContract::new().required([
            "reciprocal",
            "square",
            "squareRoot",
            "pressOperator",
        ]),
    )?;
    let component = with_template(
        component,
        "CalculatorSevenRow",
        seven_row::calculator_seven_row,
        RsxComponentContract::new().required(["pressDigit", "pressOperator"]),
    )?;
    let component = with_template(
        component,
        "CalculatorFourRow",
        four_row::calculator_four_row,
        RsxComponentContract::new().required(["pressDigit", "pressOperator"]),
    )?;
    let component = with_template(
        component,
        "CalculatorOneRow",
        one_row::calculator_one_row,
        RsxComponentContract::new().required(["pressDigit", "pressOperator"]),
    )?;
    let component = with_template(
        component,
        "CalculatorZeroRow",
        zero_row::calculator_zero_row,
        RsxComponentContract::new().required([
            "toggleSign",
            "pressDigit",
            "pressDecimal",
            "pressEquals",
        ]),
    )?;
    with_template(
        component,
        "CalculatorButton",
        button::calculator_button,
        RsxComponentContract::new()
            .required(["label", "onPress"])
            .default_prop("actionValue", "")?
            .default_prop("className", "")?,
    )
}

fn with_template<S, P, F>(
    component: RsxComponent<S>,
    name: &str,
    render: F,
    contract: RsxComponentContract,
) -> GuiResult<RsxComponent<S>>
where
    P: 'static,
    F: FnOnce(&mut ComponentCx<P>) -> RSX,
{
    let template = ComponentCx::compile(name, render)?;
    component.use_template_component_with_contract(name, template.template().clone(), contract)
}
