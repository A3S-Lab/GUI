use super::*;

pub(super) fn pump_winui_message(
    surface: &WinUiNativeSurface,
    wait: WinUiEventWait,
) -> GuiResult<bool> {
    let mut message = MSG::default();
    let received = match wait {
        WinUiEventWait::Poll => unsafe {
            PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool()
        },
        WinUiEventWait::Wait => {
            let result = unsafe { GetMessageW(&mut message, None, 0, 0) };
            if result.0 == -1 {
                return Err(GuiError::host("failed to read WinUI window message"));
            }
            result.as_bool()
        }
    };
    if !received {
        return Ok(false);
    }
    if message.message != WM_QUIT {
        let key_dispatch = surface.enqueue_key_message(&message);
        if !key_dispatch.is_some_and(|(_, prevent_default)| prevent_default) {
            unsafe {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        interaction::clear_pending_activation_contexts(
            &surface.activation_contexts,
            &surface.pending_activation_cleanup,
        );
        if let Some((node, _)) = key_dispatch {
            interaction::forget_activation_context(&surface.activation_contexts, node);
        }
    }
    Ok(true)
}

pub(super) fn winui_key_event_kind(message: u32) -> Option<NativeEventKind> {
    match message {
        WM_KEYDOWN | WM_SYSKEYDOWN => Some(NativeEventKind::KeyDown),
        WM_KEYUP | WM_SYSKEYUP => Some(NativeEventKind::KeyUp),
        _ => None,
    }
}

pub(super) fn winui_key_value(message: &MSG) -> String {
    winui_key_value_from_parts(message.wParam.0, translated_winui_key(message).as_deref())
}

fn translated_winui_key(message: &MSG) -> Option<String> {
    const DO_NOT_CHANGE_KEYBOARD_STATE: u32 = 1 << 2;

    let mut keyboard_state = [0_u8; 256];
    unsafe { GetKeyboardState(&mut keyboard_state) }.ok()?;
    let scan_code = ((message.lParam.0 as usize >> 16) & 0xff) as u32;
    let mut buffer = [0_u16; 8];
    let written = unsafe {
        ToUnicode(
            message.wParam.0 as u32,
            scan_code,
            Some(&keyboard_state),
            &mut buffer,
            DO_NOT_CHANGE_KEYBOARD_STATE,
        )
    };
    if written <= 0 {
        return None;
    }
    let value = String::from_utf16(&buffer[..written as usize]).ok()?;
    (!value.is_empty() && !value.chars().any(char::is_control)).then_some(value)
}

fn winui_key_value_from_parts(virtual_key: usize, translated: Option<&str>) -> String {
    translated
        .filter(|value| !value.is_empty() && !value.chars().any(char::is_control))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| winui_key_value_from_virtual_key(virtual_key))
}

pub(super) fn winui_key_value_from_virtual_key(virtual_key: usize) -> String {
    match virtual_key {
        0x08 => "Backspace".to_string(),
        0x09 => "Tab".to_string(),
        0x0D => "Enter".to_string(),
        0x10 => "Shift".to_string(),
        0x11 => "Control".to_string(),
        0x12 => "Alt".to_string(),
        0x1B => "Escape".to_string(),
        0x20 => " ".to_string(),
        0x21 => "PageUp".to_string(),
        0x22 => "PageDown".to_string(),
        0x23 => "End".to_string(),
        0x24 => "Home".to_string(),
        0x25 => "ArrowLeft".to_string(),
        0x26 => "ArrowUp".to_string(),
        0x27 => "ArrowRight".to_string(),
        0x28 => "ArrowDown".to_string(),
        0x2D => "Insert".to_string(),
        0x2E => "Delete".to_string(),
        0x30..=0x39 | 0x41..=0x5A => char::from_u32(virtual_key as u32)
            .map(|value| value.to_string())
            .unwrap_or_else(|| format!("VirtualKey:{virtual_key}")),
        0x5B | 0x5C => "Meta".to_string(),
        0x60..=0x69 => (virtual_key - 0x60).to_string(),
        0x70..=0x87 => format!("F{}", virtual_key - 0x6F),
        _ => format!("VirtualKey:{virtual_key}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winui_key_event_kind_maps_key_messages() {
        assert_eq!(
            winui_key_event_kind(WM_KEYDOWN),
            Some(NativeEventKind::KeyDown)
        );
        assert_eq!(winui_key_event_kind(WM_KEYUP), Some(NativeEventKind::KeyUp));
        assert_eq!(
            winui_key_event_kind(WM_SYSKEYDOWN),
            Some(NativeEventKind::KeyDown)
        );
        assert_eq!(
            winui_key_event_kind(WM_SYSKEYUP),
            Some(NativeEventKind::KeyUp)
        );
        assert_eq!(winui_key_event_kind(WM_QUIT), None);
    }

    #[test]
    fn winui_key_value_normalizes_common_virtual_keys() {
        assert_eq!(winui_key_value_from_virtual_key(0x0D), "Enter");
        assert_eq!(winui_key_value_from_virtual_key(0x09), "Tab");
        assert_eq!(winui_key_value_from_virtual_key(0x20), " ");
        assert_eq!(winui_key_value_from_virtual_key(0x08), "Backspace");
        assert_eq!(winui_key_value_from_virtual_key(0x1B), "Escape");
        assert_eq!(winui_key_value_from_virtual_key(0x24), "Home");
        assert_eq!(winui_key_value_from_virtual_key(0x23), "End");
        assert_eq!(winui_key_value_from_virtual_key(0x25), "ArrowLeft");
        assert_eq!(winui_key_value_from_virtual_key(0x26), "ArrowUp");
        assert_eq!(winui_key_value_from_virtual_key(0x27), "ArrowRight");
        assert_eq!(winui_key_value_from_virtual_key(0x28), "ArrowDown");
        assert_eq!(winui_key_value_from_virtual_key(0x41), "A");
        assert_eq!(winui_key_value_from_virtual_key(0x31), "1");
        assert_eq!(winui_key_value_from_virtual_key(0x70), "F1");
        assert_eq!(winui_key_value_from_virtual_key(0xFF), "VirtualKey:255");
        assert_eq!(winui_key_value_from_parts(0x32, Some("é")), "é");
        assert_eq!(winui_key_value_from_parts(0x41, Some("\u{1}")), "A");
    }
}
