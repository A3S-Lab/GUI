use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::{NativeEventContext, NativeInputModality, NativeKeyModifiers};
use crate::native::{
    is_number_input_type, number_field_wheel_step_direction, NativeRole,
    NUMBER_FIELD_INPUT_METADATA_KEY, NUMBER_FIELD_STEP_METADATA_KEY,
    NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY,
};
use crate::platform::{NativeWidgetBlueprint, NativeWidgetSetter};

use super::move_interaction::PointerMoveState;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct NativeInteractionSubscriptions {
    pub(crate) terminal_press: bool,
    pub(crate) press_lifecycle: bool,
    pub(crate) long_press: bool,
    pub(crate) movement: bool,
    pub(crate) hover: bool,
    pub(crate) key_down: bool,
    pub(crate) key_up: bool,
}

impl NativeInteractionSubscriptions {
    pub(crate) fn from_blueprint(blueprint: &NativeWidgetBlueprint) -> Self {
        let mut subscriptions = Self::from_events(
            &blueprint.events,
            blueprint
                .action
                .as_deref()
                .is_some_and(|action| !action.is_empty()),
        );
        subscriptions.merge(Self::from_style(&blueprint.portable_style));
        subscriptions.merge(Self::from_metadata(&blueprint.metadata));
        subscriptions.terminal_press |= has_collection_action(blueprint);
        subscriptions.long_press |= has_collection_action(blueprint);
        subscriptions
    }

    fn from_style(style: &crate::style::PortableStyle) -> Self {
        let requirements = style.interaction_requirements();
        Self {
            terminal_press: false,
            press_lifecycle: requirements.press,
            long_press: requirements.long_press,
            movement: requirements.movement,
            hover: requirements.hover,
            key_down: requirements.keyboard_modality,
            key_up: requirements.keyboard_modality,
        }
    }

    fn from_metadata(metadata: &BTreeMap<String, String>) -> Self {
        let captures_overlay_events = metadata
            .get(crate::overlay::OVERLAY_CAPTURE_METADATA_KEY)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"));
        Self {
            press_lifecycle: captures_overlay_events,
            key_down: captures_overlay_events,
            ..Self::default()
        }
    }

    fn from_events(events: &BTreeMap<String, String>, has_action: bool) -> Self {
        Self {
            terminal_press: has_action || has_event(events, &["onPress", "onClick"]),
            press_lifecycle: has_event(
                events,
                &["onPressStart", "onPressUp", "onPressEnd", "onPressChange"],
            ),
            long_press: has_event(
                events,
                &["onLongPressStart", "onLongPressEnd", "onLongPress"],
            ),
            movement: has_event(events, &["onMoveStart", "onMove", "onMoveEnd"]),
            hover: has_event(events, &["onHoverStart", "onHoverEnd", "onHoverChange"]),
            key_down: has_event(events, &["onKeyDown"]),
            key_up: has_event(events, &["onKeyUp"]),
        }
    }

    pub(crate) fn tracks_press(self) -> bool {
        self.terminal_press || self.press_lifecycle || self.long_press
    }

    fn merge(&mut self, other: Self) {
        self.terminal_press |= other.terminal_press;
        self.press_lifecycle |= other.press_lifecycle;
        self.long_press |= other.long_press;
        self.movement |= other.movement;
        self.hover |= other.hover;
        self.key_down |= other.key_down;
        self.key_up |= other.key_up;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum NativeLongPressMode {
    #[default]
    Disabled,
    AnyPointer,
    TouchOrPen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeLongPressConfig {
    mode: NativeLongPressMode,
    threshold: Duration,
}

impl NativeLongPressConfig {
    pub(crate) const fn disabled() -> Self {
        Self {
            mode: NativeLongPressMode::Disabled,
            threshold: DEFAULT_LONG_PRESS_THRESHOLD,
        }
    }

    fn accepts(self, modality: NativeInputModality) -> bool {
        match self.mode {
            NativeLongPressMode::Disabled => false,
            NativeLongPressMode::AnyPointer => matches!(
                modality,
                NativeInputModality::Mouse | NativeInputModality::Touch | NativeInputModality::Pen
            ),
            NativeLongPressMode::TouchOrPen => {
                matches!(
                    modality,
                    NativeInputModality::Touch | NativeInputModality::Pen
                )
            }
        }
    }
}

const DEFAULT_LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(500);
const LONG_PRESS_ACTIVE: u8 = 0;
const LONG_PRESS_RECOGNIZED: u8 = 1;
const LONG_PRESS_CANCELLED: u8 = 2;
const NUMBER_FIELD_STEPPER_ACTIVE: u8 = 0;
const NUMBER_FIELD_STEPPER_SPINNING: u8 = 1;
const NUMBER_FIELD_STEPPER_CANCELLED: u8 = 2;
const NUMBER_FIELD_STEPPER_SPUN_CANCELLED: u8 = 3;
const NUMBER_FIELD_STEPPER_POINTER_DELAY: Duration = Duration::from_millis(400);
const NUMBER_FIELD_STEPPER_TOUCH_DELAY: Duration = Duration::from_millis(600);
const NUMBER_FIELD_STEPPER_REPEAT_INTERVAL: Duration = Duration::from_millis(60);

#[derive(Debug, Clone)]
pub(crate) struct NativeLongPressTimer {
    threshold: Duration,
    node: HostNodeId,
    context: NativeEventContext,
    state: Arc<AtomicU8>,
}

#[derive(Debug, Clone)]
pub(crate) struct NativeLongPressRecognition {
    node: HostNodeId,
    context: NativeEventContext,
}

impl NativeLongPressRecognition {
    pub(crate) fn node(&self) -> HostNodeId {
        self.node
    }

    pub(crate) fn context(&self) -> NativeEventContext {
        self.context
    }

    pub(crate) fn cancellation_events(&self) -> [NativeEvent; 2] {
        [
            event(self.node, NativeEventKind::LongPressEnd, self.context),
            event(self.node, NativeEventKind::PressCancel, self.context),
        ]
    }

    pub(crate) fn terminal_event(&self) -> NativeEvent {
        event(self.node, NativeEventKind::LongPress, self.context)
    }

    pub(crate) fn into_events(self) -> Vec<NativeEvent> {
        let mut events = Vec::from(self.cancellation_events());
        events.push(self.terminal_event());
        events
    }

    pub(crate) fn into_events_with_movement(
        self,
        movement: &mut PointerMoveState,
    ) -> Vec<NativeEvent> {
        let mut events = Vec::from(self.cancellation_events());
        events.extend(movement.cancel(self.node, self.context));
        events.push(self.terminal_event());
        events
    }
}

impl NativeLongPressTimer {
    pub(crate) fn threshold(&self) -> Duration {
        self.threshold
    }

    pub(crate) fn try_fire(&self) -> Option<NativeLongPressRecognition> {
        self.state
            .compare_exchange(
                LONG_PRESS_ACTIVE,
                LONG_PRESS_RECOGNIZED,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
            .then(|| NativeLongPressRecognition {
                node: self.node,
                context: self.context,
            })
    }
}

/// A cancellable UI-thread timer for a NumberField stepper press.
///
/// The first callback runs after the modality-specific hold delay. Native
/// adapters then invoke `try_fire` every `repeat_interval` until it returns
/// `None`.
#[derive(Debug, Clone)]
pub(crate) struct NumberFieldStepperTimer {
    initial_delay: Duration,
    node: HostNodeId,
    context: NativeEventContext,
    state: Arc<AtomicU8>,
}

impl NumberFieldStepperTimer {
    pub(crate) fn initial_delay(&self) -> Duration {
        self.initial_delay
    }

    pub(crate) fn repeat_interval(&self) -> Duration {
        NUMBER_FIELD_STEPPER_REPEAT_INTERVAL
    }

    pub(crate) fn try_fire(&self, enabled: bool) -> Option<NativeEvent> {
        if !enabled {
            cancel_number_field_stepper(&self.state);
            return None;
        }

        loop {
            match self.state.load(Ordering::Acquire) {
                NUMBER_FIELD_STEPPER_ACTIVE => {
                    if self
                        .state
                        .compare_exchange(
                            NUMBER_FIELD_STEPPER_ACTIVE,
                            NUMBER_FIELD_STEPPER_SPINNING,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_err()
                    {
                        continue;
                    }
                    break;
                }
                NUMBER_FIELD_STEPPER_SPINNING => break,
                NUMBER_FIELD_STEPPER_CANCELLED | NUMBER_FIELD_STEPPER_SPUN_CANCELLED => {
                    return None;
                }
                _ => return None,
            }
        }

        Some(event(
            self.node,
            NativeEventKind::Press,
            self.context.repeat(true),
        ))
    }
}

/// Tracks React Aria compatible pointer-hold stepping independently of the
/// native backend's timer implementation.
#[derive(Debug, Default)]
pub(crate) struct NumberFieldStepperPressState {
    active: bool,
    over_target: bool,
    modality: NativeInputModality,
    timer_state: Option<Arc<AtomicU8>>,
    timer_taken: bool,
}

impl NumberFieldStepperPressState {
    pub(crate) fn begin(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        self.cancel();
        self.active = true;
        self.over_target = true;
        self.modality = context.modality;
        self.start_cycle();
        self.immediate_events(node, context)
    }

    pub(crate) fn take_timer(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Option<NumberFieldStepperTimer> {
        if self.timer_taken || !self.active || !self.over_target {
            return None;
        }
        let state = Arc::clone(self.timer_state.as_ref()?);
        self.timer_taken = true;
        let initial_delay = if self.modality == NativeInputModality::Touch {
            NUMBER_FIELD_STEPPER_TOUCH_DELAY
        } else {
            NUMBER_FIELD_STEPPER_POINTER_DELAY
        };
        Some(NumberFieldStepperTimer {
            initial_delay,
            node,
            context,
            state,
        })
    }

    pub(crate) fn enter(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if !self.active || self.over_target {
            return Vec::new();
        }
        self.over_target = true;
        self.modality = context.modality;
        self.start_cycle();
        self.immediate_events(node, context)
    }

    pub(crate) fn leave(&mut self) {
        if !self.active || !self.over_target {
            return;
        }
        self.over_target = false;
        self.end_cycle();
    }

    pub(crate) fn release(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if !self.active {
            return Vec::new();
        }
        let emit_touch_tap = self.over_target
            && self.modality == NativeInputModality::Touch
            && self
                .timer_state
                .as_ref()
                .is_some_and(|state| state.load(Ordering::Acquire) == NUMBER_FIELD_STEPPER_ACTIVE);
        self.cancel();
        if emit_touch_tap {
            vec![event(node, NativeEventKind::Press, context)]
        } else {
            Vec::new()
        }
    }

    pub(crate) fn cancel(&mut self) {
        self.active = false;
        self.over_target = false;
        self.modality = NativeInputModality::Unknown;
        self.end_cycle();
    }

    fn immediate_events(&self, node: HostNodeId, context: NativeEventContext) -> Vec<NativeEvent> {
        if self.modality == NativeInputModality::Touch {
            Vec::new()
        } else {
            vec![event(node, NativeEventKind::Press, context)]
        }
    }

    fn start_cycle(&mut self) {
        self.end_cycle();
        self.timer_state = Some(Arc::new(AtomicU8::new(NUMBER_FIELD_STEPPER_ACTIVE)));
        self.timer_taken = false;
    }

    fn end_cycle(&mut self) {
        if let Some(state) = self.timer_state.take() {
            cancel_number_field_stepper(&state);
        }
        self.timer_taken = false;
    }
}

impl Drop for NumberFieldStepperPressState {
    fn drop(&mut self) {
        self.cancel();
    }
}

fn cancel_number_field_stepper(state: &AtomicU8) {
    loop {
        let current = state.load(Ordering::Acquire);
        let cancelled = match current {
            NUMBER_FIELD_STEPPER_ACTIVE => NUMBER_FIELD_STEPPER_CANCELLED,
            NUMBER_FIELD_STEPPER_SPINNING => NUMBER_FIELD_STEPPER_SPUN_CANCELLED,
            NUMBER_FIELD_STEPPER_CANCELLED | NUMBER_FIELD_STEPPER_SPUN_CANCELLED => return,
            _ => return,
        };
        if state
            .compare_exchange(current, cancelled, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return;
        }
    }
}

/// The portable interaction contract retained by a mounted native widget.
///
/// Event callbacks can change without remounting, so platform adapters update
/// this profile from `SetAction` and `SetEvents` instead of capturing the
/// initial blueprint forever.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeInteractionProfile {
    pub(crate) role: NativeRole,
    pub(crate) subscriptions: NativeInteractionSubscriptions,
    event_subscriptions: NativeInteractionSubscriptions,
    style_subscriptions: NativeInteractionSubscriptions,
    overlay_subscriptions: NativeInteractionSubscriptions,
    has_action: bool,
    has_terminal_event: bool,
    has_long_press_event: bool,
    has_collection_action: bool,
    number_field_stepper: bool,
    number_field_input: bool,
    number_field_wheel_disabled: bool,
    number_input: bool,
    enabled: bool,
    read_only: bool,
    long_press_threshold: Duration,
}

impl NativeInteractionProfile {
    pub(crate) fn from_blueprint(blueprint: &NativeWidgetBlueprint) -> Self {
        let has_action = blueprint
            .action
            .as_deref()
            .is_some_and(|action| !action.is_empty());
        let has_terminal_event = has_event(&blueprint.events, &["onPress", "onClick"]);
        let has_long_press_event = has_event(
            &blueprint.events,
            &["onLongPressStart", "onLongPressEnd", "onLongPress"],
        );
        let has_collection_action = has_collection_action(blueprint);
        let number_field_stepper = is_number_field_stepper(blueprint);
        let number_field_input = blueprint
            .metadata
            .get(NUMBER_FIELD_INPUT_METADATA_KEY)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"));
        let number_field_wheel_disabled = blueprint
            .metadata
            .get(NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"));
        let number_input = is_number_input_type(blueprint.control_state.input_type.as_deref());
        let event_subscriptions = NativeInteractionSubscriptions::from_events(
            &blueprint.events,
            has_action || has_collection_action,
        );
        let style_subscriptions =
            NativeInteractionSubscriptions::from_style(&blueprint.portable_style);
        let overlay_subscriptions =
            NativeInteractionSubscriptions::from_metadata(&blueprint.metadata);
        let mut profile = Self {
            role: blueprint.role,
            subscriptions: NativeInteractionSubscriptions::default(),
            event_subscriptions,
            style_subscriptions,
            overlay_subscriptions,
            has_action,
            has_terminal_event,
            has_long_press_event,
            has_collection_action,
            number_field_stepper,
            number_field_input,
            number_field_wheel_disabled,
            number_input,
            enabled: !blueprint.control_state.disabled,
            read_only: blueprint.control_state.read_only,
            long_press_threshold: long_press_threshold(&blueprint.metadata),
        };
        profile.refresh_subscriptions();
        profile
    }

    pub(crate) fn apply_setter(&mut self, setter: &NativeWidgetSetter) {
        match setter {
            NativeWidgetSetter::SetAction(action) => {
                self.has_action = action.as_deref().is_some_and(|action| !action.is_empty());
                self.event_subscriptions.terminal_press =
                    self.has_action || self.has_terminal_event || self.has_collection_action;
                self.refresh_subscriptions();
            }
            NativeWidgetSetter::SetEvents(events) => {
                self.has_terminal_event = has_event(events, &["onPress", "onClick"]);
                self.has_long_press_event = has_event(
                    events,
                    &["onLongPressStart", "onLongPressEnd", "onLongPress"],
                );
                self.event_subscriptions = NativeInteractionSubscriptions::from_events(
                    events,
                    self.has_action || self.has_collection_action,
                );
                self.refresh_subscriptions();
            }
            NativeWidgetSetter::SetMetadata(metadata) => {
                self.has_collection_action = metadata
                    .get(crate::selection::COLLECTION_ACTION_METADATA_KEY)
                    .is_some_and(|value| value.eq_ignore_ascii_case("true"));
                self.long_press_threshold = long_press_threshold(metadata);
                self.number_field_stepper = metadata
                    .get(NUMBER_FIELD_STEP_METADATA_KEY)
                    .is_some_and(|value| matches!(value.as_str(), "increment" | "decrement"));
                self.number_field_input = metadata
                    .get(NUMBER_FIELD_INPUT_METADATA_KEY)
                    .is_some_and(|value| value.eq_ignore_ascii_case("true"));
                self.number_field_wheel_disabled = metadata
                    .get(NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY)
                    .is_some_and(|value| value.eq_ignore_ascii_case("true"));
                self.overlay_subscriptions =
                    NativeInteractionSubscriptions::from_metadata(metadata);
                self.refresh_subscriptions();
            }
            NativeWidgetSetter::SetInputType(input_type) => {
                self.number_input = is_number_input_type(input_type.as_deref());
            }
            NativeWidgetSetter::SetPortableStyle(style) => {
                self.style_subscriptions = NativeInteractionSubscriptions::from_style(style);
                self.refresh_subscriptions();
            }
            NativeWidgetSetter::SetEnabled(enabled) => {
                self.enabled = *enabled;
            }
            NativeWidgetSetter::SetReadOnly(read_only) => {
                self.read_only = *read_only;
            }
            _ => {}
        }
    }

    pub(crate) fn normalizes_keyboard_press(self) -> bool {
        self.subscriptions.tracks_press()
            && matches!(
                self.role,
                NativeRole::Button
                    | NativeRole::DisclosureSummary
                    | NativeRole::Link
                    | NativeRole::ImageMapArea
                    | NativeRole::MenuItem
                    | NativeRole::ListBoxItem
                    | NativeRole::TreeItem
            )
    }

    pub(crate) fn long_press_config(self) -> NativeLongPressConfig {
        let mode = if self.has_long_press_event || self.style_subscriptions.long_press {
            NativeLongPressMode::AnyPointer
        } else if self.has_collection_action {
            NativeLongPressMode::TouchOrPen
        } else {
            NativeLongPressMode::Disabled
        };
        NativeLongPressConfig {
            mode,
            threshold: self.long_press_threshold,
        }
    }

    pub(crate) fn tracks_movement(self) -> bool {
        self.enabled && self.subscriptions.movement
    }

    pub(crate) fn is_number_field_stepper(self) -> bool {
        self.role == NativeRole::Button && self.number_field_stepper
    }

    pub(crate) fn tracks_number_field_stepper(self) -> bool {
        self.enabled
            && self.is_number_field_stepper()
            && (self.has_action || self.has_terminal_event)
    }

    pub(crate) fn handles_number_field_step_key(
        self,
        kind: NativeEventKind,
        key: &str,
        modifiers: NativeKeyModifiers,
    ) -> bool {
        self.enabled
            && self.number_field_input
            && self.number_input
            && kind == NativeEventKind::KeyDown
            && modifiers.is_empty()
            && matches!(
                super::native_key_value(key).as_str(),
                "ArrowUp" | "ArrowDown" | "PageUp" | "PageDown" | "Home" | "End"
            )
    }

    pub(crate) fn handles_number_field_wheel(
        self,
        focused: bool,
        delta_x: f64,
        delta_y: f64,
        modifiers: NativeKeyModifiers,
    ) -> bool {
        self.enabled
            && !self.read_only
            && self.number_field_input
            && self.number_input
            && !self.number_field_wheel_disabled
            && focused
            && !modifiers.control
            && number_field_wheel_step_direction(delta_x, delta_y).is_some()
    }

    fn refresh_subscriptions(&mut self) {
        self.event_subscriptions.terminal_press =
            self.has_action || self.has_terminal_event || self.has_collection_action;
        self.event_subscriptions.long_press =
            self.has_long_press_event || self.has_collection_action;
        self.subscriptions = self.event_subscriptions;
        self.subscriptions.merge(self.style_subscriptions);
        self.subscriptions.merge(self.overlay_subscriptions);
    }
}

/// Tracks one pointer press independently of a platform widget toolkit.
///
/// Native adapters feed pointer boundary and release events into this state
/// machine so every backend emits the same semantic lifecycle ordering.
#[derive(Debug, Default)]
pub(crate) struct PointerPressState {
    active: bool,
    over_target: bool,
    start_emitted: bool,
    active_click_count: u8,
    last_completed: Option<(HostNodeId, Instant, u8)>,
    long_press_enabled: bool,
    long_press_started_at: Option<Instant>,
    long_press_threshold: Duration,
    long_press_state: Option<Arc<AtomicU8>>,
    long_press_timer_taken: bool,
    long_press_was_recognized: bool,
}

const MULTI_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// Tracks semantic keyboard presses across focus changes.
///
/// Native key-up delivery may target the newly focused widget, so adapters use
/// `target_for_key` to finish the lifecycle on the node where it started.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct KeyboardPressState {
    active: BTreeMap<HostNodeId, String>,
}

impl KeyboardPressState {
    pub(crate) fn target_for_key(&self, key: &str) -> Option<HostNodeId> {
        self.active
            .iter()
            .find_map(|(node, active_key)| (active_key == key).then_some(*node))
    }

    pub(crate) fn events(
        &mut self,
        node: HostNodeId,
        key: String,
        kind: NativeEventKind,
        context: NativeEventContext,
        role: NativeRole,
        tracks_press: bool,
    ) -> Vec<NativeEvent> {
        if !tracks_press || !super::is_press_activation_key(role, Some(&key)) {
            return vec![NativeEvent::new(node, kind).value(key).context(context)];
        }

        let handled_context = context.handled_activation(true);
        match kind {
            NativeEventKind::KeyDown => {
                let starts_press = !context.repeat && !self.active.contains_key(&node);
                if starts_press {
                    self.active.insert(node, key.clone());
                }
                let mut events = Vec::with_capacity(2);
                if starts_press {
                    events.push(
                        NativeEvent::new(node, NativeEventKind::PressStart)
                            .context(handled_context),
                    );
                }
                events.push(
                    NativeEvent::new(node, NativeEventKind::KeyDown)
                        .value(key)
                        .context(handled_context),
                );
                events
            }
            NativeEventKind::KeyUp => {
                let ended_press = self.active.get(&node).is_some_and(|active| active == &key);
                if ended_press {
                    self.active.remove(&node);
                }
                let mut events = Vec::with_capacity(4);
                if ended_press {
                    events.extend(
                        [
                            NativeEventKind::PressUp,
                            NativeEventKind::PressEnd,
                            NativeEventKind::Press,
                        ]
                        .into_iter()
                        .map(|kind| NativeEvent::new(node, kind).context(handled_context)),
                    );
                }
                events.push(
                    NativeEvent::new(node, NativeEventKind::KeyUp)
                        .value(key)
                        .context(handled_context),
                );
                events
            }
            _ => vec![NativeEvent::new(node, kind).value(key).context(context)],
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove(&mut self, node: HostNodeId) {
        self.active.remove(&node);
    }
}

impl PointerPressState {
    pub(crate) fn begin(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        self.begin_at(
            node,
            context,
            NativeLongPressConfig::disabled(),
            Instant::now(),
        )
    }

    pub(crate) fn begin_with_long_press(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
        config: NativeLongPressConfig,
    ) -> Vec<NativeEvent> {
        self.begin_at(node, context, config, Instant::now())
    }

    fn begin_at(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
        config: NativeLongPressConfig,
        now: Instant,
    ) -> Vec<NativeEvent> {
        if self.active {
            return Vec::new();
        }

        self.active = true;
        self.over_target = true;
        self.start_emitted = true;
        self.long_press_was_recognized = false;
        self.active_click_count = if context.click_count > 0 {
            context.click_count
        } else {
            self.next_click_count(node, now)
        };
        self.long_press_enabled = config.accepts(context.modality);
        if self.long_press_enabled {
            self.long_press_threshold = config.threshold;
            self.start_long_press_tracking(now);
        }
        context.click_count = self.active_click_count;
        let mut events = Vec::with_capacity(2);
        if self.long_press_started_at.is_some() {
            events.push(event(node, NativeEventKind::LongPressStart, context));
        }
        events.push(event(node, NativeEventKind::PressStart, context));
        events
    }

    pub(crate) fn take_long_press_timer(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
    ) -> Option<NativeLongPressTimer> {
        if self.long_press_timer_taken {
            return None;
        }
        let state = Arc::clone(self.long_press_state.as_ref()?);
        self.long_press_timer_taken = true;
        context.click_count = self.active_click_count;
        Some(NativeLongPressTimer {
            threshold: self.long_press_threshold,
            node,
            context,
            state,
        })
    }

    pub(crate) fn long_press_recognized(&self) -> bool {
        self.long_press_was_recognized
            || self
                .long_press_state
                .as_ref()
                .is_some_and(|state| state.load(Ordering::Acquire) == LONG_PRESS_RECOGNIZED)
    }

    pub(crate) fn enter(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        self.enter_at(node, context, Instant::now())
    }

    fn enter_at(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
        now: Instant,
    ) -> Vec<NativeEvent> {
        if !self.active || self.over_target || self.long_press_recognized() {
            return Vec::new();
        }

        self.over_target = true;
        self.start_emitted = true;
        context.click_count = self.active_click_count;
        let mut events = Vec::with_capacity(2);
        if self.long_press_enabled {
            self.start_long_press_tracking(now);
            events.push(event(node, NativeEventKind::LongPressStart, context));
        }
        events.push(event(node, NativeEventKind::PressStart, context));
        events
    }

    pub(crate) fn leave(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if !self.active || !self.over_target || !self.start_emitted || self.long_press_recognized()
        {
            return Vec::new();
        }

        self.over_target = false;
        self.start_emitted = false;
        context.click_count = self.active_click_count;
        let mut events = Vec::with_capacity(2);
        if self.end_long_press_tracking() {
            events.push(event(node, NativeEventKind::LongPressEnd, context));
        }
        events.push(event(node, NativeEventKind::PressEnd, context));
        events
    }

    /// Finishes the active pointer interaction.
    ///
    /// `emit_press` is false for native controls that produce a separate
    /// activation callback (for example a WinUI button click). Those controls
    /// still emit `pressUp` and `pressEnd` here, then emit `press` from their
    /// native activation callback.
    pub(crate) fn release(
        &mut self,
        node: HostNodeId,
        context: NativeEventContext,
        emit_press: bool,
    ) -> Vec<NativeEvent> {
        self.release_at(node, context, emit_press, Instant::now())
    }

    fn release_at(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
        emit_press: bool,
        now: Instant,
    ) -> Vec<NativeEvent> {
        if !self.active {
            return Vec::new();
        }

        let over_target = self.over_target;
        let start_emitted = self.start_emitted;
        let click_count = self.active_click_count;
        let long_press_started = self.long_press_started_at;
        let elapsed = long_press_started.is_some_and(|started_at| {
            now.saturating_duration_since(started_at) >= self.long_press_threshold
        });
        let recognized = self.long_press_recognized();
        let emit_long_press = elapsed
            && !recognized
            && self.long_press_state.as_ref().is_some_and(|state| {
                state
                    .compare_exchange(
                        LONG_PRESS_ACTIVE,
                        LONG_PRESS_RECOGNIZED,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
            });
        let long_pressed = recognized || emit_long_press;
        self.reset_active();

        if !over_target {
            return Vec::new();
        }

        context.click_count = click_count;
        self.last_completed = (!long_pressed).then_some((node, now, click_count));

        let mut events = Vec::with_capacity(5);
        if emit_long_press {
            events.push(event(node, NativeEventKind::LongPressEnd, context));
            events.push(event(node, NativeEventKind::PressCancel, context));
            events.push(event(node, NativeEventKind::LongPress, context));
            return events;
        }
        if recognized {
            return events;
        }
        if long_press_started.is_some() {
            events.push(event(node, NativeEventKind::LongPressEnd, context));
        }
        events.push(event(node, NativeEventKind::PressUp, context));
        if start_emitted {
            events.push(event(node, NativeEventKind::PressEnd, context));
            if emit_press && !long_pressed {
                events.push(event(node, NativeEventKind::Press, context));
            }
        }
        events
    }

    pub(crate) fn cancel(
        &mut self,
        node: HostNodeId,
        mut context: NativeEventContext,
    ) -> Vec<NativeEvent> {
        if !self.active {
            return Vec::new();
        }

        let recognized = self.long_press_recognized();
        let start_emitted = self.start_emitted;
        let long_press_started = self.long_press_started_at.is_some();
        context.click_count = self.active_click_count;
        self.reset_active();
        if recognized {
            return Vec::new();
        }
        let mut events = Vec::with_capacity(2);
        if long_press_started {
            events.push(event(node, NativeEventKind::LongPressEnd, context));
        }
        if start_emitted {
            events.push(event(node, NativeEventKind::PressCancel, context));
        }
        events
    }

    #[cfg(test)]
    pub(crate) fn is_active(self) -> bool {
        self.active
    }

    fn next_click_count(&self, node: HostNodeId, now: Instant) -> u8 {
        let Some((last_node, last_at, last_count)) = self.last_completed else {
            return 1;
        };
        if last_node != node || now.saturating_duration_since(last_at) > MULTI_CLICK_INTERVAL {
            return 1;
        }
        if last_count == 1 {
            2
        } else {
            1
        }
    }

    fn reset_active(&mut self) {
        self.active = false;
        self.over_target = false;
        self.start_emitted = false;
        self.active_click_count = 0;
        self.end_long_press_tracking();
        self.long_press_enabled = false;
        self.long_press_threshold = Duration::ZERO;
        self.long_press_was_recognized = false;
    }

    fn end_long_press_tracking(&mut self) -> bool {
        let tracked = self.long_press_started_at.take().is_some();
        if let Some(state) = self.long_press_state.take() {
            self.long_press_was_recognized |=
                state.load(Ordering::Acquire) == LONG_PRESS_RECOGNIZED;
            let _ = state.compare_exchange(
                LONG_PRESS_ACTIVE,
                LONG_PRESS_CANCELLED,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
        }
        self.long_press_timer_taken = false;
        tracked
    }

    fn start_long_press_tracking(&mut self, now: Instant) {
        self.long_press_started_at = Some(now);
        self.long_press_state = Some(Arc::new(AtomicU8::new(LONG_PRESS_ACTIVE)));
        self.long_press_timer_taken = false;
        self.long_press_was_recognized = false;
    }
}

impl Drop for PointerPressState {
    fn drop(&mut self) {
        self.end_long_press_tracking();
    }
}

pub(crate) fn virtual_press_events(node: HostNodeId) -> Vec<NativeEvent> {
    let context = NativeEventContext::new().modality(NativeInputModality::Virtual);
    [
        NativeEventKind::PressStart,
        NativeEventKind::PressUp,
        NativeEventKind::PressEnd,
        NativeEventKind::Press,
    ]
    .into_iter()
    .map(|kind| event(node, kind, context))
    .collect()
}

fn event(node: HostNodeId, kind: NativeEventKind, context: NativeEventContext) -> NativeEvent {
    NativeEvent::new(node, kind).context(context)
}

fn has_event(events: &BTreeMap<String, String>, names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| events.get(*name).is_some_and(|action| !action.is_empty()))
}

fn has_collection_action(blueprint: &NativeWidgetBlueprint) -> bool {
    blueprint
        .metadata
        .get(crate::selection::COLLECTION_ACTION_METADATA_KEY)
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn is_number_field_stepper(blueprint: &NativeWidgetBlueprint) -> bool {
    blueprint.role == NativeRole::Button
        && blueprint
            .metadata
            .get(NUMBER_FIELD_STEP_METADATA_KEY)
            .is_some_and(|value| matches!(value.as_str(), "increment" | "decrement"))
}

fn long_press_threshold(metadata: &BTreeMap<String, String>) -> Duration {
    ["threshold", "data-long-press-threshold"]
        .into_iter()
        .find_map(|name| metadata.get(name))
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .map(|value| Duration::from_millis(value.min(60_000)))
        .unwrap_or(DEFAULT_LONG_PRESS_THRESHOLD)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{AppKitAdapter, Gtk4Adapter, PlatformAdapter, WinUiAdapter};
    use crate::web::WebProps;

    fn kinds(events: &[NativeEvent]) -> Vec<NativeEventKind> {
        events.iter().map(|event| event.kind).collect()
    }

    #[test]
    fn pointer_release_over_target_has_portable_lifecycle_order() {
        let node = HostNodeId::new(3);
        let context = NativeEventContext::new().modality(NativeInputModality::Touch);
        let mut state = PointerPressState::default();

        assert_eq!(
            kinds(&state.begin(node, context)),
            vec![NativeEventKind::PressStart]
        );
        assert_eq!(
            kinds(&state.release(node, context, true)),
            vec![
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press
            ]
        );
        assert!(!state.is_active());
    }

    #[test]
    fn number_field_mouse_stepper_fires_immediately_then_repeats() {
        let node = HostNodeId::new(20);
        let context = NativeEventContext::new().modality(NativeInputModality::Mouse);
        let mut state = NumberFieldStepperPressState::default();

        assert_eq!(
            kinds(&state.begin(node, context)),
            vec![NativeEventKind::Press]
        );
        let timer = state.take_timer(node, context).unwrap();
        assert_eq!(timer.initial_delay(), Duration::from_millis(400));
        assert_eq!(timer.repeat_interval(), Duration::from_millis(60));
        assert!(state.take_timer(node, context).is_none());

        let first_repeat = timer.try_fire(true).unwrap();
        assert_eq!(first_repeat.kind, NativeEventKind::Press);
        assert!(first_repeat.context.repeat);
        assert!(timer.try_fire(true).unwrap().context.repeat);

        assert!(state.release(node, context).is_empty());
        assert!(timer.try_fire(true).is_none());
    }

    #[test]
    fn number_field_touch_stepper_defers_taps_and_suppresses_them_after_spinning() {
        let node = HostNodeId::new(21);
        let context = NativeEventContext::new().modality(NativeInputModality::Touch);
        let mut state = NumberFieldStepperPressState::default();

        assert!(state.begin(node, context).is_empty());
        let tap_timer = state.take_timer(node, context).unwrap();
        assert_eq!(tap_timer.initial_delay(), Duration::from_millis(600));
        assert_eq!(
            kinds(&state.release(node, context)),
            vec![NativeEventKind::Press]
        );
        assert!(tap_timer.try_fire(true).is_none());

        state.begin(node, context);
        let spinning = state.take_timer(node, context).unwrap();
        assert_eq!(
            spinning.try_fire(true).unwrap().kind,
            NativeEventKind::Press
        );
        assert!(state.release(node, context).is_empty());
        assert!(spinning.try_fire(true).is_none());

        state.begin(node, context);
        let disabled = state.take_timer(node, context).unwrap();
        assert!(disabled.try_fire(false).is_none());
        assert!(state.release(node, context).is_empty());
    }

    #[test]
    fn number_field_stepper_cancels_outside_and_restarts_on_reentry() {
        let node = HostNodeId::new(22);
        let context = NativeEventContext::new().modality(NativeInputModality::Mouse);
        let mut state = NumberFieldStepperPressState::default();

        state.begin(node, context);
        let cancelled = state.take_timer(node, context).unwrap();
        state.leave();
        assert!(cancelled.try_fire(true).is_none());

        assert_eq!(
            kinds(&state.enter(node, context)),
            vec![NativeEventKind::Press]
        );
        let restarted = state.take_timer(node, context).unwrap();
        assert!(restarted.try_fire(true).is_some());
        state.cancel();
        assert!(restarted.try_fire(true).is_none());
    }

    #[test]
    fn long_press_uses_the_configured_threshold_without_sleeping() {
        let node = HostNodeId::new(30);
        let context = NativeEventContext::new().modality(NativeInputModality::Touch);
        let config = NativeLongPressConfig {
            mode: NativeLongPressMode::AnyPointer,
            threshold: Duration::from_millis(500),
        };
        let now = Instant::now();
        let mut state = PointerPressState::default();

        assert_eq!(
            kinds(&state.begin_at(node, context, config, now)),
            vec![NativeEventKind::LongPressStart, NativeEventKind::PressStart]
        );
        assert_eq!(
            kinds(&state.release_at(node, context, true, now + Duration::from_millis(499),)),
            vec![
                NativeEventKind::LongPressEnd,
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
            ]
        );

        let later = now + Duration::from_secs(1);
        state.begin_at(node, context, config, later);
        assert_eq!(
            kinds(&state.release_at(node, context, true, later + Duration::from_millis(500),)),
            vec![
                NativeEventKind::LongPressEnd,
                NativeEventKind::PressCancel,
                NativeEventKind::LongPress,
            ]
        );
    }

    #[test]
    fn threshold_timer_emits_once_and_suppresses_the_terminal_press() {
        let node = HostNodeId::new(32);
        let context = NativeEventContext::new().modality(NativeInputModality::Touch);
        let config = NativeLongPressConfig {
            mode: NativeLongPressMode::AnyPointer,
            threshold: Duration::from_millis(500),
        };
        let now = Instant::now();
        let mut state = PointerPressState::default();

        state.begin_at(node, context, config, now);
        let timer = state.take_long_press_timer(node, context).unwrap();
        assert_eq!(timer.threshold(), Duration::from_millis(500));
        assert!(state.take_long_press_timer(node, context).is_none());
        assert_eq!(
            kinds(&timer.try_fire().unwrap().into_events()),
            vec![
                NativeEventKind::LongPressEnd,
                NativeEventKind::PressCancel,
                NativeEventKind::LongPress,
            ]
        );
        assert!(timer.try_fire().is_none());
        assert!(state.long_press_recognized());
        assert_eq!(
            kinds(&state.release_at(node, context, true, now + Duration::from_millis(100),)),
            Vec::<NativeEventKind>::new()
        );

        let reentry_start = now + Duration::from_secs(1);
        state.begin_at(node, context, config, reentry_start);
        state
            .take_long_press_timer(node, context)
            .unwrap()
            .try_fire()
            .unwrap();
        assert_eq!(
            kinds(&state.leave(node, context)),
            Vec::<NativeEventKind>::new()
        );
        assert_eq!(
            kinds(&state.enter(node, context)),
            Vec::<NativeEventKind>::new()
        );
        assert_eq!(
            kinds(&state.release_at(
                node,
                context,
                true,
                reentry_start + Duration::from_millis(100),
            )),
            Vec::<NativeEventKind>::new()
        );

        state.begin_at(node, context, config, now + Duration::from_secs(2));
        let cancelled = state.take_long_press_timer(node, context).unwrap();
        state.cancel(node, context);
        assert!(cancelled.try_fire().is_none());
    }

    #[test]
    fn long_press_recognition_cancels_active_movement_before_terminal_event() {
        let node = HostNodeId::new(33);
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Touch)
            .position(10.0, 20.0);
        let config = NativeLongPressConfig {
            mode: NativeLongPressMode::AnyPointer,
            threshold: Duration::from_millis(500),
        };
        let mut press = PointerPressState::default();
        let mut movement = PointerMoveState::default();

        press.begin_at(node, context, config, Instant::now());
        let timer = press.take_long_press_timer(node, context).unwrap();
        movement.begin(context);
        assert_eq!(
            kinds(&movement.update(node, context.position(14.0, 20.0))),
            vec![NativeEventKind::MoveStart, NativeEventKind::Move]
        );

        assert_eq!(
            kinds(
                &timer
                    .try_fire()
                    .unwrap()
                    .into_events_with_movement(&mut movement)
            ),
            vec![
                NativeEventKind::LongPressEnd,
                NativeEventKind::PressCancel,
                NativeEventKind::MoveEnd,
                NativeEventKind::LongPress,
            ]
        );
        assert!(movement
            .update(node, context.position(18.0, 20.0))
            .is_empty());
    }

    #[test]
    fn collection_long_press_only_tracks_touch_and_pen_and_cancels_on_leave() {
        let node = HostNodeId::new(31);
        let config = NativeLongPressConfig {
            mode: NativeLongPressMode::TouchOrPen,
            threshold: Duration::from_millis(500),
        };
        let now = Instant::now();
        let mut state = PointerPressState::default();
        let mouse = NativeEventContext::new().modality(NativeInputModality::Mouse);

        assert_eq!(
            kinds(&state.begin_at(node, mouse, config, now)),
            vec![NativeEventKind::PressStart]
        );
        state.cancel(node, mouse);

        let pen = NativeEventContext::new().modality(NativeInputModality::Pen);
        assert_eq!(
            kinds(&state.begin_at(node, pen, config, now)),
            vec![NativeEventKind::LongPressStart, NativeEventKind::PressStart]
        );
        assert_eq!(
            kinds(&state.leave(node, pen)),
            vec![NativeEventKind::LongPressEnd, NativeEventKind::PressEnd]
        );
        assert_eq!(
            kinds(&state.enter_at(node, pen, now + Duration::from_millis(100),)),
            vec![NativeEventKind::LongPressStart, NativeEventKind::PressStart]
        );
        assert_eq!(
            kinds(&state.release_at(node, pen, true, now + Duration::from_millis(200),)),
            vec![
                NativeEventKind::LongPressEnd,
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
            ]
        );
    }

    #[test]
    fn pointer_press_state_normalizes_single_and_double_click_counts() {
        let node = HostNodeId::new(4);
        let context = NativeEventContext::new().modality(NativeInputModality::Mouse);
        let mut state = PointerPressState::default();

        let first_start = state.begin(node, context);
        let first_end = state.release(node, context, true);
        let second_start = state.begin(node, context);
        let second_end = state.release(node, context, true);

        assert_eq!(first_start[0].context.click_count, 1);
        assert!(first_end.iter().all(|event| event.context.click_count == 1));
        assert_eq!(second_start[0].context.click_count, 2);
        assert!(second_end
            .iter()
            .all(|event| event.context.click_count == 2));
    }

    #[test]
    fn pointer_can_end_on_leave_and_restart_on_reentry() {
        let node = HostNodeId::new(5);
        let context = NativeEventContext::new().modality(NativeInputModality::Mouse);
        let mut state = PointerPressState::default();

        state.begin(node, context);
        assert_eq!(
            kinds(&state.leave(node, context)),
            vec![NativeEventKind::PressEnd]
        );
        assert_eq!(
            kinds(&state.enter(node, context)),
            vec![NativeEventKind::PressStart]
        );
        assert_eq!(
            kinds(&state.release(node, context, false)),
            vec![NativeEventKind::PressUp, NativeEventKind::PressEnd]
        );
    }

    #[test]
    fn release_outside_does_not_activate_and_cancel_ends_an_active_press() {
        let node = HostNodeId::new(7);
        let context = NativeEventContext::new().modality(NativeInputModality::Pen);
        let mut state = PointerPressState::default();

        state.begin(node, context);
        state.leave(node, context);
        assert!(state.release(node, context, true).is_empty());

        state.begin(node, context);
        assert_eq!(
            kinds(&state.cancel(node, context)),
            vec![NativeEventKind::PressCancel]
        );
    }

    #[test]
    fn virtual_activation_emits_the_complete_lifecycle() {
        let events = virtual_press_events(HostNodeId::new(9));
        assert_eq!(
            kinds(&events),
            vec![
                NativeEventKind::PressStart,
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press
            ]
        );
        assert!(events
            .iter()
            .all(|event| event.context.modality == NativeInputModality::Virtual));
    }

    #[test]
    fn subscriptions_are_derived_once_from_the_native_blueprint() {
        let element = NativeElement::new("target", NativeRole::Button).with_props(
            NativeProps::new().web(
                WebProps::new()
                    .on_press("activate")
                    .on_press_change("changePress")
                    .on_hover_end("leave")
                    .event("onMove", "moveTarget")
                    .on_key_up("releaseKey"),
            ),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let subscriptions = NativeInteractionSubscriptions::from_blueprint(&blueprint);

        assert!(subscriptions.terminal_press);
        assert!(subscriptions.press_lifecycle);
        assert!(subscriptions.tracks_press());
        assert!(subscriptions.hover);
        assert!(subscriptions.movement);
        assert!(!subscriptions.key_down);
        assert!(subscriptions.key_up);
    }

    #[test]
    fn style_variants_subscribe_to_the_native_events_that_drive_them() {
        let element = NativeElement::new("target", NativeRole::Button).with_props(
            NativeProps::new().web(WebProps::new().class_name(
                "hover:opacity-75 active:opacity-50 focus-visible:opacity-100 \
                 focus-within:opacity-90 data-[pressed=true]:opacity-25 \
                 data-[long-pressed=true]:opacity-60 data-[moving=true]:opacity-70",
            )),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let subscriptions = NativeInteractionSubscriptions::from_blueprint(&blueprint);

        assert!(subscriptions.hover);
        assert!(subscriptions.press_lifecycle);
        assert!(subscriptions.long_press);
        assert!(subscriptions.movement);
        assert!(subscriptions.key_down);
        assert!(subscriptions.key_up);
        assert!(!subscriptions.terminal_press);

        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);
        profile.apply_setter(&NativeWidgetSetter::SetPortableStyle(Box::new(
            crate::style::PortableStyle::default(),
        )));
        assert!(!profile.subscriptions.hover);
        assert!(!profile.subscriptions.press_lifecycle);
        assert!(!profile.subscriptions.long_press);
        assert!(!profile.subscriptions.movement);
        assert!(!profile.subscriptions.key_down);
    }

    #[test]
    fn mounted_interaction_profile_tracks_event_and_action_setters() {
        let element = NativeElement::new("target", NativeRole::Button)
            .with_props(NativeProps::new().action("activate"));
        let blueprint = AppKitAdapter.blueprint(&element);
        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);

        assert!(profile.subscriptions.terminal_press);
        profile.apply_setter(&NativeWidgetSetter::SetAction(None));
        assert!(!profile.subscriptions.terminal_press);

        profile.apply_setter(&NativeWidgetSetter::SetEvents(BTreeMap::from([
            ("onPressStart".to_string(), "start".to_string()),
            ("onHoverEnd".to_string(), "leave".to_string()),
            ("onMoveEnd".to_string(), "endMove".to_string()),
        ])));
        assert!(profile.subscriptions.press_lifecycle);
        assert!(profile.subscriptions.hover);
        assert!(profile.subscriptions.movement);
        assert!(!profile.subscriptions.terminal_press);
        assert!(profile.tracks_movement());

        profile.apply_setter(&NativeWidgetSetter::SetEnabled(false));
        assert!(!profile.tracks_movement());
        profile.apply_setter(&NativeWidgetSetter::SetEnabled(true));
        assert!(profile.tracks_movement());

        profile.apply_setter(&NativeWidgetSetter::SetAction(Some("activate".to_string())));
        assert!(profile.subscriptions.terminal_press);
        assert!(profile.normalizes_keyboard_press());
    }

    #[test]
    fn mounted_interaction_profile_tracks_number_field_stepper_state() {
        let element = NativeElement::new("increment", NativeRole::Button).with_props(
            NativeProps::new()
                .action("setQuantity")
                .metadata(NUMBER_FIELD_STEP_METADATA_KEY, "increment"),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);

        assert!(profile.is_number_field_stepper());
        assert!(profile.tracks_number_field_stepper());
        profile.apply_setter(&NativeWidgetSetter::SetEnabled(false));
        assert!(profile.is_number_field_stepper());
        assert!(!profile.tracks_number_field_stepper());
        profile.apply_setter(&NativeWidgetSetter::SetEnabled(true));
        assert!(profile.tracks_number_field_stepper());
        profile.apply_setter(&NativeWidgetSetter::SetMetadata(BTreeMap::new()));
        assert!(!profile.is_number_field_stepper());
        assert!(!profile.tracks_number_field_stepper());
    }

    #[test]
    fn number_field_profiles_claim_only_unmodified_portable_step_inputs() {
        let element = NativeElement::new("quantity", NativeRole::TextField).with_props(
            NativeProps::new()
                .input_type("number")
                .metadata(NUMBER_FIELD_INPUT_METADATA_KEY, "true"),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);

        let no_modifiers = NativeKeyModifiers::new();
        assert!(profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowUp",
            no_modifiers
        ));
        assert!(profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "Down",
            no_modifiers
        ));
        assert!(profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "Home",
            no_modifiers
        ));
        assert!(!profile.handles_number_field_step_key(
            NativeEventKind::KeyUp,
            "ArrowUp",
            no_modifiers
        ));
        assert!(!profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowLeft",
            no_modifiers
        ));
        assert!(!profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowUp",
            NativeKeyModifiers::new().shift(true)
        ));
        assert!(profile.handles_number_field_wheel(true, 0.0, 1.0, no_modifiers));
        assert!(!profile.handles_number_field_wheel(true, 2.0, 1.0, no_modifiers));
        assert!(!profile.handles_number_field_wheel(false, 0.0, 1.0, no_modifiers));
        assert!(!profile.handles_number_field_wheel(
            true,
            0.0,
            1.0,
            NativeKeyModifiers::new().control(true)
        ));
        profile.apply_setter(&NativeWidgetSetter::SetReadOnly(true));
        assert!(!profile.handles_number_field_wheel(true, 0.0, 1.0, no_modifiers));
        assert!(profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowUp",
            no_modifiers
        ));
        profile.apply_setter(&NativeWidgetSetter::SetReadOnly(false));
        profile.apply_setter(&NativeWidgetSetter::SetMetadata(BTreeMap::from([
            (
                NUMBER_FIELD_INPUT_METADATA_KEY.to_string(),
                "true".to_string(),
            ),
            (
                NUMBER_FIELD_WHEEL_DISABLED_METADATA_KEY.to_string(),
                "true".to_string(),
            ),
        ])));
        assert!(!profile.handles_number_field_wheel(true, 0.0, 1.0, no_modifiers));

        profile.apply_setter(&NativeWidgetSetter::SetInputType(Some("text".to_string())));
        assert!(!profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowUp",
            no_modifiers
        ));
        profile.apply_setter(&NativeWidgetSetter::SetInputType(Some(
            "number".to_string(),
        )));
        profile.apply_setter(&NativeWidgetSetter::SetMetadata(BTreeMap::new()));
        assert!(!profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            "ArrowUp",
            no_modifiers
        ));
    }

    #[test]
    fn collection_action_marker_subscribes_items_without_copying_the_callback() {
        let element = NativeElement::new("ada", NativeRole::ListBoxItem).with_props(
            NativeProps::new().metadata(crate::selection::COLLECTION_ACTION_METADATA_KEY, "true"),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);

        assert!(profile.subscriptions.terminal_press);
        assert!(profile.subscriptions.long_press);
        assert_eq!(
            profile.long_press_config().mode,
            NativeLongPressMode::TouchOrPen
        );
        assert!(profile.normalizes_keyboard_press());
        assert!(!blueprint.events.contains_key("onAction"));

        profile.apply_setter(&NativeWidgetSetter::SetMetadata(BTreeMap::new()));
        assert!(!profile.subscriptions.terminal_press);
    }

    #[test]
    fn overlay_capture_marker_subscribes_pointer_lifecycle_and_escape_delivery() {
        let element = NativeElement::new("overlay-capture", NativeRole::View).with_props(
            NativeProps::new().metadata(crate::overlay::OVERLAY_CAPTURE_METADATA_KEY, "true"),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let mut profile = NativeInteractionProfile::from_blueprint(&blueprint);

        assert!(profile.subscriptions.press_lifecycle);
        assert!(profile.subscriptions.key_down);
        assert!(!profile.subscriptions.terminal_press);

        profile.apply_setter(&NativeWidgetSetter::SetMetadata(BTreeMap::new()));
        assert!(!profile.subscriptions.press_lifecycle);
        assert!(!profile.subscriptions.key_down);
    }

    #[test]
    fn explicit_long_press_profile_honors_a_bounded_threshold() {
        let element = NativeElement::new("target", NativeRole::Button).with_props(
            NativeProps::new()
                .metadata("threshold", "75000")
                .web(WebProps::new().event("onLongPress", "openMenu")),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let profile = NativeInteractionProfile::from_blueprint(&blueprint);
        let config = profile.long_press_config();

        assert_eq!(config.mode, NativeLongPressMode::AnyPointer);
        assert_eq!(config.threshold, Duration::from_secs(60));
    }

    #[test]
    fn every_planning_adapter_preserves_collection_action_item_capture() {
        for role in [NativeRole::ListBoxItem, NativeRole::TreeItem] {
            let element = NativeElement::new("item", role).with_props(
                NativeProps::new()
                    .metadata(crate::selection::COLLECTION_ACTION_METADATA_KEY, "true"),
            );
            let blueprints = [
                AppKitAdapter.blueprint(&element),
                Gtk4Adapter.blueprint(&element),
                WinUiAdapter.blueprint(&element),
            ];

            for blueprint in blueprints {
                let profile = NativeInteractionProfile::from_blueprint(&blueprint);
                assert!(profile.subscriptions.tracks_press());
                assert!(profile.normalizes_keyboard_press());
            }
        }
    }

    #[test]
    fn keyboard_press_finishes_on_the_node_where_it_started() {
        let first = HostNodeId::new(11);
        let second = HostNodeId::new(12);
        let context = NativeEventContext::new().modality(NativeInputModality::Keyboard);
        let mut state = KeyboardPressState::default();

        let down = state.events(
            first,
            "Enter".to_string(),
            NativeEventKind::KeyDown,
            context,
            NativeRole::Button,
            true,
        );
        assert_eq!(
            kinds(&down),
            vec![NativeEventKind::PressStart, NativeEventKind::KeyDown]
        );
        assert_eq!(state.target_for_key("Enter"), Some(first));

        let unrelated = state.events(
            second,
            "Space".to_string(),
            NativeEventKind::KeyUp,
            context,
            NativeRole::Button,
            true,
        );
        assert_eq!(kinds(&unrelated), vec![NativeEventKind::KeyUp]);
        assert_eq!(state.target_for_key("Enter"), Some(first));

        let up = state.events(
            first,
            "Enter".to_string(),
            NativeEventKind::KeyUp,
            context,
            NativeRole::Button,
            true,
        );
        assert_eq!(
            kinds(&up),
            vec![
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
                NativeEventKind::KeyUp,
            ]
        );
        assert_eq!(state.target_for_key("Enter"), None);
    }

    #[test]
    fn keyboard_repeat_and_link_space_do_not_start_duplicate_presses() {
        let node = HostNodeId::new(13);
        let context = NativeEventContext::new().modality(NativeInputModality::Keyboard);
        let mut state = KeyboardPressState::default();

        state.events(
            node,
            "Enter".to_string(),
            NativeEventKind::KeyDown,
            context,
            NativeRole::Link,
            true,
        );
        let repeat = state.events(
            node,
            "Enter".to_string(),
            NativeEventKind::KeyDown,
            context.repeat(true),
            NativeRole::Link,
            true,
        );
        assert_eq!(kinds(&repeat), vec![NativeEventKind::KeyDown]);

        let link_space = state.events(
            HostNodeId::new(14),
            "Space".to_string(),
            NativeEventKind::KeyDown,
            context,
            NativeRole::Link,
            true,
        );
        assert_eq!(kinds(&link_space), vec![NativeEventKind::KeyDown]);
        assert!(!link_space[0].context.handled_activation);
    }

    #[test]
    fn disclosure_summary_uses_button_keyboard_activation() {
        let node = HostNodeId::new(15);
        let context = NativeEventContext::new().modality(NativeInputModality::Keyboard);
        let mut state = KeyboardPressState::default();

        let down = state.events(
            node,
            "Space".to_string(),
            NativeEventKind::KeyDown,
            context,
            NativeRole::DisclosureSummary,
            true,
        );
        let up = state.events(
            node,
            "Space".to_string(),
            NativeEventKind::KeyUp,
            context,
            NativeRole::DisclosureSummary,
            true,
        );

        assert_eq!(
            kinds(&down),
            vec![NativeEventKind::PressStart, NativeEventKind::KeyDown]
        );
        assert_eq!(
            kinds(&up),
            vec![
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
                NativeEventKind::KeyUp,
            ]
        );
    }
}
