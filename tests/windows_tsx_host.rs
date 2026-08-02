#![cfg(all(
    target_os = "windows",
    feature = "host-windows",
    feature = "platform-runtime",
    feature = "gpu"
))]
#![allow(unsafe_code)]

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use a3s_gui::tsx_protocol::{
    read_tsx_json_frame_v1, write_tsx_json_frame_v1, TsxClientMessageV1, TsxClosePayloadV1,
    TsxCloseReasonV1, TsxFrameLimitsV1, TsxHostCapabilityV1, TsxHostMessageV1, TsxHostPlatformV1,
    TsxRendererV1, TSX_PROTOCOL_NAME,
};
use a3s_gui::{ProtocolCompiledNodeV1, UiAction, WindowOptions};
use windows_sys::Win32::Foundation::{HWND, LPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, PostMessageW, WM_CLOSE,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_PAINT,
};

const HELLO_FIXTURE: &str = include_str!("fixtures/tsx-protocol/hello-v1.json");
const RENDER_FIXTURE: &str = include_str!("fixtures/tsx-protocol/render-counter-v1.json");
const MESSAGE_TIMEOUT: Duration = Duration::from_secs(20);
const MK_LBUTTON: usize = 0x0001;

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
fn native_tsx_host_opens_a_visible_self_drawn_window_and_returns_win32_input() {
    let mut child = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_a3s-gui-tsx-host"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the native TSX host process"),
    );
    let mut input = child.0.stdin.take().expect("take child stdin");
    let output = child.0.stdout.take().expect("take child stdout");
    let messages = spawn_message_reader(output);
    let hard_limits = TsxFrameLimitsV1::default();

    let mut hello: TsxClientMessageV1 = serde_json::from_str(HELLO_FIXTURE).unwrap();
    let TsxClientMessageV1::Hello { payload, .. } = &mut hello else {
        unreachable!()
    };
    payload.requested_renderer = TsxRendererV1::Gpu;
    write_tsx_json_frame_v1(&mut input, &hello, hard_limits).unwrap();

    let welcome = receive_message(&messages);
    let TsxHostMessageV1::Welcome { payload, .. } = welcome else {
        panic!("expected native welcome message")
    };
    assert_eq!(payload.platform, TsxHostPlatformV1::Windows);
    assert_eq!(payload.renderer, TsxRendererV1::Gpu);
    assert!(payload
        .capabilities
        .contains(&TsxHostCapabilityV1::SelfDrawnRendering));
    assert!(!payload
        .capabilities
        .contains(&TsxHostCapabilityV1::HeadlessRendering));
    let limits = TsxFrameLimitsV1::new(payload.limits.maximum_frame_bytes).unwrap();

    let mut render: TsxClientMessageV1 = serde_json::from_str(RENDER_FIXTURE).unwrap();
    let TsxClientMessageV1::Render { payload, .. } = &mut render else {
        unreachable!()
    };
    let ProtocolCompiledNodeV1::Element { props, .. } = &mut payload.root else {
        unreachable!()
    };
    props.class_name = Some("h-[100px] w-[100px] bg-black".to_string());
    let window = WindowOptions {
        title: "A3S native TSX smoke".to_string(),
        on_close: Some("closeWindow".to_string()),
        width: Some(320.0),
        height: Some(240.0),
        min_width: None,
        min_height: None,
        max_width: None,
        max_height: None,
        resizable: true,
    };
    payload.window = Some((&window).into());
    let close_action = UiAction {
        id: "closeWindow".to_string(),
        disabled: false,
        label: None,
    };
    payload.actions.push((&close_action).into());
    write_tsx_json_frame_v1(&mut input, &render, limits).unwrap();
    let committed = receive_message(&messages);
    let TsxHostMessageV1::Committed { payload, .. } = committed else {
        panic!("expected native committed message")
    };
    assert_eq!(payload.host_revision, 1);

    let hwnd = find_visible_window(child.0.id()).expect("find the visible TSX HWND");
    assert_eq!(window_title(hwnd), "A3S native TSX smoke");
    assert_ne!(unsafe { PostMessageW(hwnd, WM_PAINT, 0, 0) }, 0);
    let point = client_lparam(4, 4);
    assert_ne!(
        unsafe { PostMessageW(hwnd, WM_LBUTTONDOWN, MK_LBUTTON, point) },
        0
    );
    assert_ne!(unsafe { PostMessageW(hwnd, WM_LBUTTONUP, 0, point) }, 0);

    let event = loop {
        let message = receive_message(&messages);
        let TsxHostMessageV1::Event { payload, .. } = message else {
            panic!("expected native input event, got {message:?}")
        };
        if payload
            .invocations
            .iter()
            .any(|invocation| invocation.action == "increment")
        {
            break payload;
        }
    };
    assert!(event.host_revision > 1);
    assert!(event
        .target
        .as_deref()
        .is_some_and(|target| target.ends_with("/9:increment")));

    assert_ne!(unsafe { PostMessageW(hwnd, WM_CLOSE, 0, 0) }, 0);
    let close_event = loop {
        let message = receive_message(&messages);
        let TsxHostMessageV1::Event { payload, .. } = message else {
            panic!("expected native close event, got {message:?}")
        };
        if payload
            .invocations
            .iter()
            .any(|invocation| invocation.action == "closeWindow")
        {
            break payload;
        }
    };
    assert!(close_event
        .target
        .as_deref()
        .is_some_and(|target| target.ends_with(":counter:window")));

    let close = TsxClientMessageV1::Close {
        protocol: TSX_PROTOCOL_NAME.to_string(),
        protocol_version: 1,
        session_id: "tsx-fixture".to_string(),
        message_id: 3,
        render_revision: 1,
        payload: TsxClosePayloadV1 {
            reason: TsxCloseReasonV1::Requested,
            message: Some("native process test complete".to_string()),
        },
    };
    write_tsx_json_frame_v1(&mut input, &close, limits).unwrap();
    loop {
        if matches!(receive_message(&messages), TsxHostMessageV1::Close { .. }) {
            break;
        }
    }
    drop(input);

    let deadline = Instant::now() + MESSAGE_TIMEOUT;
    loop {
        if let Some(status) = child.0.try_wait().expect("poll native TSX host") {
            if !status.success() {
                let mut stderr = String::new();
                child
                    .0
                    .stderr
                    .take()
                    .unwrap()
                    .read_to_string(&mut stderr)
                    .unwrap();
                panic!("native TSX host exited with {status}: {stderr}");
            }
            break;
        }
        assert!(Instant::now() < deadline, "native TSX host did not exit");
        thread::sleep(Duration::from_millis(10));
    }
}

fn spawn_message_reader(
    mut output: impl Read + Send + 'static,
) -> Receiver<Result<TsxHostMessageV1, String>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let limits = TsxFrameLimitsV1::default();
        loop {
            let message = match read_tsx_json_frame_v1(&mut output, limits) {
                Ok(Some(message)) => Ok(message),
                Ok(None) => return,
                Err(error) => Err(error.to_string()),
            };
            let failed = message.is_err();
            if sender.send(message).is_err() || failed {
                return;
            }
        }
    });
    receiver
}

fn receive_message(receiver: &Receiver<Result<TsxHostMessageV1, String>>) -> TsxHostMessageV1 {
    receiver
        .recv_timeout(MESSAGE_TIMEOUT)
        .expect("TSX host did not return a protocol message")
        .unwrap_or_else(|message| panic!("TSX host protocol reader failed: {message}"))
}

struct WindowSearch {
    process_id: u32,
    hwnd: HWND,
}

fn find_visible_window(process_id: u32) -> Option<HWND> {
    let deadline = Instant::now() + MESSAGE_TIMEOUT;
    loop {
        let mut search = WindowSearch {
            process_id,
            hwnd: std::ptr::null_mut(),
        };
        unsafe {
            EnumWindows(
                Some(find_process_window),
                (&mut search as *mut WindowSearch) as LPARAM,
            );
        }
        if !search.hwnd.is_null() {
            return Some(search.hwnd);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

unsafe extern "system" fn find_process_window(hwnd: HWND, parameter: LPARAM) -> i32 {
    let search = unsafe { &mut *(parameter as *mut WindowSearch) };
    let mut process_id = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut process_id);
    }
    if process_id == search.process_id && unsafe { IsWindowVisible(hwnd) } != 0 {
        search.hwnd = hwnd;
        0
    } else {
        1
    }
}

fn window_title(hwnd: HWND) -> String {
    let mut buffer = vec![0_u16; 256];
    let length = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), 256) };
    assert!(length >= 0);
    String::from_utf16(&buffer[..length as usize]).unwrap()
}

fn client_lparam(x: i16, y: i16) -> isize {
    ((u32::from(y as u16) << 16) | u32::from(x as u16)) as isize
}
