#![allow(unsafe_code)]

use std::marker::PhantomData;
use std::num::NonZeroIsize;

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, Win32WindowHandle, WindowHandle,
};

use super::super::PlatformWindowId;

/// Borrowed Win32 surface identity for a committed A3S top-level window.
///
/// The lifetime prevents safe callers from mutating or closing the owning host
/// while the handle is borrowed. It creates no WinUI, XAML, child control, or
/// drawing object; a Graphics presenter may use it to attach a DXGI/DX12
/// surface.
pub struct WindowsSurfaceHandle<'a> {
    window: PlatformWindowId,
    hwnd: NonZeroIsize,
    hinstance: NonZeroIsize,
    physical_size: (u32, u32),
    scale_factor: f64,
    _owner: PhantomData<&'a ()>,
}

impl std::fmt::Debug for WindowsSurfaceHandle<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WindowsSurfaceHandle")
            .field("window", &self.window)
            .field("physical_size", &self.physical_size)
            .field("scale_factor", &self.scale_factor)
            .finish_non_exhaustive()
    }
}

impl<'a> WindowsSurfaceHandle<'a> {
    pub(super) fn new(
        window: PlatformWindowId,
        hwnd: NonZeroIsize,
        hinstance: NonZeroIsize,
        physical_size: (u32, u32),
        scale_factor: f64,
    ) -> Self {
        Self {
            window,
            hwnd,
            hinstance,
            physical_size,
            scale_factor,
            _owner: PhantomData,
        }
    }

    /// Returns the portable identifier of the owning top-level window.
    pub const fn window(&self) -> PlatformWindowId {
        self.window
    }

    /// Returns the current physical client-area dimensions.
    pub const fn physical_size(&self) -> (u32, u32) {
        self.physical_size
    }

    /// Returns the current physical-pixels-per-logical-pixel ratio.
    pub const fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}

impl HasDisplayHandle for WindowsSurfaceHandle<'_> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::windows())
    }
}

impl HasWindowHandle for WindowsSurfaceHandle<'_> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let mut handle = Win32WindowHandle::new(self.hwnd);
        handle.hinstance = Some(self.hinstance);
        // SAFETY: both handles belong to the live NativeWindow borrowed by the
        // lifetime of this WindowsSurfaceHandle.
        Ok(unsafe { WindowHandle::borrow_raw(handle.into()) })
    }
}
