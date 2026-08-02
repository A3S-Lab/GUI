#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::{HWND, POINT, WPARAM};
use windows_sys::Win32::UI::Input::Pointer::{
    POINTER_CHANGE_FIFTHBUTTON_DOWN, POINTER_CHANGE_FIFTHBUTTON_UP,
    POINTER_CHANGE_FIRSTBUTTON_DOWN, POINTER_CHANGE_FIRSTBUTTON_UP,
    POINTER_CHANGE_FOURTHBUTTON_DOWN, POINTER_CHANGE_FOURTHBUTTON_UP,
    POINTER_CHANGE_SECONDBUTTON_DOWN, POINTER_CHANGE_SECONDBUTTON_UP,
    POINTER_CHANGE_THIRDBUTTON_DOWN, POINTER_CHANGE_THIRDBUTTON_UP, POINTER_FLAG_FIFTHBUTTON,
    POINTER_FLAG_FIRSTBUTTON, POINTER_FLAG_FOURTHBUTTON, POINTER_FLAG_SECONDBUTTON,
    POINTER_FLAG_THIRDBUTTON,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IsWindowVisible, WM_POINTERCAPTURECHANGED, WM_POINTERDOWN, WM_POINTERENTER, WM_POINTERLEAVE,
    WM_POINTERUP, WM_POINTERUPDATE,
};

use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::platform_host::{
    PlatformHostEvent, PlatformInputDeviceId, PlatformInputEvent, PlatformPoint,
    PlatformPointerButton, PlatformPointerEvent, PlatformPointerId, PlatformPointerPhase,
    PlatformWindowId,
};

use super::super::events::WindowsEventQueue;
use super::{WindowsInputMessage, WindowsInputState};

mod native;

use native::read_pointer_sample;

const BASE_DPI: f64 = 96.0;
const PRIMARY_BUTTON_MASK: u32 = 1;
const SECONDARY_BUTTON_MASK: u32 = 2;
const AUXILIARY_BUTTON_MASK: u32 = 4;
const BACK_BUTTON_MASK: u32 = 8;
const FORWARD_BUTTON_MASK: u32 = 16;
const MAX_POINTER_DEVICES: usize = 64;
pub(super) const MAX_ACTIVE_POINTERS: usize = 256;
pub(super) const POINTER_ID_NAMESPACE: u64 = 1_u64 << 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NativePointerPhase {
    Enter,
    Leave,
    Move,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WindowsPointerSample {
    native_id: u32,
    source_device: usize,
    modality: NativeInputModality,
    position: PlatformPoint,
    pressed_buttons: u32,
    changed_button: Option<PlatformPointerButton>,
    pressure: Option<f64>,
    modifiers: NativeKeyModifiers,
    cancelled: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct WindowsPointerState {
    device: PlatformInputDeviceId,
    pointer: PlatformPointerId,
    modality: NativeInputModality,
    position: PlatformPoint,
    pressed_buttons: u32,
    pressure: Option<f64>,
    modifiers: NativeKeyModifiers,
}

impl WindowsInputState {
    pub(super) fn pointer_message(
        &mut self,
        hwnd: HWND,
        window: PlatformWindowId,
        dpi: u32,
        message: u32,
        wparam: WPARAM,
        events: &WindowsEventQueue,
    ) -> WindowsInputMessage {
        let native_id = pointer_id(wparam);
        if message == WM_POINTERCAPTURECHANGED {
            self.cancel_contact_pointer(window, native_id, events);
            return WindowsInputMessage::handled(0);
        }
        let Some(phase) = native_phase(message) else {
            return WindowsInputMessage::handled(0).pass_to_default();
        };
        let sample = match read_pointer_sample(hwnd, dpi, native_id) {
            Ok(Some(sample)) => sample,
            Ok(None) => return WindowsInputMessage::handled(0).pass_to_default(),
            Err(error) => {
                events.fail(error);
                return WindowsInputMessage::handled(0);
            }
        };
        self.handle_pointer_sample(window, phase, sample, events);

        let handled = WindowsInputMessage::handled(0);
        if phase == NativePointerPhase::Down {
            // SAFETY: hwnd is the live top-level window handling this message.
            let visible = unsafe { IsWindowVisible(hwnd) } != 0;
            if visible {
                return handled.focus();
            }
        }
        handled
    }

    pub(super) fn handle_pointer_sample(
        &mut self,
        window: PlatformWindowId,
        phase: NativePointerPhase,
        sample: WindowsPointerSample,
        events: &WindowsEventQueue,
    ) {
        if sample.native_id == 0 {
            events.fail("Windows delivered a zero pointer identifier");
            return;
        }
        if !matches!(
            sample.modality,
            NativeInputModality::Touch | NativeInputModality::Pen
        ) {
            events.fail("Windows WM_POINTER samples must be touch or pen input");
            return;
        }
        if sample.cancelled {
            self.cancel_contact_sample(window, sample, events);
            return;
        }
        if phase == NativePointerPhase::Leave {
            self.leave_contact_pointer(window, sample, events);
            return;
        }

        let existing = self.active_pointers.get(&sample.native_id).copied();
        if existing.is_none() && phase == NativePointerPhase::Up {
            return;
        }
        if existing.is_none() && self.active_pointers.len() >= MAX_ACTIVE_POINTERS {
            events.fail(format!(
                "Windows input reached its {MAX_ACTIVE_POINTERS}-pointer active-state limit"
            ));
            return;
        }
        let timestamp_micros = self.timestamp_micros();
        let mut state = match existing {
            Some(state) => state,
            None => {
                let Some(device) = self.pointer_device(sample, events) else {
                    return;
                };
                let state = WindowsPointerState {
                    device,
                    pointer: namespaced_pointer_id(sample.native_id),
                    modality: sample.modality,
                    position: sample.position,
                    pressed_buttons: 0,
                    pressure: sample.pressure,
                    modifiers: sample.modifiers,
                };
                self.push_contact_pointer(
                    window,
                    state,
                    PlatformPointerPhase::Entered,
                    None,
                    timestamp_micros,
                    events,
                );
                state
            }
        };
        self.set_modifiers(
            window,
            state.device,
            sample.modifiers,
            timestamp_micros,
            events,
        );
        let previous_buttons = state.pressed_buttons;
        state.position = sample.position;
        state.pressed_buttons = sample.pressed_buttons;
        state.pressure = sample.pressure;
        state.modifiers = sample.modifiers;

        match phase {
            NativePointerPhase::Enter => {}
            NativePointerPhase::Move => self.push_contact_pointer(
                window,
                state,
                PlatformPointerPhase::Moved,
                None,
                timestamp_micros,
                events,
            ),
            NativePointerPhase::Down | NativePointerPhase::Up => {
                let phase = if phase == NativePointerPhase::Down {
                    PlatformPointerPhase::Pressed
                } else {
                    PlatformPointerPhase::Released
                };
                let button = sample.changed_button.unwrap_or_else(|| {
                    edge_button(phase, previous_buttons, sample.pressed_buttons)
                });
                self.push_contact_pointer(
                    window,
                    state,
                    phase,
                    Some(button),
                    timestamp_micros,
                    events,
                );
            }
            NativePointerPhase::Leave => unreachable!("leave is handled before state lookup"),
        }
        self.active_pointers.insert(sample.native_id, state);
    }

    pub(super) fn cancel_contact_pointers(
        &mut self,
        window: PlatformWindowId,
        events: &WindowsEventQueue,
    ) {
        let timestamp_micros = self.timestamp_micros();
        for (_, mut state) in std::mem::take(&mut self.active_pointers) {
            state.pressed_buttons = 0;
            self.push_contact_pointer(
                window,
                state,
                PlatformPointerPhase::Cancelled,
                None,
                timestamp_micros,
                events,
            );
        }
    }

    fn cancel_contact_pointer(
        &mut self,
        window: PlatformWindowId,
        native_id: u32,
        events: &WindowsEventQueue,
    ) {
        let Some(mut state) = self.active_pointers.remove(&native_id) else {
            return;
        };
        state.pressed_buttons = 0;
        self.push_contact_pointer(
            window,
            state,
            PlatformPointerPhase::Cancelled,
            None,
            self.timestamp_micros(),
            events,
        );
    }

    fn cancel_contact_sample(
        &mut self,
        window: PlatformWindowId,
        sample: WindowsPointerSample,
        events: &WindowsEventQueue,
    ) {
        let Some(mut state) = self.active_pointers.remove(&sample.native_id) else {
            return;
        };
        let timestamp_micros = self.timestamp_micros();
        self.set_modifiers(
            window,
            state.device,
            sample.modifiers,
            timestamp_micros,
            events,
        );
        state.position = sample.position;
        state.pressed_buttons = 0;
        state.pressure = sample.pressure;
        state.modifiers = sample.modifiers;
        self.push_contact_pointer(
            window,
            state,
            PlatformPointerPhase::Cancelled,
            None,
            timestamp_micros,
            events,
        );
    }

    fn leave_contact_pointer(
        &mut self,
        window: PlatformWindowId,
        sample: WindowsPointerSample,
        events: &WindowsEventQueue,
    ) {
        let Some(mut state) = self.active_pointers.remove(&sample.native_id) else {
            return;
        };
        let cancelled = state.pressed_buttons != 0;
        state.position = sample.position;
        state.pressed_buttons = 0;
        state.pressure = sample.pressure;
        state.modifiers = sample.modifiers;
        self.push_contact_pointer(
            window,
            state,
            if cancelled {
                PlatformPointerPhase::Cancelled
            } else {
                PlatformPointerPhase::Left
            },
            None,
            self.timestamp_micros(),
            events,
        );
    }

    fn pointer_device(
        &mut self,
        sample: WindowsPointerSample,
        events: &WindowsEventQueue,
    ) -> Option<PlatformInputDeviceId> {
        let key = (sample.source_device, modality_tag(sample.modality));
        if let Some(device) = self.pointer_devices.get(&key) {
            return Some(*device);
        }
        if self.pointer_devices.len() >= MAX_POINTER_DEVICES {
            events.fail(format!(
                "Windows input reached its {MAX_POINTER_DEVICES}-pointer-device identity limit"
            ));
            return None;
        }
        let device = PlatformInputDeviceId::new(self.next_pointer_device);
        self.next_pointer_device += 1;
        self.pointer_devices.insert(key, device);
        Some(device)
    }

    fn push_contact_pointer(
        &self,
        window: PlatformWindowId,
        state: WindowsPointerState,
        phase: PlatformPointerPhase,
        button: Option<PlatformPointerButton>,
        timestamp_micros: u64,
        events: &WindowsEventQueue,
    ) {
        events.push(PlatformHostEvent::Input {
            event: PlatformInputEvent::Pointer {
                event: PlatformPointerEvent {
                    window,
                    device: state.device,
                    pointer: state.pointer,
                    modality: state.modality,
                    phase,
                    position: state.position,
                    button,
                    pressed_buttons: state.pressed_buttons,
                    pressure: state.pressure,
                    modifiers: state.modifiers,
                    timestamp_micros,
                },
            },
        });
    }
}

fn native_phase(message: u32) -> Option<NativePointerPhase> {
    match message {
        WM_POINTERENTER => Some(NativePointerPhase::Enter),
        WM_POINTERLEAVE => Some(NativePointerPhase::Leave),
        WM_POINTERUPDATE => Some(NativePointerPhase::Move),
        WM_POINTERDOWN => Some(NativePointerPhase::Down),
        WM_POINTERUP => Some(NativePointerPhase::Up),
        _ => None,
    }
}

fn pointer_id(wparam: WPARAM) -> u32 {
    wparam as u16 as u32
}

fn namespaced_pointer_id(native_id: u32) -> PlatformPointerId {
    PlatformPointerId::new(POINTER_ID_NAMESPACE | u64::from(native_id))
}

fn modality_tag(modality: NativeInputModality) -> u8 {
    match modality {
        NativeInputModality::Touch => 0,
        NativeInputModality::Pen => 1,
        _ => 2,
    }
}

fn pointer_buttons(flags: u32) -> u32 {
    let mut buttons = 0;
    for (native, portable) in [
        (POINTER_FLAG_FIRSTBUTTON, PRIMARY_BUTTON_MASK),
        (POINTER_FLAG_SECONDBUTTON, SECONDARY_BUTTON_MASK),
        (POINTER_FLAG_THIRDBUTTON, AUXILIARY_BUTTON_MASK),
        (POINTER_FLAG_FOURTHBUTTON, BACK_BUTTON_MASK),
        (POINTER_FLAG_FIFTHBUTTON, FORWARD_BUTTON_MASK),
    ] {
        if flags & native != 0 {
            buttons |= portable;
        }
    }
    buttons
}

fn changed_button(change: i32) -> Option<PlatformPointerButton> {
    match change {
        POINTER_CHANGE_FIRSTBUTTON_DOWN | POINTER_CHANGE_FIRSTBUTTON_UP => {
            Some(PlatformPointerButton::Primary)
        }
        POINTER_CHANGE_SECONDBUTTON_DOWN | POINTER_CHANGE_SECONDBUTTON_UP => {
            Some(PlatformPointerButton::Secondary)
        }
        POINTER_CHANGE_THIRDBUTTON_DOWN | POINTER_CHANGE_THIRDBUTTON_UP => {
            Some(PlatformPointerButton::Auxiliary)
        }
        POINTER_CHANGE_FOURTHBUTTON_DOWN | POINTER_CHANGE_FOURTHBUTTON_UP => {
            Some(PlatformPointerButton::Back)
        }
        POINTER_CHANGE_FIFTHBUTTON_DOWN | POINTER_CHANGE_FIFTHBUTTON_UP => {
            Some(PlatformPointerButton::Forward)
        }
        _ => None,
    }
}

fn edge_button(
    phase: PlatformPointerPhase,
    previous_buttons: u32,
    pressed_buttons: u32,
) -> PlatformPointerButton {
    let changed = if phase == PlatformPointerPhase::Pressed {
        pressed_buttons & !previous_buttons
    } else {
        previous_buttons & !pressed_buttons
    };
    button_from_mask(changed).unwrap_or(PlatformPointerButton::Primary)
}

fn button_from_mask(mask: u32) -> Option<PlatformPointerButton> {
    [
        (PRIMARY_BUTTON_MASK, PlatformPointerButton::Primary),
        (SECONDARY_BUTTON_MASK, PlatformPointerButton::Secondary),
        (AUXILIARY_BUTTON_MASK, PlatformPointerButton::Auxiliary),
        (BACK_BUTTON_MASK, PlatformPointerButton::Back),
        (FORWARD_BUTTON_MASK, PlatformPointerButton::Forward),
    ]
    .into_iter()
    .find_map(|(button_mask, button)| (mask & button_mask != 0).then_some(button))
}

fn normalize_pressure(mask: u32, pressure_mask: u32, pressure: u32) -> Option<f64> {
    (mask & pressure_mask != 0).then(|| f64::from(pressure.min(1024)) / 1024.0)
}

fn logical_client_position(point: POINT, dpi: u32) -> PlatformPoint {
    let scale = f64::from(dpi.max(1)) / BASE_DPI;
    PlatformPoint::new(f64::from(point.x) / scale, f64::from(point.y) / scale)
}

#[cfg(test)]
mod tests;
