use super::*;

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::{NativeEventContext, NativeInputModality};
use crate::native::NativeRole;

fn environment_for(backend: NativeBackendKind) -> NativeInputConformanceEnvironmentV1 {
    let operating_system = match backend {
        NativeBackendKind::AppKit => NativeOperatingSystemV1::MacOS,
        NativeBackendKind::Gtk4 => NativeOperatingSystemV1::Linux,
        NativeBackendKind::WinUI => NativeOperatingSystemV1::Windows,
        NativeBackendKind::Headless => NativeOperatingSystemV1::Other,
    };
    NativeInputConformanceEnvironmentV1::new(
        operating_system,
        "test-os-1",
        "test-toolkit-1",
        "test-native-automation",
    )
}

fn native_modality(modality: NativeInputConformanceModalityV1) -> NativeInputModality {
    match modality {
        NativeInputConformanceModalityV1::Mouse => NativeInputModality::Mouse,
        NativeInputConformanceModalityV1::Pen => NativeInputModality::Pen,
        NativeInputConformanceModalityV1::Touch => NativeInputModality::Touch,
        NativeInputConformanceModalityV1::Keyboard => NativeInputModality::Keyboard,
        NativeInputConformanceModalityV1::Virtual => NativeInputModality::Virtual,
        NativeInputConformanceModalityV1::Unspecified => NativeInputModality::Unknown,
    }
}

fn passing_observation(
    requirement: &NativeInputConformanceRequirementV1,
) -> NativeInputConformanceObservationV1 {
    let target = HostNodeId::new(42);
    let modality = native_modality(requirement.stimulus_modality);
    let pointer = matches!(
        requirement.stimulus_modality,
        NativeInputConformanceModalityV1::Mouse
            | NativeInputConformanceModalityV1::Pen
            | NativeInputConformanceModalityV1::Touch
    );
    let keyboard = requirement.stimulus_modality == NativeInputConformanceModalityV1::Keyboard;
    let events = requirement
        .expected_events
        .iter()
        .map(|kind| {
            NativeEvent::new(target, *kind).context(
                NativeEventContext::new()
                    .modality(modality)
                    .click_count(u8::from(pointer))
                    .handled_activation(keyboard),
            )
        })
        .collect::<Vec<_>>();
    NativeInputConformanceObservationV1::capture(requirement.case.clone(), target, true, &events)
}

fn passing_run(
    manifest: &NativeInputConformanceManifestV1,
    source: NativeInputEvidenceSourceV1,
) -> NativeInputConformanceRunV1 {
    NativeInputConformanceRunV1::new(manifest.backend, source)
        .environment(environment_for(manifest.backend))
        .observations(
            manifest
                .requirements
                .iter()
                .map(passing_observation)
                .collect(),
        )
}

fn has_case(
    manifest: &NativeInputConformanceManifestV1,
    role: NativeRole,
    scenario: NativeInputConformanceScenarioV1,
) -> bool {
    manifest
        .requirements
        .iter()
        .any(|requirement| requirement.case.role == role && requirement.case.scenario == scenario)
}

#[test]
fn manifests_expand_every_native_press_role_into_platform_scenarios() {
    let appkit = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );
    let gtk4 = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::Gtk4),
    );
    let winui = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );

    for manifest in [&appkit, &gtk4, &winui] {
        for scenario in [
            NativeInputConformanceScenarioV1::MouseActivation,
            NativeInputConformanceScenarioV1::PenActivation,
            NativeInputConformanceScenarioV1::KeyboardActivation,
            NativeInputConformanceScenarioV1::AssistiveActivation,
            NativeInputConformanceScenarioV1::MouseCancellation,
            NativeInputConformanceScenarioV1::PenCancellation,
            NativeInputConformanceScenarioV1::KeyedRerenderCancellation,
            NativeInputConformanceScenarioV1::DisabledMouseActivation,
            NativeInputConformanceScenarioV1::DisabledPenActivation,
            NativeInputConformanceScenarioV1::DisabledKeyboardActivation,
            NativeInputConformanceScenarioV1::DisabledAssistiveActivation,
        ] {
            assert!(has_case(manifest, NativeRole::Button, scenario));
        }
    }

    assert!(!has_case(
        &appkit,
        NativeRole::Button,
        NativeInputConformanceScenarioV1::TouchActivation
    ));
    for manifest in [&gtk4, &winui] {
        assert!(has_case(
            manifest,
            NativeRole::Button,
            NativeInputConformanceScenarioV1::TouchActivation
        ));
        assert!(has_case(
            manifest,
            NativeRole::Button,
            NativeInputConformanceScenarioV1::TouchCancellation
        ));
        assert!(has_case(
            manifest,
            NativeRole::Button,
            NativeInputConformanceScenarioV1::DisabledTouchActivation
        ));
    }

    for manifest in [&appkit, &gtk4] {
        assert!(has_case(
            manifest,
            NativeRole::MenuItem,
            NativeInputConformanceScenarioV1::TerminalActivation
        ));
        assert!(!has_case(
            manifest,
            NativeRole::MenuItem,
            NativeInputConformanceScenarioV1::MouseActivation
        ));
    }
    assert!(has_case(
        &winui,
        NativeRole::MenuItem,
        NativeInputConformanceScenarioV1::MouseActivation
    ));

    assert_eq!(appkit.requirements.len(), 67);
    assert_eq!(gtk4.requirements.len(), 85);
    assert_eq!(winui.requirements.len(), 98);
}

#[test]
fn complete_os_automation_evidence_passes_the_generated_manifest() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::WinUI),
    );
    let run = passing_run(
        &manifest,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );

    let report = manifest.verify(&run);

    assert!(report.is_conformant());
    assert_eq!(report.required_cases, manifest.requirements.len());
    assert_eq!(report.verified_cases, manifest.requirements.len());
    assert!(report.issues.is_empty());
}

#[test]
fn adapter_and_portable_traces_cannot_satisfy_native_evidence() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );

    for source in [
        NativeInputEvidenceSourceV1::AdapterKernel,
        NativeInputEvidenceSourceV1::PortableRuntime,
    ] {
        let report = manifest.verify(&passing_run(&manifest, source));
        assert!(!report.is_conformant());
        assert_eq!(report.verified_cases, 0);
        assert!(report.issues.iter().any(|issue| {
            issue.code == NativeInputConformanceIssueCodeV1::IneligibleEvidenceSource
        }));
    }
}

#[test]
fn verifier_reports_missing_duplicate_and_unexpected_cases() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );
    let mut run = passing_run(
        &manifest,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );
    let missing = run.observations.remove(0);
    run.observations.push(run.observations[0].clone());
    run.observations
        .push(NativeInputConformanceObservationV1::capture(
            NativeInputConformanceCaseV1::new(
                NativeRole::Window,
                NativeInputConformanceScenarioV1::MouseActivation,
            ),
            HostNodeId::new(99),
            true,
            &[],
        ));

    let report = manifest.verify(&run);

    assert!(report.issues.iter().any(|issue| {
        issue.code == NativeInputConformanceIssueCodeV1::MissingObservation
            && issue.case.as_ref() == Some(&missing.case)
    }));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == NativeInputConformanceIssueCodeV1::DuplicateObservation));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == NativeInputConformanceIssueCodeV1::UnexpectedObservation));
}

#[test]
fn verifier_rejects_wrong_order_modality_target_and_keyboard_marker() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );
    let requirement = manifest
        .requirements
        .iter()
        .find(|requirement| {
            requirement.case.role == NativeRole::Button
                && requirement.case.scenario == NativeInputConformanceScenarioV1::KeyboardActivation
        })
        .expect("button keyboard requirement");
    let mut run = passing_run(
        &manifest,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );
    let observation = run
        .observations
        .iter_mut()
        .find(|observation| observation.case == requirement.case)
        .expect("button keyboard observation");
    observation.events.swap(0, 1);
    observation.events[0].modality = NativeInputModality::Mouse;
    observation.events[1].node = HostNodeId::new(7);
    observation.events[2].handled_activation = false;

    let report = manifest.verify(&run);

    for code in [
        NativeInputConformanceIssueCodeV1::EventOrderMismatch,
        NativeInputConformanceIssueCodeV1::ModalityMismatch,
        NativeInputConformanceIssueCodeV1::TargetMismatch,
        NativeInputConformanceIssueCodeV1::KeyboardActivationMarkerMissing,
    ] {
        assert!(report.issues.iter().any(|issue| issue.code == code));
    }
}

#[test]
fn verifier_requires_a_real_environment_and_completed_disabled_stimulus() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );
    let mut run = passing_run(
        &manifest,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );
    let environment = run.environment.as_mut().expect("test environment");
    environment.operating_system = NativeOperatingSystemV1::Windows;
    environment.toolkit_version = " ".to_string();
    let disabled = run
        .observations
        .iter_mut()
        .find(|observation| {
            observation.case.role == NativeRole::Button
                && observation.case.scenario
                    == NativeInputConformanceScenarioV1::DisabledMouseActivation
        })
        .expect("disabled mouse observation");
    disabled.stimulus_dispatched = false;
    disabled.target = HostNodeId::new(0);
    let mouse = run
        .observations
        .iter_mut()
        .find(|observation| {
            observation.case.role == NativeRole::Button
                && observation.case.scenario == NativeInputConformanceScenarioV1::MouseActivation
        })
        .expect("mouse observation");
    for event in &mut mouse.events {
        event.click_count = 0;
    }

    let report = manifest.verify(&run);

    assert_eq!(report.verified_cases, 0);
    for code in [
        NativeInputConformanceIssueCodeV1::OperatingSystemMismatch,
        NativeInputConformanceIssueCodeV1::IncompleteEnvironment,
        NativeInputConformanceIssueCodeV1::StimulusNotDispatched,
        NativeInputConformanceIssueCodeV1::InvalidTarget,
        NativeInputConformanceIssueCodeV1::ClickCountMismatch,
    ] {
        assert!(report.issues.iter().any(|issue| issue.code == code));
    }
}

#[test]
fn missing_environment_and_headless_runs_never_pass_as_native_evidence() {
    let appkit = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::AppKit),
    );
    let mut appkit_run = passing_run(
        &appkit,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );
    appkit_run.environment = None;
    let appkit_report = appkit.verify(&appkit_run);
    assert!(appkit_report
        .issues
        .iter()
        .any(|issue| issue.code == NativeInputConformanceIssueCodeV1::MissingEnvironment));

    let headless = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::Headless),
    );
    let headless_run = NativeInputConformanceRunV1::new(
        NativeBackendKind::Headless,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    )
    .environment(environment_for(NativeBackendKind::Headless));
    let headless_report = headless.verify(&headless_run);
    assert!(!headless_report.is_conformant());
    assert!(headless_report
        .issues
        .iter()
        .any(|issue| issue.code == NativeInputConformanceIssueCodeV1::UnsupportedBackend));
}

#[test]
fn capture_keeps_only_redacted_semantic_press_trace_data() {
    let case = NativeInputConformanceCaseV1::new(
        NativeRole::Button,
        NativeInputConformanceScenarioV1::KeyboardActivation,
    );
    let node = HostNodeId::new(11);
    let context = NativeEventContext::new()
        .modality(NativeInputModality::Keyboard)
        .handled_activation(true);
    let observation = NativeInputConformanceObservationV1::capture(
        case,
        node,
        true,
        &[
            NativeEvent::new(node, NativeEventKind::PressStart).context(context),
            NativeEvent::new(node, NativeEventKind::KeyDown).value("secret-value"),
            NativeEvent::new(node, NativeEventKind::Press).context(context),
        ],
    );

    assert_eq!(observation.events.len(), 2);
    assert_eq!(observation.events[0].kind, NativeEventKind::PressStart);
    assert_eq!(observation.events[1].kind, NativeEventKind::Press);
    let json = serde_json::to_string(&observation).unwrap();
    assert!(!json.contains("secret-value"));
}

#[test]
fn manifest_run_and_report_are_versioned_serializable_artifacts() {
    let manifest = NativeInputConformanceManifestV1::from_capabilities(
        &NativeCapabilities::for_backend(NativeBackendKind::Gtk4),
    );
    let run = passing_run(
        &manifest,
        NativeInputEvidenceSourceV1::OperatingSystemAutomation,
    );
    let report = manifest.verify(&run);

    let manifest_json = serde_json::to_value(&manifest).unwrap();
    let run_json = serde_json::to_value(&run).unwrap();
    let report_json = serde_json::to_value(&report).unwrap();
    assert_eq!(
        manifest_json["schemaVersion"],
        NATIVE_INPUT_CONFORMANCE_VERSION_V1
    );
    assert_eq!(
        run_json["schemaVersion"],
        NATIVE_INPUT_CONFORMANCE_VERSION_V1
    );
    assert_eq!(
        report_json["schemaVersion"],
        NATIVE_INPUT_CONFORMANCE_VERSION_V1
    );
    assert_eq!(
        serde_json::from_value::<NativeInputConformanceManifestV1>(manifest_json).unwrap(),
        manifest
    );
    assert_eq!(
        serde_json::from_value::<NativeInputConformanceRunV1>(run_json).unwrap(),
        run
    );
    assert_eq!(
        serde_json::from_value::<NativeInputConformanceReportV1>(report_json).unwrap(),
        report
    );
}

#[test]
fn conformance_artifacts_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<NativeInputConformanceManifestV1>();
    assert_send_sync::<NativeInputConformanceRequirementV1>();
    assert_send_sync::<NativeInputConformanceObservationV1>();
    assert_send_sync::<NativeInputConformanceRunV1>();
    assert_send_sync::<NativeInputConformanceReportV1>();
}
