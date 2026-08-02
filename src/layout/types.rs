use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};
use crate::native::NativeRole;

use super::text::LayoutText;

pub const LAYOUT_SCHEMA_VERSION: u16 = 2;
pub const LAYOUT_QUANTIZATION: f64 = 1.0 / 64.0;

pub(super) fn quantize(value: f64) -> f64 {
    let value = (value / LAYOUT_QUANTIZATION).round() * LAYOUT_QUANTIZATION;
    if value == -0.0 {
        0.0
    } else {
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LayoutElementId(String);

impl LayoutElementId {
    pub(crate) fn root(key: &str) -> Self {
        Self(path_segment(key))
    }

    pub(crate) fn child(&self, key: &str) -> Self {
        Self(format!("{}/{}", self.0, path_segment(key)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for LayoutElementId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn path_segment(key: &str) -> String {
    format!("{}:{key}", key.len())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutEdgeWidths {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl LayoutEdgeWidths {
    pub const ZERO: Self = Self::all(0.0);

    pub const fn all(value: f64) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn is_zero(self) -> bool {
        self.top == 0.0 && self.right == 0.0 && self.bottom == 0.0 && self.left == 0.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutCornerRadii {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl LayoutCornerRadii {
    pub const ZERO: Self = Self::all(0.0);

    pub const fn all(value: f64) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    pub fn is_zero(self) -> bool {
        self.top_left == 0.0
            && self.top_right == 0.0
            && self.bottom_right == 0.0
            && self.bottom_left == 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl LayoutColor {
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgba(0, 0, 0, 255);
    pub const WHITE: Self = Self::rgba(255, 255, 255, 255);

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutEdgeColors {
    pub top: Option<LayoutColor>,
    pub right: Option<LayoutColor>,
    pub bottom: Option<LayoutColor>,
    pub left: Option<LayoutColor>,
}

impl LayoutEdgeColors {
    pub const fn all(color: Option<LayoutColor>) -> Self {
        Self {
            top: color,
            right: color,
            bottom: color,
            left: color,
        }
    }

    pub fn uniform(self) -> Option<LayoutColor> {
        (self.top == self.right && self.top == self.bottom && self.top == self.left)
            .then_some(self.top)
            .flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPaint {
    pub background: Option<LayoutColor>,
    pub border_widths: LayoutEdgeWidths,
    pub border_colors: LayoutEdgeColors,
    pub corner_radii: LayoutCornerRadii,
    pub opacity: f64,
}

impl Default for LayoutPaint {
    fn default() -> Self {
        Self {
            background: None,
            border_widths: LayoutEdgeWidths::ZERO,
            border_colors: LayoutEdgeColors::default(),
            corner_radii: LayoutCornerRadii::ZERO,
            opacity: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutNodeRecord {
    pub id: LayoutElementId,
    pub parent_id: Option<LayoutElementId>,
    pub role: NativeRole,
    pub border_box: Rect,
    pub content_box: Rect,
    pub clip: Option<Rect>,
    pub z_index: i32,
    pub paint_order: u32,
    pub hit_testable: bool,
    pub paint: LayoutPaint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<LayoutText>,
}

impl LayoutNodeRecord {
    pub fn visual_bounds(&self) -> Rect {
        self.text
            .as_ref()
            .and_then(LayoutText::ink_bounds)
            .map_or(self.border_box, |ink| rect_union(self.border_box, ink))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutHitRegion {
    pub id: LayoutElementId,
    pub role: NativeRole,
    pub bounds: Rect,
    pub clip: Option<Rect>,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LayoutDiagnosticSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LayoutDiagnosticCode {
    DeferredRole,
    DeferredStyleField,
    UnsupportedM3StyleField,
    UnsupportedLayoutMode,
    UnresolvedLength,
    UnsupportedColor,
    UnsupportedBorderStyle,
    UnsupportedRoundedBorder,
    UnparsedStyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiagnostic {
    pub severity: LayoutDiagnosticSeverity,
    pub code: LayoutDiagnosticCode,
    pub element: LayoutElementId,
    pub field: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSnapshot {
    pub schema_version: u16,
    pub logical_size: Size,
    pub nodes: Vec<LayoutNodeRecord>,
    pub hit_regions: Vec<LayoutHitRegion>,
    pub diagnostics: Vec<LayoutDiagnostic>,
}

impl LayoutSnapshot {
    pub fn node(&self, id: &LayoutElementId) -> Option<&LayoutNodeRecord> {
        self.nodes.iter().find(|node| node.id == *id)
    }

    pub fn node_by_path(&self, path: &str) -> Option<&LayoutNodeRecord> {
        self.nodes.iter().find(|node| node.id.as_str() == path)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == LayoutDiagnosticSeverity::Error)
    }

    pub fn require_supported(&self) -> GuiResult<()> {
        let Some(diagnostic) = self
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.severity == LayoutDiagnosticSeverity::Error)
        else {
            return Ok(());
        };
        Err(GuiError::invalid_tree(format!(
            "layout field support is incomplete at {}: {}",
            diagnostic.element, diagnostic.message
        )))
    }

    pub fn fingerprint(&self) -> GuiResult<u64> {
        let bytes = serde_json::to_vec(self).map_err(|error| {
            GuiError::invalid_tree(format!("failed to fingerprint layout snapshot: {error}"))
        })?;
        let mut fingerprint = 0xcbf29ce484222325_u64;
        for byte in bytes {
            fingerprint ^= u64::from(byte);
            fingerprint = fingerprint.wrapping_mul(0x100000001b3);
        }
        Ok(fingerprint)
    }

    pub fn diff(&self, next: &Self) -> LayoutDiff {
        if self.schema_version != next.schema_version || self.logical_size != next.logical_size {
            return LayoutDiff {
                full_rebuild: true,
                dirty_bounds: Some(rect_union(
                    Rect::new(0.0, 0.0, self.logical_size.width, self.logical_size.height),
                    Rect::new(0.0, 0.0, next.logical_size.width, next.logical_size.height),
                )),
                changes: vec![LayoutChange {
                    kind: LayoutChangeKind::SurfaceChanged,
                    id: None,
                    previous_bounds: None,
                    next_bounds: None,
                }],
            };
        }

        let previous = self
            .nodes
            .iter()
            .map(|node| (&node.id, node))
            .collect::<BTreeMap<_, _>>();
        let following = next
            .nodes
            .iter()
            .map(|node| (&node.id, node))
            .collect::<BTreeMap<_, _>>();
        let previous_hits = self
            .hit_regions
            .iter()
            .map(|region| (&region.id, region))
            .collect::<BTreeMap<_, _>>();
        let following_hits = next
            .hit_regions
            .iter()
            .map(|region| (&region.id, region))
            .collect::<BTreeMap<_, _>>();
        let mut changes = Vec::new();
        let mut dirty_bounds = None;

        for (id, node) in &previous {
            match following.get(id) {
                None => push_change(
                    &mut changes,
                    &mut dirty_bounds,
                    LayoutChangeKind::Removed,
                    (*id).clone(),
                    Some(node.visual_bounds()),
                    None,
                ),
                Some(next_node)
                    if *node != *next_node || previous_hits.get(id) != following_hits.get(id) =>
                {
                    push_change(
                        &mut changes,
                        &mut dirty_bounds,
                        LayoutChangeKind::Changed,
                        (*id).clone(),
                        Some(node.visual_bounds()),
                        Some(next_node.visual_bounds()),
                    )
                }
                Some(_) => {}
            }
        }
        for (id, node) in following {
            if !previous.contains_key(id) {
                push_change(
                    &mut changes,
                    &mut dirty_bounds,
                    LayoutChangeKind::Added,
                    id.clone(),
                    None,
                    Some(node.visual_bounds()),
                );
            }
        }

        LayoutDiff {
            full_rebuild: false,
            dirty_bounds,
            changes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LayoutChangeKind {
    Added,
    Removed,
    Changed,
    SurfaceChanged,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutChange {
    pub kind: LayoutChangeKind,
    pub id: Option<LayoutElementId>,
    pub previous_bounds: Option<Rect>,
    pub next_bounds: Option<Rect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiff {
    pub full_rebuild: bool,
    pub dirty_bounds: Option<Rect>,
    pub changes: Vec<LayoutChange>,
}

impl LayoutDiff {
    pub fn is_empty(&self) -> bool {
        !self.full_rebuild && self.changes.is_empty()
    }
}

fn push_change(
    changes: &mut Vec<LayoutChange>,
    dirty_bounds: &mut Option<Rect>,
    kind: LayoutChangeKind,
    id: LayoutElementId,
    previous_bounds: Option<Rect>,
    next_bounds: Option<Rect>,
) {
    for bounds in [previous_bounds, next_bounds].into_iter().flatten() {
        *dirty_bounds = Some(match *dirty_bounds {
            Some(accumulated) => rect_union(accumulated, bounds),
            None => bounds,
        });
    }
    changes.push(LayoutChange {
        kind,
        id: Some(id),
        previous_bounds,
        next_bounds,
    });
}

fn rect_union(left: Rect, right: Rect) -> Rect {
    let x = left.x.min(right.x);
    let y = left.y.min(right.y);
    let right_edge = (left.x + left.width).max(right.x + right.width);
    let bottom_edge = (left.y + left.height).max(right.y + right.height);
    Rect::new(x, y, right_edge - x, bottom_edge - y)
}
