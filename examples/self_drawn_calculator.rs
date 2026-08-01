// H1 is render-only until portable event routing lands. The shared support
// module also exports its reducer for interactive legacy examples.
#[allow(dead_code)]
#[path = "support/calculator/mod.rs"]
mod calculator;

use a3s_gui::{
    GuiError, GuiResult, PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost,
    ReferenceScenePresenter, RsxCompilerBridge, SelfDrawnWindowRuntime, Size, WindowOptions,
};

const WINDOW_ID: PlatformWindowId = PlatformWindowId::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SmokeResult {
    revision: u64,
    layout_fingerprint: u64,
    scene_fingerprint: u64,
    width: u32,
    height: u32,
    pixel_bytes: usize,
}

fn run_smoke() -> GuiResult<SmokeResult> {
    let component = calculator::shared_calculator_component("layout-calculator", "A3S Calculator")?;
    let frame = calculator::calculator_frame(&component, &calculator::CalculatorState::default())?;
    frame.validate()?;
    let window = frame
        .window
        .as_ref()
        .ok_or_else(|| GuiError::host("the shared self-drawn calculator needs window options"))?;
    let content = RsxCompilerBridge::new().lower_to_native(&frame.root)?;
    let native_root = window.wrap_native_root(&frame.frame_id, content);
    let mut runtime = SelfDrawnWindowRuntime::new(
        RecordingPlatformHost::new(),
        ReferenceScenePresenter::new(),
        window_spec(window)?,
        1.0,
    )?;

    let commit = runtime.render(native_root)?;
    let snapshot = runtime
        .snapshot()
        .ok_or_else(|| GuiError::host("self-drawn calculator did not commit a frame"))?;
    let pixels = runtime
        .presenter()
        .committed()
        .ok_or_else(|| GuiError::host("self-drawn calculator did not publish Graphics pixels"))?;
    Ok(SmokeResult {
        revision: commit.revision.get(),
        layout_fingerprint: snapshot.layout_fingerprint(),
        scene_fingerprint: snapshot.scene_fingerprint(),
        width: pixels.width(),
        height: pixels.height(),
        pixel_bytes: pixels.rgba8().len(),
    })
}

fn window_spec(options: &WindowOptions) -> GuiResult<PlatformWindowSpec> {
    let logical_size = paired_size("window size", options.width, options.height)?
        .unwrap_or_else(|| Size::new(800.0, 600.0));
    Ok(PlatformWindowSpec {
        id: WINDOW_ID,
        title: options.title.clone(),
        logical_size,
        min_size: paired_size("minimum window size", options.min_width, options.min_height)?,
        max_size: paired_size("maximum window size", options.max_width, options.max_height)?,
        resizable: options.resizable,
        visible: true,
    })
}

fn paired_size(name: &str, width: Option<f64>, height: Option<f64>) -> GuiResult<Option<Size>> {
    match (width, height) {
        (None, None) => Ok(None),
        (Some(width), Some(height)) => Ok(Some(Size::new(width, height))),
        _ => Err(GuiError::host(format!(
            "self-drawn {name} needs both width and height"
        ))),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = run_smoke()?;
    println!(
        "self-drawn calculator committed revision {} at {}x{} (layout {}, scene {}, {} pixel bytes)",
        result.revision,
        result.width,
        result.height,
        result.layout_fingerprint,
        result.scene_fingerprint,
        result.pixel_bytes
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_calculator_commits_through_the_zero_widget_runtime() {
        let result = run_smoke().unwrap();

        assert_eq!(result.revision, 1);
        assert_eq!(result.layout_fingerprint, 16_529_597_026_056_060_935);
        assert_eq!(result.scene_fingerprint, 2_100_550_662_756_266_801);
        assert_eq!((result.width, result.height), (410, 620));
        assert_eq!(result.pixel_bytes, 410 * 620 * 4);
    }
}
