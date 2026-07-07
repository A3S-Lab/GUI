use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorZeroRowProps {
    pub toggle_sign: String,
    pub press_digit: String,
    pub press_decimal: String,
    pub press_equals: String,
}

#[allow(non_snake_case)]
pub fn calculator_zero_row(cx: &mut ComponentCx<CalculatorZeroRowProps>) -> RSX {
    let toggleSign = cx.use_prop("toggleSign", |props: &CalculatorZeroRowProps| {
        props.toggle_sign.clone()
    });
    let pressDigit = cx.use_prop("pressDigit", |props: &CalculatorZeroRowProps| {
        props.press_digit.clone()
    });
    let pressDecimal = cx.use_prop("pressDecimal", |props: &CalculatorZeroRowProps| {
        props.press_decimal.clone()
    });
    let pressEquals = cx.use_prop("pressEquals", |props: &CalculatorZeroRowProps| {
        props.press_equals.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="Sign zero decimal equals">
            <CalculatorButton key="toggle-sign" label="±" onPress={toggleSign} className="bg-white text-[20px] font-normal" />
            <CalculatorButton key="zero" label="0" onPress={pressDigit} actionValue="0" className="bg-white text-[20px] font-semibold" />
            <CalculatorButton key="decimal" label="." onPress={pressDecimal} className="bg-white text-[20px] font-normal" />
            <CalculatorButton key="equals" label="=" onPress={pressEquals} actionValue="=" className="border-[#0067c0] bg-[#0067c0] text-[22px] font-semibold text-white" />
        </CalculatorKeypadRow>
    )
}
