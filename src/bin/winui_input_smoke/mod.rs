mod automation;
mod scenarios;
mod synthetic_pointer;

use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use a3s_gui::{
    ActionInvocation, GuiError, GuiResult, HostNodeId, NativeBackendKind, NativeCapabilities,
    NativeEvent, NativeEventKind, NativeInputConformanceCaseV1, NativeInputConformanceIssueCodeV1,
    NativeInputConformanceManifestV1, NativeInputConformanceObservationV1,
    NativeInputConformanceRunV1, NativeInputConformanceScenarioV1, NativeRole, UiFrame,
    WinUiEventWait, WinUiOsWidget, WinUiRuntimeApp,
};
use automation::TARGET_LABEL;
use scenarios::capture_smoke_run;
use serde_json::json;
use windows_core::Interface;
use winui3::Microsoft::UI::Xaml as xaml;

const WORKER_TIMEOUT: Duration = Duration::from_secs(10);
const EVENT_SETTLE_TIME: Duration = Duration::from_millis(250);
const CAPTURED_BUTTON_CASES: usize = 14;

#[derive(Debug, Default)]
struct FixtureState {
    generation: u32,
    disabled: bool,
    rerender_on_press_start: bool,
}

type FixtureApp = WinUiRuntimeApp<
    FixtureState,
    fn(&FixtureState) -> GuiResult<UiFrame>,
    fn(&mut FixtureState, &ActionInvocation) -> GuiResult<()>,
>;

struct WindowGuard(xaml::Window);

impl Drop for WindowGuard {
    fn drop(&mut self) {
        let _ = self.0.Close();
    }
}

pub(super) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = parse_output_path()?;
    let (run, diagnostics) = capture_smoke_run()?;
    let json = serde_json::to_string_pretty(&run)?;
    if let Some(path) = output {
        std::fs::write(&path, format!("{json}\n"))?;
        eprintln!(
            "wrote partial WinUI OS-input evidence to {}",
            path.display()
        );
    } else {
        println!("{json}");
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(std::io::Error::other(diagnostics.join("; ")).into())
    }
}

fn parse_output_path() -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = args.next().map(PathBuf::from);
    if args.next().is_some() {
        return Err(
            std::io::Error::other("usage: a3s-gui-winui-input-smoke [evidence.json]").into(),
        );
    }
    Ok(output)
}

fn fixture_frame(state: &FixtureState) -> GuiResult<UiFrame> {
    serde_json::from_value(json!({
        "frameId": "winui-native-input-smoke-v1",
        "window": {
            "title": "A3S WinUI Native Input Smoke",
            "width": 360,
            "height": 180,
            "resizable": false
        },
        "actions": [{"id": "recordPress", "label": "Record semantic press event"}],
        "root": {
            "kind": "element",
            "key": format!("native-input-target-{}", state.generation),
            "tag": "Button",
            "props": {
                "label": TARGET_LABEL,
                "disabled": state.disabled,
                "events": {
                    "onPressStart": "recordPress",
                    "onPressUp": "recordPress",
                    "onPressEnd": "recordPress",
                    "onPress": "recordPress"
                }
            }
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid WinUI input fixture: {error}")))
}

fn fixture_reduce(state: &mut FixtureState, invocation: &ActionInvocation) -> GuiResult<()> {
    if state.rerender_on_press_start && invocation.event == NativeEventKind::PressStart {
        state.generation = state.generation.saturating_add(1);
        state.rerender_on_press_start = false;
    }
    Ok(())
}

fn remount_fixture(
    app: &mut FixtureApp,
    disabled: bool,
    rerender_on_press_start: bool,
) -> GuiResult<HostNodeId> {
    let state = app.state_mut();
    state.generation = state.generation.saturating_add(1);
    state.disabled = disabled;
    state.rerender_on_press_start = rerender_on_press_start;
    app.render()?;
    let _ = pump_for(app, EVENT_SETTLE_TIME)?;
    fixture_target(app)
}

fn set_fixture_disabled(app: &mut FixtureApp, disabled: bool) -> GuiResult<()> {
    app.state_mut().disabled = disabled;
    app.render()?;
    let _ = pump_for(app, EVENT_SETTLE_TIME)?;
    Ok(())
}

fn fixture_target(app: &FixtureApp) -> GuiResult<HostNodeId> {
    app.runtime()
        .host()
        .executor()
        .driver()
        .handles()
        .iter()
        .find_map(|(id, handle)| matches!(handle.widget, WinUiOsWidget::Button(_)).then_some(*id))
        .ok_or_else(|| GuiError::host("WinUI input fixture did not mount its button"))
}

fn fixture_handles(app: &FixtureApp) -> GuiResult<(HostNodeId, xaml::Window, isize)> {
    let handles = app.runtime().host().executor().driver().handles();
    let target = fixture_target(app)?;
    let window = handles
        .values()
        .find_map(|handle| match &handle.widget {
            WinUiOsWidget::Window(window) => Some(window.clone()),
            _ => None,
        })
        .ok_or_else(|| GuiError::host("WinUI input fixture did not mount its window"))?;
    let native: winui3::IWindowNative = window.cast().map_err(|error| {
        GuiError::host(format!(
            "failed to read WinUI window native interface: {error}"
        ))
    })?;
    let hwnd = unsafe { native.WindowHandle() }.map_err(|error| {
        GuiError::host(format!("failed to read WinUI input fixture HWND: {error}"))
    })?;
    if hwnd.is_invalid() {
        return Err(GuiError::host(
            "WinUI input fixture returned an invalid HWND",
        ));
    }
    Ok((target, window, hwnd.0 as isize))
}

fn run_scenario(
    app: &mut FixtureApp,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    capture_scenario_result(app, target, scenario, worker, None, diagnostics)
}

fn run_scenario_with_pointer_release(
    app: &mut FixtureApp,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    capture_scenario_result(app, target, scenario, worker, Some(target), diagnostics)
}

fn capture_scenario_result(
    app: &mut FixtureApp,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    release_on_press_start: Option<HostNodeId>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let (events, result) = collect_worker_events(app, worker, release_on_press_start)?;
    let stimulus_dispatched = result.is_ok();
    if let Err(error) = result {
        diagnostics.push(format!("{scenario:?}: {error}"));
    }
    Ok(NativeInputConformanceObservationV1::capture(
        NativeInputConformanceCaseV1::new(NativeRole::Button, scenario),
        target,
        stimulus_dispatched,
        &events,
    ))
}

fn collect_worker_events(
    app: &mut FixtureApp,
    worker: JoinHandle<Result<(), String>>,
    mut release_on_press_start: Option<HostNodeId>,
) -> GuiResult<(Vec<NativeEvent>, Result<(), String>)> {
    let started = Instant::now();
    let mut events = Vec::new();
    let mut intervention_error = None;
    while !worker.is_finished() {
        let batch = pump_once(app)?;
        let release_target = release_on_press_start.filter(|target| {
            batch
                .iter()
                .any(|event| event.node == *target && event.kind == NativeEventKind::PressStart)
        });
        events.extend(batch);
        if let Some(target) = release_target {
            if let Err(error) = release_pointer_captures(app, target) {
                intervention_error = Some(error.to_string());
            }
            release_on_press_start = None;
        }
        if started.elapsed() >= WORKER_TIMEOUT {
            return Err(GuiError::host(format!(
                "Windows input automation worker exceeded {} seconds",
                WORKER_TIMEOUT.as_secs()
            )));
        }
        thread::park_timeout(Duration::from_millis(2));
    }
    let mut result = worker
        .join()
        .map_err(|_| GuiError::host("Windows input automation worker panicked"))?;
    let intervention_error = intervention_error.or_else(|| {
        release_on_press_start.map(|_| {
            "mouse cancellation did not observe PressStart before the input worker finished"
                .to_string()
        })
    });
    if let Some(error) = intervention_error {
        result = Err(match result {
            Ok(()) => error,
            Err(worker_error) => format!("{worker_error}; {error}"),
        });
    }
    events.extend(pump_for(app, EVENT_SETTLE_TIME)?);
    Ok((events, result))
}

fn release_pointer_captures(app: &FixtureApp, target: HostNodeId) -> GuiResult<()> {
    let handle = app
        .runtime()
        .host()
        .executor()
        .driver()
        .handles()
        .get(&target)
        .ok_or_else(|| GuiError::host("mouse cancellation target was unmounted too early"))?;
    let WinUiOsWidget::Button(button) = &handle.widget else {
        return Err(GuiError::host(
            "mouse cancellation target is not a WinUI Button",
        ));
    };
    button.ReleasePointerCaptures().map_err(|error| {
        GuiError::host(format!(
            "failed to release WinUI Button pointer capture: {error}"
        ))
    })
}

fn pump_for(app: &mut FixtureApp, duration: Duration) -> GuiResult<Vec<NativeEvent>> {
    let deadline = Instant::now() + duration;
    let mut events = Vec::new();
    while Instant::now() < deadline {
        events.extend(pump_once(app)?);
        thread::park_timeout(Duration::from_millis(2));
    }
    Ok(events)
}

fn pump_once(app: &mut FixtureApp) -> GuiResult<Vec<NativeEvent>> {
    Ok(app
        .pump_winui_event_batch(WinUiEventWait::Poll)?
        .responses
        .into_iter()
        .map(|response| response.event)
        .collect())
}

fn validate_partial_smoke(run: &NativeInputConformanceRunV1, diagnostics: &mut Vec<String>) {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );
    let report = manifest.verify(run);
    let unexpected = report
        .issues
        .iter()
        .filter(|issue| issue.code != NativeInputConformanceIssueCodeV1::MissingObservation)
        .map(|issue| issue.message.clone())
        .collect::<Vec<_>>();
    if report.verified_cases != run.observations.len() || !unexpected.is_empty() {
        diagnostics.push(format!(
            "partial smoke validation verified {} of {} captured cases{}",
            report.verified_cases,
            run.observations.len(),
            if unexpected.is_empty() {
                String::new()
            } else {
                format!(": {}", unexpected.join(", "))
            }
        ));
    }
}

#[cfg(test)]
mod tests;
