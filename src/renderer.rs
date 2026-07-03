use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};
use crate::host::{HostNodeId, NativeHost};
use crate::html::HTML_TAG_METADATA_KEY;
use crate::native::{
    normalize_props_for_native_role, ElementKey, NativeElement, NativeProps, NativeRole,
};

#[derive(Debug, Clone, PartialEq)]
struct MountedNode {
    id: HostNodeId,
    key: ElementKey,
    role: NativeRole,
    props: NativeProps,
    children: Vec<MountedNode>,
}

#[derive(Debug, Default)]
pub struct Renderer {
    root: Option<MountedNode>,
}

#[derive(Debug, Default)]
struct ReconcileRollback {
    new_mounts: Vec<MountedNode>,
    updated_props: Vec<(HostNodeId, NativeProps)>,
    child_orders: Vec<(HostNodeId, Vec<HostNodeId>)>,
    deferred_unmounts: Vec<MountedNode>,
}

impl ReconcileRollback {
    fn record_new_mount(&mut self, mounted: MountedNode) {
        self.new_mounts.push(mounted);
    }

    fn record_update(&mut self, id: HostNodeId, previous_props: NativeProps) {
        self.updated_props.push((id, previous_props));
    }

    fn record_child_order(&mut self, parent: HostNodeId, children: Vec<HostNodeId>) {
        if self
            .child_orders
            .iter()
            .any(|(recorded_parent, _)| *recorded_parent == parent)
        {
            return;
        }
        self.child_orders.push((parent, children));
    }

    fn record_deferred_unmount(&mut self, mounted: MountedNode) {
        self.deferred_unmounts.push(mounted);
    }

    fn commit_deferred_unmounts<H: NativeHost>(&mut self, host: &mut H) -> GuiResult<()> {
        for mounted in std::mem::take(&mut self.deferred_unmounts) {
            unmount_node(mounted, host)?;
        }
        Ok(())
    }

    fn rollback<H: NativeHost>(&mut self, host: &mut H) {
        for mounted in std::mem::take(&mut self.new_mounts).into_iter().rev() {
            best_effort_unmount_node(mounted, host);
        }
        for (parent, children) in std::mem::take(&mut self.child_orders).into_iter().rev() {
            restore_child_order(parent, children, host);
        }
        for (id, props) in std::mem::take(&mut self.updated_props).into_iter().rev() {
            let _ = host.update(id, &props);
        }
        self.deferred_unmounts.clear();
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render<H: NativeHost>(
        &mut self,
        element: &NativeElement,
        host: &mut H,
    ) -> GuiResult<HostNodeId> {
        let element = normalize_native_element(element);
        validate_native_tree(&element)?;
        let previous_root = self.root.clone();
        let mut rollback = ReconcileRollback::default();
        let mounted = match self.root.take() {
            Some(old) if needs_replacement(&old, &element) => {
                let mounted = match mount_node(None, 0, &element, host) {
                    Ok(mounted) => mounted,
                    Err(error) => {
                        self.root = Some(old);
                        return Err(error);
                    }
                };
                rollback.record_new_mount(mounted.clone());
                rollback.record_deferred_unmount(old);
                mounted
            }
            Some(old) => match reconcile_node(None, 0, old, &element, host, &mut rollback) {
                Ok(mounted) => mounted,
                Err(error) => {
                    rollback.rollback(host);
                    self.root = previous_root;
                    return Err(error);
                }
            },
            None => mount_node(None, 0, &element, host)?,
        };
        if let Err(error) = host.set_root(mounted.id) {
            if previous_root.is_none() {
                best_effort_unmount_node(mounted, host);
            } else {
                rollback.rollback(host);
            }
            self.root = previous_root;
            return Err(error);
        }
        if let Err(error) = rollback.commit_deferred_unmounts(host) {
            rollback.rollback(host);
            if let Some(previous_root) = &previous_root {
                let _ = host.set_root(previous_root.id);
            }
            self.root = previous_root;
            return Err(error);
        }
        let root_id = mounted.id;
        self.root = Some(mounted);
        Ok(root_id)
    }

    pub fn mounted_node_ids(&self) -> BTreeSet<HostNodeId> {
        let mut ids = BTreeSet::new();
        if let Some(root) = &self.root {
            collect_mounted_node_ids(root, &mut ids);
        }
        ids
    }

    pub fn mounted_node_props(&self) -> Vec<(HostNodeId, NativeProps)> {
        let mut props = Vec::new();
        if let Some(root) = &self.root {
            collect_mounted_node_props(root, &mut props);
        }
        props
    }

    pub fn ancestor_ids(&self, node: HostNodeId) -> Vec<HostNodeId> {
        let mut ancestors = Vec::new();
        if let Some(root) = &self.root {
            collect_ancestor_ids(root, node, &mut ancestors);
        }
        ancestors
    }
}

fn normalize_native_element(element: &NativeElement) -> NativeElement {
    NativeElement {
        key: element.key.clone(),
        role: element.role,
        props: normalize_props_for_native_role(element.role, &element.props),
        children: element
            .children
            .iter()
            .map(normalize_native_element)
            .collect(),
    }
}

fn validate_native_tree(element: &NativeElement) -> GuiResult<()> {
    if element.key.as_str().is_empty() {
        return Err(GuiError::invalid_tree(
            "a3s-gui native elements need non-empty keys",
        ));
    }

    let mut sibling_keys = BTreeSet::new();
    for child in &element.children {
        validate_native_tree(child)?;
        let key = child.key.as_str();
        if !sibling_keys.insert(key) {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui native sibling elements need unique keys; duplicate key {key:?}"
            )));
        }
    }
    Ok(())
}

fn needs_replacement(old: &MountedNode, new: &NativeElement) -> bool {
    old.key != new.key
        || old.role != new.role
        || native_widget_shape(old.role, &old.props) != native_widget_shape(new.role, &new.props)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeWidgetShape {
    Default,
    Number,
    Password,
    Search,
    TextArea,
}

fn native_widget_shape(role: NativeRole, props: &NativeProps) -> NativeWidgetShape {
    if role == NativeRole::TextField
        && props
            .metadata
            .get(HTML_TAG_METADATA_KEY)
            .is_some_and(|tag| tag == "textarea")
    {
        NativeWidgetShape::TextArea
    } else if role == NativeRole::TextField
        && props
            .input_type
            .as_deref()
            .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("search"))
    {
        NativeWidgetShape::Search
    } else if role == NativeRole::TextField
        && props
            .input_type
            .as_deref()
            .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("number"))
    {
        NativeWidgetShape::Number
    } else if role == NativeRole::TextField
        && props
            .input_type
            .as_deref()
            .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("password"))
    {
        NativeWidgetShape::Password
    } else {
        NativeWidgetShape::Default
    }
}

fn collect_mounted_node_ids(node: &MountedNode, ids: &mut BTreeSet<HostNodeId>) {
    ids.insert(node.id);
    for child in &node.children {
        collect_mounted_node_ids(child, ids);
    }
}

fn collect_mounted_node_props(node: &MountedNode, props: &mut Vec<(HostNodeId, NativeProps)>) {
    props.push((node.id, node.props.clone()));
    for child in &node.children {
        collect_mounted_node_props(child, props);
    }
}

fn collect_ancestor_ids(
    node: &MountedNode,
    target: HostNodeId,
    ancestors: &mut Vec<HostNodeId>,
) -> bool {
    if node.id == target {
        return true;
    }

    for child in &node.children {
        if collect_ancestor_ids(child, target, ancestors) {
            ancestors.push(node.id);
            return true;
        }
    }

    false
}

fn mount_node<H: NativeHost>(
    parent: Option<HostNodeId>,
    index: usize,
    element: &NativeElement,
    host: &mut H,
) -> GuiResult<MountedNode> {
    let id = host.create(element)?;
    let mut children = Vec::with_capacity(element.children.len());
    for (child_index, child) in element.children.iter().enumerate() {
        match mount_node(Some(id), child_index, child, host) {
            Ok(child) => children.push(child),
            Err(error) => {
                best_effort_unmount_node(
                    MountedNode {
                        id,
                        key: element.key.clone(),
                        role: element.role,
                        props: element.props.clone(),
                        children,
                    },
                    host,
                );
                return Err(error);
            }
        }
    }
    if let Some(parent) = parent {
        if let Err(error) = host.insert_child(parent, id, index) {
            best_effort_unmount_node(
                MountedNode {
                    id,
                    key: element.key.clone(),
                    role: element.role,
                    props: element.props.clone(),
                    children,
                },
                host,
            );
            return Err(error);
        }
    }
    Ok(MountedNode {
        id,
        key: element.key.clone(),
        role: element.role,
        props: element.props.clone(),
        children,
    })
}

fn reconcile_node<H: NativeHost>(
    parent: Option<HostNodeId>,
    index: usize,
    old: MountedNode,
    new: &NativeElement,
    host: &mut H,
    rollback: &mut ReconcileRollback,
) -> GuiResult<MountedNode> {
    if needs_replacement(&old, new) {
        let mounted = mount_node(parent, index, new, host)?;
        rollback.record_new_mount(mounted.clone());
        rollback.record_deferred_unmount(old);
        return Ok(mounted);
    }

    if old.props != new.props {
        let previous_props = old.props.clone();
        host.update(old.id, &new.props)?;
        rollback.record_update(old.id, previous_props);
    }

    let children = reconcile_children(old.id, old.children, &new.children, host, rollback)?;
    Ok(MountedNode {
        id: old.id,
        key: old.key,
        role: old.role,
        props: new.props.clone(),
        children,
    })
}

fn reconcile_children<H: NativeHost>(
    parent: HostNodeId,
    old_children: Vec<MountedNode>,
    new_children: &[NativeElement],
    host: &mut H,
    rollback: &mut ReconcileRollback,
) -> GuiResult<Vec<MountedNode>> {
    let mut mounted_children = Vec::with_capacity(new_children.len());
    let previous_child_order = old_children
        .iter()
        .map(|child| child.id)
        .collect::<Vec<_>>();
    let mut old_by_key: BTreeMap<ElementKey, (usize, MountedNode)> = old_children
        .into_iter()
        .enumerate()
        .map(|(index, child)| (child.key.clone(), (index, child)))
        .collect();

    for (index, new_child) in new_children.iter().enumerate() {
        match old_by_key.remove(&new_child.key) {
            Some((old_index, old_child)) => {
                let old_id = old_child.id;
                let mounted =
                    reconcile_node(Some(parent), index, old_child, new_child, host, rollback)?;
                if mounted.id == old_id && old_index != index {
                    rollback.record_child_order(parent, previous_child_order.clone());
                    host.insert_child(parent, mounted.id, index)?;
                }
                mounted_children.push(mounted);
            }
            None => match mount_node(Some(parent), index, new_child, host) {
                Ok(mounted) => {
                    rollback.record_new_mount(mounted.clone());
                    mounted_children.push(mounted);
                }
                Err(error) => return Err(error),
            },
        }
    }

    for (_, old_child) in old_by_key.into_values() {
        rollback.record_deferred_unmount(old_child);
    }

    Ok(mounted_children)
}

fn restore_child_order<H: NativeHost>(parent: HostNodeId, children: Vec<HostNodeId>, host: &mut H) {
    for (index, child) in children.into_iter().enumerate() {
        let _ = host.insert_child(parent, child, index);
    }
}

fn unmount_node<H: NativeHost>(node: MountedNode, host: &mut H) -> GuiResult<()> {
    host.remove(node.id)
}

fn best_effort_unmount_node<H: NativeHost>(node: MountedNode, host: &mut H) {
    let _ = host.remove(node.id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{HeadlessHost, HostOperation};
    use crate::html::HTML_TAG_METADATA_KEY;
    use crate::native::{NativeElement, NativeProps, NativeRole};

    #[derive(Default)]
    struct FailingUpdateHost {
        inner: HeadlessHost,
        create_calls: usize,
        fail_create_call: Option<usize>,
        fail_inserts: bool,
        fail_set_root: bool,
        fail_updates: bool,
    }

    impl FailingUpdateHost {
        fn root(&self) -> Option<HostNodeId> {
            self.inner.root()
        }

        fn node(&self, id: HostNodeId) -> Option<&crate::host::HeadlessNode> {
            self.inner.node(id)
        }

        fn operations(&self) -> &[HostOperation] {
            self.inner.operations()
        }

        fn nodes(&self) -> &BTreeMap<HostNodeId, crate::host::HeadlessNode> {
            self.inner.nodes()
        }

        fn clear_operations(&mut self) {
            self.inner.clear_operations();
        }
    }

    impl NativeHost for FailingUpdateHost {
        fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
            self.create_calls += 1;
            if self.fail_create_call == Some(self.create_calls) {
                return Err(GuiError::host("forced host create failure"));
            }
            self.inner.create(element)
        }

        fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
            if self.fail_updates {
                return Err(GuiError::host("forced host update failure"));
            }
            self.inner.update(id, props)
        }

        fn insert_child(
            &mut self,
            parent: HostNodeId,
            child: HostNodeId,
            index: usize,
        ) -> GuiResult<()> {
            if self.fail_inserts {
                return Err(GuiError::host("forced host insert failure"));
            }
            self.inner.insert_child(parent, child, index)
        }

        fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
            self.inner.remove(id)
        }

        fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
            if self.fail_set_root {
                return Err(GuiError::host("forced host set_root failure"));
            }
            self.inner.set_root(id)
        }
    }

    #[test]
    fn keyed_children_are_reordered_without_remounting() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            );
        let second = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            )
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.clear_operations();
        let second_root_id = renderer.render(&second, &mut host).unwrap();

        assert_eq!(root_id, second_root_id);
        assert!(!host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { .. } | HostOperation::Remove { .. }
        )));
        assert!(host
            .operations()
            .iter()
            .any(|operation| matches!(operation, HostOperation::InsertChild { parent, index, .. } if *parent == root_id && *index == 0)));

        let labels: Vec<_> = host
            .node(root_id)
            .unwrap()
            .children
            .iter()
            .map(|id| host.node(*id).unwrap().props.label.as_deref().unwrap())
            .collect();
        assert_eq!(labels, vec!["B", "A"]);
    }

    #[test]
    fn text_field_textarea_shape_changes_remount_same_key_role() {
        let single_line =
            NativeElement::new("message", NativeRole::TextField).with_props(NativeProps::new());
        let textarea = NativeElement::new("message", NativeRole::TextField)
            .with_props(NativeProps::new().metadata(HTML_TAG_METADATA_KEY, "textarea"));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let first_id = renderer.render(&single_line, &mut host).unwrap();
        host.clear_operations();
        let second_id = renderer.render(&textarea, &mut host).unwrap();

        assert_ne!(first_id, second_id);
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { id, .. } if *id == second_id
        )));
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Remove { id } if *id == first_id
        )));
    }

    #[test]
    fn text_field_password_shape_changes_remount_same_key_role() {
        let text =
            NativeElement::new("password", NativeRole::TextField).with_props(NativeProps::new());
        let password = NativeElement::new("password", NativeRole::TextField)
            .with_props(NativeProps::new().input_type("password"));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let first_id = renderer.render(&text, &mut host).unwrap();
        host.clear_operations();
        let second_id = renderer.render(&password, &mut host).unwrap();

        assert_ne!(first_id, second_id);
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { id, .. } if *id == second_id
        )));
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Remove { id } if *id == first_id
        )));
    }

    #[test]
    fn text_field_search_shape_changes_remount_same_key_role() {
        let text = NativeElement::new("query", NativeRole::TextField)
            .with_props(NativeProps::new().input_type("text"));
        let search = NativeElement::new("query", NativeRole::TextField)
            .with_props(NativeProps::new().input_type("search"));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let first_id = renderer.render(&text, &mut host).unwrap();
        host.clear_operations();
        let second_id = renderer.render(&search, &mut host).unwrap();

        assert_ne!(first_id, second_id);
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { id, .. } if *id == second_id
        )));
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Remove { id } if *id == first_id
        )));
    }

    #[test]
    fn text_field_number_shape_changes_remount_same_key_role() {
        let text = NativeElement::new("quantity", NativeRole::TextField)
            .with_props(NativeProps::new().input_type("text"));
        let number = NativeElement::new("quantity", NativeRole::TextField)
            .with_props(NativeProps::new().input_type("number"));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let first_id = renderer.render(&text, &mut host).unwrap();
        host.clear_operations();
        let second_id = renderer.render(&number, &mut host).unwrap();

        assert_ne!(first_id, second_id);
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { id, .. } if *id == second_id
        )));
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Remove { id } if *id == first_id
        )));
    }

    #[test]
    fn renderer_removes_deferred_subtrees_with_one_host_remove() {
        let first = NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("group", NativeRole::View)
                .child(NativeElement::new("save", NativeRole::Button)),
        );
        let second = NativeElement::new("root", NativeRole::View);
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let group_id = host.node(root_id).unwrap().children[0];
        host.clear_operations();

        renderer.render(&second, &mut host).unwrap();

        assert_eq!(
            host.operations()
                .iter()
                .filter(|operation| matches!(operation, HostOperation::Remove { .. }))
                .count(),
            1
        );
        assert!(host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Remove { id } if *id == group_id
        )));
        assert_eq!(
            host.node(root_id).unwrap().children,
            Vec::<HostNodeId>::new()
        );
        assert!(!renderer.mounted_node_ids().contains(&group_id));
    }

    #[test]
    fn renderer_rejects_unstable_native_keys_before_mounting() {
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let empty_key = NativeElement::new("", NativeRole::View);
        let error = renderer.render(&empty_key, &mut host).unwrap_err();

        assert!(error
            .to_string()
            .contains("native elements need non-empty keys"));
        assert!(host.operations().is_empty());

        let duplicate_child_key = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("item", NativeRole::Button))
            .child(NativeElement::new("item", NativeRole::Text));
        let error = renderer
            .render(&duplicate_child_key, &mut host)
            .unwrap_err();

        assert!(error
            .to_string()
            .contains("native sibling elements need unique keys"));
        assert!(host.operations().is_empty());
    }

    #[test]
    fn renderer_preserves_mounted_tree_after_host_update_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().label("Old"));
        let failed = NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().label("Failed"));
        let recovered = NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().label("Recovered"));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.fail_updates = true;
        let error = renderer.render(&failed, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host update failure"));
        assert!(renderer.mounted_node_ids().contains(&root_id));
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(
            host.node(root_id).unwrap().props.label.as_deref(),
            Some("Old")
        );

        host.fail_updates = false;
        host.clear_operations();
        let recovered_id = renderer.render(&recovered, &mut host).unwrap();

        assert_eq!(recovered_id, root_id);
        assert!(!host.operations().iter().any(|operation| matches!(
            operation,
            HostOperation::Create { .. } | HostOperation::Remove { .. }
        )));
        assert_eq!(
            host.node(root_id).unwrap().props.label.as_deref(),
            Some("Recovered")
        );
    }

    #[test]
    fn renderer_cleans_up_partial_first_mount_after_child_create_failure() {
        let tree = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost {
            fail_create_call: Some(2),
            ..FailingUpdateHost::default()
        };

        let error = renderer.render(&tree, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert!(renderer.mounted_node_ids().is_empty());
        assert!(host.nodes().is_empty());
        assert!(host.root().is_none());
    }

    #[test]
    fn renderer_cleans_up_partial_first_mount_after_child_insert_failure() {
        let tree = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost {
            fail_inserts: true,
            ..FailingUpdateHost::default()
        };

        let error = renderer.render(&tree, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host insert failure"));
        assert!(renderer.mounted_node_ids().is_empty());
        assert!(host.nodes().is_empty());
        assert!(host.root().is_none());
    }

    #[test]
    fn renderer_cleans_up_incremental_child_mount_after_later_create_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button));
        let second = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let first_child = host.node(root_id).unwrap().children[0];
        host.fail_create_call = Some(host.create_calls + 2);
        let error = renderer.render(&second, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 2);
        assert!(renderer.mounted_node_ids().contains(&root_id));
        assert!(renderer.mounted_node_ids().contains(&first_child));
        assert_eq!(host.nodes().len(), 2);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, vec![first_child]);
        assert_eq!(host.node(first_child).unwrap().role, NativeRole::Button);
    }

    #[test]
    fn renderer_rolls_back_incremental_updates_after_later_create_failure() {
        let first = NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("Old")),
        );
        let second = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("New")),
            )
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let first_child = host.node(root_id).unwrap().children[0];
        host.fail_create_call = Some(host.create_calls + 2);
        let error = renderer.render(&second, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 2);
        assert_eq!(host.nodes().len(), 2);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, vec![first_child]);
        assert_eq!(
            host.node(first_child).unwrap().props.label.as_deref(),
            Some("Old")
        );
    }

    #[test]
    fn renderer_rolls_back_child_reorder_after_later_create_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            );
        let second = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            )
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let original_children = host.node(root_id).unwrap().children.clone();
        host.fail_create_call = Some(host.create_calls + 1);
        let error = renderer.render(&second, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 3);
        assert_eq!(host.nodes().len(), 3);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, original_children);
    }

    #[test]
    fn renderer_rolls_back_incremental_changes_after_set_root_failure() {
        let first = NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("Old")),
        );
        let second = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("New")),
            )
            .child(NativeElement::new("b", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let first_child = host.node(root_id).unwrap().children[0];
        host.fail_set_root = true;
        let error = renderer.render(&second, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host set_root failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 2);
        assert_eq!(host.nodes().len(), 2);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, vec![first_child]);
        assert_eq!(
            host.node(first_child).unwrap().props.label.as_deref(),
            Some("Old")
        );
    }

    #[test]
    fn renderer_rolls_back_child_replacement_after_set_root_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Text));
        let replacement = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let child_id = host.node(root_id).unwrap().children[0];
        host.fail_set_root = true;
        let error = renderer.render(&replacement, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host set_root failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 2);
        assert!(renderer.mounted_node_ids().contains(&child_id));
        assert_eq!(host.nodes().len(), 2);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, vec![child_id]);
        assert_eq!(host.node(child_id).unwrap().role, NativeRole::Text);
    }

    #[test]
    fn renderer_rolls_back_child_removal_after_set_root_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button));
        let removal = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let original_children = host.node(root_id).unwrap().children.clone();
        host.fail_set_root = true;
        let error = renderer.render(&removal, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host set_root failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 3);
        assert_eq!(host.nodes().len(), 3);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, original_children);
    }

    #[test]
    fn renderer_cleans_up_first_mount_after_set_root_failure() {
        let tree = NativeElement::new("root", NativeRole::View);
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost {
            fail_set_root: true,
            ..FailingUpdateHost::default()
        };

        let error = renderer.render(&tree, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host set_root failure"));
        assert!(renderer.mounted_node_ids().is_empty());
        assert!(host.nodes().is_empty());
        assert!(host.root().is_none());
    }

    #[test]
    fn renderer_preserves_root_after_replacement_create_failure() {
        let first = NativeElement::new("root", NativeRole::View);
        let replacement = NativeElement::new("root", NativeRole::Button);
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.fail_create_call = Some(host.create_calls + 1);
        let error = renderer.render(&replacement, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert!(renderer.mounted_node_ids().contains(&root_id));
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().role, NativeRole::View);
    }

    #[test]
    fn renderer_preserves_root_after_replacement_set_root_failure() {
        let first = NativeElement::new("root", NativeRole::View);
        let replacement = NativeElement::new("root", NativeRole::Button);
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.fail_set_root = true;
        let error = renderer.render(&replacement, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host set_root failure"));
        assert!(renderer.mounted_node_ids().contains(&root_id));
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.nodes().len(), 1);
        assert_eq!(host.node(root_id).unwrap().role, NativeRole::View);
    }

    #[test]
    fn renderer_preserves_child_after_replacement_create_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Text));
        let replacement = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("child", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let child_id = host.node(root_id).unwrap().children[0];
        host.fail_create_call = Some(host.create_calls + 1);
        let error = renderer.render(&replacement, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert!(renderer.mounted_node_ids().contains(&child_id));
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, vec![child_id]);
        assert_eq!(host.node(child_id).unwrap().role, NativeRole::Text);
    }

    #[test]
    fn renderer_rolls_back_child_replacement_after_later_create_failure() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Text))
            .child(NativeElement::new("b", NativeRole::Button));
        let failed = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = FailingUpdateHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let original_children = host.node(root_id).unwrap().children.clone();
        let original_child = original_children[0];
        host.fail_create_call = Some(host.create_calls + 2);
        let error = renderer.render(&failed, &mut host).unwrap_err();

        assert!(error.to_string().contains("forced host create failure"));
        assert_eq!(renderer.mounted_node_ids().len(), 3);
        assert!(renderer.mounted_node_ids().contains(&original_child));
        assert_eq!(host.nodes().len(), 3);
        assert_eq!(host.root(), Some(root_id));
        assert_eq!(host.node(root_id).unwrap().children, original_children);
        assert_eq!(host.node(original_child).unwrap().role, NativeRole::Text);
    }

    #[test]
    fn mounted_node_ids_follow_reconciled_tree() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button));
        let second = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&first, &mut host).unwrap();
        let first_children = host.node(root_id).unwrap().children.clone();
        let removed = first_children[0];
        renderer.render(&second, &mut host).unwrap();
        let mounted = renderer.mounted_node_ids();

        assert!(mounted.contains(&root_id));
        assert!(!mounted.contains(&removed));
        assert_eq!(mounted.len(), 3);
    }

    #[test]
    fn mounted_node_props_follow_tree_order() {
        let tree = NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().label("Root"))
            .child(
                NativeElement::new("a", NativeRole::Button)
                    .with_props(NativeProps::new().label("A")),
            )
            .child(
                NativeElement::new("b", NativeRole::Button)
                    .with_props(NativeProps::new().label("B")),
            );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        renderer.render(&tree, &mut host).unwrap();

        let labels = renderer
            .mounted_node_props()
            .into_iter()
            .map(|(_, props)| props.label)
            .collect::<Vec<_>>();
        assert_eq!(
            labels,
            vec![
                Some("Root".to_string()),
                Some("A".to_string()),
                Some("B".to_string())
            ]
        );
    }

    #[test]
    fn ancestor_ids_return_nearest_parent_first() {
        let tree = NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("group", NativeRole::View)
                .child(NativeElement::new("save", NativeRole::Button)),
        );
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let root_id = renderer.render(&tree, &mut host).unwrap();
        let group_id = host.node(root_id).unwrap().children[0];
        let save_id = host.node(group_id).unwrap().children[0];

        assert_eq!(renderer.ancestor_ids(save_id), vec![group_id, root_id]);
        assert!(renderer.ancestor_ids(root_id).is_empty());
    }
}
