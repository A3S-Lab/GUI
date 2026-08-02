use std::sync::Arc;

use crate::drawing::{LayoutSceneOptions, Scene};
use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};
use crate::layout::{layout_native_tree, LayoutOptions, LayoutSnapshot};
use crate::native::NativeElement;
use crate::platform_host::{
    PlatformAccessibilitySnapshot, PlatformHostRevision, PlatformPresentationRequest,
    PlatformWindowId,
};

use super::accessibility::accessibility_snapshot;
use super::interaction_tree::SelfDrawnInteractionTree;

/// Immutable Graphics work prepared for an already-attached platform surface.
///
/// Raw window, display, GPU, and toolkit handles remain inside the concrete
/// presenter. The shared runtime transfers only owned scene data and bounded
/// presentation metadata across this edge.
#[derive(Clone)]
pub struct PlatformRenderFrame {
    pub revision: PlatformHostRevision,
    pub window: PlatformWindowId,
    pub logical_size: Size,
    pub scale_factor: f64,
    pub layout_fingerprint: u64,
    pub scene_fingerprint: u64,
    pub damage: Vec<Rect>,
    pub scene: Arc<Scene>,
}

impl std::fmt::Debug for PlatformRenderFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlatformRenderFrame")
            .field("revision", &self.revision)
            .field("window", &self.window)
            .field("logical_size", &self.logical_size)
            .field("scale_factor", &self.scale_factor)
            .field("layout_fingerprint", &self.layout_fingerprint)
            .field("scene_fingerprint", &self.scene_fingerprint)
            .field("damage", &self.damage)
            .field("scene_commands", &self.scene.commands.len())
            .finish()
    }
}

impl PlatformRenderFrame {
    pub fn validate(&self) -> GuiResult<()> {
        self.revision.validate()?;
        self.window.validate()?;
        self.scene.validate()?;
        let expected_fingerprint = self.scene.fingerprint()?;
        if self.scene_fingerprint != expected_fingerprint {
            return Err(GuiError::host(
                "platform render frame scene fingerprint does not match its scene",
            ));
        }
        let scene_width = f64::from(self.scene.logical_size.width);
        let scene_height = f64::from(self.scene.logical_size.height);
        if scene_width != self.logical_size.width || scene_height != self.logical_size.height {
            return Err(GuiError::host(
                "platform render frame logical size does not match its scene",
            ));
        }
        if f64::from(self.scene.scale_factor) != self.scale_factor {
            return Err(GuiError::host(
                "platform render frame scale factor does not match its scene",
            ));
        }
        PlatformPresentationRequest {
            window: self.window,
            logical_size: self.logical_size,
            scale_factor: self.scale_factor,
            scene_fingerprint: self.scene_fingerprint,
            damage: self.damage.clone(),
        }
        .validate()
    }
}

/// Last frame atomically committed by [`super::SelfDrawnWindowRuntime`].
#[derive(Clone)]
pub struct SelfDrawnFrameSnapshot {
    revision: PlatformHostRevision,
    window: PlatformWindowId,
    logical_size: Size,
    scale_factor: f64,
    layout_fingerprint: u64,
    scene_fingerprint: u64,
    native_root: Arc<NativeElement>,
    layout: Arc<LayoutSnapshot>,
    scene: Arc<Scene>,
    accessibility: PlatformAccessibilitySnapshot,
    interaction: Arc<SelfDrawnInteractionTree>,
    damage: Vec<Rect>,
}

impl std::fmt::Debug for SelfDrawnFrameSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SelfDrawnFrameSnapshot")
            .field("revision", &self.revision)
            .field("window", &self.window)
            .field("logical_size", &self.logical_size)
            .field("scale_factor", &self.scale_factor)
            .field("layout_fingerprint", &self.layout_fingerprint)
            .field("scene_fingerprint", &self.scene_fingerprint)
            .field("layout_nodes", &self.layout.nodes.len())
            .field("hit_regions", &self.layout.hit_regions.len())
            .field("interaction_nodes", &self.interaction.len())
            .field("scene_commands", &self.scene.commands.len())
            .field("damage", &self.damage)
            .finish()
    }
}

impl SelfDrawnFrameSnapshot {
    pub fn revision(&self) -> PlatformHostRevision {
        self.revision
    }

    pub fn window(&self) -> PlatformWindowId {
        self.window
    }

    pub fn logical_size(&self) -> Size {
        self.logical_size
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn layout_fingerprint(&self) -> u64 {
        self.layout_fingerprint
    }

    pub fn scene_fingerprint(&self) -> u64 {
        self.scene_fingerprint
    }

    pub fn native_root(&self) -> &NativeElement {
        &self.native_root
    }

    pub fn layout(&self) -> &LayoutSnapshot {
        &self.layout
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn accessibility(&self) -> &PlatformAccessibilitySnapshot {
        &self.accessibility
    }

    pub fn damage(&self) -> &[Rect] {
        &self.damage
    }

    pub(super) fn interaction_tree(&self) -> &Arc<SelfDrawnInteractionTree> {
        &self.interaction
    }

    pub(super) fn render_frame(&self) -> PlatformRenderFrame {
        PlatformRenderFrame {
            revision: self.revision,
            window: self.window,
            logical_size: self.logical_size,
            scale_factor: self.scale_factor,
            layout_fingerprint: self.layout_fingerprint,
            scene_fingerprint: self.scene_fingerprint,
            damage: self.damage.clone(),
            scene: self.scene.clone(),
        }
    }

    pub(super) fn replay(&self, revision: PlatformHostRevision) -> Self {
        let mut replay = self.clone();
        replay.revision = revision;
        replay.damage = vec![Rect::new(
            0.0,
            0.0,
            self.logical_size.width,
            self.logical_size.height,
        )];
        replay
    }
}

pub(super) fn build_snapshot(
    revision: PlatformHostRevision,
    window: PlatformWindowId,
    native_root: NativeElement,
    logical_size: Size,
    scale_factor: f64,
    previous: Option<&SelfDrawnFrameSnapshot>,
) -> GuiResult<SelfDrawnFrameSnapshot> {
    revision.validate()?;
    window.validate()?;
    let narrowed_scale = scale_factor as f32;
    if !scale_factor.is_finite()
        || scale_factor <= 0.0
        || !narrowed_scale.is_finite()
        || narrowed_scale <= 0.0
    {
        return Err(GuiError::host(
            "self-drawn scale factor must fit in a positive finite 32-bit float",
        ));
    }
    let scale_factor = f64::from(narrowed_scale);
    let layout = layout_native_tree(&native_root, LayoutOptions::boxes_only(logical_size))?;
    layout.require_supported()?;
    let interaction = SelfDrawnInteractionTree::build(&native_root, &layout)?;
    let layout_fingerprint = layout.fingerprint()?;
    let scene = crate::drawing::scene_from_layout(
        &layout,
        LayoutSceneOptions {
            scale_factor: narrowed_scale,
            ..LayoutSceneOptions::default()
        },
    )?;
    let scene_fingerprint = scene.fingerprint()?;
    let damage = scene_damage(previous.map(SelfDrawnFrameSnapshot::scene), &scene)?;
    let accessibility = accessibility_snapshot(window, &native_root, &layout)?;
    let snapshot = SelfDrawnFrameSnapshot {
        revision,
        window,
        logical_size: layout.logical_size,
        scale_factor,
        layout_fingerprint,
        scene_fingerprint,
        native_root: Arc::new(native_root),
        layout: Arc::new(layout),
        scene: Arc::new(scene),
        accessibility,
        interaction: Arc::new(interaction),
        damage,
    };
    snapshot.render_frame().validate()?;
    Ok(snapshot)
}

fn scene_damage(previous: Option<&Scene>, next: &Scene) -> GuiResult<Vec<Rect>> {
    let bounds = match previous {
        None => Some(next.bounds()),
        Some(previous) => {
            let diff = previous.diff(next)?;
            if !diff.full_repaint && diff.changes.is_empty() {
                None
            } else if diff.full_repaint {
                Some(next.bounds())
            } else {
                diff.dirty_bounds
            }
        }
    };
    Ok(bounds
        .map(|bounds| {
            vec![Rect::new(
                f64::from(bounds.x),
                f64::from(bounds.y),
                f64::from(bounds.width),
                f64::from(bounds.height),
            )]
        })
        .unwrap_or_default())
}
