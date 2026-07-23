use std::time::Duration;

use a3s_gui::{
    ActionInvocation, GuiResult, NativeBackendKind, NativeInputConformanceObservationV1,
    NativeInputConformanceRunV1, NativeInputConformanceScenarioV1, NativeInputEvidenceSourceV1,
    NativeRole, UiFrame, WinUiRuntimeApp,
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
    run_scenario_with_pointer_release, set_fixture_disabled, validate_smoke, FixtureApp,
    FixtureState, WindowGuard, CAPTURED_NATIVE_CASES, CASES_PER_ROLE, EVENT_SETTLE_TIME,
    NATIVE_INPUT_ROLES,
};

pub(super) fn create_smoke_app() -> GuiResult<FixtureApp> {
    let mut app: FixtureApp = WinUiRuntimeApp::winui(
        FixtureState::default(),
        fixture_frame as fn(&FixtureState) -> GuiResult<UiFrame>,
        fixture_reduce as fn(&mut FixtureState, &ActionInvocation) -> GuiResult<()>,
    )?;
    app.render()?;
    Ok(app)
}

pub(super) async fn capture_smoke_run(
    mut app: FixtureApp,
) -> GuiResult<(NativeInputConformanceRunV1, Vec<String>)> {
    let (_, window, hwnd_value) = fixture_handles(&app, NativeRole::Button)?;
    let _window_guard = WindowGuard(window);
    let _ = pump_for(&mut app, EVENT_SETTLE_TIME).await?;

    let mut diagnostics = Vec::new();
    let mut observations = Vec::new();
    for role in NATIVE_INPUT_ROLES {
        observations
            .extend(capture_role_scenarios(&mut app, hwnd_value, role, &mut diagnostics).await?);
    }

    if observations.len() != CAPTURED_NATIVE_CASES {
        diagnostics.push(format!(
            "WinUI native input smoke captured {} cases, expected {CAPTURED_NATIVE_CASES}",
            observations.len()
        ));
    }

    let run = NativeInputConformanceRunV1::new(
        NativeBackendKind::WinUI,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    )
    .environment(windows_environment()?)
    .observations(observations);
    validate_smoke(&run, &mut diagnostics);
    Ok((run, diagnostics))
}

async fn capture_role_scenarios(
    app: &mut FixtureApp,
    hwnd_value: isize,
    role: NativeRole,
    diagnostics: &mut Vec<String>,
) -> GuiResult<Vec<NativeInputConformanceObservationV1>> {
    let mut observations = Vec::with_capacity(CASES_PER_ROLE);
    observations.push(
        capture_mouse_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::MouseActivation,
            false,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::PenActivation,
            SyntheticPointerKind::Pen,
            SyntheticPointerCompletion::Activate,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::TouchActivation,
            SyntheticPointerKind::Touch,
            SyntheticPointerCompletion::Activate,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_keyboard_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::KeyboardActivation,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_assistive_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::AssistiveActivation,
            false,
            diagnostics,
        )
        .await?,
    );
    observations
        .push(capture_mouse_cancellation_scenario(app, hwnd_value, role, diagnostics).await?);
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::PenCancellation,
            SyntheticPointerKind::Pen,
            SyntheticPointerCompletion::Cancel,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::TouchCancellation,
            SyntheticPointerKind::Touch,
            SyntheticPointerCompletion::Cancel,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_mouse_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::DisabledMouseActivation,
            true,
            false,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::DisabledPenActivation,
            SyntheticPointerKind::Pen,
            SyntheticPointerCompletion::Activate,
            true,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_synthetic_pointer_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::DisabledTouchActivation,
            SyntheticPointerKind::Touch,
            SyntheticPointerCompletion::Activate,
            true,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_keyboard_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::DisabledKeyboardActivation,
            true,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_assistive_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::DisabledAssistiveActivation,
            true,
            diagnostics,
        )
        .await?,
    );
    observations.push(
        capture_mouse_scenario(
            app,
            hwnd_value,
            role,
            NativeInputConformanceScenarioV1::KeyedRerenderCancellation,
            false,
            true,
            diagnostics,
        )
        .await?,
    );

    if observations.len() != CASES_PER_ROLE {
        diagnostics.push(format!(
            "WinUI {role:?} smoke captured {} cases, expected {CASES_PER_ROLE}",
            observations.len()
        ));
    }
    Ok(observations)
}

async fn capture_mouse_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    role: NativeRole,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    keyed_rerender: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, role, disabled, keyed_rerender).await?;
    let worker = if keyed_rerender {
        spawn_keyed_rerender_activation(hwnd, role)
    } else {
        spawn_mouse_activation(hwnd, role, !disabled)
    };
    run_scenario(app, role, target, scenario, worker, diagnostics).await
}

async fn capture_mouse_cancellation_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    role: NativeRole,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, role, false, false).await?;
    run_scenario_with_pointer_release(
        app,
        role,
        target,
        NativeInputConformanceScenarioV1::MouseCancellation,
        spawn_mouse_cancellation(hwnd, role),
        diagnostics,
    )
    .await
}

async fn capture_synthetic_pointer_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    role: NativeRole,
    scenario: NativeInputConformanceScenarioV1,
    kind: SyntheticPointerKind,
    completion: SyntheticPointerCompletion,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, role, disabled, false).await?;
    let worker = spawn_synthetic_pointer(hwnd, role, !disabled, kind, completion);
    if completion == SyntheticPointerCompletion::Cancel && !disabled {
        run_scenario_with_pointer_release(app, role, target, scenario, worker, diagnostics).await
    } else {
        run_scenario(app, role, target, scenario, worker, diagnostics).await
    }
}

async fn capture_keyboard_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    role: NativeRole,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, role, false, false).await?;
    app.runtime_mut().request_focus(target)?;
    let _ = pump_for(app, Duration::from_millis(100)).await?;
    if disabled {
        set_fixture_disabled(app, true).await?;
    }
    run_scenario(
        app,
        role,
        target,
        scenario,
        spawn_keyboard_activation(hwnd, role, !disabled),
        diagnostics,
    )
    .await
}

async fn capture_assistive_scenario(
    app: &mut FixtureApp,
    hwnd: isize,
    role: NativeRole,
    scenario: NativeInputConformanceScenarioV1,
    disabled: bool,
    diagnostics: &mut Vec<String>,
) -> GuiResult<NativeInputConformanceObservationV1> {
    let target = remount_fixture(app, role, disabled, false).await?;
    run_scenario(
        app,
        role,
        target,
        scenario,
        spawn_assistive_activation(hwnd, role, !disabled),
        diagnostics,
    )
    .await
}
