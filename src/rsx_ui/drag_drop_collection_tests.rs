use crate::compiler::CompiledRsxNode;
use crate::rsx_app::RsxComponent;

#[test]
fn collection_components_lower_shared_native_drag_and_drop_contract() {
    let component = RsxComponent::<()>::new(
        "collection-drag-drop",
        r#"
        <View key="root">
          <UiListBox key="list" onInsert="listInsert" acceptedDragTypes="text/plain">
            <UiListBoxItem key="list-a" id="list-a" isDraggable={true} dragType="text/plain" dragValue="a">A</UiListBoxItem>
            <UiDropIndicator key="list-before" targetKey="list-a" dropPosition="before" />
          </UiListBox>
          <UiGridList key="grid" onItemDrop="gridItemDrop" acceptedDragTypes="text/plain">
            <UiGridListItem key="grid-a" id="grid-a" isDraggable={true} dragType="text/plain" dragValue="a">A</UiGridListItem>
          </UiGridList>
          <UiTree key="tree" onRootDrop="treeRootDrop" acceptedDragTypes="text/plain">
            <UiTreeItem key="tree-a" id="tree-a" isDraggable={true} dragType="text/plain" dragValue="a">A</UiTreeItem>
          </UiTree>
          <UiTable key="table" onDrop="tableDrop" acceptedDragTypes="text/plain">
            <UiTableBody key="body">
              <UiTableRow key="row-a" id="row-a" isDraggable={true} dragType="text/plain" dragValue="a" />
            </UiTableBody>
          </UiTable>
        </View>
        "#,
    )
    .unwrap()
    .use_reducer("listInsert", |_state: &mut (), _| Ok(()))
    .use_reducer("gridItemDrop", |_state: &mut (), _| Ok(()))
    .use_reducer("treeRootDrop", |_state: &mut (), _| Ok(()))
    .use_reducer("tableDrop", |_state: &mut (), _| Ok(()));

    let frame = component.render(&()).unwrap();
    for (slot, event, action) in [
        ("list-box", "onInsert", "listInsert"),
        ("grid-list", "onItemDrop", "gridItemDrop"),
        ("tree", "onRootDrop", "treeRootDrop"),
        ("table", "onDrop", "tableDrop"),
    ] {
        let props = props_by_slot(&frame.root, slot);
        assert_eq!(
            props.events.get(event).map(String::as_str),
            Some(action),
            "{slot} should retain {event}"
        );
        assert_eq!(
            props
                .attributes
                .get("data-draggable-collection")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(
            props
                .attributes
                .get("data-droppable-collection")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(
            props
                .attributes
                .get("data-drop-orientation")
                .map(String::as_str),
            Some("vertical")
        );
    }

    for (slot, key) in [
        ("list-box-item", "list-a"),
        ("grid-list-item", "grid-a"),
        ("tree-item", "tree-a"),
        ("table-row", "row-a"),
    ] {
        let props = props_by_slot(&frame.root, slot);
        assert_eq!(
            props
                .attributes
                .get("data-collection-drop-item")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(
            props.attributes.get("dragType").map(String::as_str),
            Some("text/plain")
        );
        assert_eq!(
            props.attributes.get("draggable").map(String::as_str),
            Some("true")
        );
        assert_eq!(
            props
                .attributes
                .get("data-collection-key")
                .map(String::as_str),
            Some(key)
        );
    }

    let indicator = props_by_slot(&frame.root, "drop-indicator");
    assert_eq!(
        indicator
            .attributes
            .get("data-collection-drop-indicator")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        indicator
            .attributes
            .get("data-drop-target-key")
            .map(String::as_str),
        Some("list-a")
    );
    assert_eq!(
        indicator
            .attributes
            .get("data-drop-position")
            .map(String::as_str),
        Some("before")
    );
}

fn props_by_slot<'a>(node: &'a CompiledRsxNode, slot: &str) -> &'a crate::compiler::CompiledProps {
    let CompiledRsxNode::Element {
        props, children, ..
    } = node
    else {
        panic!("compiled node should be an element")
    };
    if props.attributes.get("data-slot").map(String::as_str) == Some(slot) {
        return props;
    }
    children
        .iter()
        .find_map(|child| find_props_by_slot(child, slot))
        .unwrap_or_else(|| panic!("missing slot {slot:?}"))
}

fn find_props_by_slot<'a>(
    node: &'a CompiledRsxNode,
    slot: &str,
) -> Option<&'a crate::compiler::CompiledProps> {
    let CompiledRsxNode::Element {
        props, children, ..
    } = node
    else {
        return None;
    };
    if props.attributes.get("data-slot").map(String::as_str) == Some(slot) {
        return Some(props);
    }
    children
        .iter()
        .find_map(|child| find_props_by_slot(child, slot))
}
