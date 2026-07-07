use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorEditRowProps {
    pub percent: String,
    pub clear_entry: String,
    pub clear: String,
    pub backspace: String,
}

#[allow(non_snake_case)]
pub fn calculator_edit_row(cx: &mut ComponentCx<CalculatorEditRowProps>) -> RSX {
    let percent = cx.use_prop("percent", |props: &CalculatorEditRowProps| {
        props.percent.clone()
    });
    let clearEntry = cx.use_prop("clearEntry", |props: &CalculatorEditRowProps| {
        props.clear_entry.clone()
    });
    let clear = cx.use_prop("clear", |props: &CalculatorEditRowProps| {
        props.clear.clone()
    });
    let backspace = cx.use_prop("backspace", |props: &CalculatorEditRowProps| {
        props.backspace.clone()
    });

    a3s_gui::rsx!(
        <CalculatorKeypadRow key="root" label="Edit controls">
            <CalculatorButton key="percent" label="%" onPress={percent} className="bg-[#f9f9f9] text-[20px] font-normal" />
            <CalculatorButton key="clear-entry" label="CE" onPress={clearEntry} className="bg-[#f9f9f9] text-[15px] font-normal" />
            <CalculatorButton key="clear" label="C" onPress={clear} className="bg-[#f9f9f9] text-[15px] font-normal" />
            <CalculatorButton key="backspace" label="⌫" onPress={backspace} className="bg-[#f9f9f9] text-[18px] font-normal" />
        </CalculatorKeypadRow>
    )
}
