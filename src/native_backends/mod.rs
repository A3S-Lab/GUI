//! Private helpers used by real OS native surfaces.
//!
//! This module is not a second native runtime layer. The public native surface
//! implementations live in `appkit_native`, `gtk4_native`, and `winui_native`.
//! Keep code here only when a platform surface needs focused support code that
//! would make the surface module harder to read, such as menu registries.

#[cfg(all(feature = "appkit-native", target_os = "macos"))]
pub(crate) mod appkit;

#[cfg(all(feature = "gtk4-native", target_os = "linux"))]
pub(crate) mod gtk4;

#[cfg(all(feature = "winui-native", target_os = "windows"))]
pub(crate) mod winui;
