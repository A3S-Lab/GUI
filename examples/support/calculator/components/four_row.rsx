use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorFourRowProps {
    pub press_digit: String,
    pub press_operator: String,
}

#[allow(non_snake_case)]
pub fn calculator_four_row(cx: &mut ComponentCx<CalculatorFourRowProps>) -> RSX {
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorFourRowProps| {
        props.press_digit.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorFourRowProps| {
        props.press_operator.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="Four five six subtract">
            <CalculatorButton key="four" label="4" onPress={pressDigit} actionValue="4" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="five" label="5" onPress={pressDigit} actionValue="5" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="six" label="6" onPress={pressDigit} actionValue="6" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="subtract" label="−" onPress={pressOperator} actionValue="-" className="bg-[#f9f9f9] text-[20px] font-normal" />
        </CalculatorKeypadRow>
    )
}
