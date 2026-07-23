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

#[allow(dead_code)]
mod move_interaction;
#[allow(dead_code)]
mod press;

#[allow(unused_imports)]
pub(crate) use move_interaction::{keyboard_move_events, PointerMoveState};
#[allow(unused_imports)]
pub(crate) use press::{
    virtual_press_events, KeyboardPressState, NativeInteractionProfile,
    NativeInteractionSubscriptions, NativeLongPressConfig, NativeLongPressMode,
    NativeLongPressTimer, PointerPressState,
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

fn action_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Option<&'a str> {
    let events = &blueprint.events;
    match event.kind {
        NativeEventKind::PressStart => non_empty_action(events.get("onPressStart"))
            .or_else(|| non_empty_action(events.get("onPressChange"))),
        NativeEventKind::PressEnd => non_empty_action(events.get("onPressEnd"))
            .or_else(|| non_empty_action(events.get("onPressChange"))),
        NativeEventKind::PressUp => non_empty_action(events.get("onPressUp")),
        NativeEventKind::PressCancel => non_empty_action(events.get("onPressEnd"))
            .or_else(|| non_empty_action(events.get("onPressChange"))),
        NativeEventKind::Press => press_action(blueprint),
        NativeEventKind::LongPressStart => non_empty_action(events.get("onLongPressStart")),
        NativeEventKind::LongPressEnd => non_empty_action(events.get("onLongPressEnd")),
        NativeEventKind::LongPress => non_empty_action(events.get("onLongPress")),
        NativeEventKind::MoveStart => non_empty_action(events.get("onMoveStart")),
        NativeEventKind::Move => non_empty_action(events.get("onMove")),
        NativeEventKind::MoveEnd => non_empty_action(events.get("onMoveEnd")),
        NativeEventKind::Action => non_empty_action(events.get("onAction")),
        NativeEventKind::HoverStart if event.effective_modality().supports_hover() => {
            non_empty_action(events.get("onHoverStart"))
                .or_else(|| non_empty_action(events.get("onHoverChange")))
        }
        NativeEventKind::HoverEnd if event.effective_modality().supports_hover() => {
            non_empty_action(events.get("onHoverEnd"))
                .or_else(|| non_empty_action(events.get("onHoverChange")))
        }
        NativeEventKind::HoverStart | NativeEventKind::HoverEnd => None,
        NativeEventKind::Change => non_empty_action(events.get("onChange"))
            .or_else(|| non_empty_action(events.get("onInput")))
            .or_else(|| non_empty_action(blueprint.action.as_ref())),
        NativeEventKind::SelectionChange => non_empty_action(events.get("onSelectionChange"))
            .or_else(|| non_empty_action(events.get("onChange")))
            .or_else(|| non_empty_action(events.get("onInput")))
            .or_else(|| non_empty_action(blueprint.action.as_ref())),
        NativeEventKind::Toggle
            if blueprint.role == crate::native::NativeRole::Tree
                || is_expansion_toggle(blueprint) =>
        {
            non_empty_action(events.get("onExpandedChange"))
                .or_else(|| non_empty_action(events.get("onToggle")))
                .or_else(|| non_empty_action(events.get("onChange")))
                .or_else(|| non_empty_action(blueprint.action.as_ref()))
        }
        NativeEventKind::Toggle => non_empty_action(events.get("onChange"))
            .or_else(|| non_empty_action(events.get("onInput")))
            .or_else(|| non_empty_action(events.get("onToggle")))
            .or_else(|| non_empty_action(events.get("onClick")))
            .or_else(|| non_empty_action(blueprint.action.as_ref())),
        NativeEventKind::Focus => non_empty_action(events.get("onFocus"))
            .or_else(|| non_empty_action(events.get("onFocusChange"))),
        NativeEventKind::Blur => non_empty_action(events.get("onBlur"))
            .or_else(|| non_empty_action(events.get("onFocusChange"))),
        NativeEventKind::KeyDown => non_empty_action(events.get("onKeyDown"))
            .or_else(|| activation_key_action(blueprint, event)),
        NativeEventKind::KeyUp => non_empty_action(events.get("onKeyUp")),
        NativeEventKind::Wheel => non_empty_action(events.get("onWheel")),
        NativeEventKind::Copy => non_empty_action(events.get("onCopy")),
        NativeEventKind::Cut => non_empty_action(events.get("onCut")),
        NativeEventKind::Paste => non_empty_action(events.get("onPaste")),
        NativeEventKind::Close => non_empty_action(events.get("onClose"))
            .or_else(|| non_empty_action(events.get("onCloseRequest"))),
    }
}

fn actions_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Vec<&'a str> {
    let events = &blueprint.events;
    match event.kind {
        NativeEventKind::PressStart => [
            non_empty_action(events.get("onPressStart")),
            non_empty_action(events.get("onPressChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::PressEnd | NativeEventKind::PressCancel => [
            non_empty_action(events.get("onPressEnd")),
            non_empty_action(events.get("onPressChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::HoverStart if event.effective_modality().supports_hover() => [
            non_empty_action(events.get("onHoverStart")),
            non_empty_action(events.get("onHoverChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::HoverEnd if event.effective_modality().supports_hover() => [
            non_empty_action(events.get("onHoverEnd")),
            non_empty_action(events.get("onHoverChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::Focus => [
            non_empty_action(events.get("onFocus")),
            non_empty_action(events.get("onFocusChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::Blur => [
            non_empty_action(events.get("onBlur")),
            non_empty_action(events.get("onFocusChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        _ => action_for_event(blueprint, event).into_iter().collect(),
    }
}

fn focus_within_actions_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Vec<&'a str> {
    let events = &blueprint.events;
    match event.kind {
        NativeEventKind::Focus => [
            non_empty_action(events.get("onFocusWithin")),
            non_empty_action(events.get("onFocusWithinChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::Blur => [
            non_empty_action(events.get("onBlurWithin")),
            non_empty_action(events.get("onFocusWithinChange")),
        ]
        .into_iter()
        .flatten()
        .collect(),
        _ => Vec::new(),
    }
}

fn press_action(blueprint: &NativeWidgetBlueprint) -> Option<&str> {
    non_empty_action(blueprint.events.get("onPress"))
        .or_else(|| non_empty_action(blueprint.events.get("onClick")))
        .or_else(|| non_empty_action(blueprint.action.as_ref()))
}

fn static_action_value(blueprint: &NativeWidgetBlueprint) -> Option<String> {
    [
        "actionValue",
        "action-value",
        "actionPayload",
        "action-payload",
        "data-action-value",
        "data-action-payload",
        "data-a3s-action-value",
        "data-a3s-action-payload",
    ]
    .into_iter()
    .find_map(|name| {
        blueprint
            .metadata
            .get(name)
            .filter(|value| !value.is_empty())
            .cloned()
    })
}

fn action_payload_json(raw: &str) -> GuiResult<JsonValue> {
    serde_json::from_str(raw).or_else(|_| Ok(JsonValue::String(raw.to_string())))
}

pub(crate) fn non_empty_action(action: Option<&String>) -> Option<&str> {
    action
        .map(String::as_str)
        .filter(|action| !action.is_empty())
}

pub(crate) fn native_key_value(raw: &str) -> String {
    if raw == " " {
        return " ".to_string();
    }
    let trimmed = raw.trim();
    match trimmed {
        "Return" | "KP_Enter" | "ISO_Enter" => "Enter".to_string(),
        "space" | "Space" | "Spacebar" => " ".to_string(),
        "BackSpace" => "Backspace".to_string(),
        "Esc" => "Escape".to_string(),
        "ISO_Left_Tab" => "Tab".to_string(),
        "Left" => "ArrowLeft".to_string(),
        "Right" => "ArrowRight".to_string(),
        "Up" => "ArrowUp".to_string(),
        "Down" => "ArrowDown".to_string(),
        "Page_Up" => "PageUp".to_string(),
        "Page_Down" => "PageDown".to_string(),
        "" => String::new(),
        value => value.to_string(),
    }
}

fn activation_key_action<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Option<&'a str> {
    if event.context.handled_activation
        || !is_press_activation_key(blueprint.role, event.value.as_deref())
    {
        return None;
    }

    press_action(blueprint)
}

pub(crate) fn is_press_activation_key(
    role: crate::native::NativeRole,
    value: Option<&str>,
) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = native_key_value(value);
    match role {
        crate::native::NativeRole::Link | crate::native::NativeRole::ImageMapArea => {
            normalized.eq_ignore_ascii_case("enter")
        }
        crate::native::NativeRole::Button
        | crate::native::NativeRole::DisclosureSummary
        | crate::native::NativeRole::MenuItem => is_activation_key(Some(&normalized)),
        crate::native::NativeRole::ListBoxItem | crate::native::NativeRole::TreeItem => {
            normalized.eq_ignore_ascii_case("enter")
        }
        _ => false,
    }
}

pub(crate) fn is_activation_key(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = native_key_value(value);
    normalized.eq_ignore_ascii_case("enter")
        || normalized == " "
        || normalized.eq_ignore_ascii_case("space")
        || normalized.eq_ignore_ascii_case("spacebar")
}

fn is_expansion_toggle(blueprint: &NativeWidgetBlueprint) -> bool {
    matches!(
        blueprint.role,
        crate::native::NativeRole::Disclosure
            | crate::native::NativeRole::DisclosureSummary
            | crate::native::NativeRole::Popover
    ) || blueprint.control_state.expanded.is_some()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "event/conformance_tests.rs"]
mod conformance_tests;
