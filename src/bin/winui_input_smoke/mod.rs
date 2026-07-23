mod automation;

use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use a3s_gui::{
    ActionInvocation, GuiError, GuiResult, HostNodeId, NativeBackendKind, NativeCapabilities,
    NativeEvent, NativeInputConformanceCaseV1, NativeInputConformanceIssueCodeV1,
    NativeInputConformanceManifestV1, NativeInputConformanceObservationV1,
    NativeInputConformanceRunV1, NativeInputConformanceScenarioV1, NativeInputEvidenceSourceV1,
    NativeRole, UiFrame, WinUiEventWait, WinUiOsWidget, WinUiRuntimeApp,
};
use automation::{
    spawn_assistive_activation, spawn_keyboard_activation, spawn_mouse_activation,
    windows_environment, TARGET_LABEL,
};
use serde_json::json;
use windows_core::Interface;
use winui3::Microsoft::UI::Xaml as xaml;

const WORKER_TIMEOUT: Duration = Duration::from_secs(10);
const EVENT_SETTLE_TIME: Duration = Duration::from_millis(250);

#[derive(Debug, Default)]
struct FixtureState;

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

fn capture_smoke_run() -> GuiResult<(NativeInputConformanceRunV1, Vec<String>)> {
    let mut app: FixtureApp = WinUiRuntimeApp::winui(
        FixtureState,
        fixture_frame as fn(&FixtureState) -> GuiResult<UiFrame>,
        fixture_reduce as fn(&mut FixtureState, &ActionInvocation) -> GuiResult<()>,
    )?;
    app.render()?;
    let (target, window, hwnd_value) = fixture_handles(&app)?;
    let _window_guard = WindowGuard(window);
    let _ = pump_for(&mut app, EVENT_SETTLE_TIME)?;

    let mut diagnostics = Vec::new();
    let mut observations = Vec::new();
    observations.push(run_scenario(
        &mut app,
        target,
        NativeInputConformanceScenarioV1::MouseActivation,
        spawn_mouse_activation(hwnd_value),
        &mut diagnostics,
    )?);

    app.runtime_mut().request_focus(target)?;
    let _ = pump_for(&mut app, Duration::from_millis(100))?;
    observations.push(run_scenario(
        &mut app,
        target,
        NativeInputConformanceScenarioV1::KeyboardActivation,
        spawn_keyboard_activation(hwnd_value),
        &mut diagnostics,
    )?);

    observations.push(run_scenario(
        &mut app,
        target,
        NativeInputConformanceScenarioV1::AssistiveActivation,
        spawn_assistive_activation(hwnd_value),
        &mut diagnostics,
    )?);

    let run = NativeInputConformanceRunV1::new(
        NativeBackendKind::WinUI,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    )
    .environment(windows_environment()?)
    .observations(observations);
    validate_partial_smoke(&run, &mut diagnostics);
    Ok((run, diagnostics))
}

fn fixture_frame(_: &FixtureState) -> GuiResult<UiFrame> {
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
            "key": "native-input-target",
            "tag": "Button",
            "props": {
                "label": TARGET_LABEL,
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

fn fixture_reduce(_: &mut FixtureState, _: &ActionInvocation) -> GuiResult<()> {
    Ok(())
}

fn fixture_handles(app: &FixtureApp) -> GuiResult<(HostNodeId, xaml::Window, isize)> {
    let handles = app.runtime().host().executor().driver().handles();
    let target = handles
        .iter()
        .find_map(|(id, handle)| matches!(handle.widget, WinUiOsWidget::Button(_)).then_some(*id))
        .ok_or_else(|| GuiError::host("WinUI input fixture did not mount its button"))?;
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
    let (events, result) = collect_worker_events(app, worker)?;
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
) -> GuiResult<(Vec<NativeEvent>, Result<(), String>)> {
    let started = Instant::now();
    let mut events = Vec::new();
    while !worker.is_finished() {
        events.extend(pump_once(app)?);
        if started.elapsed() >= WORKER_TIMEOUT {
            return Err(GuiError::host(format!(
                "Windows input automation worker exceeded {} seconds",
                WORKER_TIMEOUT.as_secs()
            )));
        }
        thread::park_timeout(Duration::from_millis(2));
    }
    let result = worker
        .join()
        .map_err(|_| GuiError::host("Windows input automation worker panicked"))?;
    events.extend(pump_for(app, EVENT_SETTLE_TIME)?);
    Ok((events, result))
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
mod tests {
    use super::*;
    use a3s_gui::{
        NativeEventContext, NativeEventKind, NativeInputConformanceEnvironmentV1,
        NativeInputModality, NativeOperatingSystemV1,
    };

    #[test]
    fn partial_validation_accepts_only_missing_uncaptured_cases() {
        let target = HostNodeId::new(7);
        let observations = vec![
            valid_activation_observation(
                target,
                NativeInputConformanceScenarioV1::MouseActivation,
                NativeInputModality::Mouse,
                false,
                1,
            ),
            valid_activation_observation(
                target,
                NativeInputConformanceScenarioV1::KeyboardActivation,
                NativeInputModality::Keyboard,
                true,
                0,
            ),
            valid_activation_observation(
                target,
                NativeInputConformanceScenarioV1::AssistiveActivation,
                NativeInputModality::Virtual,
                false,
                0,
            ),
        ];
        let run = NativeInputConformanceRunV1::new(
            NativeBackendKind::WinUI,
            NativeInputEvidenceSourceV1::OperatingSystemAutomation,
        )
        .environment(NativeInputConformanceEnvironmentV1::new(
            NativeOperatingSystemV1::Windows,
            "test-windows",
            "test-winui",
            "test-os-driver",
        ))
        .observations(observations);

        let mut diagnostics = Vec::new();
        validate_partial_smoke(&run, &mut diagnostics);
        assert!(diagnostics.is_empty());

        let mut broken = run;
        broken.observations[0].events.remove(1);
        validate_partial_smoke(&broken, &mut diagnostics);
        assert_eq!(diagnostics.len(), 1);
    }

    fn valid_activation_observation(
        target: HostNodeId,
        scenario: NativeInputConformanceScenarioV1,
        modality: NativeInputModality,
        handled_activation: bool,
        click_count: u8,
    ) -> NativeInputConformanceObservationV1 {
        let context = NativeEventContext::new()
            .modality(modality)
            .handled_activation(handled_activation)
            .click_count(click_count);
        let events = [
            NativeEventKind::PressStart,
            NativeEventKind::PressUp,
            NativeEventKind::PressEnd,
            NativeEventKind::Press,
        ]
        .into_iter()
        .map(|kind| NativeEvent::new(target, kind).context(context))
        .collect::<Vec<_>>();
        NativeInputConformanceObservationV1::capture(
            NativeInputConformanceCaseV1::new(NativeRole::Button, scenario),
            target,
            true,
            &events,
        )
    }
}
