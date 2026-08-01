//! Boundary between GUI scene production and the standalone graphics engine.
//!
//! This module exposes the engine's platform-neutral scene vocabulary without
//! leaking `wgpu` or window-system types into GUI semantics. The optional
//! software reference renderer is deterministic and intended for tests,
//! snapshots, and GPU parity checks.

pub use a3s_graphics::{
    Affine, Color, CornerRadii, Damage, DrawCommand, DrawId, EdgeWidths, FillRect, FillRoundedRect,
    Primitive, Rect, Scene, SceneBuilder, Size, StrokeRect, SCENE_SCHEMA_VERSION,
};

#[cfg(feature = "gpu")]
pub use a3s_graphics::{
    GpuBackend, GpuBackendPreference, GpuCapabilities, GpuDeviceType, GpuFrame, GpuPowerPreference,
    GpuReadback, GpuRendererOptions,
};

#[cfg(any(feature = "software-reference", feature = "gpu"))]
use a3s_graphics::FramePlanner;
#[cfg(feature = "software-reference")]
use a3s_graphics::SoftwareRenderer;

#[cfg(any(feature = "software-reference", feature = "gpu"))]
use crate::GuiResult;

/// Metadata and pixels produced by one deterministic reference frame.
#[cfg(feature = "software-reference")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReferenceFrame<'a> {
    index: u64,
    fingerprint: u64,
    damage: Damage,
    width: u32,
    height: u32,
    rgba8: &'a [u8],
}

#[cfg(feature = "software-reference")]
impl<'a> ReferenceFrame<'a> {
    pub const fn index(self) -> u64 {
        self.index
    }

    pub const fn fingerprint(self) -> u64 {
        self.fingerprint
    }

    pub const fn damage(self) -> Damage {
        self.damage
    }

    pub const fn width(self) -> u32 {
        self.width
    }

    pub const fn height(self) -> u32 {
        self.height
    }

    pub const fn rgba8(self) -> &'a [u8] {
        self.rgba8
    }
}

/// Stateful reference renderer used to verify GUI scene extraction.
///
/// Stable draw IDs let the internal planner calculate retained damage across
/// calls. Use [`Self::reset`] when the associated surface or document is
/// discarded.
#[cfg(feature = "software-reference")]
#[derive(Debug, Default)]
pub struct ReferenceRenderer {
    planner: FramePlanner,
    renderer: SoftwareRenderer,
}

#[cfg(feature = "software-reference")]
impl ReferenceRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, scene: Scene) -> GuiResult<ReferenceFrame<'_>> {
        let frame = self.planner.plan(scene)?;
        let index = frame.index;
        let fingerprint = frame.fingerprint;
        let damage = frame.damage;
        let pixels = self.renderer.render(&frame)?;
        Ok(ReferenceFrame {
            index,
            fingerprint,
            damage,
            width: pixels.width(),
            height: pixels.height(),
            rgba8: pixels.as_rgba8(),
        })
    }

    pub fn reset(&mut self) {
        self.planner.reset();
        self.renderer = SoftwareRenderer::new();
    }
}

/// GPU renderer that plans retained frames directly from GUI-produced scenes.
#[cfg(feature = "gpu")]
#[derive(Debug)]
pub struct GpuSceneRenderer {
    planner: FramePlanner,
    renderer: a3s_graphics::GpuRenderer,
}

#[cfg(feature = "gpu")]
impl GpuSceneRenderer {
    pub async fn request(options: GpuRendererOptions) -> GuiResult<Self> {
        Ok(Self {
            planner: FramePlanner::new(),
            renderer: a3s_graphics::GpuRenderer::request(options).await?,
        })
    }

    pub fn capabilities(&self) -> &GpuCapabilities {
        self.renderer.capabilities()
    }

    pub async fn render(&mut self, scene: Scene) -> GuiResult<GpuFrame> {
        let frame = self.planner.plan(scene)?;
        Ok(self.renderer.render(&frame).await?)
    }

    pub async fn request_readback(&self) -> GuiResult<GpuReadbackTicket> {
        Ok(GpuReadbackTicket {
            inner: self.renderer.request_readback().await?,
        })
    }

    pub fn poll(&self) -> GuiResult<bool> {
        Ok(self.renderer.poll()?)
    }

    pub fn wait(&self) -> GuiResult<()> {
        Ok(self.renderer.wait()?)
    }

    pub fn reset(&mut self) {
        self.planner.reset();
    }
}

/// GUI-error-aware handle for an asynchronous GPU readback.
#[cfg(feature = "gpu")]
#[derive(Debug)]
pub struct GpuReadbackTicket {
    inner: a3s_graphics::GpuReadbackTicket,
}

#[cfg(feature = "gpu")]
impl GpuReadbackTicket {
    pub fn try_finish(&mut self) -> GuiResult<Option<GpuReadback>> {
        Ok(self.inner.try_finish()?)
    }

    pub fn finish(self) -> GuiResult<GpuReadback> {
        Ok(self.inner.finish()?)
    }
}

#[cfg(all(test, feature = "software-reference"))]
mod tests {
    use super::*;
    use crate::GuiError;

    fn scene() -> Scene {
        let mut builder = SceneBuilder::new(Size::new(4.0, 3.0), 1.0, Color::WHITE);
        builder
            .push(DrawCommand::new(
                DrawId::new(1).unwrap(),
                Primitive::FillRect(FillRect {
                    rect: Rect::new(1.0, 1.0, 2.0, 1.0),
                    color: Color::BLACK,
                }),
            ))
            .unwrap();
        builder.finish().unwrap()
    }

    #[test]
    fn reference_renderer_retains_identical_scenes() {
        let mut renderer = ReferenceRenderer::new();
        let first = renderer.render(scene()).unwrap();
        assert_eq!((first.width(), first.height()), (4, 3));
        assert!(first.damage().full_repaint);
        let fingerprint = first.fingerprint();
        let pixels = first.rgba8().to_vec();

        let second = renderer.render(scene()).unwrap();
        assert_eq!(second.index(), 1);
        assert_eq!(second.fingerprint(), fingerprint);
        assert!(!second.damage().requires_render());
        assert_eq!(second.rgba8(), pixels);
    }

    #[test]
    fn reset_forces_a_full_reference_frame_without_rewinding_diagnostics() {
        let mut renderer = ReferenceRenderer::new();
        renderer.render(scene()).unwrap();
        renderer.reset();

        let frame = renderer.render(scene()).unwrap();
        assert_eq!(frame.index(), 1);
        assert!(frame.damage().full_repaint);
    }

    #[test]
    fn graphics_validation_errors_keep_the_gui_boundary() {
        let mut renderer = ReferenceRenderer::new();
        let invalid = Scene::new(Size::new(0.0, 3.0), 1.0, Color::WHITE);

        let error = renderer.render(invalid).unwrap_err();
        assert!(matches!(error, GuiError::Graphics { .. }));
        assert!(error.to_string().contains("logical surface size"));
    }

    #[test]
    fn reference_renderer_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ReferenceRenderer>();
    }
}

#[cfg(all(test, feature = "gpu"))]
mod gpu_tests {
    use super::*;
    use crate::GuiError;

    fn scene() -> Scene {
        let mut builder = SceneBuilder::new(Size::new(4.0, 3.0), 1.0, Color::WHITE);
        builder
            .push(DrawCommand::new(
                DrawId::new(1).unwrap(),
                Primitive::FillRect(FillRect {
                    rect: Rect::new(1.0, 1.0, 2.0, 1.0),
                    color: Color::BLACK,
                }),
            ))
            .unwrap();
        builder.finish().unwrap()
    }

    #[test]
    fn invalid_gpu_options_keep_the_gui_error_boundary() {
        let options = GpuRendererOptions {
            max_instances_per_frame: 0,
            ..Default::default()
        };
        let error = pollster::block_on(GpuSceneRenderer::request(options)).unwrap_err();
        assert!(matches!(error, GuiError::Graphics { .. }));
        assert!(error.to_string().contains("max_instances_per_frame"));
    }

    #[test]
    fn gpu_scene_renderer_completes_the_gui_scene_boundary() {
        let options = GpuRendererOptions {
            power_preference: GpuPowerPreference::None,
            allow_software_adapter: true,
            ..Default::default()
        };
        let mut renderer = match pollster::block_on(GpuSceneRenderer::request(options)) {
            Ok(renderer) => renderer,
            Err(error) if error.to_string().contains("no compatible GPU adapter") => return,
            Err(error) => panic!("failed to initialize GUI test GPU: {error}"),
        };

        let frame = pollster::block_on(renderer.render(scene())).unwrap();
        assert!(frame.rendered);
        let image = pollster::block_on(renderer.request_readback())
            .unwrap()
            .finish()
            .unwrap();
        assert_eq!(image.pixel(1, 1), Some(Color::BLACK));
    }

    #[test]
    fn gpu_scene_renderer_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        fn assert_send<T: Send>() {}
        assert_send_sync::<GpuSceneRenderer>();
        assert_send::<GpuReadbackTicket>();
    }
}
