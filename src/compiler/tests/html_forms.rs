use super::support::*;

#[test]
fn lowers_html_placeholder_attributes_to_native_text_fields() {
    let bridge = ReactCompilerBridge::new();
    let input = CompiledJsxNode::Element {
        key: "email".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "email".to_string()),
                ("placeholder".to_string(), "you@example.com".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let textarea = CompiledJsxNode::Element {
        key: "message".to_string(),
        tag: "textarea".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([(
                "placeholder".to_string(),
                "Write a message".to_string(),
            )]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native_input = bridge.lower_to_native(&input).unwrap();
    let native_textarea = bridge.lower_to_native(&textarea).unwrap();

    assert_eq!(native_input.role, NativeRole::TextField);
    assert_eq!(
        native_input.props.placeholder.as_deref(),
        Some("you@example.com")
    );
    assert_eq!(native_textarea.role, NativeRole::TextField);
    assert_eq!(
        native_textarea.props.placeholder.as_deref(),
        Some("Write a message")
    );
}

#[test]
fn lowers_html_textarea_child_text_to_native_value() {
    let bridge = ReactCompilerBridge::new();
    let textarea = CompiledJsxNode::Element {
        key: "message".to_string(),
        tag: "textarea".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([(
                "placeholder".to_string(),
                "Write a message".to_string(),
            )]),
            ..CompiledProps::default()
        },
        children: vec![CompiledJsxNode::Text {
            key: "message-text".to_string(),
            value: "Hello".to_string(),
        }],
    };

    let native = bridge.lower_to_native(&textarea).unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.value.as_deref(), Some("Hello"));
    assert_eq!(native.props.label, None);
    assert_eq!(native.props.placeholder.as_deref(), Some("Write a message"));
    assert_eq!(
        native
            .props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .map(String::as_str),
        Some("textarea")
    );
}

#[test]
fn preserves_explicit_html_textarea_values_over_child_text() {
    let bridge = ReactCompilerBridge::new();
    let textarea = CompiledJsxNode::Element {
        key: "message".to_string(),
        tag: "textarea".to_string(),
        import_source: None,
        props: CompiledProps {
            value: Some("Controlled".to_string()),
            ..CompiledProps::default()
        },
        children: vec![CompiledJsxNode::Text {
            key: "message-text".to_string(),
            value: "Ignored".to_string(),
        }],
    };

    let native = bridge.lower_to_native(&textarea).unwrap();

    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.value.as_deref(), Some("Controlled"));
    assert_eq!(native.props.label, None);
}

#[test]
fn lowers_html_form_grouping_and_value_tags_to_native_roles() {
    let bridge = ReactCompilerBridge::new();
    let form = CompiledJsxNode::Element {
        key: "settings".to_string(),
        tag: "form".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("aria-label".to_string(), "Settings".to_string())]),
            ..CompiledProps::default()
        },
        children: vec![
            CompiledJsxNode::Element {
                key: "notifications".to_string(),
                tag: "fieldset".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![
                    CompiledJsxNode::Element {
                        key: "notifications-legend".to_string(),
                        tag: "legend".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Text {
                            key: "notifications-legend-text".to_string(),
                            value: "Notifications".to_string(),
                        }],
                    },
                    CompiledJsxNode::Element {
                        key: "notification-level".to_string(),
                        tag: "select".to_string(),
                        import_source: None,
                        props: CompiledProps::default(),
                        children: vec![CompiledJsxNode::Element {
                            key: "standard-options".to_string(),
                            tag: "optgroup".to_string(),
                            import_source: None,
                            props: CompiledProps {
                                attributes: BTreeMap::from([(
                                    "label".to_string(),
                                    "Standard".to_string(),
                                )]),
                                ..CompiledProps::default()
                            },
                            children: vec![CompiledJsxNode::Element {
                                key: "daily".to_string(),
                                tag: "option".to_string(),
                                import_source: None,
                                props: CompiledProps {
                                    attributes: BTreeMap::from([
                                        ("label".to_string(), "Daily".to_string()),
                                        ("value".to_string(), "daily".to_string()),
                                    ]),
                                    ..CompiledProps::default()
                                },
                                children: Vec::new(),
                            }],
                        }],
                    },
                ],
            },
            CompiledJsxNode::Element {
                key: "result".to_string(),
                tag: "output".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: vec![CompiledJsxNode::Text {
                    key: "result-text".to_string(),
                    value: "Saved".to_string(),
                }],
            },
            CompiledJsxNode::Element {
                key: "quota".to_string(),
                tag: "meter".to_string(),
                import_source: None,
                props: CompiledProps {
                    attributes: BTreeMap::from([
                        ("min".to_string(), "0".to_string()),
                        ("max".to_string(), "10".to_string()),
                        ("value".to_string(), "7".to_string()),
                    ]),
                    ..CompiledProps::default()
                },
                children: Vec::new(),
            },
        ],
    };

    let native = bridge.lower_to_native(&form).unwrap();
    assert_eq!(native.role, NativeRole::Form);
    assert_eq!(native.props.label.as_deref(), Some("Settings"));
    assert_eq!(native.children[0].role, NativeRole::FieldSet);
    assert_eq!(
        native.children[0].props.label.as_deref(),
        Some("Notifications")
    );
    assert_eq!(native.children[0].children[0].role, NativeRole::Legend);
    assert_eq!(
        native.children[0].children[0].props.label.as_deref(),
        Some("Notifications")
    );
    assert_eq!(native.children[0].children[1].role, NativeRole::Select);
    assert_eq!(
        native.children[0].children[1].children[0].role,
        NativeRole::OptionGroup
    );
    assert_eq!(
        native.children[0].children[1].children[0]
            .props
            .label
            .as_deref(),
        Some("Standard")
    );
    assert_eq!(
        native.children[0].children[1].children[0].children[0].role,
        NativeRole::ListBoxItem
    );
    assert_eq!(
        native.children[0].children[1].children[0].children[0]
            .props
            .label
            .as_deref(),
        Some("Daily")
    );
    assert_eq!(
        native.children[0].children[1].children[0].children[0]
            .props
            .value
            .as_deref(),
        Some("daily")
    );
    assert_eq!(native.children[1].role, NativeRole::Output);
    assert_eq!(native.children[1].props.label.as_deref(), Some("Saved"));
    assert_eq!(native.children[2].role, NativeRole::Meter);
    assert_eq!(native.children[2].props.min, Some(0.0));
    assert_eq!(native.children[2].props.max, Some(10.0));
    assert_eq!(native.children[2].props.current, Some(7.0));
}

#[test]
fn lowers_html_input_types_to_native_form_roles() {
    let bridge = ReactCompilerBridge::new();
    let input = |input_type: &str| CompiledJsxNode::Element {
        key: format!("{input_type}-input"),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("type".to_string(), input_type.to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    assert_eq!(
        bridge.lower_to_native(&input("checkbox")).unwrap().role,
        NativeRole::Checkbox
    );
    assert_eq!(
        bridge.lower_to_native(&input("radio")).unwrap().role,
        NativeRole::Radio
    );
    assert_eq!(
        bridge.lower_to_native(&input("range")).unwrap().role,
        NativeRole::Slider
    );
    assert_eq!(
        bridge.lower_to_native(&input("email")).unwrap().role,
        NativeRole::TextField
    );

    let submit = CompiledJsxNode::Element {
        key: "submit".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            value: Some("Save changes".to_string()),
            attributes: BTreeMap::from([("type".to_string(), "submit".to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_submit = bridge.lower_to_native(&submit).unwrap();

    assert_eq!(native_submit.role, NativeRole::Button);
    assert_eq!(native_submit.props.label.as_deref(), Some("Save changes"));

    let submit_default = CompiledJsxNode::Element {
        key: "submit-default".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("type".to_string(), "submit".to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_submit_default = bridge.lower_to_native(&submit_default).unwrap();

    assert_eq!(native_submit_default.role, NativeRole::Button);
    assert_eq!(native_submit_default.props.label.as_deref(), Some("Submit"));

    let reset = CompiledJsxNode::Element {
        key: "reset".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("type".to_string(), "reset".to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_reset = bridge.lower_to_native(&reset).unwrap();

    assert_eq!(native_reset.role, NativeRole::Button);
    assert_eq!(native_reset.props.label.as_deref(), Some("Reset"));

    let button = CompiledJsxNode::Element {
        key: "button".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "button".to_string()),
                ("value".to_string(), "Open panel".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_button = bridge.lower_to_native(&button).unwrap();

    assert_eq!(native_button.role, NativeRole::Button);
    assert_eq!(native_button.props.label.as_deref(), Some("Open panel"));

    let image_button = CompiledJsxNode::Element {
        key: "image-submit".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "image".to_string()),
                ("alt".to_string(), "Search".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_image_button = bridge.lower_to_native(&image_button).unwrap();

    assert_eq!(native_image_button.role, NativeRole::Button);
    assert_eq!(native_image_button.props.label.as_deref(), Some("Search"));

    let number = CompiledJsxNode::Element {
        key: "quantity".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            value: Some("7".to_string()),
            attributes: BTreeMap::from([
                ("type".to_string(), "number".to_string()),
                ("min".to_string(), "1".to_string()),
                ("max".to_string(), "10".to_string()),
                ("step".to_string(), "0.5".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_number = bridge.lower_to_native(&number).unwrap();

    assert_eq!(native_number.role, NativeRole::TextField);
    assert_eq!(native_number.props.value.as_deref(), Some("7"));
    assert_eq!(native_number.props.current, Some(7.0));
    assert_eq!(native_number.props.min, Some(1.0));
    assert_eq!(native_number.props.max, Some(10.0));
    assert_eq!(native_number.props.step, Some(0.5));
    assert_eq!(
        native_number.props.metadata.get("type").map(String::as_str),
        Some("number")
    );

    let range = CompiledJsxNode::Element {
        key: "volume".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            value: Some("42".to_string()),
            min_value: Some(0.0),
            max_value: Some(100.0),
            step_value: Some(5.0),
            attributes: BTreeMap::from([("type".to_string(), "range".to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_range = bridge.lower_to_native(&range).unwrap();

    assert_eq!(native_range.role, NativeRole::Slider);
    assert_eq!(native_range.props.current, Some(42.0));
    assert_eq!(native_range.props.min, Some(0.0));
    assert_eq!(native_range.props.max, Some(100.0));
    assert_eq!(native_range.props.step, Some(5.0));
    assert_eq!(
        native_range.props.metadata.get("type").map(String::as_str),
        Some("range")
    );
}
