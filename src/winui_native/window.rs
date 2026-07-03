use windows::Win32::Foundation::{
    GetLastError, SetLastError, ERROR_SUCCESS, HWND, LPARAM, LRESULT, RECT, WPARAM,
};
use windows::Win32::UI::Shell::{
    DefSubclassProc, GetWindowSubclass, RemoveWindowSubclass, SetWindowSubclass,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, GetWindowRect, IsWindow, SetWindowLongPtrW, SetWindowPos, GWL_STYLE,
    MINMAXINFO, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WM_GETMINMAXINFO,
    WM_NCDESTROY, WS_MAXIMIZEBOX, WS_THICKFRAME,
};
use windows_core::Interface;
use winui3::Microsoft::UI::Xaml as xaml;

use crate::error::{GuiError, GuiResult};
use crate::style::{NativeSizeConstraints, PortableStyle};

use super::helpers::map_winui;

const WINUI_RESIZE_BOUNDS_SUBCLASS_ID: usize = 0xA35;

fn winui_window_hwnd(window: &xaml::Window) -> GuiResult<HWND> {
    let native: winui3::IWindowNative = map_winui(
        "failed to read WinUI window native interface",
        window.cast(),
    )?;
    map_winui("failed to read WinUI window handle", unsafe {
        native.WindowHandle()
    })
}

pub(super) fn winui_window_is_open(window: &xaml::Window) -> GuiResult<bool> {
    let hwnd = winui_window_hwnd(window)?;
    Ok(!hwnd.is_invalid() && unsafe { IsWindow(Some(hwnd)).as_bool() })
}

pub(super) fn apply_winui_window_portable_style(
    window: &xaml::Window,
    style: &PortableStyle,
) -> GuiResult<()> {
    let size = style.native_size_constraints();
    let hwnd = winui_window_hwnd(window)?;
    apply_winui_window_resize_bounds(hwnd, WinUiWindowResizeBounds::from_size(size))?;

    if size.width.is_none() && size.height.is_none() {
        return Ok(());
    }

    let current = if size.width.is_none() || size.height.is_none() {
        Some(winui_window_rect_size(hwnd)?)
    } else {
        None
    };
    let bounds = WinUiWindowResizeBounds::from_size(size);
    let width = bounds.clamp_width(winui_window_dimension(
        size.width,
        current.map(|size| size.width),
    ));
    let height = bounds.clamp_height(winui_window_dimension(
        size.height,
        current.map(|size| size.height),
    ));

    map_winui("failed to resize WinUI window", unsafe {
        SetWindowPos(hwnd, None, 0, 0, width, height, SWP_NOMOVE | SWP_NOZORDER)
    })
}

fn apply_winui_window_resize_bounds(hwnd: HWND, bounds: WinUiWindowResizeBounds) -> GuiResult<()> {
    let (installed, ref_data) = winui_window_resize_bounds_subclass(hwnd);
    if bounds.is_empty() {
        if installed {
            remove_winui_window_resize_bounds_subclass(hwnd, ref_data)?;
        }
        return Ok(());
    }

    if installed && ref_data != 0 {
        unsafe {
            *(ref_data as *mut WinUiWindowResizeBounds) = bounds;
        }
        return Ok(());
    }

    let bounds_ptr = Box::into_raw(Box::new(bounds)) as usize;
    unsafe {
        SetLastError(ERROR_SUCCESS);
        if !SetWindowSubclass(
            hwnd,
            Some(winui_window_resize_bounds_proc),
            WINUI_RESIZE_BOUNDS_SUBCLASS_ID,
            bounds_ptr,
        )
        .as_bool()
        {
            let error = GetLastError();
            drop(Box::from_raw(bounds_ptr as *mut WinUiWindowResizeBounds));
            return Err(GuiError::host(format!(
                "failed to install WinUI window resize bounds: Win32 error {}",
                error.0
            )));
        }
    }
    Ok(())
}

fn winui_window_resize_bounds_subclass(hwnd: HWND) -> (bool, usize) {
    let mut ref_data = 0usize;
    let installed = unsafe {
        GetWindowSubclass(
            hwnd,
            Some(winui_window_resize_bounds_proc),
            WINUI_RESIZE_BOUNDS_SUBCLASS_ID,
            Some(&mut ref_data),
        )
        .as_bool()
    };
    (installed, ref_data)
}

fn remove_winui_window_resize_bounds_subclass(hwnd: HWND, ref_data: usize) -> GuiResult<()> {
    unsafe {
        SetLastError(ERROR_SUCCESS);
        if !RemoveWindowSubclass(
            hwnd,
            Some(winui_window_resize_bounds_proc),
            WINUI_RESIZE_BOUNDS_SUBCLASS_ID,
        )
        .as_bool()
        {
            let error = GetLastError();
            return Err(GuiError::host(format!(
                "failed to remove WinUI window resize bounds: Win32 error {}",
                error.0
            )));
        }
        if ref_data != 0 {
            drop(Box::from_raw(ref_data as *mut WinUiWindowResizeBounds));
        }
    }
    Ok(())
}

unsafe extern "system" fn winui_window_resize_bounds_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    subclass_id: usize,
    ref_data: usize,
) -> LRESULT {
    if message == WM_GETMINMAXINFO {
        if ref_data != 0 {
            if let Some(info) = (lparam.0 as *mut MINMAXINFO).as_mut() {
                (*(ref_data as *const WinUiWindowResizeBounds)).apply_to_minmax_info(info);
                return LRESULT(0);
            }
        }
    }

    if message == WM_NCDESTROY {
        let _ = RemoveWindowSubclass(hwnd, Some(winui_window_resize_bounds_proc), subclass_id);
        if ref_data != 0 {
            drop(Box::from_raw(ref_data as *mut WinUiWindowResizeBounds));
        }
    }

    DefSubclassProc(hwnd, message, wparam, lparam)
}

fn winui_window_rect_size(hwnd: HWND) -> GuiResult<WinUiWindowSize> {
    let mut rect = RECT::default();
    map_winui("failed to read WinUI window rectangle", unsafe {
        GetWindowRect(hwnd, &mut rect)
    })?;
    Ok(winui_rect_size(rect))
}

fn winui_rect_size(rect: RECT) -> WinUiWindowSize {
    WinUiWindowSize {
        width: rect.right.saturating_sub(rect.left).max(1),
        height: rect.bottom.saturating_sub(rect.top).max(1),
    }
}

fn winui_window_dimension(value: Option<f64>, current: Option<i32>) -> i32 {
    value
        .map(winui_window_points_to_i32)
        .or(current)
        .unwrap_or(1)
        .max(1)
}

fn winui_window_points_to_i32(value: f64) -> i32 {
    if !value.is_finite() || value <= 0.0 {
        return 1;
    }
    if value >= i32::MAX as f64 {
        return i32::MAX;
    }
    value.round().max(1.0) as i32
}

pub(super) fn set_winui_window_resizable(
    window: &xaml::Window,
    value: Option<bool>,
) -> GuiResult<()> {
    let hwnd = winui_window_hwnd(window)?;
    let style =
        winui_resizable_window_style(unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32, value);

    unsafe {
        SetLastError(ERROR_SUCCESS);
        let previous = SetWindowLongPtrW(hwnd, GWL_STYLE, style as isize);
        if previous == 0 {
            let error = GetLastError();
            if error != ERROR_SUCCESS {
                return Err(GuiError::host(format!(
                    "failed to set WinUI window resizable style: Win32 error {}",
                    error.0
                )));
            }
        }
    }

    map_winui("failed to refresh WinUI window resizable style", unsafe {
        SetWindowPos(
            hwnd,
            None,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
        )
    })
}

fn winui_resizable_window_style(style: u32, value: Option<bool>) -> u32 {
    let resize_style = WS_THICKFRAME.0 | WS_MAXIMIZEBOX.0;
    if value.unwrap_or(true) {
        style | resize_style
    } else {
        style & !resize_style
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WinUiWindowSize {
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct WinUiWindowResizeBounds {
    min_width: Option<i32>,
    min_height: Option<i32>,
    max_width: Option<i32>,
    max_height: Option<i32>,
}

impl WinUiWindowResizeBounds {
    fn from_size(size: NativeSizeConstraints) -> Self {
        Self {
            min_width: size.min_width.map(winui_window_points_to_i32),
            min_height: size.min_height.map(winui_window_points_to_i32),
            max_width: size.max_width.map(winui_window_points_to_i32),
            max_height: size.max_height.map(winui_window_points_to_i32),
        }
    }

    fn is_empty(self) -> bool {
        self.min_width.is_none()
            && self.min_height.is_none()
            && self.max_width.is_none()
            && self.max_height.is_none()
    }

    fn clamp_width(self, width: i32) -> i32 {
        clamp_winui_window_axis(width, self.min_width, self.max_width)
    }

    fn clamp_height(self, height: i32) -> i32 {
        clamp_winui_window_axis(height, self.min_height, self.max_height)
    }

    fn apply_to_minmax_info(self, info: &mut MINMAXINFO) {
        apply_winui_window_track_axis(
            self.min_width,
            self.max_width,
            &mut info.ptMinTrackSize.x,
            &mut info.ptMaxTrackSize.x,
        );
        apply_winui_window_track_axis(
            self.min_height,
            self.max_height,
            &mut info.ptMinTrackSize.y,
            &mut info.ptMaxTrackSize.y,
        );
    }
}

fn clamp_winui_window_axis(value: i32, min: Option<i32>, max: Option<i32>) -> i32 {
    let mut value = value.max(1);
    if let Some(min) = min {
        value = value.max(min);
    }
    if let Some(max) = max {
        value = value.min(max.max(min.unwrap_or(1)));
    }
    value.max(1)
}

fn apply_winui_window_track_axis(
    min: Option<i32>,
    max: Option<i32>,
    min_track: &mut i32,
    max_track: &mut i32,
) {
    match (min, max) {
        (Some(min), Some(max)) => {
            *min_track = min;
            *max_track = max.max(min);
        }
        (Some(min), None) => {
            *min_track = min;
            if *max_track < min {
                *max_track = min;
            }
        }
        (None, Some(max)) => {
            *max_track = max;
            if *min_track > max {
                *min_track = max;
            }
        }
        (None, None) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winui_resizable_window_style_toggles_resize_bits() {
        let fixed = winui_resizable_window_style(WS_THICKFRAME.0 | WS_MAXIMIZEBOX.0, Some(false));
        assert_eq!(fixed & WS_THICKFRAME.0, 0);
        assert_eq!(fixed & WS_MAXIMIZEBOX.0, 0);

        let resizable = winui_resizable_window_style(0, Some(true));
        assert_ne!(resizable & WS_THICKFRAME.0, 0);
        assert_ne!(resizable & WS_MAXIMIZEBOX.0, 0);

        let defaulted = winui_resizable_window_style(0, None);
        assert_ne!(defaulted & WS_THICKFRAME.0, 0);
        assert_ne!(defaulted & WS_MAXIMIZEBOX.0, 0);
    }

    #[test]
    fn winui_window_points_to_i32_rounds_and_clamps() {
        assert_eq!(winui_window_points_to_i32(0.1), 1);
        assert_eq!(winui_window_points_to_i32(10.4), 10);
        assert_eq!(winui_window_points_to_i32(10.5), 11);
        assert_eq!(winui_window_points_to_i32(f64::NAN), 1);
        assert_eq!(winui_window_points_to_i32(f64::INFINITY), 1);
        assert_eq!(winui_window_points_to_i32(i32::MAX as f64 * 2.0), i32::MAX);
    }

    #[test]
    fn winui_window_dimension_keeps_current_when_missing() {
        assert_eq!(winui_window_dimension(None, Some(640)), 640);
        assert_eq!(winui_window_dimension(None, Some(0)), 1);
        assert_eq!(winui_window_dimension(None, None), 1);
        assert_eq!(winui_window_dimension(Some(320.0), Some(640)), 320);
    }

    #[test]
    fn winui_rect_size_clamps_empty_or_inverted_rects() {
        assert_eq!(
            winui_rect_size(RECT {
                left: 10,
                top: 20,
                right: 110,
                bottom: 220,
            }),
            WinUiWindowSize {
                width: 100,
                height: 200,
            }
        );
        assert_eq!(
            winui_rect_size(RECT {
                left: 10,
                top: 20,
                right: 10,
                bottom: 10,
            }),
            WinUiWindowSize {
                width: 1,
                height: 1,
            }
        );
    }

    #[test]
    fn winui_window_resize_bounds_from_size_maps_min_max_constraints() {
        assert_eq!(
            WinUiWindowResizeBounds::from_size(NativeSizeConstraints {
                min_width: Some(320.4),
                min_height: Some(240.5),
                max_width: Some(960.0),
                max_height: Some(720.0),
                ..NativeSizeConstraints::default()
            }),
            WinUiWindowResizeBounds {
                min_width: Some(320),
                min_height: Some(241),
                max_width: Some(960),
                max_height: Some(720),
            }
        );
    }

    #[test]
    fn winui_window_resize_bounds_clamps_initial_window_size() {
        let bounds = WinUiWindowResizeBounds {
            min_width: Some(320),
            min_height: Some(240),
            max_width: Some(960),
            max_height: Some(720),
        };

        assert_eq!(bounds.clamp_width(200), 320);
        assert_eq!(bounds.clamp_width(1200), 960);
        assert_eq!(bounds.clamp_width(640), 640);
        assert_eq!(bounds.clamp_height(100), 240);
        assert_eq!(bounds.clamp_height(800), 720);
        assert_eq!(bounds.clamp_height(480), 480);
    }

    #[test]
    fn winui_window_resize_bounds_apply_to_minmax_info_updates_track_axes() {
        let bounds = WinUiWindowResizeBounds {
            min_width: Some(320),
            min_height: None,
            max_width: Some(960),
            max_height: Some(720),
        };
        let mut info = MINMAXINFO::default();
        info.ptMinTrackSize.x = 120;
        info.ptMinTrackSize.y = 180;
        info.ptMaxTrackSize.x = 1600;
        info.ptMaxTrackSize.y = 1400;

        bounds.apply_to_minmax_info(&mut info);

        assert_eq!(info.ptMinTrackSize.x, 320);
        assert_eq!(info.ptMaxTrackSize.x, 960);
        assert_eq!(info.ptMinTrackSize.y, 180);
        assert_eq!(info.ptMaxTrackSize.y, 720);
    }

    #[test]
    fn winui_window_resize_bounds_keep_track_axes_consistent() {
        let mut min_track = 500;
        let mut max_track = 400;
        apply_winui_window_track_axis(Some(640), None, &mut min_track, &mut max_track);
        assert_eq!(min_track, 640);
        assert_eq!(max_track, 640);

        let mut min_track = 500;
        let mut max_track = 400;
        apply_winui_window_track_axis(None, Some(320), &mut min_track, &mut max_track);
        assert_eq!(min_track, 320);
        assert_eq!(max_track, 320);

        let mut min_track = 100;
        let mut max_track = 400;
        apply_winui_window_track_axis(Some(640), Some(320), &mut min_track, &mut max_track);
        assert_eq!(min_track, 640);
        assert_eq!(max_track, 640);
    }
}
