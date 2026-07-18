use std::collections::BTreeMap;

use super::GuiRuntime;
use crate::accessibility::{AccessibilityNode, AccessibilityRole, AccessibilityTreeHost};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::{InteractionNodeState, InteractionState};

impl<H: NativeHost + AccessibilityTreeHost> GuiRuntime<H> {
    pub fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        let mut tree = self.host.accessibility_tree()?;
        apply_interactions_to_accessibility_tree(
            &mut tree,
            &self.interaction_state,
            &self.interaction_revisions,
            self.render_revision,
        );
        Some(tree)
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
    if node.value_sensitivity.is_public() {
        if let Some(value) = &state.value {
            node.value = Some(value.clone());
        }
    } else {
        // Interaction state keeps the real value for in-process reducers and
        // native controls, but accessibility is always a redacted projection.
        node.value = None;
        node.description.value_text = None;
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
    let parent = node.node.and_then(|parent| {
        (interaction_revisions.get(&parent).copied() == Some(render_revision))
            .then(|| interactions.value_change_sequence(parent))
            .flatten()
            .map(|sequence| (sequence, SelectionSource::ParentValue))
    });
    let child = node
        .children
        .iter()
        .filter(|child| is_selectable_child(child.role))
        .filter_map(|child| child.node)
        .filter(|child| interaction_revisions.get(child).copied() == Some(render_revision))
        .filter_map(|child| {
            interactions
                .selection_change_sequence(child)
                .map(|sequence| (sequence, SelectionSource::Child(child)))
        })
        .max_by_key(|(sequence, _)| *sequence);

    parent
        .into_iter()
        .chain(child)
        .max_by_key(|(sequence, _)| *sequence)
        .map(|(_, source)| source)
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

fn selected_accessibility_value(child: &AccessibilityNode) -> Option<String> {
    child.value.clone().or_else(|| child.label.clone())
}

fn child_matches_selection_value(child: &AccessibilityNode, value: &str) -> bool {
    child.value.as_deref() == Some(value) || child.label.as_deref() == Some(value)
}
