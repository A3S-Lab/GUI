use std::collections::BTreeMap;

use crate::accessibility::{AccessibilityNode, AccessibilityRole, AccessibilityTreeHost};
use crate::backend::NativeEventHost;
use crate::compiler::{CompiledJsxNode, ReactCompilerBridge};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, ActionRegistry, EventRouter, NativeEvent};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::{InteractionChange, InteractionNodeState, InteractionState};
use crate::native::NativeElement;
use crate::platform::{BlueprintHost, NativeWidgetBlueprint};
use crate::react_aria::{AriaElement, ReactAriaMapper};
use crate::renderer::Renderer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandledNativeEvent {
    pub event: NativeEvent,
    pub invocation: Option<ActionInvocation>,
    pub interaction_changes: Vec<InteractionChange>,
}

#[derive(Debug)]
pub struct GuiRuntime<H: NativeHost> {
    bridge: ReactCompilerBridge,
    mapper: ReactAriaMapper,
    renderer: Renderer,
    host: H,
    event_router: EventRouter,
    action_registry: ActionRegistry,
    interaction_state: InteractionState,
    interaction_revisions: BTreeMap<HostNodeId, u64>,
    render_revision: u64,
}

impl<H: NativeHost> GuiRuntime<H> {
    pub fn new(host: H) -> Self {
        Self {
            bridge: ReactCompilerBridge::new(),
            mapper: ReactAriaMapper::new(),
            renderer: Renderer::new(),
            host,
            event_router: EventRouter::new(),
            action_registry: ActionRegistry::new(),
            interaction_state: InteractionState::new(),
            interaction_revisions: BTreeMap::new(),
            render_revision: 0,
        }
    }

    pub fn render_compiled(&mut self, node: &CompiledJsxNode) -> GuiResult<HostNodeId> {
        let native = self.bridge.lower_to_native(node)?;
        self.render_native(&native)
    }

    pub fn render_aria(&mut self, element: &AriaElement) -> GuiResult<HostNodeId> {
        let native = self.mapper.map(element)?;
        self.render_native(&native)
    }

    pub fn render_native(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let root = self.renderer.render(element, &mut self.host)?;
        self.render_revision = self.render_revision.saturating_add(1);
        self.prune_unmounted_interactions();
        Ok(root)
    }

    pub fn host(&self) -> &H {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut H {
        &mut self.host
    }

    pub fn actions(&self) -> &ActionRegistry {
        &self.action_registry
    }

    pub fn actions_mut(&mut self) -> &mut ActionRegistry {
        &mut self.action_registry
    }

    pub fn interactions(&self) -> &InteractionState {
        &self.interaction_state
    }

    pub fn dispatch_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<ActionInvocation> {
        self.handle_event(blueprint, event)?
            .ok_or_else(|| GuiError::host("native event has no registered Web action"))
    }

    pub fn handle_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        let event = normalize_event_value(blueprint, event);
        self.handle_event_with_routes(blueprint, &[], event)
    }

    fn handle_event_with_routes(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        self.refresh_stale_interaction_baseline(event.node, blueprint);
        let interaction_start = self.interaction_state.changes().len();
        self.interaction_state.apply_event(blueprint, &event);
        self.record_interaction_revisions(interaction_start);
        let Some(invocation) = self.route_event(blueprint, route_blueprints, &event) else {
            return Ok(None);
        };
        self.action_registry.invoke(invocation.clone())?;
        Ok(Some(invocation))
    }

    fn route_event(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        event: &NativeEvent,
    ) -> Option<ActionInvocation> {
        self.event_router.route(blueprint, event).or_else(|| {
            route_blueprints
                .iter()
                .find_map(|route_blueprint| self.event_router.route(route_blueprint, event))
        })
    }

    fn refresh_stale_interaction_baseline(
        &mut self,
        node: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) {
        if self.interaction_state.node(node).is_none()
            || self.interaction_revisions.get(&node).copied() == Some(self.render_revision)
        {
            return;
        }

        self.interaction_state
            .sync_node_from_blueprint(node, blueprint);
    }

    fn record_interaction_revisions(&mut self, interaction_start: usize) {
        for change in &self.interaction_state.changes()[interaction_start..] {
            self.interaction_revisions
                .insert(change.node, self.render_revision);
        }
    }

    fn prune_unmounted_interactions(&mut self) {
        let mounted_nodes = self.renderer.mounted_node_ids();
        self.interaction_state.retain_nodes(&mounted_nodes);
        self.interaction_revisions
            .retain(|node, _| mounted_nodes.contains(node));
    }

    pub fn into_host(self) -> H {
        self.host
    }
}

impl<H: NativeHost + BlueprintHost> GuiRuntime<H> {
    pub fn dispatch_native_event(&mut self, event: NativeEvent) -> GuiResult<ActionInvocation> {
        self.handle_native_event(event)?
            .ok_or_else(|| GuiError::host("native event has no registered Web action"))
    }

    pub fn handle_native_event(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self.handle_native_event_with_changes(event)?.invocation)
    }

    pub fn handle_native_event_with_changes(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        let blueprint = self.host.blueprint(event.node).cloned().ok_or_else(|| {
            GuiError::host(format!("native node {} has no blueprint", event.node.get()))
        })?;
        let route_blueprints = self
            .renderer
            .ancestor_ids(event.node)
            .into_iter()
            .filter_map(|node| self.host.blueprint(node).cloned())
            .collect::<Vec<_>>();
        let event = normalize_event_value(&blueprint, event);

        let interaction_start = self.interaction_state.changes().len();
        let invocation =
            self.handle_event_with_routes(&blueprint, &route_blueprints, event.clone())?;
        let interaction_changes = self.interaction_state.changes()[interaction_start..].to_vec();
        Ok(HandledNativeEvent {
            event,
            invocation,
            interaction_changes,
        })
    }
}

fn normalize_event_value(blueprint: &NativeWidgetBlueprint, mut event: NativeEvent) -> NativeEvent {
    if event.value.is_none() && event.kind == crate::event::NativeEventKind::SelectionChange {
        event.value = selected_node_value(blueprint);
    }
    event
}

fn selected_node_value(blueprint: &NativeWidgetBlueprint) -> Option<String> {
    if !is_selectable_native_role(blueprint.role) {
        return None;
    }
    blueprint.value.clone().or_else(|| blueprint.label.clone())
}

impl<H: NativeHost + AccessibilityTreeHost> GuiRuntime<H> {
    pub fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        let mut tree = self.host.accessibility_tree()?;
        apply_interactions_to_accessibility_tree(
            &mut tree,
            &self.interaction_state,
            &self.interaction_revisions,
            self.render_revision,
        );
        Some(tree)
    }
}

fn apply_interactions_to_accessibility_tree(
    node: &mut AccessibilityNode,
    interactions: &InteractionState,
    interaction_revisions: &BTreeMap<HostNodeId, u64>,
    render_revision: u64,
) {
    if let Some(id) = node.node {
        if let Some(state) = interactions.node(id) {
            let current_interaction =
                interaction_revisions.get(&id).copied() == Some(render_revision);
            apply_interaction_state(node, state, current_interaction);
        }
    }

    for child in &mut node.children {
        apply_interactions_to_accessibility_tree(
            child,
            interactions,
            interaction_revisions,
            render_revision,
        );
    }

    apply_selection_value_to_children(node);
    apply_latest_child_selection_to_children(
        node,
        interactions,
        interaction_revisions,
        render_revision,
    );
}

fn apply_interaction_state(
    node: &mut AccessibilityNode,
    state: &InteractionNodeState,
    current_interaction: bool,
) {
    node.focused = state.focused;
    if !current_interaction {
        return;
    }
    if let Some(value) = &state.value {
        node.value = Some(value.clone());
    }
    if state.selected {
        node.selected = true;
    }
    if let Some(checked) = state.checked {
        node.checked = Some(checked);
    }
    if let Some(expanded) = state.expanded {
        node.expanded = Some(expanded);
    }
}

fn apply_selection_value_to_children(node: &mut AccessibilityNode) {
    if !is_selection_container(node.role) {
        return;
    }
    let Some(value) = node.value.as_deref() else {
        return;
    };

    for child in &mut node.children {
        if is_selectable_child(child.role) {
            let selected = child_matches_selection_value(child, value);
            child.selected = selected;
            if child.role == AccessibilityRole::RadioButton {
                child.checked = Some(selected);
            }
        }
    }
}

fn apply_latest_child_selection_to_children(
    node: &mut AccessibilityNode,
    interactions: &InteractionState,
    interaction_revisions: &BTreeMap<HostNodeId, u64>,
    render_revision: u64,
) {
    if !is_exclusive_child_selection_container(node.role) {
        return;
    }
    let Some(SelectionSource::Child(selected_node)) =
        latest_selection_source(node, interactions, interaction_revisions, render_revision)
    else {
        return;
    };

    for child in &mut node.children {
        if is_selectable_child(child.role) {
            let selected = child.node == Some(selected_node);
            child.selected = selected;
            if child.role == AccessibilityRole::RadioButton {
                child.checked = Some(selected);
            }
            if selected {
                if let Some(value) = selected_accessibility_value(child) {
                    node.value = Some(value);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectionSource {
    ParentValue,
    Child(HostNodeId),
}

fn latest_selection_source(
    node: &AccessibilityNode,
    interactions: &InteractionState,
    interaction_revisions: &BTreeMap<HostNodeId, u64>,
    render_revision: u64,
) -> Option<SelectionSource> {
    for change in interactions.changes().iter().rev() {
        if interaction_revisions.get(&change.node).copied() != Some(render_revision) {
            continue;
        }
        if Some(change.node) == node.node
            && change.before.value != change.after.value
            && change.after.value.is_some()
        {
            return Some(SelectionSource::ParentValue);
        }
        if change.before.selected != change.after.selected
            && change.after.selected
            && node
                .children
                .iter()
                .any(|child| child.node == Some(change.node) && is_selectable_child(child.role))
        {
            return Some(SelectionSource::Child(change.node));
        }
    }
    None
}

fn is_selection_container(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ComboBox
            | AccessibilityRole::ListBox
            | AccessibilityRole::Menu
            | AccessibilityRole::RadioGroup
            | AccessibilityRole::TabGroup
            | AccessibilityRole::TabList
    )
}

fn is_exclusive_child_selection_container(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ComboBox
            | AccessibilityRole::RadioGroup
            | AccessibilityRole::TabGroup
            | AccessibilityRole::TabList
    )
}

fn is_selectable_child(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ListBoxOption
            | AccessibilityRole::MenuItem
            | AccessibilityRole::RadioButton
            | AccessibilityRole::Tab
    )
}

fn is_selectable_native_role(role: crate::native::NativeRole) -> bool {
    matches!(
        role,
        crate::native::NativeRole::ListBoxItem
            | crate::native::NativeRole::MenuItem
            | crate::native::NativeRole::Radio
            | crate::native::NativeRole::Tab
    )
}

fn selected_accessibility_value(child: &AccessibilityNode) -> Option<String> {
    child.value.clone().or_else(|| child.label.clone())
}

fn child_matches_selection_value(child: &AccessibilityNode, value: &str) -> bool {
    child.value.as_deref() == Some(value) || child.label.as_deref() == Some(value)
}

impl<H: NativeHost + BlueprintHost + NativeEventHost> GuiRuntime<H> {
    pub fn dispatch_pending_native_events(&mut self) -> GuiResult<Vec<ActionInvocation>> {
        let events = self.host.take_native_events();
        events
            .into_iter()
            .map(|event| self.dispatch_native_event(event))
            .collect()
    }

    pub fn handle_pending_native_events(&mut self) -> GuiResult<Vec<ActionInvocation>> {
        let events = self.handle_pending_native_event_results()?;
        Ok(events
            .into_iter()
            .filter_map(|event| event.invocation)
            .collect())
    }

    pub fn handle_pending_native_event_results(&mut self) -> GuiResult<Vec<HandledNativeEvent>> {
        let events = self.host.take_native_events();
        let mut handled = Vec::new();
        for event in events {
            handled.push(self.handle_native_event_with_changes(event)?);
        }
        Ok(handled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibility::AccessibilityRole;
    use crate::host::HeadlessHost;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{Gtk4Adapter, PlatformCommand, PlatformPlanningHost, WinUiAdapter};
    use crate::web::WebProps;

    #[test]
    fn runtime_renders_compiled_jsx_to_platform_host() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();

        let root = runtime.host().node(root_id).unwrap();
        assert_eq!(root.blueprint.widget_class, "gtk::Button");
        assert_eq!(root.blueprint.action.as_deref(), Some("saveDocument"));
    }

    #[test]
    fn runtime_exports_headless_accessibility_tree() {
        let root = NativeElement::new("dialog", NativeRole::Dialog)
            .with_props(
                NativeProps::new()
                    .label("Preferences")
                    .accessibility_description_text("Keyboard and display settings")
                    .accessibility_level(Some(1))
                    .modal(Some(true)),
            )
            .child(
                NativeElement::new("close", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Close")
                        .accessibility_controls("dialog")
                        .pressed("false"),
                ),
            );
        let mut runtime = GuiRuntime::new(HeadlessHost::default());

        let root_id = runtime.render_native(&root).unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.node, Some(root_id));
        assert_eq!(accessibility.role, AccessibilityRole::Dialog);
        assert_eq!(accessibility.label.as_deref(), Some("Preferences"));
        assert_eq!(
            accessibility.description.description.as_deref(),
            Some("Keyboard and display settings")
        );
        assert_eq!(accessibility.structure.level, Some(1));
        assert_eq!(accessibility.state.modal, Some(true));
        assert_eq!(accessibility.children.len(), 1);
        assert!(accessibility.children[0].node.is_some());
        assert_eq!(accessibility.children[0].role, AccessibilityRole::Button);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Close"));
        assert_eq!(
            accessibility.children[0].relationships.controls.as_deref(),
            Some("dialog")
        );
        assert_eq!(
            accessibility.children[0].state.pressed.as_deref(),
            Some("false")
        );
    }

    #[test]
    fn runtime_exports_platform_accessibility_tree_from_compiled_jsx() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "preferences",
              "tag": "Dialog",
              "props": {
                "attributes": {
                  "aria-label": "Preferences",
                  "aria-describedby": "preferences-help",
                  "aria-description": "Keyboard and display settings",
                  "aria-roledescription": "settings dialog",
                  "aria-level": "2",
                  "aria-posinset": "1",
                  "aria-setsize": "3",
                  "aria-hidden": "false",
                  "aria-modal": "true",
                  "aria-live": "polite"
                }
              },
              "children": [
                {
                  "kind": "element",
                  "key": "close",
                  "tag": "Button",
                  "props": {
                    "attributes": {
                      "aria-label": "Close",
                      "aria-controls": "preferences",
                      "aria-pressed": "false"
                    }
                  }
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.node, Some(root_id));
        assert_eq!(accessibility.role, AccessibilityRole::Dialog);
        assert_eq!(accessibility.label.as_deref(), Some("Preferences"));
        assert_eq!(
            accessibility.relationships.described_by.as_deref(),
            Some("preferences-help")
        );
        assert_eq!(
            accessibility.description.description.as_deref(),
            Some("Keyboard and display settings")
        );
        assert_eq!(
            accessibility.description.role_description.as_deref(),
            Some("settings dialog")
        );
        assert_eq!(accessibility.structure.level, Some(2));
        assert_eq!(accessibility.structure.position_in_set, Some(1));
        assert_eq!(accessibility.structure.set_size, Some(3));
        assert_eq!(accessibility.state.hidden, Some(false));
        assert_eq!(accessibility.state.modal, Some(true));
        assert_eq!(accessibility.state.live.as_deref(), Some("polite"));
        assert_eq!(accessibility.children.len(), 1);
        assert!(accessibility.children[0].node.is_some());
        assert_eq!(accessibility.children[0].role, AccessibilityRole::Button);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Close"));
        assert_eq!(
            accessibility.children[0].relationships.controls.as_deref(),
            Some("preferences")
        );
        assert_eq!(
            accessibility.children[0].state.pressed.as_deref(),
            Some("false")
        );
    }

    #[test]
    fn runtime_dispatches_native_event_to_registered_web_action() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let blueprint = runtime.host().node(root_id).unwrap().blueprint.clone();
        let invocation = runtime
            .dispatch_event(
                &blueprint,
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Press),
            )
            .unwrap();

        assert_eq!(invocation.action, "saveDocument");
        assert_eq!(runtime.actions().invocations().len(), 1);
    }

    #[test]
    fn runtime_handles_state_event_without_registered_action() {
        let element = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save"));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let invocation = runtime
            .handle_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();

        assert!(invocation.is_none());
        assert!(runtime.interactions().node(root_id).unwrap().focused);
        assert!(runtime.accessibility_tree().unwrap().focused);
    }

    #[test]
    fn runtime_accessibility_tree_exposes_single_focused_node() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save")),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                children[0],
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                children[1],
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].focused);
        assert!(accessibility.children[1].focused);
    }

    #[test]
    fn runtime_dispatch_stays_strict_for_unbound_events() {
        let element = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save"));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let error = runtime
            .dispatch_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap_err();

        assert!(error.to_string().contains("no registered Web action"));
        assert!(runtime.interactions().node(root_id).unwrap().focused);
    }

    #[test]
    fn runtime_updates_interaction_state_before_dispatching_action() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEmail");

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let invocation = runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("a@b.c"),
            )
            .unwrap();

        assert_eq!(invocation.action, "setEmail");
        assert_eq!(
            runtime
                .interactions()
                .node(root_id)
                .unwrap()
                .value
                .as_deref(),
            Some("a@b.c")
        );
    }

    #[test]
    fn runtime_accessibility_tree_reflects_interaction_state() {
        let tree = NativeElement::new("settings", NativeRole::Form)
            .child(
                NativeElement::new("email", NativeRole::TextField).with_props(
                    NativeProps::new()
                        .label("Email")
                        .value("old@example.com")
                        .web(WebProps::new().on_change("setEmail")),
                ),
            )
            .child(
                NativeElement::new("notifications", NativeRole::Switch).with_props(
                    NativeProps::new()
                        .label("Notifications")
                        .checked(false)
                        .web(WebProps::new().on_change("setNotifications")),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEmail");
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&tree).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        let email = children[0];
        let notifications = children[1];

        runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(email, crate::event::NativeEventKind::Change)
                    .value("new@example.com"),
            )
            .unwrap();
        runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(
                    notifications,
                    crate::event::NativeEventKind::Toggle,
                )
                .value("true"),
            )
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.children[0].node, Some(email));
        assert_eq!(
            accessibility.children[0].value.as_deref(),
            Some("new@example.com")
        );
        assert_eq!(accessibility.children[1].node, Some(notifications));
        assert_eq!(accessibility.children[1].checked, Some(true));
    }

    #[test]
    fn runtime_accessibility_tree_uses_rerendered_control_state_after_interaction() {
        let first = NativeElement::new("email", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Email")
                .value("old@example.com")
                .web(WebProps::new().on_change("setEmail")),
        );
        let second = NativeElement::new("email", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Email")
                .value("controlled@example.com")
                .web(WebProps::new().on_change("setEmail")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEmail");

        let root_id = runtime.render_native(&first).unwrap();
        runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("local@example.com"),
            )
            .unwrap();
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("local@example.com")
        );

        let second_id = runtime.render_native(&second).unwrap();

        assert_eq!(second_id, root_id);
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("controlled@example.com")
        );
    }

    #[test]
    fn runtime_interactions_start_from_rerendered_control_state() {
        let first = NativeElement::new("notifications", NativeRole::Switch)
            .with_props(NativeProps::new().label("Notifications").checked(false));
        let second = NativeElement::new("notifications", NativeRole::Switch)
            .with_props(NativeProps::new().label("Notifications").checked(false));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&first).unwrap();
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Toggle,
            ))
            .unwrap();
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));

        let second_id = runtime.render_native(&second).unwrap();
        assert_eq!(second_id, root_id);
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));

        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Toggle,
            ))
            .unwrap();

        assert_eq!(handled.interaction_changes.len(), 1);
        assert_eq!(handled.interaction_changes[0].before.checked, Some(false));
        assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
    }

    #[test]
    fn runtime_accessibility_tree_preserves_focus_across_rerender() {
        let first = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save"));
        let second = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Saved"));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&first).unwrap();
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        let second_id = runtime.render_native(&second).unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(second_id, root_id);
        assert_eq!(accessibility.label.as_deref(), Some("Saved"));
        assert!(accessibility.focused);
    }

    #[test]
    fn runtime_prunes_interaction_state_for_unmounted_nodes() {
        let first = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save")),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel")),
            );
        let second = NativeElement::new("tools", NativeRole::Toolbar).child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&first).unwrap();
        let save = runtime.host().node(root_id).unwrap().children[0];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                save,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        assert!(runtime.interactions().node(save).unwrap().focused);

        runtime.render_native(&second).unwrap();

        assert!(runtime.interactions().node(save).is_none());
        assert!(runtime.interactions().changes().is_empty());
        assert!(!runtime.accessibility_tree().unwrap().children[0].focused);
    }

    #[test]
    fn runtime_accessibility_tree_projects_selection_value_to_children() {
        let tree = NativeElement::new("project", NativeRole::Select)
            .with_props(
                NativeProps::new()
                    .label("Project")
                    .web(WebProps::new().on_selection_change("setProject")),
            )
            .child(
                NativeElement::new("a3s", NativeRole::ListBoxItem)
                    .with_props(NativeProps::new().label("A3S").value("a3s").selected(true)),
            )
            .child(
                NativeElement::new("other", NativeRole::ListBoxItem)
                    .with_props(NativeProps::new().label("Other").value("other")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setProject");

        let root_id = runtime.render_native(&tree).unwrap();
        runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value("other"),
            )
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.value.as_deref(), Some("other"));
        assert!(!accessibility.children[0].selected);
        assert!(accessibility.children[1].selected);
    }

    #[test]
    fn runtime_accessibility_tree_projects_radio_group_value_to_checked_child() {
        let tree = NativeElement::new("theme", NativeRole::RadioGroup)
            .with_props(
                NativeProps::new()
                    .label("Theme")
                    .web(WebProps::new().on_selection_change("setTheme")),
            )
            .child(
                NativeElement::new("light", NativeRole::Radio).with_props(
                    NativeProps::new()
                        .label("Light")
                        .value("light")
                        .selected(true),
                ),
            )
            .child(
                NativeElement::new("dark", NativeRole::Radio)
                    .with_props(NativeProps::new().label("Dark").value("dark")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&tree).unwrap();
        runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value("dark"),
            )
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(false));
        assert!(accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(true));

        runtime
            .handle_native_event(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value("light"),
            )
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(true));
        assert!(!accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(false));
    }

    #[test]
    fn runtime_accessibility_tree_reflects_direct_radio_selection_as_checked() {
        let tree = NativeElement::new("theme", NativeRole::RadioGroup)
            .with_props(NativeProps::new().label("Theme"))
            .child(
                NativeElement::new("light", NativeRole::Radio).with_props(
                    NativeProps::new()
                        .label("Light")
                        .value("light")
                        .selected(true)
                        .checked(true),
                ),
            )
            .child(
                NativeElement::new("dark", NativeRole::Radio)
                    .with_props(NativeProps::new().label("Dark").value("dark")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&tree).unwrap();
        let radio_id = runtime.host().node(root_id).unwrap().children[1];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                radio_id,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(false));
        assert!(accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(true));
    }

    #[test]
    fn runtime_bubbles_child_selection_to_parent_action_with_value() {
        let tree = NativeElement::new("theme", NativeRole::RadioGroup)
            .with_props(
                NativeProps::new()
                    .label("Theme")
                    .web(WebProps::new().on_selection_change("setTheme")),
            )
            .child(
                NativeElement::new("light", NativeRole::Radio).with_props(
                    NativeProps::new()
                        .label("Light")
                        .value("light")
                        .selected(true)
                        .checked(true),
                ),
            )
            .child(
                NativeElement::new("dark", NativeRole::Radio)
                    .with_props(NativeProps::new().label("Dark").value("dark")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&tree).unwrap();
        let radio_id = runtime.host().node(root_id).unwrap().children[1];
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                radio_id,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();
        let invocation = handled.invocation.unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("dark"));
        assert_eq!(invocation.node, radio_id);
        assert_eq!(invocation.action, "setTheme");
        assert_eq!(invocation.value.as_deref(), Some("dark"));
        assert_eq!(handled.interaction_changes.len(), 1);
        assert_eq!(handled.interaction_changes[0].node, radio_id);
        assert_eq!(
            handled.interaction_changes[0].after.value.as_deref(),
            Some("dark")
        );

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.value.as_deref(), Some("dark"));
        assert!(!accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(false));
        assert!(accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(true));
    }

    #[test]
    fn runtime_accessibility_tree_projects_direct_tab_selection_to_siblings() {
        let tree = NativeElement::new("settings", NativeRole::Tabs)
            .with_props(NativeProps::new().label("Settings"))
            .child(
                NativeElement::new("profile", NativeRole::Tab)
                    .with_props(NativeProps::new().label("Profile").selected(true)),
            )
            .child(
                NativeElement::new("billing", NativeRole::Tab)
                    .with_props(NativeProps::new().label("Billing")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&tree).unwrap();
        let tab_id = runtime.host().node(root_id).unwrap().children[1];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                tab_id,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].selected);
        assert!(accessibility.children[1].selected);
    }

    #[test]
    fn runtime_renders_compiled_jsx_to_native_command_stream() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "form",
              "tag": "form",
              "props": {"className": "profile-form"},
              "children": [
                {
                  "kind": "element",
                  "key": "email",
                  "tag": "TextField",
                  "children": [
                    {"kind": "element", "key": "label", "tag": "Label", "children": [
                      {"kind": "text", "key": "label-text", "value": "Email"}
                    ]},
                    {"kind": "element", "key": "input", "tag": "Input", "props": {
                      "placeholder": "you@example.com",
                      "events": {"onChange": "setEmail"}
                    }}
                  ]
                },
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveProfile"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(WinUiAdapter);
        let mut runtime = GuiRuntime::new(host);

        runtime.render_compiled(&compiled).unwrap();

        let commands = runtime.host().commands();
        assert!(commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.TextBox"
                && blueprint.label.as_deref() == Some("Email")
        )));
        assert!(commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.Button"
                && blueprint.action.as_deref() == Some("saveProfile")
        )));
    }
}
