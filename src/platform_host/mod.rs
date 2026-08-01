//! Zero-widget operating-system host boundary for the self-drawn renderer.
//!
//! The records in this module describe top-level windows, presentation,
//! un-targeted input, text services, accessibility projection, and explicit
//! system-service requests. They deliberately contain no component tree,
//! portable style, native-control operation, toolkit object, or GPU handle.
//!
//! [`PlatformHost`] itself is not required to be `Send` or `Sync`: real event
//! loops and window handles remain on their owning OS thread. Every record that
//! crosses that boundary is `Send + Sync` and validated before mutation.

mod accessibility;
mod contract;
mod input;
mod recording;
mod system;
mod text_input;
mod validation;
mod window;

pub use accessibility::{
    PlatformAccessibilityAction, PlatformAccessibilityActionKind, PlatformAccessibilityNode,
    PlatformAccessibilitySnapshot, PlatformElementId, MAX_PLATFORM_ACCESSIBILITY_BYTES,
    MAX_PLATFORM_ACCESSIBILITY_DEPTH, MAX_PLATFORM_ACCESSIBILITY_NODES,
    MAX_PLATFORM_ELEMENT_ID_BYTES,
};
pub use contract::{
    PlatformHost, PlatformHostCommand, PlatformHostCommitAck, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformPresentationAck,
    PlatformPresentationRequest, PlatformPresentationStatus, MAX_PLATFORM_HOST_COMMANDS,
    MAX_PLATFORM_HOST_EVENT_BYTES, MAX_PLATFORM_HOST_TRANSACTION_BYTES,
    MAX_PLATFORM_PRESENTATION_DAMAGE_RECTS,
};
pub use input::{
    PlatformInputDeviceId, PlatformInputEvent, PlatformKeyEvent, PlatformKeyState, PlatformPoint,
    PlatformPointerButton, PlatformPointerEvent, PlatformPointerId, PlatformPointerPhase,
    PlatformWheelDeltaMode, PlatformWheelEvent, MAX_PLATFORM_KEY_BYTES,
    MAX_PLATFORM_KEY_TEXT_BYTES,
};
pub use recording::{
    RecordingPlatformHost, DEFAULT_PLATFORM_HOST_EVENT_QUEUE_LIMIT,
    DEFAULT_PLATFORM_HOST_HISTORY_LIMIT,
};
pub use system::{
    PlatformClipboardContent, PlatformClipboardFormat, PlatformFileFilter, PlatformFilePickerMode,
    PlatformFilePickerRequest, PlatformMenuItem, PlatformNotification, PlatformPermission,
    PlatformSystemCommand, PlatformSystemEvent, PlatformSystemOutcome, PlatformSystemRequest,
    PlatformSystemRequestId, MAX_PLATFORM_CLIPBOARD_BYTES, MAX_PLATFORM_FILE_FILTERS,
    MAX_PLATFORM_MENU_DEPTH, MAX_PLATFORM_MENU_ITEMS, MAX_PLATFORM_SYSTEM_TEXT_BYTES,
};
pub use text_input::{
    PlatformTextInputCommand, PlatformTextInputEvent, PlatformTextInputPurpose,
    PlatformTextInputSessionId, PlatformTextInputState, PlatformTextInputUpdate, PlatformTextRange,
    MAX_PLATFORM_COMPOSITION_BYTES, MAX_PLATFORM_SURROUNDING_TEXT_BYTES,
};
pub use window::{
    PlatformWindowCommand, PlatformWindowEvent, PlatformWindowId, PlatformWindowSpec,
    MAX_PLATFORM_WINDOW_TITLE_BYTES,
};

#[cfg(test)]
mod tests;
