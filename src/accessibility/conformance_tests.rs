use super::*;

fn node(role: AccessibilityRole) -> AccessibilityNode {
    AccessibilityNode {
        node: None,
        role,
        label: None,
        value: None,
        value_sensitivity: Default::default(),
        relationships: AccessibilityRelationshipProps::default(),
        description: AccessibilityDescriptionProps::default(),
        structure: AccessibilityStructureProps::default(),
        state: AccessibilityStateProps::default(),
        disabled: false,
        required: false,
        invalid: false,
        read_only: false,
        multiple: false,
        focused: false,
        selected: false,
        checked: None,
        expanded: None,
        children: Vec::new(),
    }
}

#[test]
fn labelled_interactive_tree_is_conformant() {
    let mut list = node(AccessibilityRole::ListBox);
    list.node = Some(HostNodeId::new(1));
    list.label = Some("People".to_string());
    let mut option = node(AccessibilityRole::ListBoxOption);
    option.node = Some(HostNodeId::new(2));
    option.value = Some("Ada".to_string());
    option.selected = true;
    list.children.push(option);

    let report = AccessibilityConformanceReport::validate(&list);

    assert!(report.is_conformant(), "{:?}", report.issues);
    assert!(report.issues.is_empty());
}

#[test]
fn conformance_reports_names_states_focus_and_exclusive_selection() {
    let mut list = node(AccessibilityRole::ListBox);
    list.node = Some(HostNodeId::new(1));
    list.focused = true;
    for id in [2, 3] {
        let mut option = node(AccessibilityRole::ListBoxOption);
        option.node = Some(HostNodeId::new(id));
        option.focused = true;
        option.selected = true;
        list.children.push(option);
    }
    list.checked = Some(true);

    let report = AccessibilityConformanceReport::validate(&list);

    assert!(!report.is_conformant());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == AccessibilityIssueCode::MissingAccessibleName));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == AccessibilityIssueCode::InvalidCheckedState));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == AccessibilityIssueCode::MultipleSelectedItems));
    assert_eq!(
        report
            .issues
            .iter()
            .filter(|issue| issue.code == AccessibilityIssueCode::MultipleFocusedNodes)
            .count(),
        3
    );
}

#[test]
fn conformance_rejects_invalid_live_region_tokens() {
    let mut status = node(AccessibilityRole::Group);
    status.state.live = Some("urgent".to_string());
    status.state.relevant = Some("additions replacements".to_string());

    let report = AccessibilityConformanceReport::validate(&status);

    assert!(!report.is_conformant());
    assert!(report
        .issues
        .iter()
        .any(|issue| { issue.code == AccessibilityIssueCode::InvalidLiveRegionPoliteness }));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == AccessibilityIssueCode::InvalidLiveRegionRelevant));
}

#[test]
fn conformance_rejects_invalid_accessibility_state_tokens() {
    let mut stateful = node(AccessibilityRole::Button);
    stateful.label = Some("Stateful".to_string());
    stateful.state.autocomplete = Some("automatic".to_string());
    stateful.state.current = Some("yesterday".to_string());
    stateful.state.has_popup = Some("sheet".to_string());
    stateful.state.pressed = Some("indeterminate".to_string());

    let report = AccessibilityConformanceReport::validate(&stateful);

    assert!(!report.is_conformant());
    for code in [
        AccessibilityIssueCode::InvalidAutocomplete,
        AccessibilityIssueCode::InvalidCurrent,
        AccessibilityIssueCode::InvalidHasPopup,
        AccessibilityIssueCode::InvalidPressed,
    ] {
        assert!(report.issues.iter().any(|issue| issue.code == code));
    }
}

#[test]
fn conformance_rejects_multiple_active_descendant_references() {
    let mut list = node(AccessibilityRole::ListBox);
    list.label = Some("Results".to_string());
    list.relationships.active_descendant = Some("first second".to_string());

    let report = AccessibilityConformanceReport::validate(&list);

    assert!(!report.is_conformant());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == AccessibilityIssueCode::InvalidActiveDescendant));
}

#[test]
fn conformance_rejects_invalid_accessibility_structure_values() {
    let mut cell = node(AccessibilityRole::TableCell);
    cell.structure = AccessibilityStructureProps::default()
        .level(Some(0))
        .position_in_set(Some(3))
        .set_size(Some(2))
        .row_count(Some(-2))
        .row_index(Some(0))
        .row_span(Some(0))
        .column_count(Some(-2))
        .column_index(Some(0))
        .column_span(Some(0))
        .sort("sideways");

    let report = AccessibilityConformanceReport::validate(&cell);

    assert!(!report.is_conformant());
    assert_eq!(
        report
            .issues
            .iter()
            .filter(|issue| issue.code == AccessibilityIssueCode::InvalidAccessibilityStructure)
            .count(),
        9
    );
}
