#[allow(dead_code)]
#[path = "support/calculator/mod.rs"]
mod calculator;

use a3s_gui::{
    GuiError, GuiResult, NativeInputModality, NativeKeyModifiers, PlatformAccessibilityNode,
    PlatformHostEvent, PlatformInputDeviceId, PlatformInputEvent, PlatformPoint,
    PlatformPointerButton, PlatformPointerEvent, PlatformPointerId, PlatformPointerPhase,
    PlatformWindowId, PlatformWindowSpec, RecordingPlatformHost, ReferenceScenePresenter,
    RsxCompilerBridge, SelfDrawnActionPropagation, SelfDrawnHostEventOutcome,
    SelfDrawnWindowRuntime, Size, WindowOptions,
};

const WINDOW_ID: PlatformWindowId = PlatformWindowId::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
struct SmokeResult {
    revision: u64,
    final_revision: u64,
    event_sequence: u64,
    action_invocations: usize,
    layout_fingerprint: u64,
    scene_fingerprint: u64,
    width: u32,
    height: u32,
    pixel_bytes: usize,
    display: String,
}

fn run_smoke() -> GuiResult<SmokeResult> {
    let component = calculator::shared_calculator_component("layout-calculator", "A3S Calculator")?;
    let mut state = calculator::CalculatorState::default();
    let frame = calculator::calculator_frame(&component, &state)?;
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
    let initial = runtime
        .snapshot()
        .ok_or_else(|| GuiError::host("self-drawn calculator did not commit a frame"))?;
    let pixels = runtime
        .presenter()
        .committed()
        .ok_or_else(|| GuiError::host("self-drawn calculator did not publish Graphics pixels"))?;
    let layout_fingerprint = initial.layout_fingerprint();
    let scene_fingerprint = initial.scene_fingerprint();
    let (width, height, pixel_bytes) = (pixels.width(), pixels.height(), pixels.rgba8().len());
    let mut timestamp_micros = 1_u64;
    let mut action_invocations = 0;
    for label in ["7", "+", "3", "="] {
        activate(
            &mut runtime,
            &component,
            &mut state,
            window,
            label,
            &mut timestamp_micros,
            &mut action_invocations,
        )?;
    }
    let final_revision = runtime
        .snapshot()
        .ok_or_else(|| GuiError::host("interactive calculator lost its committed frame"))?
        .revision()
        .get();
    Ok(SmokeResult {
        revision: commit.revision.get(),
        final_revision,
        event_sequence: runtime.event_sequence(),
        action_invocations,
        layout_fingerprint,
        scene_fingerprint,
        width,
        height,
        pixel_bytes,
        display: state.display().to_string(),
    })
}

#[allow(clippy::too_many_arguments)]
fn activate(
    runtime: &mut SelfDrawnWindowRuntime<RecordingPlatformHost, ReferenceScenePresenter>,
    component: &calculator::CalculatorComponent,
    state: &mut calculator::CalculatorState,
    window: &WindowOptions,
    label: &str,
    timestamp_micros: &mut u64,
    action_invocations: &mut usize,
) -> GuiResult<()> {
    let position = action_position(runtime, label)?;
    for phase in [
        PlatformPointerPhase::Pressed,
        PlatformPointerPhase::Released,
    ] {
        let event = PlatformHostEvent::Input {
            event: PlatformInputEvent::Pointer {
                event: PlatformPointerEvent {
                    window: WINDOW_ID,
                    device: PlatformInputDeviceId::new(1),
                    pointer: PlatformPointerId::new(1),
                    modality: NativeInputModality::Mouse,
                    phase,
                    position,
                    button: Some(PlatformPointerButton::Primary),
                    pressed_buttons: u32::from(phase == PlatformPointerPhase::Pressed),
                    pressure: None,
                    modifiers: NativeKeyModifiers::new(),
                    timestamp_micros: *timestamp_micros,
                },
            },
        };
        *timestamp_micros = timestamp_micros.saturating_add(1);
        let outcome = runtime.handle_event_with_reducer(event, |invocation| {
            calculator::calculator_reduce_self_drawn(component, state, invocation)?;
            Ok(SelfDrawnActionPropagation::Continue)
        })?;
        if let SelfDrawnHostEventOutcome::Input(dispatch) = outcome {
            *action_invocations = action_invocations.saturating_add(dispatch.invocations.len());
        }
    }
    let frame = calculator::calculator_frame(component, state)?;
    let content = RsxCompilerBridge::new().lower_to_native(&frame.root)?;
    runtime.render(window.wrap_native_root(&frame.frame_id, content))?;
    Ok(())
}

fn action_position(
    runtime: &SelfDrawnWindowRuntime<RecordingPlatformHost, ReferenceScenePresenter>,
    label: &str,
) -> GuiResult<PlatformPoint> {
    let root = runtime
        .snapshot()
        .and_then(|snapshot| snapshot.accessibility().root.as_ref())
        .ok_or_else(|| GuiError::host("self-drawn calculator has no accessibility root"))?;
    let node = find_label(root, label)
        .ok_or_else(|| GuiError::host(format!("calculator action {label:?} was not mounted")))?;
    Ok(PlatformPoint::new(
        node.logical_bounds.x + node.logical_bounds.width / 2.0,
        node.logical_bounds.y + node.logical_bounds.height / 2.0,
    ))
}

fn find_label<'a>(
    node: &'a PlatformAccessibilityNode,
    label: &str,
) -> Option<&'a PlatformAccessibilityNode> {
    if node.label.as_deref() == Some(label) {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_label(child, label))
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
        "self-drawn calculator committed revisions {}..{} at {}x{}, routed {} actions across {} raw events, and displayed {} (layout {}, scene {}, {} pixel bytes)",
        result.revision,
        result.final_revision,
        result.width,
        result.height,
        result.action_invocations,
        result.event_sequence,
        result.display,
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
        assert_eq!(result.final_revision, 5);
        assert_eq!(result.event_sequence, 8);
        assert_eq!(result.action_invocations, 4);
        assert_eq!(result.display, "10");
        assert_eq!(result.layout_fingerprint, 16_529_597_026_056_060_935);
        assert_eq!(result.scene_fingerprint, 2_100_550_662_756_266_801);
        assert_eq!((result.width, result.height), (410, 620));
        assert_eq!(result.pixel_bytes, 410 * 620 * 4);
    }
}
