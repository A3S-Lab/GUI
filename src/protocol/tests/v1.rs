use super::*;
use crate::overlay_position::OverlayPlacement;

fn render_request(
    session: &NativeProtocolSession<Gtk4Adapter>,
    revision: u64,
    frame: &UiFrame,
) -> ProtocolRenderRequestV1 {
    ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::render(session.session_id(), revision),
        ProtocolUiFrameV1::try_from(frame).unwrap(),
    )
}

fn render_ack(
    session: &NativeProtocolSession<Gtk4Adapter>,
    revision: u64,
    command_sequence: u64,
) -> ProtocolRenderAckV1 {
    ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::render(session.session_id(), revision),
        ProtocolCommandAckV1 { command_sequence },
    )
}

fn host_event(
    session: &NativeProtocolSession<Gtk4Adapter>,
    revision: u64,
    event_sequence: u64,
    frame_id: &str,
    node: u64,
    kind: NativeEventKind,
    value: Option<&str>,
) -> ProtocolHostEventV1 {
    ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::event(session.session_id(), revision, event_sequence),
        ProtocolHostEventPayloadV1 {
            frame_id: frame_id.to_string(),
            event: ProtocolNativeEventV1 {
                node,
                kind: kind.into(),
                value: value.map(ToOwned::to_owned),
            },
        },
    )
}

#[test]
fn protocol_v1_host_event_has_a_stable_golden_wire_shape() {
    let request = ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::event("golden-session", 7, 9),
        ProtocolHostEventPayloadV1 {
            frame_id: "frame-7".to_string(),
            event: ProtocolNativeEventV1 {
                node: 42,
                kind: NativeEventKind::Change.into(),
                value: Some("public-value".to_string()),
            },
        },
    );

    assert_eq!(
        serde_json::to_string(&request).unwrap(),
        r#"{"metadata":{"protocolVersion":1,"sessionId":"golden-session","renderRevision":7,"eventSequence":9},"payload":{"frameId":"frame-7","event":{"node":42,"kind":"change","value":"public-value"}}}"#
    );
    assert_eq!(
        serde_json::to_value(ProtocolAccessibilityDescriptionPropsV1 {
            description: Some("Help".to_string()),
            ..Default::default()
        })
        .unwrap(),
        serde_json::json!({
            "description": "Help",
            "roleDescription": null,
            "keyShortcuts": null,
            "valueText": null
        })
    );
    let render_response = ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::render("golden-session", 7),
        ProtocolRenderPayloadV1 {
            frame_id: "frame-7".to_string(),
            root: 42,
            command_sequence: 11,
            commands: vec![ProtocolCommandV1::SetRoot { id: 42 }],
            accessibility_tree: None,
        },
    );
    assert_eq!(
        serde_json::to_string(&render_response).unwrap(),
        r#"{"metadata":{"protocolVersion":1,"sessionId":"golden-session","renderRevision":7},"payload":{"frameId":"frame-7","root":42,"commandSequence":11,"commands":[{"type":"setRoot","id":42}]}}"#
    );
    assert_eq!(
        serde_json::to_value(ProtocolNativeWidgetKindV1::TextInput(
            ProtocolNativeTextInputKindV1::Password
        ))
        .unwrap(),
        serde_json::json!({"textInput": "password"})
    );

    let unknown_metadata = r#"{
        "metadata": {
            "protocolVersion": 1,
            "sessionId": "golden-session",
            "renderRevision": 7,
            "eventSequence": 9,
            "legacyFrameId": "bypass"
        },
        "payload": {
            "frameId": "frame-7",
            "event": {"node": 42, "kind": "change"}
        }
    }"#;
    assert!(serde_json::from_str::<ProtocolHostEventV1>(unknown_metadata).is_err());

    let unknown_payload = r#"{
        "metadata": {
            "protocolVersion": 1,
            "sessionId": "golden-session",
            "renderRevision": 7,
            "eventSequence": 9
        },
        "payload": {
            "frameId": "frame-7",
            "event": {"node": 42, "kind": "change", "legacyValue": "bypass"}
        }
    }"#;
    assert!(serde_json::from_str::<ProtocolHostEventV1>(unknown_payload).is_err());
}

#[test]
fn protocol_v1_round_trips_typed_overlay_position_commands() {
    let command = PlatformCommand::PositionOverlay {
        overlay: HostNodeId::new(7),
        anchor: HostNodeId::new(3),
        request: OverlayPositionRequest::new(
            OverlayPositionOptions {
                placement: OverlayPlacement::BottomStart,
                offset: 8.0,
                cross_offset: -2.0,
                should_flip: false,
                should_update_position: true,
                container_padding: 16.0,
                arrow_size: 6.0,
                arrow_boundary_offset: 4.0,
                max_height: Some(320.0),
            },
            TextDirection::Rtl,
        )
        .unwrap(),
    };

    let protocol = ProtocolCommandV1::from(&command);
    assert_eq!(
        serde_json::to_value(&protocol).unwrap(),
        serde_json::json!({
            "type": "positionOverlay",
            "overlay": 7,
            "anchor": 3,
            "request": {
                "placement": "bottom start",
                "offset": 8.0,
                "crossOffset": -2.0,
                "shouldFlip": false,
                "shouldUpdatePosition": true,
                "containerPadding": 16.0,
                "arrowSize": 6.0,
                "arrowBoundaryOffset": 4.0,
                "maxHeight": 320.0,
                "direction": "rtl"
            }
        })
    );
    assert_eq!(PlatformCommand::try_from(protocol).unwrap(), command);
}

#[test]
fn protocol_v1_round_trips_accessibility_announcement_commands() {
    let command = PlatformCommand::AccessibilityAnnouncement {
        announcement: AccessibilityAnnouncement::assertive(HostNodeId::new(42), "\u{2212}1.5"),
    };

    let protocol = ProtocolCommandV1::from(&command);
    assert_eq!(
        serde_json::to_value(&protocol).unwrap(),
        serde_json::json!({
            "type": "accessibilityAnnouncement",
            "node": 42,
            "message": "\u{2212}1.5",
            "priority": "assertive"
        })
    );
    assert_eq!(PlatformCommand::try_from(protocol).unwrap(), command);

    assert!(
        PlatformCommand::try_from(ProtocolCommandV1::AccessibilityAnnouncement {
            node: 42,
            message: " ".to_string(),
            priority: AccessibilityAnnouncementPriority::Polite,
        })
        .is_err()
    );
}

#[test]
fn protocol_v1_keeps_accessible_names_independent_from_visible_labels() {
    let frame: UiFrame = serde_json::from_value(serde_json::json!({
        "frameId": "accessible-name",
        "root": {
            "kind": "element",
            "key": "save",
            "tag": "Button",
            "props": {
                "attributes": {"aria-label": "Save document"}
            },
            "children": [
                {"kind": "text", "key": "save-label", "value": "Save"}
            ]
        }
    }))
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);
    let request = render_request(&session, 1, &frame);
    let response = session.render_v1(&request).unwrap();
    let command = response
        .payload
        .commands
        .iter()
        .find(|command| matches!(command, ProtocolCommandV1::Create { .. }))
        .unwrap();
    let ProtocolCommandV1::Create { blueprint, .. } = command else {
        unreachable!();
    };

    assert_eq!(blueprint.label.as_deref(), Some("Save"));
    assert_eq!(
        blueprint.accessibility_label.as_deref(),
        Some("Save document")
    );
    assert_eq!(
        serde_json::to_value(command).unwrap()["blueprint"]["accessibilityLabel"],
        "Save document"
    );

    let platform = PlatformCommand::try_from(command.clone()).unwrap();
    let PlatformCommand::Create { blueprint, .. } = platform else {
        unreachable!();
    };
    assert_eq!(blueprint.label.as_deref(), Some("Save"));
    assert_eq!(
        blueprint.accessibility_label.as_deref(),
        Some("Save document")
    );
}

#[test]
fn protocol_v1_retains_and_resends_render_until_exact_ack() {
    let mut session =
        NativeProtocolSession::new_with_session_id(Gtk4Adapter, "delivery-session").unwrap();
    let frame = counter_frame(&CounterState::default()).unwrap();
    let request = render_request(&session, 1, &frame);

    let first = session.render_v1(&request).unwrap();

    assert_eq!(session.mode(), ProtocolSessionMode::StrictV1);
    assert_eq!(first.metadata.render_revision, 1);
    assert_eq!(first.payload.command_sequence, 1);
    assert_eq!(session.pending_render_v1(), Some(&first));
    assert_eq!(session.pending_command_ack(), Some((1, 1)));
    assert!(session.runtime().host().commands().is_empty());

    let resent = session.render_v1(&request).unwrap();
    assert_eq!(resent, first);
    assert_eq!(session.render_revision(), 1);
    assert_eq!(session.pending_render_v1(), Some(&first));

    let mut conflicting = request.clone();
    conflicting.payload.frame_id = "different-frame".to_string();
    let conflict_retry = session.render_v1(&conflicting).unwrap();
    assert_eq!(conflict_retry, first);
    assert_eq!(session.active_frame_id(), Some("counter"));

    let next_frame = counter_frame(&CounterState { count: 1 }).unwrap();
    let next_request = render_request(&session, 2, &next_frame);
    let blocked = session.render_v1(&next_request).unwrap_err();
    assert!(blocked.to_string().contains("awaits acknowledgement"));

    let wrong_ack = session
        .acknowledge_render_v1(&render_ack(&session, 1, 2))
        .unwrap_err();
    assert!(wrong_ack
        .to_string()
        .contains("does not match pending sequence 1"));
    assert_eq!(session.pending_render_v1(), Some(&first));

    let wrong_revision = session
        .acknowledge_render_v1(&render_ack(&session, 2, 1))
        .unwrap_err();
    assert!(wrong_revision
        .to_string()
        .contains("does not match pending revision 1"));
    assert_eq!(session.pending_render_v1(), Some(&first));

    session
        .acknowledge_render_v1(&render_ack(&session, 1, 1))
        .unwrap();
    assert!(session.pending_render_v1().is_none());

    let duplicate_ack = session
        .acknowledge_render_v1(&render_ack(&session, 1, 1))
        .unwrap_err();
    assert!(duplicate_ack
        .to_string()
        .contains("no version-1 render command batch"));

    let second = session.render_v1(&next_request).unwrap();
    assert_eq!(second.metadata.render_revision, 2);
    assert_eq!(second.payload.command_sequence, 2);
    session
        .acknowledge_render_v1(&render_ack(&session, 2, 2))
        .unwrap();
}

#[test]
fn protocol_v1_rejects_wrong_identity_revision_and_event_order() {
    let mut session =
        NativeProtocolSession::new_with_session_id(Gtk4Adapter, "ordered-session").unwrap();
    let frame = counter_frame(&CounterState::default()).unwrap();
    let valid_render = render_request(&session, 1, &frame);

    let mut wrong_version = valid_render.clone();
    wrong_version.metadata.protocol_version = 2;
    assert!(session.render_v1(&wrong_version).is_err());
    assert_eq!(session.mode(), ProtocolSessionMode::Unbound);

    let mut wrong_session = valid_render.clone();
    wrong_session.metadata.session_id = "other-session".to_string();
    assert!(session.render_v1(&wrong_session).is_err());
    assert_eq!(session.mode(), ProtocolSessionMode::Unbound);

    let mut wrong_revision = valid_render.clone();
    wrong_revision.metadata.render_revision = 2;
    assert!(session.render_v1(&wrong_revision).is_err());
    assert_eq!(session.mode(), ProtocolSessionMode::StrictV1);
    assert_eq!(session.render_revision(), 0);
    assert!(session.root().is_none());

    let rendered = session.render_v1(&valid_render).unwrap();
    session
        .acknowledge_render_v1(&render_ack(&session, 1, 1))
        .unwrap();

    let mut missing_sequence = host_event(
        &session,
        1,
        1,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    missing_sequence.metadata.event_sequence = None;
    assert!(session.handle_host_event_v1(&missing_sequence).is_err());

    let out_of_order = host_event(
        &session,
        1,
        2,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    assert!(session.handle_host_event_v1(&out_of_order).is_err());

    let stale = host_event(
        &session,
        0,
        1,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    assert!(session.handle_host_event_v1(&stale).is_err());
    assert_eq!(session.last_event_sequence(), 0);

    let first_event = host_event(
        &session,
        1,
        1,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    let first_response = session.handle_host_event_v1(&first_event).unwrap();
    assert_eq!(first_response.metadata.event_sequence, Some(1));
    assert_eq!(session.last_event_sequence(), 1);

    let duplicate = session.handle_host_event_v1(&first_event).unwrap_err();
    assert!(duplicate.to_string().contains("expected 2"));

    let skipped = host_event(
        &session,
        1,
        3,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    assert!(session.handle_host_event_v1(&skipped).is_err());

    let wrong_frame = host_event(
        &session,
        1,
        2,
        "stale-frame",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    assert!(session.handle_host_event_v1(&wrong_frame).is_err());
    assert_eq!(session.last_event_sequence(), 1);

    let second_event = host_event(
        &session,
        1,
        2,
        "counter",
        rendered.payload.root,
        NativeEventKind::Press,
        None,
    );
    session.handle_host_event_v1(&second_event).unwrap();
    assert_eq!(session.last_event_sequence(), 2);
}

#[test]
fn protocol_v1_and_legacy_modes_cannot_be_mixed() {
    let frame = counter_frame(&CounterState::default()).unwrap();

    let mut strict =
        NativeProtocolSession::new_with_session_id(Gtk4Adapter, "strict-session").unwrap();
    let request = render_request(&strict, 1, &frame);
    let rendered = strict.render_v1(&request).unwrap();
    strict
        .acknowledge_render_v1(&render_ack(&strict, 1, 1))
        .unwrap();

    assert!(strict.render_frame(&frame).is_err());
    assert!(strict
        .handle_host_event(&HostEvent {
            frame_id: "counter".to_string(),
            event: NativeEvent::new(
                HostNodeId::new(rendered.payload.root),
                NativeEventKind::Press,
            ),
        })
        .is_err());
    assert!(strict.pending_commands().is_empty());
    assert_eq!(strict.mode(), ProtocolSessionMode::StrictV1);

    let mut legacy = NativeProtocolSession::new(Gtk4Adapter);
    legacy.render_frame(&frame).unwrap();
    assert_eq!(legacy.mode(), ProtocolSessionMode::Legacy);
    let v1_after_legacy = ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::render(legacy.session_id(), 2),
        ProtocolUiFrameV1::try_from(&frame).unwrap(),
    );
    assert!(legacy.render_v1(&v1_after_legacy).is_err());
    assert_eq!(legacy.mode(), ProtocolSessionMode::Legacy);
}

#[test]
fn protocol_v1_invalid_frame_is_atomic_and_a_valid_retry_can_commit() {
    let mut session =
        NativeProtocolSession::new_with_session_id(Gtk4Adapter, "atomic-session").unwrap();
    let invalid = ProtocolEnvelopeV1::new(
        ProtocolMetadataV1::render(session.session_id(), 1),
        ProtocolUiFrameV1 {
            frame_id: "invalid".to_string(),
            root: ProtocolCompiledNodeV1::Text {
                key: "text".to_string(),
                value: "not a root element".to_string(),
            },
            actions: Vec::new(),
            window: None,
        },
    );

    assert!(session.render_v1(&invalid).is_err());
    assert_eq!(session.render_revision(), 0);
    assert!(session.root().is_none());
    assert!(session.active_frame_id().is_none());
    assert!(session.pending_render_v1().is_none());
    assert!(session.runtime().host().commands().is_empty());
    assert!(session.runtime().host().nodes().is_empty());

    let frame = counter_frame(&CounterState::default()).unwrap();
    let valid = render_request(&session, 1, &frame);
    let response = session.render_v1(&valid).unwrap();
    assert_eq!(response.payload.command_sequence, 1);
    assert_eq!(session.render_revision(), 1);
}

#[test]
fn protocol_v1_password_values_never_leave_response_boundaries() {
    let frame: UiFrame = serde_json::from_value(serde_json::json!({
        "frameId": "password",
        "root": {
            "kind": "element",
            "key": "password",
            "tag": "TextField",
            "props": {
                "inputType": "password",
                "value": "initial-v1-password-secret",
                "attributes": {"aria-valuetext": "described-v1-password-secret"},
                "events": {"onChange": "setPassword"}
            }
        }
    }))
    .unwrap();
    let mut session =
        NativeProtocolSession::new_with_session_id(Gtk4Adapter, "password-session").unwrap();
    let request = render_request(&session, 1, &frame);

    // The request is the authorized input boundary; the in-process native plan
    // must still receive its value. Only responses and diagnostics are redacted.
    assert!(serde_json::to_string(&request)
        .unwrap()
        .contains("initial-v1-password-secret"));
    let rendered = session.render_v1(&request).unwrap();
    assert_eq!(
        session
            .runtime()
            .host()
            .node(HostNodeId::new(rendered.payload.root))
            .and_then(|node| node.blueprint.value.as_deref()),
        Some("initial-v1-password-secret")
    );

    let render_wire = serde_json::to_string(&rendered).unwrap();
    assert!(
        !render_wire.contains("initial-v1-password-secret"),
        "{render_wire}"
    );
    assert!(!render_wire.contains("described-v1-password-secret"));
    assert!(!render_wire.contains("valueSensitivity"));
    assert!(rendered
        .payload
        .accessibility_tree
        .as_ref()
        .is_some_and(|tree| tree.value.is_none()));
    let session_debug = format!("{session:?}");
    assert!(!session_debug.contains("initial-v1-password-secret"));
    assert!(!session_debug.contains("described-v1-password-secret"));

    session
        .acknowledge_render_v1(&render_ack(&session, 1, 1))
        .unwrap();
    let event = host_event(
        &session,
        1,
        1,
        "password",
        rendered.payload.root,
        NativeEventKind::Change,
        Some("changed-v1-password-secret"),
    );
    let response = session.handle_host_event_v1(&event).unwrap();

    assert_eq!(
        session
            .runtime()
            .interactions()
            .node(HostNodeId::new(rendered.payload.root))
            .and_then(|state| state.value.as_deref()),
        Some("changed-v1-password-secret")
    );
    assert_eq!(
        session
            .runtime()
            .actions()
            .invocations()
            .last()
            .and_then(|invocation| invocation.value.as_deref()),
        None
    );
    assert!(session
        .runtime()
        .interactions()
        .changes()
        .last()
        .is_some_and(|change| change.before.value.is_none() && change.after.value.is_none()));
    assert!(session
        .accessibility_tree()
        .is_some_and(|tree| tree.value.is_none()));
    assert!(response
        .payload
        .invocation
        .as_ref()
        .is_some_and(|invocation| invocation.value.is_none()));
    assert!(response
        .payload
        .interaction_changes
        .iter()
        .all(|change| { change.before.value.is_none() && change.after.value.is_none() }));

    let event_wire = serde_json::to_string(&response).unwrap();
    assert!(!event_wire.contains("initial-v1-password-secret"));
    assert!(!event_wire.contains("changed-v1-password-secret"));
    assert!(!event_wire.contains("described-v1-password-secret"));
    assert!(!event_wire.contains("valueSensitivity"));
    let session_debug = format!("{session:?}");
    assert!(!session_debug.contains("initial-v1-password-secret"));
    assert!(!session_debug.contains("changed-v1-password-secret"));
}

#[test]
fn legacy_native_render_response_redacts_password_commands() {
    let frame: UiFrame = serde_json::from_value(serde_json::json!({
        "frameId": "legacy-password",
        "root": {
            "kind": "element",
            "key": "password",
            "tag": "TextField",
            "props": {
                "inputType": "password",
                "value": "legacy-password-secret",
                "attributes": {"aria-valuetext": "legacy-described-secret"}
            }
        }
    }))
    .unwrap();
    let mut session = NativeProtocolSession::new(Gtk4Adapter);

    let response = session.render_frame(&frame).unwrap();
    let planned_value = response.commands.iter().find_map(|command| match command {
        PlatformCommand::Create { blueprint, .. } => blueprint.value.as_deref(),
        _ => None,
    });
    assert_eq!(planned_value, Some("legacy-password-secret"));

    let wire = serde_json::to_string(&response).unwrap();
    assert!(!wire.contains("legacy-password-secret"));
    assert!(!wire.contains("legacy-described-secret"));
}
