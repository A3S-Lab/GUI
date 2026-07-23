use std::collections::{BTreeMap, BTreeSet};

use crate::backend::NativeEventHost;
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::compiler::{CompiledRsxNode, RsxCompilerBridge};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, ActionRegistry, EventRouter, NativeEvent};
use crate::focus::{FocusManager, FocusNavigationMode};
use crate::host::{HostNodeId, NativeHost, ProgrammaticFocusHost};
use crate::i18n::{
    cached_number_parser, I18nManager, NumberFormatOptions, NumberFormatStyle,
    DEFAULT_FORMATTING_LOCALE,
};
use crate::input::NativeInputModality;
use crate::interaction::{InteractionChange, InteractionState};
use crate::native::{
    format_normalized_number, is_number_input_type, normalize_range_value,
    number_field_wheel_step_direction, step_range_value, truncate_to_max_length, NativeElement,
    NativeProps, RangeStepDirection, ValueSensitivity, NUMBER_FIELD_INPUT_METADATA_KEY,
    NUMBER_FIELD_STEP_METADATA_KEY, NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY,
};
use crate::overlay::{MountedOverlayRegistry, OverlayEventDisposition};
use crate::overlay_position::mounted_overlay_positions;
use crate::platform::{BlueprintHost, NativeWidgetBlueprint};
use crate::renderer::Renderer;
use crate::selection::{
    apply_item_selection_props, apply_item_tree_props, validate_native_collection_keys,
    CollectionLayoutSnapshot, MountedSelectionRegistry, MountedSelectionUpdate,
};
use crate::semantic_ui::{SemanticElement, SemanticMapper};
use crate::style::PortableStyle;
use serde::{Deserialize, Serialize};

mod accessibility;
mod interaction_style;
mod live_region;

type RoutedBlueprint = (HostNodeId, NativeWidgetBlueprint);

#[derive(Debug, Clone)]
struct FocusWithinTransition {
    node: HostNodeId,
    blueprint: NativeWidgetBlueprint,
    focused: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
    #[serde(default)]
    pub value_sensitivity: ValueSensitivity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HandledNativeEventWire {
    event: NativeEvent,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    invocations: Vec<ActionInvocation>,
    invocation: Option<ActionInvocation>,
    interaction_changes: Vec<InteractionChange>,
}

impl Serialize for HandledNativeEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut event = self.event.clone();
        let mut invocations = self.invocations.clone();
        let mut invocation = self.invocation.clone();
        let mut interaction_changes = self.interaction_changes.clone();
        redact_event_output(
            &mut event,
            invocation.as_mut(),
            &mut interaction_changes,
            self.value_sensitivity,
        );
        if self.value_sensitivity.is_sensitive() {
            for invocation in &mut invocations {
                invocation.value = None;
            }
        }
        HandledNativeEventWire {
            event,
            invocations,
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
    focus_manager: FocusManager,
    overlay_registry: MountedOverlayRegistry,
    i18n_manager: I18nManager,
    selection_registry: MountedSelectionRegistry,
    interaction_state: InteractionState,
    projected_interaction_styles: BTreeMap<HostNodeId, PortableStyle>,
    pending_live_region_updates: BTreeMap<HostNodeId, live_region::PendingLiveRegionUpdate>,
    focus_owner: Option<HostNodeId>,
    pending_focus_modality: Option<(HostNodeId, NativeInputModality)>,
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
            focus_manager: FocusManager::new(),
            overlay_registry: MountedOverlayRegistry::new(),
            i18n_manager: I18nManager::new(),
            selection_registry: MountedSelectionRegistry::new(),
            interaction_state: InteractionState::new(),
            projected_interaction_styles: BTreeMap::new(),
            pending_live_region_updates: BTreeMap::new(),
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
        let previous_mounted_snapshot = self.renderer.mounted_snapshot();
        let previous_mounted_props = previous_mounted_snapshot
            .iter()
            .map(|mounted| (mounted.node, mounted.props.clone()))
            .collect::<BTreeMap<_, _>>();
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
        self.overlay_registry.sync(&snapshot);
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
        self.project_mounted_overlays_to_host()?;
        self.project_mounted_overlay_positions_to_host()?;
        self.focus_manager.sync(&self.renderer.mounted_snapshot());
        let overlay_auto_focus = self
            .overlay_registry
            .take_opened_auto_focus_overlay()
            .and_then(|scope| {
                self.focus_manager
                    .first(Some(scope), FocusNavigationMode::Tabbable)
                    .or_else(|| {
                        self.focus_manager
                            .first(Some(scope), FocusNavigationMode::Focusable)
                    })
            });
        let tree_focus_fallback = focus_before_render
            .and_then(|focused| self.selection_registry.tree_focus_fallback(focused));
        self.prune_unmounted_interactions();
        self.sync_mounted_selection_interactions();
        let auto_focus = self.auto_focus_target();
        if let Some(target) = overlay_auto_focus
            .or(tree_focus_fallback)
            .or(restore_focus)
            .or(auto_focus)
        {
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
        self.invalidate_interaction_style_projections(&previous_mounted_props);
        self.project_all_interaction_styles()?;
        let current_mounted_snapshot = self.renderer.mounted_snapshot();
        let mut announcements = Vec::new();
        if let Some(announcement) =
            self.number_field_value_announcement(&previous_mounted_props, &current_mounted_snapshot)
        {
            announcements.push(announcement);
        }
        for announcement in
            self.live_region_announcements(&previous_mounted_snapshot, &current_mounted_snapshot)
        {
            if !announcements.contains(&announcement) {
                announcements.push(announcement);
            }
        }
        if !announcements.is_empty() {
            if let Some(host) = self.host.accessibility_announcement_host() {
                for announcement in announcements {
                    host.announce(announcement)?;
                }
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

    pub fn interactions_mut(&mut self) -> &mut InteractionState {
        &mut self.interaction_state
    }

    pub fn focus_manager(&self) -> &FocusManager {
        &self.focus_manager
    }

    pub fn overlays(&self) -> &MountedOverlayRegistry {
        &self.overlay_registry
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

    /// Installs explicit collection geometry until the next render.
    ///
    /// Native command hosts normally measure this automatically when handling
    /// PageUp or PageDown. This method supports custom and headless hosts.
    pub fn set_collection_layout(
        &mut self,
        collection: HostNodeId,
        layout: CollectionLayoutSnapshot,
    ) -> GuiResult<()> {
        self.selection_registry
            .set_collection_layout(collection, layout)
    }

    pub fn clear_collection_layout(&mut self, collection: HostNodeId) -> bool {
        self.selection_registry.clear_collection_layout(collection)
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
            .handle_event_with_route_results(blueprint, &[], event, false)?
            .invocations)
    }

    fn handle_event_with_routes(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        event: NativeEvent,
    ) -> GuiResult<Option<ActionInvocation>> {
        Ok(self
            .handle_event_with_route_results(blueprint, route_blueprints, event, false)?
            .invocation)
    }

    fn handle_event_with_route_results(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        event: NativeEvent,
        dispatch_unchanged_selection: bool,
    ) -> GuiResult<HandledNativeEvent> {
        self.handle_event_with_route_results_and_extra_invocations(
            blueprint,
            route_blueprints,
            event,
            dispatch_unchanged_selection,
            Vec::new(),
        )
    }

    fn handle_event_with_route_results_and_extra_invocations(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        mut event: NativeEvent,
        dispatch_unchanged_selection: bool,
        extra_invocations: Vec<ActionInvocation>,
    ) -> GuiResult<HandledNativeEvent> {
        let value_sensitivity = effective_blueprint_value_sensitivity(blueprint);
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
                value_sensitivity,
            });
        }
        if is_disabled_user_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
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
                value_sensitivity,
            });
        }
        let mut event = self.normalize_event_value(blueprint, route_blueprints, event);
        if is_invalid_numeric_change_value(blueprint, &event) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        if is_read_only_value_event(blueprint, route_blueprints, event.kind) {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
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
            if !update.changed && !dispatch_unchanged_selection {
                return Ok(HandledNativeEvent {
                    event,
                    invocations: Vec::new(),
                    invocation: None,
                    interaction_changes: Vec::new(),
                    value_sensitivity,
                });
            }
            if update.changed {
                if let Err(error) = self.project_mounted_selection_to_host() {
                    if let Some(selection_snapshot) = selection_snapshot {
                        self.selection_registry = selection_snapshot;
                    }
                    return Err(error);
                }
            }
        }
        let focus_within_transitions =
            self.focus_within_transitions(blueprint, route_blueprints, &event);
        let mut invocations = self.route_event(
            blueprint,
            route_blueprints,
            &focus_within_transitions,
            &event,
        );
        invocations.extend(extra_invocations);
        let interaction_snapshot = (
            self.interaction_state.clone(),
            self.interaction_revisions.clone(),
            selection_snapshot,
            self.focus_owner,
        );
        let interaction_blueprint = (blueprint.value_sensitivity != value_sensitivity).then(|| {
            let mut blueprint = blueprint.clone();
            blueprint.value_sensitivity = value_sensitivity;
            blueprint
        });
        let mut interaction_changes = self
            .interaction_state
            .apply_event_with_changes(interaction_blueprint.as_ref().unwrap_or(blueprint), &event);
        for transition in &focus_within_transitions {
            if let Some(change) = self.interaction_state.set_focus_within(
                transition.node,
                &transition.blueprint,
                transition.focused,
            ) {
                interaction_changes.push(change);
            }
        }
        if let Some(update) = &selection_update {
            self.apply_mounted_selection_update(update);
        }
        let mut style_nodes = interaction_changes
            .iter()
            .map(|change| change.node)
            .chain(selection_update.iter().flat_map(|update| {
                std::iter::once(update.collection)
                    .chain(update.items.iter().map(|(node, _, _)| *node))
            }))
            .collect::<BTreeSet<_>>();
        self.include_focus_within_style_ancestors(&mut style_nodes);
        self.record_interaction_revisions(&interaction_changes);
        if let Err(error) = self.project_interaction_style_nodes(style_nodes.iter().copied()) {
            self.interaction_state = interaction_snapshot.0.clone();
            self.interaction_revisions = interaction_snapshot.1.clone();
            self.focus_owner = interaction_snapshot.3;
            let mut rollback_errors = Vec::new();
            if let Some(selection_snapshot) = &interaction_snapshot.2 {
                self.selection_registry = selection_snapshot.clone();
                if let Err(rollback_error) = self.project_mounted_selection_to_host() {
                    rollback_errors.push(rollback_error);
                }
            }
            if let Err(rollback_error) =
                self.project_interaction_style_nodes(style_nodes.iter().copied())
            {
                rollback_errors.push(rollback_error);
            }
            return Err(with_runtime_rollback_context(error, rollback_errors));
        }
        match event.kind {
            crate::event::NativeEventKind::Focus => self.focus_owner = Some(event.node),
            _ => {}
        }
        if let Err(error) = self
            .action_registry
            .invoke_all_with_sensitivity(&invocations, value_sensitivity)
        {
            self.interaction_state = interaction_snapshot.0;
            self.interaction_revisions = interaction_snapshot.1;
            self.focus_owner = interaction_snapshot.3;
            let mut rollback_errors = Vec::new();
            if let Some(selection_snapshot) = interaction_snapshot.2 {
                self.selection_registry = selection_snapshot;
                if let Err(rollback_error) = self.project_mounted_selection_to_host() {
                    rollback_errors.push(rollback_error);
                }
            }
            if let Err(rollback_error) = self.project_interaction_style_nodes(style_nodes) {
                rollback_errors.push(rollback_error);
            }
            return Err(with_runtime_rollback_context(error, rollback_errors));
        }
        let invocation = invocations.first().cloned();
        Ok(HandledNativeEvent {
            event,
            invocations,
            invocation,
            interaction_changes,
            value_sensitivity,
        })
    }

    fn route_event(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        focus_within_transitions: &[FocusWithinTransition],
        event: &NativeEvent,
    ) -> Vec<ActionInvocation> {
        let mut invocations = self
            .event_router
            .route_all_for_current_target(blueprint, event, event.node);
        if event.kind == crate::event::NativeEventKind::Close {
            return invocations;
        }
        if !matches!(
            event.kind,
            crate::event::NativeEventKind::Focus | crate::event::NativeEventKind::Blur
        ) {
            for (current_target, route_blueprint) in route_blueprints {
                invocations.extend(self.event_router.route_all_for_current_target(
                    route_blueprint,
                    event,
                    *current_target,
                ));
            }
        }
        for transition in focus_within_transitions {
            if has_focus_within_handler(&transition.blueprint) {
                invocations.extend(self.event_router.route_focus_within_for_current_target(
                    &transition.blueprint,
                    event,
                    transition.node,
                ));
            }
        }
        invocations
    }

    fn focus_within_transitions(
        &self,
        blueprint: &NativeWidgetBlueprint,
        route_blueprints: &[RoutedBlueprint],
        event: &NativeEvent,
    ) -> Vec<FocusWithinTransition> {
        let focused = match event.kind {
            crate::event::NativeEventKind::Focus => true,
            crate::event::NativeEventKind::Blur => false,
            _ => return Vec::new(),
        };
        let related_target = event.context.related_target.or_else(|| {
            (event.kind == crate::event::NativeEventKind::Focus)
                .then_some(self.focus_owner.filter(|owner| {
                    self.interaction_state
                        .node(*owner)
                        .is_some_and(|state| state.focused)
                }))
                .flatten()
        });
        let mut related_path = BTreeSet::new();
        if let Some(related_target) = related_target {
            related_path.insert(related_target);
            related_path.extend(self.renderer.ancestor_ids(related_target));
        }

        std::iter::once((event.node, blueprint))
            .chain(
                route_blueprints
                    .iter()
                    .map(|(node, blueprint)| (*node, blueprint)),
            )
            .filter(|(node, blueprint)| {
                !related_path.contains(node)
                    && (tracks_focus_within(blueprint)
                        || self
                            .interaction_state
                            .node(*node)
                            .is_some_and(|state| state.focus_within))
            })
            .map(|(node, blueprint)| FocusWithinTransition {
                node,
                blueprint: blueprint.clone(),
                focused,
            })
            .collect()
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
        self.projected_interaction_styles
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
        self.update_mounted_props_and_invalidate_interaction_styles(&updates)
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
        self.update_mounted_props_and_invalidate_interaction_styles(&updates)?;
        self.project_interaction_style_nodes(updates.keys().copied())
    }

    fn project_mounted_overlays_to_host(&mut self) -> GuiResult<()> {
        let snapshot = self.renderer.mounted_snapshot();
        let updates = self.overlay_registry.projected_props(&snapshot);
        self.update_mounted_props_and_invalidate_interaction_styles(&updates)
    }

    fn project_mounted_overlay_positions_to_host(&mut self) -> GuiResult<()> {
        let positions = mounted_overlay_positions(&self.renderer.mounted_snapshot())?;
        if positions.is_empty() {
            return Ok(());
        }
        let host = self.host.overlay_position_host().ok_or_else(|| {
            GuiError::host(
                "native host does not support the anchored overlay positioning requested by this tree",
            )
        })?;
        for position in positions {
            host.position_overlay(position.overlay, position.anchor, position.request)?;
        }
        Ok(())
    }

    fn update_mounted_props_and_invalidate_interaction_styles(
        &mut self,
        updates: &BTreeMap<HostNodeId, NativeProps>,
    ) -> GuiResult<()> {
        self.renderer
            .update_mounted_props(updates, &mut self.host)?;
        for node in updates.keys() {
            self.projected_interaction_styles.remove(node);
            self.interaction_revisions.remove(node);
        }
        Ok(())
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

    fn constrain_focus_target(
        &self,
        current: HostNodeId,
        requested: HostNodeId,
    ) -> Option<HostNodeId> {
        if self
            .overlay_registry
            .allows_focus_transition(current, requested)
            && self.focus_manager.is_focusable(requested)
        {
            Some(requested)
        } else {
            self.focus_manager.constrain_focus(current, requested)
        }
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

pub(crate) fn effective_blueprint_value_sensitivity(
    blueprint: &NativeWidgetBlueprint,
) -> ValueSensitivity {
    blueprint.effective_value_sensitivity()
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
            self.constrain_focus_target(current, requested)
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
        event = normalize_keyboard_event_value(event);
        if event.context.modality == NativeInputModality::Unknown {
            event.context.modality = event.effective_modality();
        }
        let dispatch_unchanged_selection = event.kind
            == crate::event::NativeEventKind::SelectionChange
            && is_missing_selection_value(event.value.as_deref());
        if self.redirect_contained_focus(&event)? {
            let value_sensitivity = self
                .host
                .blueprint(event.node)
                .map(effective_blueprint_value_sensitivity)
                .unwrap_or(ValueSensitivity::Sensitive);
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity,
            });
        }
        let mut dismiss_after_event = None;
        match self.overlay_registry.handle_event(&event) {
            OverlayEventDisposition::Continue => {}
            OverlayEventDisposition::Suppress => {
                let value_sensitivity = self
                    .host
                    .blueprint(event.node)
                    .map(effective_blueprint_value_sensitivity)
                    .unwrap_or(ValueSensitivity::Sensitive);
                return Ok(HandledNativeEvent {
                    event,
                    invocations: Vec::new(),
                    invocation: None,
                    interaction_changes: Vec::new(),
                    value_sensitivity,
                });
            }
            OverlayEventDisposition::Dismiss(overlay) => {
                let Some((blueprint, route_blueprints)) = self.native_event_route(overlay) else {
                    return Ok(HandledNativeEvent {
                        event,
                        invocations: Vec::new(),
                        invocation: None,
                        interaction_changes: Vec::new(),
                        value_sensitivity: ValueSensitivity::Sensitive,
                    });
                };
                let close = NativeEvent::new(overlay, crate::event::NativeEventKind::Close)
                    .context(event.context);
                return self.handle_event_with_route_results(
                    &blueprint,
                    &route_blueprints,
                    close,
                    false,
                );
            }
            OverlayEventDisposition::DismissAfterEvent(overlay) => {
                dismiss_after_event = Some(overlay);
            }
        }
        let Some((mut blueprint, mut route_blueprints)) = self.native_event_route(event.node)
        else {
            return Ok(HandledNativeEvent {
                event,
                invocations: Vec::new(),
                invocation: None,
                interaction_changes: Vec::new(),
                value_sensitivity: ValueSensitivity::Sensitive,
            });
        };
        let close_invocations = dismiss_after_event
            .and_then(|overlay| {
                self.native_event_route(overlay)
                    .map(|(blueprint, route_blueprints)| {
                        let close = NativeEvent::new(overlay, crate::event::NativeEventKind::Close)
                            .context(event.context);
                        self.route_event(&blueprint, &route_blueprints, &[], &close)
                    })
            })
            .unwrap_or_default();
        self.preserve_number_field_input_focus(&blueprint, &event)?;
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
        let focused = self
            .focus_owner
            .or_else(|| self.interaction_state.focused_node())
            == Some(event.node);
        let number_field_step_handled =
            normalize_number_field_step_event(&blueprint, focused, &mut event);
        if !number_field_step_handled
            && !has_explicit_key_down_handler(&blueprint, &route_blueprints)
            && !move_handles_keyboard_event(&blueprint, &route_blueprints, &event)
        {
            self.refresh_collection_layout_for_page_navigation(&event)?;
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
                        false,
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
                            false,
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
        self.handle_event_with_route_results_and_extra_invocations(
            &blueprint,
            &route_blueprints,
            event,
            dispatch_unchanged_selection,
            close_invocations,
        )
    }

    fn refresh_collection_layout_for_page_navigation(
        &mut self,
        event: &NativeEvent,
    ) -> GuiResult<()> {
        if event.kind != crate::event::NativeEventKind::KeyDown
            || !event
                .value
                .as_deref()
                .map(crate::event::native_key_value)
                .is_some_and(|key| matches!(key.as_str(), "PageUp" | "PageDown"))
        {
            return Ok(());
        }
        let Some((collection, items)) = self
            .selection_registry
            .collection_layout_request(event.node)
        else {
            return Ok(());
        };
        if let Some(layout) = self.host.measure_collection_layout(collection, &items)? {
            self.selection_registry
                .set_collection_layout(collection, layout)?;
        }
        Ok(())
    }

    fn restore_suppressed_native_selection(
        &mut self,
        mut event: NativeEvent,
    ) -> GuiResult<HandledNativeEvent> {
        let value_sensitivity = self
            .host
            .blueprint(event.node)
            .map(effective_blueprint_value_sensitivity)
            .unwrap_or(ValueSensitivity::Sensitive);
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
            value_sensitivity,
        })
    }

    fn preserve_number_field_input_focus(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> GuiResult<()> {
        if event.kind != crate::event::NativeEventKind::PressStart
            || event.context.modality != NativeInputModality::Mouse
            || blueprint.role != crate::native::NativeRole::Button
            || blueprint.control_state.disabled
            || !blueprint
                .metadata
                .get(NUMBER_FIELD_STEP_METADATA_KEY)
                .is_some_and(|value| matches!(value.as_str(), "increment" | "decrement"))
        {
            return Ok(());
        }

        let Some(parent) = self.renderer.ancestor_ids(event.node).into_iter().next() else {
            return Ok(());
        };
        let Some(input) = self.renderer.child_ids(parent).into_iter().find(|child| {
            self.host.blueprint(*child).is_some_and(|candidate| {
                candidate.role == crate::native::NativeRole::TextField
                    && is_number_text_input(candidate)
                    && candidate
                        .metadata
                        .get(NUMBER_FIELD_INPUT_METADATA_KEY)
                        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
            })
        }) else {
            return Ok(());
        };
        let current = self
            .focus_owner
            .or_else(|| self.interaction_state.focused_node());
        if current == Some(input) {
            return Ok(());
        }
        let constrained_from = current.unwrap_or(event.node);
        if self.constrain_focus_target(constrained_from, input) != Some(input) {
            return Ok(());
        }
        let Some(host) = self.host.programmatic_focus_host() else {
            return Ok(());
        };

        host.request_focus(input)?;
        self.pending_focus_modality = Some((input, NativeInputModality::Mouse));
        Ok(())
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
        if self.constrain_focus_target(current, requested) != Some(requested) {
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
        let Some(target) = self.constrain_focus_target(current, event.node) else {
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

fn normalize_number_field_step_event(
    blueprint: &NativeWidgetBlueprint,
    focused: bool,
    event: &mut NativeEvent,
) -> bool {
    if blueprint.role != crate::native::NativeRole::TextField
        || !is_number_text_input(blueprint)
        || !blueprint
            .metadata
            .get(NUMBER_FIELD_INPUT_METADATA_KEY)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    {
        return false;
    }

    enum StepTarget {
        Direction(RangeStepDirection),
        Bound(Option<f64>),
    }

    let target = match event.kind {
        crate::event::NativeEventKind::KeyDown if event.context.modifiers.is_empty() => match event
            .value
            .as_deref()
            .map(crate::event::native_key_value)
            .as_deref()
        {
            Some("ArrowUp" | "PageUp") => StepTarget::Direction(RangeStepDirection::Increment),
            Some("ArrowDown" | "PageDown") => StepTarget::Direction(RangeStepDirection::Decrement),
            Some("Home") => StepTarget::Bound(blueprint.control_state.min),
            Some("End") => StepTarget::Bound(blueprint.control_state.max),
            _ => return false,
        },
        crate::event::NativeEventKind::Wheel
            if focused
                && !event.context.modifiers.control
                && !blueprint
                    .metadata
                    .get(NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY)
                    .is_some_and(|value| value.eq_ignore_ascii_case("true")) =>
        {
            let Some(delta) = event.context.delta else {
                return false;
            };
            let Some(direction) = number_field_wheel_step_direction(delta.x, delta.y) else {
                return false;
            };
            StepTarget::Direction(direction)
        }
        _ => return false,
    };
    if blueprint.control_state.disabled || blueprint.control_state.read_only {
        return event.kind == crate::event::NativeEventKind::KeyDown;
    }

    let current = blueprint.control_state.current;
    let next = match target {
        StepTarget::Direction(direction) => step_range_value(
            current,
            blueprint.control_state.min,
            blueprint.control_state.max,
            blueprint.control_state.step,
            direction,
        ),
        StepTarget::Bound(bound) => bound.filter(|value| value.is_finite()),
    };
    let Some(next) = next else {
        return true;
    };
    let changed = current != Some(next);
    if changed {
        event.kind = crate::event::NativeEventKind::Change;
        event.value = Some(format_normalized_number(next));
    }
    true
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
        normalize_number_text_change_value(blueprint, event)
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
    event: NativeEvent,
) -> NativeEvent {
    normalize_ranged_change_value_with(blueprint, event, parse_event_number)
}

fn normalize_number_text_change_value(
    blueprint: &NativeWidgetBlueprint,
    event: NativeEvent,
) -> NativeEvent {
    normalize_ranged_change_value_with(blueprint, event, |value| {
        parse_localized_event_number(blueprint, value)
    })
}

fn normalize_ranged_change_value_with(
    blueprint: &NativeWidgetBlueprint,
    mut event: NativeEvent,
    parse: impl FnOnce(&str) -> Option<f64>,
) -> NativeEvent {
    let Some(value) = event.value.as_deref().and_then(parse) else {
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

fn parse_localized_event_number(blueprint: &NativeWidgetBlueprint, value: &str) -> Option<f64> {
    let locale = blueprint
        .control_state
        .lang
        .as_deref()
        .unwrap_or(DEFAULT_FORMATTING_LOCALE);
    let options = NumberFormatOptions::from_metadata(&blueprint.metadata);
    match cached_number_parser(locale) {
        Ok(parser) => parser.parse_with_options(value, options).ok(),
        Err(_) => parse_number_style_fallback(value, options.style),
    }
}

fn parse_number_style_fallback(value: &str, style: NumberFormatStyle) -> Option<f64> {
    match style {
        NumberFormatStyle::Decimal => parse_event_number(value),
        NumberFormatStyle::Percent => {
            let value = value
                .trim()
                .trim_start_matches(['%', '\u{066a}', '\u{fe6a}', '\u{ff05}'])
                .trim_end_matches(['%', '\u{066a}', '\u{fe6a}', '\u{ff05}'])
                .trim();
            parse_event_number(value).map(|value| value / 100.0)
        }
    }
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

fn has_focus_within_handler(blueprint: &NativeWidgetBlueprint) -> bool {
    !blueprint.control_state.disabled
        && ["onFocusWithin", "onBlurWithin", "onFocusWithinChange"]
            .into_iter()
            .any(|name| {
                blueprint
                    .events
                    .get(name)
                    .is_some_and(|action| !action.is_empty())
            })
}

fn tracks_focus_within(blueprint: &NativeWidgetBlueprint) -> bool {
    has_focus_within_handler(blueprint)
        || blueprint
            .portable_style
            .interaction_requirements()
            .focus_within
}

fn with_runtime_rollback_context(error: GuiError, rollback_errors: Vec<GuiError>) -> GuiError {
    if rollback_errors.is_empty() {
        return error;
    }
    let details = rollback_errors
        .into_iter()
        .map(|rollback_error| rollback_error.to_string())
        .collect::<Vec<_>>()
        .join("; ");
    GuiError::host(format!("{error}; runtime rollback also failed: {details}"))
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
        let mut events = self.host.take_native_events();
        crate::event::link_focus_transitions(&mut events);
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
#[path = "runtime/focus_within_tests.rs"]
mod focus_within_tests;

#[cfg(test)]
#[path = "runtime/interaction_style_tests.rs"]
mod interaction_style_tests;

#[cfg(test)]
#[path = "runtime/overlay_tests.rs"]
mod overlay_tests;

#[cfg(test)]
#[path = "runtime/accessibility_conformance_tests.rs"]
mod accessibility_conformance_tests;

#[cfg(test)]
mod tests;
