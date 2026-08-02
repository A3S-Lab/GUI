#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::{GetLastError, HWND};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::Pointer::{
    GetPointerPenInfo, GetPointerTouchInfo, GetPointerType, POINTER_FLAG_CANCELED,
    POINTER_FLAG_INCONTACT, POINTER_PEN_INFO, POINTER_TOUCH_INFO,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    PEN_MASK_PRESSURE, PT_PEN, PT_POINTER, PT_TOUCH, TOUCH_MASK_PRESSURE,
};

use crate::input::NativeInputModality;

use super::super::mouse::pointer_modifiers;
use super::{
    changed_button, logical_client_position, normalize_pressure, pointer_buttons,
    WindowsPointerSample, PRIMARY_BUTTON_MASK,
};

pub(super) fn read_pointer_sample(
    hwnd: HWND,
    dpi: u32,
    native_id: u32,
) -> Result<Option<WindowsPointerSample>, String> {
    let mut pointer_type = PT_POINTER;
    // SAFETY: pointer_type is writable and native_id came from the current
    // pointer message.
    if unsafe { GetPointerType(native_id, &mut pointer_type) } == 0 {
        return Err(windows_error("GetPointerType"));
    }
    match pointer_type {
        PT_TOUCH => {
            let mut touch = POINTER_TOUCH_INFO::default();
            // SAFETY: touch is writable and native_id belongs to the current
            // message on this window thread.
            if unsafe { GetPointerTouchInfo(native_id, &mut touch) } == 0 {
                return Err(windows_error("GetPointerTouchInfo"));
            }
            let pressure = normalize_pressure(touch.touchMask, TOUCH_MASK_PRESSURE, touch.pressure);
            pointer_sample(
                hwnd,
                dpi,
                NativeInputModality::Touch,
                touch.pointerInfo,
                pressure,
            )
            .map(Some)
        }
        PT_PEN => {
            let mut pen = POINTER_PEN_INFO::default();
            // SAFETY: pen is writable and native_id belongs to the current
            // message on this window thread.
            if unsafe { GetPointerPenInfo(native_id, &mut pen) } == 0 {
                return Err(windows_error("GetPointerPenInfo"));
            }
            let pressure = normalize_pressure(pen.penMask, PEN_MASK_PRESSURE, pen.pressure);
            pointer_sample(
                hwnd,
                dpi,
                NativeInputModality::Pen,
                pen.pointerInfo,
                pressure,
            )
            .map(Some)
        }
        _ => Ok(None),
    }
}

fn pointer_sample(
    hwnd: HWND,
    dpi: u32,
    modality: NativeInputModality,
    info: windows_sys::Win32::UI::Input::Pointer::POINTER_INFO,
    pressure: Option<f64>,
) -> Result<WindowsPointerSample, String> {
    let mut point = info.ptPixelLocation;
    // SAFETY: hwnd owns the current pointer message and point is writable.
    if unsafe { ScreenToClient(hwnd, &mut point) } == 0 {
        return Err(windows_error("ScreenToClient"));
    }
    let mut pressed_buttons = pointer_buttons(info.pointerFlags);
    if modality == NativeInputModality::Touch
        && info.pointerFlags & POINTER_FLAG_INCONTACT != 0
        && pressed_buttons == 0
    {
        pressed_buttons = PRIMARY_BUTTON_MASK;
    }
    Ok(WindowsPointerSample {
        native_id: info.pointerId,
        source_device: info.sourceDevice as usize,
        modality,
        position: logical_client_position(point, dpi),
        pressed_buttons,
        changed_button: changed_button(info.ButtonChangeType),
        pressure,
        modifiers: pointer_modifiers(info.dwKeyStates as u16),
        cancelled: info.pointerFlags & POINTER_FLAG_CANCELED != 0,
    })
}

fn windows_error(operation: &str) -> String {
    // SAFETY: GetLastError has no preconditions and is read immediately after
    // the failed Win32 call.
    let code = unsafe { GetLastError() };
    format!(
        "Windows {operation} failed: {}",
        std::io::Error::from_raw_os_error(code as i32)
    )
}
