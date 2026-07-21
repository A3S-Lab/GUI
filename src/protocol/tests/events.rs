use super::*;

#[test]
fn ui_frame_infers_actions_from_compiled_event_props_when_actions_are_omitted() {
    let frame: UiFrame = serde_json::from_str(
            r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "toolbar",
                "tag": "Toolbar",
                "children": [
                  {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {"events": {"onPress": "saveProfile"}},
                    "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                  },
                  {
                    "kind": "element",
                    "key": "save-labeled",
                    "tag": "Button",
                    "props": {
                      "events": {"onClick": "saveProfile"},
                      "actionLabels": {"saveProfile": "Save profile"}
                    },
                    "children": [{"kind": "text", "key": "save-labeled-text", "value": "Save labeled"}]
                  },
                  {
                    "kind": "element",
                    "key": "query",
                    "tag": "Input",
                    "props": {
                      "events": {"onKeyDown": "handleSearchKey"},
                      "actionLabels": {"handleSearchKey": "Handle search key"}
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
    let toolbar = session.runtime().host().node(rendered.root).unwrap();
    let save = toolbar.children[0];
    let response = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::Press),
        })
        .unwrap();

    assert_eq!(
        frame.actions,
        vec![
            UiAction {
                id: "saveProfile".to_string(),
                disabled: false,
                label: Some("Save profile".to_string()),
            },
            UiAction {
                id: "handleSearchKey".to_string(),
                disabled: false,
                label: Some("Handle search key".to_string()),
            },
        ]
    );
    assert_eq!(response.invocation.action, "saveProfile");
    assert_eq!(
        session
            .runtime()
            .actions()
            .registered("saveProfile")
            .and_then(|action| action.label.as_deref()),
        Some("Save profile")
    );
}

#[test]
fn explicit_ui_frame_actions_override_compiled_event_inference() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "explicitAction", "label": "Explicit action"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {
                  "events": {"onPress": "saveProfile"},
                  "actionLabels": {"saveProfile": "Save profile"}
                },
                "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();

    assert_eq!(
        frame.actions,
        vec![UiAction {
            id: "explicitAction".to_string(),
            disabled: false,
            label: Some("Explicit action".to_string()),
        }]
    );
}

#[test]
fn native_protocol_session_dispatches_keyboard_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "search",
              "actions": [{"id": "handleSearchKey"}],
              "root": {
                "kind": "element",
                "key": "query",
                "tag": "Input",
                "props": {"events": {"onKeyDown": "handleSearchKey"}}
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .dispatch_host_event(&HostEvent {
            frame_id: "search".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::KeyDown).value(" Return "),
        })
        .unwrap();

    assert_eq!(response.invocation.action, "handleSearchKey");
    assert_eq!(response.invocation.event, NativeEventKind::KeyDown);
    assert_eq!(response.invocation.value.as_deref(), Some("Enter"));
    assert!(response.interaction_changes.is_empty());
}

#[test]
fn native_protocol_session_routes_activation_keys_to_press_actions() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"events": {"onPress": "saveProfile"}},
                "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::KeyDown).value("Enter"),
        })
        .unwrap();

    assert_eq!(response.invocation.action, "saveProfile");
    assert_eq!(response.invocation.event, NativeEventKind::KeyDown);
    assert_eq!(response.invocation.value.as_deref(), Some("Enter"));
    assert!(response.interaction_changes.is_empty());
}

#[test]
fn native_protocol_session_routes_space_key_to_toggle_actions() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setNotifications"}],
              "root": {
                "kind": "element",
                "key": "notifications",
                "tag": "Switch",
                "props": {
                  "isChecked": false,
                  "events": {"onChange": "setNotifications"}
                },
                "children": [{"kind": "text", "key": "label", "value": "Notifications"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let response = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::KeyDown).value(" "),
        })
        .unwrap();

    assert_eq!(response.invocation.action, "setNotifications");
    assert_eq!(response.invocation.event, NativeEventKind::Toggle);
    assert_eq!(response.invocation.value.as_deref(), Some("true"));
    assert_eq!(response.interaction_changes[0].after.checked, Some(true));
}

#[test]
fn native_protocol_session_canonicalizes_boolean_event_payloads() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setNotifications"}, {"id": "setFocus"}],
              "root": {
                "kind": "element",
                "key": "notifications",
                "tag": "Switch",
                "props": {
                  "isChecked": false,
                  "events": {
                    "onChange": "setNotifications",
                    "onFocusChange": "setFocus"
                  }
                },
                "children": [{"kind": "text", "key": "label", "value": "Notifications"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let focus = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Focus).value("maybe"),
        })
        .unwrap();

    assert_eq!(focus.invocation.action, "setFocus");
    assert_eq!(focus.invocation.value.as_deref(), Some("true"));
    assert_eq!(focus.interaction_changes[0].after.focused, true);

    let toggle = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Toggle).value("not-a-bool"),
        })
        .unwrap();

    assert_eq!(toggle.invocation.action, "setNotifications");
    assert_eq!(toggle.invocation.value.as_deref(), Some("true"));
    assert_eq!(toggle.interaction_changes[0].after.checked, Some(true));
}

#[test]
fn native_protocol_session_canonicalizes_boolean_change_payloads() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setNotifications"}],
              "root": {
                "kind": "element",
                "key": "notifications",
                "tag": "Switch",
                "props": {
                  "isChecked": false,
                  "events": {"onChange": "setNotifications"}
                },
                "children": [{"kind": "text", "key": "label", "value": "Notifications"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    let first = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("1"),
        })
        .unwrap();
    let second = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("not-a-bool"),
        })
        .unwrap();

    assert_eq!(first.invocation.action, "setNotifications");
    assert_eq!(first.invocation.value.as_deref(), Some("true"));
    assert_eq!(first.interaction_changes[0].after.checked, Some(true));
    assert_eq!(second.invocation.value.as_deref(), Some("false"));
    assert_eq!(second.interaction_changes[0].after.checked, Some(false));
}

#[test]
fn native_protocol_session_preserves_ancestor_key_down_handlers() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "handleRowKey"}, {"id": "setNotifications"}],
              "root": {
                "kind": "element",
                "key": "row",
                "tag": "Group",
                "props": {"events": {"onKeyDown": "handleRowKey"}},
                "children": [
                  {
                    "kind": "element",
                    "key": "notifications",
                    "tag": "Switch",
                    "props": {
                      "isChecked": false,
                      "events": {"onChange": "setNotifications"}
                    },
                    "children": [{"kind": "text", "key": "label", "value": "Notifications"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();
    let switch = session
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let response = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(switch, NativeEventKind::KeyDown).value(" "),
        })
        .unwrap();

    assert_eq!(response.invocation.action, "handleRowKey");
    assert_eq!(response.invocation.event, NativeEventKind::KeyDown);
    assert!(response.interaction_changes.is_empty());
}

#[test]
fn native_protocol_session_replaces_registered_actions_on_render() {
    let first: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"events": {"onPress": "saveProfile"}},
                "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let second: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"events": {"onPress": "saveProfile"}},
                "children": [{"kind": "text", "key": "save-text", "value": "Saved"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let first_response = session.render_frame(&first).unwrap();
    assert!(session.runtime().actions().contains("saveProfile"));
    session.render_frame(&second).unwrap();
    assert!(!session.runtime().actions().contains("saveProfile"));
    let error = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(first_response.root, NativeEventKind::Press),
        })
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("unregistered action saveProfile"));
}

#[test]
fn native_protocol_session_rejects_disabled_registered_actions() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile", "disabled": true}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"events": {"onPress": "saveProfile"}},
                "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();

    assert!(session.runtime().actions().is_disabled("saveProfile"));
    let error = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap_err();

    assert!(error.to_string().contains("disabled action saveProfile"));
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_handles_state_event_without_action() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"attributes": {"aria-label": "Save profile"}}
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
            event: NativeEvent::new(rendered.root, NativeEventKind::Focus),
        })
        .unwrap();
    let accessibility = response.accessibility_tree.as_ref().unwrap();

    assert!(response.invocation.is_none());
    assert_eq!(accessibility.node, Some(rendered.root));
    assert!(accessibility.focused);
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
    assert_eq!(response.interaction_changes.len(), 1);
    assert_eq!(response.interaction_changes[0].node, rendered.root);
    assert!(response.interaction_changes[0].after.focused);
}

#[test]
fn native_protocol_session_suppresses_disabled_user_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {
                  "isDisabled": true,
                  "events": {"onPress": "saveProfile"}
                },
                "children": [{"kind": "text", "key": "label", "value": "Save"}]
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
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap();

    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .map(|tree| tree.disabled),
        Some(true)
    );
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_disabled_subtree_user_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "review-gate",
                "tag": "FieldSet",
                "props": {"isDisabled": true, "label": "Review gate"},
                "children": [
                  {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {"events": {"onPress": "saveProfile"}},
                    "children": [{"kind": "text", "key": "label", "value": "Save"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();
    let save = session
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let press = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::Press),
        })
        .unwrap();
    let key = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::KeyDown).value("Enter"),
        })
        .unwrap();

    assert!(press.invocation.is_none());
    assert!(press.interaction_changes.is_empty());
    assert!(key.invocation.is_none());
    assert!(key.interaction_changes.is_empty());
    assert_eq!(
        press.accessibility_tree.as_ref().map(|tree| tree.disabled),
        Some(true)
    );
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_inert_subtree_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "tools",
                "tag": "Toolbar",
                "props": {"attributes": {"inert": "true"}},
                "children": [
                  {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {"events": {"onPress": "saveProfile"}},
                    "children": [{"kind": "text", "key": "label", "value": "Save"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();
    let save = session
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::Press),
        })
        .unwrap();

    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_css_inert_subtree_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
              "root": {
                "kind": "element",
                "key": "tools",
                "tag": "Toolbar",
                "props": {"style": {"interactivity": "inert"}},
                "children": [
                  {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {"events": {"onPress": "saveProfile"}},
                    "children": [{"kind": "text", "key": "label", "value": "Save"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();
    let save = session
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[0];

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::Press),
        })
        .unwrap();

    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert!(response.accessibility_tree.is_none());
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_read_only_value_events() {
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
                  "isReadOnly": true,
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
            event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("Grace"),
        })
        .unwrap();

    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("Ada")
    );
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_read_only_selection_events() {
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
                  "isReadOnly": true,
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

    let inferred = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::SelectionChange),
        })
        .unwrap();
    let explicit = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::SelectionChange)
                .value("compact"),
        })
        .unwrap();

    assert!(inferred.invocation.is_none());
    assert!(inferred.interaction_changes.is_empty());
    assert_eq!(
        inferred
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("comfortable")
    );
    assert!(explicit.invocation.is_none());
    assert!(explicit.interaction_changes.is_empty());
    assert_eq!(
        explicit
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("comfortable")
    );
    assert!(session.runtime().actions().invocations().is_empty());
}

#[test]
fn native_protocol_session_suppresses_read_only_ancestor_selection_value_events() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "setTheme"}],
              "root": {
                "kind": "element",
                "key": "theme",
                "tag": "RadioGroup",
                "props": {
                  "label": "Theme",
                  "isReadOnly": true,
                  "events": {"onSelectionChange": "setTheme"}
                },
                "children": [
                  {
                    "kind": "element",
                    "key": "light",
                    "tag": "Radio",
                    "props": {
                      "label": "Light",
                      "value": "light",
                      "isSelected": true,
                      "isChecked": true
                    }
                  },
                  {
                    "kind": "element",
                    "key": "dark",
                    "tag": "Radio",
                    "props": {"label": "Dark", "value": "dark"}
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame).unwrap();
    let dark = session
        .runtime()
        .host()
        .node(rendered.root)
        .unwrap()
        .children[1];

    let selection = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(dark, NativeEventKind::SelectionChange),
        })
        .unwrap();
    let toggle = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(dark, NativeEventKind::Toggle),
        })
        .unwrap();

    assert!(selection.invocation.is_none());
    assert!(selection.interaction_changes.is_empty());
    assert!(toggle.invocation.is_none());
    assert!(toggle.interaction_changes.is_empty());
    let accessibility = toggle.accessibility_tree.as_ref().unwrap();
    assert_eq!(accessibility.value.as_deref(), Some("light"));
    assert!(accessibility.children[0].selected);
    assert_eq!(accessibility.children[0].checked, Some(true));
    assert!(!accessibility.children[1].selected);
    assert_eq!(accessibility.children[1].checked, Some(false));
    assert!(session.runtime().actions().invocations().is_empty());
}
