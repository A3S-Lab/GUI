use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::input::{NativeInputModality, NativeKeyModifiers};

use super::validation::{validate_finite_pair, validate_non_zero, validate_text};
use super::PlatformWindowId;

pub const MAX_PLATFORM_KEY_BYTES: usize = 128;
pub const MAX_PLATFORM_KEY_TEXT_BYTES: usize = 4 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformInputDeviceId(u64);

impl PlatformInputDeviceId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("input device id", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformPointerId(u64);

impl PlatformPointerId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("pointer id", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformPoint {
    pub x: f64,
    pub y: f64,
}

impl PlatformPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn validate(self, name: &str) -> GuiResult<()> {
        validate_finite_pair(name, self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformPointerPhase {
    Entered,
    Left,
    Moved,
    Pressed,
    Released,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformPointerButton {
    Primary,
    Secondary,
    Auxiliary,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformPointerEvent {
    pub window: PlatformWindowId,
    pub device: PlatformInputDeviceId,
    pub pointer: PlatformPointerId,
    pub modality: NativeInputModality,
    pub phase: PlatformPointerPhase,
    pub position: PlatformPoint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button: Option<PlatformPointerButton>,
    /// Currently pressed buttons using the UI Events `buttons` layout for the
    /// standard five buttons: primary `1`, secondary `2`, auxiliary `4`, back
    /// `8`, and forward `16`.
    #[serde(default)]
    pub pressed_buttons: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pressure: Option<f64>,
    #[serde(default, skip_serializing_if = "NativeKeyModifiers::is_empty")]
    pub modifiers: NativeKeyModifiers,
    pub timestamp_micros: u64,
}

impl PlatformPointerEvent {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        self.device.validate()?;
        self.pointer.validate()?;
        self.position.validate("pointer position")?;
        if !matches!(
            self.modality,
            NativeInputModality::Mouse | NativeInputModality::Touch | NativeInputModality::Pen
        ) {
            return Err(GuiError::host(
                "platform host pointer modality must be mouse, touch, or pen",
            ));
        }
        if matches!(
            self.phase,
            PlatformPointerPhase::Pressed | PlatformPointerPhase::Released
        ) && self.button.is_none()
        {
            return Err(GuiError::host(
                "platform host pressed and released pointer events need a button",
            ));
        }
        if self.button.is_some()
            && !matches!(
                self.phase,
                PlatformPointerPhase::Pressed | PlatformPointerPhase::Released
            )
        {
            return Err(GuiError::host(
                "platform host pointer buttons are valid only for pressed and released events",
            ));
        }
        if self
            .pressure
            .is_some_and(|pressure| !pressure.is_finite() || !(0.0..=1.0).contains(&pressure))
        {
            return Err(GuiError::host(
                "platform host pointer pressure must be finite and between zero and one",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformKeyState {
    Pressed,
    Released,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformKeyEvent {
    pub window: PlatformWindowId,
    pub device: PlatformInputDeviceId,
    /// Stable physical key location, using UI Events `code` names when known.
    pub physical_key: String,
    /// Layout-aware key value, using the UI Events `key` vocabulary.
    pub logical_key: String,
    /// Printable text produced by a press; releases and composition use `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub state: PlatformKeyState,
    #[serde(default)]
    pub repeat: bool,
    #[serde(default, skip_serializing_if = "NativeKeyModifiers::is_empty")]
    pub modifiers: NativeKeyModifiers,
    pub timestamp_micros: u64,
}

impl std::fmt::Debug for PlatformKeyEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlatformKeyEvent")
            .field("window", &self.window)
            .field("device", &self.device)
            .field("physical_key", &self.physical_key)
            .field("logical_key", &self.logical_key)
            .field("has_text", &self.text.is_some())
            .field("state", &self.state)
            .field("repeat", &self.repeat)
            .field("modifiers", &self.modifiers)
            .field("timestamp_micros", &self.timestamp_micros)
            .finish()
    }
}

impl PlatformKeyEvent {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        self.device.validate()?;
        validate_text(
            "physical key",
            &self.physical_key,
            MAX_PLATFORM_KEY_BYTES,
            false,
        )?;
        validate_text(
            "logical key",
            &self.logical_key,
            MAX_PLATFORM_KEY_BYTES,
            false,
        )?;
        if let Some(text) = &self.text {
            validate_text("key text", text, MAX_PLATFORM_KEY_TEXT_BYTES, true)?;
        }
        if self.repeat && self.state != PlatformKeyState::Pressed {
            return Err(GuiError::host(
                "platform host key repeat is valid only for pressed events",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformWheelDeltaMode {
    Pixels,
    Lines,
    Pages,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformWheelEvent {
    pub window: PlatformWindowId,
    pub device: PlatformInputDeviceId,
    pub position: PlatformPoint,
    pub delta: PlatformPoint,
    pub delta_mode: PlatformWheelDeltaMode,
    #[serde(default, skip_serializing_if = "NativeKeyModifiers::is_empty")]
    pub modifiers: NativeKeyModifiers,
    pub timestamp_micros: u64,
}

impl PlatformWheelEvent {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        self.device.validate()?;
        self.position.validate("wheel position")?;
        self.delta.validate("wheel delta")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformInputEvent {
    Pointer {
        event: PlatformPointerEvent,
    },
    Key {
        event: PlatformKeyEvent,
    },
    Wheel {
        event: PlatformWheelEvent,
    },
    ModifiersChanged {
        window: PlatformWindowId,
        device: PlatformInputDeviceId,
        modifiers: NativeKeyModifiers,
        timestamp_micros: u64,
    },
}

impl PlatformInputEvent {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Pointer { event } => event.validate(),
            Self::Key { event } => event.validate(),
            Self::Wheel { event } => event.validate(),
            Self::ModifiersChanged { window, device, .. } => {
                window.validate()?;
                device.validate()
            }
        }
    }
}
