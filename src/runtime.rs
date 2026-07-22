use std::collections::{BTreeMap, BTreeSet};

use crate::accessibility::{
    AccessibilityConformanceReport, AccessibilityNode, AccessibilityRole, AccessibilityTreeHost,
};
use crate::backend::NativeEventHost;
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::compiler::{CompiledRsxNode, RsxCompilerBridge};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, ActionRegistry, EventRouter, NativeEvent};
use crate::focus::{FocusManager, FocusNavigationMode};
use crate::host::{HostNodeId, NativeHost, ProgrammaticFocusHost};
use crate::i18n::I18nManager;
use crate::input::NativeInputModality;
use crate::interaction::{InteractionChange, InteractionNodeState, InteractionState};
use crate::native::{
    format_normalized_number, is_number_input_type, normalize_range_value, truncate_to_max_length,
    NativeElement, NativeProps, NativeRole,
};
use crate::platform::{BlueprintHost, NativeWidgetBlueprint};
use crate::renderer::Renderer;
use crate::selection::{
    apply_item_selection_props, apply_item_tree_props, validate_native_collection_keys,
    MountedSelectionRegistry, MountedSelectionUpdate,
};
use crate::semantic_ui::{SemanticElement, SemanticMapper};
use crate::style::PortableStyle;
use serde::{Deserialize, Serialize};

type RoutedBlueprint = (HostNodeId, NativeWidgetBlueprint);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandledNativeEvent {
    pub event: NativeEvent,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invocations: Vec<ActionInvocation>,
    /// First invocation retained for compatibility with the original
    /// single-action event API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invocation: Option<ActionInvocation>,
    pub interaction_changes: Vec<InteractionChange>,
}

#[derive(Debug)]
pub struct GuiRuntime<H: NativeHost> {
    bridge: RsxCompilerBridge,
    mapper: SemanticMapper,
    renderer: Renderer,
    host: H,
    event_router: EventRouter,
    action_registry: ActionRegistry,
    focus_manager: FocusManager,
    i18n_manager: I18nManager,
    selection_registry: MountedSelectionRegistry,
    interaction_state: InteractionState,
    focus_owner: Option<HostNodeId>,
    pending_focus_modality: Option<(HostNodeId, NativeInputModality)>,
    interaction_revisions: BTreeMap<HostNodeId, u64>,
    render_revision: u64,
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
            focus_manager: FocusManager::new(),
            i18n_manager: I18nManager::new(),
            selection_registry: MountedSelectionRegistry::new(),
            interaction_state: InteractionState::new(),
            focus_owner: None,
            pending_focus_modality: None,
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
        let focus_before_render = self
            .focus_owner
            .or_else(|| self.interaction_state.focused_node());
        let mut element = element.clone();
        self.i18n_manager.project_native_tree(&mut element);
        self.selection_registry.project_native_tree(&mut element);
        validate_native_collection_keys(&element)?;
        let root = self.renderer.render(&element, &mut self.host)?;
        self.render_revision = self.render_revision.saturating_add(1);
        let snapshot = self.renderer.mounted_snapshot();
        let restore_focus = self
            .focus_manager
            .sync_with_focus(&snapshot, focus_before_render);
        if self
            .focus_owner
            .is_some_and(|focused| !snapshot.iter().any(|record| record.node == focused))
        {
            self.focus_owner = None;
        }
        self.i18n_manager.sync(&snapshot);
        self.selection_registry.sync(&snapshot)?;
        self.project_mounted_selection_to_host()?;
        self.project_mounted_tree_to_host()?;
        self.focus_manager.sync(&self.renderer.mounted_snapshot());
        let tree_focus_fallback = focus_before_render
            .and_then(|focused| self.selection_registry.tree_focus_fallback(focused));
        self.prune_unmounted_interactions();
        self.sync_mounted_selection_interactions();
        let auto_focus = self.auto_focus_target();
        if let Some(target) = tree_focus_fallback.or(restore_focus).or(auto_focus) {
            if let Some(host) = self.host.programmatic_focus_host() {
                host.request_focus(target)?;
                self.pending_focus_modality = Some((target, NativeInputModality::Virtual));
            }
        }
        if restore_focus.is_none() {
            if let Some(target) = auto_focus {
                self.initialize_auto_focus(target);
            }
        }
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

    pub fn focus_manager(&self) -> &FocusManager {
        &self.focus_manager
    }

    pub fn i18n(&self) -> &I18nManager {
        &self.i18n_manager
    }

    pub fn i18n_mut(&mut self) -> &mut I18nManager {
        &mut self.i18n_manager
    }

    pub fn selections(&self) -> &MountedSelectionRegistry {
        &self.selection_registry
    }

    pub fn mounted_snapshot(&self) -> Vec<crate::renderer::MountedNodeSnapshot> {
        self.renderer.mounted_snapshot()
    }

    pub fn dispatch_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<ActionInvocation> {
        self.handle_event(blueprint, event)?
            .ok_or_else(|| GuiError::host("native event has no registered RSX action"))
    }

    pub fn dispatch_events(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<Vec<ActionInvocation>> {
        let invocations = self.handle_events(blueprint, event)?;
        if invocations.is_empty() {
            return Err(GuiError::host("native event has no registered RSX action"));
        }
        Ok(invocations)
    }

    pub fn handle_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        self.handle_event_with_routes(blueprint, &[], event)
    }

    pub fn handle_events(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<Vec<ActionInvocation>> {
        Ok(self
            .handle_event_with_route_results(blueprint, &[], event)?
            .invocations)
    }

    fn handle_event_with_routes(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self
            .handle_event_with_route_results(blueprint, route_blueprints, event)?
            .invocation)
    }

    fn handle_event_with_route_results(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        mut event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        if event.kind == crate::event::NativeEventKind::Focus {
            if let Some((target, modality)) = self.pending_focus_modality.take() {
                if target == event.node && event.context.modality == NativeInputModality::Unknown {
                    event.context.modality = modality;
                }
            }
        }
        self.refresh_stale_interaction_baseline(event.node, blueprint);
        if is_invisible_or_inert_event(blueprint, route_blueprints) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        if is_disabled_user_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        if matches!(
            event.kind,
            crate::event::NativeEventKind::Change | crate::event::NativeEventKind::Toggle
        ) && is_read_only_value_event(blueprint, route_blueprints, event.kind)
        {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        let mut event = self.normalize_event_value(blueprint, route_blueprints, event);
        if is_invalid_numeric_change_value(blueprint, &event) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        if is_read_only_value_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        let selection_snapshot = matches!(
            event.kind,
            crate::event::NativeEventKind::SelectionChange
                | crate::event::NativeEventKind::Focus
                | crate::event::NativeEventKind::Blur
        )
        .then(|| self.selection_registry.clone());
        let selection_update = if event.kind == crate::event::NativeEventKind::SelectionChange {
            self.selection_registry.apply_event(&event)?
        } else {
            None
        };
        if matches!(
            event.kind,
            crate::event::NativeEventKind::Focus | crate::event::NativeEventKind::Blur
        ) {
            self.selection_registry.apply_focus_event(&event);
        }
        if let Some(update) = &selection_update {
            event.value = Some(update.event_value());
            if !update.changed {
                return Ok(HandledNativeEvent {
                    event,
                    invocations: Vec::new(),
                    invocation: None,
                    interaction_changes: Vec::new(),
                });
            }
            if let Err(error) = self.project_mounted_selection_to_host() {
                if let Some(selection_snapshot) = selection_snapshot {
                    self.selection_registry = selection_snapshot;
                }
                return Err(error);
            }
        }
        let invocations = self.route_event(blueprint, route_blueprints, &event);
        let interaction_snapshot = (!invocations.is_empty()).then(|| {
            (
                self.interaction_state.clone(),
                self.interaction_revisions.clone(),
                selection_snapshot,
            )
        });
        let interaction_start = self.interaction_state.changes().len();
        self.interaction_state.apply_event(blueprint, &event);
        if event.kind == crate::event::NativeEventKind::Focus {
            self.focus_owner = Some(event.node);
        }
        if let Some(update) = &selection_update {
            self.apply_mounted_selection_update(update);
        }
        self.record_interaction_revisions(interaction_start);
        if let Err(error) = self.action_registry.invoke_all(&invocations) {
            if let Some((interaction_state, interaction_revisions, selection_snapshot)) =
                interaction_snapshot
            {
                self.interaction_state = interaction_state;
                self.interaction_revisions = interaction_revisions;
                if let Some(selection_snapshot) = selection_snapshot {
                    self.selection_registry = selection_snapshot;
                    let _ = self.project_mounted_selection_to_host();
                }
            }
            return Err(error);
        }
        let invocation = invocations.first().cloned();
        let interaction_changes = self.interaction_state.changes()[interaction_start..].to_vec();
        Ok(HandledNativeEvent {
            event,
            invocations,
            invocation,
            interaction_changes,
        })
    }

    fn route_event(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        event: &NativeEvent,
    ) -> Vec<ActionInvocation> {
        let mut invocations = self
            .event_router
            .route_all_for_current_target(blueprint, event, event.node);
        if event.kind == crate::event::NativeEventKind::Close {
            return invocations;
        }
        for (current_target, route_blueprint) in route_blueprints {
            invocations.extend(self.event_router.route_all_for_current_target(
                route_blueprint,
                event,
                *current_target,
            ));
        }
        invocations
    }

    fn normalize_event_value(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        mut event: NativeEvent,
    ) -> NativeEvent {
        if event.context.modality == crate::input::NativeInputModality::Unknown {
            event.context.modality = event.effective_modality();
        }
        event = normalize_keyboard_event_value(event);
        event = self.normalize_keyboard_activation(blueprint, route_blueprints, event);

        match event.kind {
            crate::event::NativeEventKind::PressStart => event.value = Some("true".to_string()),
            crate::event::NativeEventKind::PressEnd
            | crate::event::NativeEventKind::PressCancel => event.value = Some("false".to_string()),
            crate::event::NativeEventKind::LongPressStart => event.value = Some("true".to_string()),
            crate::event::NativeEventKind::LongPressEnd => event.value = Some("false".to_string()),
            crate::event::NativeEventKind::Focus => event.value = Some("true".to_string()),
            crate::event::NativeEventKind::Blur => event.value = Some("false".to_string()),
            crate::event::NativeEventKind::HoverStart => event.value = Some("true".to_string()),
            crate::event::NativeEventKind::HoverEnd => event.value = Some("false".to_string()),
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
        route_blueprints: &[RoutedBlueprint],
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
        } else if self.is_keyboard_selection(blueprint, event.node, event.value.as_deref()) {
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
        node: HostNodeId,
        value: Option<&str>,
    ) -> bool {
        match blueprint.role {
            crate::native::NativeRole::Radio => is_space_key(value),
            crate::native::NativeRole::TreeItem => is_space_key(value),
            crate::native::NativeRole::ListBoxItem => {
                if crate::event::native_key_value(value.unwrap_or_default())
                    .eq_ignore_ascii_case("enter")
                    && self.selection_registry.has_action_for_item(node)
                {
                    false
                } else {
                    crate::event::is_activation_key(value)
                }
            }
            crate::native::NativeRole::Tab => crate::event::is_activation_key(value),
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

    fn record_interaction_revisions(&mut self, interaction_start: usize) {
        for change in &self.interaction_state.changes()[interaction_start..] {
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

    fn sync_mounted_selection_interactions(&mut self) {
        let props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        for projection in self.selection_registry.projections() {
            if let Some(container_props) = props.get(&projection.collection) {
                self.interaction_state.sync_collection_container_from_props(
                    projection.collection,
                    container_props,
                    projection.event_value(),
                );
                self.interaction_revisions
                    .insert(projection.collection, self.render_revision);
            }
            for (node, role, selected) in projection.items {
                let Some(item_props) = props.get(&node) else {
                    continue;
                };
                self.interaction_state
                    .sync_collection_item_from_props(node, role, item_props, selected);
                self.interaction_revisions
                    .insert(node, self.render_revision);
            }
        }
    }

    fn project_mounted_selection_to_host(&mut self) -> GuiResult<()> {
        let mut props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let mut updates = BTreeMap::new();
        for projection in self.selection_registry.projections() {
            for (node, role, selected) in projection.items {
                let Some(item_props) = props.get_mut(&node) else {
                    continue;
                };
                let before = item_props.clone();
                apply_item_selection_props(item_props, role, selected);
                if *item_props != before {
                    updates.insert(node, item_props.clone());
                }
            }
        }
        self.renderer.update_mounted_props(&updates, &mut self.host)
    }

    fn project_mounted_tree_to_host(&mut self) -> GuiResult<()> {
        let mut props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let mut updates = BTreeMap::new();
        for projection in self.selection_registry.tree_projections() {
            for item in projection.items {
                let Some(item_props) = props.get_mut(&item.node) else {
                    continue;
                };
                let before = item_props.clone();
                apply_item_tree_props(item_props, item.expanded, item.hidden);
                if *item_props != before {
                    updates.insert(item.node, item_props.clone());
                }
            }
        }
        self.renderer.update_mounted_props(&updates, &mut self.host)
    }

    fn apply_mounted_selection_update(&mut self, update: &MountedSelectionUpdate) {
        let props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        if let Some(container_props) = props.get(&update.collection) {
            if self.interaction_revisions.get(&update.collection).copied()
                != Some(self.render_revision)
            {
                self.interaction_state.sync_collection_container_from_props(
                    update.collection,
                    container_props,
                    update.event_value(),
                );
            }
            self.interaction_state.set_collection_value(
                update.collection,
                container_props,
                update.event_value(),
            );
        }
        for (node, role, selected) in &update.items {
            let Some(item_props) = props.get(node) else {
                continue;
            };
            if self.interaction_revisions.get(node).copied() != Some(self.render_revision) {
                self.interaction_state
                    .sync_collection_item_from_props(*node, *role, item_props, *selected);
            }
            self.interaction_state
                .set_collection_item_selected(*node, *role, item_props, *selected);
        }
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

    fn auto_focus_target(&self) -> Option<HostNodeId> {
        if self.interaction_state.has_focused_node() || self.interaction_state.has_focus_history() {
            return None;
        }

        self.focus_manager.auto_focus_target()
    }

    fn initialize_auto_focus(&mut self, node: HostNodeId) {
        let Some(props) = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .find_map(|(candidate, props)| (candidate == node).then_some(props))
        else {
            return;
        };

        self.interaction_state
            .set_initial_focus_from_props(node, &props);
        self.interaction_revisions
            .insert(node, self.render_revision);
    }

    pub fn into_host(self) -> H {
        self.host
    }
}

impl<H: NativeHost + CapabilityHost> GuiRuntime<H> {
    pub fn capabilities(&self) -> NativeCapabilities {
        self.host.native_capabilities()
    }
}

impl<H: NativeHost + ProgrammaticFocusHost> GuiRuntime<H> {
    /// Requests native focus for a mounted focusable node.
    ///
    /// If focus currently sits inside a contained focus scope, requests outside
    /// that scope are redirected according to [`FocusManager::constrain_focus`].
    /// Interaction state is updated by the resulting native focus event, so the
    /// platform remains the source of truth for whether focus actually moved.
    pub fn request_focus(&mut self, requested: HostNodeId) -> GuiResult<HostNodeId> {
        let target = if let Some(current) = self.interaction_state.focused_node() {
            self.focus_manager.constrain_focus(current, requested)
        } else {
            self.focus_manager
                .is_focusable(requested)
                .then_some(requested)
        }
        .ok_or_else(|| {
            GuiError::host(format!(
                "host node {} is not a mounted focusable node",
                requested.get()
            ))
        })?;

        self.host.request_focus(target)?;
        self.pending_focus_modality = Some((target, NativeInputModality::Virtual));
        Ok(target)
    }

    pub fn focus_first(
        &mut self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
    ) -> GuiResult<Option<HostNodeId>> {
        let target = self.focus_manager.first(scope, mode);
        self.request_optional_focus(target)
    }

    pub fn focus_last(
        &mut self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
    ) -> GuiResult<Option<HostNodeId>> {
        let target = self.focus_manager.last(scope, mode);
        self.request_optional_focus(target)
    }

    pub fn focus_next(
        &mut self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
        wrap: bool,
    ) -> GuiResult<Option<HostNodeId>> {
        let target = match self.interaction_state.focused_node() {
            Some(current) => self.focus_manager.next(current, scope, mode, wrap),
            None => self.focus_manager.first(scope, mode),
        };
        self.request_optional_focus(target)
    }

    pub fn focus_previous(
        &mut self,
        scope: Option<HostNodeId>,
        mode: FocusNavigationMode,
        wrap: bool,
    ) -> GuiResult<Option<HostNodeId>> {
        let target = match self.interaction_state.focused_node() {
            Some(current) => self.focus_manager.previous(current, scope, mode, wrap),
            None => self.focus_manager.last(scope, mode),
        };
        self.request_optional_focus(target)
    }

    fn request_optional_focus(
        &mut self,
        target: Option<HostNodeId>,
    ) -> GuiResult<Option<HostNodeId>> {
        target.map(|target| self.request_focus(target)).transpose()
    }
}

impl<H: NativeHost + BlueprintHost> GuiRuntime<H> {
    pub fn dispatch_native_event(&mut self, event: NativeEvent) -> GuiResult<ActionInvocation> {
        self.handle_native_event(event)?
            .ok_or_else(|| GuiError::host("native event has no registered RSX action"))
    }

    pub fn dispatch_native_events(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<Vec<ActionInvocation>> {
        let invocations = self.handle_native_events(event)?;
        if invocations.is_empty() {
            return Err(GuiError::host("native event has no registered RSX action"));
        }
        Ok(invocations)
    }

    pub fn handle_native_event(
        &mut self,
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self.handle_native_event_with_changes(event)?.invocation)
    }

    pub fn handle_native_events(&mut self, event: NativeEvent) -> GuiResult<Vec<ActionInvocation>> {
        Ok(self.handle_native_event_with_changes(event)?.invocations)
    }

    pub fn handle_native_event_with_changes(
        &mut self,
        mut event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        event.validate()?;
        if self.redirect_contained_focus(&event)? {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        }
        let Some((mut blueprint, mut route_blueprints)) = self.native_event_route(event.node)
        else {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
            });
        };
        event = normalize_keyboard_event_value(event);
        if event.context.modality == NativeInputModality::Unknown {
            event.context.modality = event.effective_modality();
        }
        match event.kind {
            crate::event::NativeEventKind::PressStart
                if !is_disabled_user_event(&blueprint, &route_blueprints, event.kind) =>
            {
                self.selection_registry.begin_action_press(&event);
            }
            crate::event::NativeEventKind::PressCancel => {
                self.selection_registry.cancel_action_press(&event);
            }
            crate::event::NativeEventKind::Press => {
                if let Some(action) = self.selection_registry.take_action(&event) {
                    event.node = action.item;
                    event.kind = crate::event::NativeEventKind::Action;
                    event.value = Some(action.event_value());
                }
            }
            crate::event::NativeEventKind::LongPress
                if !is_disabled_user_event(&blueprint, &route_blueprints, event.kind)
                    && !is_read_only_value_event(
                        &blueprint,
                        &route_blueprints,
                        crate::event::NativeEventKind::SelectionChange,
                    )
                    && self.selection_registry.take_long_press_selection(&event) =>
            {
                event.kind = crate::event::NativeEventKind::SelectionChange;
                event.value = None;
            }
            _ => {}
        }
        if self
            .selection_registry
            .take_suppressed_native_selection(&event)
        {
            return self.restore_suppressed_native_selection(event);
        }
        if !has_explicit_key_down_handler(&blueprint, &route_blueprints)
            && !move_handles_keyboard_event(&blueprint, &route_blueprints, &event)
        {
            let selection_before_navigation = self.selection_registry.clone();
            let direction = self.i18n_manager.direction(event.node);
            let locale = self.i18n_manager.locale(event.node).map(ToOwned::to_owned);
            if let Some(navigation) = self.selection_registry.keyboard_navigation_with_locale(
                &event,
                direction,
                locale.as_deref(),
            ) {
                if let Some(selection) = navigation.selection {
                    let (selection_blueprint, selection_route) = self
                        .native_event_route(selection.collection)
                        .ok_or_else(|| GuiError::host("selection owner unmounted"))?;
                    event.node = selection.collection;
                    event.kind = crate::event::NativeEventKind::SelectionChange;
                    event.value = Some(selection.event_value());
                    return self.handle_event_with_route_results(
                        &selection_blueprint,
                        &selection_route,
                        event,
                    );
                }
                if let Some(expansion) = navigation.expansion {
                    let Some((tree_blueprint, tree_route)) =
                        self.native_event_route(expansion.collection)
                    else {
                        self.selection_registry = selection_before_navigation;
                        return Err(GuiError::host("tree expansion owner unmounted"));
                    };
                    if tree_blueprint.control_state.disabled
                        || tree_blueprint.control_state.read_only
                    {
                        self.selection_registry = selection_before_navigation;
                    } else {
                        if let Err(error) = self.project_mounted_tree_to_host() {
                            self.selection_registry = selection_before_navigation;
                            let _ = self.project_mounted_tree_to_host();
                            return Err(error);
                        }
                        self.focus_manager.sync(&self.renderer.mounted_snapshot());
                        event.node = expansion.collection;
                        event.kind = crate::event::NativeEventKind::Toggle;
                        event.value = Some(expansion.event_value());
                        blueprint = tree_blueprint;
                        route_blueprints = tree_route;
                        let result = self.handle_event_with_route_results(
                            &blueprint,
                            &route_blueprints,
                            event,
                        );
                        if result.is_err() {
                            self.selection_registry = selection_before_navigation;
                            let _ = self.project_mounted_tree_to_host();
                            self.focus_manager.sync(&self.renderer.mounted_snapshot());
                        }
                        return result;
                    }
                } else {
                    let current = event.node;
                    if self.request_collection_keyboard_focus(current, navigation.target)?
                        && navigation.select
                    {
                        event.node = navigation.target;
                        event.kind = crate::event::NativeEventKind::SelectionChange;
                        event.value = None;
                        (blueprint, route_blueprints) =
                            self.native_event_route(event.node).ok_or_else(|| {
                                GuiError::host("collection navigation target unmounted")
                            })?;
                    }
                }
            }
        }
        event = self.infer_container_selection_value(&blueprint, event);
        self.handle_event_with_route_results(&blueprint, &route_blueprints, event)
    }

    fn restore_suppressed_native_selection(
        &mut self,
        mut event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        let selection_before = self.selection_registry.clone();
        let native_update = self.selection_registry.apply_event(&event)?;
        if native_update.as_ref().is_some_and(|update| update.changed) {
            if let Err(error) = self.project_mounted_selection_to_host() {
                self.selection_registry = selection_before;
                let _ = self.project_mounted_selection_to_host();
                return Err(error);
            }
        }

        self.selection_registry = selection_before;
        if native_update.as_ref().is_some_and(|update| update.changed) {
            self.project_mounted_selection_to_host()?;
        }
        if let Some(projection) = self
            .selection_registry
            .projections()
            .into_iter()
            .find(|projection| projection.collection == event.node)
        {
            event.value = Some(projection.event_value());
        }
        Ok(HandledNativeEvent {
            event,
            invocations: Vec::new(),
            invocation: None,
            interaction_changes: Vec::new(),
        })
    }

    fn native_event_route(
        &self,
        node: HostNodeId,
    ) -> Option<(NativeWidgetBlueprint, Vec<RoutedBlueprint>)> {
        let blueprint = self.host.blueprint(node)?.clone();
        let route = self
            .renderer
            .ancestor_ids(node)
            .into_iter()
            .filter_map(|ancestor| {
                self.host
                    .blueprint(ancestor)
                    .cloned()
                    .map(|blueprint| (ancestor, blueprint))
            })
            .collect();
        Some((blueprint, route))
    }

    fn request_collection_keyboard_focus(
        &mut self,
        current: HostNodeId,
        requested: HostNodeId,
    ) -> GuiResult<bool> {
        let current = self
            .focus_owner
            .or_else(|| self.interaction_state.focused_node())
            .unwrap_or(current);
        if self.focus_manager.constrain_focus(current, requested) != Some(requested) {
            return Ok(false);
        }
        let Some(host) = self.host.programmatic_focus_host() else {
            return Ok(false);
        };
        host.request_focus(requested)?;
        self.pending_focus_modality = Some((requested, NativeInputModality::Keyboard));
        Ok(true)
    }

    fn redirect_contained_focus(&mut self, event: &NativeEvent) -> GuiResult<bool> {
        if event.kind != crate::event::NativeEventKind::Focus {
            return Ok(false);
        }
        let Some(current) = self
            .focus_owner
            .or_else(|| self.interaction_state.focused_node())
        else {
            return Ok(false);
        };
        let Some(target) = self.focus_manager.constrain_focus(current, event.node) else {
            return Ok(false);
        };
        if target == event.node {
            return Ok(false);
        }
        let Some(host) = self.host.programmatic_focus_host() else {
            return Ok(false);
        };

        host.request_focus(target)?;
        let modality = match self.interaction_state.input_modality() {
            NativeInputModality::Unknown => NativeInputModality::Virtual,
            modality => modality,
        };
        self.pending_focus_modality = Some((target, modality));
        Ok(true)
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
    route_blueprints: &[RoutedBlueprint],
) -> bool {
    crate::event::non_empty_action(blueprint.events.get("onKeyDown")).is_some()
        || route_blueprints.iter().any(|(_, route_blueprint)| {
            crate::event::non_empty_action(route_blueprint.events.get("onKeyDown")).is_some()
        })
}

fn move_handles_keyboard_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[RoutedBlueprint],
    event: &NativeEvent,
) -> bool {
    if event.kind != crate::event::NativeEventKind::KeyDown
        || !matches!(
            event
                .value
                .as_deref()
                .map(crate::event::native_key_value)
                .as_deref(),
            Some("ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown")
        )
    {
        return false;
    }

    let has_move_handler = |candidate: &NativeWidgetBlueprint| {
        ["onMoveStart", "onMove", "onMoveEnd"]
            .into_iter()
            .any(|name| crate::event::non_empty_action(candidate.events.get(name)).is_some())
    };
    has_move_handler(blueprint)
        || route_blueprints
            .iter()
            .any(|(_, candidate)| has_move_handler(candidate))
}

fn is_invisible_or_inert_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[RoutedBlueprint],
) -> bool {
    is_invisible_or_inert(blueprint)
        || route_blueprints
            .iter()
            .any(|(_, route_blueprint)| is_invisible_or_inert(route_blueprint))
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
    route_blueprints: &[RoutedBlueprint],
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
            .any(|(_, route_blueprint)| route_blueprint.control_state.disabled)
}

fn is_read_only_value_event(
    blueprint: &NativeWidgetBlueprint,
    route_blueprints: &[RoutedBlueprint],
    event: crate::event::NativeEventKind,
) -> bool {
    if !is_value_mutation_event(event) {
        return false;
    }
    if blueprint.control_state.read_only {
        return true;
    }

    route_blueprints.iter().any(|(_, route_blueprint)| {
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

impl<H: NativeHost + AccessibilityTreeHost> GuiRuntime<H> {
    pub fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        let mut tree = self.host.accessibility_tree()?;
        apply_interactions_to_accessibility_tree(
            &mut tree,
            &self.interaction_state,
            &self.interaction_revisions,
            self.render_revision,
        );
        let selection_by_node = self
            .selection_registry
            .projections()
            .into_iter()
            .flat_map(|projection| projection.items)
            .map(|(node, role, selected)| (node, (role, selected)))
            .collect::<BTreeMap<_, _>>();
        apply_mounted_selection_to_accessibility_tree(&mut tree, &selection_by_node);
        Some(tree)
    }

    pub fn accessibility_conformance(&self) -> Option<AccessibilityConformanceReport> {
        self.accessibility_tree()
            .as_ref()
            .map(AccessibilityConformanceReport::validate)
    }
}

fn apply_mounted_selection_to_accessibility_tree(
    node: &mut AccessibilityNode,
    selection_by_node: &BTreeMap<HostNodeId, (NativeRole, bool)>,
) {
    if let Some((role, selected)) = node
        .node
        .and_then(|node| selection_by_node.get(&node).copied())
    {
        node.selected = selected;
        if role == NativeRole::Radio {
            node.checked = Some(selected);
        }
    }
    for child in &mut node.children {
        apply_mounted_selection_to_accessibility_tree(child, selection_by_node);
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

    apply_selected_child_value_to_container(node);
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
    if state.pressed {
        node.state.pressed = Some("true".to_string());
    }
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

fn apply_selected_child_value_to_container(node: &mut AccessibilityNode) {
    if node.value.is_some() || !is_exclusive_child_selection_container(node) {
        return;
    }

    node.value = node
        .children
        .iter()
        .find(|child| {
            is_selectable_child(child.role) && (child.selected || child.checked == Some(true))
        })
        .and_then(selected_accessibility_value);
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
    if !is_exclusive_child_selection_container(node) {
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

fn is_exclusive_child_selection_container(node: &AccessibilityNode) -> bool {
    match node.role {
        AccessibilityRole::ComboBox
        | AccessibilityRole::RadioGroup
        | AccessibilityRole::TabGroup
        | AccessibilityRole::TabList => true,
        AccessibilityRole::ListBox => !node.multiple,
        _ => false,
    }
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
            | crate::native::NativeRole::TreeItem
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
            | crate::native::NativeRole::Tree
            | crate::native::NativeRole::Menu
            | crate::native::NativeRole::RadioGroup
            | crate::native::NativeRole::Tabs
            | crate::native::NativeRole::TabList
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
        self.handle_pending_native_events()
    }

    pub fn handle_pending_native_events(&mut self) -> GuiResult<Vec<ActionInvocation>> {
        let events = self.handle_pending_native_event_results()?;
        Ok(events
            .into_iter()
            .flat_map(|event| event.invocations)
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
#[path = "runtime/multi_action_tests.rs"]
mod multi_action_tests;

#[cfg(test)]
#[path = "runtime/selection_tests.rs"]
mod selection_tests;

#[cfg(test)]
#[path = "runtime/collection_navigation_tests.rs"]
mod collection_navigation_tests;

#[cfg(test)]
#[path = "runtime/i18n_tests.rs"]
mod i18n_tests;

#[cfg(test)]
#[path = "runtime/accessibility_conformance_tests.rs"]
mod accessibility_conformance_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibility::AccessibilityRole;
    use crate::event::NativeEventKind;
    use crate::host::HeadlessHost;
    use crate::html::HtmlDialogProps;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{Gtk4Adapter, PlatformCommand, PlatformPlanningHost, WinUiAdapter};
    use crate::web::WebProps;

    #[test]
    fn runtime_renders_compiled_rsx_to_platform_host() {
        let compiled: CompiledRsxNode = serde_json::from_str(
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
    fn runtime_exports_platform_accessibility_tree_from_compiled_rsx() {
        let compiled: CompiledRsxNode = serde_json::from_str(
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
    fn runtime_dispatches_native_event_to_registered_rsx_action() {
        let compiled: CompiledRsxNode = serde_json::from_str(
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
    fn runtime_routes_long_press_phases_and_tracks_the_transient_state() {
        let element = NativeElement::new("menu", NativeRole::Button).with_props(
            NativeProps::new().web(
                WebProps::new()
                    .event("onLongPressStart", "startMenuPress")
                    .event("onLongPress", "openMenu")
                    .event("onLongPressEnd", "endMenuPress"),
            ),
        );
        let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
        runtime.actions_mut().register("startMenuPress");
        runtime.actions_mut().register("openMenu");
        runtime.actions_mut().register("endMenuPress");
        let node = runtime.render_native(&element).unwrap();
        let touch = crate::input::NativeEventContext::new()
            .modality(crate::input::NativeInputModality::Touch);

        let start = runtime
            .handle_native_event_with_changes(
                NativeEvent::new(node, NativeEventKind::LongPressStart).context(touch),
            )
            .unwrap();
        assert_eq!(start.event.value.as_deref(), Some("true"));
        assert_eq!(start.invocations[0].action, "startMenuPress");
        assert!(runtime.interactions().node(node).unwrap().long_pressed);

        let terminal = runtime
            .handle_native_event_with_changes(
                NativeEvent::new(node, NativeEventKind::LongPress).context(touch),
            )
            .unwrap();
        assert_eq!(terminal.invocations[0].action, "openMenu");
        assert!(runtime.interactions().node(node).unwrap().long_pressed);

        let end = runtime
            .handle_native_event_with_changes(
                NativeEvent::new(node, NativeEventKind::LongPressEnd).context(touch),
            )
            .unwrap();
        assert_eq!(end.event.value.as_deref(), Some("false"));
        assert_eq!(end.invocations[0].action, "endMenuPress");
        assert!(!runtime.interactions().node(node).unwrap().long_pressed);
    }

    #[test]
    fn runtime_routes_button_activation_key_to_primary_action() {
        let element = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().web(WebProps::new().on_press("saveDocument")));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let invocation = runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value(" Return "),
            )
            .unwrap();

        assert_eq!(invocation.action, "saveDocument");
        assert_eq!(invocation.event, crate::event::NativeEventKind::KeyDown);
        assert_eq!(invocation.value.as_deref(), Some("Enter"));
        assert_eq!(runtime.actions().invocations().len(), 1);
        assert!(runtime.interactions().changes().is_empty());
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
    fn runtime_initializes_first_renderable_auto_focus_node() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("hidden", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Hidden")
                        .auto_focus(true)
                        .hidden(true),
                ),
            )
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save").auto_focus(true)),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel").auto_focus(true)),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        let accessibility = runtime.accessibility_tree().unwrap();

        assert!(runtime.interactions().changes().is_empty());
        assert!(runtime.interactions().node(children[1]).unwrap().focused);
        assert_eq!(accessibility.children.len(), 2);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
        assert!(accessibility.children[0].focused);
        assert_eq!(accessibility.children[1].label.as_deref(), Some("Cancel"));
        assert!(!accessibility.children[1].focused);
    }

    #[test]
    fn runtime_auto_focus_skips_hidden_and_inert_ancestor_subtrees() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("hidden-group", NativeRole::View)
                    .with_props(NativeProps::new().hidden(true))
                    .child(
                        NativeElement::new("hidden-save", NativeRole::Button)
                            .with_props(NativeProps::new().label("Hidden save").auto_focus(true)),
                    ),
            )
            .child(
                NativeElement::new("inert-group", NativeRole::View)
                    .with_props(NativeProps::new().inert(true))
                    .child(
                        NativeElement::new("inert-save", NativeRole::Button)
                            .with_props(NativeProps::new().label("Inert save").auto_focus(true)),
                    ),
            )
            .child(
                NativeElement::new("css-hidden-group", NativeRole::View)
                    .with_props(NativeProps::new().web(WebProps::new().style("display", "none")))
                    .child(
                        NativeElement::new("css-hidden-save", NativeRole::Button).with_props(
                            NativeProps::new().label("CSS hidden save").auto_focus(true),
                        ),
                    ),
            )
            .child(
                NativeElement::new("closed-dialog", NativeRole::Dialog)
                    .with_props(
                        NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)),
                    )
                    .child(
                        NativeElement::new("dialog-save", NativeRole::Button)
                            .with_props(NativeProps::new().label("Dialog save").auto_focus(true)),
                    ),
            )
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save").auto_focus(true)),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        let hidden_save = runtime.host().node(children[0]).unwrap().children[0];
        let inert_save = runtime.host().node(children[1]).unwrap().children[0];
        let css_hidden_save = runtime.host().node(children[2]).unwrap().children[0];
        let dialog_save = runtime.host().node(children[3]).unwrap().children[0];
        let save = children[4];
        let accessibility = runtime.accessibility_tree().unwrap();

        assert!(runtime.interactions().node(hidden_save).is_none());
        assert!(runtime.interactions().node(inert_save).is_none());
        assert!(runtime.interactions().node(css_hidden_save).is_none());
        assert!(runtime.interactions().node(dialog_save).is_none());
        assert!(runtime.interactions().node(save).unwrap().focused);
        assert_eq!(accessibility.children.len(), 1);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
        assert!(accessibility.children[0].focused);
    }

    #[test]
    fn runtime_auto_focus_skips_disabled_ancestor_subtrees() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("review-gate", NativeRole::FieldSet)
                    .with_props(NativeProps::new().label("Review gate").disabled(true))
                    .child(
                        NativeElement::new("finish-review", NativeRole::Button).with_props(
                            NativeProps::new().label("Complete review").auto_focus(true),
                        ),
                    ),
            )
            .child(
                NativeElement::new("title", NativeRole::TextField)
                    .with_props(NativeProps::new().label("Task title").auto_focus(true)),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        let finish_review = runtime.host().node(children[0]).unwrap().children[0];
        let title = children[1];
        let accessibility = runtime.accessibility_tree().unwrap();

        assert!(runtime.interactions().node(finish_review).is_none());
        assert!(runtime.interactions().node(title).unwrap().focused);
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
    }

    #[test]
    fn runtime_auto_focus_yields_to_native_focus_history() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save").auto_focus(true)),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel")),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        assert!(runtime.accessibility_tree().unwrap().children[0].focused);

        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                children[1],
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        runtime.render_native(&element).unwrap();
        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].focused);
        assert!(accessibility.children[1].focused);

        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                children[1],
                crate::event::NativeEventKind::Blur,
            ))
            .unwrap();
        runtime.render_native(&element).unwrap();
        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.children[0].focused);
        assert!(!accessibility.children[1].focused);
    }

    #[test]
    fn runtime_auto_focus_yields_after_focused_node_is_removed() {
        let first = NativeElement::new("tools", NativeRole::Toolbar).child(
            NativeElement::new("temporary", NativeRole::TextField)
                .with_props(NativeProps::new().label("Temporary field")),
        );
        let second = NativeElement::new("tools", NativeRole::Toolbar).child(
            NativeElement::new("next", NativeRole::TextField)
                .with_props(NativeProps::new().label("Next field").auto_focus(true)),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&first).unwrap();
        let temporary = runtime.host().node(root_id).unwrap().children[0];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                temporary,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();

        runtime.render_native(&second).unwrap();
        let accessibility = runtime.accessibility_tree().unwrap();

        assert!(runtime.interactions().has_focus_history());
        assert_eq!(accessibility.children.len(), 1);
        assert_eq!(
            accessibility.children[0].label.as_deref(),
            Some("Next field")
        );
        assert!(!accessibility.children[0].focused);
    }

    #[test]
    fn runtime_routes_focus_change_with_boolean_payloads() {
        let element = NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().web(WebProps::new().on_focus_change("setFocus")));
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setFocus");

        let root_id = runtime.render_native(&element).unwrap();
        let focus = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Focus)
                    .value("maybe"),
            )
            .unwrap();
        let blur = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Blur)
                    .value("true"),
            )
            .unwrap();

        assert_eq!(focus.event.value.as_deref(), Some("true"));
        assert_eq!(
            focus
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("true")
        );
        assert_eq!(blur.event.value.as_deref(), Some("false"));
        assert_eq!(
            blur.invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("false")
        );
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

        assert!(error.to_string().contains("no registered RSX action"));
        assert!(runtime.interactions().node(root_id).unwrap().focused);
    }

    #[test]
    fn runtime_rolls_back_interactions_after_unregistered_action() {
        let element = NativeElement::new("name", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Name")
                .value("Ada")
                .web(WebProps::new().on_change("setName")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        assert!(runtime.interactions().node(root_id).unwrap().focused);
        let error = runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("Grace"),
            )
            .unwrap_err();

        assert!(error.to_string().contains("unregistered action setName"));
        let state = runtime.interactions().node(root_id).unwrap();
        assert!(state.focused);
        assert_eq!(state.value.as_deref(), Some("Ada"));
        assert_eq!(runtime.interactions().changes().len(), 1);
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("Ada")
        );
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_treats_empty_action_ids_as_unbound_events() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new()
                .label("Save")
                .web(WebProps::new().on_press("")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let error = runtime
            .dispatch_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap_err();

        assert!(error.to_string().contains("no registered RSX action"));
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_suppresses_disabled_press_actions() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new()
                .label("Save")
                .disabled(true)
                .web(WebProps::new().on_press("saveDocument")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();
        let error = runtime
            .dispatch_native_event(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap_err();

        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert!(runtime.actions().invocations().is_empty());
        assert!(error.to_string().contains("no registered RSX action"));
    }

    #[test]
    fn runtime_suppresses_disabled_toggle_state_changes() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .disabled(true)
                .checked(false)
                .web(WebProps::new().on_change("setNotifications")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let toggle = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Toggle,
            ))
            .unwrap();
        let key = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value(" "),
            )
            .unwrap();

        assert!(toggle.invocation.is_none());
        assert!(toggle.interaction_changes.is_empty());
        assert_eq!(toggle.event.value, None);
        assert!(key.invocation.is_none());
        assert_eq!(key.event.kind, crate::event::NativeEventKind::KeyDown);
        assert!(key.interaction_changes.is_empty());
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_suppresses_disabled_ancestor_user_events() {
        let element = NativeElement::new("review-gate", NativeRole::FieldSet)
            .with_props(NativeProps::new().label("Review gate").disabled(true))
            .child(
                NativeElement::new("finish-review", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Complete review")
                        .web(WebProps::new().on_press("finishReview")),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("finishReview");

        let root_id = runtime.render_native(&element).unwrap();
        let button_id = runtime.host().node(root_id).unwrap().children[0];
        let press = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                button_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();
        let key = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(button_id, crate::event::NativeEventKind::KeyDown)
                    .value("Enter"),
            )
            .unwrap();
        let focus = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                button_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();

        assert!(press.invocation.is_none());
        assert!(press.interaction_changes.is_empty());
        assert!(key.invocation.is_none());
        assert_eq!(key.event.kind, crate::event::NativeEventKind::KeyDown);
        assert!(key.interaction_changes.is_empty());
        assert!(focus.invocation.is_none());
        assert_eq!(focus.interaction_changes.len(), 1);
        assert!(runtime.interactions().node(button_id).unwrap().focused);
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_allows_disabled_focus_state_changes() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new()
                .label("Save")
                .disabled(true)
                .web(WebProps::new().on_focus("inspectSave")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("inspectSave");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();

        assert!(handled.invocation.is_some());
        assert_eq!(handled.interaction_changes.len(), 1);
        assert!(runtime.interactions().node(root_id).unwrap().focused);
        assert_eq!(runtime.actions().invocations().len(), 1);
    }

    #[test]
    fn runtime_suppresses_invisible_focus_and_actions() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new().label("Save").hidden(true).web(
                WebProps::new()
                    .on_focus("inspectSave")
                    .on_press("saveDocument"),
            ),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("inspectSave");
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let focus = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        let press = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert!(focus.invocation.is_none());
        assert!(focus.interaction_changes.is_empty());
        assert!(press.invocation.is_none());
        assert!(press.interaction_changes.is_empty());
        assert!(runtime.accessibility_tree().is_none());
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_suppresses_non_rendered_style_actions() {
        let cases = [
            ("display", "none"),
            ("visibility", "hidden"),
            ("visibility", "collapse"),
            ("contentVisibility", "hidden"),
        ];

        for (property, value) in cases {
            let element = NativeElement::new(format!("{property}-{value}"), NativeRole::Button)
                .with_props(
                    NativeProps::new().label("Save").web(
                        WebProps::new()
                            .style(property, value)
                            .on_press("saveDocument"),
                    ),
                );
            let host = PlatformPlanningHost::new(Gtk4Adapter);
            let mut runtime = GuiRuntime::new(host);
            runtime.actions_mut().register("saveDocument");

            let root_id = runtime.render_native(&element).unwrap();
            let handled = runtime
                .handle_native_event_with_changes(crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::Press,
                ))
                .unwrap();

            assert!(
                handled.invocation.is_none(),
                "{property}: {value} should suppress invocation"
            );
            assert!(
                handled.interaction_changes.is_empty(),
                "{property}: {value} should suppress interaction changes"
            );
            assert!(
                runtime.accessibility_tree().is_none(),
                "{property}: {value} should suppress accessibility projection"
            );
            assert!(
                runtime.actions().invocations().is_empty(),
                "{property}: {value} should suppress action dispatch"
            );
        }
    }

    #[test]
    fn runtime_suppresses_closed_dialog_subtree_actions() {
        let element = NativeElement::new("dialog", NativeRole::Dialog)
            .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)))
            .child(
                NativeElement::new("save", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Save")
                        .web(WebProps::new().on_press("saveDocument")),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let child = runtime.host().node(root_id).unwrap().children[0];
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                child,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert!(runtime.accessibility_tree().is_none());
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_accessibility_tree_prunes_invisible_inert_and_aria_hidden_subtrees() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save")),
            )
            .child(
                NativeElement::new("archive", NativeRole::Button)
                    .with_props(NativeProps::new().label("Archive").hidden(true)),
            )
            .child(
                NativeElement::new("delete", NativeRole::Button)
                    .with_props(NativeProps::new().label("Delete").inert(true)),
            )
            .child(
                NativeElement::new("preview", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Preview")
                        .accessibility_hidden(Some(true)),
                ),
            )
            .child(
                NativeElement::new("details", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Details")
                        .web(WebProps::new().style("display", "none")),
                ),
            )
            .child(
                NativeElement::new("filters", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Filters")
                        .web(WebProps::new().style("visibility", "hidden")),
                ),
            )
            .child(
                NativeElement::new("summary", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Summary")
                        .web(WebProps::new().style("contentVisibility", "hidden")),
                ),
            )
            .child(
                NativeElement::new("activity", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Activity")
                        .web(WebProps::new().style("interactivity", "inert")),
                ),
            )
            .child(
                NativeElement::new("dialog", NativeRole::Dialog)
                    .with_props(
                        NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)),
                    )
                    .child(
                        NativeElement::new("close", NativeRole::Button)
                            .with_props(NativeProps::new().label("Close")),
                    ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        runtime.render_native(&element).unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.children.len(), 1);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
    }

    #[test]
    fn runtime_routes_aria_hidden_actions() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new()
                .label("Save")
                .accessibility_hidden(Some(true))
                .web(WebProps::new().on_press("saveDocument")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("saveDocument")
        );
        assert!(runtime.accessibility_tree().is_none());
        assert_eq!(runtime.actions().invocations().len(), 1);
    }

    #[test]
    fn runtime_suppresses_inert_subtree_actions() {
        let cases = [
            (
                "html inert",
                "tools-html-inert",
                NativeProps::new().inert(true),
            ),
            (
                "css interactivity inert",
                "tools-css-interactivity-inert",
                NativeProps::new().web(WebProps::new().style("interactivity", "inert")),
            ),
        ];

        for (name, key, props) in cases {
            let element = NativeElement::new(key, NativeRole::Toolbar)
                .with_props(props)
                .child(
                    NativeElement::new("save", NativeRole::Button).with_props(
                        NativeProps::new()
                            .label("Save")
                            .web(WebProps::new().on_press("saveDocument")),
                    ),
                );
            let host = PlatformPlanningHost::new(Gtk4Adapter);
            let mut runtime = GuiRuntime::new(host);
            runtime.actions_mut().register("saveDocument");

            let root_id = runtime.render_native(&element).unwrap();
            let child = runtime.host().node(root_id).unwrap().children[0];
            let handled = runtime
                .handle_native_event_with_changes(crate::event::NativeEvent::new(
                    child,
                    crate::event::NativeEventKind::Press,
                ))
                .unwrap();

            assert!(handled.invocation.is_none(), "{name}");
            assert!(handled.interaction_changes.is_empty(), "{name}");
            assert!(runtime.accessibility_tree().is_none(), "{name}");
            assert!(runtime.actions().invocations().is_empty(), "{name}");
        }
    }

    #[test]
    fn runtime_suppresses_read_only_change_actions() {
        let element = NativeElement::new("name", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Name")
                .value("Ada")
                .read_only(true)
                .web(WebProps::new().on_change("setName")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setName");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("Grace"),
            )
            .unwrap();

        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("Ada")
        );
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_suppresses_read_only_selection_actions() {
        let element = NativeElement::new("theme", NativeRole::Select)
            .with_props(
                NativeProps::new()
                    .label("Theme")
                    .read_only(true)
                    .web(WebProps::new().on_selection_change("setTheme")),
            )
            .child(
                NativeElement::new("compact", NativeRole::ListBoxItem)
                    .with_props(NativeProps::new().label("Compact").value("compact")),
            )
            .child(
                NativeElement::new("comfortable", NativeRole::ListBoxItem).with_props(
                    NativeProps::new()
                        .label("Comfortable")
                        .value("comfortable")
                        .selected(true),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&element).unwrap();
        let inferred = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();
        let explicit = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value("compact"),
            )
            .unwrap();

        assert_eq!(inferred.event.value.as_deref(), Some("comfortable"));
        assert!(inferred.invocation.is_none());
        assert!(inferred.interaction_changes.is_empty());
        assert_eq!(explicit.event.value.as_deref(), Some("compact"));
        assert!(explicit.invocation.is_none());
        assert!(explicit.interaction_changes.is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("comfortable")
        );
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_suppresses_read_only_ancestor_selection_value_events() {
        let element = NativeElement::new("theme", NativeRole::RadioGroup)
            .with_props(
                NativeProps::new()
                    .label("Theme")
                    .read_only(true)
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

        let root_id = runtime.render_native(&element).unwrap();
        let dark_id = runtime.host().node(root_id).unwrap().children[1];
        let selection = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                dark_id,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();
        let toggle = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                dark_id,
                crate::event::NativeEventKind::Toggle,
            ))
            .unwrap();

        assert_eq!(selection.event.value.as_deref(), Some("dark"));
        assert!(selection.invocation.is_none());
        assert!(selection.interaction_changes.is_empty());
        assert_eq!(toggle.event.value.as_deref(), None);
        assert!(toggle.invocation.is_none());
        assert!(toggle.interaction_changes.is_empty());
        let accessibility = runtime.accessibility_tree().unwrap();
        assert_eq!(accessibility.value.as_deref(), Some("light"));
        assert!(accessibility.children[0].selected);
        assert_eq!(accessibility.children[0].checked, Some(true));
        assert!(!accessibility.children[1].selected);
        assert_eq!(accessibility.children[1].checked, Some(false));
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_clamps_text_change_values_to_max_length() {
        let element = NativeElement::new("name", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Name")
                .value("Ada")
                .max_length(Some(3))
                .web(WebProps::new().on_change("setName")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setName");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("aé日b"),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("aé日"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("aé日")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("aé日")
        );
        assert_eq!(
            runtime.actions().invocations()[0].value.as_deref(),
            Some("aé日")
        );
    }

    #[test]
    fn runtime_clamps_initial_text_value_to_max_length_before_rendering() {
        let element = NativeElement::new("name", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Name")
                .value("aé日b")
                .max_length(Some(3)),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.max_length, Some(3));
        assert_eq!(blueprint.value.as_deref(), Some("aé日"));
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("aé日")
        );

        let updated = NativeElement::new("name", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Name")
                .value("Ada Lovelace")
                .max_length(Some(3)),
        );
        runtime.render_native(&updated).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.value.as_deref(), Some("Ada"));
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("Ada")
        );
    }

    #[test]
    fn runtime_clamps_slider_change_values_to_range_bounds() {
        let element = NativeElement::new("estimate", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Estimate")
                .range(Some(1.0), Some(12.0), Some(6.0))
                .web(WebProps::new().on_change("setEstimate")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEstimate");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value(" 99 "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("12"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("12")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("12")
        );
        assert_eq!(
            runtime.actions().invocations()[0].value.as_deref(),
            Some("12")
        );

        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value(" 0 "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("1"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("1")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("1")
        );
        assert_eq!(
            runtime.actions().invocations()[1].value.as_deref(),
            Some("1")
        );
    }

    #[test]
    fn runtime_clamps_number_input_change_values_to_range_bounds() {
        let element = NativeElement::new("estimate", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Estimate")
                .input_type("number")
                .range(Some(1.0), Some(12.0), Some(6.0))
                .web(WebProps::new().on_change("setEstimate")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEstimate");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value(" 99 "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("12"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("12")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("12")
        );
        assert_eq!(
            runtime.actions().invocations()[0].value.as_deref(),
            Some("12")
        );

        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value(" 0 "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("1"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("1")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("1")
        );
        assert_eq!(
            runtime.actions().invocations()[1].value.as_deref(),
            Some("1")
        );
    }

    #[test]
    fn runtime_snaps_ranged_change_values_to_step() {
        let element = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .range(Some(0.0), Some(100.0), Some(50.0))
                .step(Some(5.0))
                .web(WebProps::new().on_change("setVolume")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setVolume");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("43"),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("45"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("45")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("45")
        );
        assert_eq!(
            runtime.actions().invocations()[0].value.as_deref(),
            Some("45")
        );

        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("42"),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("40"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("40")
        );
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("40")
        );
        assert_eq!(
            runtime.actions().invocations()[1].value.as_deref(),
            Some("40")
        );
    }

    #[test]
    fn runtime_suppresses_invalid_numeric_change_values() {
        let slider = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .range(Some(0.0), Some(100.0), Some(6.0))
                .web(WebProps::new().on_change("setVolume")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setVolume");

        let root_id = runtime.render_native(&slider).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("not-a-number"),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("not-a-number"));
        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert!(runtime.actions().invocations().is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("6")
        );

        let number_input = NativeElement::new("estimate", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Estimate")
                .input_type("number")
                .range(Some(1.0), Some(12.0), Some(6.0))
                .web(WebProps::new().on_change("setEstimate")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEstimate");

        let root_id = runtime.render_native(&number_input).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value(" "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some(" "));
        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert!(runtime.actions().invocations().is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("6")
        );
    }

    #[test]
    fn runtime_normalizes_initial_ranged_values_before_rendering() {
        let element = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .range(Some(0.0), Some(100.0), Some(43.0))
                .step(Some(5.0)),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.current, Some(45.0));
        assert_eq!(blueprint.value.as_deref(), Some("45"));
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("45")
        );

        let updated = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .range(Some(0.0), Some(100.0), Some(17.0))
                .step(Some(5.0)),
        );
        runtime.render_native(&updated).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.current, Some(15.0));
        assert_eq!(blueprint.value.as_deref(), Some("15"));
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("15")
        );
    }

    #[test]
    fn runtime_normalizes_initial_number_input_values_before_rendering() {
        let element = NativeElement::new("estimate", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Estimate")
                .input_type("number")
                .range(Some(1.0), Some(12.0), Some(99.0)),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&element).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.current, Some(12.0));
        assert_eq!(blueprint.value.as_deref(), Some("12"));
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("12")
        );
    }

    #[test]
    fn runtime_omits_invalid_initial_numeric_values_before_rendering() {
        let slider = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .value("not-a-number")
                .range(Some(0.0), Some(100.0), None),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&slider).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.current, None);
        assert_eq!(blueprint.value, None);
        assert_eq!(runtime.accessibility_tree().unwrap().value, None);

        let number_input = NativeElement::new("estimate", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Estimate")
                .value(" ")
                .input_type("number")
                .range(Some(1.0), Some(12.0), None),
        );
        let root_id = runtime.render_native(&number_input).unwrap();
        let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

        assert_eq!(blueprint.control_state.current, None);
        assert_eq!(blueprint.value, None);
        assert_eq!(runtime.accessibility_tree().unwrap().value, None);
    }

    #[test]
    fn runtime_event_number_parser_trims_values_without_coercing_empty_input() {
        assert_eq!(parse_event_number(" 42 "), Some(42.0));
        assert_eq!(parse_event_number("\t0.5\n"), Some(0.5));
        assert_eq!(parse_event_number(" "), None);
        assert_eq!(parse_event_number("not-a-number"), None);
    }

    #[test]
    fn runtime_event_bool_parser_canonicalizes_common_native_payloads() {
        assert_eq!(parse_event_bool(Some(" true ")), Some(true));
        assert_eq!(parse_event_bool(Some("ON")), Some(true));
        assert_eq!(parse_event_bool(Some("1")), Some(true));
        assert_eq!(parse_event_bool(Some(" false ")), Some(false));
        assert_eq!(parse_event_bool(Some("OFF")), Some(false));
        assert_eq!(parse_event_bool(Some("0")), Some(false));
        assert_eq!(parse_event_bool(Some("maybe")), None);
        assert_eq!(parse_event_bool(None), None);
    }

    #[test]
    fn runtime_suppresses_read_only_keyboard_toggle_normalization() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .read_only(true)
                .checked(false)
                .web(WebProps::new().on_change("setNotifications")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value(" "),
            )
            .unwrap();

        assert!(handled.invocation.is_none());
        assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
        assert!(handled.interaction_changes.is_empty());
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_updates_interaction_state_before_dispatching_action() {
        let compiled: CompiledRsxNode = serde_json::from_str(
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
    fn runtime_routes_expanded_toggle_with_current_boolean_payload() {
        let element = NativeElement::new("details", NativeRole::Disclosure).with_props(
            NativeProps::new()
                .label("Details")
                .expanded(false)
                .web(WebProps::new().on_expanded_change("setOpen")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setOpen");

        let root_id = runtime.render_native(&element).unwrap();
        let first = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                    .value("on"),
            )
            .unwrap();
        let second = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                    .value("not-a-bool"),
            )
            .unwrap();

        assert_eq!(first.event.value.as_deref(), Some("true"));
        assert_eq!(
            first
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("true")
        );
        assert_eq!(first.interaction_changes[0].after.expanded, Some(true));
        assert_eq!(second.event.value.as_deref(), Some("false"));
        assert_eq!(
            second
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("false")
        );
        assert_eq!(second.interaction_changes[0].after.expanded, Some(false));
        assert_eq!(runtime.accessibility_tree().unwrap().expanded, Some(false));
    }

    #[test]
    fn runtime_routes_checked_toggle_with_current_boolean_payload() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .checked(false)
                .web(WebProps::new().on_change("setNotifications")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let first = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                    .value("1"),
            )
            .unwrap();
        let second = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Toggle)
                    .value("not-a-bool"),
            )
            .unwrap();

        assert_eq!(first.event.value.as_deref(), Some("true"));
        assert_eq!(
            first
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("true")
        );
        assert_eq!(first.interaction_changes[0].after.checked, Some(true));
        assert_eq!(second.event.value.as_deref(), Some("false"));
        assert_eq!(
            second
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("false")
        );
        assert_eq!(second.interaction_changes[0].after.checked, Some(false));
    }

    #[test]
    fn runtime_routes_checked_change_with_current_boolean_payload() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .checked(false)
                .web(WebProps::new().on_change("setNotifications")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let first = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("on"),
            )
            .unwrap();
        let second = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("not-a-bool"),
            )
            .unwrap();

        assert_eq!(first.event.value.as_deref(), Some("true"));
        assert_eq!(
            first
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("true")
        );
        assert_eq!(first.interaction_changes[0].after.checked, Some(true));
        assert_eq!(second.event.value.as_deref(), Some("false"));
        assert_eq!(
            second
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("false")
        );
        assert_eq!(second.interaction_changes[0].after.checked, Some(false));
        assert_eq!(
            runtime.actions().invocations()[1].value.as_deref(),
            Some("false")
        );
    }

    #[test]
    fn runtime_routes_switch_space_key_to_toggle_action() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .checked(false)
                .web(WebProps::new().on_change("setNotifications")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value(" "),
            )
            .unwrap();

        assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
        assert_eq!(handled.event.value.as_deref(), Some("true"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.event),
            Some(crate::event::NativeEventKind::Toggle)
        );
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("true")
        );
        assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
    }

    #[test]
    fn runtime_explicit_key_down_prevents_keyboard_toggle_normalization() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .checked(false)
                .web(
                    WebProps::new()
                        .on_change("setNotifications")
                        .on_key_down("handleKeyDown"),
                ),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");
        runtime.actions_mut().register("handleKeyDown");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value("space"),
            )
            .unwrap();

        assert_eq!(handled.event.kind, crate::event::NativeEventKind::KeyDown);
        assert_eq!(handled.event.value.as_deref(), Some(" "));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("handleKeyDown")
        );
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some(" ")
        );
        assert!(handled.interaction_changes.is_empty());
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
    }

    #[test]
    fn runtime_empty_key_down_handler_does_not_block_keyboard_toggle_normalization() {
        let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
            NativeProps::new()
                .label("Notifications")
                .checked(false)
                .web(
                    WebProps::new()
                        .on_change("setNotifications")
                        .on_key_down(""),
                ),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value(" "),
            )
            .unwrap();

        assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("setNotifications")
        );
        assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
        assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(true));
    }

    #[test]
    fn runtime_ancestor_key_down_prevents_keyboard_toggle_normalization() {
        let element = NativeElement::new("row", NativeRole::View)
            .with_props(NativeProps::new().web(WebProps::new().on_key_down("handleRowKey")))
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
        runtime.actions_mut().register("handleRowKey");
        runtime.actions_mut().register("setNotifications");

        let root_id = runtime.render_native(&element).unwrap();
        let switch = runtime.host().node(root_id).unwrap().children[0];
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(switch, crate::event::NativeEventKind::KeyDown)
                    .value(" "),
            )
            .unwrap();

        assert_eq!(handled.event.kind, crate::event::NativeEventKind::KeyDown);
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("handleRowKey")
        );
        assert!(handled.interaction_changes.is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().children[0].checked,
            Some(false)
        );
    }

    #[test]
    fn runtime_routes_radio_space_key_to_selection_action() {
        let element = NativeElement::new("dark", NativeRole::Radio).with_props(
            NativeProps::new()
                .label("Dark")
                .value("dark")
                .web(WebProps::new().on_change("setTheme")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                    .value("Space"),
            )
            .unwrap();

        assert_eq!(
            handled.event.kind,
            crate::event::NativeEventKind::SelectionChange
        );
        assert_eq!(handled.event.value.as_deref(), Some("dark"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .map(|invocation| invocation.event),
            Some(crate::event::NativeEventKind::SelectionChange)
        );
        assert_eq!(handled.interaction_changes[0].after.checked, Some(true));
        assert!(handled.interaction_changes[0].after.selected);
    }

    #[test]
    fn runtime_suppresses_redundant_container_selection_value_from_selected_child() {
        let element = NativeElement::new("theme", NativeRole::Select)
            .with_props(
                NativeProps::new()
                    .label("Theme")
                    .web(WebProps::new().on_selection_change("setTheme")),
            )
            .child(
                NativeElement::new("compact", NativeRole::ListBoxItem)
                    .with_props(NativeProps::new().label("Compact").value("compact")),
            )
            .child(
                NativeElement::new("comfortable", NativeRole::ListBoxItem).with_props(
                    NativeProps::new()
                        .label("Comfortable")
                        .value("comfortable")
                        .selected(true),
                ),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value(" "),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("comfortable"));
        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert_eq!(
            runtime.accessibility_tree().unwrap().value.as_deref(),
            Some("comfortable")
        );
        assert!(runtime.actions().invocations().is_empty());
    }

    #[test]
    fn runtime_infers_selectable_node_value_from_empty_selection_payload() {
        let element = NativeElement::new("compact", NativeRole::ListBoxItem).with_props(
            NativeProps::new()
                .label("Compact")
                .value("compact")
                .web(WebProps::new().on_selection_change("setTheme")),
        );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setTheme");

        let root_id = runtime.render_native(&element).unwrap();
        let handled = runtime
            .handle_native_event_with_changes(
                crate::event::NativeEvent::new(
                    root_id,
                    crate::event::NativeEventKind::SelectionChange,
                )
                .value(""),
            )
            .unwrap();

        assert_eq!(handled.event.value.as_deref(), Some("compact"));
        assert_eq!(
            handled
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some("compact")
        );
        assert!(handled.interaction_changes[0].after.selected);
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
    fn runtime_ignores_native_events_for_unmounted_nodes() {
        let first = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("save", NativeRole::Button).with_props(
                    NativeProps::new()
                        .label("Save")
                        .web(WebProps::new().on_press("saveDocument")),
                ),
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
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_native(&first).unwrap();
        let save = runtime.host().node(root_id).unwrap().children[0];
        runtime.render_native(&second).unwrap();

        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                save,
                crate::event::NativeEventKind::Press,
            ))
            .unwrap();

        assert_eq!(handled.event.node, save);
        assert!(handled.invocation.is_none());
        assert!(handled.interaction_changes.is_empty());
        assert!(runtime.actions().invocations().is_empty());
        assert!(runtime.host().node(save).is_none());
    }

    #[test]
    fn runtime_prunes_interaction_state_for_non_interactive_rerendered_subtrees() {
        let first = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("primary", NativeRole::View).child(
                    NativeElement::new("save", NativeRole::Button)
                        .with_props(NativeProps::new().label("Save")),
                ),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel")),
            );
        let second = NativeElement::new("tools", NativeRole::Toolbar)
            .child(
                NativeElement::new("primary", NativeRole::View)
                    .with_props(NativeProps::new().hidden(true))
                    .child(
                        NativeElement::new("save", NativeRole::Button)
                            .with_props(NativeProps::new().label("Save")),
                    ),
            )
            .child(
                NativeElement::new("cancel", NativeRole::Button)
                    .with_props(NativeProps::new().label("Cancel").auto_focus(true)),
            );
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_native(&first).unwrap();
        let children = runtime.host().node(root_id).unwrap().children.clone();
        let save = runtime.host().node(children[0]).unwrap().children[0];
        let cancel = children[1];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                save,
                crate::event::NativeEventKind::Focus,
            ))
            .unwrap();
        assert!(runtime.interactions().node(save).unwrap().focused);

        runtime.render_native(&second).unwrap();

        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(runtime.interactions().node(save).is_none());
        assert!(runtime.interactions().node(cancel).is_none());
        assert!(runtime.interactions().has_focus_history());
        assert!(runtime.interactions().changes().is_empty());
        assert_eq!(accessibility.children.len(), 1);
        assert_eq!(accessibility.children[0].label.as_deref(), Some("Cancel"));
        assert!(!accessibility.children[0].focused);
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
    fn runtime_accessibility_tree_projects_single_listbox_child_selection_to_siblings() {
        let list_box = NativeElement::new("project", NativeRole::ListBox)
            .with_props(NativeProps::new().label("Project"))
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

        let root_id = runtime.render_native(&list_box).unwrap();
        let other = runtime.host().node(root_id).unwrap().children[1];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                other,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();
        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(!accessibility.multiple);
        assert_eq!(accessibility.value.as_deref(), Some("other"));
        assert!(!accessibility.children[0].selected);
        assert!(accessibility.children[1].selected);
    }

    #[test]
    fn runtime_accessibility_tree_preserves_multiple_listbox_child_selections() {
        let list_box = NativeElement::new("project", NativeRole::ListBox)
            .with_props(NativeProps::new().label("Project").multiple(true))
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

        let root_id = runtime.render_native(&list_box).unwrap();
        let other = runtime.host().node(root_id).unwrap().children[1];
        runtime
            .handle_native_event(crate::event::NativeEvent::new(
                other,
                crate::event::NativeEventKind::SelectionChange,
            ))
            .unwrap();
        let accessibility = runtime.accessibility_tree().unwrap();
        assert!(accessibility.multiple);
        assert!(accessibility.children[0].selected);
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
        assert_eq!(handled.interaction_changes.len(), 3);
        assert_eq!(handled.interaction_changes[0].node, radio_id);
        assert_eq!(
            handled.interaction_changes[0].after.value.as_deref(),
            Some("dark")
        );
        assert!(handled.interaction_changes.iter().any(|change| {
            change.node == root_id && change.after.value.as_deref() == Some("dark")
        }));
        assert!(handled.interaction_changes.iter().any(|change| {
            change.node != radio_id
                && change.node != root_id
                && !change.after.selected
                && change.after.checked == Some(false)
        }));

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
    fn runtime_renders_compiled_rsx_to_native_command_stream() {
        let compiled: CompiledRsxNode = serde_json::from_str(
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
