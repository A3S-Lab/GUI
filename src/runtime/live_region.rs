use std::collections::{BTreeMap, BTreeSet};

use super::GuiRuntime;
use crate::accessibility::{
    accessibility_live_setting, live_region_announces_on_initial_render,
    live_region_implicit_atomic, AccessibilityAnnouncement, AccessibilityAnnouncementPriority,
    AccessibilityLiveSetting,
};
use crate::host::{HostNodeId, NativeHost};
use crate::native::{effective_input_type, NativeProps, ValueSensitivity};
use crate::renderer::MountedNodeSnapshot;
use crate::style::PortableStyle;

impl<H: NativeHost> GuiRuntime<H> {
    pub(super) fn live_region_announcements(
        &mut self,
        previous_snapshot: &[MountedNodeSnapshot],
        current_snapshot: &[MountedNodeSnapshot],
    ) -> Vec<AccessibilityAnnouncement> {
        let previous = SnapshotTree::new(previous_snapshot);
        let current = SnapshotTree::new(current_snapshot);
        let initial_render = previous_snapshot.is_empty();
        let mut active_regions = BTreeSet::new();
        let mut announcements = Vec::new();

        for mounted in current_snapshot {
            let Some(LiveBoundary::Active(policy)) = live_boundary(mounted) else {
                continue;
            };
            active_regions.insert(mounted.node);
            if !current.is_effectively_visible(mounted.node) {
                self.pending_live_region_updates.remove(&mounted.node);
                continue;
            }

            let current_content = current.live_region_content(mounted.node);
            let previous_mounted = previous.node(mounted.node);
            let was_active_and_visible = previous_mounted.is_some_and(|previous_mounted| {
                matches!(
                    live_boundary(previous_mounted),
                    Some(LiveBoundary::Active(_))
                ) && previous.is_effectively_visible(previous_mounted.node)
            });
            let previous_content = was_active_and_visible
                .then(|| previous.live_region_content(mounted.node))
                .unwrap_or_default();
            let should_consider_update = !initial_render || policy.announce_on_initial;
            let changed_message = if should_consider_update {
                if was_active_and_visible {
                    changed_live_region_message(&previous_content, &current_content, policy)
                } else if policy.relevant.additions {
                    current_content.message()
                } else {
                    None
                }
            } else {
                None
            };

            if current.is_effectively_busy(mounted.node) {
                if should_consider_update
                    && previous_content != current_content
                    && !self.pending_live_region_updates.contains_key(&mounted.node)
                {
                    self.pending_live_region_updates.insert(
                        mounted.node,
                        PendingLiveRegionUpdate {
                            baseline: previous_content,
                        },
                    );
                }
                continue;
            }

            let pending = self.pending_live_region_updates.remove(&mounted.node);
            let was_busy = previous_mounted.is_some_and(|previous_mounted| {
                previous.is_effectively_busy(previous_mounted.node)
            });
            let message = match pending {
                Some(pending) => {
                    coalesced_busy_message(&pending.baseline, &current_content, policy)
                }
                None if was_busy && changed_message.is_some() => {
                    coalesced_busy_message(&previous_content, &current_content, policy)
                }
                None => changed_message,
            };

            if let Some(message) = message.filter(|message| !message.is_empty()) {
                announcements.push(AccessibilityAnnouncement::new(
                    mounted.node,
                    message,
                    policy.priority,
                ));
            }
        }

        self.pending_live_region_updates
            .retain(|node, _| active_regions.contains(node));
        announcements
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct PendingLiveRegionUpdate {
    baseline: LiveRegionContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiveBoundary {
    Active(LiveRegionPolicy),
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LiveRegionPolicy {
    priority: AccessibilityAnnouncementPriority,
    atomic: bool,
    relevant: LiveRelevant,
    announce_on_initial: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LiveRelevant {
    additions: bool,
    removals: bool,
    text: bool,
}

impl Default for LiveRelevant {
    fn default() -> Self {
        Self {
            additions: true,
            removals: false,
            text: true,
        }
    }
}

impl LiveRelevant {
    fn parse(value: Option<&str>) -> Self {
        let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
            return Self::default();
        };
        let tokens = value
            .split_ascii_whitespace()
            .map(str::to_ascii_lowercase)
            .collect::<BTreeSet<_>>();
        if tokens.contains("all") {
            return Self {
                additions: true,
                removals: true,
                text: true,
            };
        }
        let parsed = Self {
            additions: tokens.contains("additions"),
            removals: tokens.contains("removals"),
            text: tokens.contains("text"),
        };
        (parsed.additions || parsed.removals || parsed.text)
            .then_some(parsed)
            .unwrap_or_default()
    }
}

fn live_boundary(mounted: &MountedNodeSnapshot) -> Option<LiveBoundary> {
    let setting = accessibility_live_setting(mounted.role, &mounted.props);
    let priority = match setting {
        AccessibilityLiveSetting::Polite => AccessibilityAnnouncementPriority::Polite,
        AccessibilityLiveSetting::Assertive => AccessibilityAnnouncementPriority::Assertive,
        AccessibilityLiveSetting::Off => return Some(LiveBoundary::Off),
        AccessibilityLiveSetting::Inherit => return None,
    };

    Some(LiveBoundary::Active(LiveRegionPolicy {
        priority,
        atomic: mounted
            .props
            .accessibility_state
            .atomic
            .unwrap_or_else(|| live_region_implicit_atomic(mounted.role, &mounted.props)),
        relevant: LiveRelevant::parse(mounted.props.accessibility_state.relevant.as_deref()),
        announce_on_initial: live_region_announces_on_initial_render(&mounted.props),
    }))
}

struct SnapshotTree<'a> {
    nodes: BTreeMap<HostNodeId, &'a MountedNodeSnapshot>,
    children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
}

impl<'a> SnapshotTree<'a> {
    fn new(snapshot: &'a [MountedNodeSnapshot]) -> Self {
        let mut nodes = BTreeMap::new();
        let mut children = BTreeMap::<HostNodeId, Vec<HostNodeId>>::new();
        for mounted in snapshot {
            nodes.insert(mounted.node, mounted);
            if let Some(parent) = mounted.parent {
                children.entry(parent).or_default().push(mounted.node);
            }
        }
        Self { nodes, children }
    }

    fn node(&self, node: HostNodeId) -> Option<&MountedNodeSnapshot> {
        self.nodes.get(&node).copied()
    }

    fn is_effectively_visible(&self, node: HostNodeId) -> bool {
        let mut current = Some(node);
        let mut visited = BTreeSet::new();
        while let Some(id) = current {
            if !visited.insert(id) {
                return false;
            }
            let Some(mounted) = self.node(id) else {
                return false;
            };
            if !is_accessibility_visible(&mounted.props) {
                return false;
            }
            current = mounted.parent;
        }
        true
    }

    fn is_effectively_busy(&self, node: HostNodeId) -> bool {
        let mut current = Some(node);
        let mut visited = BTreeSet::new();
        while let Some(id) = current {
            if !visited.insert(id) {
                return true;
            }
            let Some(mounted) = self.node(id) else {
                return true;
            };
            if mounted.props.accessibility_state.busy == Some(true) {
                return true;
            }
            current = mounted.parent;
        }
        false
    }

    fn live_region_content(&self, root: HostNodeId) -> LiveRegionContent {
        let mut content = LiveRegionContent::default();
        self.collect_live_region_content(root, root, &mut content);
        content
    }

    fn collect_live_region_content(
        &self,
        root: HostNodeId,
        node: HostNodeId,
        content: &mut LiveRegionContent,
    ) {
        let Some(mounted) = self.node(node) else {
            return;
        };
        if !is_accessibility_visible(&mounted.props)
            || (node != root && live_boundary(mounted).is_some())
        {
            return;
        }
        if let Some(text) = accessible_node_text(mounted, node == root) {
            content.fragments.push(LiveFragment { node, text });
        }
        if let Some(children) = self.children.get(&node) {
            for child in children {
                self.collect_live_region_content(root, *child, content);
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct LiveRegionContent {
    fragments: Vec<LiveFragment>,
}

impl LiveRegionContent {
    fn message(&self) -> Option<String> {
        join_fragments(self.fragments.iter().map(|fragment| fragment.text.as_str()))
    }

    fn by_node(&self) -> BTreeMap<HostNodeId, &str> {
        self.fragments
            .iter()
            .map(|fragment| (fragment.node, fragment.text.as_str()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LiveFragment {
    node: HostNodeId,
    text: String,
}

fn changed_live_region_message(
    previous: &LiveRegionContent,
    current: &LiveRegionContent,
    policy: LiveRegionPolicy,
) -> Option<String> {
    let previous_by_node = previous.by_node();
    let current_by_node = current.by_node();
    let has_addition = policy.relevant.additions
        && current_by_node
            .keys()
            .any(|node| !previous_by_node.contains_key(node));
    let has_removal = policy.relevant.removals
        && previous_by_node
            .keys()
            .any(|node| !current_by_node.contains_key(node));
    let has_text_change = policy.relevant.text
        && current_by_node.iter().any(|(node, text)| {
            previous_by_node
                .get(node)
                .is_some_and(|previous_text| previous_text != text)
        });

    if policy.atomic {
        if !(has_addition || has_removal || has_text_change) {
            return None;
        }
        return current.message().or_else(|| {
            policy.relevant.removals.then(|| {
                join_fragments(previous.fragments.iter().filter_map(|fragment| {
                    (!current_by_node.contains_key(&fragment.node))
                        .then_some(fragment.text.as_str())
                }))
            })?
        });
    }

    join_fragments(
        current
            .fragments
            .iter()
            .filter_map(|fragment| match previous_by_node.get(&fragment.node) {
                None if policy.relevant.additions => Some(fragment.text.as_str()),
                Some(previous_text) if policy.relevant.text && *previous_text != fragment.text => {
                    Some(fragment.text.as_str())
                }
                _ => None,
            })
            .chain(previous.fragments.iter().filter_map(|fragment| {
                (policy.relevant.removals && !current_by_node.contains_key(&fragment.node))
                    .then_some(fragment.text.as_str())
            })),
    )
}

fn coalesced_busy_message(
    baseline: &LiveRegionContent,
    current: &LiveRegionContent,
    policy: LiveRegionPolicy,
) -> Option<String> {
    changed_live_region_message(baseline, current, policy)
        .and_then(|fallback| current.message().or(Some(fallback)))
}

fn accessible_node_text(mounted: &MountedNodeSnapshot, is_region_root: bool) -> Option<String> {
    let props = &mounted.props;
    let role = props
        .explicit_role
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_lowercase);
    let mut parts = Vec::new();
    if !(is_region_root && matches!(role.as_deref(), Some("region" | "log"))) {
        push_accessible_part(&mut parts, props.label.as_deref());
    }
    push_accessible_part(
        &mut parts,
        props.accessibility_description.description.as_deref(),
    );
    let sensitivity = ValueSensitivity::from_input_type(effective_input_type(props));
    if sensitivity.is_public() {
        push_accessible_part(
            &mut parts,
            props
                .accessibility_description
                .value_text
                .as_deref()
                .or(props.value.as_deref()),
        );
    }
    join_fragments(parts.iter().map(String::as_str))
}

fn push_accessible_part(parts: &mut Vec<String>, value: Option<&str>) {
    let Some(value) = value.and_then(normalize_accessible_text) else {
        return;
    };
    if !parts.iter().any(|part| part == &value) {
        parts.push(value);
    }
}

fn normalize_accessible_text(value: &str) -> Option<String> {
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    (!value.is_empty()).then_some(value)
}

fn join_fragments<'a>(fragments: impl Iterator<Item = &'a str>) -> Option<String> {
    let mut normalized = Vec::new();
    for fragment in fragments {
        let Some(fragment) = normalize_accessible_text(fragment) else {
            continue;
        };
        if normalized.last() != Some(&fragment) {
            normalized.push(fragment);
        }
    }
    (!normalized.is_empty()).then(|| normalized.join(" "))
}

fn is_accessibility_visible(props: &NativeProps) -> bool {
    let style = PortableStyle::from_web(&props.web);
    !props.hidden
        && !props.inert
        && props.accessibility_state.hidden != Some(true)
        && props.html_dialog.open.unwrap_or(true)
        && style.renders_native_widget()
        && !style.makes_native_widget_inert()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relevant_tokens_default_and_support_all() {
        assert_eq!(LiveRelevant::parse(None), LiveRelevant::default());
        assert_eq!(
            LiveRelevant::parse(Some("all")),
            LiveRelevant {
                additions: true,
                removals: true,
                text: true,
            }
        );
        assert_eq!(
            LiveRelevant::parse(Some("removals text")),
            LiveRelevant {
                additions: false,
                removals: true,
                text: true,
            }
        );
        assert_eq!(
            LiveRelevant::parse(Some("unknown")),
            LiveRelevant::default()
        );
    }
}
