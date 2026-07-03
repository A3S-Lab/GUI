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
fn lowers_common_html_form_control_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let input = CompiledJsxNode::Element {
        key: "email".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "email".to_string()),
                ("readOnly".to_string(), "true".to_string()),
                ("autofocus".to_string(), String::new()),
                ("autocomplete".to_string(), "email".to_string()),
                ("inputMode".to_string(), "email".to_string()),
                ("enterKeyHint".to_string(), "send".to_string()),
                ("autoCapitalize".to_string(), "sentences".to_string()),
                ("autoCorrect".to_string(), "on".to_string()),
                ("virtualKeyboardPolicy".to_string(), "manual".to_string()),
                ("pattern".to_string(), ".+@example\\.com".to_string()),
                ("minLength".to_string(), "3".to_string()),
                ("maxLength".to_string(), "64".to_string()),
                ("size".to_string(), "32".to_string()),
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
            attributes: BTreeMap::from([
                ("readonly".to_string(), String::new()),
                ("rows".to_string(), "6".to_string()),
                ("cols".to_string(), "40".to_string()),
                ("maxlength".to_string(), "280".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let select = CompiledJsxNode::Element {
        key: "projects".to_string(),
        tag: "select".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([("multiple".to_string(), String::new())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native_input = bridge.lower_to_native(&input).unwrap();
    let native_textarea = bridge.lower_to_native(&textarea).unwrap();
    let native_select = bridge.lower_to_native(&select).unwrap();

    assert_eq!(native_input.role, NativeRole::TextField);
    assert!(native_input.props.read_only);
    assert!(native_input.props.auto_focus);
    assert_eq!(native_input.props.autocomplete.as_deref(), Some("email"));
    assert_eq!(native_input.props.input_mode.as_deref(), Some("email"));
    assert_eq!(native_input.props.enter_key_hint.as_deref(), Some("send"));
    assert_eq!(
        native_input.props.auto_capitalize.as_deref(),
        Some("sentences")
    );
    assert_eq!(native_input.props.auto_correct.as_deref(), Some("on"));
    assert_eq!(
        native_input.props.virtual_keyboard_policy.as_deref(),
        Some("manual")
    );
    assert_eq!(
        native_input.props.pattern.as_deref(),
        Some(".+@example\\.com")
    );
    assert_eq!(native_input.props.min_length, Some(3));
    assert_eq!(native_input.props.max_length, Some(64));
    assert_eq!(native_input.props.size, Some(32));
    assert_eq!(
        native_input
            .props
            .metadata
            .get("readOnly")
            .map(String::as_str),
        Some("true")
    );

    assert_eq!(native_textarea.role, NativeRole::TextField);
    assert!(native_textarea.props.read_only);
    assert_eq!(native_textarea.props.rows, Some(6));
    assert_eq!(native_textarea.props.cols, Some(40));
    assert_eq!(native_textarea.props.max_length, Some(280));

    assert_eq!(native_select.role, NativeRole::Select);
    assert!(native_select.props.multiple);
}

#[test]
fn lowers_top_level_protocol_form_hints_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let file_input: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "avatar",
          "tag": "input",
          "props": {
            "inputType": "image",
            "name": "avatar",
            "form": "profile-form",
            "accept": "image/*",
            "capture": "environment",
            "alt": "Upload profile",
            "src": "/submit.png",
            "list": "avatar-presets",
            "dirname": "avatar.dir",
            "formAction": "/profiles/avatar",
            "formEncType": "multipart/form-data",
            "formMethod": "post",
            "formTarget": "_blank",
            "formNoValidate": true
          }
        }
        "#,
    )
    .unwrap();
    let override_input: CompiledJsxNode = serde_json::from_str(
        r#"
        {
          "kind": "element",
          "key": "search",
          "tag": "input",
          "props": {
            "inputType": "email",
            "attributes": {"type": "search"}
          }
        }
        "#,
    )
    .unwrap();

    let native_file = bridge.lower_to_native(&file_input).unwrap();
    let native_override = bridge.lower_to_native(&override_input).unwrap();

    assert_eq!(native_file.role, NativeRole::Button);
    assert_eq!(native_file.props.input_type.as_deref(), Some("image"));
    assert_eq!(native_file.props.name.as_deref(), Some("avatar"));
    assert_eq!(native_file.props.form.as_deref(), Some("profile-form"));
    assert_eq!(native_file.props.accept.as_deref(), Some("image/*"));
    assert_eq!(native_file.props.capture.as_deref(), Some("environment"));
    assert_eq!(native_file.props.alt.as_deref(), Some("Upload profile"));
    assert_eq!(native_file.props.src.as_deref(), Some("/submit.png"));
    assert_eq!(native_file.props.list.as_deref(), Some("avatar-presets"));
    assert_eq!(native_file.props.dirname.as_deref(), Some("avatar.dir"));
    assert_eq!(
        native_file.props.form_action.as_deref(),
        Some("/profiles/avatar")
    );
    assert_eq!(
        native_file.props.form_enctype.as_deref(),
        Some("multipart/form-data")
    );
    assert_eq!(native_file.props.form_method.as_deref(), Some("post"));
    assert_eq!(native_file.props.form_target.as_deref(), Some("_blank"));
    assert!(native_file.props.form_no_validate);
    assert_eq!(native_override.role, NativeRole::TextField);
    assert_eq!(native_override.props.input_type.as_deref(), Some("search"));
}

#[test]
fn lowers_html_form_submission_attributes_to_native_state() {
    let bridge = ReactCompilerBridge::new();
    let form = CompiledJsxNode::Element {
        key: "profile-form".to_string(),
        tag: "form".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("name".to_string(), "profile".to_string()),
                ("action".to_string(), "/profiles".to_string()),
                ("method".to_string(), "post".to_string()),
                ("encType".to_string(), "multipart/form-data".to_string()),
                ("target".to_string(), "_self".to_string()),
                ("noValidate".to_string(), "true".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let file_input = CompiledJsxNode::Element {
        key: "avatar".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "file".to_string()),
                ("name".to_string(), "avatar".to_string()),
                ("form".to_string(), "profile-form".to_string()),
                ("accept".to_string(), "image/*".to_string()),
                ("capture".to_string(), "environment".to_string()),
                ("list".to_string(), "avatar-presets".to_string()),
                ("dirname".to_string(), "avatar.dir".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let image_submit = CompiledJsxNode::Element {
        key: "submit-image".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "image".to_string()),
                ("alt".to_string(), "Upload profile".to_string()),
                ("src".to_string(), "/submit.png".to_string()),
                ("formAction".to_string(), "/profiles/avatar".to_string()),
                ("formEncType".to_string(), "multipart/form-data".to_string()),
                ("formMethod".to_string(), "post".to_string()),
                ("formTarget".to_string(), "_blank".to_string()),
                ("formNoValidate".to_string(), String::new()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let textarea = CompiledJsxNode::Element {
        key: "bio".to_string(),
        tag: "textarea".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("name".to_string(), "bio".to_string()),
                ("form".to_string(), "profile-form".to_string()),
                ("dirname".to_string(), "bio.dir".to_string()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let button = CompiledJsxNode::Element {
        key: "publish".to_string(),
        tag: "button".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "submit".to_string()),
                ("name".to_string(), "publish".to_string()),
                ("form".to_string(), "profile-form".to_string()),
                ("formaction".to_string(), "/profiles/publish".to_string()),
                (
                    "formenctype".to_string(),
                    "application/x-www-form-urlencoded".to_string(),
                ),
                ("formmethod".to_string(), "dialog".to_string()),
                ("formtarget".to_string(), "_top".to_string()),
                ("formnovalidate".to_string(), String::new()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let plain_button = CompiledJsxNode::Element {
        key: "preview".to_string(),
        tag: "button".to_string(),
        import_source: None,
        props: CompiledProps {
            attributes: BTreeMap::from([
                ("type".to_string(), "button".to_string()),
                ("formAction".to_string(), "/profiles/preview".to_string()),
                ("formNoValidate".to_string(), String::new()),
            ]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };

    let native_form = bridge.lower_to_native(&form).unwrap();
    let native_file = bridge.lower_to_native(&file_input).unwrap();
    let native_image = bridge.lower_to_native(&image_submit).unwrap();
    let native_textarea = bridge.lower_to_native(&textarea).unwrap();
    let native_button = bridge.lower_to_native(&button).unwrap();
    let native_plain_button = bridge.lower_to_native(&plain_button).unwrap();

    assert_eq!(native_form.role, NativeRole::Form);
    assert_eq!(native_form.props.name.as_deref(), Some("profile"));
    assert_eq!(native_form.props.form_action.as_deref(), Some("/profiles"));
    assert_eq!(native_form.props.form_method.as_deref(), Some("post"));
    assert_eq!(
        native_form.props.form_enctype.as_deref(),
        Some("multipart/form-data")
    );
    assert_eq!(native_form.props.form_target.as_deref(), Some("_self"));
    assert!(native_form.props.form_no_validate);

    assert_eq!(native_file.role, NativeRole::TextField);
    assert_eq!(native_file.props.name.as_deref(), Some("avatar"));
    assert_eq!(native_file.props.form.as_deref(), Some("profile-form"));
    assert_eq!(native_file.props.input_type.as_deref(), Some("file"));
    assert_eq!(native_file.props.accept.as_deref(), Some("image/*"));
    assert_eq!(native_file.props.capture.as_deref(), Some("environment"));
    assert_eq!(native_file.props.list.as_deref(), Some("avatar-presets"));
    assert_eq!(native_file.props.dirname.as_deref(), Some("avatar.dir"));

    assert_eq!(native_image.role, NativeRole::Button);
    assert_eq!(native_image.props.label.as_deref(), Some("Upload profile"));
    assert_eq!(native_image.props.input_type.as_deref(), Some("image"));
    assert_eq!(native_image.props.alt.as_deref(), Some("Upload profile"));
    assert_eq!(native_image.props.src.as_deref(), Some("/submit.png"));
    assert_eq!(
        native_image.props.form_action.as_deref(),
        Some("/profiles/avatar")
    );
    assert_eq!(
        native_image.props.form_enctype.as_deref(),
        Some("multipart/form-data")
    );
    assert_eq!(native_image.props.form_method.as_deref(), Some("post"));
    assert_eq!(native_image.props.form_target.as_deref(), Some("_blank"));
    assert!(native_image.props.form_no_validate);

    assert_eq!(native_textarea.role, NativeRole::TextField);
    assert_eq!(native_textarea.props.name.as_deref(), Some("bio"));
    assert_eq!(native_textarea.props.form.as_deref(), Some("profile-form"));
    assert_eq!(native_textarea.props.dirname.as_deref(), Some("bio.dir"));

    assert_eq!(native_button.role, NativeRole::Button);
    assert_eq!(native_button.props.name.as_deref(), Some("publish"));
    assert_eq!(native_button.props.form.as_deref(), Some("profile-form"));
    assert_eq!(native_button.props.input_type.as_deref(), Some("submit"));
    assert_eq!(
        native_button.props.form_action.as_deref(),
        Some("/profiles/publish")
    );
    assert_eq!(
        native_button.props.form_enctype.as_deref(),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(native_button.props.form_method.as_deref(), Some("dialog"));
    assert_eq!(native_button.props.form_target.as_deref(), Some("_top"));
    assert!(native_button.props.form_no_validate);

    assert_eq!(native_plain_button.role, NativeRole::Button);
    assert_eq!(
        native_plain_button.props.input_type.as_deref(),
        Some("button")
    );
    assert_eq!(native_plain_button.props.form_action, None);
    assert!(!native_plain_button.props.form_no_validate);
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

    let protocol_number = CompiledJsxNode::Element {
        key: "quantity-protocol".to_string(),
        tag: "input".to_string(),
        import_source: None,
        props: CompiledProps {
            value_number: Some(7.0),
            attributes: BTreeMap::from([("type".to_string(), "number".to_string())]),
            ..CompiledProps::default()
        },
        children: Vec::new(),
    };
    let native_protocol_number = bridge.lower_to_native(&protocol_number).unwrap();

    assert_eq!(native_protocol_number.role, NativeRole::TextField);
    assert_eq!(native_protocol_number.props.value.as_deref(), Some("7"));
    assert_eq!(native_protocol_number.props.current, Some(7.0));

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
