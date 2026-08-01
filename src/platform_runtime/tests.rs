use crate::accessibility::AccessibilityRole;
use crate::geometry::{Rect, Size};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{
    PlatformHostEvent, PlatformPresentationAck, PlatformPresentationStatus, PlatformWindowEvent,
    PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost,
};
use crate::web::WebProps;

use super::*;

fn spec() -> PlatformWindowSpec {
    PlatformWindowSpec {
        id: PlatformWindowId::new(1),
        title: "Self-drawn test".to_string(),
        logical_size: Size::new(100.0, 80.0),
        min_size: None,
        max_size: None,
        resizable: true,
        visible: true,
    }
}

fn tree(label: &str, color: &str) -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name(format!("relative h-[80px] w-[100px] {color}"))),
        )
        .child(
            NativeElement::new("action", NativeRole::Button).with_props(
                NativeProps::new().label(label).web(
                    WebProps::new()
                        .class_name("absolute left-[10px] top-[12px] h-[20px] w-[40px] bg-black"),
                ),
            ),
        )
}

fn runtime() -> SelfDrawnWindowRuntime<RecordingPlatformHost, RecordingScenePresenter> {
    SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        RecordingScenePresenter::new(),
        spec(),
        1.0,
    )
    .unwrap()
}

#[test]
fn first_frame_commits_layout_scene_accessibility_and_presentation_together() {
    let mut runtime = runtime();

    let commit = runtime.render(tree("Run", "bg-white")).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Committed);
    assert!(commit.layout_rebuilt);
    assert!(commit.scene_rebuilt);
    assert!(commit.presentation_requested);
    assert_eq!(commit.host_commands, 3);
    let snapshot = runtime.snapshot().unwrap();
    assert_eq!(snapshot.revision(), commit.revision);
    assert_eq!(snapshot.layout().logical_size, Size::new(100.0, 80.0));
    assert_eq!(snapshot.accessibility().window, spec().id);
    let root = snapshot.accessibility().root.as_ref().unwrap();
    assert_eq!(root.role, AccessibilityRole::Group);
    assert_eq!(root.children[0].label.as_deref(), Some("Run"));
    assert_eq!(
        root.children[0].logical_bounds,
        Rect::new(10.0, 12.0, 40.0, 20.0)
    );
    assert_eq!(runtime.host().committed().len(), 1);
    assert_eq!(runtime.presenter().publish_count(), 1);
}

#[test]
fn identical_native_frame_creates_no_layout_scene_host_or_present_work() {
    let mut runtime = runtime();
    let root = tree("Run", "bg-white");
    runtime.render(root.clone()).unwrap();
    let before = runtime.stats();

    let commit = runtime.render(root).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Unchanged);
    assert!(!commit.layout_rebuilt);
    assert!(!commit.scene_rebuilt);
    assert!(!commit.presentation_requested);
    assert_eq!(commit.host_commands, 0);
    assert_eq!(runtime.stats().layout_builds, before.layout_builds);
    assert_eq!(runtime.stats().scene_builds, before.scene_builds);
    assert_eq!(runtime.stats().host_commits, before.host_commits);
    assert_eq!(runtime.presenter().prepare_count(), 1);
}

#[test]
fn semantic_only_change_commits_accessibility_without_presenting_again() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    let scene_fingerprint = runtime.snapshot().unwrap().scene_fingerprint();

    let commit = runtime.render(tree("Launch", "bg-white")).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Committed);
    assert!(!commit.presentation_requested);
    assert_eq!(commit.host_commands, 1);
    assert_eq!(runtime.presenter().prepare_count(), 1);
    assert_eq!(
        runtime.snapshot().unwrap().scene_fingerprint(),
        scene_fingerprint
    );
    assert_eq!(
        runtime
            .snapshot()
            .unwrap()
            .accessibility()
            .root
            .as_ref()
            .unwrap()
            .children[0]
            .label
            .as_deref(),
        Some("Launch")
    );
}

#[test]
fn rejected_commit_keeps_the_complete_previous_frame_and_discards_prepared_pixels() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    let previous = runtime.snapshot().unwrap().clone();
    runtime.host_mut().fail_next_commit("injected");

    let error = runtime.render(tree("Run", "bg-black")).unwrap_err();

    assert!(error.to_string().contains("injected"));
    assert_eq!(runtime.snapshot().unwrap().revision(), previous.revision());
    assert_eq!(
        runtime.snapshot().unwrap().scene_fingerprint(),
        previous.scene_fingerprint()
    );
    assert_eq!(
        runtime.snapshot().unwrap().accessibility(),
        previous.accessibility()
    );
    assert!(runtime.host().pending().is_none());
    assert_eq!(runtime.presenter().publish_count(), 1);
    assert_eq!(runtime.presenter().discard_count(), 1);

    let retry = runtime.render(tree("Run", "bg-black")).unwrap();
    assert_eq!(retry.revision.get(), previous.revision().get() + 1);
}

#[test]
fn resize_and_scale_events_rebuild_with_stable_semantic_identity() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    let element_id = runtime
        .snapshot()
        .unwrap()
        .accessibility()
        .root
        .as_ref()
        .unwrap()
        .children[0]
        .id
        .clone();

    let resized = runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::Resized {
                window: spec().id,
                logical_size: Size::new(120.0, 90.0),
            },
        })
        .unwrap();
    assert!(matches!(resized, SelfDrawnHostEventOutcome::Frame(_)));
    let scaled = runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::ScaleChanged {
                window: spec().id,
                scale_factor: 2.0,
            },
        })
        .unwrap();
    assert!(matches!(scaled, SelfDrawnHostEventOutcome::Frame(_)));

    let snapshot = runtime.snapshot().unwrap();
    assert_eq!(snapshot.layout().logical_size, Size::new(120.0, 90.0));
    assert_eq!(snapshot.scale_factor(), 2.0);
    assert_eq!(
        snapshot.accessibility().root.as_ref().unwrap().children[0].id,
        element_id
    );
    assert_eq!(runtime.stats().layout_builds, 3);
    assert_eq!(runtime.presenter().publish_count(), 3);
}

#[test]
fn fractional_scale_is_canonicalized_to_the_graphics_precision() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();

    runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::ScaleChanged {
                window: spec().id,
                scale_factor: 1.2,
            },
        })
        .unwrap();

    assert_eq!(
        runtime.snapshot().unwrap().scale_factor(),
        f64::from(1.2_f32)
    );
}

#[test]
fn fractional_window_size_is_canonicalized_once_and_then_retained() {
    let mut fractional = spec();
    fractional.logical_size = Size::new(100.1, 80.1);
    let mut runtime = SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        RecordingScenePresenter::new(),
        fractional,
        1.0,
    )
    .unwrap();
    let root = tree("Run", "bg-white");

    runtime.render(root.clone()).unwrap();
    let expected = Size::new(100.09375, 80.09375);
    assert_eq!(runtime.window_spec().logical_size, expected);
    assert_eq!(runtime.snapshot().unwrap().logical_size(), expected);

    let unchanged = runtime.render(root).unwrap();
    assert_eq!(unchanged.status, SelfDrawnFrameCommitStatus::Unchanged);
}

#[test]
fn occlusion_defers_pixels_until_the_window_is_exposed() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::OcclusionChanged {
                window: spec().id,
                occluded: true,
            },
        })
        .unwrap();

    let hidden = runtime.render(tree("Run", "bg-black")).unwrap();
    assert!(!hidden.presentation_requested);
    assert!(runtime.pending_redraw());
    assert_eq!(runtime.presenter().publish_count(), 1);

    let exposed = runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::OcclusionChanged {
                window: spec().id,
                occluded: false,
            },
        })
        .unwrap();
    let SelfDrawnHostEventOutcome::Frame(exposed) = exposed else {
        panic!("exposure should redraw the retained scene");
    };
    assert!(exposed.presentation_requested);
    assert!(!runtime.pending_redraw());
    assert_eq!(runtime.presenter().publish_count(), 2);
}

#[test]
fn surface_loss_replays_the_last_committed_scene_without_relayout() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    let before = runtime.stats();
    let revision = runtime.snapshot().unwrap().revision();

    let recovered = runtime
        .handle_event(PlatformHostEvent::Presentation {
            ack: PlatformPresentationAck {
                revision,
                window: spec().id,
                status: PlatformPresentationStatus::SurfaceLost,
            },
        })
        .unwrap();

    let SelfDrawnHostEventOutcome::Frame(recovered) = recovered else {
        panic!("surface loss should replay the retained scene");
    };
    assert!(recovered.presentation_requested);
    assert!(!recovered.layout_rebuilt);
    assert!(!recovered.scene_rebuilt);
    assert_eq!(runtime.stats().layout_builds, before.layout_builds);
    assert_eq!(runtime.stats().scene_builds, before.scene_builds);
    assert_eq!(runtime.presenter().surface_loss_count(), 1);
}

#[test]
fn delayed_surface_loss_after_a_semantic_commit_replays_the_latest_scene() {
    let mut runtime = runtime();
    let presented = runtime.render(tree("Run", "bg-white")).unwrap().revision;
    let semantic = runtime.render(tree("Launch", "bg-white")).unwrap();
    assert!(!semantic.presentation_requested);

    let recovered = runtime
        .handle_event(PlatformHostEvent::Presentation {
            ack: PlatformPresentationAck {
                revision: presented,
                window: spec().id,
                status: PlatformPresentationStatus::SurfaceLost,
            },
        })
        .unwrap();

    let SelfDrawnHostEventOutcome::Frame(recovered) = recovered else {
        panic!("the last presented revision should remain recoverable");
    };
    assert!(recovered.presentation_requested);
    assert_eq!(runtime.presenter().surface_loss_count(), 1);
    assert_eq!(
        runtime
            .snapshot()
            .unwrap()
            .accessibility()
            .root
            .as_ref()
            .unwrap()
            .children[0]
            .label
            .as_deref(),
        Some("Launch")
    );
}

#[test]
fn dropped_presentation_replays_without_invalidating_the_surface() {
    let mut runtime = runtime();
    let presented = runtime.render(tree("Run", "bg-white")).unwrap().revision;

    let replayed = runtime
        .handle_event(PlatformHostEvent::Presentation {
            ack: PlatformPresentationAck {
                revision: presented,
                window: spec().id,
                status: PlatformPresentationStatus::Dropped,
            },
        })
        .unwrap();

    let SelfDrawnHostEventOutcome::Frame(replayed) = replayed else {
        panic!("a dropped presentation should replay while visible");
    };
    assert!(replayed.presentation_requested);
    assert_eq!(runtime.presenter().surface_loss_count(), 0);
    assert_eq!(runtime.presenter().publish_count(), 2);
}

#[test]
fn invalid_candidate_does_not_replace_committed_state() {
    let mut runtime = runtime();
    runtime.render(tree("Run", "bg-white")).unwrap();
    let previous = runtime.snapshot().unwrap().clone();
    let invalid = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().style("width", "calc(100% - 2rem)")));

    assert!(runtime.render(invalid).is_err());

    assert_eq!(runtime.snapshot().unwrap().revision(), previous.revision());
    assert_eq!(runtime.presenter().publish_count(), 1);
    assert_eq!(runtime.host().committed().len(), 1);
}

#[test]
fn public_runtime_records_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<PlatformRenderFrame>();
    assert_send_sync::<SelfDrawnFrameSnapshot>();
    assert_send_sync::<SelfDrawnFrameCommit>();
    assert_send_sync::<SelfDrawnRuntimeStats>();
}

#[cfg(feature = "software-reference")]
#[test]
fn reference_presenter_publishes_graphics_pixels_only_after_host_commit() {
    let mut runtime = SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        ReferenceScenePresenter::new(),
        spec(),
        1.0,
    )
    .unwrap();

    runtime.render(tree("Run", "bg-white")).unwrap();

    let frame = runtime.presenter().committed().unwrap();
    assert_eq!((frame.width(), frame.height()), (100, 80));
    assert_eq!(frame.rgba8().len(), 100 * 80 * 4);
    assert_eq!(&frame.rgba8()[0..4], &[255, 255, 255, 255]);
    let black = ((12 * 100 + 10) * 4) as usize;
    assert_eq!(&frame.rgba8()[black..black + 4], &[0, 0, 0, 255]);
}
