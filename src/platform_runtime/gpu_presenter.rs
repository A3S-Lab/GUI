use a3s_graphics::{
    FramePlanner, GpuCapabilities, GpuPreparedSurfaceFrame, GpuRendererOptions,
    GpuSurfaceAcquireStatus, GpuSurfaceFrame, GpuSurfacePreparation, GpuSurfaceRenderer,
    GraphicsError,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::error::{GuiError, GuiResult};
use crate::platform_host::PlatformWindowId;

use super::{
    PlatformRenderFrame, PlatformSceneDeferral, PlatformScenePreparation, PlatformScenePresenter,
    PlatformScenePublishStatus,
};

/// Candidate GPU work retained until the matching host transaction commits.
#[derive(Debug)]
pub struct GpuPreparedSceneFrame {
    frame: PlatformRenderFrame,
    inner: GpuPreparedSurfaceFrame,
}

impl GpuPreparedSceneFrame {
    pub fn frame(&self) -> &PlatformRenderFrame {
        &self.frame
    }
}

/// Metadata for the last frame synchronously handed to native presentation.
#[derive(Debug, Clone)]
pub struct GpuPresentedSceneFrame {
    frame: PlatformRenderFrame,
    gpu: GpuSurfaceFrame,
}

impl GpuPresentedSceneFrame {
    pub fn frame(&self) -> &PlatformRenderFrame {
        &self.frame
    }

    pub const fn gpu(&self) -> GpuSurfaceFrame {
        self.gpu
    }
}

/// Graphics-owned presenter for a host-provided native window target.
///
/// This type creates no window or platform widget. It owns the Graphics
/// surface renderer and retains the host's lifetime token until surface loss,
/// shutdown, or presenter drop.
pub struct GpuScenePresenter {
    options: GpuRendererOptions,
    planner: FramePlanner,
    renderer: Option<GpuSurfaceRenderer>,
    committed: Option<GpuPresentedSceneFrame>,
    prepare_count: u64,
    publish_count: u64,
    discard_count: u64,
    deferred_count: u64,
    surface_loss_count: u64,
    last_failure: Option<String>,
    shutdown: bool,
}

impl std::fmt::Debug for GpuScenePresenter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GpuScenePresenter")
            .field("options", &self.options)
            .field("attached", &self.renderer.is_some())
            .field(
                "committed_revision",
                &self.committed.as_ref().map(|frame| frame.frame.revision),
            )
            .field("prepare_count", &self.prepare_count)
            .field("publish_count", &self.publish_count)
            .field("discard_count", &self.discard_count)
            .field("deferred_count", &self.deferred_count)
            .field("surface_loss_count", &self.surface_loss_count)
            .field("last_failure", &self.last_failure)
            .field("shutdown", &self.shutdown)
            .finish_non_exhaustive()
    }
}

impl Default for GpuScenePresenter {
    fn default() -> Self {
        Self::with_options(GpuRendererOptions::default())
    }
}

impl GpuScenePresenter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: GpuRendererOptions) -> Self {
        Self {
            options,
            planner: FramePlanner::new(),
            renderer: None,
            committed: None,
            prepare_count: 0,
            publish_count: 0,
            discard_count: 0,
            deferred_count: 0,
            surface_loss_count: 0,
            last_failure: None,
            shutdown: false,
        }
    }

    pub fn capabilities(&self) -> Option<&GpuCapabilities> {
        self.renderer.as_ref().map(GpuSurfaceRenderer::capabilities)
    }

    pub fn committed(&self) -> Option<&GpuPresentedSceneFrame> {
        self.committed.as_ref()
    }

    pub const fn prepare_count(&self) -> u64 {
        self.prepare_count
    }

    pub const fn publish_count(&self) -> u64 {
        self.publish_count
    }

    pub const fn discard_count(&self) -> u64 {
        self.discard_count
    }

    pub const fn deferred_count(&self) -> u64 {
        self.deferred_count
    }

    pub const fn surface_loss_count(&self) -> u64 {
        self.surface_loss_count
    }

    pub fn last_failure(&self) -> Option<&str> {
        self.last_failure.as_deref()
    }

    pub const fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    /// Destroys the attached Graphics device between frames so recovery tests
    /// can exercise the typed device-loss path.
    #[cfg(feature = "gpu-fault-injection")]
    pub fn inject_device_loss(&mut self) -> GuiResult<()> {
        self.ensure_running()?;
        let renderer = self
            .renderer
            .as_mut()
            .ok_or_else(|| GuiError::graphics("GPU scene presenter has no attached device"))?;
        renderer.inject_device_loss();
        Ok(())
    }

    fn ensure_running(&self) -> GuiResult<()> {
        if self.shutdown {
            Err(GuiError::host("GPU scene presenter is shut down"))
        } else {
            Ok(())
        }
    }

    fn detach_surface(&mut self) {
        self.renderer = None;
        self.planner.reset();
        self.committed = None;
    }

    fn defer_device_loss(&mut self, error: GraphicsError) {
        self.deferred_count = self.deferred_count.saturating_add(1);
        self.last_failure = Some(error.to_string());
        self.detach_surface();
    }
}

impl<T> PlatformScenePresenter<T> for GpuScenePresenter
where
    T: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static,
{
    type PreparedFrame = GpuPreparedSceneFrame;

    fn prepare(
        &mut self,
        target: T,
        frame: PlatformRenderFrame,
    ) -> GuiResult<PlatformScenePreparation<Self::PreparedFrame>> {
        self.ensure_running()?;
        frame.validate()?;
        if self.renderer.is_none() {
            match pollster::block_on(GpuSurfaceRenderer::request(target, self.options)) {
                Ok(renderer) => self.renderer = Some(renderer),
                Err(error @ GraphicsError::GpuDeviceLost { .. }) => {
                    self.defer_device_loss(error);
                    return Ok(PlatformScenePreparation::Deferred(
                        PlatformSceneDeferral::SurfaceLost,
                    ));
                }
                Err(error) => return Err(error.into()),
            }
        }
        let graphics_frame = self.planner.plan((*frame.scene).clone())?;
        if graphics_frame.fingerprint != frame.scene_fingerprint {
            self.planner.reset();
            return Err(GuiError::graphics(
                "GPU presenter planner fingerprint diverged from the runtime frame",
            ));
        }
        let renderer = self
            .renderer
            .as_mut()
            .ok_or_else(|| GuiError::graphics("GPU surface renderer was not attached"))?;
        let preparation = match pollster::block_on(renderer.prepare(&graphics_frame)) {
            Ok(preparation) => preparation,
            Err(error @ GraphicsError::GpuDeviceLost { .. }) => {
                self.defer_device_loss(error);
                return Ok(PlatformScenePreparation::Deferred(
                    PlatformSceneDeferral::SurfaceLost,
                ));
            }
            Err(error) => {
                self.planner.reset();
                return Err(error.into());
            }
        };
        match preparation {
            GpuSurfacePreparation::Ready(inner) => {
                self.prepare_count = self.prepare_count.saturating_add(1);
                Ok(PlatformScenePreparation::Ready(GpuPreparedSceneFrame {
                    frame,
                    inner,
                }))
            }
            GpuSurfacePreparation::Deferred(status) => {
                self.deferred_count = self.deferred_count.saturating_add(1);
                self.planner.reset();
                Ok(PlatformScenePreparation::Deferred(scene_deferral(status)))
            }
        }
    }

    fn publish(&mut self, prepared: Self::PreparedFrame) -> PlatformScenePublishStatus {
        let GpuPreparedSceneFrame { frame, inner } = prepared;
        let result = self
            .renderer
            .as_mut()
            .ok_or_else(|| GuiError::graphics("GPU surface renderer was detached before publish"))
            .and_then(|renderer| renderer.present(inner).map_err(GuiError::from));
        match result {
            Ok(gpu) => {
                self.publish_count = self.publish_count.saturating_add(1);
                self.last_failure = None;
                self.committed = Some(GpuPresentedSceneFrame { frame, gpu });
                PlatformScenePublishStatus::Presented
            }
            Err(error) => {
                self.last_failure = Some(error.to_string());
                self.detach_surface();
                PlatformScenePublishStatus::SurfaceLost
            }
        }
    }

    fn discard(&mut self, _prepared: Self::PreparedFrame) {
        self.discard_count = self.discard_count.saturating_add(1);
        self.planner.reset();
    }

    fn surface_lost(&mut self, window: PlatformWindowId) -> GuiResult<()> {
        self.ensure_running()?;
        window.validate()?;
        self.surface_loss_count = self.surface_loss_count.saturating_add(1);
        self.detach_surface();
        Ok(())
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        if self.shutdown {
            return Ok(());
        }
        self.detach_surface();
        self.shutdown = true;
        Ok(())
    }
}

const fn scene_deferral(status: GpuSurfaceAcquireStatus) -> PlatformSceneDeferral {
    match status {
        GpuSurfaceAcquireStatus::Timeout
        | GpuSurfaceAcquireStatus::Occluded
        | GpuSurfaceAcquireStatus::Outdated => PlatformSceneDeferral::Dropped,
        GpuSurfaceAcquireStatus::Lost => PlatformSceneDeferral::SurfaceLost,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_acquire_outcomes_map_to_runtime_recovery_statuses() {
        assert_eq!(
            scene_deferral(GpuSurfaceAcquireStatus::Timeout),
            PlatformSceneDeferral::Dropped
        );
        assert_eq!(
            scene_deferral(GpuSurfaceAcquireStatus::Occluded),
            PlatformSceneDeferral::Dropped
        );
        assert_eq!(
            scene_deferral(GpuSurfaceAcquireStatus::Outdated),
            PlatformSceneDeferral::Dropped
        );
        assert_eq!(
            scene_deferral(GpuSurfaceAcquireStatus::Lost),
            PlatformSceneDeferral::SurfaceLost
        );
    }

    #[test]
    fn public_gpu_presenter_records_are_thread_safe() {
        fn assert_send_sync<T: Send + Sync>() {}
        fn assert_send<T: Send>() {}

        assert_send_sync::<GpuScenePresenter>();
        assert_send::<GpuPreparedSceneFrame>();
        assert_send_sync::<GpuPresentedSceneFrame>();
    }
}
