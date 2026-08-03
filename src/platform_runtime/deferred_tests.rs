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
    assert!(runtime.retry_pending_redraw().unwrap().is_none());
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

#[test]
fn pending_surface_loss_retries_once_against_the_retained_scene() {
    let mut runtime = runtime();
    let revision = runtime
        .render(NativeElement::new("root", NativeRole::View))
        .unwrap()
        .revision;
    runtime
        .presenter_mut()
        .defer_next_prepare(PlatformSceneDeferral::SurfaceLost);

    let lost = runtime.redraw().unwrap();

    assert_eq!(lost.status, SelfDrawnFrameCommitStatus::Deferred);
    assert_eq!(lost.revision, revision);
    assert_eq!(
        lost.presentation_status,
        Some(PlatformPresentationStatus::SurfaceLost)
    );
    assert_eq!(runtime.stats().surface_recoveries, 1);
    assert!(runtime.pending_redraw());

    let recovered = runtime.retry_pending_redraw().unwrap().unwrap();

    assert_eq!(recovered.status, SelfDrawnFrameCommitStatus::Committed);
    assert_eq!(
        recovered.presentation_status,
        Some(PlatformPresentationStatus::Presented)
    );
    assert!(!runtime.pending_redraw());
    assert!(runtime.retry_pending_redraw().unwrap().is_none());
}
