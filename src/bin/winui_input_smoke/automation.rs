use std::ffi::c_void;
use std::mem::size_of;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use a3s_gui::{
    GuiError, GuiResult, NativeInputConformanceEnvironmentV1, NativeOperatingSystemV1, NativeRole,
};
use windows::Win32::Foundation::{GetLastError, HWND, POINT};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
};
use windows::Win32::System::SystemInformation::{GetVersionExW, OSVERSIONINFOW};
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationInvokePattern,
    IUIAutomationSelectionItemPattern, TreeScope_Descendants, UIA_ButtonControlTypeId,
    UIA_InvokePatternId, UIA_ListItemControlTypeId, UIA_SelectionItemPatternId, UIA_CONTROLTYPE_ID,
    UIA_E_ELEMENTNOTENABLED,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEINPUT, VK_RETURN,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, GetWindowThreadProcessId, SetCursorPos, SetForegroundWindow, SetWindowPos,
    WindowFromPoint, SM_CXSCREEN, SWP_NOSIZE, SWP_NOZORDER,
};

pub(super) const TARGET_LABEL: &str = "A3S WinUI native input target";
const UI_AUTOMATION_TIMEOUT: Duration = Duration::from_secs(5);
const SMOKE_WINDOW_OUTER_WIDTH: i32 = 420;
const SMOKE_WINDOW_MARGIN: i32 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiAutomationAction {
    Invoke,
    Select,
}

impl UiAutomationAction {
    fn for_role(role: NativeRole) -> Result<Self, String> {
        match role {
            NativeRole::Button
            | NativeRole::DisclosureSummary
            | NativeRole::Link
            | NativeRole::ImageMapArea
            | NativeRole::MenuItem => Ok(Self::Invoke),
            NativeRole::ListBoxItem | NativeRole::TreeItem => Ok(Self::Select),
            _ => Err(format!(
                "WinUI input smoke has no UI Automation action for {role:?}"
            )),
        }
    }

    const fn control_type(self) -> UIA_CONTROLTYPE_ID {
        match self {
            Self::Invoke => UIA_ButtonControlTypeId,
            Self::Select => UIA_ListItemControlTypeId,
        }
    }

    const fn control_name(self) -> &'static str {
        match self {
            Self::Invoke => "button",
            Self::Select => "list item",
        }
    }
}

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
    action: UiAutomationAction,
    _apartment: ComApartment,
}

impl UiAutomationTarget {
    fn find(hwnd: HWND, label: &str, role: NativeRole) -> Result<Self, String> {
        let action = UiAutomationAction::for_role(role)?;
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
                if name.to_string() == label && control_type == action.control_type() {
                    return Ok(Self {
                        element,
                        action,
                        _apartment: apartment,
                    });
                }
            }

            if started.elapsed() >= UI_AUTOMATION_TIMEOUT {
                return Err(format!(
                    "UI Automation did not expose {} {label:?} for {role:?} within {} seconds",
                    action.control_name(),
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

    fn verified_center(&self) -> Result<(i32, i32), String> {
        let center = self.center()?;
        let hit = unsafe {
            WindowFromPoint(POINT {
                x: center.0,
                y: center.1,
            })
        };
        let mut hit_process = 0;
        unsafe { GetWindowThreadProcessId(hit, Some(&mut hit_process)) };
        if hit_process != std::process::id() {
            return Err(format!(
                "WinUI automation target center ({}, {}) is covered by window {:?} from process {hit_process}",
                center.0, center.1, hit
            ));
        }
        Ok(center)
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

    fn activate(&self, expected_enabled: bool) -> Result<(), String> {
        let (pattern, result) = match self.action {
            UiAutomationAction::Invoke => {
                let pattern: IUIAutomationInvokePattern =
                    unsafe { self.element.GetCurrentPatternAs(UIA_InvokePatternId) }.map_err(
                        |error| format!("WinUI target does not expose InvokePattern: {error}"),
                    )?;
                ("InvokePattern", unsafe { pattern.Invoke() })
            }
            UiAutomationAction::Select => {
                let pattern: IUIAutomationSelectionItemPattern =
                    unsafe { self.element.GetCurrentPatternAs(UIA_SelectionItemPatternId) }
                        .map_err(|error| {
                            format!("WinUI target does not expose SelectionItemPattern: {error}")
                        })?;
                if expected_enabled
                    && unsafe { pattern.CurrentIsSelected() }
                        .map_err(|error| {
                            format!(
                                "failed to read WinUI SelectionItemPattern selected state: {error}"
                            )
                        })?
                        .as_bool()
                {
                    unsafe { pattern.RemoveFromSelection() }.map_err(|error| {
                        format!("failed to reset WinUI SelectionItemPattern state: {error}")
                    })?;
                }
                ("SelectionItemPattern", unsafe { pattern.Select() })
            }
        };
        match (expected_enabled, result) {
            (true, Ok(())) => Ok(()),
            (true, Err(error)) => Err(format!("UI Automation {pattern} failed: {error}")),
            (false, Err(error)) if error.code().0 as u32 == UIA_E_ELEMENTNOTENABLED => Ok(()),
            (false, Ok(())) => Err(format!(
                "disabled WinUI target unexpectedly accepted {pattern}"
            )),
            (false, Err(error)) => Err(format!(
                "disabled WinUI {pattern} returned an unexpected error: {error}"
            )),
        }
    }
}

pub(super) fn position_smoke_window(hwnd: isize) -> GuiResult<()> {
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let x = (screen_width - SMOKE_WINDOW_OUTER_WIDTH - SMOKE_WINDOW_MARGIN).max(0);
    unsafe {
        SetWindowPos(
            hwnd_from_value(hwnd),
            None,
            x,
            SMOKE_WINDOW_MARGIN,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER,
        )
    }
    .map_err(|error| GuiError::host(format!("failed to position WinUI smoke window: {error}")))
}

pub(super) fn target_center(
    hwnd: isize,
    role: NativeRole,
    expected_enabled: bool,
) -> Result<(i32, i32), String> {
    let hwnd = hwnd_from_value(hwnd);
    let _ = unsafe { SetForegroundWindow(hwnd) };
    thread::sleep(Duration::from_millis(50));
    let target = UiAutomationTarget::find(hwnd, TARGET_LABEL, role)?;
    target.require_enabled(expected_enabled)?;
    target.verified_center()
}

pub(super) fn spawn_mouse_activation(
    hwnd: isize,
    role: NativeRole,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, role, expected_enabled, Duration::from_millis(30))
}

pub(super) fn spawn_keyed_rerender_activation(
    hwnd: isize,
    role: NativeRole,
) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, role, true, Duration::from_millis(200))
}

pub(super) fn spawn_mouse_cancellation(
    hwnd: isize,
    role: NativeRole,
) -> JoinHandle<Result<(), String>> {
    spawn_mouse_input(hwnd, role, true, Duration::from_millis(300))
}

fn spawn_mouse_input(
    hwnd: isize,
    role: NativeRole,
    expected_enabled: bool,
    hold_time: Duration,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let hwnd = hwnd_from_value(hwnd);
        let _ = unsafe { SetForegroundWindow(hwnd) };
        thread::sleep(Duration::from_millis(50));
        let target = UiAutomationTarget::find(hwnd, TARGET_LABEL, role)?;
        target.require_enabled(expected_enabled)?;
        let (x, y) = target.verified_center()?;
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
    role: NativeRole,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let hwnd = hwnd_from_value(hwnd);
        let _ = unsafe { SetForegroundWindow(hwnd) };
        thread::sleep(Duration::from_millis(50));
        let target = UiAutomationTarget::find(hwnd, TARGET_LABEL, role)?;
        target.require_enabled(expected_enabled)?;
        send_input(keyboard_input(false))?;
        thread::sleep(Duration::from_millis(30));
        send_input(keyboard_input(true))
    })
}

pub(super) fn spawn_assistive_activation(
    hwnd: isize,
    role: NativeRole,
    expected_enabled: bool,
) -> JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let target = UiAutomationTarget::find(hwnd_from_value(hwnd), TARGET_LABEL, role)?;
        target.require_enabled(expected_enabled)?;
        target.activate(expected_enabled)
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
        "Windows UI Automation Invoke/SelectionItem + SendInput + synthetic pointer injection",
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

    #[test]
    fn automation_actions_follow_native_control_semantics() {
        assert_eq!(
            UiAutomationAction::for_role(NativeRole::Button).unwrap(),
            UiAutomationAction::Invoke
        );
        assert_eq!(
            UiAutomationAction::for_role(NativeRole::ListBoxItem).unwrap(),
            UiAutomationAction::Select
        );
        assert_eq!(
            UiAutomationAction::for_role(NativeRole::TreeItem).unwrap(),
            UiAutomationAction::Select
        );
        assert_eq!(
            UiAutomationAction::Select.control_type(),
            UIA_ListItemControlTypeId
        );
    }
}
