use std::collections::{BTreeMap, BTreeSet};

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::NativeInputModality;
use crate::native::{NativeProps, NativeRole};
use crate::platform::NativeWidgetBlueprint;
use serde::{Deserialize, Serialize};

/// Maximum number of interaction changes retained for diagnostics by default.
pub const DEFAULT_INTERACTION_CHANGE_HISTORY_LIMIT: usize = 256;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionNodeState {
    pub focused: bool,
    #[serde(default)]
    pub focus_visible: bool,
    #[serde(default)]
    pub focus_within: bool,
    pub pressed: bool,
    #[serde(default)]
    pub long_pressed: bool,
    #[serde(default)]
    pub moving: bool,
    #[serde(default)]
    pub x_delta: f64,
    #[serde(default)]
    pub y_delta: f64,
    #[serde(default)]
    pub hovered: bool,
    pub value: Option<String>,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
}

impl std::fmt::Debug for InteractionNodeState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("InteractionNodeState")
            .field("focused", &self.focused)
            .field("focus_within", &self.focus_within)
            .field("pressed", &self.pressed)
            .field("has_value", &self.value.is_some())
            .field("selected", &self.selected)
            .field("checked", &self.checked)
            .field("expanded", &self.expanded)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionChange {
    pub node: HostNodeId,
    pub before: InteractionNodeState,
    pub after: InteractionNodeState,
}

#[derive(Debug, Clone)]
pub struct InteractionState {
    nodes: BTreeMap<HostNodeId, InteractionNodeState>,
    changes: Vec<InteractionChange>,
    change_history_limit: usize,
    next_change_sequence: u64,
    value_change_sequences: BTreeMap<HostNodeId, u64>,
    selection_change_sequences: BTreeMap<HostNodeId, u64>,
    focus_history: bool,
    input_modality: NativeInputModality,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            changes: Vec::new(),
            change_history_limit: DEFAULT_INTERACTION_CHANGE_HISTORY_LIMIT,
            next_change_sequence: 0,
            value_change_sequences: BTreeMap::new(),
            selection_change_sequences: BTreeMap::new(),
            focus_history: false,
            input_modality: NativeInputModality::Unknown,
        }
    }
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates interaction state with a bounded diagnostic change history.
    ///
    /// A limit of zero disables change history without affecting current interaction state.
    pub fn with_change_history_limit(change_history_limit: usize) -> Self {
        Self {
            change_history_limit,
            ..Self::default()
        }
    }

    pub fn node(&self, id: HostNodeId) -> Option<&InteractionNodeState> {
        self.nodes.get(&id)
    }

    pub fn has_focused_node(&self) -> bool {
        self.nodes.values().any(|state| state.focused)
    }

    pub fn focused_node(&self) -> Option<HostNodeId> {
        self.nodes
            .iter()
            .find_map(|(node, state)| state.focused.then_some(*node))
    }

    pub fn has_focus_history(&self) -> bool {
        self.focus_history
    }

    pub fn input_modality(&self) -> NativeInputModality {
        self.input_modality
    }

    pub fn changes(&self) -> &[InteractionChange] {
        &self.changes
    }

    pub fn change_history_limit(&self) -> usize {
        self.change_history_limit
    }

    /// Takes the retained diagnostic changes without clearing current node state.
    pub fn take_changes(&mut self) -> Vec<InteractionChange> {
        std::mem::take(&mut self.changes)
    }

    pub(crate) fn value_change_sequence(&self, id: HostNodeId) -> Option<u64> {
        self.value_change_sequences.get(&id).copied()
    }

    pub(crate) fn selection_change_sequence(&self, id: HostNodeId) -> Option<u64> {
        self.selection_change_sequences.get(&id).copied()
    }

    pub fn sync_node_from_blueprint(&mut self, id: HostNodeId, blueprint: &NativeWidgetBlueprint) {
        let transient_state = self.nodes.get(&id).map(|state| {
            (
                state.focused,
                state.focus_visible,
                state.focus_within,
                state.pressed,
                state.long_pressed,
                state.moving,
                state.x_delta,
                state.y_delta,
                state.hovered,
            )
        });
        let mut state = initial_state_from_blueprint(blueprint);
        if let Some((
            focused,
            focus_visible,
            focus_within,
            pressed,
            long_pressed,
            moving,
            x_delta,
            y_delta,
            hovered,
        )) = transient_state
        {
            state.focused = focused;
            state.focus_visible = focus_visible;
            state.focus_within = focus_within;
            state.pressed = pressed;
            state.long_pressed = long_pressed;
            state.moving = moving;
            state.x_delta = x_delta;
            state.y_delta = y_delta;
            state.hovered = hovered;
        }
        self.nodes.insert(id, state);
    }

    pub(crate) fn sync_collection_item_from_props(
        &mut self,
        id: HostNodeId,
        role: NativeRole,
        props: &NativeProps,
        selected: bool,
    ) {
        let transient_state = self.nodes.get(&id).map(|state| {
            (
                state.focused,
                state.focus_visible,
                state.focus_within,
                state.pressed,
                state.long_pressed,
                state.moving,
                state.x_delta,
                state.y_delta,
                state.hovered,
            )
        });
        let mut state = initial_state_from_props(props);
        state.selected = selected;
        if role == NativeRole::Radio {
            state.checked = Some(selected);
        }
        if let Some((
            focused,
            focus_visible,
            focus_within,
            pressed,
            long_pressed,
            moving,
            x_delta,
            y_delta,
            hovered,
        )) = transient_state
        {
            state.focused = focused;
            state.focus_visible = focus_visible;
            state.focus_within = focus_within;
            state.pressed = pressed;
            state.long_pressed = long_pressed;
            state.moving = moving;
            state.x_delta = x_delta;
            state.y_delta = y_delta;
            state.hovered = hovered;
        }
        self.nodes.insert(id, state);
    }

    pub(crate) fn set_collection_item_selected(
        &mut self,
        id: HostNodeId,
        role: NativeRole,
        props: &NativeProps,
        selected: bool,
    ) -> Option<InteractionChange> {
        let before = self
            .nodes
            .get(&id)
            .cloned()
            .unwrap_or_else(|| initial_state_from_props(props));
        let mut after = before.clone();
        after.selected = selected;
        if role == NativeRole::Radio {
            after.checked = Some(selected);
        }
        if before == after {
            return None;
        }

        self.nodes.insert(id, after.clone());
        let change = InteractionChange {
            node: id,
            before,
            after,
        };
        push_bounded(&mut self.changes, change.clone(), self.change_history_limit);
        Some(change)
    }

    pub(crate) fn sync_collection_container_from_props(
        &mut self,
        id: HostNodeId,
        props: &NativeProps,
        value: String,
    ) {
        let transient_state = self.nodes.get(&id).map(|state| {
            (
                state.focused,
                state.focus_visible,
                state.focus_within,
                state.pressed,
                state.long_pressed,
                state.moving,
                state.x_delta,
                state.y_delta,
                state.hovered,
            )
        });
        let mut state = initial_state_from_props(props);
        state.value = Some(value);
        if let Some((
            focused,
            focus_visible,
            focus_within,
            pressed,
            long_pressed,
            moving,
            x_delta,
            y_delta,
            hovered,
        )) = transient_state
        {
            state.focused = focused;
            state.focus_visible = focus_visible;
            state.focus_within = focus_within;
            state.pressed = pressed;
            state.long_pressed = long_pressed;
            state.moving = moving;
            state.x_delta = x_delta;
            state.y_delta = y_delta;
            state.hovered = hovered;
        }
        self.nodes.insert(id, state);
    }

    pub(crate) fn set_collection_value(
        &mut self,
        id: HostNodeId,
        props: &NativeProps,
        value: String,
    ) -> Option<InteractionChange> {
        let before = self
            .nodes
            .get(&id)
            .cloned()
            .unwrap_or_else(|| initial_state_from_props(props));
        let mut after = before.clone();
        after.value = Some(value);
        if before == after {
            return None;
        }

        self.nodes.insert(id, after.clone());
        let change = InteractionChange {
            node: id,
            before,
            after,
        };
        push_bounded(&mut self.changes, change.clone(), self.change_history_limit);
        Some(change)
    }

    pub fn set_initial_focus_from_props(&mut self, id: HostNodeId, props: &NativeProps) {
        self.input_modality = NativeInputModality::Virtual;
        let state = self
            .nodes
            .entry(id)
            .or_insert_with(|| initial_state_from_props(props));
        state.focused = true;
        state.focus_visible = true;
    }

    pub(crate) fn set_focus_within(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
        focus_within: bool,
    ) -> Option<InteractionChange> {
        let before = self
            .nodes
            .get(&id)
            .cloned()
            .unwrap_or_else(|| initial_state_from_blueprint(blueprint));
        let mut after = before.clone();
        after.focus_within = focus_within;
        if before == after {
            return None;
        }

        self.nodes.insert(id, after.clone());
        let change = InteractionChange {
            node: id,
            before,
            after,
        };
        let mut history_change = change.clone();
        if blueprint.effective_value_sensitivity().is_sensitive() {
            history_change.before.value = None;
            history_change.after.value = None;
        }
        self.record_change(history_change);
        Some(change)
    }

    pub fn retain_nodes(&mut self, mounted_nodes: &BTreeSet<HostNodeId>) {
        self.nodes.retain(|node, _| mounted_nodes.contains(node));
        self.changes
            .retain(|change| mounted_nodes.contains(&change.node));
        self.value_change_sequences
            .retain(|node, _| mounted_nodes.contains(node));
        self.selection_change_sequences
            .retain(|node, _| mounted_nodes.contains(node));
    }

    pub fn apply_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Option<InteractionChange> {
        self.apply_event_internal(blueprint, event).0
    }

    pub(crate) fn apply_event_with_changes(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Vec<InteractionChange> {
        self.apply_event_internal(blueprint, event).1
    }

    fn apply_event_internal(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> (Option<InteractionChange>, Vec<InteractionChange>) {
        let before = self
            .nodes
            .get(&event.node)
            .cloned()
            .unwrap_or_else(|| initial_state_from_blueprint(blueprint));
        let mut after = before.clone();
        let mut changes = Vec::new();
        let modality_changed = self.update_input_modality(
            event.node,
            event.kind,
            event.effective_modality(),
            &mut changes,
        );
        if modality_changed && after.focused {
            after.focus_visible = self.input_modality.shows_focus_ring();
        }

        match event.kind {
            NativeEventKind::PressStart => {
                after.pressed = true;
            }
            NativeEventKind::PressEnd
            | NativeEventKind::PressUp
            | NativeEventKind::PressCancel
            | NativeEventKind::Press => {
                after.pressed = false;
                if matches!(event.kind, NativeEventKind::PressCancel) {
                    after.long_pressed = false;
                }
            }
            NativeEventKind::LongPressStart => {
                after.long_pressed = true;
            }
            NativeEventKind::LongPressEnd => {
                after.long_pressed = false;
            }
            NativeEventKind::MoveStart => {
                after.moving = true;
                after.x_delta = 0.0;
                after.y_delta = 0.0;
            }
            NativeEventKind::Move => {
                after.moving = true;
                if let Some(delta) = event.context.delta {
                    after.x_delta = delta.x;
                    after.y_delta = delta.y;
                }
            }
            NativeEventKind::MoveEnd => {
                after.moving = false;
            }
            NativeEventKind::HoverStart => {
                if event.effective_modality().supports_hover() {
                    after.hovered = true;
                }
            }
            NativeEventKind::HoverEnd => {
                after.hovered = false;
            }
            NativeEventKind::Focus => {
                self.focus_history = true;
                self.clear_other_focused_nodes(event.node, &mut changes);
                after.focused = true;
                after.focus_visible = self.input_modality.shows_focus_ring();
            }
            NativeEventKind::Blur => {
                self.focus_history = true;
                after.focused = false;
                after.focus_visible = false;
            }
            NativeEventKind::Change => apply_change(blueprint.role, event, &mut after),
            NativeEventKind::SelectionChange => apply_selection(blueprint.role, event, &mut after),
            NativeEventKind::Toggle => apply_toggle(blueprint.role, event, &mut after),
            NativeEventKind::KeyDown
            | NativeEventKind::KeyUp
            | NativeEventKind::Wheel
            | NativeEventKind::LongPress
            | NativeEventKind::Action
            | NativeEventKind::Copy
            | NativeEventKind::Cut
            | NativeEventKind::Paste
            | NativeEventKind::Close => {}
        }

        if before == after {
            return (None, changes);
        }

        self.nodes.insert(event.node, after.clone());
        let change = InteractionChange {
            node: event.node,
            before,
            after,
        };
        let mut history_change = change.clone();
        if blueprint.effective_value_sensitivity().is_sensitive() {
            history_change.before.value = None;
            history_change.after.value = None;
        }
        self.record_change(history_change);
        changes.push(change.clone());
        (Some(change), changes)
    }

    fn update_input_modality(
        &mut self,
        event_node: HostNodeId,
        event_kind: NativeEventKind,
        modality: NativeInputModality,
        changes: &mut Vec<InteractionChange>,
    ) -> bool {
        if modality == NativeInputModality::Unknown
            || !matches!(
                event_kind,
                NativeEventKind::PressStart
                    | NativeEventKind::PressEnd
                    | NativeEventKind::PressUp
                    | NativeEventKind::PressCancel
                    | NativeEventKind::Press
                    | NativeEventKind::LongPressStart
                    | NativeEventKind::LongPressEnd
                    | NativeEventKind::LongPress
                    | NativeEventKind::MoveStart
                    | NativeEventKind::Move
                    | NativeEventKind::MoveEnd
                    | NativeEventKind::Focus
                    | NativeEventKind::KeyDown
                    | NativeEventKind::KeyUp
            )
            || modality == self.input_modality
        {
            return false;
        }

        self.input_modality = modality;
        let focus_visible = modality.shows_focus_ring();
        let mut updated = Vec::new();
        for (node, state) in &mut self.nodes {
            if *node == event_node || !state.focused || state.focus_visible == focus_visible {
                continue;
            }

            let before = state.clone();
            state.focus_visible = focus_visible;
            let mut change = InteractionChange {
                node: *node,
                before,
                after: state.clone(),
            };
            // A focus-only change must not copy an unrelated control value
            // into diagnostics or an event response.
            change.before.value = None;
            change.after.value = None;
            updated.push(change);
        }
        for change in updated {
            self.record_change(change.clone());
            changes.push(change);
        }
        true
    }

    fn clear_other_focused_nodes(
        &mut self,
        focused_node: HostNodeId,
        changes: &mut Vec<InteractionChange>,
    ) {
        let mut cleared = Vec::new();
        for (node, state) in &mut self.nodes {
            if *node == focused_node || !state.focused {
                continue;
            }

            let before = state.clone();
            state.focused = false;
            state.focus_visible = false;
            let mut change = InteractionChange {
                node: *node,
                before,
                after: state.clone(),
            };
            // A focus-only change must not copy an unrelated control value
            // into diagnostics or an event response.
            change.before.value = None;
            change.after.value = None;
            cleared.push(change);
        }
        for change in cleared {
            self.record_change(change.clone());
            changes.push(change);
        }
    }

    fn record_change(&mut self, change: InteractionChange) {
        self.next_change_sequence = self.next_change_sequence.saturating_add(1);
        if change.before.value != change.after.value && change.after.value.is_some() {
            self.value_change_sequences
                .insert(change.node, self.next_change_sequence);
        }
        if change.before.selected != change.after.selected && change.after.selected {
            self.selection_change_sequences
                .insert(change.node, self.next_change_sequence);
        }
        push_bounded(&mut self.changes, change, self.change_history_limit);
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

fn initial_state_from_blueprint(blueprint: &NativeWidgetBlueprint) -> InteractionNodeState {
    InteractionNodeState {
        focused: false,
        focus_visible: false,
        focus_within: false,
        pressed: false,
        long_pressed: false,
        moving: false,
        x_delta: 0.0,
        y_delta: 0.0,
        hovered: false,
        value: blueprint.value.clone(),
        selected: blueprint.control_state.selected,
        checked: blueprint.control_state.checked,
        expanded: blueprint.control_state.expanded,
    }
}

fn initial_state_from_props(props: &NativeProps) -> InteractionNodeState {
    InteractionNodeState {
        focused: false,
        focus_visible: false,
        focus_within: false,
        pressed: false,
        long_pressed: false,
        moving: false,
        x_delta: 0.0,
        y_delta: 0.0,
        hovered: false,
        value: props.value.clone(),
        selected: props.selected,
        checked: props.checked,
        expanded: props.expanded,
    }
}

fn apply_change(role: NativeRole, event: &NativeEvent, state: &mut InteractionNodeState) {
    match role {
        NativeRole::TextField | NativeRole::Select | NativeRole::ComboBox | NativeRole::Slider => {
            state.value = event.value.clone();
        }
        NativeRole::Checkbox | NativeRole::Switch | NativeRole::Radio => {
            state.checked = event.value.as_deref().and_then(parse_bool).or(Some(true));
        }
        _ => {
            state.value = event.value.clone();
        }
    }
}

fn apply_selection(role: NativeRole, event: &NativeEvent, state: &mut InteractionNodeState) {
    match role {
        NativeRole::ListBoxItem | NativeRole::TreeItem | NativeRole::Tab | NativeRole::MenuItem => {
            state.selected = true;
        }
        NativeRole::Radio => {
            state.selected = true;
            state.checked = Some(true);
        }
        NativeRole::Select
        | NativeRole::ComboBox
        | NativeRole::ListBox
        | NativeRole::Tree
        | NativeRole::Menu
        | NativeRole::Tabs
        | NativeRole::TabList
        | NativeRole::RadioGroup => {
            state.value = event.value.clone();
        }
        _ => {
            state.selected = true;
            state.value = event.value.clone();
        }
    }
}

fn apply_toggle(role: NativeRole, event: &NativeEvent, state: &mut InteractionNodeState) {
    if role == NativeRole::Tree {
        state.value = event.value.clone();
        return;
    }
    if is_expansion_toggle_role(role) || state.expanded.is_some() {
        state.expanded = match event.value.as_deref().and_then(parse_bool) {
            Some(value) => Some(value),
            None => Some(!state.expanded.unwrap_or(false)),
        };
        return;
    }

    state.checked = match event.value.as_deref().and_then(parse_bool) {
        Some(value) => Some(value),
        None => Some(!state.checked.unwrap_or(false)),
    };
}

fn is_expansion_toggle_role(role: NativeRole) -> bool {
    matches!(
        role,
        NativeRole::Disclosure | NativeRole::DisclosureSummary | NativeRole::Popover
    )
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" | "1" | "on" => Some(true),
        "false" | "0" | "off" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{Gtk4Adapter, PlatformAdapter};
    use crate::web::WebProps;

    #[test]
    fn text_field_change_updates_value_state() {
        let element = NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().web(WebProps::new().on_change("setEmail")));
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(1), NativeEventKind::Change).value("a@b.c"),
            )
            .unwrap();

        assert_eq!(change.after.value.as_deref(), Some("a@b.c"));
        assert_eq!(
            state.node(HostNodeId::new(1)).unwrap().value.as_deref(),
            Some("a@b.c")
        );
    }

    #[test]
    fn toggle_event_updates_checked_state() {
        let element = NativeElement::new("enabled", NativeRole::Switch);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(2), NativeEventKind::Toggle),
            )
            .unwrap();
        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(2), NativeEventKind::Toggle),
            )
            .unwrap();

        assert_eq!(change.after.checked, Some(false));
    }

    #[test]
    fn toggle_event_starts_from_blueprint_checked_state() {
        let element = NativeElement::new("enabled", NativeRole::Switch)
            .with_props(NativeProps::new().checked(true));
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(2), NativeEventKind::Toggle),
            )
            .unwrap();

        assert_eq!(change.before.checked, Some(true));
        assert_eq!(change.after.checked, Some(false));
    }

    #[test]
    fn disclosure_toggle_updates_expanded_state() {
        let element = NativeElement::new("details", NativeRole::Disclosure)
            .with_props(NativeProps::new().expanded(false));
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(5), NativeEventKind::Toggle),
            )
            .unwrap();

        assert_eq!(change.before.expanded, Some(false));
        assert_eq!(change.after.expanded, Some(true));
        assert_eq!(change.after.checked, None);
    }

    #[test]
    fn radio_selection_marks_checked_state() {
        let element = NativeElement::new("dark", NativeRole::Radio);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::SelectionChange),
            )
            .unwrap();

        assert!(change.after.selected);
        assert_eq!(change.after.checked, Some(true));
    }

    #[test]
    fn sync_node_from_blueprint_preserves_focus_and_refreshes_control_state() {
        let first = NativeElement::new("enabled", NativeRole::Switch)
            .with_props(NativeProps::new().checked(true));
        let second = NativeElement::new("enabled", NativeRole::Switch)
            .with_props(NativeProps::new().checked(false));
        let mut state = InteractionState::new();

        state
            .apply_event(
                &Gtk4Adapter.blueprint(&first),
                &NativeEvent::new(HostNodeId::new(4), NativeEventKind::Focus),
            )
            .unwrap();
        state.sync_node_from_blueprint(HostNodeId::new(4), &Gtk4Adapter.blueprint(&second));

        let node = state.node(HostNodeId::new(4)).unwrap();
        assert!(node.focused);
        assert_eq!(node.checked, Some(false));
    }

    #[test]
    fn set_initial_focus_from_props_does_not_record_event_change() {
        let props = NativeProps::new().value("Ada").selected(true);
        let mut state = InteractionState::new();

        state.set_initial_focus_from_props(HostNodeId::new(9), &props);

        let node = state.node(HostNodeId::new(9)).unwrap();
        assert!(node.focused);
        assert!(node.focus_visible);
        assert_eq!(node.value.as_deref(), Some("Ada"));
        assert!(node.selected);
        assert_eq!(state.input_modality(), NativeInputModality::Virtual);
        assert!(!state.has_focus_history());
        assert!(state.changes().is_empty());
    }

    #[test]
    fn set_initial_focus_preserves_existing_node_state() {
        let element = NativeElement::new("enabled", NativeRole::Switch)
            .with_props(NativeProps::new().checked(false));
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(10), NativeEventKind::Toggle),
            )
            .unwrap();
        state.set_initial_focus_from_props(HostNodeId::new(10), &NativeProps::new());

        let node = state.node(HostNodeId::new(10)).unwrap();
        assert!(node.focused);
        assert_eq!(node.checked, Some(true));
    }

    #[test]
    fn focus_and_blur_update_focus_state() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        assert!(!state.has_focus_history());
        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::Focus),
            )
            .unwrap();
        assert!(state.has_focus_history());
        let change = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::Blur),
            )
            .unwrap();

        assert!(!change.after.focused);
    }

    #[test]
    fn focus_event_clears_previous_focused_node() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::Focus),
            )
            .unwrap();
        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(4), NativeEventKind::Focus),
            )
            .unwrap();

        assert!(!state.node(HostNodeId::new(3)).unwrap().focused);
        assert!(state.node(HostNodeId::new(4)).unwrap().focused);
        assert_eq!(state.changes().len(), 3);
        assert_eq!(state.changes()[1].node, HostNodeId::new(3));
        assert!(!state.changes()[1].after.focused);
    }

    #[test]
    fn retain_nodes_prunes_state_and_change_history() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::Focus),
            )
            .unwrap();
        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(4), NativeEventKind::Focus),
            )
            .unwrap();

        state.retain_nodes(&BTreeSet::from([HostNodeId::new(4)]));

        assert!(state.node(HostNodeId::new(3)).is_none());
        assert!(state.node(HostNodeId::new(4)).is_some());
        assert!(state.has_focus_history());
        assert_eq!(state.changes().len(), 1);
        assert_eq!(state.changes()[0].node, HostNodeId::new(4));
    }

    #[test]
    fn keyboard_focus_is_visible_and_pointer_press_hides_the_ring() {
        let blueprint = Gtk4Adapter.blueprint(&NativeElement::new("save", NativeRole::Button));
        let node = HostNodeId::new(11);
        let mut state = InteractionState::new();

        assert!(state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::KeyDown).value("Tab"),
            )
            .is_none());
        let focus = state
            .apply_event(&blueprint, &NativeEvent::new(node, NativeEventKind::Focus))
            .unwrap();

        assert_eq!(state.input_modality(), NativeInputModality::Keyboard);
        assert!(focus.after.focused);
        assert!(focus.after.focus_visible);

        let press = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::PressStart)
                    .modality(NativeInputModality::Mouse),
            )
            .unwrap();

        assert_eq!(state.input_modality(), NativeInputModality::Mouse);
        assert!(press.after.focused);
        assert!(!press.after.focus_visible);
        assert!(press.after.pressed);
    }

    #[test]
    fn press_end_and_cancel_clear_transient_press_state() {
        let blueprint = Gtk4Adapter.blueprint(&NativeElement::new("save", NativeRole::Button));
        let node = HostNodeId::new(12);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::PressStart)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        let end = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::PressEnd)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        assert!(!end.after.pressed);

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::PressStart)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        let cancel = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::PressCancel)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        assert!(!cancel.after.pressed);
    }

    #[test]
    fn hover_ignores_touch_and_tracks_mouse_or_pen() {
        let blueprint = Gtk4Adapter.blueprint(&NativeElement::new("save", NativeRole::Button));
        let node = HostNodeId::new(13);
        let mut state = InteractionState::new();

        assert!(state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::HoverStart)
                    .modality(NativeInputModality::Touch),
            )
            .is_none());

        let start = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::HoverStart)
                    .modality(NativeInputModality::Pen),
            )
            .unwrap();
        assert!(start.after.hovered);

        let end = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::HoverEnd)
                    .modality(NativeInputModality::Pen),
            )
            .unwrap();
        assert!(!end.after.hovered);
    }

    #[test]
    fn move_lifecycle_tracks_active_state_and_incremental_delta() {
        let blueprint = Gtk4Adapter.blueprint(&NativeElement::new("thumb", NativeRole::View));
        let node = HostNodeId::new(15);
        let mut state = InteractionState::new();

        let start = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::MoveStart)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        assert!(start.after.moving);

        let movement = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::Move)
                    .modality(NativeInputModality::Touch)
                    .delta(4.0, -3.0),
            )
            .unwrap();
        assert!(movement.after.moving);
        assert_eq!(movement.after.x_delta, 4.0);
        assert_eq!(movement.after.y_delta, -3.0);

        let end = state
            .apply_event(
                &blueprint,
                &NativeEvent::new(node, NativeEventKind::MoveEnd)
                    .modality(NativeInputModality::Touch),
            )
            .unwrap();
        assert!(!end.after.moving);
        assert_eq!(end.after.x_delta, 4.0);
        assert_eq!(end.after.y_delta, -3.0);
    }

    #[test]
    fn blueprint_sync_preserves_transient_interaction_state() {
        let first = Gtk4Adapter.blueprint(&NativeElement::new("save", NativeRole::Button));
        let second = Gtk4Adapter.blueprint(
            &NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().selected(true)),
        );
        let node = HostNodeId::new(14);
        let mut state = InteractionState::new();

        state.apply_event(
            &first,
            &NativeEvent::new(node, NativeEventKind::KeyDown).value("Tab"),
        );
        state.apply_event(&first, &NativeEvent::new(node, NativeEventKind::Focus));
        state.apply_event(
            &first,
            &NativeEvent::new(node, NativeEventKind::HoverStart)
                .modality(NativeInputModality::Mouse),
        );
        state.apply_event(
            &first,
            &NativeEvent::new(node, NativeEventKind::PressStart)
                .modality(NativeInputModality::Mouse),
        );

        state.sync_node_from_blueprint(node, &second);

        let current = state.node(node).unwrap();
        assert!(current.focused);
        assert!(current.hovered);
        assert!(current.pressed);
        assert!(current.selected);
    }
}
