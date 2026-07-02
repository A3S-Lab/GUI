use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::compiler::{CompiledJsxNode, ReactCompilerBridge};
use crate::error::GuiResult;
use crate::event::{ActionInvocation, NativeEvent, RegisteredAction};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionChange;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{BlueprintHost, PlatformAdapter, PlatformCommand, PlatformPlanningHost};
use crate::runtime::GuiRuntime;
use crate::web::WebProps;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiFrame {
    pub frame_id: String,
    pub root: CompiledJsxNode,
    #[serde(default)]
    pub actions: Vec<UiAction>,
    #[serde(default)]
    pub window: Option<WindowOptions>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UiFrameWire {
    frame_id: String,
    root: CompiledJsxNode,
    #[serde(default)]
    actions: Option<Vec<UiAction>>,
    #[serde(default)]
    window: Option<WindowOptions>,
}

impl<'de> Deserialize<'de> for UiFrame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = UiFrameWire::deserialize(deserializer)?;
        let actions = wire
            .actions
            .unwrap_or_else(|| collect_actions_from_node(&wire.root));
        Ok(Self {
            frame_id: wire.frame_id,
            root: wire.root,
            actions,
            window: wire.window,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiAction {
    pub id: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOptions {
    pub title: String,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default = "default_true")]
    pub resizable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderedFrame {
    pub frame_id: String,
    pub root: HostNodeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeRenderResponse {
    pub frame_id: String,
    pub root: HostNodeId,
    pub commands: Vec<PlatformCommand>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEvent {
    pub frame_id: String,
    pub event: NativeEvent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEventResponse {
    pub frame_id: String,
    pub invocation: ActionInvocation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeHostEventResponse {
    pub frame_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ActionInvocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
}

impl UiFrame {
    pub fn render_into<H: NativeHost>(
        &self,
        runtime: &mut GuiRuntime<H>,
    ) -> GuiResult<RenderedFrame> {
        runtime
            .actions_mut()
            .replace_registered(self.actions.iter().map(UiAction::registered_action));
        let root = match &self.window {
            Some(window) => {
                let content = ReactCompilerBridge::new().lower_to_native(&self.root)?;
                let window = window.wrap_native_root(&self.frame_id, content);
                runtime.render_native(&window)?
            }
            None => runtime.render_compiled(&self.root)?,
        };
        Ok(RenderedFrame {
            frame_id: self.frame_id.clone(),
            root,
        })
    }
}

impl UiAction {
    fn registered_action(&self) -> RegisteredAction {
        RegisteredAction {
            id: self.id.clone(),
            label: self.label.clone(),
        }
    }
}

impl WindowOptions {
    pub fn wrap_native_root(&self, frame_id: &str, content: NativeElement) -> NativeElement {
        let resizable = self.resizable.to_string();
        let mut web = WebProps::new()
            .attribute("data-a3s-frame", frame_id)
            .attribute("data-a3s-window-resizable", resizable.clone());
        if let Some(width) = self.width {
            web = web.style("width", width.to_string());
        }
        if let Some(height) = self.height {
            web = web.style("height", height.to_string());
        }

        NativeElement::new(format!("{frame_id}:window"), NativeRole::Window)
            .with_props(
                NativeProps::new()
                    .label(self.title.clone())
                    .metadata("data-a3s-window-resizable", resizable)
                    .web(web),
            )
            .child(content)
    }
}

fn default_true() -> bool {
    true
}

fn collect_actions_from_node(root: &CompiledJsxNode) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mut indexes = BTreeMap::new();
    collect_actions_into(root, &mut actions, &mut indexes);
    actions
}

fn collect_actions_into(
    node: &CompiledJsxNode,
    actions: &mut Vec<UiAction>,
    indexes: &mut BTreeMap<String, usize>,
) {
    let CompiledJsxNode::Element {
        props, children, ..
    } = node
    else {
        return;
    };

    for id in props.events.values().filter(|id| !id.is_empty()) {
        let label = props
            .action_labels
            .get(id)
            .filter(|label| !label.is_empty())
            .cloned();
        match indexes.get(id).copied() {
            Some(index) if actions[index].label.is_none() && label.is_some() => {
                actions[index].label = label;
            }
            Some(_) => {}
            None => {
                indexes.insert(id.clone(), actions.len());
                actions.push(UiAction {
                    id: id.clone(),
                    label,
                });
            }
        }
    }

    for child in children {
        collect_actions_into(child, actions, indexes);
    }
}

#[derive(Debug)]
pub struct NativeProtocolSession<A: PlatformAdapter> {
    runtime: GuiRuntime<PlatformPlanningHost<A>>,
    active_frame_id: Option<String>,
    root: Option<HostNodeId>,
    command_cursor: usize,
}

impl<A: PlatformAdapter> NativeProtocolSession<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            runtime: GuiRuntime::new(PlatformPlanningHost::new(adapter)),
            active_frame_id: None,
            root: None,
            command_cursor: 0,
        }
    }

    pub fn runtime(&self) -> &GuiRuntime<PlatformPlanningHost<A>> {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut GuiRuntime<PlatformPlanningHost<A>> {
        &mut self.runtime
    }

    pub fn active_frame_id(&self) -> Option<&str> {
        self.active_frame_id.as_deref()
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        self.runtime.accessibility_tree()
    }

    pub fn render_frame(&mut self, frame: &UiFrame) -> GuiResult<NativeRenderResponse> {
        let rendered = frame.render_into(&mut self.runtime)?;
        self.active_frame_id = Some(rendered.frame_id.clone());
        self.root = Some(rendered.root);
        let commands = self.pending_commands();
        let accessibility_tree = self.runtime.accessibility_tree();
        Ok(NativeRenderResponse {
            frame_id: rendered.frame_id,
            root: rendered.root,
            commands,
            accessibility_tree,
        })
    }

    pub fn dispatch_host_event(&mut self, event: &HostEvent) -> GuiResult<HostEventResponse> {
        self.ensure_active_frame(event)?;
        event.dispatch_into(&mut self.runtime)
    }

    pub fn handle_host_event(&mut self, event: &HostEvent) -> GuiResult<NativeHostEventResponse> {
        self.ensure_active_frame(event)?;
        event.handle_into(&mut self.runtime)
    }

    pub fn pending_commands(&mut self) -> Vec<PlatformCommand> {
        let commands = self.runtime.host().commands()[self.command_cursor..].to_vec();
        self.command_cursor = self.runtime.host().commands().len();
        commands
    }

    fn ensure_active_frame(&self, event: &HostEvent) -> GuiResult<()> {
        let active_frame_id = self
            .active_frame_id
            .as_deref()
            .ok_or_else(|| crate::error::GuiError::host("no active native frame"))?;
        if event.frame_id != active_frame_id {
            return Err(crate::error::GuiError::host(format!(
                "native event for frame {} cannot be dispatched into active frame {}",
                event.frame_id, active_frame_id
            )));
        }
        Ok(())
    }
}

impl<A: PlatformAdapter + Default> Default for NativeProtocolSession<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}

impl HostEvent {
    pub fn dispatch_into<H: NativeHost + BlueprintHost>(
        &self,
        runtime: &mut GuiRuntime<H>,
    ) -> GuiResult<HostEventResponse> {
        let handled = runtime.handle_native_event_with_changes(self.event.clone())?;
        let invocation = handled.invocation.ok_or_else(|| {
            crate::error::GuiError::host("native event has no registered Web action")
        })?;
        Ok(HostEventResponse {
            frame_id: self.frame_id.clone(),
            invocation,
            interaction_changes: handled.interaction_changes,
        })
    }

    pub fn handle_into<H: NativeHost + BlueprintHost + AccessibilityTreeHost>(
        &self,
        runtime: &mut GuiRuntime<H>,
    ) -> GuiResult<NativeHostEventResponse> {
        let handled = runtime.handle_native_event_with_changes(self.event.clone())?;
        let accessibility_tree = runtime.accessibility_tree();
        Ok(NativeHostEventResponse {
            frame_id: self.frame_id.clone(),
            invocation: handled.invocation,
            accessibility_tree,
            interaction_changes: handled.interaction_changes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibility::AccessibilityRole;
    use crate::backend::{CommandExecutingHost, RecordingBackend};
    use crate::event::NativeEventKind;
    use crate::platform::{Gtk4Adapter, NativeWidgetSetter};

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
        assert!(first_response.commands.iter().any(|command| {
            matches!(command, crate::platform::PlatformCommand::SetRoot { .. })
        }));
        assert_eq!(second_response.root, first_response.root);
        assert!(second_response.commands.iter().any(|command| matches!(
            command,
            crate::platform::PlatformCommand::Update {
                id,
                blueprint,
            } if *id == first_response.root && blueprint.label.as_deref() == Some("Saved")
        )));
        assert!(second_response.commands.iter().all(|command| {
            !matches!(command, crate::platform::PlatformCommand::Create { .. })
        }));
        assert!(session.pending_commands().is_empty());
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

        assert_eq!(response.invocation.action, "saveProfile");
        assert!(error.to_string().contains("active frame profile"));
    }

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
                    label: Some("Save profile".to_string()),
                },
                UiAction {
                    id: "handleSearchKey".to_string(),
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
                event: NativeEvent::new(rendered.root, NativeEventKind::KeyDown).value("Enter"),
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
    fn protocol_window_options_wrap_root_in_native_window() {
        let frame: UiFrame = serde_json::from_str(
            r#"
            {
              "frameId": "frame-window",
              "window": {"title": "A3S Profile", "width": 640, "height": 480},
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

        let rendered = frame.render_into(&mut runtime).unwrap();
        let window = runtime.host().planning().node(rendered.root).unwrap();

        assert_eq!(window.blueprint.widget_class, "gtk::ApplicationWindow");
        assert_eq!(window.blueprint.label.as_deref(), Some("A3S Profile"));
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
        assert_eq!(window.children.len(), 1);

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
}
