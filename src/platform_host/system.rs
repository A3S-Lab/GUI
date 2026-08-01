use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::native::ValueSensitivity;

use super::validation::{validate_non_zero, validate_text};
use super::PlatformWindowId;

pub const MAX_PLATFORM_SYSTEM_TEXT_BYTES: usize = 64 * 1024;
pub const MAX_PLATFORM_CLIPBOARD_BYTES: usize = 1024 * 1024;
pub const MAX_PLATFORM_FILE_FILTERS: usize = 64;
pub const MAX_PLATFORM_MENU_ITEMS: usize = 4096;
pub const MAX_PLATFORM_MENU_DEPTH: usize = 32;
const MAX_PLATFORM_FILE_RESULTS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformSystemRequestId(u64);

impl PlatformSystemRequestId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn validate(self) -> GuiResult<()> {
        validate_non_zero("system request id", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformClipboardFormat {
    Text,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformClipboardContent {
    pub format: PlatformClipboardFormat,
    pub text: String,
    #[serde(default)]
    pub sensitivity: ValueSensitivity,
}

impl std::fmt::Debug for PlatformClipboardContent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlatformClipboardContent")
            .field("format", &self.format)
            .field("text_bytes", &self.text.len())
            .field("sensitivity", &self.sensitivity)
            .finish()
    }
}

impl PlatformClipboardContent {
    pub fn validate(&self) -> GuiResult<()> {
        validate_text(
            "clipboard text",
            &self.text,
            MAX_PLATFORM_CLIPBOARD_BYTES,
            true,
        )
    }

    fn redacted_for_diagnostics(&self) -> Self {
        let mut content = self.clone();
        if content.sensitivity.is_sensitive() {
            content.text = "*".repeat(content.text.len());
        }
        content
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformFileFilter {
    pub label: String,
    #[serde(default)]
    pub extensions: Vec<String>,
}

impl PlatformFileFilter {
    fn validate(&self) -> GuiResult<()> {
        validate_text(
            "file filter label",
            &self.label,
            MAX_PLATFORM_SYSTEM_TEXT_BYTES,
            false,
        )?;
        if self.extensions.len() > MAX_PLATFORM_FILE_FILTERS {
            return Err(GuiError::host(format!(
                "platform host file filter exceeds the {}-extension limit",
                MAX_PLATFORM_FILE_FILTERS
            )));
        }
        for extension in &self.extensions {
            validate_text("file extension", extension, 256, false)?;
            if extension.contains(['/', '\\']) || extension.starts_with('.') {
                return Err(GuiError::host(
                    "platform host file extensions must not contain separators or a leading dot",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformFilePickerMode {
    OpenFile,
    OpenFiles,
    OpenDirectory,
    SaveFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformFilePickerRequest {
    pub mode: PlatformFilePickerMode,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<PlatformFileFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<String>,
}

impl PlatformFilePickerRequest {
    fn validate(&self) -> GuiResult<()> {
        validate_text(
            "file picker title",
            &self.title,
            MAX_PLATFORM_SYSTEM_TEXT_BYTES,
            true,
        )?;
        if self.filters.len() > MAX_PLATFORM_FILE_FILTERS {
            return Err(GuiError::host(format!(
                "platform host file picker exceeds the {}-filter limit",
                MAX_PLATFORM_FILE_FILTERS
            )));
        }
        for filter in &self.filters {
            filter.validate()?;
        }
        if let Some(name) = &self.suggested_name {
            validate_text(
                "suggested file name",
                name,
                MAX_PLATFORM_SYSTEM_TEXT_BYTES,
                false,
            )?;
            if name.contains(['/', '\\']) {
                return Err(GuiError::host(
                    "platform host suggested file names must not contain path separators",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformPermission {
    Notifications,
    Camera,
    Microphone,
    Location,
    Accessibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformNotification {
    pub title: String,
    pub body: String,
}

impl PlatformNotification {
    fn validate(&self) -> GuiResult<()> {
        validate_text(
            "notification title",
            &self.title,
            MAX_PLATFORM_SYSTEM_TEXT_BYTES,
            false,
        )?;
        validate_text(
            "notification body",
            &self.body,
            MAX_PLATFORM_SYSTEM_TEXT_BYTES,
            true,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformMenuItem {
    pub id: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub checked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PlatformMenuItem>,
}

impl PlatformMenuItem {
    fn validate_tree(items: &[Self]) -> GuiResult<()> {
        let mut count = 0usize;
        let mut ids = std::collections::BTreeSet::new();
        for item in items {
            item.validate(1, &mut count, &mut ids)?;
        }
        Ok(())
    }

    fn validate(
        &self,
        depth: usize,
        count: &mut usize,
        ids: &mut std::collections::BTreeSet<String>,
    ) -> GuiResult<()> {
        if depth > MAX_PLATFORM_MENU_DEPTH {
            return Err(GuiError::host(format!(
                "platform host application menu exceeds the {}-level depth limit",
                MAX_PLATFORM_MENU_DEPTH
            )));
        }
        *count += 1;
        if *count > MAX_PLATFORM_MENU_ITEMS {
            return Err(GuiError::host(format!(
                "platform host application menu exceeds the {}-item limit",
                MAX_PLATFORM_MENU_ITEMS
            )));
        }
        validate_text("menu item id", &self.id, 1024, false)?;
        validate_text(
            "menu item label",
            &self.label,
            MAX_PLATFORM_SYSTEM_TEXT_BYTES,
            false,
        )?;
        if !ids.insert(self.id.clone()) {
            return Err(GuiError::host(format!(
                "platform host application menu contains duplicate item id {:?}",
                self.id
            )));
        }
        if let Some(action_id) = &self.action_id {
            validate_text("menu action id", action_id, 1024, false)?;
        }
        if !self.children.is_empty() && self.action_id.is_some() {
            return Err(GuiError::host(
                "platform host application menu branches cannot also carry an action",
            ));
        }
        for child in &self.children {
            child.validate(depth + 1, count, ids)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformSystemCommand {
    ReadClipboard { format: PlatformClipboardFormat },
    WriteClipboard { content: PlatformClipboardContent },
    PickFiles { picker: PlatformFilePickerRequest },
    OpenUrl { url: String },
    RequestPermission { permission: PlatformPermission },
    ShowNotification { notification: PlatformNotification },
    SetApplicationMenu { items: Vec<PlatformMenuItem> },
}

impl std::fmt::Debug for PlatformSystemCommand {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadClipboard { format } => formatter
                .debug_struct("ReadClipboard")
                .field("format", format)
                .finish(),
            Self::WriteClipboard { content } => formatter
                .debug_struct("WriteClipboard")
                .field("content", content)
                .finish(),
            Self::PickFiles { picker } => formatter
                .debug_struct("PickFiles")
                .field("mode", &picker.mode)
                .field("filter_count", &picker.filters.len())
                .field("has_suggested_name", &picker.suggested_name.is_some())
                .finish(),
            Self::OpenUrl { url } => formatter
                .debug_struct("OpenUrl")
                .field("url_bytes", &url.len())
                .finish(),
            Self::RequestPermission { permission } => formatter
                .debug_struct("RequestPermission")
                .field("permission", permission)
                .finish(),
            Self::ShowNotification { notification } => formatter
                .debug_struct("ShowNotification")
                .field("title_bytes", &notification.title.len())
                .field("body_bytes", &notification.body.len())
                .finish(),
            Self::SetApplicationMenu { items } => formatter
                .debug_struct("SetApplicationMenu")
                .field("root_items", &items.len())
                .finish(),
        }
    }
}

impl PlatformSystemCommand {
    fn validate(&self) -> GuiResult<()> {
        match self {
            Self::ReadClipboard { .. } | Self::RequestPermission { .. } => Ok(()),
            Self::WriteClipboard { content } => content.validate(),
            Self::PickFiles { picker } => picker.validate(),
            Self::OpenUrl { url } => {
                validate_text("URL", url, MAX_PLATFORM_SYSTEM_TEXT_BYTES, false)?;
                if !url.contains(':') || url.chars().any(char::is_whitespace) {
                    return Err(GuiError::host(
                        "platform host URLs need a scheme and cannot contain whitespace",
                    ));
                }
                Ok(())
            }
            Self::ShowNotification { notification } => notification.validate(),
            Self::SetApplicationMenu { items } => PlatformMenuItem::validate_tree(items),
        }
    }

    pub fn redacted_for_diagnostics(&self) -> Self {
        match self {
            Self::WriteClipboard { content } => Self::WriteClipboard {
                content: content.redacted_for_diagnostics(),
            },
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformSystemRequest {
    pub id: PlatformSystemRequestId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<PlatformWindowId>,
    pub command: PlatformSystemCommand,
}

impl PlatformSystemRequest {
    pub fn validate(&self) -> GuiResult<()> {
        self.id.validate()?;
        if let Some(window) = self.window {
            window.validate()?;
        }
        self.command.validate()
    }

    pub fn redacted_for_diagnostics(&self) -> Self {
        Self {
            id: self.id,
            window: self.window,
            command: self.command.redacted_for_diagnostics(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformSystemOutcome {
    Completed,
    Cancelled,
    Clipboard { content: PlatformClipboardContent },
    Files { paths: Vec<String> },
    Permission { granted: bool },
    Failed { code: String, message: String },
}

impl std::fmt::Debug for PlatformSystemOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completed => formatter.write_str("Completed"),
            Self::Cancelled => formatter.write_str("Cancelled"),
            Self::Clipboard { content } => formatter
                .debug_struct("Clipboard")
                .field("content", content)
                .finish(),
            Self::Files { paths } => formatter
                .debug_struct("Files")
                .field("path_count", &paths.len())
                .finish(),
            Self::Permission { granted } => formatter
                .debug_struct("Permission")
                .field("granted", granted)
                .finish(),
            Self::Failed { code, message } => formatter
                .debug_struct("Failed")
                .field("code", code)
                .field("message_bytes", &message.len())
                .finish(),
        }
    }
}

impl PlatformSystemOutcome {
    fn validate(&self) -> GuiResult<()> {
        match self {
            Self::Completed | Self::Cancelled | Self::Permission { .. } => Ok(()),
            Self::Clipboard { content } => content.validate(),
            Self::Files { paths } => {
                if paths.len() > MAX_PLATFORM_FILE_RESULTS {
                    return Err(GuiError::host(format!(
                        "platform host file result exceeds the {}-path limit",
                        MAX_PLATFORM_FILE_RESULTS
                    )));
                }
                for path in paths {
                    validate_text(
                        "file result path",
                        path,
                        MAX_PLATFORM_SYSTEM_TEXT_BYTES,
                        false,
                    )?;
                }
                Ok(())
            }
            Self::Failed { code, message } => {
                validate_text("system error code", code, 1024, false)?;
                validate_text(
                    "system error message",
                    message,
                    MAX_PLATFORM_SYSTEM_TEXT_BYTES,
                    true,
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformSystemEvent {
    pub id: PlatformSystemRequestId,
    pub outcome: PlatformSystemOutcome,
}

impl PlatformSystemEvent {
    pub fn validate(&self) -> GuiResult<()> {
        self.id.validate()?;
        self.outcome.validate()
    }
}

const fn default_true() -> bool {
    true
}
