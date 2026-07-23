use std::time::Duration;

use a3s_gui::{
    ActionInvocation, GuiResult, NativeBackendKind, NativeInputConformanceObservationV1,
    NativeInputConformanceRunV1, NativeInputConformanceScenarioV1, NativeInputEvidenceSourceV1,
    UiFrame, WinUiRuntimeApp,
};

use super::automation::{
    spawn_assistive_activation, spawn_keyboard_activation, spawn_keyed_rerender_activation,
    spawn_mouse_activation, spawn_mouse_cancellation, windows_environment,
};
use super::synthetic_pointer::{
    spawn_synthetic_pointer, SyntheticPointerCompletion, SyntheticPointerKind,
};
use super::{
    fixture_frame, fixture_handles, fixture_reduce, pump_for, remount_fixture, run_scenario,
    run_scenario_with_pointer_release, set_fixture_disabled, validate_partial_smoke, FixtureApp,
    FixtureState, WindowGuard, CAPTURED_BUTTON_CASES, EVENT_SETTLE_TIME,
};

pub(super) fn capture_smoke_run() -> GuiResult<(NativeInputConformanceRunV1, Vec<String>)> {
    let mut app: FixtureApp = WinUiRuntimeApp::winui(
        FixtureState::default(),
        fixture_frame as fn(&FixtureState) -> GuiResult<UiFrame>,
        fixture_reduce as fn(&mut FixtureState, &ActionInvocation) -> GuiResult<()>,
    )?;
    app.render()?;
    let (_, window, hwnd_value) = fixture_handles(&app)?;
    let _window_guard = WindowGuard(window);
    let _ = pump_for(&mut app, EVENT_SETTLE_TIME)?;

    let mut diagnostics = Vec::new();
    let mut observations = Vec::new();
    observations.push(capture_mouse_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::MouseActivation,
        false,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::PenActivation,
        SyntheticPointerKind::Pen,
        SyntheticPointerCompletion::Activate,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::TouchActivation,
        SyntheticPointerKind::Touch,
        SyntheticPointerCompletion::Activate,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_keyboard_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::KeyboardActivation,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_assistive_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::AssistiveActivation,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_mouse_cancellation_scenario(
        &mut app,
        hwnd_value,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::PenCancellation,
        SyntheticPointerKind::Pen,
        SyntheticPointerCompletion::Cancel,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::TouchCancellation,
        SyntheticPointerKind::Touch,
        SyntheticPointerCompletion::Cancel,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_mouse_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::KeyedRerenderCancellation,
        false,
        true,
        &mut diagnostics,
    )?);
    observations.push(capture_mouse_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::DisabledMouseActivation,
        true,
        false,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::DisabledPenActivation,
        SyntheticPointerKind::Pen,
        SyntheticPointerCompletion::Activate,
        true,
        &mut diagnostics,
    )?);
    observations.push(capture_synthetic_pointer_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::DisabledTouchActivation,
        SyntheticPointerKind::Touch,
        SyntheticPointerCompletion::Activate,
        true,
        &mut diagnostics,
    )?);
    observations.push(capture_keyboard_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::DisabledKeyboardActivation,
        true,
        &mut diagnostics,
    )?);
    observations.push(capture_assistive_scenario(
        &mut app,
        hwnd_value,
        NativeInputConformanceScenarioV1::DisabledAssistiveActivation,
        true,
        &mut diagnostics,
    )?);

    if observations.len() != CAPTURED_BUTTON_CASES {
        diagnostics.push(format!(
            "WinUI Button smoke captured {} cases, expected {CAPTURED_BUTTON_CASES}",
            observations.len()
        ));
    }

    let run = NativeInputConformanceRunV1::new(
        NativeBackendKind::WinUI,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    )
    .environment(windows_environment()?)
    .observations(observations);
    validate_partial_smoke(&run, &mut diagnostics);
    Ok((run, diagnostics))
}

fn capture_mouse_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    keyed_rerender: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, disabled, keyed_rerender)?;
    let worker = if keyed_rerender {
        spawn_keyed_rerender_activation(hwnd)
    } else {
        spawn_mouse_activation(hwnd, !disabled)
    };
    run_scenario(app, target, scenario, worker, diagnostics)
}

fn capture_mouse_cancellation_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, false, false)?;
    run_scenario_with_pointer_release(
        app,
        target,
        NativeInputConformanceScenarioV1::MouseCancellation,
        spawn_mouse_cancellation(hwnd),
        diagnostics,
    )
}

fn capture_synthetic_pointer_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    scenario: NativeInputConformanceScenarioV1,
    kind: SyntheticPointerKind,
    completion: SyntheticPointerCompletion,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, disabled, false)?;
    let worker = spawn_synthetic_pointer(hwnd, !disabled, kind, completion);
    run_scenario(app, target, scenario, worker, diagnostics)
}

fn capture_keyboard_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, false, false)?;
    app.runtime_mut().request_focus(target)?;
    let _ = pump_for(app, Duration::from_millis(100))?;
    if disabled {
        set_fixture_disabled(app, true)?;
    }
    run_scenario(
        app,
        target,
        scenario,
        spawn_keyboard_activation(hwnd, !disabled),
        diagnostics,
    )
}

fn capture_assistive_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, disabled, false)?;
    run_scenario(
        app,
        target,
        scenario,
        spawn_assistive_activation(hwnd, !disabled),
        diagnostics,
    )
}
