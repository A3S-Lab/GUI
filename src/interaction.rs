use std::collections::{BTreeMap, BTreeSet};

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::native::{NativeProps, NativeRole};
use crate::platform::NativeWidgetBlueprint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionNodeState {
    pub focused: bool,
    pub pressed: bool,
    pub value: Option<String>,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionChange {
    pub node: HostNodeId,
    pub before: InteractionNodeState,
    pub after: InteractionNodeState,
}

#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    nodes: BTreeMap<HostNodeId, InteractionNodeState>,
    changes: Vec<InteractionChange>,
    focus_history: bool,
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node(&self, id: HostNodeId) -> Option<&InteractionNodeState> {
        self.nodes.get(&id)
    }

    pub fn has_focused_node(&self) -> bool {
        self.nodes.values().any(|state| state.focused)
    }

    pub fn has_focus_history(&self) -> bool {
        self.focus_history
    }

    pub fn changes(&self) -> &[InteractionChange] {
        &self.changes
    }

    pub fn sync_node_from_blueprint(&mut self, id: HostNodeId, blueprint: &NativeWidgetBlueprint) {
        let focused = self.nodes.get(&id).is_some_and(|state| state.focused);
        let mut state = initial_state_from_blueprint(blueprint);
        state.focused = focused;
        self.nodes.insert(id, state);
    }

    pub fn set_initial_focus_from_props(&mut self, id: HostNodeId, props: &NativeProps) {
        let state = self
            .nodes
            .entry(id)
            .or_insert_with(|| initial_state_from_props(props));
        state.focused = true;
    }

    pub fn retain_nodes(&mut self, mounted_nodes: &BTreeSet<HostNodeId>) {
        self.nodes.retain(|node, _| mounted_nodes.contains(node));
        self.changes
            .retain(|change| mounted_nodes.contains(&change.node));
    }

    pub fn apply_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Option<InteractionChange> {
        let before = self
            .nodes
            .get(&event.node)
            .cloned()
            .unwrap_or_else(|| initial_state_from_blueprint(blueprint));
        let mut after = before.clone();

        match event.kind {
            NativeEventKind::PressStart => {
                after.pressed = true;
            }
            NativeEventKind::PressEnd | NativeEventKind::Press => {
                after.pressed = false;
            }
            NativeEventKind::Focus => {
                self.focus_history = true;
                self.clear_other_focused_nodes(event.node);
                after.focused = true;
            }
            NativeEventKind::Blur => {
                self.focus_history = true;
                after.focused = false;
            }
            NativeEventKind::Change => apply_change(blueprint.role, event, &mut after),
            NativeEventKind::SelectionChange => apply_selection(blueprint.role, event, &mut after),
            NativeEventKind::Toggle => apply_toggle(blueprint.role, event, &mut after),
            NativeEventKind::KeyDown | NativeEventKind::KeyUp | NativeEventKind::Close => {}
        }

        if before == after {
            return None;
        }

        self.nodes.insert(event.node, after.clone());
        let change = InteractionChange {
            node: event.node,
            before,
            after,
        };
        self.changes.push(change.clone());
        Some(change)
    }

    fn clear_other_focused_nodes(&mut self, focused_node: HostNodeId) {
        for (node, state) in &mut self.nodes {
            if *node == focused_node || !state.focused {
                continue;
            }

            let before = state.clone();
            state.focused = false;
            self.changes.push(InteractionChange {
                node: *node,
                before,
                after: state.clone(),
            });
        }
    }
}

fn initial_state_from_blueprint(blueprint: &NativeWidgetBlueprint) -> InteractionNodeState {
    InteractionNodeState {
        focused: false,
        pressed: false,
        value: blueprint.value.clone(),
        selected: blueprint.control_state.selected,
        checked: blueprint.control_state.checked,
        expanded: blueprint.control_state.expanded,
    }
}

fn initial_state_from_props(props: &NativeProps) -> InteractionNodeState {
    InteractionNodeState {
        focused: false,
        pressed: false,
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
        NativeRole::ListBoxItem | NativeRole::Tab | NativeRole::MenuItem => {
            state.selected = true;
            if let Some(value) = &event.value {
                state.value = Some(value.clone());
            }
        }
        NativeRole::Radio => {
            state.selected = true;
            state.checked = Some(true);
            if let Some(value) = &event.value {
                state.value = Some(value.clone());
            }
        }
        NativeRole::Select | NativeRole::ListBox | NativeRole::Tabs | NativeRole::RadioGroup => {
            state.value = event.value.clone();
        }
        _ => {
            state.selected = true;
            state.value = event.value.clone();
        }
    }
}

fn apply_toggle(role: NativeRole, event: &NativeEvent, state: &mut InteractionNodeState) {
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
        assert_eq!(node.value.as_deref(), Some("Ada"));
        assert!(node.selected);
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
}
