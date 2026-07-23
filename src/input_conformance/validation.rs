use super::*;

impl NativeInputConformanceManifestV1 {
    /// Verifies a run strictly against this generated manifest.
    pub fn verify(&self, run: &NativeInputConformanceRunV1) -> NativeInputConformanceReportV1 {
        let mut issues = Vec::new();
        let envelope_valid = self.validate_envelope(run, &mut issues);
        let mut valid_cases = 0;

        for requirement in &self.requirements {
            let observations = run
                .observations
                .iter()
                .filter(|observation| observation.case == requirement.case)
                .collect::<Vec<_>>();
            match observations.as_slice() {
                [] => issues.push(NativeInputConformanceIssueV1::new(
                    NativeInputConformanceIssueCodeV1::MissingObservation,
                    Some(requirement.case.clone()),
                    "required native input observation is missing",
                )),
                [observation] => {
                    let issue_count = issues.len();
                    validate_observation(requirement, observation, &mut issues);
                    if envelope_valid && issues.len() == issue_count {
                        valid_cases += 1;
                    }
                }
                _ => issues.push(NativeInputConformanceIssueV1::new(
                    NativeInputConformanceIssueCodeV1::DuplicateObservation,
                    Some(requirement.case.clone()),
                    format!(
                        "native input case has {} observations; exactly one is required",
                        observations.len()
                    ),
                )),
            }
        }

        for observation in &run.observations {
            if !self
                .requirements
                .iter()
                .any(|requirement| requirement.case == observation.case)
            {
                issues.push(NativeInputConformanceIssueV1::new(
                    NativeInputConformanceIssueCodeV1::UnexpectedObservation,
                    Some(observation.case.clone()),
                    "observation is not present in the generated capability manifest",
                ));
            }
        }

        NativeInputConformanceReportV1 {
            schema_version: NATIVE_INPUT_CONFORMANCE_VERSION_V1,
            backend: self.backend,
            required_cases: self.requirements.len(),
            verified_cases: valid_cases,
            issues,
        }
    }

    fn validate_envelope(
        &self,
        run: &NativeInputConformanceRunV1,
        issues: &mut Vec<NativeInputConformanceIssueV1>,
    ) -> bool {
        let initial_issue_count = issues.len();
        if self.schema_version != NATIVE_INPUT_CONFORMANCE_VERSION_V1
            || run.schema_version != NATIVE_INPUT_CONFORMANCE_VERSION_V1
        {
            issues.push(NativeInputConformanceIssueV1::new(
                NativeInputConformanceIssueCodeV1::SchemaVersionMismatch,
                None,
                format!(
                    "native input schema version must be {}; manifest is {} and run is {}",
                    NATIVE_INPUT_CONFORMANCE_VERSION_V1, self.schema_version, run.schema_version
                ),
            ));
        }
        if self.native_ir_version != NATIVE_IR_VERSION
            || run.native_ir_version != self.native_ir_version
        {
            issues.push(NativeInputConformanceIssueV1::new(
                NativeInputConformanceIssueCodeV1::NativeIrVersionMismatch,
                None,
                format!(
                    "native IR version must be {}; manifest is {} and run is {}",
                    NATIVE_IR_VERSION, self.native_ir_version, run.native_ir_version
                ),
            ));
        }
        if run.backend != self.backend {
            issues.push(NativeInputConformanceIssueV1::new(
                NativeInputConformanceIssueCodeV1::BackendMismatch,
                None,
                format!(
                    "evidence backend {:?} does not match manifest backend {:?}",
                    run.backend, self.backend
                ),
            ));
        }
        if self.backend == NativeBackendKind::Headless {
            issues.push(NativeInputConformanceIssueV1::new(
                NativeInputConformanceIssueCodeV1::UnsupportedBackend,
                None,
                "headless execution cannot provide operating-system input evidence",
            ));
        }
        if run.evidence_source != NativeInputEvidenceSourceV1::OperatingSystemAutomation {
            issues.push(NativeInputConformanceIssueV1::new(
                NativeInputConformanceIssueCodeV1::IneligibleEvidenceSource,
                None,
                "only operating-system automation may satisfy a native capability",
            ));
        }
        validate_environment(self.backend, run.environment.as_ref(), issues);
        issues.len() == initial_issue_count
    }
}

fn validate_environment(
    backend: NativeBackendKind,
    environment: Option<&NativeInputConformanceEnvironmentV1>,
    issues: &mut Vec<NativeInputConformanceIssueV1>,
) {
    let Some(environment) = environment else {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::MissingEnvironment,
            None,
            "operating-system automation evidence requires environment identity",
        ));
        return;
    };

    let expected = match backend {
        NativeBackendKind::AppKit => Some(NativeOperatingSystemV1::MacOS),
        NativeBackendKind::Gtk4 => Some(NativeOperatingSystemV1::Linux),
        NativeBackendKind::WinUI => Some(NativeOperatingSystemV1::Windows),
        NativeBackendKind::Headless => None,
    };
    if expected.is_some_and(|expected| expected != environment.operating_system) {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::OperatingSystemMismatch,
            None,
            format!(
                "environment operating system {:?} does not match backend {:?}",
                environment.operating_system, backend
            ),
        ));
    }
    if [
        environment.operating_system_version.as_str(),
        environment.toolkit_version.as_str(),
        environment.automation_driver.as_str(),
    ]
    .iter()
    .any(|value| value.trim().is_empty())
    {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::IncompleteEnvironment,
            None,
            "OS version, toolkit version, and automation driver must be non-empty",
        ));
    }
}

fn validate_observation(
    requirement: &NativeInputConformanceRequirementV1,
    observation: &NativeInputConformanceObservationV1,
    issues: &mut Vec<NativeInputConformanceIssueV1>,
) {
    let case = Some(requirement.case.clone());
    if !observation.stimulus_dispatched {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::StimulusNotDispatched,
            case.clone(),
            "automation driver did not confirm that the scenario stimulus was dispatched",
        ));
    }
    if observation.target.get() == 0 {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::InvalidTarget,
            case.clone(),
            "native automation target must have a non-zero node id",
        ));
    }
    if observation
        .events
        .iter()
        .any(|event| event.node != observation.target)
    {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::TargetMismatch,
            case.clone(),
            "semantic press events were emitted by a node other than the automation target",
        ));
    }

    let actual_events = observation
        .events
        .iter()
        .map(|event| event.kind)
        .collect::<Vec<_>>();
    if actual_events != requirement.expected_events {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::EventOrderMismatch,
            case.clone(),
            format!(
                "expected semantic trace {:?}, observed {:?}",
                requirement.expected_events, actual_events
            ),
        ));
    }

    if observation
        .events
        .iter()
        .any(|event| !modality_matches(requirement.stimulus_modality, event.modality))
    {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::ModalityMismatch,
            case.clone(),
            format!(
                "semantic events do not match the required {:?} stimulus modality",
                requirement.stimulus_modality
            ),
        ));
    }

    if requirement.stimulus_modality == NativeInputConformanceModalityV1::Keyboard
        && observation
            .events
            .iter()
            .any(|event| !event.handled_activation)
    {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::KeyboardActivationMarkerMissing,
            case.clone(),
            "keyboard press events must mark activation as handled to prevent duplicate synthesis",
        ));
    }

    let expected_click_count = match requirement.stimulus_modality {
        NativeInputConformanceModalityV1::Mouse
        | NativeInputConformanceModalityV1::Pen
        | NativeInputConformanceModalityV1::Touch => Some(1),
        NativeInputConformanceModalityV1::Keyboard | NativeInputConformanceModalityV1::Virtual => {
            Some(0)
        }
        NativeInputConformanceModalityV1::Unspecified => None,
    };
    if expected_click_count.is_some_and(|expected| {
        observation
            .events
            .iter()
            .any(|event| event.click_count != expected)
    }) {
        issues.push(NativeInputConformanceIssueV1::new(
            NativeInputConformanceIssueCodeV1::ClickCountMismatch,
            case,
            format!(
                "semantic events do not carry the expected click count {:?}",
                expected_click_count
            ),
        ));
    }
}

fn modality_matches(
    required: NativeInputConformanceModalityV1,
    observed: NativeInputModality,
) -> bool {
    match required {
        NativeInputConformanceModalityV1::Mouse => observed == NativeInputModality::Mouse,
        NativeInputConformanceModalityV1::Pen => observed == NativeInputModality::Pen,
        NativeInputConformanceModalityV1::Touch => observed == NativeInputModality::Touch,
        NativeInputConformanceModalityV1::Keyboard => observed == NativeInputModality::Keyboard,
        NativeInputConformanceModalityV1::Virtual => observed == NativeInputModality::Virtual,
        NativeInputConformanceModalityV1::Unspecified => true,
    }
}
