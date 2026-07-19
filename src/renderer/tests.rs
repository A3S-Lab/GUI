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
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("A")),
        )
        .child(
            NativeElement::new("b", NativeRole::Button).with_props(NativeProps::new().label("B")),
        );
    let second = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("b", NativeRole::Button).with_props(NativeProps::new().label("B")),
        )
        .child(
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("A")),
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
    let text = NativeElement::new("password", NativeRole::TextField).with_props(NativeProps::new());
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
    let first =
        NativeElement::new("root", NativeRole::View).with_props(NativeProps::new().label("Old"));
    let failed =
        NativeElement::new("root", NativeRole::View).with_props(NativeProps::new().label("Failed"));
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
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("New")),
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
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("A")),
        )
        .child(
            NativeElement::new("b", NativeRole::Button).with_props(NativeProps::new().label("B")),
        );
    let second = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("b", NativeRole::Button).with_props(NativeProps::new().label("B")),
        )
        .child(
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("A")),
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
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("New")),
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
            NativeElement::new("a", NativeRole::Button).with_props(NativeProps::new().label("A")),
        )
        .child(
            NativeElement::new("b", NativeRole::Button).with_props(NativeProps::new().label("B")),
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
