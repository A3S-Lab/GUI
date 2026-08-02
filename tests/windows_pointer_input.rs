#![cfg(all(target_os = "windows", feature = "host-windows"))]
#![allow(unsafe_code)]

use std::num::NonZeroIsize;
use std::thread;
use std::time::Duration;

use a3s_gui::geometry::{Rect, Size};
use a3s_gui::input::NativeInputModality;
use a3s_gui::platform_host::{
    PlatformAccessibilitySnapshot, PlatformHost, PlatformHostCommand, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformInputEvent, PlatformPointerButton,
    PlatformPointerPhase, PlatformPresentationRequest, PlatformWindowCommand, PlatformWindowId,
    PlatformWindowSpec, WindowsPlatformHost,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::Foundation::{POINT, RECT};
use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
use windows_sys::Win32::UI::Input::Pointer::{
    InitializeTouchInjection, InjectTouchInput, POINTER_FLAG_DOWN, POINTER_FLAG_INCONTACT,
    POINTER_FLAG_INRANGE, POINTER_FLAG_UP, POINTER_FLAG_UPDATE, POINTER_TOUCH_INFO,
    TOUCH_FEEDBACK_NONE,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    SetForegroundWindow, SetWindowPos, WindowFromPoint, HWND_TOPMOST, PT_TOUCH, SWP_NOMOVE,
    SWP_NOSIZE, TOUCH_MASK_CONTACTAREA, TOUCH_MASK_PRESSURE,
};

const WINDOW: PlatformWindowId = PlatformWindowId::new(73);
const TOUCH_ID: u32 = 0;

#[test]
fn injected_touch_reaches_the_real_hwnd_without_compatibility_mouse_events() {
    let mut host = WindowsPlatformHost::new().unwrap();
    host.prepare(open_window()).unwrap();
    host.commit().unwrap();
    while host.poll_event().unwrap().is_some() {}

    let surface = host.surface(WINDOW).unwrap();
    let scale_factor = surface.scale_factor();
    let physical_size = surface.physical_size();
    let hwnd = match surface.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd,
        other => panic!("expected Win32 surface handle, got {other:?}"),
    };
    drop(surface);

    let sequence = inject_touch_sequence(&mut host, hwnd, physical_size);
    host.shutdown().unwrap();
    let events = sequence.unwrap();
    let pointers = events
        .iter()
        .filter_map(|event| match event {
            PlatformHostEvent::Input {
                event: PlatformInputEvent::Pointer { event },
            } => Some(event),
            _ => None,
        })
        .collect::<Vec<_>>();

    let pressed = pointers
        .iter()
        .find(|event| event.phase == PlatformPointerPhase::Pressed)
        .expect("injected touch press");
    assert_eq!(pressed.modality, NativeInputModality::Touch);
    assert_eq!(pressed.button, Some(PlatformPointerButton::Primary));
    assert_eq!(pressed.pressed_buttons, 1);
    assert_eq!(pressed.pressure, Some(0.5));
    assert!((pressed.position.x - f64::from(physical_size.0 / 2) / scale_factor).abs() < 1.0);
    assert!((pressed.position.y - f64::from(physical_size.1 / 2) / scale_factor).abs() < 1.0);
    assert!(pointers.iter().any(|event| {
        event.pointer == pressed.pointer && event.phase == PlatformPointerPhase::Moved
    }));
    assert!(pointers.iter().any(|event| {
        event.pointer == pressed.pointer && event.phase == PlatformPointerPhase::Released
    }));
    assert!(pointers
        .iter()
        .all(|event| event.modality == NativeInputModality::Touch));
}

fn open_window() -> PlatformHostTransaction {
    let spec = PlatformWindowSpec {
        id: WINDOW,
        title: "A3S WM_POINTER injection".to_string(),
        logical_size: Size::new(320.0, 240.0),
        min_size: None,
        max_size: None,
        resizable: false,
        visible: true,
    };
    PlatformHostTransaction {
        revision: PlatformHostRevision::new(1),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Open { spec: spec.clone() },
            },
            PlatformHostCommand::Accessibility {
                snapshot: Box::new(PlatformAccessibilitySnapshot {
                    window: WINDOW,
                    root: None,
                }),
            },
            PlatformHostCommand::Present {
                request: PlatformPresentationRequest {
                    window: WINDOW,
                    logical_size: spec.logical_size,
                    scale_factor: 1.0,
                    scene_fingerprint: 1,
                    damage: vec![Rect::new(0.0, 0.0, 320.0, 240.0)],
                },
            },
        ],
    }
}

fn inject_touch_sequence(
    host: &mut WindowsPlatformHost,
    hwnd: NonZeroIsize,
    physical_size: (u32, u32),
) -> Result<Vec<PlatformHostEvent>, String> {
    if unsafe {
        SetWindowPos(
            hwnd.get() as _,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        )
    } == 0
    {
        return Err(last_windows_error("SetWindowPos"));
    }
    // The test process owns the newly shown top-level window. Foreground
    // activation is best-effort; topmost placement provides deterministic hit
    // testing even when Windows declines activation in a remote session.
    unsafe {
        SetForegroundWindow(hwnd.get() as _);
    }
    for _ in 0..50 {
        while host
            .poll_event()
            .map_err(|error| error.to_string())?
            .is_some()
        {}
        thread::sleep(Duration::from_millis(2));
    }
    if unsafe { InitializeTouchInjection(1, TOUCH_FEEDBACK_NONE) } == 0 {
        return Err(last_windows_error("InitializeTouchInjection"));
    }

    let client = POINT {
        x: (physical_size.0 / 2) as i32,
        y: (physical_size.1 / 2) as i32,
    };
    let mut screen = client;
    if unsafe { ClientToScreen(hwnd.get() as _, &mut screen) } == 0 {
        return Err(last_windows_error("ClientToScreen"));
    }
    let target = unsafe { WindowFromPoint(screen) };
    if target != hwnd.get() as _ {
        return Err(format!(
            "touch point targets HWND {target:?} instead of the test HWND {:?}",
            hwnd.get()
        ));
    }

    let down = touch_contact(
        screen,
        POINTER_FLAG_DOWN | POINTER_FLAG_INRANGE | POINTER_FLAG_INCONTACT,
        512,
    );
    inject(&down)?;
    let mut release = TouchReleaseGuard::new(screen);

    // Submit the complete frame sequence before pumping. The injected input
    // targets this same UI thread, so waiting for DOWN before injecting the
    // remaining frames would make the test wait on its own input producer.
    thread::sleep(Duration::from_millis(2));
    let moved_client = POINT {
        x: client.x + 12,
        y: client.y + 8,
    };
    let mut moved_screen = moved_client;
    if unsafe { ClientToScreen(hwnd.get() as _, &mut moved_screen) } == 0 {
        return Err(last_windows_error("ClientToScreen"));
    }
    let moved = touch_contact(
        moved_screen,
        POINTER_FLAG_UPDATE | POINTER_FLAG_INRANGE | POINTER_FLAG_INCONTACT,
        640,
    );
    inject(&moved)?;
    release.position = moved_screen;

    thread::sleep(Duration::from_millis(2));
    release.release()?;
    let mut events = collect_until(host, PlatformPointerPhase::Released)?;
    for _ in 0..16 {
        let Some(event) = host.poll_event().map_err(|error| error.to_string())? else {
            break;
        };
        events.push(event);
    }
    Ok(events)
}

fn collect_until(
    host: &mut WindowsPlatformHost,
    phase: PlatformPointerPhase,
) -> Result<Vec<PlatformHostEvent>, String> {
    let mut events = Vec::new();
    for _ in 0..128 {
        while let Some(event) = host.poll_event().map_err(|error| error.to_string())? {
            let reached = matches!(
                &event,
                PlatformHostEvent::Input {
                    event: PlatformInputEvent::Pointer { event }
                } if event.modality == NativeInputModality::Touch && event.phase == phase
            );
            events.push(event);
            if reached {
                return Ok(events);
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    Err(format!(
        "injected touch did not produce the expected {phase:?} event"
    ))
}

fn touch_contact(position: POINT, flags: u32, pressure: u32) -> POINTER_TOUCH_INFO {
    let mut contact = POINTER_TOUCH_INFO::default();
    contact.pointerInfo.pointerType = PT_TOUCH;
    contact.pointerInfo.pointerId = TOUCH_ID;
    contact.pointerInfo.pointerFlags = flags;
    contact.pointerInfo.ptPixelLocation = position;
    contact.touchMask = TOUCH_MASK_CONTACTAREA | TOUCH_MASK_PRESSURE;
    contact.rcContact = RECT {
        left: position.x - 2,
        top: position.y - 2,
        right: position.x + 2,
        bottom: position.y + 2,
    };
    contact.pressure = pressure;
    contact
}

fn inject(contact: &POINTER_TOUCH_INFO) -> Result<(), String> {
    if unsafe { InjectTouchInput(1, contact) } == 0 {
        Err(last_windows_error("InjectTouchInput"))
    } else {
        Ok(())
    }
}

struct TouchReleaseGuard {
    position: POINT,
    active: bool,
}

impl TouchReleaseGuard {
    fn new(position: POINT) -> Self {
        Self {
            position,
            active: true,
        }
    }

    fn release(&mut self) -> Result<(), String> {
        inject(&touch_contact(self.position, POINTER_FLAG_UP, 0))?;
        self.active = false;
        Ok(())
    }
}

impl Drop for TouchReleaseGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = inject(&touch_contact(self.position, POINTER_FLAG_UP, 0));
        }
    }
}

fn last_windows_error(operation: &str) -> String {
    format!("{operation} failed: {}", std::io::Error::last_os_error())
}
