use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorShellProps {
    pub display: String,
    pub history: String,
    pub has_error: bool,
    pub press_digit: String,
    pub press_operator: String,
    pub press_decimal: String,
    pub press_equals: String,
    pub clear: String,
    pub clear_entry: String,
    pub backspace: String,
    pub percent: String,
    pub reciprocal: String,
    pub square: String,
    pub square_root: String,
    pub toggle_sign: String,
}

#[allow(non_snake_case)]
pub fn calculator_shell(cx: &mut ComponentCx<CalculatorShellProps>) -> RSX {
    let display = cx.use_prop("display", |props: &CalculatorShellProps| {
        props.display.clone()
    });
    let history = cx.use_prop("history", |props: &CalculatorShellProps| {
        props.history.clone()
    });
    let hasError = cx.use_prop("hasError", |props: &CalculatorShellProps| props.has_error);
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorShellProps| {
        props.press_digit.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorShellProps| {
        props.press_operator.clone()
    });
    let pressDecimal = cx.use_prop("pressDecimal", |props: &CalculatorShellProps| {
        props.press_decimal.clone()
    });
    let pressEquals = cx.use_prop("pressEquals", |props: &CalculatorShellProps| {
        props.press_equals.clone()
    });
    let clear = cx.use_prop("clear", |props: &CalculatorShellProps| props.clear.clone());
    let clearEntry = cx.use_prop("clearEntry", |props: &CalculatorShellProps| {
        props.clear_entry.clone()
    });
    let backspace = cx.use_prop("backspace", |props: &CalculatorShellProps| {
        props.backspace.clone()
    });
    let percent = cx.use_prop("percent", |props: &CalculatorShellProps| {
        props.percent.clone()
    });
    let reciprocal = cx.use_prop("reciprocal", |props: &CalculatorShellProps| {
        props.reciprocal.clone()
    });
    let square = cx.use_prop("square", |props: &CalculatorShellProps| {
        props.square.clone()
    });
    let squareRoot = cx.use_prop("squareRoot", |props: &CalculatorShellProps| {
        props.square_root.clone()
    });
    let toggleSign = cx.use_prop("toggleSign", |props: &CalculatorShellProps| {
        props.toggle_sign.clone()
    });

    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label="Calculator"
            orientation="vertical"
            className="h-[620px] w-[396px] gap-0 bg-[#f3f3f3] font-[Segoe_UI,Inter,-apple-system,system-ui,sans-serif] text-[#1b1b1b]"
        >
            <CalculatorTitleBar key="titlebar" />
            <CalculatorDisplay
                key="display"
                display={display}
                history={history}
                hasError={hasError}
            />
            <CalculatorMemoryBar key="memory" />
            <CalculatorKeypad
                key="keypad"
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
        </Toolbar>
    )
}
