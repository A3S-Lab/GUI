use a3s_gui::{ComponentCx, RSX};
use serde::Serialize;

use super::super::model::CalculatorState;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct CalculatorReactiveState {
    display: String,
    history: String,
    has_error: bool,
}

#[allow(non_snake_case)]
pub fn calculator(cx: &mut ComponentCx<CalculatorState>) -> RSX {
    let calculator = cx.use_reactive("calculator", |state: &CalculatorState| {
        CalculatorReactiveState {
            display: state.display().to_string(),
            history: state.history().to_string(),
            has_error: state.has_error(),
        }
    });

    let pressDigit = cx.use_value_reducer("pressDigit", |state: &mut CalculatorState, digit| {
        state.press_digit(digit)
    });
    let pressOperator = cx
        .use_value_reducer("pressOperator", |state: &mut CalculatorState, operator| {
            state.press_operator(operator)
        });
    let pressDecimal = cx.use_reducer("pressDecimal", |state: &mut CalculatorState, _| {
        state.press_decimal();
        Ok(())
    });
    let pressEquals = cx.use_reducer("pressEquals", |state: &mut CalculatorState, _| {
        state.press_equals()
    });
    let clear = cx.use_reducer("clear", |state: &mut CalculatorState, _| {
        state.clear();
        Ok(())
    });
    let clearEntry = cx.use_reducer("clearEntry", |state: &mut CalculatorState, _| {
        state.clear_entry();
        Ok(())
    });
    let backspace = cx.use_reducer("backspace", |state: &mut CalculatorState, _| {
        state.backspace();
        Ok(())
    });
    let percent = cx.use_reducer("percent", |state: &mut CalculatorState, _| state.percent());
    let reciprocal = cx.use_reducer("reciprocal", |state: &mut CalculatorState, _| {
        state.reciprocal()
    });
    let square = cx.use_reducer("square", |state: &mut CalculatorState, _| state.square());
    let squareRoot = cx.use_reducer("squareRoot", |state: &mut CalculatorState, _| {
        state.square_root()
    });
    let toggleSign = cx.use_reducer("toggleSign", |state: &mut CalculatorState, _| {
        state.toggle_sign();
        Ok(())
    });

    a3s_gui::rsx!(
        <CalculatorShell
            key="calculator"
            display={calculator.display}
            history={calculator.history}
            hasError={calculator.hasError}
            pressDigit={pressDigit}
            pressOperator={pressOperator}
            pressDecimal={pressDecimal}
            pressEquals={pressEquals}
            clear={clear}
            clearEntry={clearEntry}
            backspace={backspace}
            percent={percent}
            reciprocal={reciprocal}
            square={square}
            squareRoot={squareRoot}
            toggleSign={toggleSign}
        />
    )
}
