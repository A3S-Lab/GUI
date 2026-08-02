#![cfg(all(feature = "authoring", feature = "software-reference"))]

#[path = "../examples/support/calculator/mod.rs"]
mod calculator;

use a3s_gui::drawing::{scene_from_layout, Color, LayoutSceneOptions, ReferenceRenderer};
use a3s_gui::layout::{layout_native_tree, LayoutDiagnosticCode, LayoutOptions, LayoutSnapshot};
use a3s_gui::{NativeRole, RsxCompilerBridge, Size};

fn shared_calculator_layout() -> LayoutSnapshot {
    let component =
        calculator::shared_calculator_component("layout-calculator", "A3S Calculator").unwrap();
    let frame =
        calculator::calculator_frame(&component, &calculator::CalculatorState::default()).unwrap();
    let content = RsxCompilerBridge::new()
        .lower_to_native(&frame.root)
        .unwrap();
    let native = frame
        .window
        .as_ref()
        .unwrap()
        .wrap_native_root(&frame.frame_id, content);
    layout_native_tree(&native, LayoutOptions::boxes_only(Size::new(410.0, 620.0))).unwrap()
}

#[test]
fn shared_calculator_native_tree_produces_a_retained_rectangle_scene() {
    let layout = shared_calculator_layout();

    assert!(!layout.has_errors(), "{:?}", layout.diagnostics);
    assert!(layout.nodes.len() >= 40);
    assert_eq!(layout.nodes[0].role, NativeRole::Window);
    assert_eq!(layout.nodes[0].border_box.width, 410.0);
    assert_eq!(layout.nodes[0].border_box.height, 620.0);
    assert!(layout
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == LayoutDiagnosticCode::DeferredRole));
    assert!(layout
        .nodes
        .iter()
        .filter(|node| node.role == NativeRole::Button)
        .all(|node| node.paint.background.is_some()));

    let scene = scene_from_layout(
        &layout,
        LayoutSceneOptions {
            scale_factor: 1.0,
            clear_color: Color::TRANSPARENT,
            ..LayoutSceneOptions::default()
        },
    )
    .unwrap();
    let scene_fingerprint = scene.fingerprint().unwrap();
    assert_eq!(layout.fingerprint().unwrap(), 11_433_846_600_555_364_104);
    assert_eq!(scene_fingerprint, 2_100_550_662_756_266_801);
    let mut renderer = ReferenceRenderer::new();
    let (first_width, first_height, first_damage, first_pixels) = {
        let first = renderer.render(scene.clone()).unwrap();
        (
            first.width(),
            first.height(),
            first.damage(),
            first.rgba8().to_vec(),
        )
    };
    let second = renderer.render(scene).unwrap();

    assert_eq!((first_width, first_height), (410, 620));
    assert!(first_damage.full_repaint);
    assert!(!second.damage().requires_render());
    assert_eq!(second.rgba8(), first_pixels);
    assert_eq!(second.fingerprint(), scene_fingerprint);
    assert_eq!(&first_pixels[0..4], &[243, 243, 243, 255]);
    let outside_shell = ((409 + 619 * 410) * 4) as usize;
    assert_eq!(
        &first_pixels[outside_shell..outside_shell + 4],
        &[0, 0, 0, 0]
    );
}

#[cfg(feature = "gpu")]
#[test]
fn shared_calculator_gpu_output_matches_the_software_snapshot() {
    use a3s_gui::drawing::{GpuPowerPreference, GpuRendererOptions, GpuSceneRenderer};

    let layout = shared_calculator_layout();
    let scene = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap();
    let software_pixels = {
        let mut renderer = ReferenceRenderer::new();
        renderer.render(scene.clone()).unwrap().rgba8().to_vec()
    };
    let options = GpuRendererOptions {
        power_preference: GpuPowerPreference::None,
        allow_software_adapter: true,
        ..Default::default()
    };
    let mut renderer = match pollster::block_on(GpuSceneRenderer::request(options)) {
        Ok(renderer) => renderer,
        Err(error) if error.to_string().contains("no compatible GPU adapter") => return,
        Err(error) => panic!("failed to initialize calculator GPU snapshot: {error}"),
    };

    let frame = pollster::block_on(renderer.render(scene)).unwrap();
    let readback = pollster::block_on(renderer.request_readback())
        .unwrap()
        .finish()
        .unwrap();

    assert!(frame.rendered);
    assert_eq!((frame.width, frame.height), (410, 620));
    assert_eq!((readback.width(), readback.height()), (410, 620));
    let differences = readback
        .as_rgba8()
        .chunks_exact(4)
        .zip(software_pixels.chunks_exact(4))
        .enumerate()
        .filter_map(|(index, (gpu, software))| (gpu != software).then_some((index, gpu, software)))
        .collect::<Vec<_>>();
    let max_channel_delta = differences
        .iter()
        .flat_map(|(_, gpu, software)| {
            gpu.iter()
                .zip(*software)
                .map(|(gpu, software)| gpu.abs_diff(*software))
        })
        .max()
        .unwrap_or(0);
    let allowed_different_pixels = (410 * 620) / 200;
    assert!(
        differences.len() <= allowed_different_pixels && max_channel_delta <= 96,
        "GPU parity drift exceeded the 0.5%/96 thresholds: {} pixels differ, max channel delta {}, first difference {:?}",
        differences.len(),
        max_channel_delta,
        differences.first()
    );
    assert_eq!(readback.pixel(0, 0), Some(Color::rgba(243, 243, 243, 255)));
    assert_eq!(readback.pixel(409, 619), Some(Color::TRANSPARENT));
    assert_eq!(readback.pixel(53, 378), Some(Color::WHITE));
    assert_eq!(readback.pixel(344, 555), Some(Color::rgb(0, 103, 192)));
}
