#![cfg(all(feature = "platform-runtime", feature = "software-reference"))]

use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use a3s_gui::tsx_protocol::{
    read_tsx_json_frame_v1, write_tsx_json_frame_v1, TsxClientMessageV1, TsxClosePayloadV1,
    TsxCloseReasonV1, TsxFrameLimitsV1, TsxHostCapabilityV1, TsxHostMessageV1, TsxHostPlatformV1,
    TsxLivenessPayloadV1, TsxRendererV1, TSX_PROTOCOL_NAME,
};

const HELLO_FIXTURE: &str = include_str!("fixtures/tsx-protocol/hello-v1.json");
const RENDER_FIXTURE: &str = include_str!("fixtures/tsx-protocol/render-counter-v1.json");

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

#[test]
fn headless_host_process_negotiates_renders_and_closes() {
    let mut child = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_a3s-gui-tsx-host"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the TSX host process"),
    );
    let mut input = child.0.stdin.take().expect("take child stdin");
    let mut output = child.0.stdout.take().expect("take child stdout");
    let hard_limits = TsxFrameLimitsV1::default();

    let hello: TsxClientMessageV1 = serde_json::from_str(HELLO_FIXTURE).unwrap();
    write_tsx_json_frame_v1(&mut input, &hello, hard_limits).unwrap();
    let welcome: TsxHostMessageV1 = read_required(&mut output, hard_limits);
    let TsxHostMessageV1::Welcome { payload, .. } = welcome else {
        panic!("expected welcome message")
    };
    assert_eq!(payload.platform, TsxHostPlatformV1::Headless);
    assert_eq!(payload.renderer, TsxRendererV1::Software);
    assert_eq!(payload.limits.maximum_in_flight_renders, 1);
    assert!(payload
        .capabilities
        .contains(&TsxHostCapabilityV1::HeadlessRendering));
    assert!(payload
        .capabilities
        .contains(&TsxHostCapabilityV1::SelfDrawnRendering));
    let session_limits = TsxFrameLimitsV1::new(payload.limits.maximum_frame_bytes).unwrap();

    let render: TsxClientMessageV1 = serde_json::from_str(RENDER_FIXTURE).unwrap();
    write_tsx_json_frame_v1(&mut input, &render, session_limits).unwrap();
    let committed: TsxHostMessageV1 = read_required(&mut output, session_limits);
    let TsxHostMessageV1::Committed {
        message_id,
        render_revision,
        payload,
        ..
    } = committed
    else {
        panic!("expected committed message")
    };
    assert_eq!(message_id, 2);
    assert_eq!(render_revision, 1);
    assert_eq!(payload.frame_id, "counter");
    assert_eq!(payload.host_revision, 1);
    assert_eq!(payload.root_id, "9:increment");
    assert!(payload.diagnostics.is_empty());

    let mut rerender = render.clone();
    let TsxClientMessageV1::Render {
        message_id,
        render_revision,
        ..
    } = &mut rerender
    else {
        unreachable!()
    };
    *message_id = 3;
    *render_revision = 2;
    write_tsx_json_frame_v1(&mut input, &rerender, session_limits).unwrap();
    let recommitted: TsxHostMessageV1 = read_required(&mut output, session_limits);
    let TsxHostMessageV1::Committed {
        message_id,
        render_revision,
        payload,
        ..
    } = recommitted
    else {
        panic!("expected second committed message")
    };
    assert_eq!(message_id, 3);
    assert_eq!(render_revision, 2);
    assert_eq!(payload.host_revision, 1);

    let ping = TsxClientMessageV1::Ping {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 4,
        render_revision: 2,
        payload: TsxLivenessPayloadV1 { nonce: 42 },
    };
    write_tsx_json_frame_v1(&mut input, &ping, session_limits).unwrap();
    let pong: TsxHostMessageV1 = read_required(&mut output, session_limits);
    assert!(matches!(
        pong,
        TsxHostMessageV1::Pong {
            message_id: 4,
            render_revision: 2,
            payload: TsxLivenessPayloadV1 { nonce: 42 },
            ..
        }
    ));

    let close = TsxClientMessageV1::Close {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 5,
        render_revision: 2,
        payload: TsxClosePayloadV1 {
            reason: TsxCloseReasonV1::Requested,
            message: Some("test complete".to_string()),
        },
    };
    write_tsx_json_frame_v1(&mut input, &close, session_limits).unwrap();
    let closed: TsxHostMessageV1 = read_required(&mut output, session_limits);
    let TsxHostMessageV1::Close {
        message_id,
        render_revision,
        payload,
        ..
    } = closed
    else {
        panic!("expected close message")
    };
    assert_eq!(message_id, 5);
    assert_eq!(render_revision, 2);
    assert_eq!(payload.reason, TsxCloseReasonV1::Requested);
    assert_eq!(payload.message.as_deref(), Some("test complete"));

    drop(input);
    let status = child.0.wait().expect("wait for TSX host process");
    if !status.success() {
        let mut stderr = String::new();
        child
            .0
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr)
            .unwrap();
        panic!("TSX host exited with {status}: {stderr}");
    }
}

#[test]
fn headless_host_process_rejects_render_before_hello() {
    let mut child = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_a3s-gui-tsx-host"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the TSX host process"),
    );
    let mut input = child.0.stdin.take().expect("take child stdin");
    let render: TsxClientMessageV1 = serde_json::from_str(RENDER_FIXTURE).unwrap();
    write_tsx_json_frame_v1(&mut input, &render, TsxFrameLimitsV1::default()).unwrap();
    drop(input);

    let status = child.0.wait().expect("wait for rejected TSX host process");
    assert!(!status.success());
    let mut stdout = Vec::new();
    child
        .0
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut stdout)
        .unwrap();
    assert!(stdout.is_empty());
    let mut stderr = String::new();
    child
        .0
        .stderr
        .take()
        .unwrap()
        .read_to_string(&mut stderr)
        .unwrap();
    assert!(stderr.contains("first TSX protocol client message must be hello"));
}

#[test]
fn node_create_app_drives_the_real_self_drawn_host_process() {
    let node = std::env::var_os("A3S_GUI_NODE_BINARY").unwrap_or_else(|| "node".into());
    let script = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("packages/typescript/tests/fixtures/rust-application-host.mjs");
    let output = Command::new(node)
        .arg(script)
        .arg(env!("CARGO_BIN_EXE_a3s-gui-tsx-host"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("NO_COLOR", "1")
        .output()
        .expect("run the Node-to-Rust TSX application fixture");
    if !output.status.success() {
        panic!(
            "Node-to-Rust TSX fixture exited with {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

fn read_required<R: Read>(reader: &mut R, limits: TsxFrameLimitsV1) -> TsxHostMessageV1 {
    read_tsx_json_frame_v1(reader, limits)
        .unwrap()
        .expect("TSX host closed before returning a message")
}
