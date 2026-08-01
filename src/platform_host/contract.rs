use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};

use super::validation::{validate_non_negative_rect, validate_non_zero, validate_positive_size};
use super::{
    PlatformAccessibilityAction, PlatformAccessibilitySnapshot, PlatformInputEvent,
    PlatformSystemEvent, PlatformSystemRequest, PlatformTextInputEvent, PlatformTextInputUpdate,
    PlatformWindowCommand, PlatformWindowEvent, PlatformWindowId,
};

pub const MAX_PLATFORM_HOST_COMMANDS: usize = 4096;
pub const MAX_PLATFORM_HOST_TRANSACTION_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_PLATFORM_HOST_EVENT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PLATFORM_PRESENTATION_DAMAGE_RECTS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformHostRevision(u64);

impl PlatformHostRevision {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("revision", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformPresentationRequest {
    pub window: PlatformWindowId,
    pub logical_size: Size,
    pub scale_factor: f64,
    pub scene_fingerprint: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub damage: Vec<Rect>,
}

impl PlatformPresentationRequest {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        validate_positive_size("presentation logical size", self.logical_size)?;
        if !self.scale_factor.is_finite() || self.scale_factor <= 0.0 {
            return Err(GuiError::host(
                "platform host presentation scale factor must be finite and greater than zero",
            ));
        }
        if self.damage.len() > MAX_PLATFORM_PRESENTATION_DAMAGE_RECTS {
            return Err(GuiError::host(format!(
                "platform host presentation exceeds the {}-rectangle damage limit",
                MAX_PLATFORM_PRESENTATION_DAMAGE_RECTS
            )));
        }
        for rect in &self.damage {
            validate_non_negative_rect("presentation damage", *rect)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformPresentationStatus {
    Queued,
    Presented,
    Dropped,
    SurfaceLost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformPresentationAck {
    pub revision: PlatformHostRevision,
    pub window: PlatformWindowId,
    pub status: PlatformPresentationStatus,
}

impl PlatformPresentationAck {
    pub fn validate(self) -> GuiResult<()> {
        self.revision.validate()?;
        self.window.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformHostCommand {
    Window {
        command: PlatformWindowCommand,
    },
    Present {
        request: PlatformPresentationRequest,
    },
    TextInput {
        update: PlatformTextInputUpdate,
    },
    Accessibility {
        snapshot: Box<PlatformAccessibilitySnapshot>,
    },
    System {
        request: PlatformSystemRequest,
    },
}

impl PlatformHostCommand {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Window { command } => command.validate(),
            Self::Present { request } => request.validate(),
            Self::TextInput { update } => update.validate(),
            Self::Accessibility { snapshot } => snapshot.validate(),
            Self::System { request } => request.validate(),
        }
    }

    pub fn redacted_for_diagnostics(&self) -> Self {
        match self {
            Self::TextInput { update } => Self::TextInput {
                update: update.redacted_for_diagnostics(),
            },
            Self::System { request } => Self::System {
                request: request.redacted_for_diagnostics(),
            },
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformHostTransaction {
    pub revision: PlatformHostRevision,
    pub commands: Vec<PlatformHostCommand>,
}

impl PlatformHostTransaction {
    pub fn validate(&self) -> GuiResult<()> {
        self.revision.validate()?;
        if self.commands.is_empty() {
            return Err(GuiError::host(
                "platform host transactions need at least one command",
            ));
        }
        if self.commands.len() > MAX_PLATFORM_HOST_COMMANDS {
            return Err(GuiError::host(format!(
                "platform host transaction exceeds the {}-command limit",
                MAX_PLATFORM_HOST_COMMANDS
            )));
        }

        let mut presentation_windows = BTreeSet::new();
        let mut accessibility_windows = BTreeSet::new();
        let mut system_requests = BTreeSet::new();
        for command in &self.commands {
            command.validate()?;
            match command {
                PlatformHostCommand::Present { request } => {
                    if !presentation_windows.insert(request.window) {
                        return Err(GuiError::host(format!(
                            "platform host transaction contains multiple presentations for window {}",
                            request.window.get()
                        )));
                    }
                }
                PlatformHostCommand::Accessibility { snapshot } => {
                    if !accessibility_windows.insert(snapshot.window) {
                        return Err(GuiError::host(format!(
                            "platform host transaction contains multiple accessibility snapshots for window {}",
                            snapshot.window.get()
                        )));
                    }
                }
                PlatformHostCommand::System { request } => {
                    if !system_requests.insert(request.id) {
                        return Err(GuiError::host(format!(
                            "platform host transaction contains duplicate system request id {}",
                            request.id.get()
                        )));
                    }
                }
                PlatformHostCommand::Window { .. } | PlatformHostCommand::TextInput { .. } => {}
            }
        }
        validate_encoded_size("transaction", self, MAX_PLATFORM_HOST_TRANSACTION_BYTES)
    }

    pub fn redacted_for_diagnostics(&self) -> Self {
        Self {
            revision: self.revision,
            commands: self
                .commands
                .iter()
                .map(PlatformHostCommand::redacted_for_diagnostics)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformHostCommitAck {
    pub revision: PlatformHostRevision,
    pub applied_commands: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presentations: Vec<PlatformPresentationAck>,
}

impl PlatformHostCommitAck {
    pub fn validate(&self) -> GuiResult<()> {
        self.revision.validate()?;
        if self.applied_commands > MAX_PLATFORM_HOST_COMMANDS {
            return Err(GuiError::host(
                "platform host commit acknowledgement exceeds the command limit",
            ));
        }
        if self.presentations.len() > self.applied_commands {
            return Err(GuiError::host(
                "platform host commit acknowledgement has more presentations than commands",
            ));
        }
        let mut windows = BTreeSet::new();
        for presentation in &self.presentations {
            presentation.validate()?;
            if presentation.revision != self.revision {
                return Err(GuiError::host(
                    "platform host presentation acknowledgement revision does not match its commit",
                ));
            }
            if !windows.insert(presentation.window) {
                return Err(GuiError::host(
                    "platform host commit acknowledgement contains duplicate presentation windows",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformHostEvent {
    Window { event: PlatformWindowEvent },
    Input { event: PlatformInputEvent },
    TextInput { event: PlatformTextInputEvent },
    Accessibility { action: PlatformAccessibilityAction },
    System { event: PlatformSystemEvent },
    Presentation { ack: PlatformPresentationAck },
}

impl PlatformHostEvent {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Window { event } => event.validate(),
            Self::Input { event } => event.validate(),
            Self::TextInput { event } => event.validate(),
            Self::Accessibility { action } => action.validate(),
            Self::System { event } => event.validate(),
            Self::Presentation { ack } => ack.validate(),
        }?;
        validate_encoded_size("event", self, MAX_PLATFORM_HOST_EVENT_BYTES)
    }
}

fn validate_encoded_size<T: Serialize>(name: &str, value: &T, max_bytes: usize) -> GuiResult<()> {
    let encoded = serde_json::to_vec(value).map_err(|error| {
        GuiError::host(format!(
            "platform host {name} could not be encoded for size validation: {error}"
        ))
    })?;
    if encoded.len() > max_bytes {
        return Err(GuiError::host(format!(
            "platform host {name} exceeds the {max_bytes}-byte limit"
        )));
    }
    Ok(())
}

/// Thread-affine zero-widget operating-system host.
///
/// A host prepares one complete revision, applies it atomically in [`Self::commit`],
/// and leaves the previous revision intact when preparation fails or the caller
/// invokes [`Self::rollback`]. A failed commit keeps the transaction pending so
/// the caller can explicitly roll it back or retry according to host policy.
/// Events are un-targeted OS facts; hit testing and action routing remain in the
/// portable runtime.
pub trait PlatformHost {
    fn prepare(&mut self, transaction: PlatformHostTransaction) -> GuiResult<()>;

    fn commit(&mut self) -> GuiResult<PlatformHostCommitAck>;

    fn rollback(&mut self) -> GuiResult<()>;

    fn poll_event(&mut self) -> GuiResult<Option<PlatformHostEvent>>;

    /// Releases windows, surfaces, platform callbacks, and queued events.
    /// Implementations reject shutdown while a transaction is pending so the
    /// owner must choose commit or rollback explicitly.
    fn shutdown(&mut self) -> GuiResult<()>;
}
