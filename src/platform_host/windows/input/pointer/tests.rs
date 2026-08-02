use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::Input::Pointer::{
    POINTER_CHANGE_FIRSTBUTTON_DOWN, POINTER_CHANGE_FIRSTBUTTON_UP,
    POINTER_CHANGE_SECONDBUTTON_DOWN, POINTER_FLAG_INCONTACT, POINTER_FLAG_SECONDBUTTON,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    PEN_FLAG_BARREL, PEN_MASK_PRESSURE, TOUCH_MASK_PRESSURE, WM_POINTERCAPTURECHANGED,
};

use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::platform_host::windows::events::WindowsEventQueue;
use crate::platform_host::windows::input::WindowsInputState;
use crate::platform_host::{
    PlatformHostEvent, PlatformInputEvent, PlatformPoint, PlatformPointerButton,
    PlatformPointerEvent, PlatformPointerPhase, PlatformWindowId,
};

use super::{
    logical_client_position, normalize_pressure, pen_buttons, pointer_buttons, NativePointerPhase,
    WindowsPointerSample, MAX_ACTIVE_POINTERS, POINTER_ID_NAMESPACE,
};

fn sample(
    native_id: u32,
    modality: NativeInputModality,
    position: PlatformPoint,
) -> WindowsPointerSample {
    WindowsPointerSample {
        native_id,
        source_device: 11,
        modality,
        position,
        pressed_buttons: 0,
        changed_button: None,
        pressure: None,
        modifiers: NativeKeyModifiers::new(),
        cancelled: false,
    }
}

fn pop_pointer(events: &WindowsEventQueue) -> PlatformPointerEvent {
    match events.pop().unwrap().unwrap() {
        PlatformHostEvent::Input {
            event: PlatformInputEvent::Pointer { event },
        } => event,
        event => panic!("expected pointer event, got {event:?}"),
    }
}

#[test]
fn touch_sequence_is_namespaced_dpi_correct_and_pressure_aware() {
    let events = WindowsEventQueue::new(16);
    let mut input = WindowsInputState::new();
    let window = PlatformWindowId::new(1);
    let mut touch = sample(
        7,
        NativeInputModality::Touch,
        PlatformPoint::new(40.0, 24.0),
    );
    touch.pressed_buttons = 1;
    touch.changed_button = Some(PlatformPointerButton::Primary);
    touch.pressure = Some(0.5);

    input.handle_pointer_sample(window, NativePointerPhase::Down, touch, &events);
    let entered = pop_pointer(&events);
    let pressed = pop_pointer(&events);
    assert_eq!(entered.phase, PlatformPointerPhase::Entered);
    assert_eq!(pressed.phase, PlatformPointerPhase::Pressed);
    assert_eq!(pressed.pointer.get(), POINTER_ID_NAMESPACE | 7);
    assert_eq!(pressed.device.get(), 3);
    assert_eq!(pressed.position, PlatformPoint::new(40.0, 24.0));
    assert_eq!(pressed.button, Some(PlatformPointerButton::Primary));
    assert_eq!(pressed.pressed_buttons, 1);
    assert_eq!(pressed.pressure, Some(0.5));

    input.handle_pointer_sample(window, NativePointerPhase::Enter, touch, &events);
    assert!(events.pop().unwrap().is_none());

    touch.position = PlatformPoint::new(48.0, 32.0);
    input.handle_pointer_sample(window, NativePointerPhase::Move, touch, &events);
    assert_eq!(pop_pointer(&events).phase, PlatformPointerPhase::Moved);

    touch.pressed_buttons = 0;
    touch.changed_button = Some(PlatformPointerButton::Primary);
    touch.pressure = Some(0.0);
    input.handle_pointer_sample(window, NativePointerPhase::Up, touch, &events);
    assert_eq!(pop_pointer(&events).phase, PlatformPointerPhase::Released);
    input.handle_pointer_sample(window, NativePointerPhase::Leave, touch, &events);
    assert_eq!(pop_pointer(&events).phase, PlatformPointerPhase::Left);
    assert!(input.active_pointers.is_empty());

    assert_eq!(
        logical_client_position(POINT { x: 80, y: 48 }, 192),
        PlatformPoint::new(40.0, 24.0)
    );
    assert_eq!(
        normalize_pressure(TOUCH_MASK_PRESSURE, TOUCH_MASK_PRESSURE, 512),
        Some(0.5)
    );
}

#[test]
fn pen_barrel_button_and_pressure_map_to_portable_semantics() {
    assert_eq!(
        pointer_buttons(POINTER_FLAG_INCONTACT | POINTER_FLAG_SECONDBUTTON),
        2
    );
    assert_eq!(pen_buttons(PEN_FLAG_BARREL), 2);
    assert_eq!(pen_buttons(0), 0);
    assert_eq!(
        super::changed_button(POINTER_CHANGE_SECONDBUTTON_DOWN),
        Some(PlatformPointerButton::Secondary)
    );
    assert_eq!(
        normalize_pressure(PEN_MASK_PRESSURE, PEN_MASK_PRESSURE, 768),
        Some(0.75)
    );
    assert_eq!(normalize_pressure(0, PEN_MASK_PRESSURE, 768), None);
    assert_eq!(
        normalize_pressure(PEN_MASK_PRESSURE, PEN_MASK_PRESSURE, 2048),
        Some(1.0)
    );
}

#[test]
fn concurrent_contacts_cancel_independently_and_on_focus_loss() {
    let events = WindowsEventQueue::new(16);
    let mut input = WindowsInputState::new();
    let window = PlatformWindowId::new(1);
    let mut first = sample(1, NativeInputModality::Touch, PlatformPoint::new(1.0, 1.0));
    first.pressed_buttons = 1;
    first.changed_button = Some(PlatformPointerButton::Primary);
    let mut second = sample(2, NativeInputModality::Touch, PlatformPoint::new(2.0, 2.0));
    second.pressed_buttons = 1;
    second.changed_button = Some(PlatformPointerButton::Primary);
    input.handle_pointer_sample(window, NativePointerPhase::Down, first, &events);
    input.handle_pointer_sample(window, NativePointerPhase::Down, second, &events);
    for _ in 0..4 {
        let _ = pop_pointer(&events);
    }

    let message = input
        .handle_message(
            std::ptr::null_mut(),
            window,
            96,
            WM_POINTERCAPTURECHANGED,
            1,
            0,
            &events,
        )
        .unwrap();
    assert_eq!(message.apply(std::ptr::null_mut()), Some(0));
    let cancelled = pop_pointer(&events);
    assert_eq!(cancelled.phase, PlatformPointerPhase::Cancelled);
    assert_eq!(cancelled.pointer.get(), POINTER_ID_NAMESPACE | 1);
    assert!(!input.active_pointers.contains_key(&1));
    assert!(input.active_pointers.contains_key(&2));

    let _ = input.focus_lost(window, &events);
    let cancelled = pop_pointer(&events);
    assert_eq!(cancelled.phase, PlatformPointerPhase::Cancelled);
    assert_eq!(cancelled.pointer.get(), POINTER_ID_NAMESPACE | 2);
    assert!(input.active_pointers.is_empty());
}

#[test]
fn cancelled_sample_reports_its_final_position_and_clears_buttons() {
    let events = WindowsEventQueue::new(8);
    let mut input = WindowsInputState::new();
    let window = PlatformWindowId::new(1);
    let mut touch = sample(9, NativeInputModality::Touch, PlatformPoint::new(4.0, 8.0));
    touch.pressed_buttons = 1;
    touch.changed_button = Some(PlatformPointerButton::Primary);
    input.handle_pointer_sample(window, NativePointerPhase::Down, touch, &events);
    let _ = pop_pointer(&events);
    let _ = pop_pointer(&events);

    touch.position = PlatformPoint::new(12.0, 16.0);
    touch.cancelled = true;
    input.handle_pointer_sample(window, NativePointerPhase::Move, touch, &events);

    let cancelled = pop_pointer(&events);
    assert_eq!(cancelled.phase, PlatformPointerPhase::Cancelled);
    assert_eq!(cancelled.position, PlatformPoint::new(12.0, 16.0));
    assert_eq!(cancelled.pressed_buttons, 0);
    assert!(input.active_pointers.is_empty());
}

#[test]
fn active_pointer_state_is_bounded_before_allocating_a_new_contact() {
    let events = WindowsEventQueue::new(MAX_ACTIVE_POINTERS * 2 + 1);
    let mut input = WindowsInputState::new();
    let window = PlatformWindowId::new(1);
    for native_id in 1..=MAX_ACTIVE_POINTERS as u32 {
        let mut touch = sample(
            native_id,
            NativeInputModality::Touch,
            PlatformPoint::default(),
        );
        touch.pressed_buttons = 1;
        touch.changed_button = Some(PlatformPointerButton::Primary);
        input.handle_pointer_sample(window, NativePointerPhase::Down, touch, &events);
    }
    let mut overflow = sample(
        MAX_ACTIVE_POINTERS as u32 + 1,
        NativeInputModality::Touch,
        PlatformPoint::default(),
    );
    overflow.pressed_buttons = 1;
    overflow.changed_button = Some(PlatformPointerButton::Primary);
    input.handle_pointer_sample(window, NativePointerPhase::Down, overflow, &events);

    assert_eq!(input.active_pointers.len(), MAX_ACTIVE_POINTERS);
    assert!(events
        .pop()
        .unwrap_err()
        .to_string()
        .contains("256-pointer"));
}

#[test]
fn native_button_change_values_cover_contact_edges() {
    assert_eq!(
        super::changed_button(POINTER_CHANGE_FIRSTBUTTON_DOWN),
        Some(PlatformPointerButton::Primary)
    );
    assert_eq!(
        super::changed_button(POINTER_CHANGE_FIRSTBUTTON_UP),
        Some(PlatformPointerButton::Primary)
    );
}
