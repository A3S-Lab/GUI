use crate::error::{GuiError, GuiResult};
use crate::platform_host::{PlatformPresentationStatus, PlatformWindowId};

use super::PlatformRenderFrame;

pub const DEFAULT_RECORDING_SCENE_HISTORY_LIMIT: usize = 256;

/// Recoverable reason why a candidate could not acquire a surface frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformSceneDeferral {
    Dropped,
    SurfaceLost,
}

impl From<PlatformSceneDeferral> for PlatformPresentationStatus {
    fn from(status: PlatformSceneDeferral) -> Self {
        match status {
            PlatformSceneDeferral::Dropped => Self::Dropped,
            PlatformSceneDeferral::SurfaceLost => Self::SurfaceLost,
        }
    }
}

/// Outcome of preparing one candidate against a platform-owned surface.
#[derive(Debug)]
pub enum PlatformScenePreparation<F> {
    Ready(F),
    Deferred(PlatformSceneDeferral),
}

/// Synchronous result of publishing a prepared frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformScenePublishStatus {
    Presented,
    Dropped,
    SurfaceLost,
}

impl From<PlatformScenePublishStatus> for PlatformPresentationStatus {
    fn from(status: PlatformScenePublishStatus) -> Self {
        match status {
            PlatformScenePublishStatus::Presented => Self::Presented,
            PlatformScenePublishStatus::Dropped => Self::Dropped,
            PlatformScenePublishStatus::SurfaceLost => Self::SurfaceLost,
        }
    }
}

/// Thread-affine Graphics edge attached to a platform-owned raw surface.
///
/// `prepare` may validate, plan, upload, or render, but it must not expose the
/// candidate frame. The runtime stages the host transaction first, passes its
/// owned target into `prepare`, commits the host, and then calls infallible
/// `publish`. Dropping or explicitly discarding the prepared value keeps the
/// previously published surface contents intact. A deferred preparation may
/// report only `Dropped` or `SurfaceLost`. Returning `SurfaceLost` from
/// `publish`, or completing `surface_lost` or `shutdown`, must release every
/// retained target lease before returning.
pub trait PlatformScenePresenter<T> {
    type PreparedFrame;

    fn prepare(
        &mut self,
        target: T,
        frame: PlatformRenderFrame,
    ) -> GuiResult<PlatformScenePreparation<Self::PreparedFrame>>;

    fn publish(&mut self, prepared: Self::PreparedFrame) -> PlatformScenePublishStatus;

    fn discard(&mut self, prepared: Self::PreparedFrame);

    fn surface_lost(&mut self, window: PlatformWindowId) -> GuiResult<()>;

    fn shutdown(&mut self) -> GuiResult<()>;
}

#[derive(Debug, Clone)]
pub struct RecordingPreparedFrame {
    frame: PlatformRenderFrame,
}

impl RecordingPreparedFrame {
    pub fn frame(&self) -> &PlatformRenderFrame {
        &self.frame
    }
}

/// Deterministic presenter used to prove H1 transaction and recovery behavior.
///
/// It contains no window or widget operation. The GPU presenter implements the
/// same prepare/publish protocol against a Graphics-owned native surface.
pub struct RecordingScenePresenter {
    committed: Option<PlatformRenderFrame>,
    history: Vec<PlatformRenderFrame>,
    history_limit: usize,
    prepare_count: u64,
    publish_count: u64,
    discard_count: u64,
    surface_loss_count: u64,
    fail_next_prepare: Option<String>,
    defer_next_prepare: Option<PlatformSceneDeferral>,
    shutdown: bool,
}

impl std::fmt::Debug for RecordingScenePresenter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecordingScenePresenter")
            .field(
                "committed_revision",
                &self.committed.as_ref().map(|frame| frame.revision),
            )
            .field("history_len", &self.history.len())
            .field("history_limit", &self.history_limit)
            .field("prepare_count", &self.prepare_count)
            .field("publish_count", &self.publish_count)
            .field("discard_count", &self.discard_count)
            .field("surface_loss_count", &self.surface_loss_count)
            .field("shutdown", &self.shutdown)
            .finish()
    }
}

impl Default for RecordingScenePresenter {
    fn default() -> Self {
        Self::with_history_limit(DEFAULT_RECORDING_SCENE_HISTORY_LIMIT)
    }
}

impl RecordingScenePresenter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_history_limit(history_limit: usize) -> Self {
        Self {
            committed: None,
            history: Vec::new(),
            history_limit,
            prepare_count: 0,
            publish_count: 0,
            discard_count: 0,
            surface_loss_count: 0,
            fail_next_prepare: None,
            defer_next_prepare: None,
            shutdown: false,
        }
    }

    pub fn committed(&self) -> Option<&PlatformRenderFrame> {
        self.committed.as_ref()
    }

    pub fn history(&self) -> &[PlatformRenderFrame] {
        &self.history
    }

    pub fn prepare_count(&self) -> u64 {
        self.prepare_count
    }

    pub fn publish_count(&self) -> u64 {
        self.publish_count
    }

    pub fn discard_count(&self) -> u64 {
        self.discard_count
    }

    pub fn surface_loss_count(&self) -> u64 {
        self.surface_loss_count
    }

    pub fn fail_next_prepare(&mut self, message: impl Into<String>) {
        self.fail_next_prepare = Some(message.into());
    }

    pub fn defer_next_prepare(&mut self, status: PlatformSceneDeferral) {
        self.defer_next_prepare = Some(status);
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    fn push_history(&mut self, frame: PlatformRenderFrame) {
        if self.history_limit == 0 {
            return;
        }
        if self.history.len() == self.history_limit {
            self.history.remove(0);
        }
        self.history.push(frame);
    }
}

impl<T> PlatformScenePresenter<T> for RecordingScenePresenter {
    type PreparedFrame = RecordingPreparedFrame;

    fn prepare(
        &mut self,
        _target: T,
        frame: PlatformRenderFrame,
    ) -> GuiResult<PlatformScenePreparation<Self::PreparedFrame>> {
        if self.shutdown {
            return Err(GuiError::host("scene presenter is shut down"));
        }
        frame.validate()?;
        if let Some(message) = self.fail_next_prepare.take() {
            return Err(GuiError::graphics(format!(
                "scene presenter prepare failed: {message}"
            )));
        }
        if let Some(status) = self.defer_next_prepare.take() {
            return Ok(PlatformScenePreparation::Deferred(status));
        }
        if self
            .committed
            .as_ref()
            .is_some_and(|committed| frame.revision <= committed.revision)
        {
            return Err(GuiError::host(
                "scene presenter revision must advance monotonically",
            ));
        }
        self.prepare_count = self.prepare_count.saturating_add(1);
        Ok(PlatformScenePreparation::Ready(RecordingPreparedFrame {
            frame,
        }))
    }

    fn publish(&mut self, prepared: Self::PreparedFrame) -> PlatformScenePublishStatus {
        self.publish_count = self.publish_count.saturating_add(1);
        self.committed = Some(prepared.frame.clone());
        self.push_history(prepared.frame);
        PlatformScenePublishStatus::Presented
    }

    fn discard(&mut self, _prepared: Self::PreparedFrame) {
        self.discard_count = self.discard_count.saturating_add(1);
    }

    fn surface_lost(&mut self, window: PlatformWindowId) -> GuiResult<()> {
        if self.shutdown {
            return Err(GuiError::host("scene presenter is shut down"));
        }
        window.validate()?;
        self.surface_loss_count = self.surface_loss_count.saturating_add(1);
        Ok(())
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        self.committed = None;
        self.shutdown = true;
        Ok(())
    }
}
