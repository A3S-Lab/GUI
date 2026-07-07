use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorSevenRowProps {
    pub press_digit: String,
    pub press_operator: String,
}

#[allow(non_snake_case)]
pub fn calculator_seven_row(cx: &mut ComponentCx<CalculatorSevenRowProps>) -> RSX {
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorSevenRowProps| {
        props.press_digit.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorSevenRowProps| {
        props.press_operator.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="Seven eight nine multiply">
            <CalculatorButton key="seven" label="7" onPress={pressDigit} actionValue="7" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="eight" label="8" onPress={pressDigit} actionValue="8" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="nine" label="9" onPress={pressDigit} actionValue="9" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="multiply" label="×" onPress={pressOperator} actionValue="*" className="bg-[#f9f9f9] text-[20px] font-normal" />
        </CalculatorKeypadRow>
    )
}
