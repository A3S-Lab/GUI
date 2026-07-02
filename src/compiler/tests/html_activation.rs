use super::support::*;

#[test]
fn lowers_html_button_activation_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let button = element(
        "show-settings",
        "button",
        BTreeMap::from([
            ("command".to_string(), "show-modal".to_string()),
            ("commandFor".to_string(), "settings-dialog".to_string()),
            ("popoverTarget".to_string(), "settings-popover".to_string()),
            ("popoverTargetAction".to_string(), "show".to_string()),
        ]),
    );

    let native = bridge.lower_to_native(&button).unwrap();

    assert_eq!(native.role, NativeRole::Button);
    assert_eq!(
        native.props.html_activation.command.as_deref(),
        Some("show-modal")
    );
    assert_eq!(
        native.props.html_activation.command_for.as_deref(),
        Some("settings-dialog")
    );
    assert_eq!(
        native.props.html_activation.popover_target.as_deref(),
        Some("settings-popover")
    );
    assert_eq!(
        native
            .props
            .html_activation
            .popover_target_action
            .as_deref(),
        Some("show")
    );
}

#[test]
fn lowers_button_like_input_popover_activation_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let input_button = element(
        "open-help",
        "input",
        BTreeMap::from([
            ("type".to_string(), "button".to_string()),
            ("value".to_string(), "Help".to_string()),
            ("popovertarget".to_string(), "help-popover".to_string()),
            ("popovertargetaction".to_string(), "toggle".to_string()),
        ]),
    );
    let text_input = element(
        "email",
        "input",
        BTreeMap::from([
            ("type".to_string(), "email".to_string()),
            ("popovertarget".to_string(), "ignored-popover".to_string()),
        ]),
    );

    let native_button = bridge.lower_to_native(&input_button).unwrap();
    let native_text = bridge.lower_to_native(&text_input).unwrap();

    assert_eq!(native_button.role, NativeRole::Button);
    assert_eq!(
        native_button
            .props
            .html_activation
            .popover_target
            .as_deref(),
        Some("help-popover")
    );
    assert_eq!(
        native_button
            .props
            .html_activation
            .popover_target_action
            .as_deref(),
        Some("toggle")
    );
    assert_eq!(native_text.role, NativeRole::TextField);
    assert_eq!(native_text.props.html_activation.popover_target, None);
}

fn element(key: &str, tag: &str, attributes: BTreeMap<String, String>) -> CompiledJsxNode {
    CompiledJsxNode::Element {
        key: key.to_string(),
        tag: tag.to_string(),
        import_source: None,
        props: CompiledProps {
            attributes,
            ..CompiledProps::default()
        },
        children: Vec::new(),
    }
}
