use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::Rect;

use super::validation::{validate_non_negative_rect, validate_non_zero, validate_text};
use super::PlatformWindowId;

pub const MAX_PLATFORM_SURROUNDING_TEXT_BYTES: usize = 1024 * 1024;
pub const MAX_PLATFORM_COMPOSITION_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformTextInputSessionId(u64);

impl PlatformTextInputSessionId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("text-input session id", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformTextRange {
    pub start: u32,
    pub end: u32,
}

impl PlatformTextRange {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn validate(self, name: &str) -> GuiResult<()> {
        if self.start > self.end {
            Err(GuiError::host(format!(
                "platform host {name} start cannot exceed its end"
            )))
        } else {
            Ok(())
        }
    }

    fn validate_for_text(self, name: &str, text: &str) -> GuiResult<()> {
        self.validate(name)?;
        let start = self.start as usize;
        let end = self.end as usize;
        if end > text.len() || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
            return Err(GuiError::host(format!(
                "platform host {name} must use UTF-8 boundaries inside the surrounding text"
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformTextInputPurpose {
    Text,
    Search,
    Email,
    Url,
    Number,
    Phone,
    Password,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformTextInputState {
    pub session: PlatformTextInputSessionId,
    pub window: PlatformWindowId,
    pub purpose: PlatformTextInputPurpose,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surrounding_text: Option<String>,
    pub selection: PlatformTextRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composition: Option<PlatformTextRange>,
    pub candidate_rect: Rect,
}

impl std::fmt::Debug for PlatformTextInputState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlatformTextInputState")
            .field("session", &self.session)
            .field("window", &self.window)
            .field("purpose", &self.purpose)
            .field(
                "surrounding_text_bytes",
                &self.surrounding_text.as_ref().map(String::len),
            )
            .field("selection", &self.selection)
            .field("composition", &self.composition)
            .field("candidate_rect", &self.candidate_rect)
            .finish()
    }
}

impl PlatformTextInputState {
    pub fn validate(&self) -> GuiResult<()> {
        self.session.validate()?;
        self.window.validate()?;
        self.selection.validate("text selection")?;
        if let Some(composition) = self.composition {
            composition.validate("text composition")?;
        }
        validate_non_negative_rect("text candidate rectangle", self.candidate_rect)?;
        if self.purpose == PlatformTextInputPurpose::Password {
            if self.surrounding_text.is_some() || self.composition.is_some() {
                return Err(GuiError::host(
                    "platform host password sessions cannot expose surrounding or composition text",
                ));
            }
            return Ok(());
        }
        let Some(text) = &self.surrounding_text else {
            return Err(GuiError::host(
                "platform host non-password text sessions need surrounding text",
            ));
        };
        validate_text(
            "surrounding text",
            text,
            MAX_PLATFORM_SURROUNDING_TEXT_BYTES,
            true,
        )?;
        self.selection
            .validate_for_text("text selection", text.as_str())?;
        if let Some(composition) = self.composition {
            composition.validate_for_text("text composition", text.as_str())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformTextInputUpdate {
    Activate { state: PlatformTextInputState },
    Update { state: PlatformTextInputState },
    Deactivate { session: PlatformTextInputSessionId },
}

impl PlatformTextInputUpdate {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Activate { state } | Self::Update { state } => state.validate(),
            Self::Deactivate { session } => session.validate(),
        }
    }

    pub fn redacted_for_diagnostics(&self) -> Self {
        match self {
            Self::Activate { state } => Self::Activate {
                state: redacted_state(state),
            },
            Self::Update { state } => Self::Update {
                state: redacted_state(state),
            },
            Self::Deactivate { session } => Self::Deactivate { session: *session },
        }
    }
}

fn redacted_state(state: &PlatformTextInputState) -> PlatformTextInputState {
    let mut state = state.clone();
    state.surrounding_text = state
        .surrounding_text
        .as_ref()
        .map(|text| "*".repeat(text.len()));
    state
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformTextInputCommand {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveToLineStart,
    MoveToLineEnd,
    DeleteBackward,
    DeleteForward,
    SelectAll,
    Undo,
    Redo,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformTextInputEvent {
    Commit {
        session: PlatformTextInputSessionId,
        text: String,
    },
    Composition {
        session: PlatformTextInputSessionId,
        text: String,
        selection: PlatformTextRange,
    },
    DeleteSurrounding {
        session: PlatformTextInputSessionId,
        before_bytes: u32,
        after_bytes: u32,
    },
    Command {
        session: PlatformTextInputSessionId,
        command: PlatformTextInputCommand,
    },
    Cancelled {
        session: PlatformTextInputSessionId,
    },
}

impl std::fmt::Debug for PlatformTextInputEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Commit { session, text } => formatter
                .debug_struct("Commit")
                .field("session", session)
                .field("text_bytes", &text.len())
                .finish(),
            Self::Composition {
                session,
                text,
                selection,
            } => formatter
                .debug_struct("Composition")
                .field("session", session)
                .field("text_bytes", &text.len())
                .field("selection", selection)
                .finish(),
            Self::DeleteSurrounding {
                session,
                before_bytes,
                after_bytes,
            } => formatter
                .debug_struct("DeleteSurrounding")
                .field("session", session)
                .field("before_bytes", before_bytes)
                .field("after_bytes", after_bytes)
                .finish(),
            Self::Command { session, command } => formatter
                .debug_struct("Command")
                .field("session", session)
                .field("command", command)
                .finish(),
            Self::Cancelled { session } => formatter
                .debug_struct("Cancelled")
                .field("session", session)
                .finish(),
        }
    }
}

impl PlatformTextInputEvent {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Commit { session, text } => {
                session.validate()?;
                validate_text("committed text", text, MAX_PLATFORM_COMPOSITION_BYTES, true)
            }
            Self::Composition {
                session,
                text,
                selection,
            } => {
                session.validate()?;
                validate_text(
                    "composition text",
                    text,
                    MAX_PLATFORM_COMPOSITION_BYTES,
                    true,
                )?;
                selection.validate_for_text("composition selection", text)
            }
            Self::DeleteSurrounding { session, .. }
            | Self::Command { session, .. }
            | Self::Cancelled { session } => session.validate(),
        }
    }
}
