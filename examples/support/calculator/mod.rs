mod components;
mod model;
mod view;

use std::sync::Arc;

use a3s_gui::{ActionInvocation, GuiResult, RsxComponent, UiFrame};

pub use model::CalculatorState;

pub type CalculatorComponent = Arc<RsxComponent<CalculatorState>>;

pub fn shared_calculator_component(frame_id: &str, title: &str) -> GuiResult<CalculatorComponent> {
    view::calculator_component(frame_id, title).map(Arc::new)
}

pub fn calculator_frame(
    component: &CalculatorComponent,
    state: &CalculatorState,
) -> GuiResult<UiFrame> {
    component.render(state)
}

pub fn calculator_reduce(
    component: &CalculatorComponent,
    state: &mut CalculatorState,
    invocation: &ActionInvocation,
) -> GuiResult<()> {
    component.reduce(state, invocation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use a3s_gui::{CompiledRsxNode, HostNodeId, NativeEventKind};

    fn invocation(action: &str, value: &str) -> ActionInvocation {
        ActionInvocation {
            node: HostNodeId::new(1),
            current_target: None,
            action: action.to_string(),
            event: NativeEventKind::Press,
            context: Default::default(),
            value: Some(value.to_string()),
        }
    }

    #[test]
    fn calculator_adds_two_operands() {
        let component = shared_calculator_component("test-calculator", "Calculator").unwrap();
        let mut state = CalculatorState::default();

        calculator_reduce(&component, &mut state, &invocation("pressDigit", "2")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressOperator", "+")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressDigit", "3")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressEquals", "=")).unwrap();

        assert_eq!(state.display(), "5");
        assert_eq!(state.history(), "2 + 3 =");
    }

    #[test]
    fn calculator_handles_division_by_zero_without_panicking() {
        let component = shared_calculator_component("test-calculator", "Calculator").unwrap();
        let mut state = CalculatorState::default();

        calculator_reduce(&component, &mut state, &invocation("pressDigit", "7")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressOperator", "/")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressDigit", "0")).unwrap();
        calculator_reduce(&component, &mut state, &invocation("pressEquals", "=")).unwrap();

        assert_eq!(state.display(), "Cannot divide by zero");
        assert!(state.has_error());
    }

    #[test]
    fn calculator_frame_uses_a3s_ui_components() {
        let component = shared_calculator_component("test-calculator", "Calculator").unwrap();
        let frame = calculator_frame(&component, &CalculatorState::default()).unwrap();

        assert_eq!(frame.frame_id, "test-calculator");
        assert!(frame.actions.iter().any(|action| action.id == "pressDigit"));
        assert!(frame
            .actions
            .iter()
            .any(|action| action.id == "pressOperator"));
    }

    #[test]
    fn calculator_buttons_carry_static_action_values() {
        let component = shared_calculator_component("test-calculator", "Calculator").unwrap();
        let frame = calculator_frame(&component, &CalculatorState::default()).unwrap();

        let digit_values = action_values(&frame.root, "pressDigit");
        let operator_values = action_values(&frame.root, "pressOperator");

        assert!(digit_values.contains(&"7"));
        assert!(digit_values.contains(&"0"));
        assert!(operator_values.contains(&"+"));
        assert!(operator_values.contains(&"/"));
    }

    #[test]
    fn calculator_keypad_has_explicit_native_spacing_and_button_sizes() {
        let component = shared_calculator_component("test-calculator", "Calculator").unwrap();
        let frame = calculator_frame(&component, &CalculatorState::default()).unwrap();
        let keypad = find_element(&frame.root, "keypad").expect("keypad node");
        let seven =
            find_action_element(&frame.root, "pressDigit", "7").expect("seven button action");

        assert!(class_name(keypad).contains("gap-[3px]"));
        assert!(class_name(seven).contains("w-[94px]"));
        assert!(class_name(seven).contains("h-14"));
        assert!(class_name(seven).contains("min-w-[94px]"));
        assert!(class_name(seven).contains("min-h-14"));
    }

    #[test]
    fn calculator_view_is_composed_from_hook_driven_rsx_components() {
        assert!(components::CALCULATOR_RSX
            .contains("pub fn calculator(cx: &mut ComponentCx<CalculatorState>) -> RSX"));
        assert!(components::CALCULATOR_RSX.contains("cx.use_reactive"));
        assert!(components::CALCULATOR_RSX.contains("cx.use_value_reducer"));
        assert!(components::CALCULATOR_RSX.contains("<CalculatorShell"));
        assert!(components::SHELL_RSX.contains("<CalculatorDisplay"));
        assert!(components::SHELL_RSX.contains("<CalculatorKeypad"));
        assert!(components::KEYPAD_RSX.contains("<CalculatorSevenRow"));
        assert!(components::SEVEN_ROW_RSX.contains("<CalculatorKeypadRow"));
        assert!(components::SEVEN_ROW_RSX.contains("<CalculatorButton"));
        assert!(!components::CALCULATOR_RSX.contains("style="));
        assert!(!components::SHELL_RSX.contains("style="));
        assert!(!components::KEYPAD_RSX.contains("style="));
        assert!(!components::SEVEN_ROW_RSX.contains("style="));
    }

    fn action_values<'a>(node: &'a CompiledRsxNode, action: &str) -> Vec<&'a str> {
        let mut values = Vec::new();
        collect_action_values(node, action, &mut values);
        values
    }

    fn collect_action_values<'a>(
        node: &'a CompiledRsxNode,
        action: &str,
        values: &mut Vec<&'a str>,
    ) {
        let CompiledRsxNode::Element {
            props, children, ..
        } = node
        else {
            return;
        };

        if props.events.get("onPress").map(String::as_str) == Some(action) {
            if let Some(value) = props.attributes.get("actionValue") {
                values.push(value);
            }
        }

        for child in children {
            collect_action_values(child, action, values);
        }
    }

    fn find_element<'a>(
        node: &'a CompiledRsxNode,
        expected_key: &str,
    ) -> Option<&'a a3s_gui::CompiledProps> {
        let CompiledRsxNode::Element {
            key,
            props,
            children,
            ..
        } = node
        else {
            return None;
        };

        if key == expected_key || key.split('-').any(|part| part == expected_key) {
            return Some(props);
        }

        children
            .iter()
            .find_map(|child| find_element(child, expected_key))
    }

    fn find_action_element<'a>(
        node: &'a CompiledRsxNode,
        action: &str,
        value: &str,
    ) -> Option<&'a a3s_gui::CompiledProps> {
        let CompiledRsxNode::Element {
            props, children, ..
        } = node
        else {
            return None;
        };

        let action_matches = props.events.get("onPress").map(String::as_str) == Some(action);
        let value_matches = props.attributes.get("actionValue").map(String::as_str) == Some(value);
        if action_matches && value_matches {
            return Some(props);
        }

        children
            .iter()
            .find_map(|child| find_action_element(child, action, value))
    }

    fn class_name(props: &a3s_gui::CompiledProps) -> &str {
        props.class_name.as_deref().unwrap_or_default()
    }
}
