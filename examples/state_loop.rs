use a3s_gui::{
    ActionInvocation, Gtk4Adapter, HostEvent, HostNodeId, NativeEvent, NativeEventKind,
    NativeProtocolApp, NativeProtocolSession, UiFrame,
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
    let mut app = NativeProtocolApp::new(
        Gtk4Adapter,
        ProfileState::default(),
        profile_frame,
        apply_action,
    );

    let rendered = app.render()?;
    let mut controls = control_nodes(app.session(), rendered.root)?;

    let response = app.dispatch_host_event(&host_event(
        controls.name,
        NativeEventKind::Change,
        Some("Grace"),
    ))?;
    controls = control_nodes(app.session(), response_render_root(&response)?)?;

    let response = app.dispatch_host_event(&host_event(
        controls.notifications,
        NativeEventKind::Toggle,
        Some("true"),
    ))?;
    controls = control_nodes(app.session(), response_render_root(&response)?)?;

    let response = app.dispatch_host_event(&host_event(
        controls.volume,
        NativeEventKind::Change,
        Some("80"),
    ))?;
    controls = control_nodes(app.session(), response_render_root(&response)?)?;

    app.dispatch_host_event(&host_event(controls.save, NativeEventKind::Press, None))?;

    assert_eq!(
        app.state(),
        &ProfileState {
            name: "Grace".to_string(),
            notifications: true,
            volume: 80.0,
            saved: true,
        }
    );
    println!(
        "profile saved for {} with notifications={} volume={}",
        app.state().name,
        app.state().notifications,
        app.state().volume
    );
    Ok(())
}

fn profile_frame(state: &ProfileState) -> a3s_gui::GuiResult<UiFrame> {
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
    .map_err(|error| a3s_gui::GuiError::invalid_tree(format!("invalid profile frame: {error}")))
}

fn host_event(node: HostNodeId, kind: NativeEventKind, value: Option<&str>) -> HostEvent {
    let mut event = NativeEvent::new(node, kind);
    if let Some(value) = value {
        event = event.value(value);
    }
    HostEvent {
        frame_id: "profile".to_string(),
        event,
    }
}

fn response_render_root(
    response: &a3s_gui::NativeAppEventResponse,
) -> a3s_gui::GuiResult<HostNodeId> {
    response
        .render
        .as_ref()
        .map(|render| render.root)
        .ok_or_else(|| a3s_gui::GuiError::host("state action did not render a follow-up frame"))
}

fn apply_action(state: &mut ProfileState, invocation: &ActionInvocation) -> a3s_gui::GuiResult<()> {
    match invocation.action.as_str() {
        "setName" => {
            state.name = invocation.value.clone().unwrap_or_default();
        }
        "setNotifications" => {
            state.notifications = invocation.value.as_deref() == Some("true");
        }
        "setVolume" => {
            state.volume = invocation
                .value
                .as_deref()
                .unwrap_or("0")
                .parse()
                .map_err(|error| a3s_gui::GuiError::host(format!("invalid volume: {error}")))?;
        }
        "saveProfile" => {
            state.saved = true;
        }
        other => {
            return Err(a3s_gui::GuiError::host(format!(
                "unexpected action {other}"
            )));
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
