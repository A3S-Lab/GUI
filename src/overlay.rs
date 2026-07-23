use std::collections::{BTreeMap, BTreeSet};

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::native::NativeProps;
use crate::renderer::MountedNodeSnapshot;
use crate::style::PortableStyle;

pub(crate) const OVERLAY_CAPTURE_METADATA_KEY: &str = "data-a3s-overlay-capture";
const OVERLAY_MARKER: &str = "data-overlay";
const OVERLAY_MODAL_MARKER: &str = "data-overlay-modal";
const OVERLAY_UNDERLAY_MARKER: &str = "data-overlay-underlay";
const OVERLAY_DISMISSABLE_MARKER: &str = "data-overlay-dismissable";
const OVERLAY_KEYBOARD_DISMISS_DISABLED_MARKER: &str = "data-overlay-keyboard-dismiss-disabled";
const OVERLAY_CLOSE_ON_BLUR_MARKER: &str = "data-overlay-close-on-blur";
const OVERLAY_AUTO_FOCUS_MARKER: &str = "data-auto-focus";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeOverlay {
    pub node: HostNodeId,
    pub modal: bool,
    pub underlay: bool,
    pub dismissable: bool,
    pub keyboard_dismiss_disabled: bool,
    pub close_on_blur: bool,
    pub auto_focus: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OverlayEventDisposition {
    Continue,
    Suppress,
    Dismiss(HostNodeId),
    DismissAfterEvent(HostNodeId),
}

/// Mounted overlay ordering and interaction policy for the native tree.
///
/// Existing overlays retain activation order and newly opened overlays are
/// appended, so only the topmost visible overlay is eligible for Escape and
/// outside-interaction dismissal. This mirrors React Aria's visible-overlay
/// stack while remaining independent of a particular native window toolkit.
#[derive(Debug, Clone, Default)]
pub struct MountedOverlayRegistry {
    overlays: Vec<NativeOverlay>,
    parents: BTreeMap<HostNodeId, Option<HostNodeId>>,
    outside_press_overlay: Option<HostNodeId>,
    opened_auto_focus_overlay: Option<HostNodeId>,
}

impl MountedOverlayRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync(&mut self, snapshot: &[MountedNodeSnapshot]) {
        let previous = self
            .overlays
            .iter()
            .map(|overlay| overlay.node)
            .collect::<BTreeSet<_>>();
        self.parents = snapshot
            .iter()
            .map(|record| (record.node, record.parent))
            .collect();
        let mut active = snapshot
            .iter()
            .filter(|record| is_active_overlay(&record.props))
            .map(|record| {
                (
                    record.node,
                    NativeOverlay {
                        node: record.node,
                        modal: bool_marker(&record.props, OVERLAY_MODAL_MARKER),
                        underlay: bool_marker(&record.props, OVERLAY_UNDERLAY_MARKER),
                        dismissable: bool_marker(&record.props, OVERLAY_DISMISSABLE_MARKER),
                        keyboard_dismiss_disabled: bool_marker(
                            &record.props,
                            OVERLAY_KEYBOARD_DISMISS_DISABLED_MARKER,
                        ),
                        close_on_blur: bool_marker(&record.props, OVERLAY_CLOSE_ON_BLUR_MARKER),
                        auto_focus: bool_marker(&record.props, OVERLAY_AUTO_FOCUS_MARKER),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut overlays = self
            .overlays
            .iter()
            .filter_map(|overlay| active.remove(&overlay.node))
            .collect::<Vec<_>>();
        overlays.extend(
            snapshot
                .iter()
                .filter_map(|record| active.remove(&record.node)),
        );
        self.overlays = overlays;
        self.opened_auto_focus_overlay = self
            .overlays
            .iter()
            .rev()
            .find(|overlay| overlay.auto_focus && !previous.contains(&overlay.node))
            .map(|overlay| overlay.node);
        let outside_capture_is_current = self.outside_press_overlay.is_some_and(|node| {
            self.topmost()
                .is_some_and(|overlay| overlay.node == node && overlay.dismissable)
        });
        if !outside_capture_is_current {
            self.outside_press_overlay = None;
        }
    }

    pub fn overlays(&self) -> impl DoubleEndedIterator<Item = &NativeOverlay> {
        self.overlays.iter()
    }

    pub fn topmost(&self) -> Option<&NativeOverlay> {
        self.overlays.last()
    }

    pub fn active_modal(&self) -> Option<&NativeOverlay> {
        self.overlays.iter().rev().find(|overlay| overlay.modal)
    }

    pub fn contains(&self, overlay: HostNodeId, node: HostNodeId) -> bool {
        if overlay == node {
            return true;
        }
        let mut current = self.parents.get(&node).copied().flatten();
        let mut visited = BTreeSet::new();
        while let Some(candidate) = current {
            if candidate == overlay {
                return true;
            }
            if !visited.insert(candidate) {
                break;
            }
            current = self.parents.get(&candidate).copied().flatten();
        }
        false
    }

    /// Returns whether focus may enter a later overlay rendered through a
    /// separate portal branch without being rejected by an earlier contained
    /// focus scope.
    pub fn allows_focus_transition(&self, current: HostNodeId, requested: HostNodeId) -> bool {
        let current_overlay = self
            .overlays
            .iter()
            .rposition(|overlay| self.contains(overlay.node, current));
        let requested_overlay = self
            .overlays
            .iter()
            .rposition(|overlay| self.contains(overlay.node, requested));
        matches!(
            (current_overlay, requested_overlay),
            (Some(current), Some(requested)) if requested > current
        )
    }

    pub(crate) fn take_opened_auto_focus_overlay(&mut self) -> Option<HostNodeId> {
        self.opened_auto_focus_overlay.take()
    }

    pub(crate) fn projected_props(
        &self,
        snapshot: &[MountedNodeSnapshot],
    ) -> BTreeMap<HostNodeId, NativeProps> {
        if self.overlays.is_empty() {
            return BTreeMap::new();
        }
        let modal_layers = self
            .active_modal_index()
            .map(|index| &self.overlays[index..]);
        snapshot
            .iter()
            .filter_map(|record| {
                let mut props = record.props.clone();
                let before = props.clone();
                props
                    .metadata
                    .insert(OVERLAY_CAPTURE_METADATA_KEY.to_string(), "true".to_string());
                if modal_layers.is_some_and(|layers| {
                    !layers
                        .iter()
                        .any(|overlay| self.is_related(record.node, overlay.node))
                }) {
                    props.inert = true;
                }
                (props != before).then_some((record.node, props))
            })
            .collect()
    }

    pub(crate) fn handle_event(&mut self, event: &NativeEvent) -> OverlayEventDisposition {
        if !self.parents.contains_key(&event.node) {
            return OverlayEventDisposition::Continue;
        }
        let Some(topmost) = self.topmost().copied() else {
            self.outside_press_overlay = None;
            return OverlayEventDisposition::Continue;
        };

        if event.kind == NativeEventKind::KeyDown && event.value.as_deref() == Some("Escape") {
            return if topmost.keyboard_dismiss_disabled {
                OverlayEventDisposition::Continue
            } else {
                OverlayEventDisposition::Dismiss(topmost.node)
            };
        }

        if event.kind == NativeEventKind::Blur
            && topmost.close_on_blur
            && self.is_overlay_content(topmost, event.node)
            && event
                .context
                .related_target
                .is_some_and(|target| !self.is_overlay_content(topmost, target))
        {
            return OverlayEventDisposition::DismissAfterEvent(topmost.node);
        }

        let outside_topmost = !self.is_overlay_content(topmost, event.node);
        let outside_modal =
            self.active_modal().is_some() && !self.is_in_modal_foreground(event.node);
        if event.kind == NativeEventKind::PressStart {
            self.outside_press_overlay = None;
        }
        match event.kind {
            NativeEventKind::PressStart if outside_topmost && topmost.dismissable => {
                self.outside_press_overlay = Some(topmost.node);
                OverlayEventDisposition::Suppress
            }
            NativeEventKind::PressStart if outside_modal => OverlayEventDisposition::Suppress,
            NativeEventKind::PressUp => {
                let started_overlay = self.outside_press_overlay.take();
                if outside_topmost && topmost.dismissable && started_overlay == Some(topmost.node) {
                    OverlayEventDisposition::Dismiss(topmost.node)
                } else if outside_modal
                    || (outside_topmost && topmost.dismissable && started_overlay.is_some())
                {
                    OverlayEventDisposition::Suppress
                } else {
                    OverlayEventDisposition::Continue
                }
            }
            NativeEventKind::PressCancel => {
                let captured = self.outside_press_overlay.take().is_some();
                if captured || outside_modal {
                    OverlayEventDisposition::Suppress
                } else {
                    OverlayEventDisposition::Continue
                }
            }
            kind if is_pointer_interaction(kind)
                && (outside_modal || (outside_topmost && topmost.dismissable)) =>
            {
                OverlayEventDisposition::Suppress
            }
            kind if is_user_interaction(kind) && outside_modal => OverlayEventDisposition::Suppress,
            _ => OverlayEventDisposition::Continue,
        }
    }

    fn is_related(&self, node: HostNodeId, overlay: HostNodeId) -> bool {
        self.contains(overlay, node) || self.contains(node, overlay)
    }

    fn is_overlay_content(&self, overlay: NativeOverlay, node: HostNodeId) -> bool {
        self.contains(overlay.node, node) && !(overlay.underlay && overlay.node == node)
    }

    fn active_modal_index(&self) -> Option<usize> {
        self.overlays.iter().rposition(|overlay| overlay.modal)
    }

    fn is_in_modal_foreground(&self, node: HostNodeId) -> bool {
        self.active_modal_index().is_some_and(|index| {
            self.overlays[index..]
                .iter()
                .any(|overlay| self.is_overlay_content(*overlay, node))
        })
    }
}

fn is_active_overlay(props: &NativeProps) -> bool {
    let style = PortableStyle::from_web(&props.web);
    bool_marker(props, OVERLAY_MARKER)
        && !props.hidden
        && !props.inert
        && props.html_dialog.open.unwrap_or(true)
        && style.renders_native_widget()
        && !style.makes_native_widget_inert()
}

fn bool_marker(props: &NativeProps, name: &str) -> bool {
    props
        .metadata
        .get(name)
        .or_else(|| props.web.attributes.get(name))
        .is_some_and(|value| {
            value.is_empty()
                || value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case(name)
        })
}

fn is_pointer_interaction(kind: NativeEventKind) -> bool {
    matches!(
        kind,
        NativeEventKind::PressStart
            | NativeEventKind::PressEnd
            | NativeEventKind::PressUp
            | NativeEventKind::PressCancel
            | NativeEventKind::Press
            | NativeEventKind::LongPressStart
            | NativeEventKind::LongPressEnd
            | NativeEventKind::LongPress
            | NativeEventKind::MoveStart
            | NativeEventKind::Move
            | NativeEventKind::MoveEnd
            | NativeEventKind::Action
    )
}

fn is_user_interaction(kind: NativeEventKind) -> bool {
    !matches!(kind, NativeEventKind::Blur | NativeEventKind::Close)
}
