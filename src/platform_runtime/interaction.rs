use std::collections::{BTreeMap, BTreeSet};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEventKind;
use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::platform_host::{
    PlatformElementId, PlatformHostRevision, PlatformInputDeviceId, PlatformPoint,
    PlatformPointerButton, PlatformPointerId, PlatformWheelDeltaMode,
};
use crate::semantic_event::{
    actions_for_event as semantic_actions_for_event,
    focus_within_actions_for_event as semantic_focus_within_actions_for_event, SemanticEventData,
};

use super::drag_drop::{SelfDrawnDragCandidate, SelfDrawnDragContext, SelfDrawnDragSession};
use super::interaction_tree::SelfDrawnInteractionTree;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Normalized details retained while a raw host event becomes semantic actions.
pub struct SelfDrawnEventContext {
    pub device: PlatformInputDeviceId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<PlatformPointerId>,
    #[serde(default)]
    pub modality: NativeInputModality,
    #[serde(default, skip_serializing_if = "NativeKeyModifiers::is_empty")]
    pub modifiers: NativeKeyModifiers,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<PlatformPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<PlatformPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button: Option<PlatformPointerButton>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wheel_delta_mode: Option<PlatformWheelDeltaMode>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub repeat: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub click_count: u8,
    #[serde(default, skip_serializing_if = "is_false")]
    pub handled_activation: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_target: Option<PlatformElementId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag: Option<SelfDrawnDragContext>,
    pub timestamp_micros: u64,
}

impl SelfDrawnEventContext {
    pub(crate) fn pointer(
        device: PlatformInputDeviceId,
        pointer: PlatformPointerId,
        modality: NativeInputModality,
        modifiers: NativeKeyModifiers,
        position: PlatformPoint,
        button: Option<PlatformPointerButton>,
        pressure: Option<f64>,
        timestamp_micros: u64,
    ) -> Self {
        Self {
            device,
            pointer: Some(pointer),
            modality,
            modifiers,
            position: Some(position),
            delta: None,
            button,
            pressure,
            wheel_delta_mode: None,
            repeat: false,
            click_count: 0,
            handled_activation: false,
            related_target: None,
            drag: None,
            timestamp_micros,
        }
    }

    pub(crate) fn keyboard(
        device: PlatformInputDeviceId,
        modifiers: NativeKeyModifiers,
        repeat: bool,
        timestamp_micros: u64,
    ) -> Self {
        Self {
            device,
            pointer: None,
            modality: NativeInputModality::Keyboard,
            modifiers,
            position: None,
            delta: None,
            button: None,
            pressure: None,
            wheel_delta_mode: None,
            repeat,
            click_count: 0,
            handled_activation: false,
            related_target: None,
            drag: None,
            timestamp_micros,
        }
    }

    pub(crate) fn wheel(
        device: PlatformInputDeviceId,
        modifiers: NativeKeyModifiers,
        position: PlatformPoint,
        delta: PlatformPoint,
        delta_mode: PlatformWheelDeltaMode,
        timestamp_micros: u64,
    ) -> Self {
        Self {
            device,
            pointer: None,
            modality: NativeInputModality::Mouse,
            modifiers,
            position: Some(position),
            delta: Some(delta),
            button: None,
            pressure: None,
            wheel_delta_mode: Some(delta_mode),
            repeat: false,
            click_count: 0,
            handled_activation: false,
            related_target: None,
            drag: None,
            timestamp_micros,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// One stable-id semantic callback selected from a committed self-drawn frame.
pub struct SelfDrawnActionInvocation {
    pub frame_revision: PlatformHostRevision,
    pub event_sequence: u64,
    pub node: PlatformElementId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_target: Option<PlatformElementId>,
    pub action: String,
    pub event: NativeEventKind,
    pub context: SelfDrawnEventContext,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Debug for SelfDrawnActionInvocation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SelfDrawnActionInvocation")
            .field("frame_revision", &self.frame_revision)
            .field("event_sequence", &self.event_sequence)
            .field("node", &self.node)
            .field("current_target", &self.current_target)
            .field("action", &self.action)
            .field("event", &self.event)
            .field("has_value", &self.value.is_some())
            .finish()
    }
}

impl SelfDrawnActionInvocation {
    pub fn current_target(&self) -> &PlatformElementId {
        self.current_target.as_ref().unwrap_or(&self.node)
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn payload_json(&self) -> GuiResult<Option<JsonValue>> {
        self.value
            .as_deref()
            .map(|raw| {
                serde_json::from_str(raw).or_else(|_| Ok(JsonValue::String(raw.to_string())))
            })
            .transpose()
    }

    pub fn payload<T>(&self) -> GuiResult<Option<T>>
    where
        T: DeserializeOwned,
    {
        let Some(raw) = self.value.as_deref() else {
            return Ok(None);
        };
        serde_json::from_str(raw)
            .or_else(|json_error| {
                serde_json::from_value(JsonValue::String(raw.to_string())).map_err(|string_error| {
                    GuiError::host(format!(
                        "self-drawn action {:?} payload did not decode as {}: {json_error}; string fallback failed: {string_error}",
                        self.action,
                        std::any::type_name::<T>()
                    ))
                })
            })
            .map(Some)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
/// Portable visual and accessibility interaction state for one layout-path id.
pub struct SelfDrawnElementInteraction {
    pub hovered: bool,
    pub pressed: bool,
    pub long_pressed: bool,
    pub moving: bool,
    pub dragging: bool,
    pub drop_target: bool,
    pub focused: bool,
    pub focus_visible: bool,
    pub focus_within: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Before/after interaction state produced by one normalized host event.
pub struct SelfDrawnInteractionChange {
    pub node: PlatformElementId,
    pub before: SelfDrawnElementInteraction,
    pub after: SelfDrawnElementInteraction,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Whether later bubbled targets should observe an ordered action batch.
pub enum SelfDrawnActionPropagation {
    #[default]
    Continue,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Complete result of routing raw input or a scheduled interaction deadline
/// against a committed frame.
pub struct SelfDrawnInputDispatch {
    pub frame_revision: PlatformHostRevision,
    pub event_sequence: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PlatformElementId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invocations: Vec<SelfDrawnActionInvocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_changes: Vec<SelfDrawnInteractionChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub propagation_stopped_at: Option<PlatformElementId>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct SelfDrawnInteractionSession {
    states: BTreeMap<PlatformElementId, SelfDrawnElementInteraction>,
    pub(super) focused: Option<PlatformElementId>,
    pub(super) pointers: BTreeMap<PlatformPointerId, PointerInteraction>,
    pub(super) keyboard_presses: BTreeMap<KeyboardPressKey, KeyboardPress>,
    pub(super) last_clicks: BTreeMap<PlatformPointerId, LastClick>,
    pub(super) hover_counts: BTreeMap<PlatformElementId, u32>,
    pub(super) pressed_counts: BTreeMap<PlatformElementId, u32>,
    pub(super) long_pressed_counts: BTreeMap<PlatformElementId, u32>,
    pub(super) moving_counts: BTreeMap<PlatformElementId, u32>,
    pub(super) active_drag: Option<SelfDrawnDragSession>,
    pub(super) event_sequence: u64,
}

#[derive(Debug, Clone, Default)]
pub(super) struct PointerInteraction {
    pub(super) hover_target: Option<PlatformElementId>,
    pub(super) active_press: Option<ActivePress>,
}

#[derive(Debug, Clone)]
pub(super) struct ActivePress {
    pub(super) target: PlatformElementId,
    pub(super) over_target: bool,
    pub(super) start_emitted: bool,
    pub(super) click_count: u8,
    pub(super) long_press_threshold_micros: Option<u64>,
    pub(super) long_press: Option<LongPressTracking>,
    pub(super) long_press_recognized: bool,
    pub(super) movement: Option<PointerMoveTracking>,
    pub(super) drag_candidate: Option<SelfDrawnDragCandidate>,
}

#[derive(Debug, Clone)]
pub(super) struct LongPressTracking {
    pub(super) deadline_micros: u64,
    pub(super) context: SelfDrawnEventContext,
}

#[derive(Debug, Clone)]
pub(super) struct PointerMoveTracking {
    pub(super) last_position: PlatformPoint,
    pub(super) did_move: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct KeyboardPressKey {
    pub(super) device: PlatformInputDeviceId,
    pub(super) physical_key: String,
}

#[derive(Debug, Clone)]
pub(super) struct KeyboardPress {
    pub(super) target: PlatformElementId,
}

#[derive(Debug, Clone)]
pub(super) struct LastClick {
    pub(super) target: PlatformElementId,
    pub(super) timestamp_micros: u64,
    pub(super) count: u8,
}

#[derive(Debug, Clone)]
pub(super) struct RoutedSemanticEvent {
    pub(super) target: PlatformElementId,
    pub(super) kind: NativeEventKind,
    pub(super) context: SelfDrawnEventContext,
    pub(super) value: Option<String>,
}

impl SelfDrawnInteractionSession {
    pub(super) fn reconcile(&mut self, tree: &SelfDrawnInteractionTree) {
        self.states.retain(|id, _| tree.contains(id));
        for id in tree.ids() {
            self.states.entry(id.clone()).or_default();
        }
        self.pointers.retain(|_, pointer| {
            pointer.hover_target = pointer
                .hover_target
                .take()
                .filter(|id| tree.is_available(id));
            pointer.active_press = pointer
                .active_press
                .take()
                .filter(|press| tree.is_available(&press.target))
                .map(|mut press| {
                    if let Some(candidate) = press.drag_candidate.as_mut() {
                        if let Some(source) = tree.drag_source_for_start(&press.target) {
                            candidate.source = source;
                        } else {
                            press.drag_candidate = None;
                        }
                    }
                    press
                });
            pointer.hover_target.is_some() || pointer.active_press.is_some()
        });
        self.keyboard_presses
            .retain(|_, press| tree.is_available(&press.target));
        self.last_clicks
            .retain(|_, click| tree.is_available(&click.target));
        self.focused = self
            .focused
            .take()
            .filter(|id| tree.node(id).is_some_and(|node| node.focusable))
            .or_else(|| tree.auto_focus_target());
        let previous_drag = self.active_drag.take();
        let active_drag =
            previous_drag.and_then(|mut drag| {
                tree.drag_source(&drag.source)?;
                if let Some(target) = drag.current_target.take() {
                    if let Some(matched) = tree.compatible_drop_target(
                        &target,
                        drag.current_collection.as_ref(),
                        drag.current_collection_target.as_ref(),
                        &drag.items,
                        &drag.types,
                        &drag.allowed_operations,
                        drag.source_collection.as_ref(),
                        &drag.dragging_keys,
                    ) {
                        drag.current_target = Some(matched.id);
                        drag.current_collection = matched.collection;
                        drag.current_collection_target = matched.collection_target;
                        drag.current_operation = matched.operation;
                        drag.current_item_indices = matched.item_indices;
                    } else {
                        drag.current_operation = super::SelfDrawnDropOperation::Cancel;
                        drag.current_collection = None;
                        drag.current_collection_target = None;
                        drag.current_item_indices.clear();
                    }
                }
                if drag.drop_activation.as_ref().is_some_and(|tracking| {
                    !tree.drop_activation_tracking_is_valid(&drag, tracking)
                }) {
                    drag.drop_activation = None;
                }
                Some(drag)
            });
        self.active_drag = active_drag;

        let focus_visible = self
            .focused
            .as_ref()
            .and_then(|id| self.states.get(id))
            .is_some_and(|state| state.focus_visible);
        for state in self.states.values_mut() {
            *state = SelfDrawnElementInteraction::default();
        }
        self.rebuild_hover_counts();
        self.rebuild_pressed_counts();
        self.rebuild_long_pressed_counts();
        self.rebuild_moving_counts();
        if let Some(drag) = &self.active_drag {
            let dragging_nodes = if drag.dragging_nodes.is_empty() {
                std::slice::from_ref(&drag.source)
            } else {
                drag.dragging_nodes.as_slice()
            };
            for node in dragging_nodes {
                if let Some(state) = self.states.get_mut(node) {
                    state.dragging = true;
                }
            }
            if let Some(target) = &drag.current_target {
                if let Some(state) = self.states.get_mut(target) {
                    state.drop_target = true;
                }
            }
        }
        if let Some(focused) = &self.focused {
            if let Some(state) = self.states.get_mut(focused) {
                state.focused = true;
                state.focus_visible = focus_visible;
            }
            for id in tree.ancestors_inclusive(focused) {
                if let Some(state) = self.states.get_mut(&id) {
                    state.focus_within = true;
                }
            }
        }
    }

    pub(super) fn element(&self, id: &PlatformElementId) -> Option<&SelfDrawnElementInteraction> {
        self.states.get(id)
    }

    pub(super) fn route_event(
        &self,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        event: &RoutedSemanticEvent,
        bubble: bool,
        invocations: &mut Vec<SelfDrawnActionInvocation>,
    ) {
        let targets = if bubble {
            tree.ancestors_inclusive(&event.target)
        } else {
            vec![event.target.clone()]
        };
        for current_target in targets {
            let Some(source) = tree.source(&current_target) else {
                continue;
            };
            let data = SemanticEventData {
                kind: event.kind,
                modality: event.context.modality,
                value: event.value.as_deref(),
                handled_activation: event.context.handled_activation,
            };
            for action in semantic_actions_for_event(source, data) {
                invocations.push(SelfDrawnActionInvocation {
                    frame_revision,
                    event_sequence,
                    node: event.target.clone(),
                    current_target: (current_target != event.target)
                        .then(|| current_target.clone()),
                    action: action.to_string(),
                    event: event.kind,
                    context: event.context.clone(),
                    value: event
                        .value
                        .clone()
                        .or_else(|| source.static_action_value().map(str::to_string)),
                });
            }
        }
    }

    pub(super) fn transition_focus(
        &mut self,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        next: Option<PlatformElementId>,
        context: &SelfDrawnEventContext,
        invocations: &mut Vec<SelfDrawnActionInvocation>,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let next = next.filter(|id| tree.node(id).is_some_and(|node| node.focusable));
        if self.focused == next {
            return;
        }
        let previous = self.focused.take();
        let previous_path = previous
            .as_ref()
            .map(|id| tree.ancestors_inclusive(id))
            .unwrap_or_default();
        let next_path = next
            .as_ref()
            .map(|id| tree.ancestors_inclusive(id))
            .unwrap_or_default();
        let previous_set = previous_path.iter().cloned().collect::<BTreeSet<_>>();
        let next_set = next_path.iter().cloned().collect::<BTreeSet<_>>();

        if let Some(previous) = &previous {
            self.change_state(previous, changes, |state| {
                state.focused = false;
                state.focus_visible = false;
            });
            let mut blur_context = context.clone();
            blur_context.related_target = next.clone();
            let event = RoutedSemanticEvent {
                target: previous.clone(),
                kind: NativeEventKind::Blur,
                context: blur_context,
                value: Some("false".to_string()),
            };
            self.route_event(
                tree,
                frame_revision,
                event_sequence,
                &event,
                false,
                invocations,
            );
            for id in previous_path.iter().filter(|id| !next_set.contains(*id)) {
                self.change_state(id, changes, |state| state.focus_within = false);
                self.route_focus_within(
                    tree,
                    frame_revision,
                    event_sequence,
                    &event,
                    id,
                    invocations,
                );
            }
        }

        self.focused = next.clone();
        if let Some(next) = &next {
            self.change_state(next, changes, |state| {
                state.focused = true;
                state.focus_visible = context.modality.shows_focus_ring();
            });
            let mut focus_context = context.clone();
            focus_context.related_target = previous;
            let event = RoutedSemanticEvent {
                target: next.clone(),
                kind: NativeEventKind::Focus,
                context: focus_context,
                value: Some("true".to_string()),
            };
            self.route_event(
                tree,
                frame_revision,
                event_sequence,
                &event,
                false,
                invocations,
            );
            for id in next_path.iter().filter(|id| !previous_set.contains(*id)) {
                self.change_state(id, changes, |state| state.focus_within = true);
                self.route_focus_within(
                    tree,
                    frame_revision,
                    event_sequence,
                    &event,
                    id,
                    invocations,
                );
            }
        }
    }

    pub(super) fn change_state(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
        update: impl FnOnce(&mut SelfDrawnElementInteraction),
    ) {
        let state = self.states.entry(id.clone()).or_default();
        let before = *state;
        update(state);
        if *state != before {
            changes.push(SelfDrawnInteractionChange {
                node: id.clone(),
                before,
                after: *state,
            });
        }
    }

    pub(super) fn begin_press(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let count = self.pressed_counts.entry(id.clone()).or_default();
        let was_zero = *count == 0;
        *count = count.saturating_add(1);
        if was_zero {
            self.change_state(id, changes, |state| state.pressed = true);
        }
    }

    pub(super) fn begin_hover(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let count = self.hover_counts.entry(id.clone()).or_default();
        let was_zero = *count == 0;
        *count = count.saturating_add(1);
        if was_zero {
            self.change_state(id, changes, |state| state.hovered = true);
        }
    }

    pub(super) fn begin_long_press(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let count = self.long_pressed_counts.entry(id.clone()).or_default();
        let was_zero = *count == 0;
        *count = count.saturating_add(1);
        if was_zero {
            self.change_state(id, changes, |state| state.long_pressed = true);
        }
    }

    pub(super) fn begin_move(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let count = self.moving_counts.entry(id.clone()).or_default();
        let was_zero = *count == 0;
        *count = count.saturating_add(1);
        if was_zero {
            self.change_state(id, changes, |state| state.moving = true);
        }
    }

    pub(super) fn end_hover(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let Some(count) = self.hover_counts.get_mut(id) else {
            return;
        };
        *count = count.saturating_sub(1);
        if *count == 0 {
            self.hover_counts.remove(id);
            self.change_state(id, changes, |state| state.hovered = false);
        }
    }

    pub(super) fn end_press(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let Some(count) = self.pressed_counts.get_mut(id) else {
            return;
        };
        *count = count.saturating_sub(1);
        if *count == 0 {
            self.pressed_counts.remove(id);
            self.change_state(id, changes, |state| state.pressed = false);
        }
    }

    pub(super) fn end_long_press(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let Some(count) = self.long_pressed_counts.get_mut(id) else {
            return;
        };
        *count = count.saturating_sub(1);
        if *count == 0 {
            self.long_pressed_counts.remove(id);
            self.change_state(id, changes, |state| state.long_pressed = false);
        }
    }

    pub(super) fn end_move(
        &mut self,
        id: &PlatformElementId,
        changes: &mut Vec<SelfDrawnInteractionChange>,
    ) {
        let Some(count) = self.moving_counts.get_mut(id) else {
            return;
        };
        *count = count.saturating_sub(1);
        if *count == 0 {
            self.moving_counts.remove(id);
            self.change_state(id, changes, |state| state.moving = false);
        }
    }

    pub(super) fn next_interaction_deadline_micros(&self) -> Option<u64> {
        let long_press = self
            .pointers
            .values()
            .filter_map(|pointer| pointer.active_press.as_ref())
            .filter_map(|press| press.long_press.as_ref())
            .map(|tracking| tracking.deadline_micros)
            .min();
        match (long_press, self.next_drop_activation_deadline_micros()) {
            (Some(left), Some(right)) => Some(left.min(right)),
            (Some(deadline), None) | (None, Some(deadline)) => Some(deadline),
            (None, None) => None,
        }
    }

    fn route_focus_within(
        &self,
        tree: &SelfDrawnInteractionTree,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        event: &RoutedSemanticEvent,
        current_target: &PlatformElementId,
        invocations: &mut Vec<SelfDrawnActionInvocation>,
    ) {
        let Some(source) = tree.source(current_target) else {
            return;
        };
        for action in semantic_focus_within_actions_for_event(source, event.kind) {
            invocations.push(SelfDrawnActionInvocation {
                frame_revision,
                event_sequence,
                node: event.target.clone(),
                current_target: (current_target != &event.target).then(|| current_target.clone()),
                action: action.to_string(),
                event: event.kind,
                context: event.context.clone(),
                value: event.value.clone(),
            });
        }
    }

    fn rebuild_pressed_counts(&mut self) {
        self.pressed_counts.clear();
        for press in self
            .pointers
            .values()
            .filter_map(|pointer| pointer.active_press.as_ref())
            .filter(|press| press.start_emitted)
        {
            *self.pressed_counts.entry(press.target.clone()).or_default() += 1;
        }
        for press in self.keyboard_presses.values() {
            *self.pressed_counts.entry(press.target.clone()).or_default() += 1;
        }
        for (id, count) in &self.pressed_counts {
            if *count > 0 {
                if let Some(state) = self.states.get_mut(id) {
                    state.pressed = true;
                }
            }
        }
    }

    fn rebuild_hover_counts(&mut self) {
        self.hover_counts.clear();
        for target in self
            .pointers
            .values()
            .filter_map(|pointer| pointer.hover_target.as_ref())
        {
            *self.hover_counts.entry(target.clone()).or_default() += 1;
        }
        for (id, count) in &self.hover_counts {
            if *count > 0 {
                if let Some(state) = self.states.get_mut(id) {
                    state.hovered = true;
                }
            }
        }
    }

    fn rebuild_long_pressed_counts(&mut self) {
        self.long_pressed_counts.clear();
        for press in self
            .pointers
            .values()
            .filter_map(|pointer| pointer.active_press.as_ref())
            .filter(|press| press.long_press.is_some())
        {
            *self
                .long_pressed_counts
                .entry(press.target.clone())
                .or_default() += 1;
        }
        for (id, count) in &self.long_pressed_counts {
            if *count > 0 {
                if let Some(state) = self.states.get_mut(id) {
                    state.long_pressed = true;
                }
            }
        }
    }

    fn rebuild_moving_counts(&mut self) {
        self.moving_counts.clear();
        for press in self
            .pointers
            .values()
            .filter_map(|pointer| pointer.active_press.as_ref())
            .filter(|press| {
                press
                    .movement
                    .as_ref()
                    .is_some_and(|movement| movement.did_move)
            })
        {
            *self.moving_counts.entry(press.target.clone()).or_default() += 1;
        }
        for (id, count) in &self.moving_counts {
            if *count > 0 {
                if let Some(state) = self.states.get_mut(id) {
                    state.moving = true;
                }
            }
        }
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero(value: &u8) -> bool {
    *value == 0
}
