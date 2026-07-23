use super::*;
use a3s_gui::{
    NativeEventContext, NativeInputConformanceEnvironmentV1, NativeInputConformanceModalityV1,
    NativeInputEvidenceSourceV1, NativeInputModality, NativeOperatingSystemV1,
};

#[test]
fn partial_validation_accepts_all_button_cases() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );
    let target = HostNodeId::new(7);
    let observations = manifest
        .requirements
        .iter()
        .filter(|requirement| requirement.case.role == NativeRole::Button)
        .map(|requirement| valid_observation(target, requirement))
        .collect::<Vec<_>>();
    assert_eq!(observations.len(), CAPTURED_BUTTON_CASES);

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

#[test]
fn press_start_replaces_the_key_only_for_the_rerender_scenario() {
    let mut state = FixtureState {
        generation: 4,
        rerender_on_press_start: true,
        ..FixtureState::default()
    };
    let invocation = ActionInvocation::new(
        HostNodeId::new(1),
        "recordPress",
        NativeEventKind::PressStart,
    );

    fixture_reduce(&mut state, &invocation).unwrap();
    assert_eq!(state.generation, 5);
    assert!(!state.rerender_on_press_start);

    fixture_reduce(&mut state, &invocation).unwrap();
    assert_eq!(state.generation, 5);
}

fn valid_observation(
    target: HostNodeId,
    requirement: &a3s_gui::NativeInputConformanceRequirementV1,
) -> NativeInputConformanceObservationV1 {
    let (modality, handled_activation, click_count) = match requirement.stimulus_modality {
        NativeInputConformanceModalityV1::Mouse => (NativeInputModality::Mouse, false, 1),
        NativeInputConformanceModalityV1::Pen => (NativeInputModality::Pen, false, 1),
        NativeInputConformanceModalityV1::Touch => (NativeInputModality::Touch, false, 1),
        NativeInputConformanceModalityV1::Keyboard => (NativeInputModality::Keyboard, true, 0),
        NativeInputConformanceModalityV1::Virtual => (NativeInputModality::Virtual, false, 0),
        NativeInputConformanceModalityV1::Unspecified => (NativeInputModality::Unknown, false, 0),
    };
    let context = NativeEventContext::new()
        .modality(modality)
        .handled_activation(handled_activation)
        .click_count(click_count);
    let events = requirement
        .expected_events
        .iter()
        .copied()
        .map(|kind| NativeEvent::new(target, kind).context(context))
        .collect::<Vec<_>>();
    NativeInputConformanceObservationV1::capture(requirement.case.clone(), target, true, &events)
}
