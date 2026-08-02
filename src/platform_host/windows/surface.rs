#![allow(unsafe_code)]

use std::num::NonZeroIsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, Win32WindowHandle, WindowHandle,
};

use super::super::PlatformWindowId;

pub(super) struct WindowsSurfaceState {
    hwnd: NonZeroIsize,
    hinstance: NonZeroIsize,
    alive: AtomicBool,
}

impl WindowsSurfaceState {
    pub(super) fn new(hwnd: NonZeroIsize, hinstance: NonZeroIsize) -> Self {
        Self {
            hwnd,
            hinstance,
            alive: AtomicBool::new(true),
        }
    }

    pub(super) fn has_external_lease(owner: &Arc<Self>) -> bool {
        Arc::strong_count(owner) > 1
    }

    pub(super) fn mark_destroyed(&self) {
        self.alive.store(false, Ordering::Release);
    }

    fn handles(&self) -> Result<(NonZeroIsize, NonZeroIsize), HandleError> {
        self.alive
            .load(Ordering::Acquire)
            .then_some((self.hwnd, self.hinstance))
            .ok_or(HandleError::Unavailable)
    }
}

/// Owned Win32 surface lifetime token for one A3S top-level window.
///
/// Cloning this value creates a surface lease. The owning host refuses to
/// destroy the HWND until every external lease is released, so safe Graphics
/// code may retain the token for the complete GPU surface lifetime. It
/// creates no WinUI, XAML, child control, or drawing object.
#[derive(Clone)]
pub struct WindowsSurfaceHandle {
    window: PlatformWindowId,
    state: Arc<WindowsSurfaceState>,
    physical_size: (u32, u32),
    scale_factor: f64,
}

impl std::fmt::Debug for WindowsSurfaceHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WindowsSurfaceHandle")
            .field("window", &self.window)
            .field("physical_size", &self.physical_size)
            .field("scale_factor", &self.scale_factor)
            .field("alive", &self.state.alive.load(Ordering::Acquire))
            .finish_non_exhaustive()
    }
}

impl WindowsSurfaceHandle {
    pub(super) fn new(
        window: PlatformWindowId,
        state: Arc<WindowsSurfaceState>,
        physical_size: (u32, u32),
        scale_factor: f64,
    ) -> Self {
        Self {
            window,
            state,
            physical_size,
            scale_factor,
        }
    }

    /// Returns the portable identifier of the owning top-level window.
    pub const fn window(&self) -> PlatformWindowId {
        self.window
    }

    /// Returns the physical client-area dimensions observed when leased.
    pub const fn physical_size(&self) -> (u32, u32) {
        self.physical_size
    }

    /// Returns the physical-pixels-per-logical-pixel ratio observed when leased.
    pub const fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Reports whether the owning HWND is still available.
    pub fn is_alive(&self) -> bool {
        self.state.alive.load(Ordering::Acquire)
    }
}

impl HasDisplayHandle for WindowsSurfaceHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.state.handles()?;
        Ok(DisplayHandle::windows())
    }
}

impl HasWindowHandle for WindowsSurfaceHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let (hwnd, hinstance) = self.state.handles()?;
        let mut handle = Win32WindowHandle::new(hwnd);
        handle.hinstance = Some(hinstance);
        // SAFETY: WindowsSurfaceHandle retains the shared lifetime state for
        // this borrow, and NativeWindow refuses DestroyWindow while this lease
        // exists. hwnd and hinstance therefore remain live through the borrow.
        Ok(unsafe { WindowHandle::borrow_raw(handle.into()) })
    }
}
