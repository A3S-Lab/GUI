use super::*;
use crate::accessibility::{AccessibilityNode, AccessibilityRole};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{HeadlessAdapter, PlatformAdapter, PlatformPlanningHost};

fn semantic_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        )
        .child(
            NativeElement::new("people", NativeRole::ListBox)
                .with_props(NativeProps::new().label("People").multiple(true))
                .child(
                    NativeElement::new("ada", NativeRole::ListBoxItem)
                        .with_props(NativeProps::new().label("Ada").selected(true)),
                )
                .child(
                    NativeElement::new("grace", NativeRole::ListBoxItem)
                        .with_props(NativeProps::new().label("Grace")),
                ),
        )
}

fn render<A: PlatformAdapter>(adapter: A) -> AccessibilityNode {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    runtime.render_native(&semantic_tree()).unwrap();
    let report = runtime.accessibility_conformance().unwrap();
    assert!(report.is_conformant(), "{:?}", report.issues);
    runtime.accessibility_tree().unwrap()
}

fn semantic_snapshot(
    node: &AccessibilityNode,
    output: &mut Vec<(AccessibilityRole, Option<String>, bool, Option<bool>)>,
) {
    output.push((node.role, node.label.clone(), node.selected, node.checked));
    for child in &node.children {
        semantic_snapshot(child, output);
    }
}

#[test]
fn headless_planner_exposes_the_portable_accessibility_semantics() {
    let tree = render(HeadlessAdapter);
    let mut snapshot = Vec::new();
    semantic_snapshot(&tree, &mut snapshot);

    assert!(snapshot.iter().any(|(role, label, _, _)| {
        *role == AccessibilityRole::Button && label.as_deref() == Some("Save")
    }));
    assert!(snapshot.iter().any(|(role, label, selected, _)| {
        *role == AccessibilityRole::ListBoxOption && label.as_deref() == Some("Ada") && *selected
    }));
}
