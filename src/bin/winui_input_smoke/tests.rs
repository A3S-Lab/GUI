use super::*;
use a3s_gui::{
    NativeEventContext, NativeInputConformanceEnvironmentV1, NativeInputConformanceModalityV1,
    NativeInputEvidenceSourceV1, NativeInputModality, NativeOperatingSystemV1, RsxCompilerBridge,
};

#[test]
fn validation_accepts_the_complete_winui_manifest() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );
    let target = HostNodeId::new(7);
    let observations = manifest
        .requirements
        .iter()
        .filter(|requirement| NATIVE_INPUT_ROLES.contains(&requirement.case.role))
        .map(|requirement| valid_observation(target, requirement))
        .collect::<Vec<_>>();
    assert_eq!(observations.len(), CAPTURED_NATIVE_CASES);

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
    validate_smoke(&run, &mut diagnostics);
    assert!(diagnostics.is_empty());

    let mut broken = run;
    broken.observations[0].events.remove(1);
    validate_smoke(&broken, &mut diagnostics);
    assert_eq!(diagnostics.len(), 1);
}

#[test]
fn fixture_frames_preserve_each_native_input_role() {
    for role in NATIVE_INPUT_ROLES {
        let state = FixtureState {
            role,
            ..FixtureState::default()
        };
        let frame = fixture_frame(&state).unwrap();
        let native = RsxCompilerBridge::new()
            .lower_to_native(&frame.root)
            .unwrap();

        assert_eq!(native.role, NativeRole::View);
        assert_eq!(native.children.len(), NATIVE_INPUT_ROLES.len());
        assert_eq!(
            native
                .children
                .iter()
                .map(|child| child.role)
                .collect::<Vec<_>>(),
            vec![
                NativeRole::Button,
                NativeRole::DisclosureSummary,
                NativeRole::Link,
                NativeRole::ImageMapArea,
                NativeRole::MenuItem,
                NativeRole::ListBox,
                NativeRole::Tree,
            ]
        );
        let active = find_role(&native, role).unwrap();
        assert_eq!(active.props.label.as_deref(), Some(TARGET_LABEL));
        assert!(!active.props.disabled);
    }
}

#[test]
fn press_start_replaces_the_key_only_for_the_rerender_scenario() {
    let role = NativeRole::MenuItem;
    let index = native_input_role_index(role).unwrap();
    let mut generations = [0; NATIVE_INPUT_ROLES.len()];
    generations[index] = 4;
    let mut state = FixtureState {
        generations,
        role,
        rerender_on_press_start: true,
        ..FixtureState::default()
    };
    let invocation = ActionInvocation::new(
        HostNodeId::new(1),
        "recordPress",
        NativeEventKind::PressStart,
    );

    fixture_reduce(&mut state, &invocation).unwrap();
    assert_eq!(state.generations[index], 5);
    assert!(state
        .generations
        .iter()
        .enumerate()
        .all(|(candidate, generation)| candidate == index || *generation == 0));
    assert!(!state.rerender_on_press_start);

    fixture_reduce(&mut state, &invocation).unwrap();
    assert_eq!(state.generations[index], 5);
}

fn find_role(
    element: &a3s_gui::NativeElement,
    role: NativeRole,
) -> Option<&a3s_gui::NativeElement> {
    if element.role == role {
        return Some(element);
    }
    element
        .children
        .iter()
        .find_map(|child| find_role(child, role))
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
