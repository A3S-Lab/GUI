use std::io::Cursor;

use super::*;
use crate::protocol::{
    ProtocolCompiledNodeV1, ProtocolNativeEventKindV1, ProtocolUiFrameV1, UiFrame,
};

const HELLO_FIXTURE: &str = include_str!("../../tests/fixtures/tsx-protocol/hello-v1.json");
const RENDER_FIXTURE: &str =
    include_str!("../../tests/fixtures/tsx-protocol/render-counter-v1.json");
const COMMITTED_FIXTURE: &str =
    include_str!("../../tests/fixtures/tsx-protocol/committed-counter-v1.json");
const EVENT_FIXTURE: &str = include_str!("../../tests/fixtures/tsx-protocol/event-counter-v1.json");

fn hello(maximum_frame_bytes: u32, requested_renderer: TsxRendererV1) -> TsxClientMessageV1 {
    TsxClientMessageV1::hello(
        "tsx-fixture",
        1,
        TsxHelloPayloadV1 {
            sdk_version: "0.1.0".to_string(),
            minimum_protocol_version: 1,
            maximum_protocol_version: 1,
            requested_renderer,
            maximum_frame_bytes,
            debug_capabilities: vec![TsxDebugCapabilityV1::StructuredDiagnostics],
        },
    )
}

fn host_config(maximum_frame_bytes: u32) -> TsxHostHandshakeConfigV1 {
    TsxHostHandshakeConfigV1::new(
        "0.1.0",
        "test-build",
        TsxHostPlatformV1::Headless,
        vec![TsxRendererV1::Software, TsxRendererV1::Gpu],
        maximum_frame_bytes,
        vec![
            TsxHostCapabilityV1::HeadlessRendering,
            TsxHostCapabilityV1::DropPolicyQueries,
        ],
        vec![TsxDebugCapabilityV1::StructuredDiagnostics],
    )
    .unwrap()
}

fn negotiated(maximum_frame_bytes: u32) -> TsxNegotiatedSessionV1 {
    let mut handshake = TsxHostHandshakeV1::new(host_config(maximum_frame_bytes));
    handshake
        .accept_hello(&hello(maximum_frame_bytes, TsxRendererV1::Software))
        .unwrap();
    handshake.negotiated().unwrap().clone()
}

fn counter_frame() -> UiFrame {
    serde_json::from_value(serde_json::json!({
        "frameId": "counter",
        "actions": [{"id": "increment"}],
        "root": {
            "kind": "element",
            "key": "increment",
            "tag": "Button",
            "props": {
                "events": {"onPress": "increment"},
                "explicitProps": ["onPress"]
            },
            "children": [{"kind": "text", "key": "text-0", "value": "Count 0"}]
        }
    }))
    .unwrap()
}

fn counter_render(message_id: u64, render_revision: u64) -> TsxClientMessageV1 {
    TsxClientMessageV1::render(
        "tsx-fixture",
        message_id,
        render_revision,
        ProtocolUiFrameV1::try_from(&counter_frame()).unwrap(),
    )
}

fn counter_committed(host_revision: u64) -> TsxCommittedPayloadV1 {
    TsxCommittedPayloadV1 {
        frame_id: "counter".to_string(),
        host_revision,
        root_id: "9:increment".to_string(),
        layout_fingerprint: 11.into(),
        scene_fingerprint: 17.into(),
        diagnostics: vec![],
    }
}

fn counter_event(host_revision: u64, event_sequence: u64) -> TsxEventPayloadV1 {
    TsxEventPayloadV1 {
        host_revision,
        event_sequence,
        target: Some("9:increment".to_string()),
        invocations: vec![TsxActionInvocationV1 {
            node: "9:increment".to_string(),
            current_target: None,
            action: "increment".to_string(),
            event: ProtocolNativeEventKindV1::Press,
            context: TsxEventContextV1 {
                device: 1,
                pointer: None,
                modality: TsxInputModalityV1::Keyboard,
                modifiers: TsxKeyModifiersV1::default(),
                position: None,
                delta: None,
                button: None,
                pressure: None,
                wheel_delta_mode: None,
                repeat: false,
                click_count: 0,
                handled_activation: true,
                related_target: None,
                drag: None,
                timestamp_micros: 42,
            },
            value: None,
        }],
        interaction_changes: vec![TsxInteractionChangeV1 {
            node: "9:increment".to_string(),
            before: TsxElementInteractionV1::default(),
            after: TsxElementInteractionV1 {
                focused: true,
                focus_visible: true,
                ..TsxElementInteractionV1::default()
            },
        }],
        propagation_stopped_at: None,
    }
}

#[test]
fn application_golden_fixtures_are_strict_and_canonical() {
    let limits = TsxFrameLimitsV1::new(8192).unwrap();

    let render =
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(RENDER_FIXTURE.trim().as_bytes(), limits)
            .unwrap();
    render.validate().unwrap();
    assert_eq!(render, counter_render(2, 1));
    assert_eq!(
        serde_json::to_string(&render).unwrap(),
        RENDER_FIXTURE.trim()
    );

    let committed =
        decode_tsx_json_payload_v1::<TsxHostMessageV1>(COMMITTED_FIXTURE.trim().as_bytes(), limits)
            .unwrap();
    committed.validate().unwrap();
    assert_eq!(
        committed,
        TsxHostMessageV1::committed("tsx-fixture", 2, 1, counter_committed(1))
    );
    assert_eq!(
        serde_json::to_string(&committed).unwrap(),
        COMMITTED_FIXTURE.trim()
    );

    let event =
        decode_tsx_json_payload_v1::<TsxHostMessageV1>(EVENT_FIXTURE.trim().as_bytes(), limits)
            .unwrap();
    event.validate().unwrap();
    assert_eq!(
        event,
        TsxHostMessageV1::event("tsx-fixture", 3, 1, counter_event(1, 1))
    );
    assert_eq!(serde_json::to_string(&event).unwrap(), EVENT_FIXTURE.trim());
}

#[test]
fn application_session_commits_and_rejects_atomically() {
    let mut session = TsxHostApplicationSessionV1::new(&negotiated(1024)).unwrap();
    let accepted = session.accept_render(&counter_render(2, 1)).unwrap();
    assert_eq!(accepted.render_revision(), 1);
    assert_eq!(accepted.frame().frame_id, "counter");
    assert_eq!(session.committed_render_revision(), 0);
    assert_eq!(session.last_client_message_id(), 2);
    assert_eq!(session.pending_render().unwrap().root_key(), "increment");

    assert!(session.reject_pending_render(2).is_err());
    assert!(session.pending_render().is_some());
    session.reject_pending_render(1).unwrap();
    assert!(session.pending_render().is_none());
    assert_eq!(session.committed_render_revision(), 0);

    assert!(session.accept_render(&counter_render(2, 1)).is_err());
    assert_eq!(session.last_client_message_id(), 2);
    session.accept_render(&counter_render(3, 1)).unwrap();

    let mut wrong_frame = counter_committed(1);
    wrong_frame.frame_id = "other".to_string();
    assert!(session.commit_pending_render(wrong_frame).is_err());
    assert_eq!(session.last_host_message_id(), 1);
    assert_eq!(session.committed_render_revision(), 0);
    assert!(session.pending_render().is_some());

    let mut oversized = counter_committed(1);
    oversized.diagnostics.push(TsxDiagnosticV1 {
        severity: TsxDiagnosticSeverityV1::Warning,
        code: "large".to_string(),
        message: "x".repeat(900),
        element_id: None,
    });
    assert!(session.commit_pending_render(oversized).is_err());
    assert_eq!(session.last_host_message_id(), 1);
    assert_eq!(session.committed_render_revision(), 0);
    assert!(session.pending_render().is_some());

    let committed = session.commit_pending_render(counter_committed(1)).unwrap();
    assert_eq!(committed.metadata().message_id, 2);
    assert_eq!(committed.metadata().render_revision, 1);
    assert_eq!(session.committed_render_revision(), 1);
    assert_eq!(session.committed_host_revision(), Some(1));
    assert!(session.pending_render().is_none());

    assert!(session.emit_event(counter_event(1, 2)).is_err());
    assert!(session.emit_event(counter_event(2, 1)).is_err());
    assert_eq!(session.last_event_sequence(), 0);
    assert_eq!(session.last_host_message_id(), 2);

    let event = session.emit_event(counter_event(1, 1)).unwrap();
    assert_eq!(event.metadata().message_id, 3);
    assert_eq!(event.metadata().render_revision, 1);
    assert_eq!(session.last_event_sequence(), 1);
    assert!(session.emit_event(counter_event(1, 1)).is_err());
    assert_eq!(session.last_host_message_id(), 3);
}

#[test]
fn stale_render_metadata_fails_before_session_mutation() {
    let mut session = TsxHostApplicationSessionV1::new(&negotiated(4096)).unwrap();

    let mut wrong_session = counter_render(2, 1);
    let TsxClientMessageV1::Render { session_id, .. } = &mut wrong_session else {
        unreachable!()
    };
    *session_id = "other-session".to_string();
    assert!(session.accept_render(&wrong_session).is_err());

    assert!(session.accept_render(&counter_render(2, 2)).is_err());
    assert_eq!(session.last_client_message_id(), 1);
    assert_eq!(session.committed_render_revision(), 0);
    assert!(session.pending_render().is_none());

    let ping = TsxClientMessageV1::Ping {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 2,
        render_revision: 0,
        payload: TsxLivenessPayloadV1 { nonce: 9 },
    };
    session.accept_control(&ping).unwrap();
    session.accept_render(&counter_render(3, 1)).unwrap();
    assert_eq!(session.last_client_message_id(), 3);
}

#[test]
fn unchanged_host_frame_can_commit_a_new_tsx_callback_scope() {
    let mut session = TsxHostApplicationSessionV1::new(&negotiated(4096)).unwrap();
    session.accept_render(&counter_render(2, 1)).unwrap();
    session.commit_pending_render(counter_committed(7)).unwrap();

    session.accept_render(&counter_render(3, 2)).unwrap();
    let committed = session.commit_pending_render(counter_committed(7)).unwrap();
    assert_eq!(committed.metadata().render_revision, 2);
    assert_eq!(session.committed_render_revision(), 2);
    assert_eq!(session.committed_host_revision(), Some(7));

    let event = session.emit_event(counter_event(7, 1)).unwrap();
    assert_eq!(event.metadata().render_revision, 2);
}

#[test]
fn nested_application_payloads_reject_unknown_fields() {
    let limits = TsxFrameLimitsV1::new(8192).unwrap();
    let unknown_context_field =
        EVENT_FIXTURE.replace("\"device\":1,", "\"device\":1,\"unexpected\":true,");
    assert!(decode_tsx_json_payload_v1::<TsxHostMessageV1>(
        unknown_context_field.as_bytes(),
        limits
    )
    .unwrap_err()
    .to_string()
    .contains("unknown field"));
}

#[test]
fn compiled_node_import_source_uses_the_camel_case_wire_name() {
    let mut frame = ProtocolUiFrameV1::try_from(&counter_frame()).unwrap();
    let ProtocolCompiledNodeV1::Element { import_source, .. } = &mut frame.root else {
        unreachable!()
    };
    *import_source = Some("@a3s/gui".to_string());

    let value = serde_json::to_value(&frame).unwrap();
    assert_eq!(value["root"]["importSource"], "@a3s/gui");
    assert!(value["root"].get("import_source").is_none());

    let legacy = serde_json::to_string(&frame)
        .unwrap()
        .replace("importSource", "import_source");
    assert!(serde_json::from_str::<ProtocolUiFrameV1>(&legacy)
        .unwrap_err()
        .to_string()
        .contains("unknown field"));
}

#[cfg(feature = "authoring")]
#[test]
fn static_tsx_and_rust_rsx_counter_share_native_and_accessibility_evidence() {
    use crate::accessibility::AccessibilityNode;
    use crate::compiler::RsxCompilerBridge;

    let limits = TsxFrameLimitsV1::new(8192).unwrap();
    let message =
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(RENDER_FIXTURE.trim().as_bytes(), limits)
            .unwrap();
    let TsxClientMessageV1::Render { payload, .. } = message else {
        panic!("expected render fixture")
    };
    let tsx_frame: UiFrame = payload.try_into().unwrap();
    let rust_frame = UiFrame::from_rsx_source(
        "counter",
        r#"
            <Button key="increment" onPress={increment}>
              Count 0
            </Button>
        "#,
    )
    .unwrap();

    assert_eq!(tsx_frame.root, rust_frame.root);

    let bridge = RsxCompilerBridge::new();
    let tsx_native = bridge.lower_to_native(&tsx_frame.root).unwrap();
    let rust_native = bridge.lower_to_native(&rust_frame.root).unwrap();
    assert_eq!(tsx_native, rust_native);
    assert_eq!(
        stable_fixture_fingerprint(format!("{tsx_native:#?}").as_bytes()),
        stable_fixture_fingerprint(format!("{rust_native:#?}").as_bytes())
    );

    let tsx_accessibility =
        serde_json::to_vec(&AccessibilityNode::from_native(&tsx_native)).unwrap();
    let rust_accessibility =
        serde_json::to_vec(&AccessibilityNode::from_native(&rust_native)).unwrap();
    assert_eq!(tsx_accessibility, rust_accessibility);
    assert_eq!(
        stable_fixture_fingerprint(&tsx_accessibility),
        stable_fixture_fingerprint(&rust_accessibility)
    );
}

#[cfg(feature = "authoring")]
fn stable_fixture_fingerprint(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(feature = "platform-runtime")]
#[test]
fn self_drawn_snapshot_and_event_dispatch_feed_the_tsx_session_directly() {
    use crate::compiler::RsxCompilerBridge;
    use crate::geometry::Size;
    use crate::input::{NativeInputModality, NativeKeyModifiers};
    use crate::platform_host::{
        PlatformElementId, PlatformHostRevision, PlatformInputDeviceId, PlatformWindowId,
        PlatformWindowSpec, RecordingPlatformHost,
    };
    use crate::platform_runtime::{
        RecordingScenePresenter, SelfDrawnActionInvocation, SelfDrawnElementInteraction,
        SelfDrawnEventContext, SelfDrawnInputDispatch, SelfDrawnInteractionChange,
        SelfDrawnWindowRuntime,
    };
    use crate::NativeEventKind;

    let native = RsxCompilerBridge::new()
        .lower_to_native(&counter_frame().root)
        .unwrap();
    let mut runtime = SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        RecordingScenePresenter::new(),
        PlatformWindowSpec {
            id: PlatformWindowId::new(1),
            title: "TSX counter".to_string(),
            logical_size: Size::new(160.0, 80.0),
            min_size: None,
            max_size: None,
            resizable: true,
            visible: true,
        },
        1.0,
    )
    .unwrap();
    runtime.render(native).unwrap();
    let snapshot = runtime.snapshot().unwrap();

    let mut session = TsxHostApplicationSessionV1::new(&negotiated(4096)).unwrap();
    session.accept_render(&counter_render(2, 1)).unwrap();
    let committed = session
        .commit_self_drawn_snapshot("counter", snapshot, vec![])
        .unwrap();
    let TsxHostMessageV1::Committed { payload, .. } = committed else {
        panic!("expected committed message")
    };
    assert_eq!(payload.root_id, "9:increment");
    assert_eq!(payload.host_revision, snapshot.revision().get());

    let revision = snapshot.revision();
    let target = PlatformElementId::new("9:increment").unwrap();
    let context = SelfDrawnEventContext {
        device: PlatformInputDeviceId::new(1),
        pointer: None,
        modality: NativeInputModality::Keyboard,
        modifiers: NativeKeyModifiers::new(),
        position: None,
        delta: None,
        button: None,
        pressure: None,
        wheel_delta_mode: None,
        repeat: false,
        click_count: 0,
        handled_activation: true,
        related_target: None,
        drag: None,
        timestamp_micros: 42,
    };
    let dispatch = SelfDrawnInputDispatch {
        frame_revision: revision,
        event_sequence: 1,
        target: Some(target.clone()),
        invocations: vec![SelfDrawnActionInvocation {
            frame_revision: revision,
            event_sequence: 1,
            node: target.clone(),
            current_target: None,
            action: "increment".to_string(),
            event: NativeEventKind::Press,
            context,
            value: None,
        }],
        interaction_changes: vec![SelfDrawnInteractionChange {
            node: target,
            before: SelfDrawnElementInteraction::default(),
            after: SelfDrawnElementInteraction {
                focused: true,
                focus_visible: true,
                ..SelfDrawnElementInteraction::default()
            },
        }],
        propagation_stopped_at: None,
    };
    let event = session.emit_self_drawn_event(&dispatch).unwrap();
    assert_eq!(event.metadata().render_revision, 1);
    let TsxHostMessageV1::Event { payload, .. } = event else {
        panic!("expected event message")
    };
    assert_eq!(payload.host_revision, revision.get());
    assert_eq!(payload.invocations.len(), 1);

    let mut inconsistent = dispatch;
    inconsistent.invocations[0].frame_revision = PlatformHostRevision::new(revision.get() + 1);
    assert!(TsxEventPayloadV1::try_from(&inconsistent).is_err());
}

#[test]
fn hello_golden_fixture_is_strict_and_has_a_little_endian_length_prefix() {
    let limits = TsxFrameLimitsV1::new(4096).unwrap();
    let message =
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(HELLO_FIXTURE.trim().as_bytes(), limits)
            .unwrap();
    message.validate().unwrap();
    assert_eq!(message, hello(4096, TsxRendererV1::Software));
    assert_eq!(
        serde_json::to_string(&message).unwrap(),
        HELLO_FIXTURE.trim()
    );

    let frame = encode_tsx_json_frame_v1(&message, limits).unwrap();
    assert_eq!(
        u32::from_le_bytes(frame[..4].try_into().unwrap()) as usize,
        HELLO_FIXTURE.trim().len()
    );
    assert_eq!(&frame[4..], HELLO_FIXTURE.trim().as_bytes());
}

#[test]
fn incremental_decoder_handles_split_headers_payloads_and_multiple_frames() {
    let limits = TsxFrameLimitsV1::new(4096).unwrap();
    let first = hello(4096, TsxRendererV1::Auto);
    let second = TsxClientMessageV1::Ping {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 2,
        render_revision: 0,
        payload: TsxLivenessPayloadV1 { nonce: 17 },
    };
    let mut bytes = encode_tsx_json_frame_v1(&first, limits).unwrap();
    bytes.extend(encode_tsx_json_frame_v1(&second, limits).unwrap());

    let mut decoder = TsxJsonFrameDecoderV1::new(limits);
    let mut decoded = Vec::new();
    for chunk in bytes.chunks(3) {
        decoded.extend(decoder.push::<TsxClientMessageV1>(chunk).unwrap());
    }
    decoder.finish().unwrap();
    assert_eq!(decoded, vec![first, second]);
}

#[test]
fn framing_rejects_empty_oversized_truncated_and_invalid_payloads() {
    let limits = TsxFrameLimitsV1::new(512).unwrap();

    let mut empty = TsxJsonFrameDecoderV1::new(limits);
    let error = empty
        .push::<TsxClientMessageV1>(&0_u32.to_le_bytes())
        .unwrap_err();
    assert!(error.to_string().contains("empty payload"));
    assert!(empty.is_poisoned());
    assert!(empty
        .push::<TsxClientMessageV1>(&[])
        .unwrap_err()
        .to_string()
        .contains("poisoned"));

    let mut oversized = TsxJsonFrameDecoderV1::new(limits);
    let error = oversized
        .push::<TsxClientMessageV1>(&513_u32.to_le_bytes())
        .unwrap_err();
    assert!(error.to_string().contains("exceeding"));
    assert!(oversized.is_poisoned());

    let mut header = TsxJsonFrameDecoderV1::new(limits);
    header.push::<TsxClientMessageV1>(&[2, 0]).unwrap();
    assert!(header.finish().unwrap_err().to_string().contains("2 of 4"));

    let mut payload = TsxJsonFrameDecoderV1::new(limits);
    payload
        .push::<TsxClientMessageV1>(&5_u32.to_le_bytes())
        .unwrap();
    payload.push::<TsxClientMessageV1>(b"{}").unwrap();
    assert!(payload.finish().unwrap_err().to_string().contains("2 of 5"));

    let error = decode_tsx_json_payload_v1::<TsxClientMessageV1>(&[0xff], limits).unwrap_err();
    assert!(error.to_string().contains("UTF-8"));
}

#[test]
fn strict_messages_reject_unknown_kinds_fields_and_duplicate_fields() {
    let limits = TsxFrameLimitsV1::new(4096).unwrap();
    let unknown_kind = HELLO_FIXTURE.replace("\"hello\"", "\"renderNow\"");
    assert!(
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(unknown_kind.as_bytes(), limits)
            .unwrap_err()
            .to_string()
            .contains("unknown variant")
    );

    let unknown_field =
        HELLO_FIXTURE.replace("\"messageId\":1,", "\"messageId\":1,\"unexpected\":true,");
    assert!(
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(unknown_field.as_bytes(), limits)
            .unwrap_err()
            .to_string()
            .contains("unknown field")
    );

    let duplicate = HELLO_FIXTURE.replace("\"messageId\":1,", "\"messageId\":1,\"messageId\":2,");
    assert!(
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(duplicate.as_bytes(), limits)
            .unwrap_err()
            .to_string()
            .contains("duplicate field")
    );

    let nested_duplicate = HELLO_FIXTURE.replace(
        "\"sdkVersion\":\"0.1.0\",",
        "\"sdkVersion\":\"0.1.0\",\"sdkVersion\":\"other\",",
    );
    assert!(
        decode_tsx_json_payload_v1::<TsxClientMessageV1>(nested_duplicate.as_bytes(), limits)
            .unwrap_err()
            .to_string()
            .contains("duplicate field")
    );
}

#[test]
fn reader_and_writer_round_trip_and_report_truncated_streams() {
    let limits = TsxFrameLimitsV1::new(4096).unwrap();
    let message = hello(4096, TsxRendererV1::Software);
    let mut writer = Cursor::new(Vec::new());
    write_tsx_json_frame_v1(&mut writer, &message, limits).unwrap();

    let mut reader = Cursor::new(writer.into_inner());
    let decoded = read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut reader, limits)
        .unwrap()
        .unwrap();
    assert_eq!(decoded, message);
    assert!(
        read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut reader, limits)
            .unwrap()
            .is_none()
    );

    let mut partial_header = Cursor::new(vec![1, 0]);
    assert!(
        read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut partial_header, limits)
            .unwrap_err()
            .to_string()
            .contains("2 of 4")
    );

    let mut partial_payload = Cursor::new([5_u32.to_le_bytes().as_slice(), b"{}"].concat());
    assert!(
        read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut partial_payload, limits)
            .unwrap_err()
            .to_string()
            .contains("2 of 5")
    );
}

#[test]
fn handshake_negotiates_limits_renderer_and_debug_capabilities_atomically() {
    let mut handshake = TsxHostHandshakeV1::new(host_config(2048));
    let welcome = handshake
        .accept_hello(&hello(1024, TsxRendererV1::Gpu))
        .unwrap();
    let TsxHostMessageV1::Welcome {
        protocol,
        protocol_version,
        session_id,
        message_id,
        render_revision,
        payload,
    } = welcome
    else {
        panic!("expected welcome")
    };
    assert_eq!(protocol, TSX_PROTOCOL_NAME);
    assert_eq!(protocol_version, 1);
    assert_eq!(session_id, "tsx-fixture");
    assert_eq!(message_id, 1);
    assert_eq!(render_revision, 0);
    assert_eq!(payload.renderer, TsxRendererV1::Gpu);
    assert_eq!(payload.limits.maximum_frame_bytes, 1024);
    assert_eq!(payload.limits.maximum_in_flight_renders, 1);
    assert_eq!(
        payload.debug_capabilities,
        vec![TsxDebugCapabilityV1::StructuredDiagnostics]
    );
    let negotiated = handshake.negotiated().unwrap();
    assert_eq!(negotiated.session_id(), "tsx-fixture");
    assert_eq!(negotiated.renderer(), TsxRendererV1::Gpu);
    assert_eq!(negotiated.limits().maximum_frame_bytes, 1024);
    assert!(handshake
        .accept_hello(&hello(1024, TsxRendererV1::Gpu))
        .unwrap_err()
        .to_string()
        .contains("already completed"));
}

#[test]
fn failed_handshake_does_not_bind_a_partial_session() {
    let mut handshake = TsxHostHandshakeV1::new(host_config(4096));
    let mut incompatible = hello(4096, TsxRendererV1::Software);
    let TsxClientMessageV1::Hello { payload, .. } = &mut incompatible else {
        unreachable!()
    };
    payload.minimum_protocol_version = 2;
    payload.maximum_protocol_version = 3;
    let error = handshake.accept_hello(&incompatible).unwrap_err();
    assert!(error.to_string().contains("does not include"));
    assert!(handshake.negotiated().is_none());

    handshake
        .accept_hello(&hello(4096, TsxRendererV1::Software))
        .unwrap();
    assert!(handshake.negotiated().is_some());

    let config = TsxHostHandshakeConfigV1::new(
        "0.1.0",
        "test-build",
        TsxHostPlatformV1::Headless,
        vec![TsxRendererV1::Software],
        4096,
        vec![],
        vec![],
    )
    .unwrap();
    let mut unsupported = TsxHostHandshakeV1::new(config);
    assert!(unsupported
        .accept_hello(&hello(4096, TsxRendererV1::Gpu))
        .unwrap_err()
        .to_string()
        .contains("not supported"));
    assert!(unsupported.negotiated().is_none());
}

#[test]
fn message_sequence_rejects_skips_duplicates_and_overflow_without_advancing() {
    let mut sequence = TsxMessageSequenceV1::new();
    assert_eq!(sequence.expected_message_id().unwrap(), 1);
    assert!(sequence.accept(2).is_err());
    assert_eq!(sequence.last_message_id(), 0);
    sequence.accept(1).unwrap();
    assert!(sequence.accept(1).is_err());
    assert_eq!(sequence.last_message_id(), 1);
    sequence.accept(2).unwrap();
    assert_eq!(sequence.last_message_id(), 2);

    let mut overflow = TsxMessageSequenceV1::from_last_message_id(TSX_PROTOCOL_V1_MAX_SAFE_INTEGER);
    assert!(overflow
        .accept(0)
        .unwrap_err()
        .to_string()
        .contains("maximum safe integer"));
    assert_eq!(overflow.last_message_id(), TSX_PROTOCOL_V1_MAX_SAFE_INTEGER);
}

#[test]
fn application_wire_integers_and_fingerprints_are_javascript_lossless() {
    assert_eq!(TsxFingerprintV1::from_u64(11).as_str(), "000000000000000b");
    assert_eq!(
        TsxFingerprintV1::from_u64(u64::MAX).value().unwrap(),
        u64::MAX
    );
    assert!(TsxFingerprintV1::new("000000000000000B").is_err());
    assert!(TsxFingerprintV1::new("b").is_err());

    let invalid_fingerprint = COMMITTED_FIXTURE.replace("000000000000000b", "000000000000000B");
    let decoded: TsxHostMessageV1 = serde_json::from_str(&invalid_fingerprint).unwrap();
    assert!(decoded
        .validate()
        .unwrap_err()
        .to_string()
        .contains("lowercase hexadecimal"));

    let mut unsafe_message = counter_render(2, 1);
    let TsxClientMessageV1::Render { message_id, .. } = &mut unsafe_message else {
        unreachable!()
    };
    *message_id = TSX_PROTOCOL_V1_MAX_SAFE_INTEGER + 1;
    assert!(unsafe_message
        .validate()
        .unwrap_err()
        .to_string()
        .contains("maximum safe integer"));

    let mut unsafe_context = counter_event(1, 1);
    unsafe_context.invocations[0].context.timestamp_micros = TSX_PROTOCOL_V1_MAX_SAFE_INTEGER + 1;
    assert!(unsafe_context
        .validate()
        .unwrap_err()
        .to_string()
        .contains("maximum safe integer"));

    let ping = TsxClientMessageV1::Ping {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 2,
        render_revision: 0,
        payload: TsxLivenessPayloadV1 {
            nonce: TSX_PROTOCOL_V1_MAX_SAFE_INTEGER + 1,
        },
    };
    assert!(ping
        .validate()
        .unwrap_err()
        .to_string()
        .contains("maximum safe integer"));
}

#[test]
fn invalid_configuration_and_message_values_fail_before_negotiation() {
    assert!(TsxHostHandshakeConfigV1::new(
        "0.1.0",
        "build",
        TsxHostPlatformV1::Headless,
        vec![TsxRendererV1::Auto],
        4096,
        vec![],
        vec![],
    )
    .unwrap_err()
    .to_string()
    .contains("cannot contain auto"));

    let mut duplicate_debug = hello(4096, TsxRendererV1::Software);
    let TsxClientMessageV1::Hello { payload, .. } = &mut duplicate_debug else {
        unreachable!()
    };
    payload
        .debug_capabilities
        .push(TsxDebugCapabilityV1::StructuredDiagnostics);
    assert!(duplicate_debug
        .validate()
        .unwrap_err()
        .to_string()
        .contains("duplicate"));

    let mut wrong_protocol = hello(4096, TsxRendererV1::Software);
    let TsxClientMessageV1::Hello { protocol, .. } = &mut wrong_protocol else {
        unreachable!()
    };
    *protocol = "other.protocol".to_string();
    assert!(wrong_protocol
        .validate()
        .unwrap_err()
        .to_string()
        .contains("protocol identifier"));

    let ping = TsxClientMessageV1::Ping {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 1,
        render_revision: 0,
        payload: TsxLivenessPayloadV1 { nonce: 1 },
    };
    let mut non_hello = TsxHostHandshakeV1::new(host_config(4096));
    assert!(non_hello
        .accept_hello(&ping)
        .unwrap_err()
        .to_string()
        .contains("must be hello"));
    assert!(non_hello.negotiated().is_none());

    let mut wrong_message_id = TsxClientMessageV1::hello(
        "tsx-fixture",
        2,
        TsxHelloPayloadV1 {
            sdk_version: "0.1.0".to_string(),
            minimum_protocol_version: 1,
            maximum_protocol_version: 1,
            requested_renderer: TsxRendererV1::Software,
            maximum_frame_bytes: 4096,
            debug_capabilities: vec![],
        },
    );
    let mut wrong_id_handshake = TsxHostHandshakeV1::new(host_config(4096));
    assert!(wrong_id_handshake
        .accept_hello(&wrong_message_id)
        .unwrap_err()
        .to_string()
        .contains("expected 1"));
    assert!(wrong_id_handshake.negotiated().is_none());

    let TsxClientMessageV1::Hello {
        render_revision, ..
    } = &mut wrong_message_id
    else {
        unreachable!()
    };
    *render_revision = 1;
    assert!(wrong_message_id
        .validate()
        .unwrap_err()
        .to_string()
        .contains("revision zero"));

    let tiny = hello(1, TsxRendererV1::Software);
    let mut handshake = TsxHostHandshakeV1::new(host_config(4096));
    assert!(handshake.accept_hello(&tiny).is_err());
    assert!(handshake.negotiated().is_none());
}
