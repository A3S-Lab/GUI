use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize};

use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::compiler::{CompiledJsxNode, ReactCompilerBridge};
use crate::error::{GuiError, GuiResult};
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
    #[serde(default, deserialize_with = "deserialize_frame_actions")]
    actions: Option<Vec<UiAction>>,
    #[serde(default, deserialize_with = "deserialize_frame_window")]
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
            .unwrap_or_else(|| collect_actions_from_frame(&wire.root, wire.window.as_ref()));
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
    pub on_close: Option<String>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub min_width: Option<f64>,
    #[serde(default)]
    pub min_height: Option<f64>,
    #[serde(default)]
    pub max_width: Option<f64>,
    #[serde(default)]
    pub max_height: Option<f64>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeAppEventResponse {
    pub frame_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ActionInvocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<NativeRenderResponse>,
}

impl UiFrame {
    pub fn validate(&self) -> GuiResult<()> {
        if self.frame_id.is_empty() {
            return Err(GuiError::invalid_tree(
                "a3s-gui frames need a non-empty string frame id",
            ));
        }
        if !matches!(&self.root, CompiledJsxNode::Element { .. }) {
            return Err(GuiError::invalid_tree(
                "a3s-gui frames need one root element",
            ));
        }
        self.root.validate()?;
        if self.actions.iter().any(|action| action.id.is_empty()) {
            return Err(GuiError::invalid_tree(
                "a3s-gui frame actions need non-empty string ids",
            ));
        }
        let mut seen_action_ids = BTreeSet::new();
        if let Some(duplicate) = self
            .actions
            .iter()
            .map(|action| action.id.as_str())
            .find(|id| !seen_action_ids.insert(*id))
        {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui frame actions need unique ids; duplicate action {duplicate:?}"
            )));
        }
        if let Some(window) = &self.window {
            window.validate()?;
        }
        Ok(())
    }

    pub fn render_into<H: NativeHost>(
        &self,
        runtime: &mut GuiRuntime<H>,
    ) -> GuiResult<RenderedFrame> {
        self.validate()?;
        let root = match &self.window {
            Some(window) => {
                let content = ReactCompilerBridge::new().lower_to_native(&self.root)?;
                let window = window.wrap_native_root(&self.frame_id, content);
                runtime.render_native(&window)?
            }
            None => runtime.render_compiled(&self.root)?,
        };
        runtime
            .actions_mut()
            .replace_registered(self.actions.iter().map(UiAction::registered_action));
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
    pub fn validate(&self) -> GuiResult<()> {
        validate_window_dimension("window.width", self.width)?;
        validate_window_dimension("window.height", self.height)?;
        validate_window_dimension("window.minWidth", self.min_width)?;
        validate_window_dimension("window.minHeight", self.min_height)?;
        validate_window_dimension("window.maxWidth", self.max_width)?;
        validate_window_dimension("window.maxHeight", self.max_height)?;
        validate_dimension_bounds(
            "window.width",
            self.width,
            "window.minWidth",
            self.min_width,
            "window.maxWidth",
            self.max_width,
        )?;
        validate_dimension_bounds(
            "window.height",
            self.height,
            "window.minHeight",
            self.min_height,
            "window.maxHeight",
            self.max_height,
        )?;
        Ok(())
    }

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
        if let Some(min_width) = self.min_width {
            web = web.style("minWidth", min_width.to_string());
        }
        if let Some(min_height) = self.min_height {
            web = web.style("minHeight", min_height.to_string());
        }
        if let Some(max_width) = self.max_width {
            web = web.style("maxWidth", max_width.to_string());
        }
        if let Some(max_height) = self.max_height {
            web = web.style("maxHeight", max_height.to_string());
        }

        NativeElement::new(format!("{frame_id}:window"), NativeRole::Window)
            .with_props(
                NativeProps::new()
                    .label(self.title.clone())
                    .metadata("data-a3s-window-resizable", resizable)
                    .web(window_web_props(web, self.on_close.as_deref())),
            )
            .child(content)
    }
}

fn window_web_props(web: WebProps, on_close: Option<&str>) -> WebProps {
    match on_close.filter(|action| !action.is_empty()) {
        Some(action) => web.event("onClose", action),
        None => web,
    }
}

fn default_true() -> bool {
    true
}

fn deserialize_frame_actions<'de, D>(deserializer: D) -> Result<Option<Vec<UiAction>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Vec<UiAction>>::deserialize(deserializer)? {
        Some(actions) => Ok(Some(actions)),
        None => Err(serde::de::Error::custom(
            "a3s-gui frame actions cannot be null; omit the field instead",
        )),
    }
}

fn deserialize_frame_window<'de, D>(deserializer: D) -> Result<Option<WindowOptions>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<WindowOptions>::deserialize(deserializer)? {
        Some(window) => Ok(Some(window)),
        None => Err(serde::de::Error::custom(
            "a3s-gui frame window cannot be null; omit the field instead",
        )),
    }
}

fn validate_window_dimension(name: &'static str, value: Option<f64>) -> GuiResult<()> {
    let Some(value) = value else {
        return Ok(());
    };
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "a3s-gui {name} must be a positive finite number"
        )))
    }
}

fn validate_dimension_bounds(
    value_name: &'static str,
    value: Option<f64>,
    min_name: &'static str,
    min: Option<f64>,
    max_name: &'static str,
    max: Option<f64>,
) -> GuiResult<()> {
    if let (Some(min), Some(max)) = (min, max) {
        if min > max {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui {min_name} cannot be greater than {max_name}"
            )));
        }
    }
    if let (Some(value), Some(min)) = (value, min) {
        if value < min {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui {value_name} cannot be smaller than {min_name}"
            )));
        }
    }
    if let (Some(value), Some(max)) = (value, max) {
        if value > max {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui {value_name} cannot be greater than {max_name}"
            )));
        }
    }
    Ok(())
}

fn collect_actions_from_frame(
    root: &CompiledJsxNode,
    window: Option<&WindowOptions>,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mut indexes = BTreeMap::new();
    collect_actions_into(root, &mut actions, &mut indexes);
    if let Some(on_close) = window.and_then(|window| window.on_close.as_ref()) {
        collect_action_id(on_close, None, &mut actions, &mut indexes);
    }
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
        collect_action_id(id, label, actions, indexes);
    }

    for child in children {
        collect_actions_into(child, actions, indexes);
    }
}

fn collect_action_id(
    id: &str,
    label: Option<String>,
    actions: &mut Vec<UiAction>,
    indexes: &mut BTreeMap<String, usize>,
) {
    if id.is_empty() {
        return;
    }
    match indexes.get(id).copied() {
        Some(index) if actions[index].label.is_none() && label.is_some() => {
            actions[index].label = label;
        }
        Some(_) => {}
        None => {
            indexes.insert(id.to_string(), actions.len());
            actions.push(UiAction {
                id: id.to_string(),
                label,
            });
        }
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
        event.validate()?;
        self.ensure_active_frame(event)?;
        event.dispatch_into(&mut self.runtime)
    }

    pub fn handle_host_event(&mut self, event: &HostEvent) -> GuiResult<NativeHostEventResponse> {
        event.validate()?;
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

#[derive(Debug)]
pub struct NativeProtocolApp<A: PlatformAdapter, S, F, R> {
    session: NativeProtocolSession<A>,
    state: S,
    frame_builder: F,
    action_reducer: R,
}

impl<A, S, F, R> NativeProtocolApp<A, S, F, R>
where
    A: PlatformAdapter,
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    pub fn new(adapter: A, state: S, frame_builder: F, action_reducer: R) -> Self {
        Self {
            session: NativeProtocolSession::new(adapter),
            state,
            frame_builder,
            action_reducer,
        }
    }

    pub fn session(&self) -> &NativeProtocolSession<A> {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut NativeProtocolSession<A> {
        &mut self.session
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn render(&mut self) -> GuiResult<NativeRenderResponse> {
        let frame = (self.frame_builder)(&self.state)?;
        self.session.render_frame(&frame)
    }

    pub fn dispatch_host_event(&mut self, event: &HostEvent) -> GuiResult<NativeAppEventResponse> {
        let response = self.session.dispatch_host_event(event)?;
        (self.action_reducer)(&mut self.state, &response.invocation)?;
        let render = self.render()?;
        let accessibility_tree = render.accessibility_tree.clone();
        Ok(NativeAppEventResponse {
            frame_id: response.frame_id,
            invocation: Some(response.invocation),
            accessibility_tree,
            interaction_changes: response.interaction_changes,
            render: Some(render),
        })
    }

    pub fn handle_host_event(&mut self, event: &HostEvent) -> GuiResult<NativeAppEventResponse> {
        let response = self.session.handle_host_event(event)?;
        let mut render = None;
        let mut accessibility_tree = response.accessibility_tree;

        if let Some(invocation) = response.invocation.as_ref() {
            (self.action_reducer)(&mut self.state, invocation)?;
            let rendered = self.render()?;
            accessibility_tree = rendered.accessibility_tree.clone();
            render = Some(rendered);
        }

        Ok(NativeAppEventResponse {
            frame_id: response.frame_id,
            invocation: response.invocation,
            accessibility_tree,
            interaction_changes: response.interaction_changes,
            render,
        })
    }
}

impl HostEvent {
    pub fn validate(&self) -> GuiResult<()> {
        if self.frame_id.is_empty() {
            return Err(GuiError::host(
                "a3s-gui host events need a non-empty frame id",
            ));
        }
        self.event.validate()
    }

    pub fn dispatch_into<H: NativeHost + BlueprintHost>(
        &self,
        runtime: &mut GuiRuntime<H>,
    ) -> GuiResult<HostEventResponse> {
        self.validate()?;
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
        self.validate()?;
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
    use crate::host::HeadlessHost;
    use crate::platform::{Gtk4Adapter, NativeWidgetSetter};

    #[derive(Default)]
    struct FailingUpdateHost {
        inner: HeadlessHost,
        fail_updates: bool,
    }

    impl NativeHost for FailingUpdateHost {
        fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
            self.inner.create(element)
        }

        fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
            if self.fail_updates {
                return Err(GuiError::host("forced host update failure"));
            }
            self.inner.update(id, props)
        }

        fn insert_child(
            &mut self,
            parent: HostNodeId,
            child: HostNodeId,
            index: usize,
        ) -> GuiResult<()> {
            self.inner.insert_child(parent, child, index)
        }

        fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
            self.inner.remove(id)
        }

        fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
            self.inner.set_root(id)
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct CounterState {
        count: u32,
    }

    fn counter_frame(state: &CounterState) -> GuiResult<UiFrame> {
        serde_json::from_value(serde_json::json!({
            "frameId": "counter",
            "actions": [{"id": "increment"}],
            "root": {
                "kind": "element",
                "key": "increment",
                "tag": "Button",
                "props": {"events": {"onPress": "increment"}},
                "children": [
                    {
                        "kind": "text",
                        "key": "label",
                        "value": format!("Count {}", state.count)
                    }
                ]
            }
        }))
        .map_err(|error| GuiError::invalid_tree(format!("invalid counter frame: {error}")))
    }

    fn counter_reduce(state: &mut CounterState, invocation: &ActionInvocation) -> GuiResult<()> {
        match invocation.action.as_str() {
            "increment" => {
                state.count += 1;
                Ok(())
            }
            other => Err(GuiError::host(format!("unexpected action {other}"))),
        }
    }

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
        assert!(second_response.commands.iter().all(|command| {
            !matches!(command, crate::platform::PlatformCommand::Create { .. })
        }));
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
    fn native_protocol_session_suppresses_read_only_ancestor_selection_events() {
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

        let response = session
            .handle_host_event(&HostEvent {
                frame_id: "profile".to_string(),
                event: NativeEvent::new(dark, NativeEventKind::SelectionChange),
            })
            .unwrap();

        assert!(response.invocation.is_none());
        assert!(response.interaction_changes.is_empty());
        let accessibility = response.accessibility_tree.as_ref().unwrap();
        assert_eq!(accessibility.value.as_deref(), Some("light"));
        assert!(accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(true));
        assert!(!accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(false));
        assert!(session.runtime().actions().invocations().is_empty());
    }

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
                event: NativeEvent::new(rendered.root, NativeEventKind::SelectionChange),
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
                event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("99"),
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
                event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("0"),
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
                event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("99"),
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
                event: NativeEvent::new(rendered.root, NativeEventKind::Change).value("0"),
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
}
