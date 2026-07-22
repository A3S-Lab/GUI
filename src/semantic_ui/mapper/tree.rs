use super::*;

impl SemanticMapper {
    pub(super) fn map_tree(&self, element: &SemanticElement) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Tree).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );
        let items = direct_tree_items(&element.children);
        self.flatten_tree_items(&items, None, 1, &mut native.children)?;
        Ok(native)
    }

    fn flatten_tree_items(
        &self,
        items: &[&SemanticElement],
        parent_key: Option<&str>,
        level: u32,
        output: &mut Vec<NativeElement>,
    ) -> GuiResult<()> {
        let set_size = i32::try_from(items.len()).unwrap_or(i32::MAX);
        for (index, item) in items.iter().enumerate() {
            let position = i32::try_from(index.saturating_add(1)).unwrap_or(i32::MAX);
            output.push(self.map_tree_item_row(item, parent_key, level, position, set_size)?);
            let children = direct_tree_items(&item.children);
            if !children.is_empty() {
                let key = semantic_collection_key(item);
                self.flatten_tree_items(&children, Some(&key), level.saturating_add(1), output)?;
            }
        }
        Ok(())
    }

    pub(super) fn map_tree_item_row(
        &self,
        element: &SemanticElement,
        parent_key: Option<&str>,
        level: u32,
        position: i32,
        set_size: i32,
    ) -> GuiResult<NativeElement> {
        let has_child_items = !direct_tree_items(&element.children).is_empty()
            || semantic_boolean_attribute(element, &["hasChildItems", "data-has-child-items"]);
        let label = non_empty_clone(element.props.label.as_ref())
            .or_else(|| aria_label(&element.props))
            .or_else(|| non_empty_clone(element.props.text_value.as_ref()))
            .or_else(|| first_tree_item_text(element));
        let mut props = native_props_from_aria(&element.props, label);
        if has_child_items {
            props.expanded = Some(props.expanded.unwrap_or(false));
        }
        props.accessibility_structure.level.get_or_insert(level);
        props
            .accessibility_structure
            .position_in_set
            .get_or_insert(position);
        props
            .accessibility_structure
            .set_size
            .get_or_insert(set_size);
        insert_tree_metadata(&mut props, "data-tree-level", level.to_string());
        insert_tree_metadata(
            &mut props,
            "data-has-child-items",
            has_child_items.to_string(),
        );
        if let Some(parent_key) = parent_key {
            insert_tree_metadata(&mut props, "data-tree-parent-key", parent_key);
        }
        Ok(NativeElement::new(element.key.clone(), NativeRole::TreeItem).with_props(props))
    }
}

fn direct_tree_items(children: &[SemanticElement]) -> Vec<&SemanticElement> {
    fn collect<'a>(children: &'a [SemanticElement], items: &mut Vec<&'a SemanticElement>) {
        for child in children {
            if child.component == SemanticComponent::TreeItem {
                items.push(child);
            } else {
                collect(&child.children, items);
            }
        }
    }

    let mut items = Vec::new();
    collect(children, &mut items);
    items
}

fn first_tree_item_text(element: &SemanticElement) -> Option<String> {
    if let Some(text) = direct_text_children(element) {
        return Some(text);
    }
    element.children.iter().find_map(|child| {
        (child.component != SemanticComponent::TreeItem)
            .then(|| first_tree_item_text(child))
            .flatten()
    })
}

fn semantic_collection_key(element: &SemanticElement) -> String {
    element
        .props
        .web
        .attributes
        .get("data-collection-key")
        .filter(|key| !key.is_empty())
        .cloned()
        .unwrap_or_else(|| element.key.clone())
}

fn semantic_boolean_attribute(element: &SemanticElement, names: &[&str]) -> bool {
    names.iter().any(|name| {
        element
            .props
            .web
            .attributes
            .get(*name)
            .is_some_and(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "" | "true" | "1" | "yes"
                )
            })
    })
}

fn insert_tree_metadata(props: &mut NativeProps, name: &str, value: impl Into<String>) {
    let value = value.into();
    props.web.attributes.insert(name.to_string(), value.clone());
    props.metadata.insert(name.to_string(), value);
}
