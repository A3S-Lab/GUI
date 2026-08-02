#![cfg(all(target_os = "windows", feature = "host-windows"))]
#![allow(unsafe_code)]

use std::ffi::c_void;
use std::num::NonZeroIsize;
use std::thread;
use std::time::Duration;

use a3s_gui::geometry::{Rect, Size};
use a3s_gui::input::NativeInputModality;
use a3s_gui::platform_host::{
    PlatformAccessibilitySnapshot, PlatformHost, PlatformHostCommand, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformInputEvent, PlatformPointerEvent,
    PlatformPointerPhase, PlatformPresentationRequest, PlatformWindowCommand, PlatformWindowId,
    PlatformWindowSpec, WindowsPlatformHost,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
use windows_sys::Win32::UI::Input::Pointer::{
    POINTER_CHANGE_FIRSTBUTTON_DOWN, POINTER_CHANGE_FIRSTBUTTON_UP,
    POINTER_CHANGE_SECONDBUTTON_DOWN, POINTER_CHANGE_SECONDBUTTON_UP, POINTER_FLAG_DOWN,
    POINTER_FLAG_FIRSTBUTTON, POINTER_FLAG_INCONTACT, POINTER_FLAG_INRANGE, POINTER_FLAG_NEW,
    POINTER_FLAG_PRIMARY, POINTER_FLAG_SECONDBUTTON, POINTER_FLAG_UP, POINTER_FLAG_UPDATE,
    POINTER_INFO, POINTER_PEN_INFO, POINTER_TOUCH_INFO,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    SetForegroundWindow, SetWindowPos, WindowFromPoint, HWND_TOPMOST, PEN_FLAG_BARREL,
    PEN_MASK_PRESSURE, PEN_MASK_ROTATION, PEN_MASK_TILT_X, PEN_MASK_TILT_Y, PT_PEN, SWP_NOMOVE,
    SWP_NOSIZE,
};

const WINDOW: PlatformWindowId = PlatformWindowId::new(74);
const PEN_ID: u32 = 1;
const POINTER_FEEDBACK_NONE: i32 = 3;

#[repr(C)]
#[derive(Clone, Copy)]
union PointerTypeInfoData {
    pointer_info: POINTER_INFO,
    touch_info: POINTER_TOUCH_INFO,
    pen_info: POINTER_PEN_INFO,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PointerTypeInfo {
    pointer_type: i32,
    data: PointerTypeInfoData,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn CreateSyntheticPointerDevice(
        pointer_type: i32,
        maximum_count: u32,
        feedback_mode: i32,
    ) -> *mut c_void;
    fn InjectSyntheticPointerInput(
        device: *mut c_void,
        pointer_info: *const PointerTypeInfo,
        count: u32,
    ) -> i32;
    fn DestroySyntheticPointerDevice(device: *mut c_void);
}

#[test]
fn injected_pen_reaches_the_real_hwnd_with_pressure_motion_and_barrel_state() {
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

    let sequence = inject_pen_sequence(&mut host, hwnd, physical_size);
    host.shutdown().unwrap();
    let events = sequence.unwrap();
    let pointers = events
        .iter()
        .filter_map(|event| match event {
            PlatformHostEvent::Input {
                event: PlatformInputEvent::Pointer { event },
            } if event.modality == NativeInputModality::Pen => Some(event),
            _ => None,
        })
        .collect::<Vec<_>>();

    let pressed = pointers
        .iter()
        .find(|event| event.phase == PlatformPointerPhase::Pressed)
        .expect("injected pen press");
    assert_eq!(pressed.pressed_buttons, 1);
    assert_eq!(pressed.pressure, Some(0.375));
    assert!((pressed.position.x - f64::from(physical_size.0 / 2) / scale_factor).abs() < 1.0);
    assert!((pressed.position.y - f64::from(physical_size.1 / 2) / scale_factor).abs() < 1.0);

    let barrel_move = pointers
        .iter()
        .find(|event| event.phase == PlatformPointerPhase::Moved && event.pressed_buttons & 2 != 0)
        .unwrap_or_else(|| panic!("injected pen barrel move; received {pointers:#?}"));
    assert_eq!(barrel_move.pointer, pressed.pointer);
    assert_eq!(barrel_move.device, pressed.device);
    assert_eq!(barrel_move.pressure, Some(0.75));
    assert!(barrel_move.position.x > pressed.position.x);
    assert!(barrel_move.position.y > pressed.position.y);

    let released = pointers
        .iter()
        .find(|event| event.phase == PlatformPointerPhase::Released)
        .expect("injected pen release");
    assert_eq!(released.pointer, pressed.pointer);
    assert_eq!(released.pressed_buttons, 0);
    assert!(pointers
        .iter()
        .all(|event| event.modality == NativeInputModality::Pen));
}

fn open_window() -> PlatformHostTransaction {
    let spec = PlatformWindowSpec {
        id: WINDOW,
        title: "A3S WM_POINTER pen injection".to_string(),
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

fn inject_pen_sequence(
    host: &mut WindowsPlatformHost,
    hwnd: NonZeroIsize,
    physical_size: (u32, u32),
) -> Result<Vec<PlatformHostEvent>, String> {
    focus_test_window(host, hwnd)?;
    let device = SyntheticPenDevice::new()?;
    let client = POINT {
        x: (physical_size.0 / 2) as i32,
        y: (physical_size.1 / 2) as i32,
    };
    let screen = client_to_screen(hwnd, client)?;
    let target = unsafe { WindowFromPoint(screen) };
    if target != hwnd.get() as _ {
        return Err(format!(
            "pen point targets HWND {target:?} instead of the test HWND {:?}",
            hwnd.get()
        ));
    }

    device.inject(pen_sample(
        screen,
        POINTER_FLAG_NEW
            | POINTER_FLAG_INRANGE
            | POINTER_FLAG_INCONTACT
            | POINTER_FLAG_FIRSTBUTTON
            | POINTER_FLAG_PRIMARY
            | POINTER_FLAG_DOWN,
        POINTER_CHANGE_FIRSTBUTTON_DOWN,
        384,
        false,
    ))?;
    let mut release = PenReleaseGuard::new(&device, screen);
    let mut events = collect_until(host, "pen press", |event| {
        event.phase == PlatformPointerPhase::Pressed
    })?;

    thread::sleep(Duration::from_millis(2));
    let moved_screen = client_to_screen(
        hwnd,
        POINT {
            x: client.x + 14,
            y: client.y + 9,
        },
    )?;
    device.inject(pen_sample(
        moved_screen,
        POINTER_FLAG_INRANGE
            | POINTER_FLAG_INCONTACT
            | POINTER_FLAG_FIRSTBUTTON
            | POINTER_FLAG_SECONDBUTTON
            | POINTER_FLAG_PRIMARY
            | POINTER_FLAG_UPDATE,
        POINTER_CHANGE_SECONDBUTTON_DOWN,
        768,
        true,
    ))?;
    release.position = moved_screen;
    events.extend(collect_until(host, "pen barrel move", |event| {
        event.phase == PlatformPointerPhase::Moved
            && event.pressed_buttons & 2 != 0
            && event.pressure == Some(0.75)
    })?);

    thread::sleep(Duration::from_millis(2));
    device.inject(pen_sample(
        moved_screen,
        POINTER_FLAG_INRANGE
            | POINTER_FLAG_INCONTACT
            | POINTER_FLAG_FIRSTBUTTON
            | POINTER_FLAG_PRIMARY
            | POINTER_FLAG_UPDATE,
        POINTER_CHANGE_SECONDBUTTON_UP,
        640,
        false,
    ))?;
    events.extend(collect_until(host, "pen barrel release", |event| {
        event.phase == PlatformPointerPhase::Moved
            && event.pressed_buttons & 2 == 0
            && event.pressure == Some(0.625)
    })?);

    thread::sleep(Duration::from_millis(2));
    release.release()?;
    events.extend(collect_until(host, "pen release", |event| {
        event.phase == PlatformPointerPhase::Released
    })?);
    Ok(events)
}

fn focus_test_window(host: &mut WindowsPlatformHost, hwnd: NonZeroIsize) -> Result<(), String> {
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
    Ok(())
}

fn client_to_screen(hwnd: NonZeroIsize, client: POINT) -> Result<POINT, String> {
    let mut screen = client;
    if unsafe { ClientToScreen(hwnd.get() as _, &mut screen) } == 0 {
        Err(last_windows_error("ClientToScreen"))
    } else {
        Ok(screen)
    }
}

fn collect_until<F>(
    host: &mut WindowsPlatformHost,
    expected: &str,
    predicate: F,
) -> Result<Vec<PlatformHostEvent>, String>
where
    F: Fn(&PlatformPointerEvent) -> bool,
{
    let mut events = Vec::new();
    for _ in 0..256 {
        while let Some(event) = host.poll_event().map_err(|error| error.to_string())? {
            let reached = matches!(
                &event,
                PlatformHostEvent::Input {
                    event: PlatformInputEvent::Pointer { event }
                } if event.modality == NativeInputModality::Pen && predicate(event)
            );
            events.push(event);
            if reached {
                return Ok(events);
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    Err(format!(
        "injected pen did not produce the expected {expected}"
    ))
}

fn pen_sample(
    position: POINT,
    pointer_flags: u32,
    button_change: i32,
    pressure: u32,
    barrel: bool,
) -> PointerTypeInfo {
    let mut pen = POINTER_PEN_INFO::default();
    pen.pointerInfo.pointerType = PT_PEN;
    pen.pointerInfo.pointerId = PEN_ID;
    pen.pointerInfo.pointerFlags = pointer_flags;
    pen.pointerInfo.ptPixelLocation = position;
    pen.pointerInfo.ptPixelLocationRaw = position;
    pen.pointerInfo.historyCount = 1;
    pen.pointerInfo.ButtonChangeType = button_change;
    pen.penFlags = if barrel { PEN_FLAG_BARREL } else { 0 };
    pen.penMask = PEN_MASK_PRESSURE | PEN_MASK_ROTATION | PEN_MASK_TILT_X | PEN_MASK_TILT_Y;
    pen.pressure = pressure;
    pen.rotation = 27;
    pen.tiltX = 18;
    pen.tiltY = -12;
    PointerTypeInfo {
        pointer_type: PT_PEN,
        data: PointerTypeInfoData { pen_info: pen },
    }
}

struct SyntheticPenDevice(*mut c_void);

impl SyntheticPenDevice {
    fn new() -> Result<Self, String> {
        let device = unsafe { CreateSyntheticPointerDevice(PT_PEN, 1, POINTER_FEEDBACK_NONE) };
        if device.is_null() {
            Err(last_windows_error("CreateSyntheticPointerDevice"))
        } else {
            Ok(Self(device))
        }
    }

    fn inject(&self, sample: PointerTypeInfo) -> Result<(), String> {
        if unsafe { InjectSyntheticPointerInput(self.0, &sample, 1) } == 0 {
            Err(last_windows_error("InjectSyntheticPointerInput"))
        } else {
            Ok(())
        }
    }
}

impl Drop for SyntheticPenDevice {
    fn drop(&mut self) {
        unsafe {
            DestroySyntheticPointerDevice(self.0);
        }
    }
}

struct PenReleaseGuard<'a> {
    device: &'a SyntheticPenDevice,
    position: POINT,
    active: bool,
}

impl<'a> PenReleaseGuard<'a> {
    fn new(device: &'a SyntheticPenDevice, position: POINT) -> Self {
        Self {
            device,
            position,
            active: true,
        }
    }

    fn release(&mut self) -> Result<(), String> {
        self.device.inject(pen_sample(
            self.position,
            POINTER_FLAG_INRANGE | POINTER_FLAG_PRIMARY | POINTER_FLAG_UP,
            POINTER_CHANGE_FIRSTBUTTON_UP,
            0,
            false,
        ))?;
        self.active = false;
        Ok(())
    }
}

impl Drop for PenReleaseGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.device.inject(pen_sample(
                self.position,
                POINTER_FLAG_INRANGE | POINTER_FLAG_PRIMARY | POINTER_FLAG_UP,
                POINTER_CHANGE_FIRSTBUTTON_UP,
                0,
                false,
            ));
        }
    }
}

fn last_windows_error(operation: &str) -> String {
    format!("{operation} failed: {}", std::io::Error::last_os_error())
}
