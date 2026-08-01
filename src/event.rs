use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;
use crate::input::{NativeEventContext, NativeInputModality, NativeKeyModifiers};
use crate::native::ValueSensitivity;
use crate::platform::NativeWidgetBlueprint;
use crate::selection::Selection;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub(crate) use crate::semantic_event::{
    actions_for_event as semantic_actions_for_event,
    focus_within_actions_for_event as semantic_focus_within_actions_for_event, is_activation_key,
    native_key_value, SemanticActionSource, SemanticEventData,
};

#[cfg(any(
    test,
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
mod move_interaction;
#[cfg(any(
    test,
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
mod press;

#[cfg(any(
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
pub(crate) use move_interaction::{keyboard_move_events, PointerMoveState};
#[cfg(any(
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
pub(crate) use press::{
    virtual_press_events, KeyboardPressState, NativeInteractionProfile, NativeLongPressTimer,
    NumberFieldStepperPressState, NumberFieldStepperTimer, PointerPressState,
};

/// Maximum number of successful action invocations retained for diagnostics by default.
pub const DEFAULT_ACTION_INVOCATION_HISTORY_LIMIT: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeEventKind {
    PressStart,
    PressEnd,
    PressUp,
    PressCancel,
    Press,
    LongPressStart,
    LongPressEnd,
    LongPress,
    MoveStart,
    Move,
    MoveEnd,
    DragStart,
    DragMove,
    DragEnd,
    DropEnter,
    DropMove,
    DropActivate,
    DropExit,
    Drop,
    Action,
    HoverStart,
    HoverEnd,
    Change,
    SelectionChange,
    Toggle,
    Focus,
    Blur,
    KeyDown,
    KeyUp,
    Wheel,
    Copy,
    Cut,
    Paste,
    Close,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeEvent {
    pub node: HostNodeId,
    pub kind: NativeEventKind,
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "NativeEventContext::is_empty")]
    pub context: NativeEventContext,
}

impl std::fmt::Debug for NativeEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("NativeEvent")
            .field("node", &self.node)
            .field("kind", &self.kind)
            .field("has_value", &self.value.is_some())
            .finish()
    }
}

impl NativeEvent {
    pub fn new(node: HostNodeId, kind: NativeEventKind) -> Self {
        Self {
            node,
            kind,
            value: None,
            context: NativeEventContext::new(),
        }
    }

    pub fn validate(&self) -> GuiResult<()> {
        if self.node.get() == 0 {
            return Err(GuiError::host(
                "a3s-gui native events need a non-zero node id",
            ));
        }
        if self
            .context
            .position
            .is_some_and(|position| !position.x.is_finite() || !position.y.is_finite())
        {
            return Err(GuiError::host(
                "a3s-gui native event positions need finite coordinates",
            ));
        }
        if self
            .context
            .delta
            .is_some_and(|delta| !delta.x.is_finite() || !delta.y.is_finite())
        {
            return Err(GuiError::host(
                "a3s-gui native event movement deltas need finite coordinates",
            ));
        }
        if self
            .context
            .related_target
            .is_some_and(|target| target.get() == 0)
        {
            return Err(GuiError::host(
                "a3s-gui native event related target needs a non-zero node id",
            ));
        }
        Ok(())
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn context(mut self, context: NativeEventContext) -> Self {
        self.context = context;
        self
    }

    pub fn modality(mut self, modality: NativeInputModality) -> Self {
        self.context.modality = modality;
        self
    }

    pub fn modifiers(mut self, modifiers: NativeKeyModifiers) -> Self {
        self.context.modifiers = modifiers;
        self
    }

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.context.position = Some(crate::input::NativeEventPosition::new(x, y));
        self
    }

    pub fn delta(mut self, x: f64, y: f64) -> Self {
        self.context.delta = Some(crate::input::NativeEventPosition::new(x, y));
        self
    }

    pub fn repeat(mut self, repeat: bool) -> Self {
        self.context.repeat = repeat;
        self
    }

    /// Returns the explicit modality, or a conservative inference for event
    /// kinds whose native source is unambiguous.
    pub fn effective_modality(&self) -> NativeInputModality {
        if self.context.modality != NativeInputModality::Unknown {
            return self.context.modality;
        }

        match self.kind {
            NativeEventKind::KeyDown | NativeEventKind::KeyUp => NativeInputModality::Keyboard,
            NativeEventKind::HoverStart
            | NativeEventKind::HoverEnd
            | NativeEventKind::MoveStart
            | NativeEventKind::Move
            | NativeEventKind::MoveEnd
            | NativeEventKind::DragStart
            | NativeEventKind::DragMove
            | NativeEventKind::DragEnd
            | NativeEventKind::DropEnter
            | NativeEventKind::DropMove
            | NativeEventKind::DropActivate
            | NativeEventKind::DropExit
            | NativeEventKind::Drop
            | NativeEventKind::Wheel => NativeInputModality::Mouse,
            _ => NativeInputModality::Unknown,
        }
    }
}

/// Adds the opposite focus target to adjacent native blur/focus pairs.
///
/// Native toolkits generally expose focus loss and focus gain as separate
/// callbacks. Linking them before portable dispatch lets focus-within avoid a
/// false exit/re-entry when focus moves between descendants of one subtree.
pub(crate) fn link_focus_transitions(events: &mut [NativeEvent]) {
    if events.len() < 2 {
        return;
    }

    for index in 0..events.len() - 1 {
        let (left_kind, left_node) = (events[index].kind, events[index].node);
        let (right_kind, right_node) = (events[index + 1].kind, events[index + 1].node);
        let is_transition = matches!(
            (left_kind, right_kind),
            (NativeEventKind::Blur, NativeEventKind::Focus)
                | (NativeEventKind::Focus, NativeEventKind::Blur)
        );
        if !is_transition || left_node == right_node {
            continue;
        }

        if events[index].context.related_target.is_none() {
            events[index].context.related_target = Some(right_node);
        }
        if events[index + 1].context.related_target.is_none() {
            events[index + 1].context.related_target = Some(left_node);
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    /// Native node that originally produced the event.
    pub node: HostNodeId,
    /// Ancestor currently handling a bubbled event. `None` means `node`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_target: Option<HostNodeId>,
    pub action: String,
    pub event: NativeEventKind,
    #[serde(default, skip_serializing_if = "NativeEventContext::is_empty")]
    pub context: NativeEventContext,
    pub value: Option<String>,
}

impl std::fmt::Debug for ActionInvocation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ActionInvocation")
            .field("node", &self.node)
            .field("action", &self.action)
            .field("event", &self.event)
            .field("has_value", &self.value.is_some())
            .finish()
    }
}

impl ActionInvocation {
    pub fn new(node: HostNodeId, action: impl Into<String>, event: NativeEventKind) -> Self {
        Self {
            node,
            current_target: None,
            action: action.into(),
            event,
            context: NativeEventContext::new(),
            value: None,
        }
    }

    pub fn with_context(mut self, context: NativeEventContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_current_target(mut self, current_target: HostNodeId) -> Self {
        self.current_target = (current_target != self.node).then_some(current_target);
        self
    }

    pub fn current_target(&self) -> HostNodeId {
        self.current_target.unwrap_or(self.node)
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn modality(&self) -> NativeInputModality {
        self.context.modality
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Decodes a selection payload as an explicit key set or `all`.
    /// Legacy scalar values decode as a one-key selection.
    pub fn selection(&self) -> GuiResult<Option<Selection>> {
        self.payload()
    }

    pub fn payload_json(&self) -> GuiResult<Option<JsonValue>> {
        self.value.as_deref().map(action_payload_json).transpose()
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
                        "action {:?} payload did not decode as {}: {json_error}; string fallback failed: {string_error}",
                        self.action,
                        std::any::type_name::<T>()
                    ))
                })
            })
            .map(Some)
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventRouter;

impl EventRouter {
    pub fn new() -> Self {
        Self
    }

    pub fn route(
        &self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Option<ActionInvocation> {
        self.route_all(blueprint, event).into_iter().next()
    }

    /// Routes every semantic callback produced by one native event.
    ///
    /// Lifecycle-specific callbacks precede their corresponding change
    /// callback. The runtime then concatenates target and ancestor batches in
    /// nearest-first bubbling order.
    pub fn route_all(
        &self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Vec<ActionInvocation> {
        self.route_all_for_current_target(blueprint, event, event.node)
    }

    pub(crate) fn route_all_for_current_target(
        &self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
        current_target: HostNodeId,
    ) -> Vec<ActionInvocation> {
        actions_for_event(blueprint, event)
            .into_iter()
            .map(|action| ActionInvocation {
                node: event.node,
                current_target: (current_target != event.node).then_some(current_target),
                action: action.to_string(),
                event: event.kind,
                context: event.context,
                value: event
                    .value
                    .clone()
                    .or_else(|| static_action_value(blueprint)),
            })
            .collect()
    }

    pub(crate) fn route_focus_within_for_current_target(
        &self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
        current_target: HostNodeId,
    ) -> Vec<ActionInvocation> {
        focus_within_actions_for_event(blueprint, event)
            .into_iter()
            .map(|action| ActionInvocation {
                node: event.node,
                current_target: (current_target != event.node).then_some(current_target),
                action: action.to_string(),
                event: event.kind,
                context: event.context,
                value: event.value.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredAction {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ActionRegistry {
    actions: BTreeMap<String, RegisteredAction>,
    invocations: Vec<ActionInvocation>,
    invocation_history_limit: usize,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self {
            actions: BTreeMap::new(),
            invocations: Vec::new(),
            invocation_history_limit: DEFAULT_ACTION_INVOCATION_HISTORY_LIMIT,
        }
    }
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry with a bounded diagnostic invocation history.
    ///
    /// A limit of zero disables invocation history without affecting action dispatch.
    pub fn with_invocation_history_limit(invocation_history_limit: usize) -> Self {
        Self {
            invocation_history_limit,
            ..Self::default()
        }
    }

    pub fn register(&mut self, id: impl Into<String>) {
        let id = id.into();
        self.actions.entry(id.clone()).or_insert(RegisteredAction {
            id,
            disabled: false,
            label: None,
        });
    }

    pub fn register_labeled(&mut self, id: impl Into<String>, label: impl Into<String>) {
        let id = id.into();
        self.actions.insert(
            id.clone(),
            RegisteredAction {
                id,
                disabled: false,
                label: Some(label.into()),
            },
        );
    }

    pub fn replace_registered<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = RegisteredAction>,
    {
        self.actions.clear();
        for action in actions {
            self.actions.insert(action.id.clone(), action);
        }
    }

    pub fn contains(&self, id: &str) -> bool {
        self.actions.contains_key(id)
    }

    pub fn registered(&self, id: &str) -> Option<&RegisteredAction> {
        self.actions.get(id)
    }

    pub fn is_disabled(&self, id: &str) -> bool {
        self.registered(id).is_some_and(|action| action.disabled)
    }

    pub fn invocations(&self) -> &[ActionInvocation] {
        &self.invocations
    }

    pub fn invocation_history_limit(&self) -> usize {
        self.invocation_history_limit
    }

    /// Takes the retained diagnostic invocations, leaving the registry empty.
    pub fn take_invocations(&mut self) -> Vec<ActionInvocation> {
        std::mem::take(&mut self.invocations)
    }

    pub(crate) fn truncate_invocations(&mut self, len: usize) {
        self.invocations.truncate(len);
    }

    pub fn invoke(&mut self, invocation: ActionInvocation) -> GuiResult<()> {
        // A low-level caller has no blueprint from which sensitivity can be
        // inferred. Default to a redacted diagnostic record; runtimes that own
        // the blueprint use the explicit method below.
        self.invoke_with_sensitivity(invocation, ValueSensitivity::Sensitive)
    }

    pub fn invoke_with_sensitivity(
        &mut self,
        invocation: ActionInvocation,
        value_sensitivity: ValueSensitivity,
    ) -> GuiResult<()> {
        self.validate_invocation(&invocation)?;
        let mut diagnostic_invocation = invocation;
        if value_sensitivity.is_sensitive() {
            diagnostic_invocation.value = None;
        }
        push_bounded(
            &mut self.invocations,
            diagnostic_invocation,
            self.invocation_history_limit,
        );
        Ok(())
    }

    /// Validates the complete batch before recording any invocation.
    pub fn invoke_all(&mut self, invocations: &[ActionInvocation]) -> GuiResult<()> {
        self.invoke_all_with_sensitivity(invocations, ValueSensitivity::Sensitive)
    }

    /// Validates the complete batch before recording redacted diagnostic entries.
    pub fn invoke_all_with_sensitivity(
        &mut self,
        invocations: &[ActionInvocation],
        value_sensitivity: ValueSensitivity,
    ) -> GuiResult<()> {
        for invocation in invocations {
            self.validate_invocation(invocation)?;
        }
        for invocation in invocations {
            let mut diagnostic_invocation = invocation.clone();
            if value_sensitivity.is_sensitive() {
                diagnostic_invocation.value = None;
            }
            push_bounded(
                &mut self.invocations,
                diagnostic_invocation,
                self.invocation_history_limit,
            );
        }
        Ok(())
    }

    fn validate_invocation(&self, invocation: &ActionInvocation) -> GuiResult<()> {
        let Some(action) = self.registered(&invocation.action) else {
            return Err(GuiError::host(format!(
                "unregistered action {}",
                invocation.action
            )));
        };
        if action.disabled {
            return Err(GuiError::host(format!(
                "disabled action {}",
                invocation.action
            )));
        }
        Ok(())
    }
}

fn push_bounded<T>(items: &mut Vec<T>, item: T, limit: usize) {
    if limit == 0 {
        return;
    }
    if items.len() == limit {
        items.remove(0);
    }
    items.push(item);
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn actions_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Vec<&'a str> {
    semantic_actions_for_event(semantic_source(blueprint), semantic_event(event))
}

fn focus_within_actions_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Vec<&'a str> {
    semantic_focus_within_actions_for_event(semantic_source(blueprint), event.kind)
}

fn static_action_value(blueprint: &NativeWidgetBlueprint) -> Option<String> {
    semantic_source(blueprint)
        .static_action_value()
        .map(str::to_string)
}

fn semantic_source(blueprint: &NativeWidgetBlueprint) -> SemanticActionSource<'_> {
    SemanticActionSource::new(
        blueprint.role,
        blueprint.action.as_deref(),
        &blueprint.events,
        &blueprint.metadata,
        None,
        blueprint.control_state.expanded,
    )
}

fn semantic_event(event: &NativeEvent) -> SemanticEventData<'_> {
    SemanticEventData {
        kind: event.kind,
        modality: event.effective_modality(),
        value: event.value.as_deref(),
        handled_activation: event.context.handled_activation,
    }
}

fn action_payload_json(raw: &str) -> GuiResult<JsonValue> {
    serde_json::from_str(raw).or_else(|_| Ok(JsonValue::String(raw.to_string())))
}

pub(crate) fn non_empty_action(action: Option<&String>) -> Option<&str> {
    action
        .map(String::as_str)
        .filter(|action| !action.is_empty())
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "event/conformance_tests.rs"]
mod conformance_tests;
