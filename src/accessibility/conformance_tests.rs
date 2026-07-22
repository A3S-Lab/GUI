use super::*;

fn node(role: AccessibilityRole) -> AccessibilityNode {
    AccessibilityNode {
        node: None,
        role,
        label: None,
        value: None,
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
