#![cfg(all(
    target_os = "windows",
    feature = "host-windows",
    feature = "platform-runtime",
    feature = "gpu",
    feature = "software-reference"
))]

use a3s_gui::drawing::{
    Color, GpuBackend, GpuPowerPreference, GpuRendererOptions, ReferenceRenderer,
};
use a3s_gui::platform_host::{PlatformWindowId, PlatformWindowSpec, WindowsPlatformHost};
use a3s_gui::platform_runtime::{
    GpuScenePresenter, SelfDrawnFrameCommitStatus, SelfDrawnWindowRuntime,
};
use a3s_gui::tsx_protocol::{decode_tsx_json_payload_v1, TsxClientMessageV1, TsxFrameLimitsV1};
use a3s_gui::{RsxCompilerBridge, Size, UiFrame};

const CALCULATOR_RENDER_FIXTURE: &str =
    include_str!("fixtures/tsx-protocol/render-calculator-v1.json");
const WINDOW: PlatformWindowId = PlatformWindowId::new(1);

#[test]
fn tsx_calculator_captures_the_exact_presented_dx12_surface() {
    let frame = calculator_frame();
    let window = frame.window.as_ref().unwrap();
    let content = RsxCompilerBridge::new()
        .lower_to_native(&frame.root)
        .unwrap();
    let native = window.wrap_native_root(&frame.frame_id, content);
    let host = WindowsPlatformHost::new().unwrap();
    let scale_factor = host.initial_scale_factor().unwrap();
    let presenter = GpuScenePresenter::with_options(GpuRendererOptions {
        power_preference: GpuPowerPreference::None,
        allow_software_adapter: true,
        ..GpuRendererOptions::default()
    });
    let mut runtime = SelfDrawnWindowRuntime::new(
        host,
        presenter,
        PlatformWindowSpec {
            id: WINDOW,
            title: window.title.clone(),
            logical_size: Size::new(window.width.unwrap(), window.height.unwrap()),
            min_size: Some(Size::new(
                window.min_width.unwrap(),
                window.min_height.unwrap(),
            )),
            max_size: None,
            resizable: window.resizable,
            visible: true,
        },
        scale_factor,
    )
    .unwrap();
    runtime
        .presenter_mut()
        .request_next_frame_capture()
        .unwrap();

    let commit = runtime.render(native).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Committed);
    assert!(!runtime.presenter().capture_pending());
    assert!(runtime.presenter().capture_failure().is_none());
    assert_eq!(
        runtime.presenter().capabilities().unwrap().backend,
        GpuBackend::Direct3d12
    );
    let snapshot = runtime.snapshot().unwrap();
    let software_pixels = ReferenceRenderer::new()
        .render(snapshot.scene().clone())
        .unwrap()
        .rgba8()
        .to_vec();
    let capture = runtime.presenter().captured().unwrap();
    let image = capture.image();

    assert!(capture.gpu().presented);
    assert_eq!(
        capture.frame().layout_fingerprint,
        snapshot.layout_fingerprint()
    );
    assert_eq!(
        capture.frame().scene_fingerprint,
        snapshot.scene_fingerprint()
    );
    assert_eq!(capture.gpu().fingerprint, snapshot.scene_fingerprint());
    assert_eq!(
        (image.width(), image.height()),
        (capture.gpu().width, capture.gpu().height)
    );
    assert_eq!(image.as_rgba8().len(), software_pixels.len());

    let differences = image
        .as_rgba8()
        .chunks_exact(4)
        .zip(software_pixels.chunks_exact(4))
        .filter(|(gpu, software)| gpu != software)
        .count();
    let max_channel_delta = image
        .as_rgba8()
        .iter()
        .zip(&software_pixels)
        .map(|(gpu, software)| gpu.abs_diff(*software))
        .max()
        .unwrap_or(0);
    let pixel_count = u64::from(image.width()) * u64::from(image.height());
    assert!(
        (differences as u64) * 200 <= pixel_count && max_channel_delta <= 96,
        "DX12 surface parity drift exceeded 0.5%/96: {differences}/{pixel_count} pixels, max channel delta {max_channel_delta}"
    );
    let rgba_fingerprint = stable_bytes_fingerprint(image.as_rgba8());
    eprintln!(
        "a3s-dx12-capture width={} height={} scale={} layout={:016x} scene={:016x} differing_pixels={} max_channel_delta={} rgba={rgba_fingerprint:016x}",
        image.width(),
        image.height(),
        scale_factor,
        snapshot.layout_fingerprint(),
        snapshot.scene_fingerprint(),
        differences,
        max_channel_delta,
    );
    if let Some(path) = std::env::var_os("A3S_CAPTURE_RGBA_PATH") {
        std::fs::write(path, image.as_rgba8()).unwrap();
    }

    assert_eq!(image.pixel(0, 0), Some(Color::rgba(243, 243, 243, 255)));
    assert_eq!(
        image.pixel(image.width() - 1, image.height() - 1),
        Some(Color::TRANSPARENT)
    );
    assert_eq!(
        image.pixel(physical(53.0, scale_factor), physical(378.0, scale_factor)),
        Some(Color::WHITE)
    );
    assert_eq!(
        image.pixel(physical(344.0, scale_factor), physical(555.0, scale_factor)),
        Some(Color::rgb(0, 103, 192))
    );

    runtime.shutdown().unwrap();
}

fn calculator_frame() -> UiFrame {
    let message = decode_tsx_json_payload_v1::<TsxClientMessageV1>(
        CALCULATOR_RENDER_FIXTURE.trim().as_bytes(),
        TsxFrameLimitsV1::new(1_048_576).unwrap(),
    )
    .unwrap();
    let TsxClientMessageV1::Render { payload, .. } = message else {
        panic!("expected calculator render fixture")
    };
    payload.try_into().unwrap()
}

fn physical(logical: f64, scale_factor: f64) -> u32 {
    (logical * scale_factor).round() as u32
}

fn stable_bytes_fingerprint(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}
