use super::*;
use crate::error::GuiError;

#[derive(Debug, Clone)]
struct NativeOwnerFrame {
    path: Vec<String>,
    role: NativeRole,
    has_action: bool,
    controlled_selection: Option<Selection>,
    default_selection: Option<Selection>,
    disabled_keys: BTreeSet<CollectionKey>,
}

pub(super) fn project_native_tree(
    root: &mut NativeElement,
    collections: &BTreeMap<Vec<String>, &MountedCollection>,
) {
    let mut path = vec![root.key.as_str().to_string()];
    let mut owners = Vec::new();
    project_native_node(root, &mut path, &mut owners, collections);
}

fn project_native_node(
    element: &mut NativeElement,
    path: &mut Vec<String>,
    owners: &mut Vec<NativeOwnerFrame>,
    collections: &BTreeMap<Vec<String>, &MountedCollection>,
) {
    let pushed_owner = is_selection_container(element.role);
    if pushed_owner {
        owners.push(NativeOwnerFrame {
            path: path.clone(),
            role: element.role,
            has_action: element
                .props
                .web
                .events
                .get("onAction")
                .is_some_and(|action| !action.is_empty()),
            controlled_selection: controlled_selection(&element.props),
            default_selection: attribute(
                &element.props,
                &["defaultSelectedKeys", "data-default-selected-keys"],
            )
            .and_then(decode_selection)
            .or_else(|| {
                attribute(&element.props, &["defaultValue", "data-default-value"])
                    .filter(|value| !value.is_empty())
                    .map(|value| Selection::keys([CollectionKey::new(value)]))
            }),
            disabled_keys: attribute(&element.props, &["disabledKeys", "data-disabled-keys"])
                .and_then(decode_selection)
                .and_then(|selection| selection.explicit_keys().cloned())
                .unwrap_or_default(),
        });
    }

    if is_selection_item(element.role) {
        if owners.iter().rev().any(|owner| owner.has_action) {
            element.props.metadata.insert(
                super::super::COLLECTION_ACTION_METADATA_KEY.to_string(),
                "true".to_string(),
            );
        } else {
            element
                .props
                .metadata
                .remove(super::super::COLLECTION_ACTION_METADATA_KEY);
        }
        if let Some(owner) = native_owner_frame(owners) {
            let key = collection_key(element.key.as_str(), &element.props);
            let selected = if let Some(selection) = &owner.controlled_selection {
                Some(selection_selects_item(
                    selection,
                    &key,
                    &element.props,
                    &owner.disabled_keys,
                ))
            } else if let Some(collection) = collections.get(&owner.path) {
                Some(match collection.manager.selection() {
                    Selection::All => {
                        !element.props.disabled
                            && !collection.manager.is_selection_disabled(&key)
                            && !key_set_matches_item(&owner.disabled_keys, &key, &element.props)
                    }
                    Selection::Keys(keys) => keys.contains(&key),
                })
            } else {
                owner.default_selection.as_ref().map(|selection| {
                    selection_selects_item(selection, &key, &element.props, &owner.disabled_keys)
                })
            };
            if let Some(selected) = selected {
                apply_item_selection_props(&mut element.props, element.role, selected);
            }
        }
    }

    for child in &mut element.children {
        path.push(child.key.as_str().to_string());
        project_native_node(child, path, owners, collections);
        path.pop();
    }
    if pushed_owner {
        owners.pop();
    }
}

fn native_owner_frame(owners: &[NativeOwnerFrame]) -> Option<&NativeOwnerFrame> {
    let nearest = owners.last()?;
    if nearest.role == NativeRole::TabList {
        if let Some(tabs) = owners
            .iter()
            .rev()
            .skip(1)
            .find(|owner| owner.role == NativeRole::Tabs)
        {
            return Some(tabs);
        }
    }
    if nearest.role == NativeRole::ListBox {
        if let Some(input) = owners
            .iter()
            .rev()
            .skip(1)
            .find(|owner| matches!(owner.role, NativeRole::ComboBox | NativeRole::Select))
        {
            return Some(input);
        }
    }
    Some(nearest)
}

fn controlled_selection(props: &NativeProps) -> Option<Selection> {
    attribute(props, &["selectedKeys", "data-selected-keys"])
        .and_then(decode_selection)
        .or_else(|| {
            props
                .value
                .as_deref()
                .filter(|value| !value.is_empty())
                .map(|value| Selection::keys([CollectionKey::new(value)]))
        })
}

fn selection_selects_item(
    selection: &Selection,
    key: &CollectionKey,
    props: &NativeProps,
    disabled_keys: &BTreeSet<CollectionKey>,
) -> bool {
    match selection {
        Selection::All => !props.disabled && !key_set_matches_item(disabled_keys, key, props),
        Selection::Keys(keys) => key_set_matches_item(keys, key, props),
    }
}

fn key_set_matches_item(
    keys: &BTreeSet<CollectionKey>,
    key: &CollectionKey,
    props: &NativeProps,
) -> bool {
    keys.contains(key)
        || props
            .value
            .as_deref()
            .is_some_and(|value| keys.contains(&CollectionKey::new(value)))
        || props
            .label
            .as_deref()
            .is_some_and(|label| keys.contains(&CollectionKey::new(label)))
}

pub(crate) fn apply_item_selection_props(
    props: &mut NativeProps,
    role: NativeRole,
    selected: bool,
) {
    props.selected = selected;
    set_existing_boolean_metadata(props, "aria-selected", selected);
    set_existing_boolean_metadata(props, "data-selected", selected);
    if role == NativeRole::Radio {
        props.checked = Some(selected);
        set_existing_boolean_metadata(props, "aria-checked", selected);
    }
}

fn set_existing_boolean_metadata(props: &mut NativeProps, name: &str, value: bool) {
    if !props.web.attributes.contains_key(name) && !props.metadata.contains_key(name) {
        return;
    }
    let value = value.to_string();
    props.web.attributes.insert(name.to_string(), value.clone());
    props.metadata.insert(name.to_string(), value);
}

pub(super) fn mounted_node_path(
    node: HostNodeId,
    nodes: &BTreeMap<HostNodeId, &MountedNodeSnapshot>,
) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = Some(node);
    while let Some(snapshot) = current.and_then(|current| nodes.get(&current).copied()) {
        path.push(snapshot.key.as_str().to_string());
        current = snapshot.parent;
    }
    path.reverse();
    path
}

pub(crate) fn validate_native_collection_keys(root: &NativeElement) -> GuiResult<()> {
    let mut records = Vec::new();
    collect_native_records(root, None, &mut records);
    let nodes = records
        .iter()
        .map(|record| (record.id, record))
        .collect::<BTreeMap<_, _>>();
    let mut keys_by_collection = BTreeMap::<usize, BTreeSet<CollectionKey>>::new();

    for item in records
        .iter()
        .filter(|record| is_selection_item(record.element.role))
    {
        let Some(owner) = native_selection_owner(item.parent, &nodes) else {
            continue;
        };
        let key = collection_key(item.element.key.as_str(), &item.element.props);
        if !keys_by_collection
            .entry(owner)
            .or_default()
            .insert(key.clone())
        {
            return Err(GuiError::invalid_tree(format!(
                "native collection items need unique stable keys; duplicate key {:?}",
                key.as_str()
            )));
        }
    }
    Ok(())
}

#[derive(Debug)]
struct NativeSelectionRecord<'a> {
    id: usize,
    parent: Option<usize>,
    element: &'a NativeElement,
}

fn collect_native_records<'a>(
    element: &'a NativeElement,
    parent: Option<usize>,
    records: &mut Vec<NativeSelectionRecord<'a>>,
) {
    let id = records.len();
    records.push(NativeSelectionRecord {
        id,
        parent,
        element,
    });
    for child in &element.children {
        collect_native_records(child, Some(id), records);
    }
}

fn native_selection_owner(
    mut parent: Option<usize>,
    nodes: &BTreeMap<usize, &NativeSelectionRecord<'_>>,
) -> Option<usize> {
    let mut nearest = None;
    while let Some(node) = parent.and_then(|node| nodes.get(&node).copied()) {
        let role = node.element.role;
        if is_selection_container(role) {
            if nearest.is_none() {
                nearest = Some((node.id, role));
            }
            if nearest.is_some_and(|(_, nearest_role)| nearest_role == NativeRole::TabList)
                && role == NativeRole::Tabs
            {
                return Some(node.id);
            }
            if nearest.is_some_and(|(_, nearest_role)| nearest_role == NativeRole::ListBox)
                && matches!(role, NativeRole::ComboBox | NativeRole::Select)
            {
                return Some(node.id);
            }
        }
        parent = node.parent;
    }
    nearest.map(|(node, _)| node)
}
