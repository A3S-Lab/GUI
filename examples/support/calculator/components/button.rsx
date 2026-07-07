use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorButtonProps {
    pub label: String,
    pub on_press: String,
    pub action_value: String,
    pub class_name: String,
}

#[allow(non_snake_case)]
pub fn calculator_button(cx: &mut ComponentCx<CalculatorButtonProps>) -> RSX {
    let label = cx.use_prop("label", |props: &CalculatorButtonProps| props.label.clone());
    let onPress = cx.use_prop("onPress", |props: &CalculatorButtonProps| {
        props.on_press.clone()
    });
    let actionValue = cx.use_prop("actionValue", |props: &CalculatorButtonProps| {
        props.action_value.clone()
    });
    let className = cx.use_prop("className", |props: &CalculatorButtonProps| {
        props.class_name.clone()
    });

    a3s_gui::rsx!(
        <Button
            key="root"
            label={label}
            onPress={onPress}
            actionValue={actionValue}
            class="h-14 min-h-14 w-[94px] min-w-[94px] rounded-[5px] border border-[#e5e5e5] p-0 text-[#1b1b1b]"
            className={className}
        />
    )
}
