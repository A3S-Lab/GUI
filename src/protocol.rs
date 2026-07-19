use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "authoring")]
use serde_json::{json, Value as JsonValue};

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityNode, AccessibilityRelationshipProps,
    AccessibilityRole, AccessibilityStateProps, AccessibilityStructureProps, AccessibilityTreeHost,
};
use crate::compiler::{
    CompiledOrientation, CompiledProps, CompiledRsxNode, CompiledStyleValue, RsxCompilerBridge,
};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, NativeEvent, RegisteredAction};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionChange;
use crate::native::{NativeElement, NativeProps, NativeRole, ValueSensitivity};
use crate::platform::{
    BlueprintHost, NativeControlState, NativeWidgetBlueprint, PlatformAdapter, PlatformCommand,
    PlatformPlanningHost,
};
use crate::runtime::{effective_blueprint_value_sensitivity, GuiRuntime};
use crate::style::PortableStyle;
use crate::web::WebProps;

pub const NATIVE_PROTOCOL_VERSION_V1: u32 = 1;

static NEXT_PROTOCOL_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiFrame {
    pub frame_id: String,
    pub root: CompiledRsxNode,
    #[serde(default)]
    pub actions: Vec<UiAction>,
    #[serde(default)]
    pub window: Option<WindowOptions>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UiFrameWire {
    frame_id: String,
    root: CompiledRsxNode,
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
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeRenderResponse {
    pub frame_id: String,
    pub root: HostNodeId,
    pub commands: Vec<PlatformCommand>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeRenderResponseWire {
    frame_id: String,
    root: HostNodeId,
    commands: Vec<PlatformCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_tree: Option<AccessibilityNode>,
}

impl Serialize for NativeRenderResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut commands = self.commands.clone();
        let mut sensitive_nodes = BTreeSet::new();
        for command in &mut commands {
            let (id, blueprint) = match command {
                PlatformCommand::Create { id, blueprint }
                | PlatformCommand::Update { id, blueprint } => (*id, blueprint),
                _ => continue,
            };
            let sensitivity = effective_blueprint_value_sensitivity(blueprint);
            blueprint.value_sensitivity = sensitivity;
            if sensitivity.is_sensitive() {
                blueprint.control_state.accessibility_description.value_text = None;
                sensitivity.redact_metadata(&mut blueprint.metadata);
                sensitive_nodes.insert(id);
            }
        }
        let mut accessibility_tree = self.accessibility_tree.clone();
        redact_accessibility_nodes(accessibility_tree.as_mut(), &sensitive_nodes);
        NativeRenderResponseWire {
            frame_id: self.frame_id.clone(),
            root: self.root,
            commands,
            accessibility_tree,
        }
        .serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEvent {
    pub frame_id: String,
    pub event: NativeEvent,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEventResponse {
    pub frame_id: String,
    pub invocation: ActionInvocation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
    #[serde(default, skip)]
    pub value_sensitivity: ValueSensitivity,
    #[serde(default, skip)]
    pub value_node: Option<HostNodeId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeHostEventResponse {
    pub frame_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ActionInvocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility_tree: Option<AccessibilityNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<InteractionChange>,
    #[serde(default, skip)]
    pub value_sensitivity: ValueSensitivity,
    #[serde(default, skip)]
    pub value_node: Option<HostNodeId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
    #[serde(default, skip)]
    pub value_sensitivity: ValueSensitivity,
    #[serde(default, skip)]
    pub value_node: Option<HostNodeId>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HostEventResponseWire {
    frame_id: String,
    invocation: ActionInvocation,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    interaction_changes: Vec<InteractionChange>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeHostEventResponseWire {
    frame_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation: Option<ActionInvocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_tree: Option<AccessibilityNode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    interaction_changes: Vec<InteractionChange>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeAppEventResponseWire {
    frame_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation: Option<ActionInvocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_tree: Option<AccessibilityNode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    interaction_changes: Vec<InteractionChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render: Option<NativeRenderResponse>,
}

impl Serialize for HostEventResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_response_values(
            Some(&mut invocation),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        HostEventResponseWire {
            frame_id: self.frame_id.clone(),
            invocation,
            interaction_changes,
        }
        .serialize(serializer)
    }
}

impl Serialize for NativeHostEventResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_response_values(
            invocation.as_mut(),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        let mut accessibility_tree = self.accessibility_tree.clone();
        redact_response_accessibility(
            accessibility_tree.as_mut(),
            self.value_node,
            self.value_sensitivity,
        );
        NativeHostEventResponseWire {
            frame_id: self.frame_id.clone(),
            invocation,
            accessibility_tree,
            interaction_changes,
        }
        .serialize(serializer)
    }
}

impl Serialize for NativeAppEventResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_response_values(
            invocation.as_mut(),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        let mut accessibility_tree = self.accessibility_tree.clone();
        redact_response_accessibility(
            accessibility_tree.as_mut(),
            self.value_node,
            self.value_sensitivity,
        );
        NativeAppEventResponseWire {
            frame_id: self.frame_id.clone(),
            invocation,
            accessibility_tree,
            interaction_changes,
            render: self.render.clone(),
        }
        .serialize(serializer)
    }
}

fn redact_response_values(
    invocation: Option<&mut ActionInvocation>,
    interaction_changes: &mut [InteractionChange],
    value_sensitivity: ValueSensitivity,
) {
    if !value_sensitivity.is_sensitive() {
        return;
    }
    if let Some(invocation) = invocation {
        invocation.value = None;
    }
    for change in interaction_changes {
        change.before.value = None;
        change.after.value = None;
    }
}

fn redact_response_accessibility(
    node: Option<&mut AccessibilityNode>,
    target: Option<HostNodeId>,
    sensitivity: ValueSensitivity,
) {
    let (Some(node), Some(target)) = (node, target) else {
        return;
    };
    if node.node == Some(target) && sensitivity.is_sensitive() {
        node.value = None;
        node.description.value_text = None;
        node.value_sensitivity = ValueSensitivity::Sensitive;
    }
    for child in &mut node.children {
        redact_response_accessibility(Some(child), Some(target), sensitivity);
    }
}

fn redact_accessibility_nodes(
    node: Option<&mut AccessibilityNode>,
    sensitive_nodes: &BTreeSet<HostNodeId>,
) {
    let Some(node) = node else {
        return;
    };
    if node.node.is_some_and(|id| sensitive_nodes.contains(&id)) {
        node.value = None;
        node.description.value_text = None;
        node.value_sensitivity = ValueSensitivity::Sensitive;
    }
    for child in &mut node.children {
        redact_accessibility_nodes(Some(child), sensitive_nodes);
    }
}

mod v1;
pub use v1::*;

impl UiFrame {
    pub fn from_compiled(frame_id: impl Into<String>, root: CompiledRsxNode) -> GuiResult<Self> {
        Self::from_compiled_parts(frame_id, root, None, None)
    }

    pub fn from_compiled_parts(
        frame_id: impl Into<String>,
        root: CompiledRsxNode,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
    ) -> GuiResult<Self> {
        let actions = actions.unwrap_or_else(|| collect_actions_from_frame(&root, window.as_ref()));
        let frame = Self {
            frame_id: frame_id.into(),
            root,
            actions,
            window,
        };
        frame.validate()?;
        Ok(frame)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source(frame_id: impl Into<String>, source: &str) -> GuiResult<Self> {
        Self::from_rsx_source_parts(frame_id, source, None, None)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_with_window(
        frame_id: impl Into<String>,
        source: &str,
        window: WindowOptions,
    ) -> GuiResult<Self> {
        Self::from_rsx_source_parts(frame_id, source, None, Some(window))
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_parts(
        frame_id: impl Into<String>,
        source: &str,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
    ) -> GuiResult<Self> {
        let root = crate::rsx::parse_rsx(source)?;
        Self::from_compiled_parts(frame_id, root, actions, window)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_with_state(
        frame_id: impl Into<String>,
        source: &str,
        state: &JsonValue,
    ) -> GuiResult<Self> {
        Self::from_rsx_source_parts_with_state(frame_id, source, state, None, None)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_parts_with_state(
        frame_id: impl Into<String>,
        source: &str,
        state: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
    ) -> GuiResult<Self> {
        let scope = json!({
            "state": state,
            "props": {},
            "derived": {},
            "context": {},
            "resource": {},
        });
        Self::from_rsx_source_parts_with_scope(frame_id, source, &scope, actions, window)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_with_scope(
        frame_id: impl Into<String>,
        source: &str,
        scope: &JsonValue,
    ) -> GuiResult<Self> {
        Self::from_rsx_source_parts_with_scope(frame_id, source, scope, None, None)
    }

    #[cfg(feature = "authoring")]
    pub fn from_rsx_source_parts_with_scope(
        frame_id: impl Into<String>,
        source: &str,
        scope: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
    ) -> GuiResult<Self> {
        let root = crate::rsx::parse_rsx(source)?.resolve_bindings(scope)?;
        Self::from_compiled_parts(frame_id, root, actions, window)
    }

    pub fn validate(&self) -> GuiResult<()> {
        if self.frame_id.is_empty() {
            return Err(GuiError::invalid_tree(
                "a3s-gui frames need a non-empty string frame id",
            ));
        }
        if !matches!(&self.root, CompiledRsxNode::Element { .. }) {
            return Err(GuiError::invalid_tree(
                "a3s-gui frames need one root element",
            ));
        }
        self.root.validate()?;
        if self.root.has_bindings() {
            return Err(GuiError::invalid_tree(
                "a3s-gui frames cannot render unresolved RSX state/props/derived/context/resource bindings; use from_rsx_source_with_state or from_rsx_source_with_scope",
            ));
        }
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
                let content = RsxCompilerBridge::new().lower_to_native(&self.root)?;
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
            disabled: self.disabled,
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

fn is_false(value: &bool) -> bool {
    !*value
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
    root: &CompiledRsxNode,
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
    node: &CompiledRsxNode,
    actions: &mut Vec<UiAction>,
    indexes: &mut BTreeMap<String, usize>,
) {
    let CompiledRsxNode::Element {
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
                disabled: false,
                label,
            });
        }
    }
}

/// In-process application orchestration for the legacy protocol API.
///
/// This type intentionally does not drive protocol v1. Version-1 transports own
/// a [`NativeProtocolSession`] directly and must follow the render -> command ACK
/// -> ordered event -> reducer/effect -> next render lifecycle explicitly.
pub struct NativeProtocolApp<A: PlatformAdapter, S, F, R> {
    session: NativeProtocolSession<A>,
    state: S,
    frame_builder: F,
    action_reducer: R,
    render_effect: Option<Box<dyn FnMut(&mut S) -> GuiResult<()>>>,
    cleanup_effect: Option<Box<dyn FnMut(&mut S) -> GuiResult<()>>>,
}

impl<A, S, F, R> std::fmt::Debug for NativeProtocolApp<A, S, F, R>
where
    A: PlatformAdapter,
    S: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeProtocolApp")
            .field("state", &self.state)
            .field("has_render_effect", &self.render_effect.is_some())
            .field("has_cleanup_effect", &self.cleanup_effect.is_some())
            .finish_non_exhaustive()
    }
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
            render_effect: None,
            cleanup_effect: None,
        }
    }

    pub fn with_render_effect(
        mut self,
        effect: impl FnMut(&mut S) -> GuiResult<()> + 'static,
    ) -> Self {
        self.render_effect = Some(Box::new(effect));
        self
    }

    pub fn with_cleanup_effect(
        mut self,
        cleanup: impl FnMut(&mut S) -> GuiResult<()> + 'static,
    ) -> Self {
        self.cleanup_effect = Some(Box::new(cleanup));
        self
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
        let rendered = self.session.render_frame(&frame)?;
        if let Some(effect) = self.render_effect.as_mut() {
            effect(&mut self.state)?;
        }
        Ok(rendered)
    }

    pub fn cleanup_effects(&mut self) -> GuiResult<()> {
        if let Some(cleanup) = self.cleanup_effect.as_mut() {
            cleanup(&mut self.state)?;
        }
        Ok(())
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
            value_sensitivity: response.value_sensitivity,
            value_node: response.value_node,
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
            value_sensitivity: response.value_sensitivity,
            value_node: response.value_node,
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
            crate::error::GuiError::host("native event has no registered RSX action")
        })?;
        Ok(HostEventResponse {
            frame_id: self.frame_id.clone(),
            invocation,
            interaction_changes: handled.interaction_changes,
            value_sensitivity: handled.value_sensitivity,
            value_node: Some(self.event.node),
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
            value_sensitivity: handled.value_sensitivity,
            value_node: Some(self.event.node),
        })
    }
}

#[cfg(test)]
#[path = "protocol/tests/mod.rs"]
mod tests;
