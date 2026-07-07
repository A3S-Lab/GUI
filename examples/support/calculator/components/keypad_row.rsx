use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorKeypadRowProps {
    pub label: String,
    pub class_name: String,
}

#[allow(non_snake_case)]
pub fn calculator_keypad_row(cx: &mut ComponentCx<CalculatorKeypadRowProps>) -> RSX {
    let label = cx.use_prop("label", |props: &CalculatorKeypadRowProps| {
        props.label.clone()
    });
    let className = cx.use_prop("className", |props: &CalculatorKeypadRowProps| {
        props.class_name.clone()
    });

    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label={label}
            orientation="horizontal"
            className="h-14 w-96 gap-[3px] bg-[#f3f3f3]"
            class={className}
        >
            <Slot key="content" />
        </Toolbar>
    )
}
