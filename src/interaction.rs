use std::collections::BTreeMap;

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::native::NativeRole;
use crate::platform::NativeWidgetBlueprint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionNodeState {
    pub focused: bool,
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
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node(&self, id: HostNodeId) -> Option<&InteractionNodeState> {
        self.nodes.get(&id)
    }

    pub fn changes(&self) -> &[InteractionChange] {
        &self.changes
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
            NativeEventKind::Focus => {
                self.clear_other_focused_nodes(event.node);
                after.focused = true;
            }
            NativeEventKind::Blur => after.focused = false,
            NativeEventKind::Change => apply_change(blueprint.role, event, &mut after),
            NativeEventKind::SelectionChange => apply_selection(blueprint.role, event, &mut after),
            NativeEventKind::Toggle => apply_toggle(event, &mut after),
            NativeEventKind::Press => {}
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
        value: blueprint.value.clone(),
        selected: blueprint.control_state.selected,
        checked: blueprint.control_state.checked,
        expanded: blueprint.control_state.expanded,
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
        }
        NativeRole::Radio => {
            state.selected = true;
            state.checked = Some(true);
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

fn apply_toggle(event: &NativeEvent, state: &mut InteractionNodeState) {
    state.checked = match event.value.as_deref().and_then(parse_bool) {
        Some(value) => Some(value),
        None => Some(!state.checked.unwrap_or(false)),
    };
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
    fn focus_and_blur_update_focus_state() {
        let element = NativeElement::new("save", NativeRole::Button);
        let blueprint = Gtk4Adapter.blueprint(&element);
        let mut state = InteractionState::new();

        state
            .apply_event(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(3), NativeEventKind::Focus),
            )
            .unwrap();
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
}
