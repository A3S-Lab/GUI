use std::sync::Arc;

use crate::error::{GuiError, GuiResult};
use crate::geometry::Size;
use crate::native::{NativeElement, NativeRole};
use crate::platform_host::{
    PlatformHost, PlatformHostCommand, PlatformHostCommitAck, PlatformHostEvent,
    PlatformHostTransaction, PlatformWindowId, PlatformWindowSpec,
};

use super::{
    PlatformRenderFrame, PlatformScenePreparation, PlatformScenePresenter,
    PlatformScenePublishStatus, SelfDrawnWindowRuntime,
};

#[derive(Clone)]
struct LeaseTarget(Arc<()>);

struct LeaseHost {
    owner: Arc<()>,
    pending: Option<PlatformHostTransaction>,
}

impl LeaseHost {
    fn new() -> Self {
        Self {
            owner: Arc::new(()),
            pending: None,
        }
    }
}

impl PlatformHost for LeaseHost {
    type PresentationTarget = LeaseTarget;

    fn prepare(&mut self, transaction: PlatformHostTransaction) -> GuiResult<()> {
        transaction.validate()?;
        self.pending = Some(transaction);
        Ok(())
    }

    fn presentation_target(&self, window: PlatformWindowId) -> GuiResult<Self::PresentationTarget> {
        let requested = self.pending.as_ref().is_some_and(|pending| {
            pending.commands.iter().any(|command| {
                matches!(
                    command,
                    PlatformHostCommand::Present { request } if request.window == window
                )
            })
        });
        if !requested {
            return Err(GuiError::host("test host has no pending presentation"));
        }
        Ok(LeaseTarget(Arc::clone(&self.owner)))
    }

    fn commit(&mut self) -> GuiResult<PlatformHostCommitAck> {
        Err(GuiError::host("injected commit failure"))
    }

    fn rollback(&mut self) -> GuiResult<()> {
        if Arc::strong_count(&self.owner) != 1 {
            return Err(GuiError::host(
                "test host rollback observed an active surface lease",
            ));
        }
        self.pending = None;
        Ok(())
    }

    fn poll_event(&mut self) -> GuiResult<Option<PlatformHostEvent>> {
        Ok(None)
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        Ok(())
    }
}

struct LeasePreparedFrame(LeaseTarget);

#[derive(Default)]
struct LeasePresenter {
    retain_then_fail_prepare: bool,
    attached: Option<LeaseTarget>,
    discard_count: u64,
    surface_loss_count: u64,
}

impl PlatformScenePresenter<LeaseTarget> for LeasePresenter {
    type PreparedFrame = LeasePreparedFrame;

    fn prepare(
        &mut self,
        target: LeaseTarget,
        _frame: PlatformRenderFrame,
    ) -> GuiResult<PlatformScenePreparation<Self::PreparedFrame>> {
        if self.retain_then_fail_prepare {
            self.attached = Some(target);
            return Err(GuiError::graphics("injected prepare failure"));
        }
        Ok(PlatformScenePreparation::Ready(LeasePreparedFrame(target)))
    }

    fn publish(&mut self, _prepared: Self::PreparedFrame) -> PlatformScenePublishStatus {
        PlatformScenePublishStatus::Presented
    }

    fn discard(&mut self, prepared: Self::PreparedFrame) {
        let LeasePreparedFrame(target) = prepared;
        let LeaseTarget(owner) = target;
        drop(owner);
        self.discard_count = self.discard_count.saturating_add(1);
    }

    fn surface_lost(&mut self, window: PlatformWindowId) -> GuiResult<()> {
        window.validate()?;
        self.attached = None;
        self.surface_loss_count = self.surface_loss_count.saturating_add(1);
        Ok(())
    }

    fn shutdown(&mut self) -> GuiResult<()> {
        self.attached = None;
        Ok(())
    }
}

fn runtime(presenter: LeasePresenter) -> SelfDrawnWindowRuntime<LeaseHost, LeasePresenter> {
    SelfDrawnWindowRuntime::new(
        LeaseHost::new(),
        presenter,
        PlatformWindowSpec {
            id: PlatformWindowId::new(1),
            title: "lease order test".to_string(),
            logical_size: Size::new(80.0, 60.0),
            min_size: None,
            max_size: None,
            resizable: true,
            visible: false,
        },
        1.0,
    )
    .unwrap()
}

#[test]
fn commit_failure_discards_prepared_surface_before_host_rollback() {
    let mut runtime = runtime(LeasePresenter::default());

    let error = runtime
        .render(NativeElement::new("root", NativeRole::View))
        .unwrap_err();

    assert!(error.to_string().contains("injected commit failure"));
    assert!(!error.to_string().contains("active surface lease"));
    assert!(runtime.host().pending.is_none());
    assert_eq!(runtime.presenter().discard_count, 1);
    assert_eq!(runtime.presenter().surface_loss_count, 1);
}

#[test]
fn first_prepare_failure_detaches_surface_before_host_rollback() {
    let mut runtime = runtime(LeasePresenter {
        retain_then_fail_prepare: true,
        ..LeasePresenter::default()
    });

    let error = runtime
        .render(NativeElement::new("root", NativeRole::View))
        .unwrap_err();

    assert!(error.to_string().contains("injected prepare failure"));
    assert!(!error.to_string().contains("active surface lease"));
    assert!(runtime.host().pending.is_none());
    assert_eq!(runtime.presenter().discard_count, 0);
    assert_eq!(runtime.presenter().surface_loss_count, 1);
}
