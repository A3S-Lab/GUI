use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::Size;

use super::validation::{validate_non_zero, validate_positive_size, validate_text};

pub const MAX_PLATFORM_WINDOW_TITLE_BYTES: usize = 4 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformWindowId(u64);

impl PlatformWindowId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("window id", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformWindowSpec {
    pub id: PlatformWindowId,
    pub title: String,
    pub logical_size: Size,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<Size>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size: Option<Size>,
    #[serde(default = "default_true")]
    pub resizable: bool,
    #[serde(default = "default_true")]
    pub visible: bool,
}

impl PlatformWindowSpec {
    pub fn validate(&self) -> GuiResult<()> {
        self.id.validate()?;
        validate_text(
            "window title",
            &self.title,
            MAX_PLATFORM_WINDOW_TITLE_BYTES,
            true,
        )?;
        validate_positive_size("window logical size", self.logical_size)?;
        if let Some(size) = self.min_size {
            validate_positive_size("window minimum size", size)?;
            if size.width > self.logical_size.width || size.height > self.logical_size.height {
                return Err(GuiError::host(
                    "platform host window minimum size cannot exceed its initial size",
                ));
            }
        }
        if let Some(size) = self.max_size {
            validate_positive_size("window maximum size", size)?;
            if size.width < self.logical_size.width || size.height < self.logical_size.height {
                return Err(GuiError::host(
                    "platform host window maximum size cannot be smaller than its initial size",
                ));
            }
        }
        if let (Some(min), Some(max)) = (self.min_size, self.max_size) {
            if min.width > max.width || min.height > max.height {
                return Err(GuiError::host(
                    "platform host window minimum size cannot exceed its maximum size",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformWindowCommand {
    Open {
        spec: PlatformWindowSpec,
    },
    SetTitle {
        window: PlatformWindowId,
        title: String,
    },
    Resize {
        window: PlatformWindowId,
        logical_size: Size,
    },
    SetConstraints {
        window: PlatformWindowId,
        min_size: Option<Size>,
        max_size: Option<Size>,
    },
    SetResizable {
        window: PlatformWindowId,
        resizable: bool,
    },
    SetVisible {
        window: PlatformWindowId,
        visible: bool,
    },
    RequestRedraw {
        window: PlatformWindowId,
    },
    Close {
        window: PlatformWindowId,
    },
}

impl PlatformWindowCommand {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Open { spec } => spec.validate(),
            Self::SetTitle { window, title } => {
                window.validate()?;
                validate_text("window title", title, MAX_PLATFORM_WINDOW_TITLE_BYTES, true)
            }
            Self::Resize {
                window,
                logical_size,
            } => {
                window.validate()?;
                validate_positive_size("window logical size", *logical_size)
            }
            Self::SetConstraints {
                window,
                min_size,
                max_size,
            } => {
                window.validate()?;
                if let Some(size) = min_size {
                    validate_positive_size("window minimum size", *size)?;
                }
                if let Some(size) = max_size {
                    validate_positive_size("window maximum size", *size)?;
                }
                if let (Some(min), Some(max)) = (min_size, max_size) {
                    if min.width > max.width || min.height > max.height {
                        return Err(GuiError::host(
                            "platform host window minimum size cannot exceed its maximum size",
                        ));
                    }
                }
                Ok(())
            }
            Self::SetResizable { window, .. }
            | Self::SetVisible { window, .. }
            | Self::RequestRedraw { window }
            | Self::Close { window } => window.validate(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformWindowEvent {
    Resized {
        window: PlatformWindowId,
        logical_size: Size,
    },
    ScaleChanged {
        window: PlatformWindowId,
        scale_factor: f64,
    },
    FocusChanged {
        window: PlatformWindowId,
        focused: bool,
    },
    OcclusionChanged {
        window: PlatformWindowId,
        occluded: bool,
    },
    RedrawRequested {
        window: PlatformWindowId,
    },
    CloseRequested {
        window: PlatformWindowId,
    },
    Closed {
        window: PlatformWindowId,
    },
}

impl PlatformWindowEvent {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Resized {
                window,
                logical_size,
            } => {
                window.validate()?;
                validate_positive_size("window logical size", *logical_size)
            }
            Self::ScaleChanged {
                window,
                scale_factor,
            } => {
                window.validate()?;
                if !scale_factor.is_finite() || *scale_factor <= 0.0 {
                    return Err(GuiError::host(
                        "platform host window scale factor must be finite and greater than zero",
                    ));
                }
                Ok(())
            }
            Self::FocusChanged { window, .. }
            | Self::OcclusionChanged { window, .. }
            | Self::RedrawRequested { window }
            | Self::CloseRequested { window }
            | Self::Closed { window } => window.validate(),
        }
    }
}

const fn default_true() -> bool {
    true
}
