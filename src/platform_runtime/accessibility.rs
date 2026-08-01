use crate::accessibility::AccessibilityNode;
use crate::error::GuiResult;
use crate::layout::{LayoutElementId, LayoutSnapshot};
use crate::native::NativeElement;
use crate::platform_host::{
    PlatformAccessibilityNode, PlatformAccessibilitySnapshot, PlatformElementId, PlatformWindowId,
};

pub(super) fn accessibility_snapshot(
    window: PlatformWindowId,
    root: &NativeElement,
    layout: &LayoutSnapshot,
) -> GuiResult<PlatformAccessibilitySnapshot> {
    let root_id = LayoutElementId::root(root.key.as_str());
    let snapshot = PlatformAccessibilitySnapshot {
        window,
        root: project_node(root, &root_id, layout)?,
    };
    snapshot.validate()?;
    Ok(snapshot)
}

fn project_node(
    element: &NativeElement,
    id: &LayoutElementId,
    layout: &LayoutSnapshot,
) -> GuiResult<Option<PlatformAccessibilityNode>> {
    let Some(layout_node) = layout.node(id) else {
        return Ok(None);
    };
    let semantic = AccessibilityNode::from_native(element);
    let mut projected = PlatformAccessibilityNode {
        id: PlatformElementId::new(id.as_str())?,
        role: semantic.role,
        label: semantic.label,
        value: semantic.value,
        value_sensitivity: semantic.value_sensitivity,
        relationships: semantic.relationships,
        description: semantic.description,
        structure: semantic.structure,
        state: semantic.state,
        disabled: semantic.disabled,
        required: semantic.required,
        invalid: semantic.invalid,
        read_only: semantic.read_only,
        multiple: semantic.multiple,
        focused: semantic.focused,
        selected: semantic.selected,
        checked: semantic.checked,
        expanded: semantic.expanded,
        logical_bounds: layout_node.border_box,
        children: Vec::new(),
    };
    for child in &element.children {
        let child_id = id.child(child.key.as_str());
        if let Some(child) = project_node(child, &child_id, layout)? {
            projected.children.push(child);
        }
    }
    Ok(Some(projected))
}
