use std::collections::{BTreeMap, BTreeSet};

use crate::backend::NativeEventHost;
use crate::compiler::{CompiledRsxNode, RsxCompilerBridge};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, ActionRegistry, EventRouter, NativeEvent};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::{InteractionChange, InteractionState};
use crate::native::{
    format_normalized_number, is_number_input_type, normalize_range_value, truncate_to_max_length,
    NativeElement, NativeProps, ValueSensitivity,
};
use crate::platform::{BlueprintHost, NativeWidgetBlueprint};
use crate::renderer::Renderer;
use crate::semantic_ui::{SemanticElement, SemanticMapper};
use crate::style::PortableStyle;
use serde::{Deserialize, Serialize};

mod accessibility;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandledNativeEvent {
    pub event: NativeEvent,
    pub invocation: Option<ActionInvocation>,
    pub interaction_changes: Vec<InteractionChange>,
    #[serde(default)]
    pub value_sensitivity: ValueSensitivity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HandledNativeEventWire {
    event: NativeEvent,
    invocation: Option<ActionInvocation>,
    interaction_changes: Vec<InteractionChange>,
}

impl Serialize for HandledNativeEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut event = self.event.clone();
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_event_output(
            &mut event,
            invocation.as_mut(),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        HandledNativeEventWire {
            event,
            invocation,
            interaction_changes,
        }
        .serialize(serializer)
    }
}

pub(crate) fn redact_event_output(
    event: &mut NativeEvent,
    invocation: Option<&mut ActionInvocation>,
    interaction_changes: &mut [InteractionChange],
    value_sensitivity: ValueSensitivity,
) {
    if !value_sensitivity.is_sensitive() {
        return;
    }
    event.value = None;
    if let Some(invocation) = invocation {
        invocation.value = None;
    }
    for change in interaction_changes {
        change.before.value = None;
        change.after.value = None;
    }
}

pub struct GuiRuntime<H: NativeHost> {
    bridge: RsxCompilerBridge,
    mapper: SemanticMapper,
    renderer: Renderer,
    host: H,
    event_router: EventRouter,
    action_registry: ActionRegistry,
    interaction_state: InteractionState,
    interaction_revisions: BTreeMap<HostNodeId, u64>,
    render_revision: u64,
}

impl<H: NativeHost> std::fmt::Debug for GuiRuntime<H> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GuiRuntime")
            .field("host_type", &std::any::type_name::<H>())
            .field("render_revision", &self.render_revision)
            .field(
                "interaction_revision_count",
                &self.interaction_revisions.len(),
            )
            .finish_non_exhaustive()
    }
}

impl<H: NativeHost> GuiRuntime<H> {
    pub fn new(host: H) -> Self {
        Self {
            bridge: RsxCompilerBridge::new(),
            mapper: SemanticMapper::new(),
            renderer: Renderer::new(),
            host,
            event_router: EventRouter::new(),
            action_registry: ActionRegistry::new(),
            interaction_state: InteractionState::new(),
            interaction_revisions: BTreeMap::new(),
            render_revision: 0,
        }
    }

    pub fn render_compiled(&mut self, node: &CompiledRsxNode) -> GuiResult<HostNodeId> {
        let native = self.bridge.lower_to_native(node)?;
        self.render_native(&native)
    }

    pub fn render_semantic(&mut self, element: &SemanticElement) -> GuiResult<HostNodeId> {
        let native = self.mapper.map(element)?;
        self.render_native(&native)
    }

    pub fn render_native(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let root = self.renderer.render(element, &mut self.host)?;
        self.render_revision = self.render_revision.saturating_add(1);
        self.prune_unmounted_interactions();
        self.initialize_auto_focus();
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

    pub fn interactions_mut(&mut self) -> &mut InteractionState {
        &mut self.interaction_state
    }

    pub fn dispatch_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<ActionInvocation> {
        self.handle_event(blueprint, event)?
            .ok_or_else(|| GuiError::host("native event has no registered RSX action"))
    }

    pub fn handle_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        self.handle_event_with_routes(blueprint, &[], event)
    }

    fn handle_event_with_routes(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self
            .handle_event_with_route_results(blueprint, route_blueprints, event)?
            .invocation)
    }

    fn handle_event_with_route_results(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        let value_sensitivity = effective_blueprint_value_sensitivity(blueprint);
        self.refresh_stale_interaction_baseline(event.node, blueprint);
        if is_invisible_or_inert_event(blueprint, route_blueprints) {
            return Ok(HandledNativeEvent {
                event,
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        if is_disabled_user_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        let event = self.normalize_event_value(blueprint, route_blueprints, event);
        if is_invalid_numeric_change_value(blueprint, &event) {
            return Ok(HandledNativeEvent {
                event,
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        if is_read_only_value_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        let invocation = self.route_event(blueprint, route_blueprints, &event);
        let interaction_snapshot = invocation.as_ref().map(|_| {
            (
                self.interaction_state.clone(),
                self.interaction_revisions.clone(),
            )
        });
        let interaction_blueprint = (blueprint.value_sensitivity != value_sensitivity).then(|| {
            let mut blueprint = blueprint.clone();
            blueprint.value_sensitivity = value_sensitivity;
            blueprint
        });
        let interaction_changes = self
            .interaction_state
            .apply_event_with_changes(interaction_blueprint.as_ref().unwrap_or(blueprint), &event);
        self.record_interaction_revisions(&interaction_changes);
        let invocation = if let Some(invocation) = invocation {
            if let Err(error) = self
                .action_registry
                .invoke_with_sensitivity(invocation.clone(), value_sensitivity)
            {
                if let Some((interaction_state, interaction_revisions)) = interaction_snapshot {
                    self.interaction_state = interaction_state;
                    self.interaction_revisions = interaction_revisions;
                }
                return Err(error);
            }
            Some(invocation)
        } else {
            None
        };
        Ok(HandledNativeEvent {
            event,
            invocation,
            interaction_changes,
            value_sensitivity,
        })
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

    fn normalize_event_value(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        mut event: NativeEvent,
    ) -> NativeEvent {
        event = normalize_keyboard_event_value(event);
        event = self.normalize_keyboard_activation(blueprint, route_blueprints, event);

        match event.kind {
            crate::event::NativeEventKind::Focus => event.value = Some("true".to_string()),
            crate::event::NativeEventKind::Blur => event.value = Some("false".to_string()),
            crate::event::NativeEventKind::SelectionChange => {
                if is_missing_selection_value(event.value.as_deref()) {
                    event.value = selected_node_value(blueprint);
                }
            }
            crate::event::NativeEventKind::Change
                if self.is_checked_change(blueprint, event.node) =>
            {
                let checked = self
                    .current_checked_state(event.node, blueprint)
                    .unwrap_or(false);
                event.value = Some(normalize_boolean_event_value(
                    event.value.as_deref(),
                    checked,
                ));
            }
            crate::event::NativeEventKind::Toggle
                if self.is_expansion_toggle(blueprint, event.node) =>
            {
                let expanded = self
                    .current_expanded_state(event.node, blueprint)
                    .unwrap_or(false);
                event.value = Some(normalize_boolean_event_value(
                    event.value.as_deref(),
                    expanded,
                ));
            }
            crate::event::NativeEventKind::Toggle
                if self.is_checked_toggle(blueprint, event.node) =>
            {
                let checked = self
                    .current_checked_state(event.node, blueprint)
                    .unwrap_or(false);
                event.value = Some(normalize_boolean_event_value(
                    event.value.as_deref(),
                    checked,
                ));
            }
            _ => {}
        }
        normalize_change_value(blueprint, event)
    }

    fn normalize_keyboard_activation(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[NativeWidgetBlueprint],
        mut event: NativeEvent,
    ) -> NativeEvent {
        if event.kind != crate::event::NativeEventKind::KeyDown
            || has_explicit_key_down_handler(blueprint, route_blueprints)
        {
            return event;
        }

        if self.is_keyboard_toggle(blueprint, event.node, event.value.as_deref()) {
            event.kind = crate::event::NativeEventKind::Toggle;
            event.value = None;
        } else if self.is_keyboard_selection(blueprint, event.value.as_deref()) {
            event.kind = crate::event::NativeEventKind::SelectionChange;
            event.value = None;
        }

        event
    }

    fn is_keyboard_toggle(
        &self,
        blueprint: &NativeWidgetBlueprint,
        node: HostNodeId,
        value: Option<&str>,
    ) -> bool {
        if self.is_expansion_toggle(blueprint, node) {
            return crate::event::is_activation_key(value);
        }

        matches!(
            blueprint.role,
            crate::native::NativeRole::Checkbox | crate::native::NativeRole::Switch
        ) && is_space_key(value)
    }

    fn is_keyboard_selection(
        &self,
        blueprint: &NativeWidgetBlueprint,
        value: Option<&str>,
    ) -> bool {
        match blueprint.role {
            crate::native::NativeRole::Radio => is_space_key(value),
            crate::native::NativeRole::ListBoxItem | crate::native::NativeRole::Tab => {
                crate::event::is_activation_key(value)
            }
            _ => false,
        }
    }

    fn is_expansion_toggle(&self, blueprint: &NativeWidgetBlueprint, node: HostNodeId) -> bool {
        matches!(
            blueprint.role,
            crate::native::NativeRole::Disclosure
                | crate::native::NativeRole::DisclosureSummary
                | crate::native::NativeRole::Popover
        ) || self.current_expanded_state(node, blueprint).is_some()
    }

    fn current_expanded_state(
        &self,
        node: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> Option<bool> {
        if self.interaction_revisions.get(&node).copied() == Some(self.render_revision) {
            if let Some(expanded) = self
                .interaction_state
                .node(node)
                .and_then(|state| state.expanded)
            {
                return Some(expanded);
            }
        }
        blueprint.control_state.expanded
    }

    fn is_checked_toggle(&self, blueprint: &NativeWidgetBlueprint, node: HostNodeId) -> bool {
        matches!(
            blueprint.role,
            crate::native::NativeRole::Checkbox | crate::native::NativeRole::Switch
        ) || self.current_checked_state(node, blueprint).is_some()
    }

    fn is_checked_change(&self, blueprint: &NativeWidgetBlueprint, node: HostNodeId) -> bool {
        matches!(
            blueprint.role,
            crate::native::NativeRole::Checkbox
                | crate::native::NativeRole::Switch
                | crate::native::NativeRole::Radio
        ) || self.current_checked_state(node, blueprint).is_some()
    }

    fn current_checked_state(
        &self,
        node: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> Option<bool> {
        if self.interaction_revisions.get(&node).copied() == Some(self.render_revision) {
            if let Some(checked) = self
                .interaction_state
                .node(node)
                .and_then(|state| state.checked)
            {
                return Some(checked);
            }
        }
        blueprint.control_state.checked
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

    fn record_interaction_revisions(&mut self, changes: &[InteractionChange]) {
        for change in changes {
            self.interaction_revisions
                .insert(change.node, self.render_revision);
        }
    }

    fn prune_unmounted_interactions(&mut self) {
        let interactive_nodes = self.interactive_mounted_node_ids();
        self.interaction_state.retain_nodes(&interactive_nodes);
        self.interaction_revisions
            .retain(|node, _| interactive_nodes.contains(node));
    }

    fn interactive_mounted_node_ids(&self) -> BTreeSet<HostNodeId> {
        let mounted_props = self.renderer.mounted_node_props();
        let props_by_node = mounted_props
            .iter()
            .map(|(node, props)| (*node, props))
            .collect::<BTreeMap<_, _>>();

        mounted_props
            .iter()
            .filter_map(|(node, props)| {
                if can_retain_interactions(props)
                    && self
                        .renderer
                        .ancestor_ids(*node)
                        .into_iter()
                        .all(|ancestor| {
                            props_by_node
                                .get(&ancestor)
                                .map(|props| can_retain_interactions(props))
                                .unwrap_or(true)
                        })
                {
                    Some(*node)
                } else {
                    None
                }
            })
            .collect()
    }

    fn initialize_auto_focus(&mut self) {
        if self.interaction_state.has_focused_node() || self.interaction_state.has_focus_history() {
            return;
        }

        let mounted_props = self.renderer.mounted_node_props();
        let props_by_node = mounted_props
            .iter()
            .map(|(node, props)| (*node, props))
            .collect::<BTreeMap<_, _>>();
        let Some((node, props)) = mounted_props.iter().find(|(node, props)| {
            can_auto_focus(props)
                && self
                    .renderer
                    .ancestor_ids(*node)
                    .into_iter()
                    .all(|ancestor| {
                        props_by_node
                            .get(&ancestor)
                            .map(|props| can_auto_focus_through(props))
                            .unwrap_or(true)
                    })
        }) else {
            return;
        };

        self.interaction_state
            .set_initial_focus_from_props(*node, props);
        self.interaction_revisions
            .insert(*node, self.render_revision);
    }

    pub fn into_host(self) -> H {
        self.host
    }
}

pub(crate) fn effective_blueprint_value_sensitivity(
    blueprint: &NativeWidgetBlueprint,
) -> ValueSensitivity {
    blueprint.effective_value_sensitivity()
}

impl<H: NativeHost + BlueprintHost> GuiRuntime<H> {
    pub fn dispatch_native_event(&mut self, event: NativeEvent) -> GuiResult<ActionInvocation> {
        self.handle_native_event(event)?
            .ok_or_else(|| GuiError::host("native event has no registered RSX action"))
    }

    pub fn handle_native_event(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self.handle_native_event_with_changes(event)?.invocation)
    }

    pub fn handle_native_event_with_changes(
        &mut self,
        mut event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        event.validate()?;
        let Some(blueprint) = self.host.blueprint(event.node).cloned() else {
            return Ok(HandledNativeEvent {
                event,
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity: ValueSensitivity::Public,
            });
        };
        let route_blueprints = self
            .renderer
            .ancestor_ids(event.node)
            .into_iter()
            .filter_map(|node| self.host.blueprint(node).cloned())
            .collect::<Vec<_>>();
        event = self.infer_container_selection_value(&blueprint, event);
        self.handle_event_with_route_results(&blueprint, &route_blueprints, event)
    }

    fn infer_container_selection_value(
        &self,
        blueprint: &NativeWidgetBlueprint,
        mut event: NativeEvent,
    ) -> NativeEvent {
        if event.kind != crate::event::NativeEventKind::SelectionChange
            || !is_missing_selection_value(event.value.as_deref())
            || !is_selection_container_native_role(blueprint.role)
        {
            return event;
        }

        event.value = selected_child_value(
            self.renderer
                .child_ids(event.node)
                .into_iter()
                .filter_map(|child| self.host.blueprint(child)),
        )
        .or_else(|| blueprint.value.clone());
        event
    }
}

fn is_missing_selection_value(value: Option<&str>) -> bool {
    match value {
        Some(value) => value.trim().is_empty(),
        None => true,
    }
}

fn selected_node_value(blueprint: &NativeWidgetBlueprint) -> Option<String> {
    if !is_selectable_native_role(blueprint.role) {
        return None;
    }
    blueprint.value.clone().or_else(|| blueprint.label.clone())
}

fn selected_child_value<'a>(
    blueprints: impl IntoIterator<Item = &'a NativeWidgetBlueprint>,
) -> Option<String> {
    blueprints
        .into_iter()
        .find(|blueprint| {
            is_selectable_native_role(blueprint.role)
                && (blueprint.control_state.selected
                    || blueprint.control_state.checked == Some(true))
        })
        .and_then(selected_node_value)
}

fn normalize_keyboard_event_value(mut event: NativeEvent) -> NativeEvent {
    if matches!(
        event.kind,
        crate::event::NativeEventKind::KeyDown | crate::event::NativeEventKind::KeyUp
    ) {
        event.value = event.value.as_deref().map(crate::event::native_key_value);
    }
    event
}

fn normalize_boolean_event_value(value: Option<&str>, current: bool) -> String {
    parse_event_bool(value).unwrap_or(!current).to_string()
}

fn parse_event_bool(value: Option<&str>) -> Option<bool> {
    match value?.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "on" => Some(true),
        "false" | "0" | "off" => Some(false),
        _ => None,
    }
}

fn normalize_change_value(blueprint: &NativeWidgetBlueprint, event: NativeEvent) -> NativeEvent {
    if event.kind != crate::event::NativeEventKind::Change {
        return event;
    }

    match blueprint.role {
        crate::native::NativeRole::TextField => normalize_text_field_change_value(blueprint, event),
        crate::native::NativeRole::Slider => normalize_ranged_change_value(blueprint, event),
        _ => event,
    }
}

fn normalize_text_field_change_value(
    blueprint: &NativeWidgetBlueprint,
    event: NativeEvent,
) -> NativeEvent {
    let event = normalize_text_change_value(blueprint, event);
    if is_number_text_input(blueprint) {
        normalize_ranged_change_value(blueprint, event)
    } else {
        event
    }
}

fn normalize_text_change_value(
    blueprint: &NativeWidgetBlueprint,
    mut event: NativeEvent,
) -> NativeEvent {
    if let (Some(max_length), Some(value)) =
        (blueprint.control_state.max_length, event.value.as_deref())
    {
        event.value = Some(truncate_to_max_length(value, max_length));
    }
    event
}

fn is_number_text_input(blueprint: &NativeWidgetBlueprint) -> bool {
    is_number_input_type(blueprint.control_state.input_type.as_deref())
}

fn normalize_ranged_change_value(
    blueprint: &NativeWidgetBlueprint,
    mut event: NativeEvent,
) -> NativeEvent {
    let Some(value) = event.value.as_deref().and_then(parse_event_number) else {
        return event;
    };
    if !value.is_finite() {
        return event;
    }
    let min = blueprint.control_state.min;
    let max = blueprint.control_state.max;
    let step = blueprint.control_state.step;
    let Some(value) = normalize_range_value(value, min, max, step) else {
        return event;
    };

    event.value = Some(format_normalized_number(value));
    event
}

fn is_invalid_numeric_change_value(blueprint: &NativeWidgetBlueprint, event: &NativeEvent) -> bool {
    if event.kind != crate::event::NativeEventKind::Change || !is_numeric_change_target(blueprint) {
        return false;
    }

    let Some(value) = event.value.as_deref().and_then(parse_event_number) else {
        return true;
    };
    !value.is_finite()
}

fn is_numeric_change_target(blueprint: &NativeWidgetBlueprint) -> bool {
    matches!(blueprint.role, crate::native::NativeRole::Slider)
        || (matches!(blueprint.role, crate::native::NativeRole::TextField)
            && is_number_text_input(blueprint))
}

fn parse_event_number(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    value.parse::<f64>().ok()
}

fn has_explicit_key_down_handler(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[NativeWidgetBlueprint],
) -> bool {
    crate::event::non_empty_action(blueprint.events.get("onKeyDown")).is_some()
        || route_blueprints.iter().any(|route_blueprint| {
            crate::event::non_empty_action(route_blueprint.events.get("onKeyDown")).is_some()
        })
}

fn is_invisible_or_inert_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[NativeWidgetBlueprint],
) -> bool {
    is_invisible_or_inert(blueprint) || route_blueprints.iter().any(is_invisible_or_inert)
}

fn is_invisible_or_inert(blueprint: &NativeWidgetBlueprint) -> bool {
    !is_visible_blueprint(blueprint) || is_inert_blueprint(blueprint)
}

fn is_visible_blueprint(blueprint: &NativeWidgetBlueprint) -> bool {
    !blueprint.control_state.hidden
        && blueprint.portable_style.renders_native_widget()
        && blueprint.control_state.html_dialog.open.unwrap_or(true)
}

fn is_inert_blueprint(blueprint: &NativeWidgetBlueprint) -> bool {
    blueprint.control_state.inert || blueprint.portable_style.makes_native_widget_inert()
}

fn can_auto_focus(props: &NativeProps) -> bool {
    props.auto_focus && can_auto_focus_through(props)
}

fn can_auto_focus_through(props: &NativeProps) -> bool {
    !props.disabled && can_retain_interactions(props)
}

fn can_retain_interactions(props: &NativeProps) -> bool {
    let style = PortableStyle::from_web(&props.web);
    !props.hidden
        && !props.inert
        && props.html_dialog.open.unwrap_or(true)
        && style.renders_native_widget()
        && !style.makes_native_widget_inert()
}

fn is_disabled_user_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[NativeWidgetBlueprint],
    event: crate::event::NativeEventKind,
) -> bool {
    if matches!(
        event,
        crate::event::NativeEventKind::Focus
            | crate::event::NativeEventKind::Blur
            | crate::event::NativeEventKind::Close
    ) {
        return false;
    }

    blueprint.control_state.disabled
        || route_blueprints
            .iter()
            .any(|route_blueprint| route_blueprint.control_state.disabled)
}

fn is_read_only_value_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[NativeWidgetBlueprint],
    event: crate::event::NativeEventKind,
) -> bool {
    if !is_value_mutation_event(event) {
        return false;
    }
    if blueprint.control_state.read_only {
        return true;
    }

    route_blueprints.iter().any(|route_blueprint| {
        read_only_ancestor_suppresses_event(route_blueprint, blueprint, event)
    })
}

fn is_value_mutation_event(event: crate::event::NativeEventKind) -> bool {
    matches!(
        event,
        crate::event::NativeEventKind::Change
            | crate::event::NativeEventKind::SelectionChange
            | crate::event::NativeEventKind::Toggle
    )
}

fn read_only_ancestor_suppresses_event(
    route_blueprint: &NativeWidgetBlueprint,
    blueprint: &NativeWidgetBlueprint,
    event: crate::event::NativeEventKind,
) -> bool {
    route_blueprint.control_state.read_only
        && is_value_mutation_event(event)
        && is_selection_container_native_role(route_blueprint.role)
        && is_selectable_native_role(blueprint.role)
}

fn is_space_key(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = value.trim();
    value == " "
        || normalized.eq_ignore_ascii_case("space")
        || normalized.eq_ignore_ascii_case("spacebar")
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

fn is_selection_container_native_role(role: crate::native::NativeRole) -> bool {
    matches!(
        role,
        crate::native::NativeRole::Select
            | crate::native::NativeRole::ComboBox
            | crate::native::NativeRole::ListBox
            | crate::native::NativeRole::Menu
            | crate::native::NativeRole::RadioGroup
            | crate::native::NativeRole::Tabs
            | crate::native::NativeRole::TabList
    )
}

impl<H: NativeHost + BlueprintHost + NativeEventHost> GuiRuntime<H> {
    pub fn dispatch_pending_native_events(&mut self) -> GuiResult<Vec<ActionInvocation>> {
        self.handle_pending_native_events()
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
mod tests;
