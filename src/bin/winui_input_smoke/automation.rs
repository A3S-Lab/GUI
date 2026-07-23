use std::ffi::c_void;
use std::mem::size_of;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use a3s_gui::{GuiError, GuiResult, NativeInputConformanceEnvironmentV1, NativeOperatingSystemV1};
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
};
use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationInvokePattern,
    TreeScope_Descendants, UIA_ButtonControlTypeId, UIA_InvokePatternId, UIA_E_ELEMENTNOTENABLED,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::{SetCursorPos, SetForegroundWindow};

pub(super) const TARGET_LABEL: &str = "A3S WinUI native input target";
const UI_AUTOMATION_TIMEOUT: Duration = Duration::from_secs(5);

struct ComApartment;

impl ComApartment {
    fn initialize() -> Result<Self, String> {
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
            .ok()
            .map_err(|error| {
                format!("failed to initialize UI Automation COM apartment: {error}")
            })?;
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

struct UiAutomationTarget {
    // COM interfaces must release before the apartment is uninitialized.
    element: IUIAutomationElement,
    _apartment: ComApartment,
}

impl UiAutomationTarget {
    fn find(hwnd: HWND, label: &str) -> Result<Self, String> {
        let apartment = ComApartment::initialize()?;
        let automation: IUIAutomation =
            unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) }.map_err(
                |error| format!("failed to create Windows UI Automation client: {error}"),
            )?;
        let root = unsafe { automation.ElementFromHandle(hwnd) }.map_err(|error| {
            format!("failed to inspect the WinUI window with UI Automation: {error}")
        })?;
        let condition = unsafe { automation.CreateTrueCondition() }
            .map_err(|error| format!("failed to create UI Automation search condition: {error}"))?;
        let started = Instant::now();

        loop {
            let elements =
                unsafe { root.FindAll(TreeScope_Descendants, &condition) }.map_err(|error| {
                    format!("failed to enumerate WinUI automation elements: {error}")
                })?;
            let length = unsafe { elements.Length() }
                .map_err(|error| format!("failed to count WinUI automation elements: {error}"))?;
            for index in 0..length {
                let element = unsafe { elements.GetElement(index) }.map_err(|error| {
                    format!("failed to read WinUI automation element {index}: {error}")
                })?;
                let name = unsafe { element.CurrentName() }
                    .map_err(|error| format!("failed to read WinUI automation name: {error}"))?;
                let control_type = unsafe { element.CurrentControlType() }.map_err(|error| {
                    format!("failed to read WinUI automation control type: {error}")
                })?;
                if name.to_string() == label && control_type == UIA_ButtonControlTypeId {
                    return Ok(Self {
                        element,
                        _apartment: apartment,
                    });
                }
            }

            if started.elapsed() >= UI_AUTOMATION_TIMEOUT {
                return Err(format!(
                    "UI Automation did not expose button {label:?} within {} seconds",
                    UI_AUTOMATION_TIMEOUT.as_secs()
                ));
            }
            thread::sleep(Duration::from_millis(25));
        }
    }

    fn center(&self) -> Result<(i32, i32), String> {
        let bounds = unsafe { self.element.CurrentBoundingRectangle() }
            .map_err(|error| format!("failed to read WinUI automation bounds: {error}"))?;
        if bounds.right <= bounds.left || bounds.bottom <= bounds.top {
            return Err(format!(
                "WinUI automation target has invalid bounds ({}, {})-({}, {})",
                bounds.left, bounds.top, bounds.right, bounds.bottom
            ));
        }
        Ok((
            bounds.left + (bounds.right - bounds.left) / 2,
            bounds.top + (bounds.bottom - bounds.top) / 2,
        ))
    }

    fn require_enabled(&self, expected: bool) -> Result<(), String> {
        let enabled = unsafe { self.element.CurrentIsEnabled() }
            .map_err(|error| format!("failed to read WinUI automation enabled state: {error}"))?
            .as_bool();
        if enabled == expected {
            Ok(())
        } else {
            Err(format!(
                "WinUI automation target enabled state was {enabled}, expected {expected}"
            ))
        }
    }

    fn invoke(&self, expected_enabled: bool) -> Result<(), String> {
        let pattern: IUIAutomationInvokePattern =
            unsafe { self.element.GetCurrentPatternAs(UIA_InvokePatternId) }
                .map_err(|error| format!("WinUI target does not expose InvokePattern: {error}"))?;
        let result = unsafe { pattern.Invoke() };
        match (expected_enabled, result) {
            (true, Ok(())) => Ok(()),
            (true, Err(error)) => Err(format!("UI Automation InvokePattern failed: {error}")),
            (false, Err(error)) if error.code().0 as u32 == UIA_E_ELEMENTNOTENABLED => Ok(()),
            (false, Ok(())) => {
                Err("disabled WinUI target unexpectedly accepted InvokePattern".to_string())
            }
            (false, Err(error)) => Err(format!(
                "disabled WinUI InvokePattern returned an unexpected error: {error}"
            )),
        }
    }
}

pub(super) fn target_center(hwnd: isize, expected_enabled: bool) -> Result<(i32, i32), String> {
    let target = UiAutomationTarget::find(hwnd_from_value(hwnd), TARGET_LABEL)?;
    target.require_enabled(expected_enabled)?;
    target.center()
}

pub(super) fn spawn_mouse_activation(
    hwnd: isize,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, expected_enabled, Duration::from_millis(30))
}

pub(super) fn spawn_keyed_rerender_activation(hwnd: isize) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, true, Duration::from_millis(200))
}

pub(super) fn spawn_mouse_cancellation(hwnd: isize) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, true, Duration::from_millis(300))
}

fn spawn_mouse_input(
    hwnd: isize,
    expected_enabled: bool,
    hold_time: Duration,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let hwnd = hwnd_from_value(hwnd);
        let target = UiAutomationTarget::find(hwnd, TARGET_LABEL)?;
        target.require_enabled(expected_enabled)?;
        let (x, y) = target.center()?;
        let _ = unsafe { SetForegroundWindow(hwnd) };
        unsafe { SetCursorPos(x, y) }
            .map_err(|error| format!("failed to position the OS cursor: {error}"))?;
        thread::sleep(Duration::from_millis(50));
        send_input(mouse_input(MOUSEEVENTF_LEFTDOWN))?;
        thread::sleep(hold_time);
        send_input(mouse_input(MOUSEEVENTF_LEFTUP))
    })
}

pub(super) fn spawn_keyboard_activation(
    hwnd: isize,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let hwnd = hwnd_from_value(hwnd);
        let target = UiAutomationTarget::find(hwnd, TARGET_LABEL)?;
        target.require_enabled(expected_enabled)?;
        let _ = unsafe { SetForegroundWindow(hwnd) };
        thread::sleep(Duration::from_millis(50));
        send_input(keyboard_input(false))?;
        thread::sleep(Duration::from_millis(30));
        send_input(keyboard_input(true))
    })
}

pub(super) fn spawn_assistive_activation(
    hwnd: isize,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let target = UiAutomationTarget::find(hwnd_from_value(hwnd), TARGET_LABEL)?;
        target.require_enabled(expected_enabled)?;
        target.invoke(expected_enabled)
    })
}

pub(super) fn windows_environment() -> GuiResult<NativeInputConformanceEnvironmentV1> {
    let mut version = OSVERSIONINFOW {
        dwOSVersionInfoSize: size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };
    unsafe { GetVersionExW(&mut version) }
        .map_err(|error| GuiError::host(format!("failed to read Windows version: {error}")))?;
    Ok(NativeInputConformanceEnvironmentV1::new(
        NativeOperatingSystemV1::Windows,
        format!(
            "{}.{}.{}",
            version.dwMajorVersion, version.dwMinorVersion, version.dwBuildNumber
        ),
        "Windows App SDK via winio-winui3 0.4.2",
        "Windows UI Automation + SendInput + synthetic pointer injection",
    ))
}

fn hwnd_from_value(value: isize) -> HWND {
    HWND(value as *mut c_void)
}

fn mouse_input(flags: windows::Win32::UI::Input::KeyboardAndMouse::MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dwFlags: flags,
                ..Default::default()
            },
        },
    }
}

fn keyboard_input(released: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_RETURN,
                dwFlags: if released {
                    KEYEVENTF_KEYUP
                } else {
                    Default::default()
                },
                ..Default::default()
            },
        },
    }
}

fn send_input(input: INPUT) -> Result<(), String> {
    let sent = unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
    if sent == 1 {
        return Ok(());
    }
    let error = unsafe { GetLastError() };
    Err(format!(
        "Windows SendInput inserted {sent} of 1 events (Win32 error {})",
        error.0
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_inputs_use_real_left_button_flags() {
        let down = mouse_input(MOUSEEVENTF_LEFTDOWN);
        let up = mouse_input(MOUSEEVENTF_LEFTUP);
        assert_eq!(down.r#type, INPUT_MOUSE);
        assert_eq!(up.r#type, INPUT_MOUSE);
        assert_eq!(unsafe { down.Anonymous.mi }.dwFlags, MOUSEEVENTF_LEFTDOWN);
        assert_eq!(unsafe { up.Anonymous.mi }.dwFlags, MOUSEEVENTF_LEFTUP);
    }

    #[test]
    fn keyboard_inputs_use_enter_down_and_up() {
        let down = keyboard_input(false);
        let up = keyboard_input(true);
        assert_eq!(down.r#type, INPUT_KEYBOARD);
        assert_eq!(up.r#type, INPUT_KEYBOARD);
        assert_eq!(unsafe { down.Anonymous.ki }.wVk, VK_RETURN);
        assert_eq!(unsafe { down.Anonymous.ki }.dwFlags, Default::default());
        assert_eq!(unsafe { up.Anonymous.ki }.dwFlags, KEYEVENTF_KEYUP);
    }
}
