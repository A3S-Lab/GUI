use serde::{Deserialize, Serialize};

use crate::capability::{
    CapabilitySupport, NativeCapabilities, NativeCapabilityFeature, NATIVE_IR_VERSION,
};
use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::input::NativeInputModality;
use crate::native::NativeRole;
use crate::platform::NativeBackendKind;

mod validation;

/// Schema version for native input conformance manifests, runs, and reports.
pub const NATIVE_INPUT_CONFORMANCE_VERSION_V1: u16 = 1;

/// A platform interaction that must be driven against an operating-system widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeInputConformanceScenarioV1 {
    MouseActivation,
    PenActivation,
    TouchActivation,
    KeyboardActivation,
    AssistiveActivation,
    MouseCancellation,
    PenCancellation,
    TouchCancellation,
    KeyedRerenderCancellation,
    DisabledMouseActivation,
    DisabledPenActivation,
    DisabledTouchActivation,
    DisabledKeyboardActivation,
    DisabledAssistiveActivation,
    TerminalActivation,
}

/// Modality used to drive a conformance scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeInputConformanceModalityV1 {
    Mouse,
    Pen,
    Touch,
    Keyboard,
    Virtual,
    Unspecified,
}

/// Stable identity of one role/scenario pair in a conformance manifest.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceCaseV1 {
    pub role: NativeRole,
    pub scenario: NativeInputConformanceScenarioV1,
}

impl NativeInputConformanceCaseV1 {
    pub const fn new(role: NativeRole, scenario: NativeInputConformanceScenarioV1) -> Self {
        Self { role, scenario }
    }
}

/// Machine-readable expected result for one native input case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceRequirementV1 {
    pub case: NativeInputConformanceCaseV1,
    pub stimulus_modality: NativeInputConformanceModalityV1,
    pub expected_events: Vec<NativeEventKind>,
}

impl NativeInputConformanceRequirementV1 {
    fn new(role: NativeRole, scenario: NativeInputConformanceScenarioV1) -> Self {
        let (stimulus_modality, expected_events) = scenario_contract(scenario);
        Self {
            case: NativeInputConformanceCaseV1::new(role, scenario),
            stimulus_modality,
            expected_events,
        }
    }
}

/// Required operating-system cases derived from a backend capability manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceManifestV1 {
    pub schema_version: u16,
    pub native_ir_version: u16,
    pub backend: NativeBackendKind,
    pub requirements: Vec<NativeInputConformanceRequirementV1>,
}

impl NativeInputConformanceManifestV1 {
    /// Expands every role advertised with native press support into executable cases.
    pub fn from_capabilities(capabilities: &NativeCapabilities) -> Self {
        let mut requirements = Vec::new();
        for role_capabilities in &capabilities.role_overrides {
            let role = role_capabilities.role;
            if capabilities.support(NativeCapabilityFeature::Press, Some(role))
                != CapabilitySupport::Native
            {
                continue;
            }

            if capabilities.support(NativeCapabilityFeature::PressLifecycle, Some(role))
                == CapabilitySupport::Native
            {
                requirements.extend(
                    complete_scenarios(capabilities.backend)
                        .into_iter()
                        .map(|scenario| NativeInputConformanceRequirementV1::new(role, scenario)),
                );
            } else {
                requirements.push(NativeInputConformanceRequirementV1::new(
                    role,
                    NativeInputConformanceScenarioV1::TerminalActivation,
                ));
            }
        }

        Self {
            schema_version: NATIVE_INPUT_CONFORMANCE_VERSION_V1,
            native_ir_version: capabilities.ir_version,
            backend: capabilities.backend,
            requirements,
        }
    }
}

/// Provenance of an input trace.
///
/// Only `OperatingSystemAutomation` is eligible to satisfy a native capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeInputEvidenceSourceV1 {
    OperatingSystemAutomation,
    AdapterKernel,
    PortableRuntime,
}

/// Operating system on which native automation executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeOperatingSystemV1 {
    MacOS,
    Linux,
    Windows,
    Other,
}

/// Non-sensitive environment identity attached to an automation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceEnvironmentV1 {
    pub operating_system: NativeOperatingSystemV1,
    pub operating_system_version: String,
    pub toolkit_version: String,
    pub automation_driver: String,
}

impl NativeInputConformanceEnvironmentV1 {
    pub fn new(
        operating_system: NativeOperatingSystemV1,
        operating_system_version: impl Into<String>,
        toolkit_version: impl Into<String>,
        automation_driver: impl Into<String>,
    ) -> Self {
        Self {
            operating_system,
            operating_system_version: operating_system_version.into(),
            toolkit_version: toolkit_version.into(),
            automation_driver: automation_driver.into(),
        }
    }
}

/// Redacted semantic event retained in a native automation trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceEventV1 {
    pub node: HostNodeId,
    pub kind: NativeEventKind,
    pub modality: NativeInputModality,
    #[serde(default, skip_serializing_if = "is_false")]
    pub handled_activation: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub click_count: u8,
}

/// Events observed after an automation driver completed one manifest case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceObservationV1 {
    pub case: NativeInputConformanceCaseV1,
    pub target: HostNodeId,
    pub stimulus_dispatched: bool,
    pub events: Vec<NativeInputConformanceEventV1>,
}

impl NativeInputConformanceObservationV1 {
    /// Captures only semantic press lifecycle data and intentionally drops event values.
    pub fn capture(
        case: NativeInputConformanceCaseV1,
        target: HostNodeId,
        stimulus_dispatched: bool,
        events: &[NativeEvent],
    ) -> Self {
        let events = events
            .iter()
            .filter(|event| is_press_trace_kind(event.kind))
            .map(|event| NativeInputConformanceEventV1 {
                node: event.node,
                kind: event.kind,
                modality: event.effective_modality(),
                handled_activation: event.context.handled_activation,
                click_count: event.context.click_count,
            })
            .collect();
        Self {
            case,
            target,
            stimulus_dispatched,
            events,
        }
    }
}

/// One platform automation artifact submitted to the strict verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceRunV1 {
    pub schema_version: u16,
    pub native_ir_version: u16,
    pub backend: NativeBackendKind,
    pub evidence_source: NativeInputEvidenceSourceV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<NativeInputConformanceEnvironmentV1>,
    pub observations: Vec<NativeInputConformanceObservationV1>,
}

impl NativeInputConformanceRunV1 {
    pub fn new(backend: NativeBackendKind, evidence_source: NativeInputEvidenceSourceV1) -> Self {
        Self {
            schema_version: NATIVE_INPUT_CONFORMANCE_VERSION_V1,
            native_ir_version: NATIVE_IR_VERSION,
            backend,
            evidence_source,
            environment: None,
            observations: Vec::new(),
        }
    }

    pub fn environment(mut self, environment: NativeInputConformanceEnvironmentV1) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn observations(mut self, observations: Vec<NativeInputConformanceObservationV1>) -> Self {
        self.observations = observations;
        self
    }
}

/// Stable category for a conformance verification failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeInputConformanceIssueCodeV1 {
    SchemaVersionMismatch,
    NativeIrVersionMismatch,
    BackendMismatch,
    UnsupportedBackend,
    IneligibleEvidenceSource,
    MissingEnvironment,
    OperatingSystemMismatch,
    IncompleteEnvironment,
    MissingObservation,
    DuplicateObservation,
    UnexpectedObservation,
    StimulusNotDispatched,
    InvalidTarget,
    TargetMismatch,
    EventOrderMismatch,
    ModalityMismatch,
    KeyboardActivationMarkerMissing,
    ClickCountMismatch,
}

/// One actionable verification failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceIssueV1 {
    pub code: NativeInputConformanceIssueCodeV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case: Option<NativeInputConformanceCaseV1>,
    pub message: String,
}

impl NativeInputConformanceIssueV1 {
    pub(crate) fn new(
        code: NativeInputConformanceIssueCodeV1,
        case: Option<NativeInputConformanceCaseV1>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            case,
            message: message.into(),
        }
    }
}

/// Strict, computed result for one native input evidence artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeInputConformanceReportV1 {
    pub schema_version: u16,
    pub backend: NativeBackendKind,
    pub required_cases: usize,
    pub verified_cases: usize,
    pub issues: Vec<NativeInputConformanceIssueV1>,
}

impl NativeInputConformanceReportV1 {
    pub fn is_conformant(&self) -> bool {
        self.required_cases > 0
            && self.verified_cases == self.required_cases
            && self.issues.is_empty()
    }
}

fn complete_scenarios(backend: NativeBackendKind) -> Vec<NativeInputConformanceScenarioV1> {
    use NativeInputConformanceScenarioV1 as Scenario;

    let mut scenarios = vec![Scenario::MouseActivation, Scenario::PenActivation];
    if matches!(backend, NativeBackendKind::Gtk4 | NativeBackendKind::WinUI) {
        scenarios.push(Scenario::TouchActivation);
    }
    scenarios.extend([
        Scenario::KeyboardActivation,
        Scenario::AssistiveActivation,
        Scenario::MouseCancellation,
        Scenario::PenCancellation,
    ]);
    if matches!(backend, NativeBackendKind::Gtk4 | NativeBackendKind::WinUI) {
        scenarios.push(Scenario::TouchCancellation);
    }
    scenarios.extend([
        Scenario::KeyedRerenderCancellation,
        Scenario::DisabledMouseActivation,
        Scenario::DisabledPenActivation,
    ]);
    if matches!(backend, NativeBackendKind::Gtk4 | NativeBackendKind::WinUI) {
        scenarios.push(Scenario::DisabledTouchActivation);
    }
    scenarios.extend([
        Scenario::DisabledKeyboardActivation,
        Scenario::DisabledAssistiveActivation,
    ]);
    scenarios
}

fn scenario_contract(
    scenario: NativeInputConformanceScenarioV1,
) -> (NativeInputConformanceModalityV1, Vec<NativeEventKind>) {
    use NativeEventKind as Event;
    use NativeInputConformanceModalityV1 as Modality;
    use NativeInputConformanceScenarioV1 as Scenario;

    let activation = || {
        vec![
            Event::PressStart,
            Event::PressUp,
            Event::PressEnd,
            Event::Press,
        ]
    };
    match scenario {
        Scenario::MouseActivation => (Modality::Mouse, activation()),
        Scenario::PenActivation => (Modality::Pen, activation()),
        Scenario::TouchActivation => (Modality::Touch, activation()),
        Scenario::KeyboardActivation => (Modality::Keyboard, activation()),
        Scenario::AssistiveActivation => (Modality::Virtual, activation()),
        Scenario::MouseCancellation | Scenario::KeyedRerenderCancellation => {
            (Modality::Mouse, vec![Event::PressStart, Event::PressCancel])
        }
        Scenario::PenCancellation => (Modality::Pen, vec![Event::PressStart, Event::PressCancel]),
        Scenario::TouchCancellation => {
            (Modality::Touch, vec![Event::PressStart, Event::PressCancel])
        }
        Scenario::DisabledMouseActivation => (Modality::Mouse, Vec::new()),
        Scenario::DisabledPenActivation => (Modality::Pen, Vec::new()),
        Scenario::DisabledTouchActivation => (Modality::Touch, Vec::new()),
        Scenario::DisabledKeyboardActivation => (Modality::Keyboard, Vec::new()),
        Scenario::DisabledAssistiveActivation => (Modality::Virtual, Vec::new()),
        Scenario::TerminalActivation => (Modality::Unspecified, vec![Event::Press]),
    }
}

fn is_press_trace_kind(kind: NativeEventKind) -> bool {
    matches!(
        kind,
        NativeEventKind::PressStart
            | NativeEventKind::PressUp
            | NativeEventKind::PressEnd
            | NativeEventKind::PressCancel
            | NativeEventKind::Press
    )
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero(value: &u8) -> bool {
    *value == 0
}

#[cfg(test)]
mod tests;
