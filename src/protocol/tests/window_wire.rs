use super::*;

#[test]
fn protocol_window_options_wrap_root_in_native_window() {
    let frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "frame-window",
              "window": {
                "title": "A3S Profile",
                "onClose": "closeWindow",
                "width": 640,
                "height": 480,
                "minWidth": 480,
                "minHeight": 320,
                "maxWidth": 1280,
                "maxHeight": 960
              },
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "children": [{"kind": "text", "key": "text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);

    assert_eq!(
        frame
            .window
            .as_ref()
            .and_then(|window| window.on_close.as_deref()),
        Some("closeWindow")
    );
    assert!(frame
        .actions
        .iter()
        .any(|action| action.id == "closeWindow"));
    let rendered = frame.render_into(&mut runtime).unwrap();
    let window = runtime.host().planning().node(rendered.root).unwrap();

    assert_eq!(window.blueprint.widget_class, "gtk::ApplicationWindow");
    assert_eq!(window.blueprint.label.as_deref(), Some("A3S Profile"));
    assert_eq!(
        window.blueprint.events.get("onClose").map(String::as_str),
        Some("closeWindow")
    );
    assert_eq!(
        window
            .blueprint
            .metadata
            .get("data-a3s-window-resizable")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(window.blueprint.config().window_resizable, Some(true));
    assert_eq!(
        window
            .blueprint
            .portable_style
            .width
            .as_ref()
            .and_then(|value| value.points()),
        Some(640.0)
    );
    assert_eq!(
        window
            .blueprint
            .portable_style
            .height
            .as_ref()
            .and_then(|value| value.points()),
        Some(480.0)
    );
    assert_eq!(
        window
            .blueprint
            .portable_style
            .min_width
            .as_ref()
            .and_then(|value| value.points()),
        Some(480.0)
    );
    assert_eq!(
        window
            .blueprint
            .portable_style
            .min_height
            .as_ref()
            .and_then(|value| value.points()),
        Some(320.0)
    );
    assert_eq!(
        window
            .blueprint
            .portable_style
            .max_width
            .as_ref()
            .and_then(|value| value.points()),
        Some(1280.0)
    );
    assert_eq!(
        window
            .blueprint
            .portable_style
            .max_height
            .as_ref()
            .and_then(|value| value.points()),
        Some(960.0)
    );
    assert_eq!(window.children.len(), 1);
    let close = runtime
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Close))
        .unwrap();
    assert_eq!(close.action, "closeWindow");
    assert_eq!(close.event, NativeEventKind::Close);

    let fixed_frame: UiFrame = serde_json::from_str(
        r#"
            {
              "frameId": "fixed-window",
              "window": {"title": "Fixed", "resizable": false},
              "root": {
                "kind": "element",
                "key": "save",
                "tag": "Button",
                "children": [{"kind": "text", "key": "text", "value": "Save"}]
              }
            }
            "#,
    )
    .unwrap();
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);

    let rendered = fixed_frame.render_into(&mut runtime).unwrap();
    let window = runtime.host().planning().node(rendered.root).unwrap();
    let config = window.blueprint.config();

    assert_eq!(config.window_resizable, Some(false));
    assert_eq!(
        config
            .metadata
            .get("data-a3s-window-resizable")
            .map(String::as_str),
        Some("false")
    );
    assert!(config
        .create_setters()
        .contains(&NativeWidgetSetter::SetWindowResizable(Some(false))));
}

#[test]
fn protocol_types_round_trip_as_json() {
    let event = HostEvent {
        frame_id: "frame-2".to_string(),
        event: NativeEvent::new(HostNodeId::new(42), NativeEventKind::KeyDown).value("Enter"),
    };

    let json = serde_json::to_string(&event).unwrap();
    let decoded: HostEvent = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, event);
    assert!(json.contains(r#""kind":"keyDown""#));

    let legacy_response: NativeRenderResponse =
        serde_json::from_str(r#"{"frameId":"legacy","root":1,"commands":[]}"#).unwrap();
    assert!(legacy_response.accessibility_tree.is_none());

    let legacy_event_response: NativeHostEventResponse =
        serde_json::from_str(r#"{"frameId":"legacy"}"#).unwrap();
    assert!(legacy_event_response.invocation.is_none());
    assert!(legacy_event_response.accessibility_tree.is_none());
    assert!(legacy_event_response.interaction_changes.is_empty());
}
