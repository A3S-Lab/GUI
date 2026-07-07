use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorFunctionRowProps {
    pub reciprocal: String,
    pub square: String,
    pub square_root: String,
    pub press_operator: String,
}

#[allow(non_snake_case)]
pub fn calculator_function_row(cx: &mut ComponentCx<CalculatorFunctionRowProps>) -> RSX {
    let reciprocal = cx.use_prop("reciprocal", |props: &CalculatorFunctionRowProps| {
        props.reciprocal.clone()
    });
    let square = cx.use_prop("square", |props: &CalculatorFunctionRowProps| {
        props.square.clone()
    });
    let squareRoot = cx.use_prop("squareRoot", |props: &CalculatorFunctionRowProps| {
        props.square_root.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorFunctionRowProps| {
        props.press_operator.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="Function controls">
            <CalculatorButton key="reciprocal" label="1/x" onPress={reciprocal} className="bg-[#f9f9f9] text-[15px] font-normal" />
            <CalculatorButton key="square" label="x²" onPress={square} className="bg-[#f9f9f9] text-[15px] font-normal" />
            <CalculatorButton key="square-root" label="√x" onPress={squareRoot} className="bg-[#f9f9f9] text-[15px] font-normal" />
            <CalculatorButton key="divide" label="÷" onPress={pressOperator} actionValue="/" className="bg-[#f9f9f9] text-[20px] font-normal" />
        </CalculatorKeypadRow>
    )
}
