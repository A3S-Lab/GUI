#![allow(unsafe_code)]

use std::collections::BTreeMap;
use std::time::Instant;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetCapture, ReleaseCapture, SetCapture, SetFocus,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    WM_CANCELMODE, WM_CAPTURECHANGED, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_POINTERCAPTURECHANGED, WM_POINTERDOWN, WM_POINTERENTER, WM_POINTERLEAVE,
    WM_POINTERUP, WM_POINTERUPDATE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP,
};

use crate::input::NativeKeyModifiers;
use crate::platform_host::{
    PlatformHostEvent, PlatformInputDeviceId, PlatformInputEvent, PlatformKeyEvent,
    PlatformKeyState, PlatformPoint, PlatformPointerPhase, PlatformWindowId,
};

use super::events::WindowsEventQueue;
use super::keyboard::{
    modifiers_after_key, same_modifier_group, translate_key, WindowsKeyTranslation,
};

mod mouse;
mod pointer;

use mouse::WM_MOUSELEAVE;
use pointer::WindowsPointerState;

const KEYBOARD_DEVICE: PlatformInputDeviceId = PlatformInputDeviceId::new(2);
const MAX_PRESSED_KEYS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaptureAction {
    None,
    Acquire,
    Release,
}

pub(super) struct WindowsInputMessage {
    result: LRESULT,
    capture: CaptureAction,
    focus: bool,
    pass_to_default: bool,
}

impl WindowsInputMessage {
    fn handled(result: LRESULT) -> Self {
        Self {
            result,
            capture: CaptureAction::None,
            focus: false,
            pass_to_default: false,
        }
    }

    fn capture(mut self, capture: CaptureAction) -> Self {
        self.capture = capture;
        self
    }

    fn focus(mut self) -> Self {
        self.focus = true;
        self
    }

    fn pass_to_default(mut self) -> Self {
        self.pass_to_default = true;
        self
    }

    pub(super) fn apply(self, hwnd: HWND) -> Option<LRESULT> {
        match self.capture {
            CaptureAction::None => {}
            CaptureAction::Acquire => {
                // SAFETY: hwnd is live and remains owned by the calling host.
                unsafe {
                    SetCapture(hwnd);
                }
            }
            CaptureAction::Release => {
                // SAFETY: both APIs operate on thread-owned capture state. The
                // equality guard avoids releasing another window's capture.
                unsafe {
                    if GetCapture() == hwnd {
                        ReleaseCapture();
                    }
                }
            }
        }
        if self.focus {
            // Capture is established first so a synchronous focus-loss callback
            // can cancel and release it without this outer message reacquiring
            // capture afterward.
            // SAFETY: hwnd is the live top-level window handling this message.
            unsafe {
                SetFocus(hwnd);
            }
        }
        (!self.pass_to_default).then_some(self.result)
    }
}

pub(super) struct WindowsInputState {
    started: Instant,
    mouse_inside: bool,
    mouse_buttons: u32,
    mouse_position: PlatformPoint,
    modifiers: NativeKeyModifiers,
    pressed_keys: BTreeMap<u32, WindowsKeyTranslation>,
    active_pointers: BTreeMap<u32, WindowsPointerState>,
    pointer_devices: BTreeMap<(usize, u8), PlatformInputDeviceId>,
    next_pointer_device: u64,
}

impl WindowsInputState {
    pub(super) fn new() -> Self {
        Self {
            started: Instant::now(),
            mouse_inside: false,
            mouse_buttons: 0,
            mouse_position: PlatformPoint::default(),
            modifiers: NativeKeyModifiers::new(),
            pressed_keys: BTreeMap::new(),
            active_pointers: BTreeMap::new(),
            pointer_devices: BTreeMap::new(),
            next_pointer_device: 3,
        }
    }

    pub(super) fn handle_message(
        &mut self,
        hwnd: HWND,
        window: PlatformWindowId,
        dpi: u32,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        events: &WindowsEventQueue,
    ) -> Option<WindowsInputMessage> {
        match message {
            WM_MOUSEMOVE => Some(self.mouse_moved(hwnd, window, dpi, wparam, lparam, events)),
            WM_MOUSELEAVE => Some(self.mouse_left(window, events)),
            WM_LBUTTONDOWN | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONDBLCLK
            | WM_MBUTTONDOWN | WM_MBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => {
                Some(self.mouse_button(
                    hwnd,
                    window,
                    dpi,
                    message,
                    wparam,
                    lparam,
                    PlatformPointerPhase::Pressed,
                    events,
                ))
            }
            WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => Some(self.mouse_button(
                hwnd,
                window,
                dpi,
                message,
                wparam,
                lparam,
                PlatformPointerPhase::Released,
                events,
            )),
            WM_MOUSEWHEEL | WM_MOUSEHWHEEL => {
                Some(self.mouse_wheel(hwnd, window, dpi, message, wparam, lparam, events))
            }
            WM_POINTERENTER
            | WM_POINTERLEAVE
            | WM_POINTERDOWN
            | WM_POINTERUPDATE
            | WM_POINTERUP
            | WM_POINTERCAPTURECHANGED => {
                Some(self.pointer_message(hwnd, window, dpi, message, wparam, events))
            }
            WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
                Some(self.key(window, message, wparam, lparam, events))
            }
            WM_CAPTURECHANGED => {
                if lparam as HWND != hwnd && self.mouse_buttons != 0 {
                    self.cancel_pointer(window, events);
                }
                Some(WindowsInputMessage::handled(0))
            }
            WM_CANCELMODE => {
                self.cancel_pointer(window, events);
                self.cancel_contact_pointers(window, events);
                Some(WindowsInputMessage::handled(0).capture(CaptureAction::Release))
            }
            _ => None,
        }
    }

    pub(super) fn focus_lost(
        &mut self,
        window: PlatformWindowId,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        self.cancel_pointer(window, events);
        self.cancel_contact_pointers(window, events);
        let timestamp_micros = self.timestamp_micros();
        let pressed_keys = std::mem::take(&mut self.pressed_keys);
        for (_, key) in pressed_keys {
            self.push_key(
                window,
                key.physical_key,
                key.logical_key,
                None,
                PlatformKeyState::Released,
                false,
                NativeKeyModifiers::new(),
                timestamp_micros,
                events,
            );
        }
        self.set_modifiers(
            window,
            KEYBOARD_DEVICE,
            NativeKeyModifiers::new(),
            timestamp_micros,
            events,
        );
        WindowsInputMessage::handled(0).capture(CaptureAction::Release)
    }

    fn key(
        &mut self,
        window: PlatformWindowId,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        let state = if matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN) {
            PlatformKeyState::Pressed
        } else {
            PlatformKeyState::Released
        };
        let virtual_key = (wparam as u32).min(0xff) as u16;
        let translated = translate_key(virtual_key, lparam, state, self.modifiers);
        let another_modifier_is_pressed = self.pressed_keys.iter().any(|(identity, key)| {
            *identity != translated.identity
                && same_modifier_group(key.virtual_key, translated.virtual_key)
        });
        let modifiers = modifiers_after_key(
            self.modifiers,
            virtual_key,
            state,
            another_modifier_is_pressed,
        );
        if state == PlatformKeyState::Pressed
            && !self.pressed_keys.contains_key(&translated.identity)
            && self.pressed_keys.len() >= MAX_PRESSED_KEYS
        {
            events.fail(format!(
                "Windows input reached its {MAX_PRESSED_KEYS}-key pressed-state limit"
            ));
            return key_message_result(message);
        }
        let timestamp_micros = self.timestamp_micros();
        self.set_modifiers(window, KEYBOARD_DEVICE, modifiers, timestamp_micros, events);
        let repeat = state == PlatformKeyState::Pressed && (lparam as u64 & (1 << 30)) != 0;
        let (physical_key, logical_key, text) = match state {
            PlatformKeyState::Pressed => {
                self.pressed_keys
                    .entry(translated.identity)
                    .or_insert_with(|| translated.clone());
                (
                    translated.physical_key,
                    translated.logical_key,
                    translated.text,
                )
            }
            PlatformKeyState::Released => {
                let pressed = self.pressed_keys.remove(&translated.identity);
                let pressed = pressed.as_ref().unwrap_or(&translated);
                (
                    pressed.physical_key.clone(),
                    pressed.logical_key.clone(),
                    None,
                )
            }
        };
        self.push_key(
            window,
            physical_key,
            logical_key,
            text,
            state,
            repeat,
            modifiers,
            timestamp_micros,
            events,
        );
        key_message_result(message)
    }

    fn set_modifiers(
        &mut self,
        window: PlatformWindowId,
        device: PlatformInputDeviceId,
        modifiers: NativeKeyModifiers,
        timestamp_micros: u64,
        events: &WindowsEventQueue,
    ) {
        if self.modifiers == modifiers {
            return;
        }
        self.modifiers = modifiers;
        events.push(PlatformHostEvent::Input {
            event: PlatformInputEvent::ModifiersChanged {
                window,
                device,
                modifiers,
                timestamp_micros,
            },
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn push_key(
        &self,
        window: PlatformWindowId,
        physical_key: String,
        logical_key: String,
        text: Option<String>,
        state: PlatformKeyState,
        repeat: bool,
        modifiers: NativeKeyModifiers,
        timestamp_micros: u64,
        events: &WindowsEventQueue,
    ) {
        events.push(PlatformHostEvent::Input {
            event: PlatformInputEvent::Key {
                event: PlatformKeyEvent {
                    window,
                    device: KEYBOARD_DEVICE,
                    physical_key,
                    logical_key,
                    text,
                    state,
                    repeat,
                    modifiers,
                    timestamp_micros,
                },
            },
        });
    }

    fn timestamp_micros(&self) -> u64 {
        self.started.elapsed().as_micros().min(u128::from(u64::MAX)) as u64
    }
}

fn key_message_result(message: u32) -> WindowsInputMessage {
    let handled = WindowsInputMessage::handled(0);
    if matches!(message, WM_SYSKEYDOWN | WM_SYSKEYUP) {
        handled.pass_to_default()
    } else {
        handled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_key_translation_preserves_default_window_processing() {
        let events = WindowsEventQueue::new(2);
        let mut input = WindowsInputState::new();
        let message = input
            .handle_message(
                std::ptr::null_mut(),
                PlatformWindowId::new(1),
                96,
                WM_SYSKEYDOWN,
                0x73,
                (0x3e_u32 << 16) as isize,
                &events,
            )
            .unwrap();

        assert_eq!(message.apply(std::ptr::null_mut()), None);
        assert!(matches!(
            events.pop().unwrap(),
            Some(PlatformHostEvent::Input {
                event: PlatformInputEvent::Key { event }
            }) if event.physical_key == "F4" && event.state == PlatformKeyState::Pressed
        ));
    }

    #[test]
    fn releasing_one_shift_keeps_the_other_shift_pressed() {
        let events = WindowsEventQueue::new(8);
        let mut input = WindowsInputState::new();
        let window = PlatformWindowId::new(1);
        let key_message = |scan_code: u32, released: bool| {
            let mut value = 1_u32 | (scan_code << 16);
            if released {
                value |= (1 << 30) | (1 << 31);
            }
            value as isize
        };

        let _ = input.handle_message(
            std::ptr::null_mut(),
            window,
            96,
            WM_KEYDOWN,
            0x10,
            key_message(0x2a, false),
            &events,
        );
        let _ = input.handle_message(
            std::ptr::null_mut(),
            window,
            96,
            WM_KEYDOWN,
            0x10,
            key_message(0x36, false),
            &events,
        );
        let _ = input.handle_message(
            std::ptr::null_mut(),
            window,
            96,
            WM_KEYUP,
            0x10,
            key_message(0x2a, true),
            &events,
        );
        assert!(input.modifiers.shift);

        let _ = input.handle_message(
            std::ptr::null_mut(),
            window,
            96,
            WM_KEYUP,
            0x10,
            key_message(0x36, true),
            &events,
        );
        assert!(!input.modifiers.shift);
    }

    #[test]
    fn pressed_key_state_is_bounded_before_accepting_a_new_identity() {
        let events = WindowsEventQueue::new(2);
        let mut input = WindowsInputState::new();
        for identity in 0..MAX_PRESSED_KEYS as u32 {
            input.pressed_keys.insert(
                identity,
                WindowsKeyTranslation {
                    identity,
                    virtual_key: 0,
                    physical_key: "Unidentified".to_string(),
                    logical_key: "Unidentified".to_string(),
                    text: None,
                },
            );
        }

        let message = input
            .handle_message(
                std::ptr::null_mut(),
                PlatformWindowId::new(1),
                96,
                WM_KEYDOWN,
                0x41,
                (0x1e_u32 << 16) as isize,
                &events,
            )
            .unwrap();

        assert_eq!(message.apply(std::ptr::null_mut()), Some(0));
        assert_eq!(input.pressed_keys.len(), MAX_PRESSED_KEYS);
        assert!(events.pop().unwrap_err().to_string().contains("512-key"));
    }
}
