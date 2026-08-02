use crate::geometry::Size;
use crate::native::{NativeElement, NativeRole};
use crate::platform_host::{
    PlatformPresentationStatus, PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost,
};

use super::{
    PlatformSceneDeferral, RecordingScenePresenter, SelfDrawnFrameCommitStatus,
    SelfDrawnWindowRuntime,
};

fn runtime() -> SelfDrawnWindowRuntime<RecordingPlatformHost, RecordingScenePresenter> {
    SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        RecordingScenePresenter::new(),
        PlatformWindowSpec {
            id: PlatformWindowId::new(1),
            title: "deferred presentation test".to_string(),
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
fn first_deferred_frame_releases_the_staged_target_and_retains_no_snapshot() {
    let mut runtime = runtime();
    runtime
        .presenter_mut()
        .defer_next_prepare(PlatformSceneDeferral::Dropped);

    let commit = runtime
        .render(NativeElement::new("root", NativeRole::View))
        .unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Deferred);
    assert_eq!(
        commit.presentation_status,
        Some(PlatformPresentationStatus::Dropped)
    );
    assert!(runtime.snapshot().is_none());
    assert!(runtime.pending_redraw());
    assert_eq!(runtime.presenter().surface_loss_count(), 1);
    assert!(runtime.host().committed().is_empty());
}

#[test]
fn deferred_redraw_keeps_the_previous_snapshot_and_surface() {
    let mut runtime = runtime();
    let revision = runtime
        .render(NativeElement::new("root", NativeRole::View))
        .unwrap()
        .revision;
    runtime
        .presenter_mut()
        .defer_next_prepare(PlatformSceneDeferral::Dropped);

    let commit = runtime.redraw().unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Deferred);
    assert_eq!(commit.revision, revision);
    assert_eq!(runtime.snapshot().unwrap().revision(), revision);
    assert!(runtime.pending_redraw());
    assert_eq!(runtime.presenter().surface_loss_count(), 0);
    assert_eq!(runtime.host().committed().len(), 1);
}
