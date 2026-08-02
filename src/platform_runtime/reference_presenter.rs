use crate::drawing::ReferenceRenderer;
use crate::error::{GuiError, GuiResult};
use crate::platform_host::PlatformWindowId;

use super::{
    PlatformRenderFrame, PlatformScenePreparation, PlatformScenePresenter,
    PlatformScenePublishStatus,
};

/// Transactional software-rendered candidate kept private until host commit.
#[derive(Debug, Clone)]
pub struct ReferencePreparedFrame {
    frame: PlatformRenderFrame,
    width: u32,
    height: u32,
    rgba8: Vec<u8>,
}

impl ReferencePreparedFrame {
    pub fn frame(&self) -> &PlatformRenderFrame {
        &self.frame
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }
}

/// Last software frame published after the matching host transaction.
#[derive(Debug, Clone)]
pub struct ReferencePresentedFrame {
    prepared: ReferencePreparedFrame,
}

impl ReferencePresentedFrame {
    pub fn frame(&self) -> &PlatformRenderFrame {
        self.prepared.frame()
    }

    pub fn width(&self) -> u32 {
        self.prepared.width()
    }

    pub fn height(&self) -> u32 {
        self.prepared.height()
    }

    pub fn rgba8(&self) -> &[u8] {
        self.prepared.rgba8()
    }
}

/// Deterministic H1 presenter that proves the committed scene produces pixels.
///
/// Each prepare reconstructs retained software state from the last published
/// scene before rendering the candidate. A rejected host transaction can
/// therefore discard the candidate without mutating the published renderer.
#[derive(Debug, Default)]
pub struct ReferenceScenePresenter {
    committed: Option<ReferencePresentedFrame>,
    prepare_count: u64,
    publish_count: u64,
    discard_count: u64,
    surface_loss_count: u64,
    surface_valid: bool,
    shutdown: bool,
}

impl ReferenceScenePresenter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn committed(&self) -> Option<&ReferencePresentedFrame> {
        self.committed.as_ref()
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
}

impl<T> PlatformScenePresenter<T> for ReferenceScenePresenter {
    type PreparedFrame = ReferencePreparedFrame;

    fn prepare(
        &mut self,
        _target: T,
        frame: PlatformRenderFrame,
    ) -> GuiResult<PlatformScenePreparation<Self::PreparedFrame>> {
        if self.shutdown {
            return Err(GuiError::host("reference scene presenter is shut down"));
        }
        frame.validate()?;
        let mut renderer = ReferenceRenderer::new();
        if self.surface_valid {
            if let Some(committed) = &self.committed {
                renderer.render((*committed.frame().scene).clone())?;
            }
        }
        let rendered = renderer.render((*frame.scene).clone())?;
        let prepared = ReferencePreparedFrame {
            frame,
            width: rendered.width(),
            height: rendered.height(),
            rgba8: rendered.rgba8().to_vec(),
        };
        self.prepare_count = self.prepare_count.saturating_add(1);
        Ok(PlatformScenePreparation::Ready(prepared))
    }

    fn publish(&mut self, prepared: Self::PreparedFrame) -> PlatformScenePublishStatus {
        self.publish_count = self.publish_count.saturating_add(1);
        self.committed = Some(ReferencePresentedFrame { prepared });
        self.surface_valid = true;
        PlatformScenePublishStatus::Presented
    }

    fn discard(&mut self, _prepared: Self::PreparedFrame) {
        self.discard_count = self.discard_count.saturating_add(1);
    }

    fn surface_lost(&mut self, window: PlatformWindowId) -> GuiResult<()> {
        if self.shutdown {
            return Err(GuiError::host("reference scene presenter is shut down"));
        }
        window.validate()?;
        self.surface_loss_count = self.surface_loss_count.saturating_add(1);
        self.surface_valid = false;
        Ok(())
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        self.committed = None;
        self.surface_valid = false;
        self.shutdown = true;
        Ok(())
    }
}
