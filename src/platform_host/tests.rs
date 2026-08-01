use crate::accessibility::AccessibilityRole;
use crate::geometry::{Rect, Size};
use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::native::ValueSensitivity;

use super::*;

fn window_id() -> PlatformWindowId {
    PlatformWindowId::new(7)
}

fn revision(value: u64) -> PlatformHostRevision {
    PlatformHostRevision::new(value)
}

fn window_spec() -> PlatformWindowSpec {
    PlatformWindowSpec {
        id: window_id(),
        title: "Calculator".to_string(),
        logical_size: Size::new(410.0, 620.0),
        min_size: Some(Size::new(320.0, 480.0)),
        max_size: Some(Size::new(820.0, 1240.0)),
        resizable: true,
        visible: true,
    }
}

fn presentation() -> PlatformPresentationRequest {
    PlatformPresentationRequest {
        window: window_id(),
        logical_size: Size::new(410.0, 620.0),
        scale_factor: 2.0,
        scene_fingerprint: 21_005_506_627_562_801,
        damage: vec![Rect::new(0.0, 0.0, 410.0, 620.0)],
    }
}

fn presentation_transaction(value: u64) -> PlatformHostTransaction {
    PlatformHostTransaction {
        revision: revision(value),
        commands: vec![PlatformHostCommand::Present {
            request: presentation(),
        }],
    }
}

fn element_id(value: &str) -> PlatformElementId {
    PlatformElementId::new(value).unwrap()
}

fn accessibility_node(
    id: &str,
    role: AccessibilityRole,
    logical_bounds: Rect,
) -> PlatformAccessibilityNode {
    PlatformAccessibilityNode::new(element_id(id), role, logical_bounds)
}

fn accessibility_snapshot() -> PlatformAccessibilitySnapshot {
    let root = accessibility_node(
        "4:root",
        AccessibilityRole::Window,
        Rect::new(0.0, 0.0, 410.0, 620.0),
    )
    .child(accessibility_node(
        "4:root/6:submit",
        AccessibilityRole::Button,
        Rect::new(20.0, 540.0, 90.0, 48.0),
    ));
    PlatformAccessibilitySnapshot {
        window: window_id(),
        root: Some(root),
    }
}

fn text_state() -> PlatformTextInputState {
    PlatformTextInputState {
        session: PlatformTextInputSessionId::new(9),
        window: window_id(),
        purpose: PlatformTextInputPurpose::Text,
        surrounding_text: Some("h\u{00e9}".to_string()),
        selection: PlatformTextRange::new(1, 3),
        composition: Some(PlatformTextRange::new(1, 3)),
        candidate_rect: Rect::new(12.0, 18.0, 1.0, 20.0),
    }
}

#[test]
fn public_platform_host_records_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<PlatformHostRevision>();
    assert_send_sync::<PlatformHostTransaction>();
    assert_send_sync::<PlatformHostCommand>();
    assert_send_sync::<PlatformHostEvent>();
    assert_send_sync::<PlatformWindowSpec>();
    assert_send_sync::<PlatformPresentationRequest>();
    assert_send_sync::<PlatformInputEvent>();
    assert_send_sync::<PlatformTextInputState>();
    assert_send_sync::<PlatformElementId>();
    assert_send_sync::<PlatformAccessibilityNode>();
    assert_send_sync::<PlatformAccessibilitySnapshot>();
    assert_send_sync::<PlatformSystemRequest>();
    assert_send_sync::<RecordingPlatformHost>();
}

#[test]
fn window_and_presentation_records_reject_invalid_geometry() {
    assert!(window_spec().validate().is_ok());
    assert!(presentation().validate().is_ok());

    let mut invalid_window = window_spec();
    invalid_window.min_size = Some(Size::new(500.0, 700.0));
    assert!(invalid_window
        .validate()
        .unwrap_err()
        .to_string()
        .contains("minimum size cannot exceed"));

    let mut invalid_presentation = presentation();
    invalid_presentation.scale_factor = f64::NAN;
    assert!(invalid_presentation
        .validate()
        .unwrap_err()
        .to_string()
        .contains("scale factor"));

    invalid_presentation = presentation();
    invalid_presentation.damage[0].width = -1.0;
    assert!(invalid_presentation
        .validate()
        .unwrap_err()
        .to_string()
        .contains("non-negative"));
}

#[test]
fn raw_input_records_are_un_targeted_and_validated() {
    let pointer = PlatformPointerEvent {
        window: window_id(),
        device: PlatformInputDeviceId::new(3),
        pointer: PlatformPointerId::new(4),
        modality: NativeInputModality::Pen,
        phase: PlatformPointerPhase::Pressed,
        position: PlatformPoint::new(12.5, 40.0),
        button: Some(PlatformPointerButton::Primary),
        pressed_buttons: 1,
        pressure: Some(0.75),
        modifiers: NativeKeyModifiers::new().shift(true),
        timestamp_micros: 88,
    };
    assert!(pointer.validate().is_ok());

    let mut invalid = pointer.clone();
    invalid.modality = NativeInputModality::Keyboard;
    assert!(invalid
        .validate()
        .unwrap_err()
        .to_string()
        .contains("pointer modality"));

    invalid = pointer;
    invalid.button = None;
    assert!(invalid
        .validate()
        .unwrap_err()
        .to_string()
        .contains("need a button"));
}

#[test]
fn key_debug_output_does_not_retain_committed_text() {
    let event = PlatformKeyEvent {
        window: window_id(),
        device: PlatformInputDeviceId::new(3),
        physical_key: "KeyA".to_string(),
        logical_key: "a".to_string(),
        text: Some("private-text".to_string()),
        state: PlatformKeyState::Pressed,
        repeat: false,
        modifiers: NativeKeyModifiers::new(),
        timestamp_micros: 89,
    };

    assert!(event.validate().is_ok());
    let debug = format!("{event:?}");
    assert!(!debug.contains("private-text"));
    assert!(debug.contains("has_text"));
}

#[test]
fn text_input_enforces_utf8_ranges_and_password_redaction() {
    assert!(text_state().validate().is_ok());

    let mut invalid_range = text_state();
    invalid_range.selection = PlatformTextRange::new(2, 3);
    assert!(invalid_range
        .validate()
        .unwrap_err()
        .to_string()
        .contains("UTF-8 boundaries"));

    let password = PlatformTextInputState {
        purpose: PlatformTextInputPurpose::Password,
        surrounding_text: Some("secret".to_string()),
        composition: None,
        selection: PlatformTextRange::new(6, 6),
        ..text_state()
    };
    assert!(password
        .validate()
        .unwrap_err()
        .to_string()
        .contains("password sessions"));

    let safe_password = PlatformTextInputState {
        surrounding_text: None,
        ..password
    };
    assert!(safe_password.validate().is_ok());

    let event = PlatformTextInputEvent::Commit {
        session: PlatformTextInputSessionId::new(9),
        text: "secret".to_string(),
    };
    assert!(!format!("{event:?}").contains("secret"));
}

#[test]
fn accessibility_snapshots_require_stable_ids_valid_bounds_and_no_secrets() {
    let snapshot = accessibility_snapshot();
    assert!(snapshot.validate().is_ok());

    let mut duplicate_id = snapshot.clone();
    duplicate_id.root.as_mut().unwrap().children[0].id = element_id("4:root");
    assert!(duplicate_id
        .validate()
        .unwrap_err()
        .to_string()
        .contains("duplicate element id"));

    let mut invalid_bounds = snapshot.clone();
    invalid_bounds.root.as_mut().unwrap().logical_bounds.width = -1.0;
    assert!(invalid_bounds
        .validate()
        .unwrap_err()
        .to_string()
        .contains("non-negative"));

    let mut sensitive = snapshot;
    let root = sensitive.root.as_mut().unwrap();
    root.value_sensitivity = ValueSensitivity::Sensitive;
    root.value = Some("password".to_string());
    assert!(sensitive
        .validate()
        .unwrap_err()
        .to_string()
        .contains("sensitive values"));
}

#[test]
fn system_requests_are_typed_bounded_and_redaction_aware() {
    let request = PlatformSystemRequest {
        id: PlatformSystemRequestId::new(1),
        window: Some(window_id()),
        command: PlatformSystemCommand::WriteClipboard {
            content: PlatformClipboardContent {
                format: PlatformClipboardFormat::Text,
                text: "secret".to_string(),
                sensitivity: ValueSensitivity::Sensitive,
            },
        },
    };
    assert!(request.validate().is_ok());
    assert!(!format!("{request:?}").contains("secret"));

    let redacted = request.redacted_for_diagnostics();
    let encoded = serde_json::to_string(&redacted).unwrap();
    assert!(!encoded.contains("secret"));
    assert!(encoded.contains("******"));

    let invalid_url = PlatformSystemRequest {
        id: PlatformSystemRequestId::new(2),
        window: None,
        command: PlatformSystemCommand::OpenUrl {
            url: "not a url".to_string(),
        },
    };
    assert!(invalid_url
        .validate()
        .unwrap_err()
        .to_string()
        .contains("need a scheme"));
}

#[test]
fn transactions_reject_ambiguous_or_duplicate_commands() {
    let valid = PlatformHostTransaction {
        revision: revision(1),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Open {
                    spec: window_spec(),
                },
            },
            PlatformHostCommand::Present {
                request: presentation(),
            },
            PlatformHostCommand::Accessibility {
                snapshot: Box::new(accessibility_snapshot()),
            },
        ],
    };
    assert!(valid.validate().is_ok());

    let empty = PlatformHostTransaction {
        revision: revision(1),
        commands: Vec::new(),
    };
    assert!(empty
        .validate()
        .unwrap_err()
        .to_string()
        .contains("at least one command"));

    let duplicate_presentation = PlatformHostTransaction {
        revision: revision(1),
        commands: vec![
            PlatformHostCommand::Present {
                request: presentation(),
            },
            PlatformHostCommand::Present {
                request: presentation(),
            },
        ],
    };
    assert!(duplicate_presentation
        .validate()
        .unwrap_err()
        .to_string()
        .contains("multiple presentations"));
}

#[test]
fn recording_host_commits_monotonic_revisions_and_bounded_history() {
    let mut host = RecordingPlatformHost::with_limits(2, 2);

    host.prepare(presentation_transaction(1)).unwrap();
    let ack = host.commit().unwrap();
    assert_eq!(ack.revision, revision(1));
    assert_eq!(ack.applied_commands, 1);
    assert_eq!(ack.presentations.len(), 1);
    assert_eq!(
        ack.presentations[0].status,
        PlatformPresentationStatus::Queued
    );

    host.prepare(presentation_transaction(2)).unwrap();
    host.commit().unwrap();
    host.prepare(presentation_transaction(3)).unwrap();
    host.commit().unwrap();

    assert_eq!(host.committed().len(), 2);
    assert_eq!(host.committed()[0].revision, revision(2));
    assert_eq!(host.last_committed_revision(), Some(revision(3)));
    assert!(host
        .prepare(presentation_transaction(3))
        .unwrap_err()
        .to_string()
        .contains("must be newer"));
}

#[test]
fn failed_commits_require_explicit_rollback_and_preserve_last_revision() {
    let mut host = RecordingPlatformHost::new();
    host.prepare(presentation_transaction(1)).unwrap();
    host.fail_next_commit("surface unavailable");

    assert!(host
        .commit()
        .unwrap_err()
        .to_string()
        .contains("surface unavailable"));
    assert_eq!(host.pending().unwrap().revision, revision(1));
    assert_eq!(host.last_committed_revision(), None);
    assert!(host.shutdown().is_err());

    host.rollback().unwrap();
    host.prepare(presentation_transaction(1)).unwrap();
    host.commit().unwrap();
    assert_eq!(host.last_committed_revision(), Some(revision(1)));
}

#[test]
fn recording_history_redacts_text_and_sensitive_clipboard_values() {
    let transaction = PlatformHostTransaction {
        revision: revision(1),
        commands: vec![
            PlatformHostCommand::TextInput {
                update: PlatformTextInputUpdate::Activate {
                    state: text_state(),
                },
            },
            PlatformHostCommand::System {
                request: PlatformSystemRequest {
                    id: PlatformSystemRequestId::new(1),
                    window: Some(window_id()),
                    command: PlatformSystemCommand::WriteClipboard {
                        content: PlatformClipboardContent {
                            format: PlatformClipboardFormat::Text,
                            text: "secret".to_string(),
                            sensitivity: ValueSensitivity::Sensitive,
                        },
                    },
                },
            },
        ],
    };
    let mut host = RecordingPlatformHost::new();
    host.prepare(transaction).unwrap();
    host.commit().unwrap();

    let history = serde_json::to_string(host.committed()).unwrap();
    assert!(!history.contains("h\u{00e9}"));
    assert!(!history.contains("secret"));
    assert!(host.committed()[0].validate().is_ok());
}

#[test]
fn event_queue_is_ordered_bounded_and_cleared_on_shutdown() {
    let mut host = RecordingPlatformHost::with_limits(1, 1);
    let event = PlatformHostEvent::Window {
        event: PlatformWindowEvent::RedrawRequested {
            window: window_id(),
        },
    };
    host.queue_event(event.clone()).unwrap();
    assert!(host.queue_event(event.clone()).is_err());
    assert_eq!(host.poll_event().unwrap(), Some(event));
    assert_eq!(host.poll_event().unwrap(), None);

    host.shutdown().unwrap();
    assert!(host.is_shutdown());
    assert!(host.poll_event().is_err());
    host.shutdown().unwrap();
}

#[test]
fn transaction_wire_shape_is_stable_and_contains_no_widget_commands() {
    let transaction = PlatformHostTransaction {
        revision: revision(1),
        commands: vec![PlatformHostCommand::Window {
            command: PlatformWindowCommand::RequestRedraw {
                window: window_id(),
            },
        }],
    };

    assert_eq!(
        serde_json::to_value(transaction).unwrap(),
        serde_json::json!({
            "revision": 1,
            "commands": [{
                "kind": "window",
                "command": {"kind": "requestRedraw", "window": 7}
            }]
        })
    );
}
