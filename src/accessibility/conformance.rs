use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{AccessibilityNode, AccessibilityRole};
use crate::host::HostNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessibilityIssueSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessibilityIssueCode {
    DuplicateNode,
    MissingAccessibleName,
    MultipleFocusedNodes,
    InvalidSelectedState,
    InvalidCheckedState,
    InvalidMultipleState,
    MultipleSelectedItems,
    EmptyRelationship,
    InvalidActiveDescendant,
    InvalidAutocomplete,
    InvalidCurrent,
    InvalidHasPopup,
    InvalidPressed,
    InvalidLiveRegionPoliteness,
    InvalidLiveRegionRelevant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityConformanceIssue {
    pub severity: AccessibilityIssueSeverity,
    pub code: AccessibilityIssueCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<HostNodeId>,
    pub role: AccessibilityRole,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityConformanceReport {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<AccessibilityConformanceIssue>,
}

impl AccessibilityConformanceReport {
    pub fn validate(root: &AccessibilityNode) -> Self {
        let mut report = Self::default();
        let mut nodes = BTreeSet::new();
        let mut focused = Vec::new();
        validate_node(root, &mut nodes, &mut focused, &mut report.issues);
        if focused.len() > 1 {
            for node in focused {
                report.issues.push(AccessibilityConformanceIssue {
                    severity: AccessibilityIssueSeverity::Error,
                    code: AccessibilityIssueCode::MultipleFocusedNodes,
                    node: node.node,
                    role: node.role,
                    message: "only one accessible node may be focused in a native tree".to_string(),
                });
            }
        }
        report
    }

    pub fn is_conformant(&self) -> bool {
        self.issues
            .iter()
            .all(|issue| issue.severity != AccessibilityIssueSeverity::Error)
    }

    pub fn errors(&self) -> impl Iterator<Item = &AccessibilityConformanceIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == AccessibilityIssueSeverity::Error)
    }
}

fn validate_node<'a>(
    node: &'a AccessibilityNode,
    nodes: &mut BTreeSet<HostNodeId>,
    focused: &mut Vec<&'a AccessibilityNode>,
    issues: &mut Vec<AccessibilityConformanceIssue>,
) {
    if let Some(id) = node.node {
        if !nodes.insert(id) {
            push_error(
                issues,
                node,
                AccessibilityIssueCode::DuplicateNode,
                format!("accessible node id {} appears more than once", id.get()),
            );
        }
    }
    if node.focused {
        focused.push(node);
    }
    if requires_accessible_name(node.role) && !has_accessible_name(node) {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::MissingAccessibleName,
            "interactive accessible role requires a non-empty name".to_string(),
        );
    }
    if node.selected && !supports_selected(node.role) {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidSelectedState,
            "selected state is not valid for this accessible role".to_string(),
        );
    }
    if node.checked.is_some() && !supports_checked(node.role) {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidCheckedState,
            "checked state is not valid for this accessible role".to_string(),
        );
    }
    if node.multiple && !supports_multiple(node.role) {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidMultipleState,
            "multiple state is not valid for this accessible role".to_string(),
        );
    }
    if is_exclusive_selection_container(node) {
        let selected = node
            .children
            .iter()
            .filter(|child| child.selected || child.checked == Some(true))
            .count();
        if selected > 1 {
            push_error(
                issues,
                node,
                AccessibilityIssueCode::MultipleSelectedItems,
                "single-selection container exposes more than one selected child".to_string(),
            );
        }
    }
    for (name, relationship) in relationships(node) {
        if relationship.is_some_and(|value| value.trim().is_empty()) {
            issues.push(AccessibilityConformanceIssue {
                severity: AccessibilityIssueSeverity::Warning,
                code: AccessibilityIssueCode::EmptyRelationship,
                node: node.node,
                role: node.role,
                message: format!("accessibility relationship {name} must not be empty"),
            });
        }
    }
    if node
        .relationships
        .active_descendant
        .as_deref()
        .is_some_and(|value| value.split_whitespace().count() > 1)
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidActiveDescendant,
            "aria-activedescendant must reference exactly one element".to_string(),
        );
    }
    if node
        .state
        .autocomplete
        .as_deref()
        .is_some_and(|value| !is_valid_autocomplete(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidAutocomplete,
            "aria-autocomplete must be none, inline, list, or both".to_string(),
        );
    }
    if node
        .state
        .current
        .as_deref()
        .is_some_and(|value| !is_valid_current(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidCurrent,
            "aria-current must be false, true, page, step, location, date, or time".to_string(),
        );
    }
    if node
        .state
        .has_popup
        .as_deref()
        .is_some_and(|value| !is_valid_has_popup(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidHasPopup,
            "aria-haspopup must be false, true, menu, listbox, tree, grid, or dialog".to_string(),
        );
    }
    if node
        .state
        .pressed
        .as_deref()
        .is_some_and(|value| !is_valid_pressed(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidPressed,
            "aria-pressed must be false, true, or mixed".to_string(),
        );
    }
    if node
        .state
        .live
        .as_deref()
        .is_some_and(|value| !is_valid_live_politeness(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidLiveRegionPoliteness,
            "aria-live must be off, polite, or assertive".to_string(),
        );
    }
    if node
        .state
        .relevant
        .as_deref()
        .is_some_and(|value| !is_valid_live_relevant(value))
    {
        push_error(
            issues,
            node,
            AccessibilityIssueCode::InvalidLiveRegionRelevant,
            "aria-relevant must contain additions, removals, text, or all".to_string(),
        );
    }

    for child in &node.children {
        validate_node(child, nodes, focused, issues);
    }
}

fn is_valid_autocomplete(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "none" | "inline" | "list" | "both"
    )
}

fn is_valid_current(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "false" | "true" | "page" | "step" | "location" | "date" | "time"
    )
}

fn is_valid_has_popup(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "false" | "true" | "menu" | "listbox" | "tree" | "grid" | "dialog"
    )
}

fn is_valid_pressed(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "false" | "true" | "mixed"
    )
}

fn is_valid_live_politeness(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "off" | "polite" | "assertive"
    )
}

fn is_valid_live_relevant(value: &str) -> bool {
    let mut has_token = false;
    for token in value.split_ascii_whitespace() {
        if !matches!(
            token.to_ascii_lowercase().as_str(),
            "additions" | "removals" | "text" | "all"
        ) {
            return false;
        }
        has_token = true;
    }
    has_token
}

fn push_error(
    issues: &mut Vec<AccessibilityConformanceIssue>,
    node: &AccessibilityNode,
    code: AccessibilityIssueCode,
    message: String,
) {
    issues.push(AccessibilityConformanceIssue {
        severity: AccessibilityIssueSeverity::Error,
        code,
        node: node.node,
        role: node.role,
        message,
    });
}

fn has_accessible_name(node: &AccessibilityNode) -> bool {
    node.label
        .as_deref()
        .is_some_and(|label| !label.trim().is_empty())
        || node
            .value
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty() && value_name_allowed(node.role))
        || node
            .relationships
            .labelled_by
            .as_deref()
            .is_some_and(|relationship| !relationship.trim().is_empty())
}

fn requires_accessible_name(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::Button
            | AccessibilityRole::Link
            | AccessibilityRole::Image
            | AccessibilityRole::TextField
            | AccessibilityRole::Checkbox
            | AccessibilityRole::Switch
            | AccessibilityRole::RadioGroup
            | AccessibilityRole::RadioButton
            | AccessibilityRole::ComboBox
            | AccessibilityRole::ListBox
            | AccessibilityRole::ListBoxOption
            | AccessibilityRole::Tree
            | AccessibilityRole::TreeItem
            | AccessibilityRole::Dialog
            | AccessibilityRole::TabList
            | AccessibilityRole::Tab
            | AccessibilityRole::Menu
            | AccessibilityRole::MenuItem
            | AccessibilityRole::Slider
    )
}

fn value_name_allowed(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ListBoxOption
            | AccessibilityRole::TreeItem
            | AccessibilityRole::Tab
            | AccessibilityRole::MenuItem
            | AccessibilityRole::RadioButton
    )
}

fn supports_selected(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ListBoxOption
            | AccessibilityRole::TreeItem
            | AccessibilityRole::Tab
            | AccessibilityRole::MenuItem
            | AccessibilityRole::RadioButton
            | AccessibilityRole::TableRow
    )
}

fn supports_checked(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::Checkbox
            | AccessibilityRole::Switch
            | AccessibilityRole::RadioButton
            | AccessibilityRole::MenuItem
    )
}

fn supports_multiple(role: AccessibilityRole) -> bool {
    matches!(
        role,
        AccessibilityRole::ListBox | AccessibilityRole::Tree | AccessibilityRole::Table
    )
}

fn is_exclusive_selection_container(node: &AccessibilityNode) -> bool {
    !node.multiple
        && matches!(
            node.role,
            AccessibilityRole::ComboBox
                | AccessibilityRole::ListBox
                | AccessibilityRole::RadioGroup
                | AccessibilityRole::TabGroup
                | AccessibilityRole::TabList
        )
}

fn relationships(node: &AccessibilityNode) -> [(&'static str, Option<&str>); 8] {
    let relationships = &node.relationships;
    [
        ("labelledBy", relationships.labelled_by.as_deref()),
        ("describedBy", relationships.described_by.as_deref()),
        ("details", relationships.details.as_deref()),
        ("controls", relationships.controls.as_deref()),
        ("owns", relationships.owns.as_deref()),
        ("flowTo", relationships.flow_to.as_deref()),
        ("errorMessage", relationships.error_message.as_deref()),
        (
            "activeDescendant",
            relationships.active_descendant.as_deref(),
        ),
    ]
}
