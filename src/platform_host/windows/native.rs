#![allow(unsafe_code)]

use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::num::NonZeroIsize;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::{null, null_mut};
use std::sync::Arc;

use windows_sys::Win32::Foundation::{
    GetLastError, SetLastError, ERROR_CLASS_ALREADY_EXISTS, HINSTANCE, HWND, LPARAM, LRESULT, RECT,
    WPARAM,
};
use windows_sys::Win32::Graphics::Gdi::{BeginPaint, EndPaint, InvalidateRect, PAINTSTRUCT};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::HiDpi::{
    AdjustWindowRectExForDpi, GetDpiForSystem, GetDpiForWindow, SetThreadDpiAwarenessContext,
    DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect,
    GetWindowLongPtrW, LoadCursorW, PeekMessageW, RegisterClassExW, SetWindowLongPtrW,
    SetWindowPos, SetWindowTextW, ShowWindow, TranslateMessage, CREATESTRUCTW, CS_DBLCLKS,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, GWL_STYLE, IDC_ARROW, MINMAXINFO, MSG,
    PM_REMOVE, SIZE_MINIMIZED, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, SW_HIDE,
    SW_SHOW, WM_CLOSE, WM_DESTROY, WM_DPICHANGED, WM_ERASEBKGND, WM_GETMINMAXINFO, WM_KILLFOCUS,
    WM_NCCREATE, WM_NCDESTROY, WM_PAINT, WM_QUIT, WM_SETFOCUS, WM_SIZE, WNDCLASSEXW,
    WS_MAXIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_THICKFRAME,
};

use crate::error::{GuiError, GuiResult};
use crate::geometry::Size;

use super::super::{PlatformHostEvent, PlatformWindowEvent, PlatformWindowId, PlatformWindowSpec};
use super::events::WindowsEventQueue;
use super::surface::WindowsSurfaceState;

const WINDOW_CLASS_NAME: &str = "A3S.Gui.SelfDrawn.Window.v1";
const BASE_DPI: f64 = 96.0;
const MAX_MESSAGES_PER_POLL: usize = 4096;

pub(super) fn register_window_class() -> GuiResult<HINSTANCE> {
    // SAFETY: a null module name requests the module of the current process.
    let hinstance = unsafe { GetModuleHandleW(null()) };
    if hinstance.is_null() {
        return Err(last_error("GetModuleHandleW"));
    }
    let class_name = wide(WINDOW_CLASS_NAME)?;
    // SAFETY: the predefined cursor identifier and null module are documented
    // for loading a shared system cursor.
    let cursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    if cursor.is_null() {
        return Err(last_error("LoadCursorW"));
    }
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: null_mut(),
        hCursor: cursor,
        hbrBackground: null_mut(),
        lpszMenuName: null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: null_mut(),
    };
    // SAFETY: every pointer in the descriptor is valid for this call and the
    // operating system copies the class name during registration.
    let atom = unsafe { RegisterClassExW(&class) };
    if atom == 0 {
        // SAFETY: GetLastError has no preconditions and is read immediately
        // after the failed registration call.
        let error = unsafe { GetLastError() };
        if error != ERROR_CLASS_ALREADY_EXISTS {
            return Err(error_code("RegisterClassExW", error));
        }
    }
    Ok(hinstance)
}

pub(super) fn pump_messages(events: &WindowsEventQueue) {
    for _ in 0..MAX_MESSAGES_PER_POLL {
        let mut message = MSG::default();
        // SAFETY: message points to initialized writable storage. This host
        // owns the current thread's Win32 message loop.
        if unsafe { PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) } == 0 {
            return;
        }
        if message.message == WM_QUIT {
            events.fail("Windows host received WM_QUIT on its owning thread");
            return;
        }
        // SAFETY: the message was returned by PeekMessageW for this thread.
        unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

pub(super) fn system_scale_factor() -> GuiResult<f64> {
    let mut dpi_guard = ThreadDpiGuard::enter()?;
    // SAFETY: GetDpiForSystem has no pointer arguments and is called while the
    // thread uses the same per-monitor-v2 context as window creation.
    let dpi = unsafe { GetDpiForSystem() };
    dpi_guard.restore()?;
    Ok(f64::from(dpi.max(BASE_DPI as u32)) / BASE_DPI)
}

pub(super) struct NativeWindow {
    hwnd: HWND,
    surface: Arc<WindowsSurfaceState>,
    context: ManuallyDrop<Box<WindowContext>>,
    spec: PlatformWindowSpec,
}

impl NativeWindow {
    pub(super) fn create(
        hinstance: HINSTANCE,
        spec: &PlatformWindowSpec,
        events: WindowsEventQueue,
    ) -> GuiResult<Self> {
        let title = wide(&spec.title)?;
        let class_name = wide(WINDOW_CLASS_NAME)?;
        let mut dpi_guard = ThreadDpiGuard::enter()?;
        // SAFETY: GetDpiForSystem has no pointer arguments and is called while
        // the thread uses the desired DPI context.
        let system_dpi = unsafe { GetDpiForSystem() };
        let dpi = if system_dpi == 0 {
            BASE_DPI as u32
        } else {
            system_dpi
        };
        let style = window_style(spec.resizable);
        let (outer_width, outer_height) = outer_size(spec.logical_size, dpi, style)?;
        let mut context = Box::new(WindowContext {
            id: spec.id,
            min_size: spec.min_size,
            max_size: spec.max_size,
            dpi,
            occluded: false,
            events,
        });
        let context_pointer = context.as_mut() as *mut WindowContext;
        // SAFETY: class and title are NUL-terminated for the duration of the
        // call, the class is registered for hinstance, and context_pointer
        // points to stable Box storage retained for the HWND lifetime.
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                title.as_ptr(),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                outer_width,
                outer_height,
                null_mut(),
                null_mut(),
                hinstance,
                context_pointer.cast::<c_void>(),
            )
        };
        let create_error = hwnd.is_null().then(|| {
            // SAFETY: read immediately after CreateWindowExW failed.
            unsafe { GetLastError() }
        });
        let restore_result = dpi_guard.restore();
        if let Some(error) = create_error {
            return Err(error_code("CreateWindowExW", error));
        }
        if let Err(error) = restore_result {
            // SAFETY: hwnd was just created successfully on this thread.
            if unsafe { DestroyWindow(hwnd) } == 0 {
                let destroy_error = last_error("DestroyWindow after DPI restore failure");
                if !detach_window_context(hwnd) {
                    // The HWND still owns context_pointer. Preserve that
                    // allocation rather than permit a future callback to
                    // dereference freed storage.
                    let _ = Box::leak(context);
                }
                return Err(GuiError::host(format!("{error}; {destroy_error}")));
            }
            return Err(error);
        }
        // SAFETY: hwnd is a live window created above.
        let window_dpi = unsafe { GetDpiForWindow(hwnd) };
        if window_dpi != 0 {
            context.dpi = window_dpi;
        }
        let mut native_spec = spec.clone();
        native_spec.visible = false;
        let surface = Arc::new(WindowsSurfaceState::new(
            NonZeroIsize::new(hwnd as isize)
                .ok_or_else(|| GuiError::host("Windows host created a null surface handle"))?,
            NonZeroIsize::new(hinstance as isize)
                .ok_or_else(|| GuiError::host("Windows host module handle is unavailable"))?,
        ));
        Ok(Self {
            hwnd,
            surface,
            context: ManuallyDrop::new(context),
            spec: native_spec,
        })
    }

    pub(super) fn apply_spec_properties(&mut self, desired: &PlatformWindowSpec) -> GuiResult<()> {
        let original = self.spec.clone();
        if let Err(error) = self.force_spec_properties(desired) {
            return match self.force_spec_properties(&original) {
                Ok(()) => Err(error),
                Err(restore) => Err(GuiError::host(format!(
                    "{error}; Windows host could not restore the native window: {restore}"
                ))),
            };
        }
        Ok(())
    }

    pub(super) fn restore_spec(&mut self, spec: &PlatformWindowSpec) -> GuiResult<()> {
        self.force_spec_properties(spec)?;
        self.set_visible(spec.visible);
        Ok(())
    }

    fn force_spec_properties(&mut self, desired: &PlatformWindowSpec) -> GuiResult<()> {
        let title = wide(&desired.title)?;
        // SAFETY: hwnd is live and title is NUL-terminated.
        if unsafe { SetWindowTextW(self.hwnd, title.as_ptr()) } == 0 {
            return Err(last_error("SetWindowTextW"));
        }
        let style = window_style(desired.resizable);
        // SAFETY: hwnd is live and GWL_STYLE accepts a WINDOW_STYLE value.
        unsafe {
            SetLastError(0);
        }
        // SAFETY: hwnd is live and owned by the current thread.
        let previous = unsafe { SetWindowLongPtrW(self.hwnd, GWL_STYLE, style as _) };
        // SAFETY: read immediately after SetWindowLongPtrW.
        let style_error = unsafe { GetLastError() };
        if previous == 0 && style_error != 0 {
            return Err(error_code("SetWindowLongPtrW", style_error));
        }
        self.context.min_size = desired.min_size;
        self.context.max_size = desired.max_size;
        // SAFETY: hwnd is live.
        let window_dpi = unsafe { GetDpiForWindow(self.hwnd) };
        if window_dpi != 0 {
            self.context.dpi = window_dpi;
        }
        let (width, height) = outer_size(desired.logical_size, self.context.dpi, style)?;
        // SAFETY: hwnd is live; SWP_NOZORDER makes the null insert-after
        // handle valid and the requested dimensions were range checked.
        if unsafe {
            SetWindowPos(
                self.hwnd,
                null_mut(),
                0,
                0,
                width,
                height,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
            )
        } == 0
        {
            return Err(last_error("SetWindowPos"));
        }
        let visible = self.spec.visible;
        self.spec = desired.clone();
        self.spec.visible = visible;
        Ok(())
    }

    pub(super) fn set_visible(&mut self, visible: bool) {
        // SAFETY: hwnd is live and ShowWindow has no failure return contract.
        unsafe {
            ShowWindow(self.hwnd, if visible { SW_SHOW } else { SW_HIDE });
        }
        self.spec.visible = visible;
    }

    pub(super) fn invalidate(&self) -> GuiResult<()> {
        // SAFETY: hwnd is live; a null RECT invalidates the complete client
        // area and erase=false preserves the self-drawn surface.
        if unsafe { InvalidateRect(self.hwnd, null(), 0) } == 0 {
            return Err(last_error("InvalidateRect"));
        }
        Ok(())
    }

    pub(super) fn destroy(&mut self) -> GuiResult<()> {
        if self.hwnd.is_null() {
            return Ok(());
        }
        if WindowsSurfaceState::has_external_lease(&self.surface) {
            return Err(GuiError::host(format!(
                "Windows host window {} still has an active Graphics surface lease",
                self.context.id.get()
            )));
        }
        // SAFETY: hwnd is live and belongs to the current thread.
        if unsafe { DestroyWindow(self.hwnd) } == 0 {
            return Err(last_error("DestroyWindow"));
        }
        self.surface.mark_destroyed();
        self.hwnd = null_mut();
        Ok(())
    }

    pub(super) fn surface_state(&self) -> Arc<WindowsSurfaceState> {
        Arc::clone(&self.surface)
    }

    pub(super) fn physical_size(&self) -> GuiResult<(u32, u32)> {
        client_physical_size(self.hwnd)
    }

    pub(super) fn scale_factor(&self) -> GuiResult<f64> {
        // SAFETY: hwnd is live.
        let dpi = unsafe { GetDpiForWindow(self.hwnd) };
        if dpi == 0 {
            return Err(last_error("GetDpiForWindow"));
        }
        Ok(f64::from(dpi) / BASE_DPI)
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let can_drop_context = if self.hwnd.is_null() {
            true
        } else if WindowsSurfaceState::has_external_lease(&self.surface) {
            // An out-of-order owner drop cannot invalidate a live Graphics
            // lease. Detach callbacks and intentionally leave the HWND for
            // process teardown instead of creating a dangling raw handle.
            detach_window_context(self.hwnd)
        } else {
            // SAFETY: NativeWindow is thread-affine through its Rc event queue
            // and still owns this HWND.
            if unsafe { DestroyWindow(self.hwnd) } != 0 {
                self.surface.mark_destroyed();
                true
            } else {
                // If destruction fails, detach before freeing the context. If
                // detachment also fails, intentionally leak the Box so a later
                // native callback can never dereference freed storage.
                detach_window_context(self.hwnd)
            }
        };
        self.hwnd = null_mut();
        if can_drop_context {
            // SAFETY: the HWND is gone or can no longer reach this context,
            // and ManuallyDrop ensures this is the allocation's only drop.
            unsafe {
                ManuallyDrop::drop(&mut self.context);
            }
        }
    }
}

struct WindowContext {
    id: PlatformWindowId,
    min_size: Option<Size>,
    max_size: Option<Size>,
    dpi: u32,
    occluded: bool,
    events: WindowsEventQueue,
}

impl WindowContext {
    fn window_event(&self, event: PlatformWindowEvent) {
        self.events.push(PlatformHostEvent::Window { event });
    }

    fn resized(&self, hwnd: HWND) {
        match client_logical_size(hwnd, self.dpi) {
            Ok(logical_size) => self.window_event(PlatformWindowEvent::Resized {
                window: self.id,
                logical_size,
            }),
            Err(error) => self.events.fail(error.to_string()),
        }
    }

    fn apply_minmax(&self, hwnd: HWND, info: *mut MINMAXINFO) {
        if info.is_null() {
            return;
        }
        // SAFETY: hwnd is live and GWL_STYLE is readable for this window.
        let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32;
        if let Some(size) = self.min_size {
            if let Some((width, height)) = outer_size_relaxed(size, self.dpi, style) {
                // SAFETY: lParam for WM_GETMINMAXINFO points to a writable
                // MINMAXINFO for the duration of the callback.
                unsafe {
                    (*info).ptMinTrackSize.x = width;
                    (*info).ptMinTrackSize.y = height;
                }
            }
        }
        if let Some(size) = self.max_size {
            if let Some((width, height)) = outer_size_relaxed(size, self.dpi, style) {
                // SAFETY: same WM_GETMINMAXINFO contract as above.
                unsafe {
                    (*info).ptMaxTrackSize.x = width;
                    (*info).ptMaxTrackSize.y = height;
                }
            }
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: Windows invokes this callback with message-specific pointer
        // contracts; window_proc_inner checks each pointer before use.
        unsafe { window_proc_inner(hwnd, message, wparam, lparam) }
    }))
    .unwrap_or_else(|_| {
        // SAFETY: fallback delegates the original message to Windows.
        unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
    })
}

unsafe fn window_proc_inner(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if message == WM_NCCREATE {
        let create = lparam as *const CREATESTRUCTW;
        if create.is_null() {
            return 0;
        }
        // SAFETY: WM_NCCREATE supplies a valid CREATESTRUCTW and lpCreateParams
        // is the stable WindowContext pointer passed to CreateWindowExW.
        let context = unsafe { (*create).lpCreateParams as *mut WindowContext };
        if context.is_null() {
            return 0;
        }
        // SAFETY: the pointer fits LONG_PTR and remains valid until
        // NativeWindow destroys this HWND.
        unsafe {
            SetLastError(0);
        }
        // SAFETY: hwnd is being initialized and context is stable Box storage.
        let previous = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, context as _) };
        // SAFETY: read immediately after SetWindowLongPtrW.
        let error = unsafe { GetLastError() };
        if previous == 0 && error != 0 {
            return 0;
        }
        // SAFETY: default WM_NCCREATE processing initializes the standard
        // top-level window state, including its title and non-client area.
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // SAFETY: GWLP_USERDATA is either zero or the WindowContext installed at
    // WM_NCCREATE.
    let context = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowContext;
    if message == WM_NCDESTROY {
        // SAFETY: DefWindowProcW receives the original message parameters.
        let result = unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
        // SAFETY: clearing user data prevents callbacks from observing a stale
        // pointer after destruction.
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        }
        return result;
    }
    if context.is_null() {
        // SAFETY: no A3S context is installed, so default handling is required.
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }
    // SAFETY: the Box owning this context outlives the HWND callback.
    let context = unsafe { &mut *context };

    match message {
        WM_CLOSE => {
            context.window_event(PlatformWindowEvent::CloseRequested { window: context.id });
            0
        }
        WM_DESTROY => {
            context.window_event(PlatformWindowEvent::Closed { window: context.id });
            0
        }
        WM_SETFOCUS => {
            context.window_event(PlatformWindowEvent::FocusChanged {
                window: context.id,
                focused: true,
            });
            0
        }
        WM_KILLFOCUS => {
            context.window_event(PlatformWindowEvent::FocusChanged {
                window: context.id,
                focused: false,
            });
            0
        }
        WM_SIZE => {
            let minimized = wparam as u32 == SIZE_MINIMIZED;
            if minimized != context.occluded {
                context.occluded = minimized;
                context.window_event(PlatformWindowEvent::OcclusionChanged {
                    window: context.id,
                    occluded: minimized,
                });
            }
            if !minimized {
                context.resized(hwnd);
            }
            0
        }
        WM_DPICHANGED => {
            let dpi = (wparam as u32 & 0xffff).max(1);
            context.dpi = dpi;
            context.window_event(PlatformWindowEvent::ScaleChanged {
                window: context.id,
                scale_factor: f64::from(dpi) / BASE_DPI,
            });
            let suggested = lparam as *const RECT;
            if !suggested.is_null() {
                // SAFETY: WM_DPICHANGED supplies a valid suggested RECT for
                // this callback.
                let suggested = unsafe { *suggested };
                // SAFETY: hwnd is live and the suggested geometry comes from
                // Windows for this DPI transition.
                if unsafe {
                    SetWindowPos(
                        hwnd,
                        null_mut(),
                        suggested.left,
                        suggested.top,
                        suggested.right - suggested.left,
                        suggested.bottom - suggested.top,
                        SWP_NOZORDER | SWP_NOACTIVATE,
                    )
                } == 0
                {
                    context.events.fail(last_error("SetWindowPos").to_string());
                }
            }
            0
        }
        WM_GETMINMAXINFO => {
            context.apply_minmax(hwnd, lparam as *mut MINMAXINFO);
            0
        }
        WM_PAINT => {
            let mut paint = PAINTSTRUCT::default();
            // SAFETY: paint is writable for BeginPaint and then passed back to
            // EndPaint for the same live HWND.
            unsafe {
                BeginPaint(hwnd, &mut paint);
                EndPaint(hwnd, &paint);
            }
            context.window_event(PlatformWindowEvent::RedrawRequested { window: context.id });
            0
        }
        WM_ERASEBKGND => 1,
        _ => {
            // SAFETY: unhandled messages retain Windows default semantics.
            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
    }
}

fn window_style(resizable: bool) -> u32 {
    if resizable {
        WS_OVERLAPPEDWINDOW
    } else {
        WS_OVERLAPPEDWINDOW & !(WS_THICKFRAME | WS_MAXIMIZEBOX)
    }
}

fn outer_size(size: Size, dpi: u32, style: u32) -> GuiResult<(i32, i32)> {
    let width = logical_pixels(size.width, dpi, "window width")?;
    let height = logical_pixels(size.height, dpi, "window height")?;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: width,
        bottom: height,
    };
    // SAFETY: rect is writable and the style/DPI values are validated.
    if unsafe { AdjustWindowRectExForDpi(&mut rect, style, 0, 0, dpi) } == 0 {
        return Err(last_error("AdjustWindowRectExForDpi"));
    }
    let outer_width = rect
        .right
        .checked_sub(rect.left)
        .ok_or_else(|| GuiError::host("Windows host outer width overflowed"))?;
    let outer_height = rect
        .bottom
        .checked_sub(rect.top)
        .ok_or_else(|| GuiError::host("Windows host outer height overflowed"))?;
    if outer_width <= 0 || outer_height <= 0 {
        return Err(GuiError::host(
            "Windows host outer window size must be greater than zero",
        ));
    }
    Ok((outer_width, outer_height))
}

fn outer_size_relaxed(size: Size, dpi: u32, style: u32) -> Option<(i32, i32)> {
    outer_size(size, dpi, style).ok()
}

fn logical_pixels(value: f64, dpi: u32, field: &str) -> GuiResult<i32> {
    let pixels = (value * f64::from(dpi) / BASE_DPI).round();
    if !pixels.is_finite() || pixels < 1.0 || pixels > f64::from(i32::MAX) {
        return Err(GuiError::host(format!(
            "Windows host {field} does not fit a positive Win32 coordinate"
        )));
    }
    Ok(pixels as i32)
}

fn client_physical_size(hwnd: HWND) -> GuiResult<(u32, u32)> {
    let mut rect = RECT::default();
    // SAFETY: hwnd is live and rect is writable.
    if unsafe { GetClientRect(hwnd, &mut rect) } == 0 {
        return Err(last_error("GetClientRect"));
    }
    let width = rect.right.saturating_sub(rect.left).max(0) as u32;
    let height = rect.bottom.saturating_sub(rect.top).max(0) as u32;
    Ok((width, height))
}

fn client_logical_size(hwnd: HWND, dpi: u32) -> GuiResult<Size> {
    let (width, height) = client_physical_size(hwnd)?;
    let scale = f64::from(dpi.max(1)) / BASE_DPI;
    Ok(Size::new(
        f64::from(width) / scale,
        f64::from(height) / scale,
    ))
}

fn wide(value: &str) -> GuiResult<Vec<u16>> {
    if value.contains('\0') {
        return Err(GuiError::host(
            "Windows host strings cannot contain NUL characters",
        ));
    }
    Ok(value.encode_utf16().chain(std::iter::once(0)).collect())
}

fn last_error(operation: &str) -> GuiError {
    // SAFETY: GetLastError has no preconditions and callers invoke this
    // immediately after a failed Win32 call.
    error_code(operation, unsafe { GetLastError() })
}

fn error_code(operation: &str, code: u32) -> GuiError {
    GuiError::host(format!(
        "Windows {operation} failed: {}",
        std::io::Error::from_raw_os_error(code as i32)
    ))
}

fn detach_window_context(hwnd: HWND) -> bool {
    // SAFETY: hwnd is owned by NativeWindow and GWLP_USERDATA stores only its
    // WindowContext pointer. Clearing it is the fail-safe before freeing that
    // context when DestroyWindow has failed.
    unsafe {
        SetLastError(0);
    }
    // SAFETY: same owned HWND/GWLP_USERDATA contract as above.
    let previous = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
    // SAFETY: read immediately after SetWindowLongPtrW.
    previous != 0 || unsafe { GetLastError() } == 0
}

struct ThreadDpiGuard {
    previous: DPI_AWARENESS_CONTEXT,
    active: bool,
}

impl ThreadDpiGuard {
    fn enter() -> GuiResult<Self> {
        // SAFETY: the constant is a documented DPI awareness pseudo-handle.
        let previous =
            unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        if previous.is_null() {
            return Err(last_error("SetThreadDpiAwarenessContext"));
        }
        Ok(Self {
            previous,
            active: true,
        })
    }

    fn restore(&mut self) -> GuiResult<()> {
        if !self.active {
            return Ok(());
        }
        // SAFETY: previous was returned by SetThreadDpiAwarenessContext on the
        // same thread and remains a valid pseudo-handle.
        if unsafe { SetThreadDpiAwarenessContext(self.previous) }.is_null() {
            return Err(last_error("SetThreadDpiAwarenessContext restore"));
        }
        self.active = false;
        Ok(())
    }
}

impl Drop for ThreadDpiGuard {
    fn drop(&mut self) {
        if self.active {
            // SAFETY: best-effort restoration on the same thread.
            unsafe {
                SetThreadDpiAwarenessContext(self.previous);
            }
            self.active = false;
        }
    }
}
