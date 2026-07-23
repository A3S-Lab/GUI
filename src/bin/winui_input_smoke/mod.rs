mod automation;
mod scenarios;
mod synthetic_pointer;

use std::path::PathBuf;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use a3s_gui::{
    run_winui_application_staged_async, wait_winui_dispatcher, ActionInvocation, GuiError,
    GuiResult, HostNodeId, NativeBackendKind, NativeCapabilities, NativeEvent, NativeEventKind,
    NativeInputConformanceCaseV1, NativeInputConformanceManifestV1,
    NativeInputConformanceObservationV1, NativeInputConformanceRunV1,
    NativeInputConformanceScenarioV1, NativeRole, UiFrame, WinUiEventWait, WinUiOsWidget,
    WinUiRuntimeApp,
};
use automation::{position_smoke_window, TARGET_LABEL};
use scenarios::{capture_smoke_run, create_smoke_app};
use serde_json::json;
use windows_core::Interface;
use winui3::Microsoft::UI::Xaml as xaml;

const WORKER_TIMEOUT: Duration = Duration::from_secs(10);
const EVENT_SETTLE_TIME: Duration = Duration::from_millis(250);
const CASES_PER_ROLE: usize = 14;
const NATIVE_INPUT_ROLES: [NativeRole; 7] = [
    NativeRole::Button,
    NativeRole::DisclosureSummary,
    NativeRole::Link,
    NativeRole::ImageMapArea,
    NativeRole::MenuItem,
    NativeRole::ListBoxItem,
    NativeRole::TreeItem,
];
const CAPTURED_NATIVE_CASES: usize = NATIVE_INPUT_ROLES.len() * CASES_PER_ROLE;

#[derive(Debug)]
struct FixtureState {
    generations: [u32; NATIVE_INPUT_ROLES.len()],
    role: NativeRole,
    disabled: bool,
    rerender_on_press_start: bool,
}

impl Default for FixtureState {
    fn default() -> Self {
        Self {
            generations: [0; NATIVE_INPUT_ROLES.len()],
            role: NativeRole::Button,
            disabled: false,
            rerender_on_press_start: false,
        }
    }
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
    let (run, diagnostics) =
        run_winui_application_staged_async(create_smoke_app, capture_smoke_run)?;
    let json = serde_json::to_string_pretty(&run)?;
    if let Some(path) = output {
        std::fs::write(&path, format!("{json}\n"))?;
        eprintln!("wrote WinUI OS-input evidence to {}", path.display());
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
    let children = NATIVE_INPUT_ROLES
        .iter()
        .enumerate()
        .map(|(index, role)| fixture_role(state, index, *role))
        .collect::<GuiResult<Vec<_>>>()?;
    serde_json::from_value(json!({
        "frameId": "winui-native-input-smoke-v1",
        "window": {
            "title": "A3S WinUI Native Input Smoke",
            "width": 360,
            "height": 420,
            "resizable": false
        },
        "actions": [{"id": "recordPress", "label": "Record semantic press event"}],
        "root": {
            "kind": "element",
            "key": "native-input-targets",
            "tag": "View",
            "children": children
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid WinUI input fixture: {error}")))
}

fn fixture_role(
    state: &FixtureState,
    index: usize,
    role: NativeRole,
) -> GuiResult<serde_json::Value> {
    let active = role == state.role;
    let target = json!({
        "kind": "element",
        "key": format!(
            "native-input-target-{index}-{}",
            state.generations[index]
        ),
        "tag": native_input_role_tag(role)?,
        "props": {
            "label": if active {
                TARGET_LABEL.to_string()
            } else {
                format!("Inactive {role:?} input target")
            },
            "value": format!("native-input-value-{index}"),
            "textValue": if active {
                TARGET_LABEL.to_string()
            } else {
                format!("Inactive {role:?} input target")
            },
            "disabled": if active { state.disabled } else { true },
            "events": {
                "onPressStart": "recordPress",
                "onPressUp": "recordPress",
                "onPressEnd": "recordPress",
                "onPress": "recordPress"
            }
        }
    });
    match role {
        NativeRole::ListBoxItem => Ok(json!({
            "kind": "element",
            "key": format!("native-input-list-{index}"),
            "tag": "ListBox",
            "props": {
                "label": "WinUI native input list fixture"
            },
            "children": [target]
        })),
        NativeRole::TreeItem => Ok(json!({
            "kind": "element",
            "key": format!("native-input-tree-{index}"),
            "tag": "Tree",
            "props": {
                "label": "WinUI native input tree fixture"
            },
            "children": [target]
        })),
        _ => Ok(target),
    }
}

fn native_input_role_tag(role: NativeRole) -> GuiResult<&'static str> {
    match role {
        NativeRole::Button => Ok("Button"),
        NativeRole::DisclosureSummary => Ok("DisclosureSummary"),
        NativeRole::Link => Ok("Link"),
        NativeRole::ImageMapArea => Ok("ImageMapArea"),
        NativeRole::MenuItem => Ok("MenuItem"),
        NativeRole::ListBoxItem => Ok("ListBoxItem"),
        NativeRole::TreeItem => Ok("TreeItem"),
        _ => Err(GuiError::invalid_tree(format!(
            "unsupported WinUI native input smoke role {role:?}"
        ))),
    }
}

fn fixture_reduce(state: &mut FixtureState, invocation: &ActionInvocation) -> GuiResult<()> {
    if state.rerender_on_press_start && invocation.event == NativeEventKind::PressStart {
        let index = native_input_role_index(state.role)?;
        state.generations[index] = state.generations[index].saturating_add(1);
        state.rerender_on_press_start = false;
    }
    Ok(())
}

async fn remount_fixture(
    app: &mut FixtureApp,
    role: NativeRole,
    disabled: bool,
    rerender_on_press_start: bool,
) -> GuiResult<HostNodeId> {
    let state = app.state_mut();
    state.role = role;
    state.disabled = disabled;
    state.rerender_on_press_start = rerender_on_press_start;
    app.render()?;
    let _ = pump_for(app, EVENT_SETTLE_TIME).await?;
    fixture_target(app, role)
}

fn native_input_role_index(role: NativeRole) -> GuiResult<usize> {
    NATIVE_INPUT_ROLES
        .iter()
        .position(|candidate| *candidate == role)
        .ok_or_else(|| GuiError::invalid_tree(format!("unsupported WinUI smoke role {role:?}")))
}

async fn set_fixture_disabled(app: &mut FixtureApp, disabled: bool) -> GuiResult<()> {
    app.state_mut().disabled = disabled;
    app.render()?;
    let _ = pump_for(app, EVENT_SETTLE_TIME).await?;
    Ok(())
}

fn fixture_target(app: &FixtureApp, role: NativeRole) -> GuiResult<HostNodeId> {
    let target = app
        .runtime()
        .mounted_snapshot()
        .into_iter()
        .find(|snapshot| snapshot.role == role)
        .map(|snapshot| snapshot.node)
        .ok_or_else(|| {
            GuiError::host(format!(
                "WinUI input fixture did not mount its {role:?} target"
            ))
        })?;
    let handle = app
        .runtime()
        .host()
        .executor()
        .driver()
        .handles()
        .get(&target)
        .ok_or_else(|| GuiError::host("WinUI input fixture target has no OS handle"))?;
    let expected_widget = match role {
        NativeRole::Button
        | NativeRole::DisclosureSummary
        | NativeRole::Link
        | NativeRole::ImageMapArea
        | NativeRole::MenuItem => matches!(handle.widget, WinUiOsWidget::Button(_)),
        NativeRole::ListBoxItem => matches!(handle.widget, WinUiOsWidget::ListBoxItem(_)),
        NativeRole::TreeItem => matches!(handle.widget, WinUiOsWidget::ListBoxItem(_)),
        _ => false,
    };
    if expected_widget {
        Ok(target)
    } else {
        Err(GuiError::host(format!(
            "WinUI {role:?} smoke target is not backed by its expected XAML control"
        )))
    }
}

fn fixture_handles(
    app: &FixtureApp,
    role: NativeRole,
) -> GuiResult<(HostNodeId, xaml::Window, isize)> {
    let handles = app.runtime().host().executor().driver().handles();
    let target = fixture_target(app, role)?;
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
    let hwnd_value = hwnd.0 as isize;
    position_smoke_window(hwnd_value)?;
    Ok((target, window, hwnd_value))
}

async fn run_scenario(
    app: &mut FixtureApp,
    role: NativeRole,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    capture_scenario_result(app, role, target, scenario, worker, None, diagnostics).await
}

async fn run_scenario_with_pointer_release(
    app: &mut FixtureApp,
    role: NativeRole,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    capture_scenario_result(
        app,
        role,
        target,
        scenario,
        worker,
        Some(target),
        diagnostics,
    )
    .await
}

async fn capture_scenario_result(
    app: &mut FixtureApp,
    role: NativeRole,
    target: HostNodeId,
    scenario: NativeInputConformanceScenarioV1,
    worker: JoinHandle<Result<(), String>>,
    release_on_press_start: Option<HostNodeId>,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let (events, result) = collect_worker_events(app, worker, release_on_press_start).await?;
    let stimulus_dispatched = result.is_ok();
    if let Err(error) = result {
        diagnostics.push(format!("{role:?}/{scenario:?}: {error}"));
    }
    Ok(NativeInputConformanceObservationV1::capture(
        NativeInputConformanceCaseV1::new(role, scenario),
        target,
        stimulus_dispatched,
        &events,
    ))
}

async fn collect_worker_events(
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
        wait_winui_dispatcher(Duration::from_millis(2)).await?;
    }
    let mut result = worker
        .join()
        .map_err(|_| GuiError::host("Windows input automation worker panicked"))?;
    let intervention_error = intervention_error.or_else(|| {
        release_on_press_start.map(|_| {
            "pointer cancellation did not observe PressStart before the input worker finished"
                .to_string()
        })
    });
    if let Some(error) = intervention_error {
        result = Err(match result {
            Ok(()) => error,
            Err(worker_error) => format!("{worker_error}; {error}"),
        });
    }
    events.extend(pump_for(app, EVENT_SETTLE_TIME).await?);
    Ok((events, result))
}

fn release_pointer_captures(app: &FixtureApp, target: HostNodeId) -> GuiResult<()> {
    let surface = app.runtime().host().executor().driver().adapter().surface();
    surface.cancel_winui_pointer_capture(target)
}

async fn pump_for(app: &mut FixtureApp, duration: Duration) -> GuiResult<Vec<NativeEvent>> {
    let deadline = Instant::now() + duration;
    let mut events = Vec::new();
    while Instant::now() < deadline {
        events.extend(pump_once(app)?);
        wait_winui_dispatcher(Duration::from_millis(2)).await?;
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

fn validate_smoke(run: &NativeInputConformanceRunV1, diagnostics: &mut Vec<String>) {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );
    let report = manifest.verify(run);
    if !report.is_conformant() {
        diagnostics.push(format!(
            "WinUI smoke validation verified {} of {} required cases{}",
            report.verified_cases,
            report.required_cases,
            if report.issues.is_empty() {
                String::new()
            } else {
                format!(
                    ": {}",
                    report
                        .issues
                        .iter()
                        .map(|issue| issue.message.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        ));
    }
}

#[cfg(test)]
mod tests;
