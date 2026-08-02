use std::io::Cursor;

use super::*;

const HELLO_FIXTURE: &str = include_str!("../../tests/fixtures/tsx-protocol/hello-v1.json");

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

    let mut overflow = TsxMessageSequenceV1::from_last_message_id(u64::MAX);
    assert!(overflow
        .accept(0)
        .unwrap_err()
        .to_string()
        .contains("overflow"));
    assert_eq!(overflow.last_message_id(), u64::MAX);
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
