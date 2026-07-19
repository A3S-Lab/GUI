use super::*;

#[test]
fn native_protocol_session_updates_window_style_options() {
    let first: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "window": {"title": "Profile", "width": 640, "height": 480},
              "root": {
                "kind": "element",
                "key": "content",
                "tag": "Group",
                "children": [{"kind": "text", "key": "text", "value": "Profile"}]
              }
            }
            "#,
    )
    .unwrap();
    let second: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "window": {
                "title": "Profile",
                "width": 800,
                "height": 560,
                "minWidth": 520,
                "minHeight": 360
              },
              "root": {
                "kind": "element",
                "key": "content",
                "tag": "Group",
                "children": [{"kind": "text", "key": "text", "value": "Profile"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let first_response = session.render_frame(&first).unwrap();
    let second_response = session.render_frame(&second).unwrap();

    assert_eq!(second_response.root, first_response.root);
    assert!(second_response.commands.iter().any(|command| matches!(
        command,
        crate::platform::PlatformCommand::Update { id, blueprint }
            if *id == first_response.root
                && blueprint
                    .portable_style
                    .width
                    .as_ref()
                    .and_then(|value| value.points()) == Some(800.0)
                && blueprint
                    .portable_style
                    .height
                    .as_ref()
                    .and_then(|value| value.points()) == Some(560.0)
                && blueprint
                    .portable_style
                    .min_width
                    .as_ref()
                    .and_then(|value| value.points()) == Some(520.0)
                && blueprint
                    .portable_style
                    .min_height
                    .as_ref()
                    .and_then(|value| value.points()) == Some(360.0)
    )));
    assert!(second_response
        .commands
        .iter()
        .all(|command| { !matches!(command, crate::platform::PlatformCommand::Create { .. }) }));
}

#[test]
fn native_protocol_session_returns_rendered_accessibility_tree() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {
                  "isReadOnly": true,
                  "attributes": {
                    "aria-label": "Save profile",
                    "aria-describedby": "save-help",
                    "aria-description": "Writes profile changes",
                    "aria-pressed": "false"
                  }
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let accessibility = response.accessibility_tree.as_ref().unwrap();

    assert_eq!(accessibility.node, Some(response.root));
    assert_eq!(accessibility.role, AccessibilityRole::Button);
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
    assert!(accessibility.read_only);
    assert!(!accessibility.focused);
    assert_eq!(
        accessibility.relationships.described_by.as_deref(),
        Some("save-help")
    );
    assert_eq!(
        accessibility.description.description.as_deref(),
        Some("Writes profile changes")
    );
    assert_eq!(accessibility.state.pressed.as_deref(), Some("false"));
    assert_eq!(session.accessibility_tree(), response.accessibility_tree);

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains(r#""accessibilityTree""#));
    assert!(json.contains(r#""role":"button""#));
    assert!(json.contains(r#""readOnly":true"#));
}

#[test]
fn native_protocol_session_projects_auto_focus_on_render() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {
                  "attributes": {
                    "aria-label": "Save profile",
                    "autoFocus": "true"
                  }
                }
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let accessibility = response.accessibility_tree.as_ref().unwrap();

    assert_eq!(accessibility.node, Some(response.root));
    assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
    assert!(accessibility.focused);
    assert!(session.runtime().interactions().changes().is_empty());
}

#[test]
fn native_protocol_session_skips_disabled_subtree_auto_focus() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "tools",
                "tag": "Toolbar",
                "children": [
                  {
                    "kind": "element",
                    "key": "review-gate",
                    "tag": "FieldSet",
                    "props": {"isDisabled": true, "label": "Review gate"},
                    "children": [
                      {
                        "kind": "element",
                        "key": "finish-review",
                        "tag": "Button",
                        "props": {
                          "attributes": {
                            "aria-label": "Complete review",
                            "autoFocus": "true"
                          }
                        }
                      }
                    ]
                  },
                  {
                    "kind": "element",
                    "key": "title",
                    "tag": "TextField",
                    "props": {
                      "attributes": {
                        "aria-label": "Task title",
                        "autoFocus": "true"
                      }
                    }
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let accessibility = response.accessibility_tree.as_ref().unwrap();

    assert_eq!(accessibility.children.len(), 2);
    assert_eq!(
        accessibility.children[0].children[0].label.as_deref(),
        Some("Complete review")
    );
    assert!(!accessibility.children[0].children[0].focused);
    assert_eq!(
        accessibility.children[1].label.as_deref(),
        Some("Task title")
    );
    assert!(accessibility.children[1].focused);
    assert!(session.runtime().interactions().changes().is_empty());
}

#[test]
fn native_protocol_session_omits_hidden_accessibility_subtrees() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "root": {
                "kind": "element",
                "key": "tools",
                "tag": "Toolbar",
                "children": [
                  {
                    "kind": "element",
                    "key": "save",
                    "tag": "Button",
                    "props": {"label": "Save"}
                  },
                  {
                    "kind": "element",
                    "key": "archive",
                    "tag": "Button",
                    "props": {
                      "label": "Archive",
                      "attributes": {"hidden": "true"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "preview",
                    "tag": "Button",
                    "props": {
                      "label": "Preview",
                      "attributes": {"aria-hidden": "true"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "details",
                    "tag": "Button",
                    "props": {
                      "label": "Details",
                      "style": {"display": "none"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "filters",
                    "tag": "Button",
                    "props": {
                      "label": "Filters",
                      "style": {"visibility": "hidden"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "summary",
                    "tag": "Button",
                    "props": {
                      "label": "Summary",
                      "style": {"contentVisibility": "hidden"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "activity",
                    "tag": "Button",
                    "props": {
                      "label": "Activity",
                      "style": {"interactivity": "inert"}
                    }
                  },
                  {
                    "kind": "element",
                    "key": "dialog",
                    "tag": "dialog",
                    "children": [
                      {"kind": "text", "key": "dialog-text", "value": "Dialog"}
                    ]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let accessibility = response.accessibility_tree.as_ref().unwrap();

    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
}

#[test]
fn native_protocol_session_dispatches_active_frame_events() {
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
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap();
    let error = session
        .dispatch_host_event(&HostEvent {
            frame_id: "other".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap_err();
    let empty_frame_error = session
        .dispatch_host_event(&HostEvent {
            frame_id: String::new(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap_err();
    let zero_node_error = session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(HostNodeId::new(0), NativeEventKind::Press),
        })
        .unwrap_err();

    assert_eq!(response.invocation.action, "saveProfile");
    assert!(error.to_string().contains("active frame profile"));
    assert!(empty_frame_error.to_string().contains("non-empty frame id"));
    assert!(zero_node_error.to_string().contains("non-zero node id"));
    assert_eq!(session.runtime().actions().invocations().len(), 1);
}
