use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorKeypadProps {
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
pub fn calculator_keypad(cx: &mut ComponentCx<CalculatorKeypadProps>) -> RSX {
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorKeypadProps| {
        props.press_digit.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorKeypadProps| {
        props.press_operator.clone()
    });
    let pressDecimal = cx.use_prop("pressDecimal", |props: &CalculatorKeypadProps| {
        props.press_decimal.clone()
    });
    let pressEquals = cx.use_prop("pressEquals", |props: &CalculatorKeypadProps| {
        props.press_equals.clone()
    });
    let clear = cx.use_prop("clear", |props: &CalculatorKeypadProps| props.clear.clone());
    let clearEntry = cx.use_prop("clearEntry", |props: &CalculatorKeypadProps| {
        props.clear_entry.clone()
    });
    let backspace = cx.use_prop("backspace", |props: &CalculatorKeypadProps| {
        props.backspace.clone()
    });
    let percent = cx.use_prop("percent", |props: &CalculatorKeypadProps| {
        props.percent.clone()
    });
    let reciprocal = cx.use_prop("reciprocal", |props: &CalculatorKeypadProps| {
        props.reciprocal.clone()
    });
    let square = cx.use_prop("square", |props: &CalculatorKeypadProps| {
        props.square.clone()
    });
    let squareRoot = cx.use_prop("squareRoot", |props: &CalculatorKeypadProps| {
        props.square_root.clone()
    });
    let toggleSign = cx.use_prop("toggleSign", |props: &CalculatorKeypadProps| {
        props.toggle_sign.clone()
    });

    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label="Calculator keypad"
            orientation="vertical"
            className="h-[390px] w-[396px] gap-[3px] bg-[#f3f3f3] px-[6px] pb-2 pt-1"
        >
            <CalculatorEditRow
                key="row-edit"
                percent={percent}
                clearEntry={clearEntry}
                clear={clear}
                backspace={backspace}
            />
            <CalculatorFunctionRow
                key="row-functions"
                reciprocal={reciprocal}
                square={square}
                squareRoot={squareRoot}
                pressOperator={pressOperator}
            />
            <CalculatorSevenRow
                key="row-seven"
                pressDigit={pressDigit}
                pressOperator={pressOperator}
            />
            <CalculatorFourRow
                key="row-four"
                pressDigit={pressDigit}
                pressOperator={pressOperator}
            />
            <CalculatorOneRow
                key="row-one"
                pressDigit={pressDigit}
                pressOperator={pressOperator}
            />
            <CalculatorZeroRow
                key="row-zero"
                toggleSign={toggleSign}
                pressDigit={pressDigit}
                pressDecimal={pressDecimal}
                pressEquals={pressEquals}
            />
        </Toolbar>
    )
}
