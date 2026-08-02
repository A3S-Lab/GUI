use std::io::{self, Read, Write};

use a3s_gui::tsx_protocol::{
    read_tsx_json_frame_v1, write_tsx_json_frame_v1, TsxClientMessageV1, TsxFrameLimitsV1,
    TsxHostApplicationSessionV1, TsxHostCapabilityV1, TsxHostHandshakeConfigV1, TsxHostHandshakeV1,
    TsxHostMessageV1, TsxHostPlatformV1, TsxRendererV1, TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
};
use a3s_gui::{
    GuiError, GuiResult, PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost,
    ReferenceScenePresenter, RsxCompilerBridge, SelfDrawnWindowRuntime, Size, UiFrame,
    WindowOptions,
};

const DEFAULT_WINDOW_WIDTH: f64 = 800.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;
const DEFAULT_WINDOW_TITLE: &str = "A3S GUI";
const WINDOW_ID: PlatformWindowId = PlatformWindowId::new(1);

type HeadlessRuntime = SelfDrawnWindowRuntime<RecordingPlatformHost, ReferenceScenePresenter>;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(error) = serve(stdin.lock(), stdout.lock()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn serve<R, W>(mut input: R, mut output: W) -> GuiResult<()>
where
    R: Read,
    W: Write,
{
    let hard_limits = TsxFrameLimitsV1::default();
    let hello = read_tsx_json_frame_v1::<_, TsxClientMessageV1>(&mut input, hard_limits)?
        .ok_or_else(|| GuiError::host("TSX host input ended before the required hello message"))?;
    let mut handshake = TsxHostHandshakeV1::new(host_config()?);
    let welcome = handshake.accept_hello(&hello)?;
    let negotiated = handshake
        .negotiated()
        .cloned()
        .ok_or_else(|| GuiError::host("TSX host handshake did not produce a session"))?;
    let session_limits = TsxFrameLimitsV1::new(negotiated.limits().maximum_frame_bytes)?;
    write_tsx_json_frame_v1(&mut output, &welcome, session_limits)?;

    let mut session = TsxHostApplicationSessionV1::new(&negotiated)?;
    let mut runtime = None;
    let result = serve_session(
        &mut input,
        &mut output,
        session_limits,
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

fn serve_session<R, W>(
    input: &mut R,
    output: &mut W,
    limits: TsxFrameLimitsV1,
    session: &mut TsxHostApplicationSessionV1,
    runtime: &mut Option<HeadlessRuntime>,
) -> GuiResult<()>
where
    R: Read,
    W: Write,
{
    while let Some(message) = read_tsx_json_frame_v1::<_, TsxClientMessageV1>(input, limits)? {
        match &message {
            TsxClientMessageV1::Render { .. } => {
                let committed = render_message(session, &message, runtime)?;
                write_tsx_json_frame_v1(output, &committed, limits)?;
            }
            TsxClientMessageV1::Close { .. } => {
                let closed = session
                    .accept_control(&message)?
                    .ok_or_else(|| GuiError::host("TSX close message did not produce a reply"))?;
                write_tsx_json_frame_v1(output, &closed, limits)?;
                return Ok(());
            }
            TsxClientMessageV1::Ping { .. } | TsxClientMessageV1::Pong { .. } => {
                if let Some(response) = session.accept_control(&message)? {
                    write_tsx_json_frame_v1(output, &response, limits)?;
                }
            }
            TsxClientMessageV1::Hello { .. } => {
                return Err(GuiError::host(
                    "TSX protocol hello is only valid as the first client message",
                ));
            }
            _ => {
                return Err(GuiError::host(
                    "TSX host received an unsupported protocol-v1 client message",
                ));
            }
        }
    }
    Ok(())
}

fn render_message(
    session: &mut TsxHostApplicationSessionV1,
    message: &TsxClientMessageV1,
    runtime: &mut Option<HeadlessRuntime>,
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
    runtime: &mut Option<HeadlessRuntime>,
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
        slot @ None => slot.insert(HeadlessRuntime::new(
            RecordingPlatformHost::new(),
            ReferenceScenePresenter::new(),
            desired_spec,
            1.0,
        )?),
    };
    runtime.render(native_root)?;
    let snapshot = runtime
        .snapshot()
        .ok_or_else(|| GuiError::host("self-drawn runtime did not commit a TSX snapshot"))?;
    session.commit_self_drawn_snapshot(frame_id, snapshot, Vec::new())
}

fn host_config() -> GuiResult<TsxHostHandshakeConfigV1> {
    TsxHostHandshakeConfigV1::new(
        env!("CARGO_PKG_VERSION"),
        option_env!("A3S_GUI_BUILD_ID").unwrap_or("development"),
        TsxHostPlatformV1::Headless,
        vec![TsxRendererV1::Software],
        TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
        vec![
            TsxHostCapabilityV1::HeadlessRendering,
            TsxHostCapabilityV1::SelfDrawnRendering,
        ],
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
        visible: false,
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
