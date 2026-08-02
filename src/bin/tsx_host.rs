use std::io::{self, Read, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

#[path = "tsx_host/event_pump.rs"]
mod event_pump;
#[path = "tsx_host/runtime_backend.rs"]
mod runtime_backend;

use event_pump::{HostEventPump, HOST_EVENT_POLL_INTERVAL};
use runtime_backend::{selected_profile, HostRuntime, HostRuntimeProfile};

use a3s_gui::tsx_protocol::{
    read_tsx_json_frame_v1, write_tsx_json_frame_v1, TsxClientMessageV1, TsxFrameLimitsV1,
    TsxHostApplicationSessionV1, TsxHostHandshakeConfigV1, TsxHostHandshakeV1, TsxHostMessageV1,
    TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES, TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
};
use a3s_gui::{
    GuiError, GuiResult, PlatformWindowId, PlatformWindowSpec, RsxCompilerBridge, Size, UiFrame,
    WindowOptions,
};

const DEFAULT_WINDOW_WIDTH: f64 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;
const DEFAULT_WINDOW_TITLE: &str = "A3S GUI";
const DEFAULT_LIVENESS_INTERVAL_MS: u64 = 30_000;
const DEFAULT_LIVENESS_TIMEOUT_MS: u64 = 5_000;
const MAXIMUM_LIVENESS_DURATION_MS: u64 = 600_000;
const WINDOW_ID: PlatformWindowId = PlatformWindowId::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProcessOptions {
    liveness_interval: Duration,
    liveness_timeout: Duration,
}

enum SessionInput {
    Message(TsxClientMessageV1),
    End,
    Failed(String),
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> GuiResult<()> {
    let options = process_options(std::env::args().skip(1))?;
    let stdout = io::stdout();
    serve(io::stdin(), stdout.lock(), options)
}

fn serve<R, W>(mut input: R, mut output: W, options: ProcessOptions) -> GuiResult<()>
where
    R: Read + Send + 'static,
    W: Write,
{
    let profile = selected_profile()?;
    let hard_limits = TsxFrameLimitsV1::default();
    let hello = read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut input, hard_limits)?
        .ok_or_else(|| GuiError::host("TSX host input ended before the required hello message"))?;
    let mut handshake = TsxHostHandshakeV1::new(host_config(profile)?);
    let welcome = handshake.accept_hello(&hello)?;
    let negotiated = handshake
        .negotiated()
        .cloned()
        .ok_or_else(|| GuiError::host("TSX host handshake did not produce a session"))?;
    let session_limits = TsxFrameLimitsV1::new(negotiated.limits().maximum_frame_bytes)?;
    write_tsx_json_frame_v1(&mut output, &welcome, session_limits)?;

    let input = spawn_session_reader(input, session_limits)?;
    let mut session = TsxHostApplicationSessionV1::new(&negotiated)?;
    let mut runtime = None;
    let result = serve_session(
        &input,
        &mut output,
        session_limits,
        options,
        &mut session,
        &mut runtime,
    );
    let shutdown = match runtime.as_mut() {
        Some(runtime) => runtime.shutdown(),
        None => Ok(()),
    };
    match (result, shutdown) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(())) => Ok(()),
    }
}

fn serve_session<W>(
    input: &Receiver<SessionInput>,
    output: &mut W,
    limits: TsxFrameLimitsV1,
    options: ProcessOptions,
    session: &mut TsxHostApplicationSessionV1,
    runtime: &mut Option<HostRuntime>,
) -> GuiResult<()>
where
    W: Write,
{
    let mut next_ping_at = Instant::now() + options.liveness_interval;
    let mut ping_deadline = None;
    let mut next_nonce = 1;
    let mut event_pump = HostEventPump::default();
    loop {
        event_pump.drain(output, limits, session, runtime)?;

        let now = Instant::now();
        if ping_deadline.is_some_and(|deadline| now >= deadline) {
            return unanswered_liveness_error(session, options);
        }
        if ping_deadline.is_none() && now >= next_ping_at {
            let ping = session.begin_host_ping(next_nonce)?;
            write_tsx_json_frame_v1(output, &ping, limits)?;
            ping_deadline = Some(Instant::now() + options.liveness_timeout);
        }
        let deadline = ping_deadline.unwrap_or(next_ping_at);
        let protocol_wait = deadline.saturating_duration_since(Instant::now());
        let wait = if runtime.is_some() {
            protocol_wait.min(HOST_EVENT_POLL_INTERVAL)
        } else {
            protocol_wait
        };
        match input.recv_timeout(wait) {
            Ok(SessionInput::Message(message)) => {
                if handle_session_message(output, limits, session, runtime, &message)? {
                    return Ok(());
                }
                if session.pending_host_ping_nonce().is_none() {
                    if ping_deadline.is_some() {
                        next_nonce = next_liveness_nonce(next_nonce);
                    }
                    ping_deadline = None;
                    next_ping_at = Instant::now() + options.liveness_interval;
                }
            }
            Ok(SessionInput::End) => {
                return Err(GuiError::host(
                    "TSX host input ended before a protocol close message",
                ));
            }
            Ok(SessionInput::Failed(message)) => return Err(GuiError::host(message)),
            Err(RecvTimeoutError::Disconnected) => {
                return Err(GuiError::host("TSX host input reader stopped unexpectedly"));
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
    }
}

fn unanswered_liveness_error(
    session: &TsxHostApplicationSessionV1,
    options: ProcessOptions,
) -> GuiResult<()> {
    let nonce = session.pending_host_ping_nonce().ok_or_else(|| {
        GuiError::host("TSX liveness deadline elapsed without a pending host nonce")
    })?;
    Err(GuiError::host(format!(
        "TSX client did not answer host liveness nonce {nonce} within {}ms",
        options.liveness_timeout.as_millis()
    )))
}

fn handle_session_message<W>(
    output: &mut W,
    limits: TsxFrameLimitsV1,
    session: &mut TsxHostApplicationSessionV1,
    runtime: &mut Option<HostRuntime>,
    message: &TsxClientMessageV1,
) -> GuiResult<bool>
where
    W: Write,
{
    match message {
        TsxClientMessageV1::Render { .. } => {
            let committed = render_message(session, message, runtime)?;
            write_tsx_json_frame_v1(output, &committed, limits)?;
            Ok(false)
        }
        TsxClientMessageV1::Close { .. } => {
            let closed = session
                .accept_control(message)?
                .ok_or_else(|| GuiError::host("TSX close message did not produce a reply"))?;
            write_tsx_json_frame_v1(output, &closed, limits)?;
            Ok(true)
        }
        TsxClientMessageV1::Ping { .. } | TsxClientMessageV1::Pong { .. } => {
            if let Some(response) = session.accept_control(message)? {
                write_tsx_json_frame_v1(output, &response, limits)?;
            }
            Ok(false)
        }
        TsxClientMessageV1::Hello { .. } => Err(GuiError::host(
            "TSX protocol hello is only valid as the first client message",
        )),
        _ => Err(GuiError::host(
            "TSX host received an unsupported protocol-v1 client message",
        )),
    }
}

fn spawn_session_reader<R>(
    mut input: R,
    limits: TsxFrameLimitsV1,
) -> GuiResult<Receiver<SessionInput>>
where
    R: Read + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    thread::Builder::new()
        .name("a3s-gui-tsx-input".to_string())
        .spawn(move || loop {
            let input = match read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut input, limits) {
                Ok(Some(message)) => SessionInput::Message(message),
                Ok(None) => SessionInput::End,
                Err(error) => SessionInput::Failed(error.to_string()),
            };
            let terminal = matches!(input, SessionInput::End | SessionInput::Failed(_));
            if sender.send(input).is_err() || terminal {
                return;
            }
        })
        .map_err(|error| GuiError::host(format!("could not start TSX input reader: {error}")))?;
    Ok(receiver)
}

fn next_liveness_nonce(nonce: u64) -> u64 {
    if nonce == TSX_PROTOCOL_V1_MAX_SAFE_INTEGER {
        0
    } else {
        nonce + 1
    }
}

fn process_options(arguments: impl IntoIterator<Item = String>) -> GuiResult<ProcessOptions> {
    let mut arguments = arguments.into_iter();
    let mut interval = None;
    let mut timeout = None;
    while let Some(argument) = arguments.next() {
        let slot = match argument.as_str() {
            "--liveness-interval-ms" => &mut interval,
            "--liveness-timeout-ms" => &mut timeout,
            _ => {
                return Err(GuiError::host(format!(
                    "unknown TSX host option {argument:?}"
                )));
            }
        };
        if slot.is_some() {
            return Err(GuiError::host(format!(
                "TSX host option {argument:?} was provided more than once"
            )));
        }
        let value = arguments.next().ok_or_else(|| {
            GuiError::host(format!("TSX host option {argument:?} requires a value"))
        })?;
        *slot = Some(parse_liveness_duration(&argument, &value)?);
    }
    Ok(ProcessOptions {
        liveness_interval: interval
            .unwrap_or_else(|| Duration::from_millis(DEFAULT_LIVENESS_INTERVAL_MS)),
        liveness_timeout: timeout
            .unwrap_or_else(|| Duration::from_millis(DEFAULT_LIVENESS_TIMEOUT_MS)),
    })
}

fn parse_liveness_duration(option: &str, value: &str) -> GuiResult<Duration> {
    let milliseconds = value
        .parse::<u64>()
        .map_err(|_| GuiError::host(format!("TSX host option {option:?} must be an integer")))?;
    if !(1..=MAXIMUM_LIVENESS_DURATION_MS).contains(&milliseconds) {
        return Err(GuiError::host(format!(
            "TSX host option {option:?} must be from 1 through {MAXIMUM_LIVENESS_DURATION_MS}"
        )));
    }
    Ok(Duration::from_millis(milliseconds))
}

fn render_message(
    session: &mut TsxHostApplicationSessionV1,
    message: &TsxClientMessageV1,
    runtime: &mut Option<HostRuntime>,
) -> GuiResult<TsxHostMessageV1> {
    let accepted = session.accept_render(message)?;
    let render_revision = accepted.render_revision();
    let result = render_frame(session, accepted.into_frame(), runtime);
    if result.is_err() {
        session.reject_pending_render(render_revision)?;
    }
    result
}

fn render_frame(
    session: &mut TsxHostApplicationSessionV1,
    frame: UiFrame,
    runtime: &mut Option<HostRuntime>,
) -> GuiResult<TsxHostMessageV1> {
    let frame_id = frame.frame_id.clone();
    let desired_spec = window_spec(frame.window.as_ref())?;
    let content = RsxCompilerBridge::new().lower_to_native(&frame.root)?;
    let native_root = match &frame.window {
        Some(window) => window.wrap_native_root(&frame.frame_id, content),
        None => content,
    };

    let runtime = match runtime {
        Some(runtime) => {
            if runtime.window_spec() != &desired_spec {
                return Err(GuiError::host(
                    "TSX protocol v1 cannot change window options after the first render",
                ));
            }
            runtime
        }
        slot @ None => slot.insert(HostRuntime::new(desired_spec)?),
    };
    runtime.render(native_root)?;
    let snapshot = runtime
        .snapshot()
        .ok_or_else(|| GuiError::host("self-drawn runtime did not commit a TSX snapshot"))?;
    session.commit_self_drawn_snapshot(frame_id, snapshot, Vec::new())
}

fn host_config(profile: HostRuntimeProfile) -> GuiResult<TsxHostHandshakeConfigV1> {
    TsxHostHandshakeConfigV1::new(
        env!("CARGO_PKG_VERSION"),
        option_env!("A3S_GUI_BUILD_ID").unwrap_or("development"),
        profile.platform,
        vec![profile.renderer],
        TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
        profile.capabilities.to_vec(),
        vec![],
    )
}

fn window_spec(options: Option<&WindowOptions>) -> GuiResult<PlatformWindowSpec> {
    let logical_size = options.map_or_else(
        || Size::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        |window| {
            Size::new(
                initial_dimension(
                    window.width,
                    window.min_width,
                    window.max_width,
                    DEFAULT_WINDOW_WIDTH,
                ),
                initial_dimension(
                    window.height,
                    window.min_height,
                    window.max_height,
                    DEFAULT_WINDOW_HEIGHT,
                ),
            )
        },
    );
    let spec = PlatformWindowSpec {
        id: WINDOW_ID,
        title: options
            .map(|window| window.title.clone())
            .unwrap_or_else(|| DEFAULT_WINDOW_TITLE.to_string()),
        logical_size,
        min_size: partial_min_size(options),
        max_size: partial_max_size(options),
        resizable: options.is_none_or(|window| window.resizable),
        visible: selected_profile()?.window_visible,
    };
    spec.validate()?;
    Ok(spec)
}

fn partial_min_size(options: Option<&WindowOptions>) -> Option<Size> {
    let options = options?;
    if options.min_width.is_none() && options.min_height.is_none() {
        return None;
    }
    Some(Size::new(
        options.min_width.unwrap_or(1.0),
        options.min_height.unwrap_or(1.0),
    ))
}

fn partial_max_size(options: Option<&WindowOptions>) -> Option<Size> {
    let options = options?;
    if options.max_width.is_none() && options.max_height.is_none() {
        return None;
    }
    Some(Size::new(
        options.max_width.unwrap_or(f64::from(f32::MAX)),
        options.max_height.unwrap_or(f64::from(f32::MAX)),
    ))
}

fn initial_dimension(
    value: Option<f64>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    fallback: f64,
) -> f64 {
    if let Some(value) = value {
        return value;
    }
    let value = minimum.map_or(fallback, |minimum| fallback.max(minimum));
    maximum.map_or(value, |maximum| value.min(maximum))
}
