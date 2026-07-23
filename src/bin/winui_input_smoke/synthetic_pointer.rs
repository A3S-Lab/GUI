use std::thread::{self, JoinHandle};
use std::time::Duration;

use a3s_gui::NativeRole;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::UI::Controls::{
    CreateSyntheticPointerDevice, DestroySyntheticPointerDevice, HSYNTHETICPOINTERDEVICE,
    POINTER_FEEDBACK_NONE, POINTER_TYPE_INFO, POINTER_TYPE_INFO_0,
};
use windows::Win32::UI::Input::Pointer::{
    InjectSyntheticPointerInput, POINTER_CHANGE_FIRSTBUTTON_DOWN, POINTER_CHANGE_FIRSTBUTTON_UP,
    POINTER_CHANGE_NONE, POINTER_FLAGS, POINTER_FLAG_CANCELED, POINTER_FLAG_CONFIDENCE,
    POINTER_FLAG_DOWN, POINTER_FLAG_FIRSTBUTTON, POINTER_FLAG_INCONTACT, POINTER_FLAG_INRANGE,
    POINTER_FLAG_NEW, POINTER_FLAG_PRIMARY, POINTER_FLAG_UP, POINTER_FLAG_UPDATE, POINTER_INFO,
    POINTER_PEN_INFO, POINTER_TOUCH_INFO,
};
use windows::Win32::UI::WindowsAndMessaging::{
    PEN_MASK_PRESSURE, POINTER_INPUT_TYPE, PT_PEN, PT_TOUCH, TOUCH_MASK_CONTACTAREA,
    TOUCH_MASK_ORIENTATION, TOUCH_MASK_PRESSURE,
};

use super::automation::target_center;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SyntheticPointerKind {
    Pen,
    Touch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SyntheticPointerCompletion {
    Activate,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyntheticPointerPhase {
    Down,
    Update,
    Up,
    Cancel,
}

struct SyntheticPointerDevice(HSYNTHETICPOINTERDEVICE);

impl SyntheticPointerDevice {
    fn create(kind: SyntheticPointerKind) -> Result<Self, String> {
        unsafe { CreateSyntheticPointerDevice(kind.pointer_type(), 1, POINTER_FEEDBACK_NONE) }
            .map(Self)
            .map_err(|error| format!("failed to create synthetic {kind:?} device: {error}"))
    }

    fn inject(&self, info: POINTER_TYPE_INFO) -> Result<(), String> {
        unsafe { InjectSyntheticPointerInput(self.0, &[info]) }
            .map_err(|error| format!("failed to inject synthetic pointer input: {error}"))
    }
}

impl Drop for SyntheticPointerDevice {
    fn drop(&mut self) {
        unsafe { DestroySyntheticPointerDevice(self.0) };
    }
}

impl SyntheticPointerKind {
    const fn pointer_type(self) -> POINTER_INPUT_TYPE {
        match self {
            Self::Pen => PT_PEN,
            Self::Touch => PT_TOUCH,
        }
    }
}

pub(super) fn spawn_synthetic_pointer(
    hwnd: isize,
    role: NativeRole,
    expected_enabled: bool,
    kind: SyntheticPointerKind,
    completion: SyntheticPointerCompletion,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let (x, y) = target_center(hwnd, role, expected_enabled)?;

        let device = SyntheticPointerDevice::create(kind)?;
        device.inject(pointer_type_info(kind, SyntheticPointerPhase::Down, x, y))?;
        thread::sleep(Duration::from_millis(40));
        if completion == SyntheticPointerCompletion::Cancel {
            device.inject(pointer_type_info(kind, SyntheticPointerPhase::Update, x, y))?;
            thread::sleep(Duration::from_millis(300));
        }
        let terminal_phase = match completion {
            SyntheticPointerCompletion::Activate => SyntheticPointerPhase::Up,
            SyntheticPointerCompletion::Cancel => SyntheticPointerPhase::Cancel,
        };
        device.inject(pointer_type_info(kind, terminal_phase, x, y))
    })
}

fn pointer_type_info(
    kind: SyntheticPointerKind,
    phase: SyntheticPointerPhase,
    x: i32,
    y: i32,
) -> POINTER_TYPE_INFO {
    let pointer_type = kind.pointer_type();
    let location = POINT { x, y };
    let pointer_info = POINTER_INFO {
        pointerType: pointer_type,
        pointerId: 1,
        pointerFlags: pointer_flags(kind, phase),
        ptPixelLocation: location,
        ptPixelLocationRaw: location,
        historyCount: 1,
        ButtonChangeType: match phase {
            SyntheticPointerPhase::Down => POINTER_CHANGE_FIRSTBUTTON_DOWN,
            SyntheticPointerPhase::Up => POINTER_CHANGE_FIRSTBUTTON_UP,
            SyntheticPointerPhase::Update | SyntheticPointerPhase::Cancel => POINTER_CHANGE_NONE,
        },
        ..Default::default()
    };

    match kind {
        SyntheticPointerKind::Pen => POINTER_TYPE_INFO {
            r#type: pointer_type,
            Anonymous: POINTER_TYPE_INFO_0 {
                penInfo: POINTER_PEN_INFO {
                    pointerInfo: pointer_info,
                    penMask: PEN_MASK_PRESSURE,
                    pressure: matches!(
                        phase,
                        SyntheticPointerPhase::Down | SyntheticPointerPhase::Update
                    )
                    .then_some(512)
                    .unwrap_or(0),
                    ..Default::default()
                },
            },
        },
        SyntheticPointerKind::Touch => POINTER_TYPE_INFO {
            r#type: pointer_type,
            Anonymous: POINTER_TYPE_INFO_0 {
                touchInfo: POINTER_TOUCH_INFO {
                    pointerInfo: pointer_info,
                    touchMask: TOUCH_MASK_CONTACTAREA
                        | TOUCH_MASK_ORIENTATION
                        | TOUCH_MASK_PRESSURE,
                    rcContact: contact_rect(x, y),
                    rcContactRaw: contact_rect(x, y),
                    orientation: 90,
                    pressure: matches!(
                        phase,
                        SyntheticPointerPhase::Down | SyntheticPointerPhase::Update
                    )
                    .then_some(512)
                    .unwrap_or(0),
                    ..Default::default()
                },
            },
        },
    }
}

fn pointer_flags(kind: SyntheticPointerKind, phase: SyntheticPointerPhase) -> POINTER_FLAGS {
    match phase {
        SyntheticPointerPhase::Down => {
            POINTER_FLAG_NEW
                | POINTER_FLAG_INRANGE
                | POINTER_FLAG_INCONTACT
                | POINTER_FLAG_FIRSTBUTTON
                | POINTER_FLAG_PRIMARY
                | POINTER_FLAG_CONFIDENCE
                | POINTER_FLAG_DOWN
        }
        SyntheticPointerPhase::Update => {
            POINTER_FLAG_INRANGE
                | POINTER_FLAG_INCONTACT
                | POINTER_FLAG_FIRSTBUTTON
                | POINTER_FLAG_PRIMARY
                | POINTER_FLAG_CONFIDENCE
                | POINTER_FLAG_UPDATE
        }
        SyntheticPointerPhase::Up if kind == SyntheticPointerKind::Pen => {
            POINTER_FLAG_INRANGE | POINTER_FLAG_PRIMARY | POINTER_FLAG_UP
        }
        SyntheticPointerPhase::Up => POINTER_FLAG_PRIMARY | POINTER_FLAG_UP,
        SyntheticPointerPhase::Cancel => {
            POINTER_FLAG_PRIMARY | POINTER_FLAG_CANCELED | POINTER_FLAG_UPDATE
        }
    }
}

fn contact_rect(x: i32, y: i32) -> RECT {
    RECT {
        left: x - 2,
        top: y - 2,
        right: x + 2,
        bottom: y + 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pen_activation_uses_contact_down_and_normal_up() {
        let down = pointer_type_info(SyntheticPointerKind::Pen, SyntheticPointerPhase::Down, 4, 8);
        let up = pointer_type_info(SyntheticPointerKind::Pen, SyntheticPointerPhase::Up, 4, 8);

        assert_eq!(down.r#type, PT_PEN);
        let down = unsafe { down.Anonymous.penInfo };
        let up = unsafe { up.Anonymous.penInfo };
        assert_flag(down.pointerInfo.pointerFlags, POINTER_FLAG_DOWN);
        assert_flag(down.pointerInfo.pointerFlags, POINTER_FLAG_INCONTACT);
        assert_flag(up.pointerInfo.pointerFlags, POINTER_FLAG_UP);
        assert_no_flag(up.pointerInfo.pointerFlags, POINTER_FLAG_CANCELED);
        assert_eq!(down.pointerInfo.ptPixelLocation, POINT { x: 4, y: 8 });
    }

    #[test]
    fn touch_cancellation_uses_the_windows_cancelled_update_contract() {
        let cancel = pointer_type_info(
            SyntheticPointerKind::Touch,
            SyntheticPointerPhase::Cancel,
            12,
            16,
        );

        assert_eq!(cancel.r#type, PT_TOUCH);
        let cancel = unsafe { cancel.Anonymous.touchInfo };
        assert_flag(cancel.pointerInfo.pointerFlags, POINTER_FLAG_CANCELED);
        assert_flag(cancel.pointerInfo.pointerFlags, POINTER_FLAG_UPDATE);
        assert_no_flag(cancel.pointerInfo.pointerFlags, POINTER_FLAG_UP);
        assert_no_flag(cancel.pointerInfo.pointerFlags, POINTER_FLAG_INCONTACT);
        assert_eq!(cancel.pointerInfo.ButtonChangeType, POINTER_CHANGE_NONE);
        assert_eq!(
            cancel.rcContact,
            RECT {
                left: 10,
                top: 14,
                right: 14,
                bottom: 18,
            }
        );
    }

    fn assert_flag(actual: POINTER_FLAGS, expected: POINTER_FLAGS) {
        assert_ne!(actual.0 & expected.0, 0);
    }

    fn assert_no_flag(actual: POINTER_FLAGS, unexpected: POINTER_FLAGS) {
        assert_eq!(actual.0 & unexpected.0, 0);
    }
}
