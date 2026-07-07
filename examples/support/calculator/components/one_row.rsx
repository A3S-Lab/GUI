use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorOneRowProps {
    pub press_digit: String,
    pub press_operator: String,
}

#[allow(non_snake_case)]
pub fn calculator_one_row(cx: &mut ComponentCx<CalculatorOneRowProps>) -> RSX {
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorOneRowProps| {
        props.press_digit.clone()
    });
    let pressOperator = cx.use_prop("pressOperator", |props: &CalculatorOneRowProps| {
        props.press_operator.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="One two three add">
            <CalculatorButton key="one" label="1" onPress={pressDigit} actionValue="1" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="two" label="2" onPress={pressDigit} actionValue="2" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="three" label="3" onPress={pressDigit} actionValue="3" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="add" label="+" onPress={pressOperator} actionValue="+" className="bg-[#f9f9f9] text-[20px] font-normal" />
        </CalculatorKeypadRow>
    )
}
