use crate::error::{GuiError, GuiResult};
use crate::platform_host::PlatformWindowId;

use super::PlatformRenderFrame;

pub const DEFAULT_RECORDING_SCENE_HISTORY_LIMIT: usize = 256;

/// Thread-affine Graphics edge attached to a platform-owned raw surface.
///
/// `prepare` may validate, plan, upload, or render, but it must not expose the
/// candidate frame. The runtime commits the zero-widget host transaction first
/// and then calls infallible `publish`. Dropping or explicitly discarding the
/// prepared value keeps the previously published surface contents intact.
pub trait PlatformScenePresenter {
    type PreparedFrame;

    fn prepare(&mut self, frame: PlatformRenderFrame) -> GuiResult<Self::PreparedFrame>;

    fn publish(&mut self, prepared: Self::PreparedFrame);

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
/// It contains no window or widget operation. A future H2-H4 presenter owns
/// the raw surface attachment while implementing this same prepare/publish
/// protocol.
pub struct RecordingScenePresenter {
    committed: Option<PlatformRenderFrame>,
    history: Vec<PlatformRenderFrame>,
    history_limit: usize,
    prepare_count: u64,
    publish_count: u64,
    discard_count: u64,
    surface_loss_count: u64,
    fail_next_prepare: Option<String>,
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

impl PlatformScenePresenter for RecordingScenePresenter {
    type PreparedFrame = RecordingPreparedFrame;

    fn prepare(&mut self, frame: PlatformRenderFrame) -> GuiResult<Self::PreparedFrame> {
        if self.shutdown {
            return Err(GuiError::host("scene presenter is shut down"));
        }
        frame.validate()?;
        if let Some(message) = self.fail_next_prepare.take() {
            return Err(GuiError::graphics(format!(
                "scene presenter prepare failed: {message}"
            )));
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
        Ok(RecordingPreparedFrame { frame })
    }

    fn publish(&mut self, prepared: Self::PreparedFrame) {
        self.publish_count = self.publish_count.saturating_add(1);
        self.committed = Some(prepared.frame.clone());
        self.push_history(prepared.frame);
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
