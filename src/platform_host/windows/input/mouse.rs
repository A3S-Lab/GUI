#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::{GetLastError, HWND, LPARAM, POINT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, TrackMouseEvent, TME_LEAVE, TRACKMOUSEEVENT, VK_LWIN, VK_MENU, VK_RWIN,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetClientRect, IsWindowVisible, WHEEL_DELTA, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEWHEEL, WM_RBUTTONDBLCLK,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP, XBUTTON1,
    XBUTTON2,
};

use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::platform_host::{
    PlatformHostEvent, PlatformInputDeviceId, PlatformInputEvent, PlatformPoint,
    PlatformPointerButton, PlatformPointerEvent, PlatformPointerId, PlatformPointerPhase,
    PlatformWheelDeltaMode, PlatformWheelEvent, PlatformWindowId,
};

use super::super::events::WindowsEventQueue;
use super::{CaptureAction, WindowsInputMessage, WindowsInputState};

const BASE_DPI: f64 = 96.0;
pub(super) const WM_MOUSELEAVE: u32 = 0x02a3;
const MK_LBUTTON: u16 = 0x0001;
const MK_RBUTTON: u16 = 0x0002;
const MK_SHIFT: u16 = 0x0004;
const MK_CONTROL: u16 = 0x0008;
const MK_MBUTTON: u16 = 0x0010;
const MK_XBUTTON1: u16 = 0x0020;
const MK_XBUTTON2: u16 = 0x0040;
const PRIMARY_BUTTON_MASK: u32 = 1;
const SECONDARY_BUTTON_MASK: u32 = 2;
const AUXILIARY_BUTTON_MASK: u32 = 4;
const BACK_BUTTON_MASK: u32 = 8;
const FORWARD_BUTTON_MASK: u32 = 16;
const LEGACY_MOUSE_DEVICE: PlatformInputDeviceId = PlatformInputDeviceId::new(1);
const LEGACY_MOUSE_POINTER: PlatformPointerId = PlatformPointerId::new(1);

impl WindowsInputState {
    pub(super) fn mouse_moved(
        &mut self,
        hwnd: HWND,
        window: PlatformWindowId,
        dpi: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        let timestamp_micros = self.timestamp_micros();
        let position = client_position(lparam, dpi);
        let buttons = mouse_buttons(low_word(wparam));
        let modifiers = pointer_modifiers(low_word(wparam));
        self.mouse_position = position;
        self.mouse_buttons = buttons;
        self.set_modifiers(
            window,
            LEGACY_MOUSE_DEVICE,
            modifiers,
            timestamp_micros,
            events,
        );
        if !self.mouse_inside && point_is_in_client(hwnd, lparam) {
            self.track_mouse_leave(hwnd, events);
            self.mouse_inside = true;
            self.push_pointer(
                window,
                PlatformPointerPhase::Entered,
                position,
                None,
                buttons,
                modifiers,
                timestamp_micros,
                events,
            );
        }
        self.push_pointer(
            window,
            PlatformPointerPhase::Moved,
            position,
            None,
            buttons,
            modifiers,
            timestamp_micros,
            events,
        );
        WindowsInputMessage::handled(0)
    }

    pub(super) fn mouse_left(
        &mut self,
        window: PlatformWindowId,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        if self.mouse_inside {
            self.mouse_inside = false;
            self.push_pointer(
                window,
                PlatformPointerPhase::Left,
                self.mouse_position,
                None,
                self.mouse_buttons,
                self.modifiers,
                self.timestamp_micros(),
                events,
            );
        }
        WindowsInputMessage::handled(0)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn mouse_button(
        &mut self,
        hwnd: HWND,
        window: PlatformWindowId,
        dpi: u32,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        phase: PlatformPointerPhase,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        let timestamp_micros = self.timestamp_micros();
        let position = client_position(lparam, dpi);
        let previous_buttons = self.mouse_buttons;
        let buttons = mouse_buttons(low_word(wparam));
        let modifiers = pointer_modifiers(low_word(wparam));
        self.mouse_position = position;
        self.mouse_buttons = buttons;
        self.set_modifiers(
            window,
            LEGACY_MOUSE_DEVICE,
            modifiers,
            timestamp_micros,
            events,
        );
        if !self.mouse_inside && point_is_in_client(hwnd, lparam) {
            self.track_mouse_leave(hwnd, events);
            self.mouse_inside = true;
            self.push_pointer(
                window,
                PlatformPointerPhase::Entered,
                position,
                None,
                buttons,
                modifiers,
                timestamp_micros,
                events,
            );
        }
        self.push_pointer(
            window,
            phase,
            position,
            Some(mouse_button(message, high_word(wparam))),
            buttons,
            modifiers,
            timestamp_micros,
            events,
        );
        let is_xbutton = matches!(message, WM_XBUTTONDOWN | WM_XBUTTONDBLCLK | WM_XBUTTONUP);
        let mut handled = WindowsInputMessage::handled(if is_xbutton { 1 } else { 0 });
        if phase == PlatformPointerPhase::Pressed {
            // SAFETY: hwnd is the live top-level window handling this message.
            let is_visible = unsafe { IsWindowVisible(hwnd) } != 0;
            // Hidden staging windows can receive synthetic posted messages but
            // must never steal process-global focus or mouse capture.
            if is_visible {
                handled = handled.focus();
                if previous_buttons == 0 && buttons != 0 {
                    handled = handled.capture(CaptureAction::Acquire);
                }
            }
        } else if buttons == 0 {
            handled = handled.capture(CaptureAction::Release);
        }
        handled
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn mouse_wheel(
        &mut self,
        hwnd: HWND,
        window: PlatformWindowId,
        dpi: u32,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        let timestamp_micros = self.timestamp_micros();
        let modifiers = pointer_modifiers(low_word(wparam));
        self.set_modifiers(
            window,
            LEGACY_MOUSE_DEVICE,
            modifiers,
            timestamp_micros,
            events,
        );
        let Some(position) = wheel_position(hwnd, lparam, dpi) else {
            events.fail(windows_error("ScreenToClient"));
            return WindowsInputMessage::handled(0);
        };
        let delta = f64::from(high_word(wparam) as i16) / f64::from(WHEEL_DELTA);
        let delta = if message == WM_MOUSEWHEEL {
            PlatformPoint::new(0.0, -delta)
        } else {
            PlatformPoint::new(delta, 0.0)
        };
        events.push(PlatformHostEvent::Input {
            event: PlatformInputEvent::Wheel {
                event: PlatformWheelEvent {
                    window,
                    device: LEGACY_MOUSE_DEVICE,
                    position,
                    delta,
                    delta_mode: PlatformWheelDeltaMode::Lines,
                    modifiers,
                    timestamp_micros,
                },
            },
        });
        WindowsInputMessage::handled(0)
    }

    pub(super) fn cancel_pointer(&mut self, window: PlatformWindowId, events: &WindowsEventQueue) {
        if !self.mouse_inside && self.mouse_buttons == 0 {
            return;
        }
        self.mouse_inside = false;
        self.mouse_buttons = 0;
        self.push_pointer(
            window,
            PlatformPointerPhase::Cancelled,
            self.mouse_position,
            None,
            0,
            self.modifiers,
            self.timestamp_micros(),
            events,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn push_pointer(
        &self,
        window: PlatformWindowId,
        phase: PlatformPointerPhase,
        position: PlatformPoint,
        button: Option<PlatformPointerButton>,
        pressed_buttons: u32,
        modifiers: NativeKeyModifiers,
        timestamp_micros: u64,
        events: &WindowsEventQueue,
    ) {
        events.push(PlatformHostEvent::Input {
            event: PlatformInputEvent::Pointer {
                event: PlatformPointerEvent {
                    window,
                    device: LEGACY_MOUSE_DEVICE,
                    pointer: LEGACY_MOUSE_POINTER,
                    modality: NativeInputModality::Mouse,
                    phase,
                    position,
                    button,
                    pressed_buttons,
                    pressure: None,
                    modifiers,
                    timestamp_micros,
                },
            },
        });
    }

    fn track_mouse_leave(&self, hwnd: HWND, events: &WindowsEventQueue) {
        let mut tracker = TRACKMOUSEEVENT {
            cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
            dwFlags: TME_LEAVE,
            hwndTrack: hwnd,
            dwHoverTime: 0,
        };
        // SAFETY: tracker is writable and hwnd is the live window currently
        // handling the mouse message.
        if unsafe { TrackMouseEvent(&mut tracker) } == 0 {
            events.fail(windows_error("TrackMouseEvent"));
        }
    }
}

fn low_word(value: usize) -> u16 {
    value as u16
}

fn high_word(value: usize) -> u16 {
    (value >> 16) as u16
}

fn signed_low_word(value: isize) -> i16 {
    value as u16 as i16
}

fn signed_high_word(value: isize) -> i16 {
    (value >> 16) as u16 as i16
}

fn client_position(lparam: LPARAM, dpi: u32) -> PlatformPoint {
    let scale = f64::from(dpi.max(1)) / BASE_DPI;
    PlatformPoint::new(
        f64::from(signed_low_word(lparam)) / scale,
        f64::from(signed_high_word(lparam)) / scale,
    )
}

fn wheel_position(hwnd: HWND, lparam: LPARAM, dpi: u32) -> Option<PlatformPoint> {
    let mut point = POINT {
        x: i32::from(signed_low_word(lparam)),
        y: i32::from(signed_high_word(lparam)),
    };
    // SAFETY: hwnd is live and point is writable.
    (unsafe { ScreenToClient(hwnd, &mut point) } != 0).then(|| {
        let scale = f64::from(dpi.max(1)) / BASE_DPI;
        PlatformPoint::new(f64::from(point.x) / scale, f64::from(point.y) / scale)
    })
}

fn point_is_in_client(hwnd: HWND, lparam: LPARAM) -> bool {
    let mut rect = windows_sys::Win32::Foundation::RECT::default();
    // SAFETY: hwnd is live and rect is writable.
    if unsafe { GetClientRect(hwnd, &mut rect) } == 0 {
        return false;
    }
    let x = i32::from(signed_low_word(lparam));
    let y = i32::from(signed_high_word(lparam));
    x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom
}

pub(super) fn pointer_modifiers(keys: u16) -> NativeKeyModifiers {
    NativeKeyModifiers::new()
        .shift(keys & MK_SHIFT != 0)
        .control(keys & MK_CONTROL != 0)
        .alt(key_is_pressed(VK_MENU))
        .meta(key_is_pressed(VK_LWIN) || key_is_pressed(VK_RWIN))
}

fn key_is_pressed(key: u16) -> bool {
    // SAFETY: GetKeyState accepts every virtual-key value.
    unsafe { GetKeyState(i32::from(key)) < 0 }
}

fn mouse_buttons(keys: u16) -> u32 {
    let mut buttons = 0;
    for (native, portable) in [
        (MK_LBUTTON, PRIMARY_BUTTON_MASK),
        (MK_RBUTTON, SECONDARY_BUTTON_MASK),
        (MK_MBUTTON, AUXILIARY_BUTTON_MASK),
        (MK_XBUTTON1, BACK_BUTTON_MASK),
        (MK_XBUTTON2, FORWARD_BUTTON_MASK),
    ] {
        if keys & native != 0 {
            buttons |= portable;
        }
    }
    buttons
}

fn mouse_button(message: u32, xbutton: u16) -> PlatformPointerButton {
    match message {
        WM_LBUTTONDOWN | WM_LBUTTONDBLCLK | WM_LBUTTONUP => PlatformPointerButton::Primary,
        WM_RBUTTONDOWN | WM_RBUTTONDBLCLK | WM_RBUTTONUP => PlatformPointerButton::Secondary,
        WM_MBUTTONDOWN | WM_MBUTTONDBLCLK | WM_MBUTTONUP => PlatformPointerButton::Auxiliary,
        WM_XBUTTONDOWN | WM_XBUTTONDBLCLK | WM_XBUTTONUP if xbutton == XBUTTON1 => {
            PlatformPointerButton::Back
        }
        WM_XBUTTONDOWN | WM_XBUTTONDBLCLK | WM_XBUTTONUP if xbutton == XBUTTON2 => {
            PlatformPointerButton::Forward
        }
        _ => PlatformPointerButton::Other(xbutton),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_button_mask_order_is_stable() {
        assert_eq!(mouse_buttons(MK_LBUTTON), PRIMARY_BUTTON_MASK);
        assert_eq!(mouse_buttons(MK_RBUTTON), SECONDARY_BUTTON_MASK);
        assert_eq!(mouse_buttons(MK_MBUTTON), AUXILIARY_BUTTON_MASK);
        assert_eq!(mouse_buttons(MK_XBUTTON1), BACK_BUTTON_MASK);
        assert_eq!(mouse_buttons(MK_XBUTTON2), FORWARD_BUTTON_MASK);
        assert_eq!(
            mouse_buttons(MK_LBUTTON | MK_RBUTTON | MK_MBUTTON),
            PRIMARY_BUTTON_MASK | SECONDARY_BUTTON_MASK | AUXILIARY_BUTTON_MASK
        );
    }

    #[test]
    fn signed_message_coordinates_preserve_negative_values() {
        let lparam = ((u32::from((-7_i16) as u16) << 16) | u32::from((-3_i16) as u16)) as isize;
        assert_eq!(signed_low_word(lparam), -3);
        assert_eq!(signed_high_word(lparam), -7);
    }

    #[test]
    fn capture_change_only_cancels_an_active_button_sequence() {
        let events = WindowsEventQueue::new(4);
        let mut released = WindowsInputState::new();
        released.mouse_inside = true;
        assert!(released
            .handle_message(
                std::ptr::null_mut(),
                PlatformWindowId::new(1),
                96,
                windows_sys::Win32::UI::WindowsAndMessaging::WM_CAPTURECHANGED,
                0,
                1,
                &events,
            )
            .is_some());
        assert!(events.pop().unwrap().is_none());

        let mut pressed = WindowsInputState::new();
        pressed.mouse_inside = true;
        pressed.mouse_buttons = PRIMARY_BUTTON_MASK;
        assert!(pressed
            .handle_message(
                std::ptr::null_mut(),
                PlatformWindowId::new(1),
                96,
                windows_sys::Win32::UI::WindowsAndMessaging::WM_CAPTURECHANGED,
                0,
                1,
                &events,
            )
            .is_some());
        assert!(matches!(
            events.pop().unwrap(),
            Some(PlatformHostEvent::Input {
                event: PlatformInputEvent::Pointer { event }
            }) if event.phase == PlatformPointerPhase::Cancelled
        ));
    }
}
