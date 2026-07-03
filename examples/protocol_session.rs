use a3s_gui::{
    Gtk4Adapter, HostEvent, NativeEvent, NativeEventKind, NativeProtocolSession, UiFrame,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame: UiFrame = serde_json::from_str(
        r#"
        {
          "frameId": "toolbar",
          "actions": [{"id": "saveDocument", "label": "Save document"}],
          "root": {
            "kind": "element",
            "key": "save",
            "tag": "Button",
            "props": {"events": {"onPress": "saveDocument"}},
            "children": [{"kind": "text", "key": "label", "value": "Save"}]
          }
        }
        "#,
    )?;

    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let rendered = session.render_frame(&frame)?;

    println!(
        "rendered frame {} as node {} with {} native command(s)",
        rendered.frame_id,
        rendered.root.get(),
        rendered.commands.len()
    );

    let handled = session.handle_host_event(&HostEvent {
        frame_id: frame.frame_id,
        event: NativeEvent::new(rendered.root, NativeEventKind::Press),
    })?;

    if let Some(invocation) = handled.invocation {
        println!("dispatched action {}", invocation.action);
    }

    Ok(())
}
