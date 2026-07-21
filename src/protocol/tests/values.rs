use super::*;

#[test]
fn native_protocol_session_infers_container_selection_value_from_selected_child() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setTheme"}],
              "root": {
                "kind": "element",
                "key": "theme",
                "tag": "Select",
                "props": {
                  "label": "Theme",
                  "events": {"onSelectionChange": "setTheme"}
                },
                "children": [
                  {
                    "kind": "element",
                    "key": "compact",
                    "tag": "ListBoxItem",
                    "props": {"label": "Compact", "value": "compact"}
                  },
                  {
                    "kind": "element",
                    "key": "comfortable",
                    "tag": "ListBoxItem",
                    "props": {
                      "label": "Comfortable",
                      "value": "comfortable",
                      "isSelected": true
                    }
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::SelectionChange).value(""),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("comfortable")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("comfortable")
    );
    assert_eq!(
        session.runtime().actions().invocations()[0]
            .value
            .as_deref(),
        Some("comfortable")
    );
}

#[test]
fn native_protocol_session_clamps_text_change_values_to_max_length() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setName"}],
              "root": {
                "kind": "element",
                "key": "name",
                "tag": "TextField",
                "props": {
                  "value": "Ada",
                  "attributes": {"maxLength": "3"},
                  "events": {"onChange": "setName"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("aé日b"),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("aé日")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("aé日")
    );
    assert_eq!(
        session.runtime().actions().invocations()[0]
            .value
            .as_deref(),
        Some("aé日")
    );
}

#[test]
fn native_protocol_session_clamps_initial_text_value_to_max_length_before_rendering() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "name",
                "tag": "TextField",
                "props": {
                  "value": "aé日b",
                  "attributes": {"maxLength": "3"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.control_state.max_length, Some(3));
    assert_eq!(blueprint.value.as_deref(), Some("aé日"));
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("aé日")
    );
}

#[test]
fn native_protocol_session_clamps_slider_change_values_to_range_bounds() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setEstimate"}],
              "root": {
                "kind": "element",
                "key": "estimate",
                "tag": "Slider",
                "props": {
                  "minValue": 1,
                  "maxValue": 12,
                  "valueNumber": 6,
                  "events": {"onChange": "setEstimate"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value(" 99 "),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        session.runtime().actions().invocations()[0]
            .value
            .as_deref(),
        Some("12")
    );

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value(" 0 "),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        session.runtime().actions().invocations()[1]
            .value
            .as_deref(),
        Some("1")
    );
}

#[test]
fn native_protocol_session_clamps_number_input_change_values_to_range_bounds() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setEstimate"}],
              "root": {
                "kind": "element",
                "key": "estimate",
                "tag": "input",
                "props": {
                  "inputType": "number",
                  "minValue": 1,
                  "maxValue": 12,
                  "valueNumber": 6,
                  "events": {"onChange": "setEstimate"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value(" 99 "),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        session.runtime().actions().invocations()[0]
            .value
            .as_deref(),
        Some("12")
    );

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value(" 0 "),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        session.runtime().actions().invocations()[1]
            .value
            .as_deref(),
        Some("1")
    );
}

#[test]
fn native_protocol_session_suppresses_invalid_numeric_change_values() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setEstimate"}],
              "root": {
                "kind": "element",
                "key": "estimate",
                "tag": "Slider",
                "props": {
                  "minValue": 1,
                  "maxValue": 12,
                  "valueNumber": 6,
                  "events": {"onChange": "setEstimate"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("not-a-number"),
        })
        .unwrap();

    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("6")
    );
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_snaps_ranged_change_values_to_step() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setVolume"}],
              "root": {
                "kind": "element",
                "key": "volume",
                "tag": "Slider",
                "props": {
                  "minValue": 0,
                  "maxValue": 100,
                  "valueNumber": 50,
                  "stepValue": 5,
                  "events": {"onChange": "setVolume"}
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("43"),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("45")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("45")
    );
    assert_eq!(
        session.runtime().actions().invocations()[0]
            .value
            .as_deref(),
        Some("45")
    );

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("42"),
        })
        .unwrap();

    assert_eq!(
        response
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("40")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("40")
    );
    assert_eq!(
        session.runtime().actions().invocations()[1]
            .value
            .as_deref(),
        Some("40")
    );
}

#[test]
fn native_protocol_session_normalizes_initial_ranged_values_before_rendering() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "volume",
                "tag": "Slider",
                "props": {
                  "minValue": 0,
                  "maxValue": 100,
                  "valueNumber": 43,
                  "stepValue": 5
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.control_state.current, Some(45.0));
    assert_eq!(blueprint.value.as_deref(), Some("45"));
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("45")
    );
}

#[test]
fn native_protocol_session_normalizes_initial_number_input_values_before_rendering() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "estimate",
                "tag": "input",
                "props": {
                  "inputType": "number",
                  "minValue": 1,
                  "maxValue": 12,
                  "valueNumber": 99
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.control_state.current, Some(12.0));
    assert_eq!(blueprint.value.as_deref(), Some("12"));
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("12")
    );
}

#[test]
fn native_protocol_session_omits_invalid_initial_numeric_values() {
    let range_frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "volume",
                "tag": "input",
                "props": {
                  "attributes": {"type": "range", "defaultValue": "not-a-number"},
                  "minValue": 0,
                  "maxValue": 100
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&range_frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.role, NativeRole::Slider);
    assert_eq!(blueprint.control_state.current, None);
    assert_eq!(blueprint.value, None);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        None
    );

    let number_frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "estimate",
                "tag": "input",
                "props": {
                  "attributes": {"type": "number", "value": ""},
                  "minValue": 1,
                  "maxValue": 12
                }
              }
            }
            "#,
    )
    .unwrap();

    let response = session.render_frame(&number_frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.role, NativeRole::TextField);
    assert_eq!(blueprint.control_state.current, None);
    assert_eq!(blueprint.value, None);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        None
    );
}

#[test]
fn native_protocol_session_projects_textarea_default_value_attributes() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "notes",
                "tag": "textarea",
                "props": {
                  "attributes": {"defaultValue": "Draft notes"}
                },
                "children": [
                  {"kind": "text", "key": "notes-text", "value": "Ignored child text"}
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let blueprint = &session
        .runtime()
        .host()
        .node(response.root)
        .unwrap()
        .blueprint;

    assert_eq!(blueprint.role, NativeRole::TextField);
    assert_eq!(blueprint.value.as_deref(), Some("Draft notes"));
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("Draft notes")
    );
}
