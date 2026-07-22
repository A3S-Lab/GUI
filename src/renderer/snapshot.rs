use super::{MountedNode, Renderer};
use crate::host::HostNodeId;
use crate::native::{ElementKey, NativeProps, NativeRole};

/// A stable, read-only view of one node in the currently mounted native tree.
#[derive(Debug, Clone, PartialEq)]
pub struct MountedNodeSnapshot {
    pub node: HostNodeId,
    pub parent: Option<HostNodeId>,
    /// Stable declarative identity used to reconcile this node.
    pub key: ElementKey,
    pub role: NativeRole,
    pub props: NativeProps,
}

impl Renderer {
    pub fn mounted_snapshot(&self) -> Vec<MountedNodeSnapshot> {
        let mut snapshot = Vec::new();
        if let Some(root) = &self.root {
            collect_snapshot(root, None, &mut snapshot);
        }
        snapshot
    }
}

fn collect_snapshot(
    mounted: &MountedNode,
    parent: Option<HostNodeId>,
    snapshot: &mut Vec<MountedNodeSnapshot>,
) {
    snapshot.push(MountedNodeSnapshot {
        node: mounted.id,
        parent,
        key: mounted.key.clone(),
        role: mounted.role,
        props: mounted.props.clone(),
    });
    for child in &mounted.children {
        collect_snapshot(child, Some(mounted.id), snapshot);
    }
}
