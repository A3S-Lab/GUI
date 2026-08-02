#![allow(unsafe_code)]

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardLayout, GetKeyboardState, ToUnicodeEx, VK_ADD, VK_APPS, VK_BACK, VK_BROWSER_BACK,
    VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME, VK_BROWSER_REFRESH,
    VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_CAPITAL, VK_CLEAR, VK_CONTROL, VK_DECIMAL, VK_DELETE,
    VK_DIVIDE, VK_DOWN, VK_END, VK_ESCAPE, VK_EXECUTE, VK_F1, VK_F24, VK_HELP, VK_HOME, VK_INSERT,
    VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LCONTROL, VK_LEFT,
    VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP, VK_MENU, VK_MULTIPLY, VK_NEXT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD9, VK_PAUSE,
    VK_PRINT, VK_PRIOR, VK_PROCESSKEY, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT,
    VK_RWIN, VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT, VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT,
    VK_TAB, VK_UP, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
};

use crate::input::NativeKeyModifiers;
use crate::platform_host::PlatformKeyState;

const TO_UNICODE_NO_STATE_CHANGE: u32 = 4;

#[derive(Debug, Clone)]
pub(super) struct WindowsKeyTranslation {
    pub(super) identity: u32,
    pub(super) virtual_key: u16,
    pub(super) physical_key: String,
    pub(super) logical_key: String,
    pub(super) text: Option<String>,
}

pub(super) fn modifiers_after_key(
    mut modifiers: NativeKeyModifiers,
    virtual_key: u16,
    state: PlatformKeyState,
    another_modifier_is_pressed: bool,
) -> NativeKeyModifiers {
    let pressed = state == PlatformKeyState::Pressed || another_modifier_is_pressed;
    match virtual_key {
        VK_SHIFT | VK_LSHIFT | VK_RSHIFT => modifiers.shift = pressed,
        VK_CONTROL | VK_LCONTROL | VK_RCONTROL => modifiers.control = pressed,
        VK_MENU | VK_LMENU | VK_RMENU => modifiers.alt = pressed,
        VK_LWIN | VK_RWIN => modifiers.meta = pressed,
        _ => {}
    }
    modifiers
}

pub(super) fn same_modifier_group(left: u16, right: u16) -> bool {
    matches!(
        (left, right),
        (
            VK_SHIFT | VK_LSHIFT | VK_RSHIFT,
            VK_SHIFT | VK_LSHIFT | VK_RSHIFT
        ) | (
            VK_CONTROL | VK_LCONTROL | VK_RCONTROL,
            VK_CONTROL | VK_LCONTROL | VK_RCONTROL
        ) | (VK_MENU | VK_LMENU | VK_RMENU, VK_MENU | VK_LMENU | VK_RMENU)
            | (VK_LWIN | VK_RWIN, VK_LWIN | VK_RWIN)
    )
}

pub(super) fn translate_key(
    virtual_key: u16,
    lparam: isize,
    state: PlatformKeyState,
    modifiers: NativeKeyModifiers,
) -> WindowsKeyTranslation {
    let scan_code = ((lparam as u64 >> 16) & 0xff) as u8;
    let extended = (lparam as u64 & (1 << 24)) != 0;
    let identity =
        u32::from(scan_code) | (u32::from(extended) << 8) | (u32::from(virtual_key) << 16);
    let physical_key = physical_key(virtual_key, scan_code, extended);
    let unicode = unicode_key(virtual_key, scan_code, modifiers);
    let named = named_logical_key(virtual_key);
    let logical_key = named
        .map(str::to_string)
        .or_else(|| unicode.logical.clone())
        .unwrap_or_else(|| "Unidentified".to_string());
    let text = (state == PlatformKeyState::Pressed)
        .then_some(unicode.text)
        .flatten();
    WindowsKeyTranslation {
        identity,
        virtual_key,
        physical_key,
        logical_key,
        text,
    }
}

struct UnicodeKey {
    logical: Option<String>,
    text: Option<String>,
}

fn unicode_key(virtual_key: u16, scan_code: u8, modifiers: NativeKeyModifiers) -> UnicodeKey {
    let mut keyboard_state = [0_u8; 256];
    // SAFETY: keyboard_state is a complete writable 256-byte key-state table.
    unsafe {
        GetKeyboardState(keyboard_state.as_mut_ptr());
    }
    apply_modifiers(&mut keyboard_state, modifiers);
    keyboard_state[usize::from(virtual_key)] |= 0x80;
    if modifiers.control && !modifiers.alt {
        for key in [VK_CONTROL, VK_LCONTROL, VK_RCONTROL] {
            keyboard_state[usize::from(key)] &= 0x7f;
        }
    }
    if modifiers.meta {
        for key in [VK_LWIN, VK_RWIN] {
            keyboard_state[usize::from(key)] &= 0x7f;
        }
    }
    if modifiers.alt && !modifiers.control {
        for key in [VK_MENU, VK_LMENU, VK_RMENU] {
            keyboard_state[usize::from(key)] &= 0x7f;
        }
    }
    let mut buffer = [0_u16; 8];
    // SAFETY: all pointers refer to fixed initialized buffers, the scan code
    // comes from the current key message, and layout is owned by Windows.
    let count = unsafe {
        ToUnicodeEx(
            u32::from(virtual_key),
            u32::from(scan_code),
            keyboard_state.as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            TO_UNICODE_NO_STATE_CHANGE,
            GetKeyboardLayout(0),
        )
    };
    if count < 0 {
        return UnicodeKey {
            logical: Some("Dead".to_string()),
            text: None,
        };
    }
    if count == 0 {
        return UnicodeKey {
            logical: None,
            text: None,
        };
    }
    let value = String::from_utf16_lossy(&buffer[..(count as usize).min(buffer.len())]);
    let printable = (!value.is_empty() && value.chars().all(|character| !character.is_control()))
        .then_some(value);
    let inserts_text = !modifiers.meta
        && ((!modifiers.control && !modifiers.alt) || (modifiers.control && modifiers.alt));
    UnicodeKey {
        logical: printable.clone(),
        text: printable.filter(|_| inserts_text),
    }
}

fn apply_modifiers(state: &mut [u8; 256], modifiers: NativeKeyModifiers) {
    for (pressed, keys) in [
        (modifiers.shift, &[VK_SHIFT, VK_LSHIFT, VK_RSHIFT][..]),
        (
            modifiers.control,
            &[VK_CONTROL, VK_LCONTROL, VK_RCONTROL][..],
        ),
        (modifiers.alt, &[VK_MENU, VK_LMENU, VK_RMENU][..]),
        (modifiers.meta, &[VK_LWIN, VK_RWIN][..]),
    ] {
        for key in keys {
            if pressed {
                state[usize::from(*key)] |= 0x80;
            } else {
                state[usize::from(*key)] &= 0x7f;
            }
        }
    }
}

fn named_logical_key(virtual_key: u16) -> Option<&'static str> {
    let value = match virtual_key {
        VK_BACK => "Backspace",
        VK_TAB => "Tab",
        VK_CLEAR => "Clear",
        VK_RETURN => "Enter",
        VK_SHIFT | VK_LSHIFT | VK_RSHIFT => "Shift",
        VK_CONTROL | VK_LCONTROL | VK_RCONTROL => "Control",
        VK_MENU | VK_LMENU | VK_RMENU => "Alt",
        VK_PAUSE => "Pause",
        VK_CAPITAL => "CapsLock",
        VK_ESCAPE => "Escape",
        VK_SPACE => " ",
        VK_PRIOR => "PageUp",
        VK_NEXT => "PageDown",
        VK_END => "End",
        VK_HOME => "Home",
        VK_LEFT => "ArrowLeft",
        VK_UP => "ArrowUp",
        VK_RIGHT => "ArrowRight",
        VK_DOWN => "ArrowDown",
        VK_SELECT => "Select",
        VK_PRINT => "Print",
        VK_EXECUTE => "Execute",
        VK_SNAPSHOT => "PrintScreen",
        VK_INSERT => "Insert",
        VK_DELETE => "Delete",
        VK_HELP => "Help",
        VK_LWIN | VK_RWIN => "Meta",
        VK_APPS => "ContextMenu",
        VK_MULTIPLY => "*",
        VK_ADD => "+",
        VK_SEPARATOR => "Separator",
        VK_SUBTRACT => "-",
        VK_DECIMAL => ".",
        VK_DIVIDE => "/",
        VK_NUMLOCK => "NumLock",
        VK_SCROLL => "ScrollLock",
        VK_BROWSER_BACK => "BrowserBack",
        VK_BROWSER_FORWARD => "BrowserForward",
        VK_BROWSER_REFRESH => "BrowserRefresh",
        VK_BROWSER_STOP => "BrowserStop",
        VK_BROWSER_SEARCH => "BrowserSearch",
        VK_BROWSER_FAVORITES => "BrowserFavorites",
        VK_BROWSER_HOME => "BrowserHome",
        VK_VOLUME_MUTE => "AudioVolumeMute",
        VK_VOLUME_DOWN => "AudioVolumeDown",
        VK_VOLUME_UP => "AudioVolumeUp",
        VK_MEDIA_NEXT_TRACK => "MediaTrackNext",
        VK_MEDIA_PREV_TRACK => "MediaTrackPrevious",
        VK_MEDIA_STOP => "MediaStop",
        VK_MEDIA_PLAY_PAUSE => "MediaPlayPause",
        VK_LAUNCH_MAIL => "LaunchMail",
        VK_LAUNCH_MEDIA_SELECT => "LaunchMediaPlayer",
        VK_LAUNCH_APP1 => "LaunchApplication1",
        VK_LAUNCH_APP2 => "LaunchApplication2",
        VK_PROCESSKEY => "Process",
        value if (VK_F1..=VK_F24).contains(&value) => {
            return Some(function_key_name(value));
        }
        _ => return None,
    };
    Some(value)
}

fn function_key_name(virtual_key: u16) -> &'static str {
    const NAMES: [&str; 24] = [
        "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "F13", "F14",
        "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24",
    ];
    NAMES[usize::from(virtual_key - VK_F1)]
}

fn physical_key(virtual_key: u16, scan_code: u8, extended: bool) -> String {
    if virtual_key == VK_PAUSE {
        return "Pause".to_string();
    }
    if virtual_key == VK_SNAPSHOT {
        return "PrintScreen".to_string();
    }
    let known = match (scan_code, extended) {
        (0x01, false) => "Escape",
        (0x02, false) => "Digit1",
        (0x03, false) => "Digit2",
        (0x04, false) => "Digit3",
        (0x05, false) => "Digit4",
        (0x06, false) => "Digit5",
        (0x07, false) => "Digit6",
        (0x08, false) => "Digit7",
        (0x09, false) => "Digit8",
        (0x0a, false) => "Digit9",
        (0x0b, false) => "Digit0",
        (0x0c, false) => "Minus",
        (0x0d, false) => "Equal",
        (0x0e, false) => "Backspace",
        (0x0f, false) => "Tab",
        (0x10, false) => "KeyQ",
        (0x11, false) => "KeyW",
        (0x12, false) => "KeyE",
        (0x13, false) => "KeyR",
        (0x14, false) => "KeyT",
        (0x15, false) => "KeyY",
        (0x16, false) => "KeyU",
        (0x17, false) => "KeyI",
        (0x18, false) => "KeyO",
        (0x19, false) => "KeyP",
        (0x1a, false) => "BracketLeft",
        (0x1b, false) => "BracketRight",
        (0x1c, false) => "Enter",
        (0x1c, true) => "NumpadEnter",
        (0x1d, false) => "ControlLeft",
        (0x1d, true) => "ControlRight",
        (0x1e, false) => "KeyA",
        (0x1f, false) => "KeyS",
        (0x20, false) => "KeyD",
        (0x21, false) => "KeyF",
        (0x22, false) => "KeyG",
        (0x23, false) => "KeyH",
        (0x24, false) => "KeyJ",
        (0x25, false) => "KeyK",
        (0x26, false) => "KeyL",
        (0x27, false) => "Semicolon",
        (0x28, false) => "Quote",
        (0x29, false) => "Backquote",
        (0x2a, false) => "ShiftLeft",
        (0x2b, false) => "Backslash",
        (0x2c, false) => "KeyZ",
        (0x2d, false) => "KeyX",
        (0x2e, false) => "KeyC",
        (0x2f, false) => "KeyV",
        (0x30, false) => "KeyB",
        (0x31, false) => "KeyN",
        (0x32, false) => "KeyM",
        (0x33, false) => "Comma",
        (0x34, false) => "Period",
        (0x35, false) => "Slash",
        (0x35, true) => "NumpadDivide",
        (0x36, false) => "ShiftRight",
        (0x37, false) => "NumpadMultiply",
        (0x38, false) => "AltLeft",
        (0x38, true) => "AltRight",
        (0x39, false) => "Space",
        (0x3a, false) => "CapsLock",
        (0x3b..=0x44, false) => return format!("F{}", scan_code - 0x3a),
        (0x45, false) => "NumLock",
        (0x46, false) => "ScrollLock",
        (0x47, false) => "Numpad7",
        (0x47, true) => "Home",
        (0x48, false) => "Numpad8",
        (0x48, true) => "ArrowUp",
        (0x49, false) => "Numpad9",
        (0x49, true) => "PageUp",
        (0x4a, false) => "NumpadSubtract",
        (0x4b, false) => "Numpad4",
        (0x4b, true) => "ArrowLeft",
        (0x4c, false) => "Numpad5",
        (0x4d, false) => "Numpad6",
        (0x4d, true) => "ArrowRight",
        (0x4e, false) => "NumpadAdd",
        (0x4f, false) => "Numpad1",
        (0x4f, true) => "End",
        (0x50, false) => "Numpad2",
        (0x50, true) => "ArrowDown",
        (0x51, false) => "Numpad3",
        (0x51, true) => "PageDown",
        (0x52, false) => "Numpad0",
        (0x52, true) => "Insert",
        (0x53, false) => "NumpadDecimal",
        (0x53, true) => "Delete",
        (0x56, false) => "IntlBackslash",
        (0x57, false) => "F11",
        (0x58, false) => "F12",
        (0x5b, true) => "MetaLeft",
        (0x5c, true) => "MetaRight",
        (0x5d, true) => "ContextMenu",
        _ => {
            if (VK_NUMPAD0..=VK_NUMPAD9).contains(&virtual_key) {
                return format!("Numpad{}", virtual_key - VK_NUMPAD0);
            }
            return if extended {
                format!("E0{:02X}", scan_code)
            } else {
                format!("ScanCode{:02X}", scan_code)
            };
        }
    };
    known.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_codes_map_to_stable_physical_keys() {
        assert_eq!(physical_key(0x41, 0x1e, false), "KeyA");
        assert_eq!(physical_key(VK_RETURN, 0x1c, true), "NumpadEnter");
        assert_eq!(physical_key(VK_LEFT, 0x4b, true), "ArrowLeft");
        assert_eq!(physical_key(0xff, 0x7f, true), "E07F");
    }

    #[test]
    fn modifier_keys_update_the_portable_snapshot() {
        let modifiers = modifiers_after_key(
            NativeKeyModifiers::new(),
            VK_SHIFT,
            PlatformKeyState::Pressed,
            false,
        );
        assert!(modifiers.shift);
        assert!(!modifiers.control);
        assert!(modifiers_after_key(modifiers, VK_SHIFT, PlatformKeyState::Released, true).shift);
        assert!(!modifiers_after_key(modifiers, VK_SHIFT, PlatformKeyState::Released, false).shift);
    }

    #[test]
    fn shortcut_modifiers_never_produce_insertable_text() {
        let lparam = (0x1e_u32 << 16) as isize;
        for modifiers in [
            NativeKeyModifiers::new().control(true),
            NativeKeyModifiers::new().alt(true),
            NativeKeyModifiers::new().meta(true),
        ] {
            let translated = translate_key(0x41, lparam, PlatformKeyState::Pressed, modifiers);
            assert!(translated.text.is_none());
        }
    }
}
