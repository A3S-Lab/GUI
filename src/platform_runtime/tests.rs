use crate::accessibility::AccessibilityRole;
use crate::geometry::{Rect, Size};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{
    PlatformHostEvent, PlatformInputDeviceId, PlatformInputEvent, PlatformKeyEvent,
    PlatformKeyState, PlatformPointerButton, PlatformPointerEvent, PlatformPointerId,
    PlatformPointerPhase, PlatformPresentationAck, PlatformPresentationStatus, PlatformWindowEvent,
    PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost,
};
use crate::web::WebProps;
use crate::{GuiError, NativeEventKind, NativeInputModality, NativeKeyModifiers, PlatformPoint};

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

pub(super) fn runtime() -> SelfDrawnWindowRuntime<RecordingPlatformHost, RecordingScenePresenter> {
    SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        RecordingScenePresenter::new(),
        spec(),
        1.0,
    )
    .unwrap()
}

fn interaction_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new().web(
                WebProps::new()
                    .class_name("relative h-[80px] w-[100px] bg-white")
                    .on_press("rootPress")
                    .on_focus_within_change("rootFocusWithin"),
            ),
        )
        .child(
            NativeElement::new("first", NativeRole::Button)
                .with_props(
                    NativeProps::new()
                        .label("First")
                        .metadata("actionValue", "first-value")
                        .web(
                            WebProps::new()
                                .class_name(
                                    "absolute left-[10px] top-[10px] h-[20px] w-[35px] bg-black",
                                )
                                .on_press_start("firstStart")
                                .on_press_end("firstEnd")
                                .on_press_up("firstUp")
                                .on_press_change("firstPressed")
                                .on_press("firstPress")
                                .on_focus("firstFocus")
                                .on_blur("firstBlur")
                                .on_key_down("firstKeyDown")
                                .on_key_up("firstKeyUp"),
                        ),
                )
                .child(NativeElement::text("label", "First")),
        )
        .child(
            NativeElement::new("second", NativeRole::Button).with_props(
                NativeProps::new().label("Second").web(
                    WebProps::new()
                        .class_name("absolute left-[55px] top-[10px] h-[20px] w-[35px] bg-black")
                        .on_press("secondPress")
                        .on_focus("secondFocus")
                        .on_blur("secondBlur"),
                ),
            ),
        )
}

fn disabled_overlay_tree() -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("back", NativeRole::Button).with_props(
                NativeProps::new().label("Back").web(
                    WebProps::new()
                        .class_name("absolute left-[10px] top-[10px] h-[30px] w-[50px]")
                        .on_press("backPress"),
                ),
            ),
        )
        .child(
            NativeElement::new("front", NativeRole::Button).with_props(
                NativeProps::new().label("Front").disabled(true).web(
                    WebProps::new()
                        .class_name("absolute z-10 left-[10px] top-[10px] h-[30px] w-[50px]")
                        .on_press("frontPress"),
                ),
            ),
        )
}

pub(super) fn pointer_event(
    phase: PlatformPointerPhase,
    x: f64,
    y: f64,
    timestamp_micros: u64,
) -> PlatformHostEvent {
    pointer_event_for(PlatformPointerId::new(1), phase, x, y, timestamp_micros)
}

pub(super) fn pointer_event_for(
    pointer: PlatformPointerId,
    phase: PlatformPointerPhase,
    x: f64,
    y: f64,
    timestamp_micros: u64,
) -> PlatformHostEvent {
    let button = matches!(
        phase,
        PlatformPointerPhase::Pressed | PlatformPointerPhase::Released
    )
    .then_some(PlatformPointerButton::Primary);
    PlatformHostEvent::Input {
        event: PlatformInputEvent::Pointer {
            event: PlatformPointerEvent {
                window: spec().id,
                device: PlatformInputDeviceId::new(1),
                pointer,
                modality: NativeInputModality::Mouse,
                phase,
                position: PlatformPoint::new(x, y),
                button,
                pressed_buttons: u32::from(phase == PlatformPointerPhase::Pressed),
                pressure: None,
                modifiers: NativeKeyModifiers::new(),
                timestamp_micros,
            },
        },
    }
}

pub(super) fn key_event(
    key: &str,
    state: PlatformKeyState,
    timestamp_micros: u64,
) -> PlatformHostEvent {
    PlatformHostEvent::Input {
        event: PlatformInputEvent::Key {
            event: PlatformKeyEvent {
                window: spec().id,
                device: PlatformInputDeviceId::new(2),
                physical_key: key.to_string(),
                logical_key: key.to_string(),
                text: None,
                state,
                repeat: false,
                modifiers: NativeKeyModifiers::new(),
                timestamp_micros,
            },
        },
    }
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
    assert_eq!(
        commit.presentation_status,
        Some(PlatformPresentationStatus::Presented)
    );
}

#[test]
fn raw_pointer_routes_stable_press_lifecycle_and_bubbling_without_a_widget_plan() {
    let mut runtime = runtime();
    let revision = runtime.render(interaction_tree()).unwrap().revision;

    let pressed = runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 14.0, 14.0, 10))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(pressed) = pressed else {
        panic!("pointer input should be routed by the self-drawn runtime");
    };
    assert_eq!(pressed.frame_revision, revision);
    assert_eq!(pressed.event_sequence, 1);
    assert_eq!(pressed.target.as_ref().unwrap().as_str(), "4:root/5:first");
    assert_eq!(
        pressed
            .invocations
            .iter()
            .map(|invocation| (
                invocation.action.as_str(),
                invocation.event,
                invocation.value()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("firstFocus", NativeEventKind::Focus, Some("true")),
            ("rootFocusWithin", NativeEventKind::Focus, Some("true")),
            ("firstStart", NativeEventKind::PressStart, Some("true")),
            ("firstPressed", NativeEventKind::PressStart, Some("true")),
        ]
    );
    assert_eq!(
        runtime.focused_element().map(|id| id.as_str()),
        Some("4:root/5:first")
    );
    assert!(
        runtime
            .element_interaction(pressed.target.as_ref().unwrap())
            .unwrap()
            .pressed
    );

    let released = runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Released,
            14.0,
            14.0,
            20,
        ))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(released) = released else {
        panic!("pointer release should be routed");
    };
    assert_eq!(released.frame_revision, revision);
    assert_eq!(released.event_sequence, 2);
    assert_eq!(
        released
            .invocations
            .iter()
            .map(|invocation| (
                invocation.action.as_str(),
                invocation.event,
                invocation.value()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("firstUp", NativeEventKind::PressUp, Some("first-value")),
            ("firstEnd", NativeEventKind::PressEnd, Some("false")),
            ("firstPressed", NativeEventKind::PressEnd, Some("false")),
            ("firstPress", NativeEventKind::Press, Some("first-value")),
            ("rootPress", NativeEventKind::Press, None),
        ]
    );
    assert_eq!(
        released
            .invocations
            .last()
            .unwrap()
            .current_target()
            .as_str(),
        "4:root"
    );
    assert!(
        !runtime
            .element_interaction(released.target.as_ref().unwrap())
            .unwrap()
            .pressed
    );
}

#[test]
fn keyboard_tab_focus_and_activation_use_the_same_stable_action_route() {
    let mut runtime = runtime();
    runtime.render(interaction_tree()).unwrap();

    let tab = runtime
        .handle_event(key_event("Tab", PlatformKeyState::Pressed, 10))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(tab) = tab else {
        panic!("tab should be routed");
    };
    assert_eq!(tab.event_sequence, 1);
    assert_eq!(
        runtime.focused_element().map(|id| id.as_str()),
        Some("4:root/5:first")
    );

    let down = runtime
        .handle_event(key_event("Space", PlatformKeyState::Pressed, 20))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(down) = down else {
        panic!("space key down should be routed");
    };
    assert_eq!(
        down.invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.event))
            .collect::<Vec<_>>(),
        vec![
            ("firstStart", NativeEventKind::PressStart),
            ("firstPressed", NativeEventKind::PressStart),
            ("firstKeyDown", NativeEventKind::KeyDown),
        ]
    );

    let up = runtime
        .handle_event(key_event("Space", PlatformKeyState::Released, 30))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(up) = up else {
        panic!("space key up should be routed");
    };
    assert_eq!(
        up.invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.event))
            .collect::<Vec<_>>(),
        vec![
            ("firstUp", NativeEventKind::PressUp),
            ("firstEnd", NativeEventKind::PressEnd),
            ("firstPressed", NativeEventKind::PressEnd),
            ("firstPress", NativeEventKind::Press),
            ("rootPress", NativeEventKind::Press),
            ("firstKeyUp", NativeEventKind::KeyUp),
        ]
    );
}

#[test]
fn reducer_failure_rolls_back_the_staged_portable_interaction() {
    let mut runtime = runtime();
    runtime.render(interaction_tree()).unwrap();

    let error = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Pressed, 14.0, 14.0, 10),
            |_| Err(GuiError::host("injected reducer failure")),
        )
        .unwrap_err();

    assert!(error.to_string().contains("injected reducer failure"));
    assert!(runtime.focused_element().is_none());
    assert_eq!(runtime.event_sequence(), 0);

    let retried = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Pressed, 14.0, 14.0, 20),
            |_| Ok(SelfDrawnActionPropagation::Continue),
        )
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(retried) = retried else {
        panic!("retry should route input");
    };
    assert_eq!(retried.event_sequence, 1);
}

#[test]
fn disabled_topmost_hit_region_blocks_actions_instead_of_clicking_through() {
    let mut runtime = runtime();
    runtime.render(disabled_overlay_tree()).unwrap();

    let outcome = runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("disabled hit should still consume the raw input sequence");
    };

    assert!(dispatch.target.is_none());
    assert!(dispatch.invocations.is_empty());
    assert!(runtime.focused_element().is_none());
}

#[test]
fn reducer_stop_prevents_later_ancestor_callbacks_without_dropping_diagnostics() {
    let mut runtime = runtime();
    runtime.render(interaction_tree()).unwrap();
    runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Pressed, 14.0, 14.0, 10),
            |_| Ok(SelfDrawnActionPropagation::Continue),
        )
        .unwrap();
    let mut reduced = Vec::new();

    let outcome = runtime
        .handle_event_with_reducer(
            pointer_event(PlatformPointerPhase::Released, 14.0, 14.0, 20),
            |invocation| {
                reduced.push(invocation.action.clone());
                Ok(if invocation.action == "firstPress" {
                    SelfDrawnActionPropagation::Stop
                } else {
                    SelfDrawnActionPropagation::Continue
                })
            },
        )
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("release should route an action batch");
    };

    assert_eq!(
        reduced,
        ["firstUp", "firstEnd", "firstPressed", "firstPress"]
    );
    assert_eq!(dispatch.invocations.last().unwrap().action, "rootPress");
    assert_eq!(
        dispatch
            .propagation_stopped_at
            .as_ref()
            .map(|id| id.as_str()),
        Some("4:root/5:first")
    );
}

#[test]
fn successful_frame_reconciliation_preserves_stable_focus_and_rejected_frames_do_not_touch_it() {
    let mut runtime = runtime();
    runtime.render(interaction_tree()).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 14.0, 14.0, 10))
        .unwrap();
    let focused = runtime.focused_element().cloned().unwrap();
    runtime.host_mut().fail_next_commit("injected");

    assert!(runtime.render(tree("Changed", "bg-black")).is_err());
    assert_eq!(runtime.focused_element(), Some(&focused));

    runtime.render(interaction_tree()).unwrap();
    assert_eq!(runtime.focused_element(), Some(&focused));
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
fn close_request_routes_the_window_root_action_without_a_content_widget() {
    let mut runtime = runtime();
    let window = NativeElement::new("window", NativeRole::Window).with_props(
        NativeProps::new().web(
            WebProps::new()
                .class_name("h-[80px] w-[100px] bg-white")
                .event("onClose", "closeWindow"),
        ),
    );
    runtime.render(window).unwrap();

    let outcome = runtime
        .handle_event(PlatformHostEvent::Window {
            event: PlatformWindowEvent::CloseRequested { window: spec().id },
        })
        .unwrap();

    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("window close should route through the portable action tree")
    };
    assert_eq!(dispatch.event_sequence, 1);
    assert_eq!(dispatch.target.as_ref().unwrap().as_str(), "6:window");
    assert_eq!(dispatch.invocations.len(), 1);
    assert_eq!(dispatch.invocations[0].action, "closeWindow");
    assert_eq!(dispatch.invocations[0].event, NativeEventKind::Close);
    assert_eq!(
        dispatch.invocations[0].context.modality,
        NativeInputModality::Virtual
    );
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
    assert!(runtime.retry_pending_redraw().unwrap().is_none());
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
    assert_send_sync::<SelfDrawnEventContext>();
    assert_send_sync::<SelfDrawnDragContext>();
    assert_send_sync::<SelfDrawnDropItem>();
    assert_send_sync::<SelfDrawnDropOperation>();
    assert_send_sync::<SelfDrawnActionInvocation>();
    assert_send_sync::<SelfDrawnElementInteraction>();
    assert_send_sync::<SelfDrawnInteractionChange>();
    assert_send_sync::<SelfDrawnInputDispatch>();
}

#[test]
fn interaction_wire_defaults_new_transient_state_fields() {
    let state: SelfDrawnElementInteraction = serde_json::from_str("{}").unwrap();
    assert_eq!(state, SelfDrawnElementInteraction::default());
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
