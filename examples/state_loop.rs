use a3s_gui::{
    ActionInvocation, Gtk4Adapter, HostEvent, HostNodeId, NativeEvent, NativeEventKind,
    NativeProtocolSession, UiFrame,
};
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
struct ProfileState {
    name: String,
    notifications: bool,
    volume: f64,
    saved: bool,
}

impl Default for ProfileState {
    fn default() -> Self {
        Self {
            name: "Ada".to_string(),
            notifications: false,
            volume: 25.0,
            saved: false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ProfileState::default();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let rendered = session.render_frame(&profile_frame(&state)?)?;
    let mut controls = control_nodes(&session, rendered.root)?;

    let invocation = dispatch(
        &mut session,
        controls.name,
        NativeEventKind::Change,
        Some("Grace"),
    )?;
    apply_action(&mut state, &invocation)?;
    let rendered = session.render_frame(&profile_frame(&state)?)?;
    controls = control_nodes(&session, rendered.root)?;

    let invocation = dispatch(
        &mut session,
        controls.notifications,
        NativeEventKind::Toggle,
        Some("true"),
    )?;
    apply_action(&mut state, &invocation)?;
    let rendered = session.render_frame(&profile_frame(&state)?)?;
    controls = control_nodes(&session, rendered.root)?;

    let invocation = dispatch(
        &mut session,
        controls.volume,
        NativeEventKind::Change,
        Some("80"),
    )?;
    apply_action(&mut state, &invocation)?;
    let rendered = session.render_frame(&profile_frame(&state)?)?;
    controls = control_nodes(&session, rendered.root)?;

    let invocation = dispatch(&mut session, controls.save, NativeEventKind::Press, None)?;
    apply_action(&mut state, &invocation)?;
    session.render_frame(&profile_frame(&state)?)?;

    assert_eq!(
        state,
        ProfileState {
            name: "Grace".to_string(),
            notifications: true,
            volume: 80.0,
            saved: true,
        }
    );
    println!(
        "profile saved for {} with notifications={} volume={}",
        state.name, state.notifications, state.volume
    );
    Ok(())
}

fn profile_frame(state: &ProfileState) -> serde_json::Result<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "profile",
        "actions": [
            {"id": "setName", "label": "Set name"},
            {"id": "setNotifications", "label": "Set notifications"},
            {"id": "setVolume", "label": "Set volume"},
            {"id": "saveProfile", "label": "Save profile"}
        ],
        "root": {
            "kind": "element",
            "key": "profile-form",
            "tag": "form",
            "children": [
                {
                    "kind": "element",
                    "key": "name",
                    "tag": "input",
                    "props": {
                        "inputType": "text",
                        "value": state.name,
                        "events": {"onInput": "setName"}
                    }
                },
                {
                    "kind": "element",
                    "key": "notifications",
                    "tag": "input",
                    "props": {
                        "inputType": "checkbox",
                        "isChecked": state.notifications,
                        "events": {"onChange": "setNotifications"}
                    }
                },
                {
                    "kind": "element",
                    "key": "volume",
                    "tag": "input",
                    "props": {
                        "inputType": "range",
                        "minValue": 0,
                        "maxValue": 100,
                        "valueNumber": state.volume,
                        "events": {"onChange": "setVolume"}
                    }
                },
                {
                    "kind": "element",
                    "key": "save",
                    "tag": "button",
                    "props": {"events": {"onPress": "saveProfile"}},
                    "children": [{"kind": "text", "key": "save-label", "value": "Save"}]
                }
            ]
        }
    }))
}

fn dispatch(
    session: &mut NativeProtocolSession<Gtk4Adapter>,
    node: HostNodeId,
    kind: NativeEventKind,
    value: Option<&str>,
) -> a3s_gui::GuiResult<ActionInvocation> {
    let mut event = NativeEvent::new(node, kind);
    if let Some(value) = value {
        event = event.value(value);
    }
    session
        .dispatch_host_event(&HostEvent {
            frame_id: "profile".to_string(),
            event,
        })
        .map(|response| response.invocation)
}

fn apply_action(
    state: &mut ProfileState,
    invocation: &ActionInvocation,
) -> Result<(), Box<dyn std::error::Error>> {
    match invocation.action.as_str() {
        "setName" => {
            state.name = invocation.value.clone().unwrap_or_default();
        }
        "setNotifications" => {
            state.notifications = invocation.value.as_deref() == Some("true");
        }
        "setVolume" => {
            state.volume = invocation.value.as_deref().unwrap_or("0").parse()?;
        }
        "saveProfile" => {
            state.saved = true;
        }
        other => {
            return Err(format!("unexpected action {other}").into());
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct ControlNodes {
    name: HostNodeId,
    notifications: HostNodeId,
    volume: HostNodeId,
    save: HostNodeId,
}

fn control_nodes(
    session: &NativeProtocolSession<Gtk4Adapter>,
    root: HostNodeId,
) -> a3s_gui::GuiResult<ControlNodes> {
    let children = &session
        .runtime()
        .host()
        .node(root)
        .ok_or_else(|| a3s_gui::GuiError::host("rendered form root missing"))?
        .children;
    if children.len() != 4 {
        return Err(a3s_gui::GuiError::host(format!(
            "expected 4 profile controls, found {}",
            children.len()
        )));
    }
    Ok(ControlNodes {
        name: children[0],
        notifications: children[1],
        volume: children[2],
        save: children[3],
    })
}
