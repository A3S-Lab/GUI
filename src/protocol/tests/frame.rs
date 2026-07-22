use super::*;

#[test]
fn native_protocol_app_reduces_actions_and_renders_next_frame() {
    let mut app = NativeProtocolApp::new(
        Gtk4Adapter,
        CounterState::default(),
        counter_frame,
        counter_reduce,
    );
    let rendered = app.render().unwrap();

    let response = app
        .dispatch_host_event(&HostEvent {
            frame_id: "counter".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(
        response
            .invocation
            .as_ref()
            .map(|action| action.action.as_str()),
        Some("increment")
    );
    assert!(response.render.is_some());
    assert_eq!(response.render.as_ref().unwrap().root, rendered.root);
}

#[test]
#[cfg(feature = "authoring")]
fn native_protocol_app_resolves_rsx_state_bindings_after_reducer() {
    let mut app = NativeProtocolApp::new(
        Gtk4Adapter,
        CounterState::default(),
        rsx_counter_frame,
        counter_reduce,
    );
    let rendered = app.render().unwrap();
    assert_eq!(
        rendered
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 0")
    );

    let response = app
        .dispatch_host_event(&HostEvent {
            frame_id: "rsx-counter".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(
        response
            .invocation
            .as_ref()
            .map(|action| action.action.as_str()),
        Some("increment")
    );
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1")
    );
}

#[test]
fn native_protocol_app_handles_state_only_events_without_rerendering() {
    let mut app = NativeProtocolApp::new(
        Gtk4Adapter,
        CounterState::default(),
        counter_frame,
        counter_reduce,
    );
    let rendered = app.render().unwrap();

    let response = app
        .handle_host_event(&HostEvent {
            frame_id: "counter".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Focus),
        })
        .unwrap();

    assert_eq!(app.state().count, 0);
    assert!(response.invocation.is_none());
    assert!(response.render.is_none());
    assert_eq!(response.interaction_changes.len(), 1);
    assert!(response.accessibility_tree.unwrap().focused);
}

#[test]
fn protocol_renders_frame_and_dispatches_native_event_to_action() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "frame-1",
              "actions": [{"id": "saveProfile", "label": "Save profile"}],
              "window": {"title": "Profile", "width": 420, "height": 320},
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
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);

    let rendered = frame.render_into(&mut runtime).unwrap();
    let button = runtime
        .host()
        .planning()
        .node(rendered.root)
        .unwrap()
        .children[0];
    let response = HostEvent {
        frame_id: rendered.frame_id.clone(),
        event: NativeEvent::new(button, NativeEventKind::Press),
    }
    .dispatch_into(&mut runtime)
    .unwrap();

    assert_eq!(rendered.frame_id, "frame-1");
    assert_eq!(response.frame_id, "frame-1");
    assert_eq!(response.invocation.action, "saveProfile");
    assert_eq!(runtime.actions().invocations().len(), 1);
}

#[test]
#[cfg(feature = "authoring")]
fn rsx_source_frame_renders_tailwind_to_native_widgets_without_node_or_bun() {
    let frame = UiFrame::from_rsx_source(
            "rsx-native",
            r##"
            <Toolbar key="root" orientation="vertical" className="min-w-[920px] gap-2 bg-[#efefef] p-3">
              <Button key="save" onPress={saveDocument} className="rounded-md border border-[#ebebeb]">
                Save
              </Button>
            </Toolbar>
            "##,
        )
        .unwrap();
    assert_eq!(frame.actions.len(), 1);
    assert_eq!(frame.actions[0].id, "saveDocument");

    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);
    let rendered = frame.render_into(&mut runtime).unwrap();
    let root = runtime.host().planning().node(rendered.root).unwrap();

    assert_eq!(root.blueprint.widget_class, "gtk::Box(toolbar)");
    assert_eq!(
        root.blueprint.class_name.as_deref(),
        Some("min-w-[920px] gap-2 bg-[#efefef] p-3")
    );
    assert!(root.blueprint.portable_style.min_width.is_some());
    assert!(root.blueprint.portable_style.background_color.is_some());

    let button = root.children[0];
    let response = HostEvent {
        frame_id: rendered.frame_id.clone(),
        event: NativeEvent::new(button, NativeEventKind::Press),
    }
    .dispatch_into(&mut runtime)
    .unwrap();

    assert_eq!(response.frame_id, "rsx-native");
    assert_eq!(response.invocation.action, "saveDocument");
}

#[test]
#[cfg(feature = "authoring")]
fn rsx_source_frame_rejects_unresolved_state_bindings() {
    let error =
        UiFrame::from_rsx_source("unresolved", r#"<Text key="title" label={state.title} />"#)
            .unwrap_err();

    assert!(error
        .to_string()
        .contains("cannot render unresolved RSX state/props/derived/context/resource bindings"));
}

#[test]
fn ui_frame_render_preserves_action_scope_after_failed_native_render() {
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
              "actions": [{"id": "deleteProfile"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "props": {"events": {"onPress": "deleteProfile"}},
                "children": [{"kind": "text", "key": "save-text", "value": "Delete"}]
              }
            }
            "#,
    )
    .unwrap();
    let mut runtime = GuiRuntime::new(FailingUpdateHost::default());

    first.render_into(&mut runtime).unwrap();
    assert!(runtime.actions().contains("saveProfile"));
    runtime.host_mut().fail_updates = true;
    let error = second.render_into(&mut runtime).unwrap_err();

    assert!(error.to_string().contains("forced host update failure"));
    assert!(runtime.actions().contains("saveProfile"));
    assert!(!runtime.actions().contains("deleteProfile"));
}

#[test]
fn native_protocol_session_returns_incremental_native_commands() {
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
              "actions": [{"id": "saveProfile"}],
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
    let second_response = session.render_frame(&second).unwrap();

    assert_eq!(first_response.frame_id, "profile");
    assert_eq!(session.active_frame_id(), Some("profile"));
    assert_eq!(session.root(), Some(first_response.root));
    assert!(first_response.commands.iter().any(|command| matches!(
        command,
        crate::platform::PlatformCommand::Create {
            blueprint,
            ..
        } if blueprint.widget_class == "gtk::Button"
            && blueprint.label.as_deref() == Some("Save")
    )));
    assert!(first_response
        .commands
        .iter()
        .any(|command| { matches!(command, crate::platform::PlatformCommand::SetRoot { .. }) }));
    assert_eq!(second_response.root, first_response.root);
    assert!(second_response.commands.iter().any(|command| matches!(
        command,
        crate::platform::PlatformCommand::Update {
            id,
            blueprint,
        } if *id == first_response.root && blueprint.label.as_deref() == Some("Saved")
    )));
    assert!(second_response
        .commands
        .iter()
        .all(|command| { !matches!(command, crate::platform::PlatformCommand::Create { .. }) }));
    assert!(session.runtime().host().commands().is_empty());
    assert!(session.pending_commands().is_empty());
}

#[test]
fn native_protocol_session_ignores_stale_host_events_after_rerender_removes_node() {
    let first: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "profile",
              "actions": [{"id": "saveProfile"}],
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
                    "key": "cancel",
                    "tag": "Button",
                    "children": [{"kind": "text", "key": "cancel-text", "value": "Cancel"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let second: UiFrame = serde_json::from_str(
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
                    "key": "cancel",
                    "tag": "Button",
                    "children": [{"kind": "text", "key": "cancel-text", "value": "Cancel"}]
                  }
                ]
              }
            }
            "#,
    )
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let first_response = session.render_frame(&first).unwrap();
    let save = session
        .runtime()
        .host()
        .node(first_response.root)
        .unwrap()
        .children[0];
    session.render_frame(&second).unwrap();

    let response = session
        .handle_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event: NativeEvent::new(save, NativeEventKind::Press),
        })
        .unwrap();

    let accessibility = response.accessibility_tree.unwrap();
    assert!(response.invocation.is_none());
    assert!(response.interaction_changes.is_empty());
    assert!(session.runtime().host().node(save).is_none());
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Cancel"));
}

#[test]
fn native_protocol_session_rejects_invalid_frame_contracts() {
    let valid: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "valid",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let empty_frame_id: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let text_root: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "text-root",
              "root": {"kind": "text", "key": "text-0", "value": "Loose text"}
            }
            "#,
    )
    .unwrap();
    let empty_action: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "empty-action",
              "actions": [{"id": ""}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let duplicate_action: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "duplicate-action",
              "actions": [{"id": "saveProfile"}, {"id": "saveProfile", "label": "Save"}],
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let empty_element_key: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "empty-element-key",
              "root": {
                "kind": "element",
                "key": "",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let empty_element_tag: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "empty-element-tag",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": ""
              }
            }
            "#,
    )
    .unwrap();
    let empty_text_key: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "empty-text-key",
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "children": [{"kind": "text", "key": "", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let duplicate_child_key: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "duplicate-child-key",
              "root": {
                "kind": "element",
                "key": "toolbar",
                "tag": "Toolbar",
                "children": [
                  {"kind": "element", "key": "save", "tag": "Button"},
                  {"kind": "element", "key": "save", "tag": "Button"}
                ]
              }
            }
            "#,
    )
    .unwrap();
    let negative_width: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "negative-width",
              "window": {"title": "Profile", "width": -1},
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let inverted_width_bounds: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "inverted-width-bounds",
              "window": {"title": "Profile", "minWidth": 800, "maxWidth": 640},
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let width_below_minimum: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "width-below-minimum",
              "window": {"title": "Profile", "width": 320, "minWidth": 640},
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap();
    let mut non_finite_window = valid.clone();
    non_finite_window.frame_id = "non-finite-window".to_string();
    non_finite_window.window = Some(WindowOptions {
        title: "Profile".to_string(),
        on_close: None,
        width: Some(f64::NAN),
        height: None,
        min_width: None,
        min_height: None,
        max_width: None,
        max_height: None,
        resizable: true,
    });

    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&valid).unwrap();

    let error = session.render_frame(&empty_frame_id).unwrap_err();
    assert!(error.to_string().contains("non-empty string frame id"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&text_root).unwrap_err();
    assert!(error.to_string().contains("one root element"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&empty_action).unwrap_err();
    assert!(error
        .to_string()
        .contains("frame actions need non-empty string ids"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&duplicate_action).unwrap_err();
    assert!(error.to_string().contains("frame actions need unique ids"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&empty_element_key).unwrap_err();
    assert!(error
        .to_string()
        .contains("compiled elements need non-empty keys"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&empty_element_tag).unwrap_err();
    assert!(error
        .to_string()
        .contains("compiled elements need non-empty tags"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&empty_text_key).unwrap_err();
    assert!(error
        .to_string()
        .contains("compiled text nodes need non-empty keys"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&duplicate_child_key).unwrap_err();
    assert!(error.to_string().contains("sibling nodes need unique keys"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&negative_width).unwrap_err();
    assert!(error.to_string().contains("positive finite number"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&inverted_width_bounds).unwrap_err();
    assert!(error
        .to_string()
        .contains("window.minWidth cannot be greater than window.maxWidth"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&width_below_minimum).unwrap_err();
    assert!(error
        .to_string()
        .contains("window.width cannot be smaller than window.minWidth"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());

    let error = session.render_frame(&non_finite_window).unwrap_err();
    assert!(error.to_string().contains("positive finite number"));
    assert_eq!(session.active_frame_id(), Some("valid"));
    assert_eq!(session.root(), Some(rendered.root));
    assert!(session.pending_commands().is_empty());
}

#[test]
fn ui_frame_rejects_null_optional_protocol_fields() {
    let actions_null = serde_json::from_str::<UiFrame>(
        r#"
            {
              "frameId": "actions-null",
              "actions": null,
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap_err();

    assert!(actions_null
        .to_string()
        .contains("a3s-gui frame actions cannot be null"));

    let window_null = serde_json::from_str::<UiFrame>(
        r#"
            {
              "frameId": "window-null",
              "window": null,
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button"
              }
            }
            "#,
    )
    .unwrap_err();

    assert!(window_null
        .to_string()
        .contains("a3s-gui frame window cannot be null"));
}
