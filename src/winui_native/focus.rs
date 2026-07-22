use core::ffi::c_void;

use windows_core::Interface;
use winui3::Microsoft::UI::Xaml as xaml;

use crate::error::{GuiError, GuiResult};

use super::map_winui;

// `winio-winui3` preserves ungenerated IUIElement methods as pointer-sized
// vtable slots. Focus is the third slot after the public StartBringIntoView
// anchor: StartBringIntoViewWithOptions, TryInvokeKeyboardAccelerator, Focus.
// The ordering is part of the fixed Microsoft.UI.Xaml.IUIElement WinRT ABI.
const FOCUS_SLOT_AFTER_START_BRING_INTO_VIEW: usize = 3;
const PROGRAMMATIC_FOCUS_STATE: i32 = 3;

type FocusMethod = unsafe extern "system" fn(
    this: *mut c_void,
    focus_state: i32,
    focused: *mut bool,
) -> windows_core::HRESULT;

pub(super) fn focus_winui_element(element: &xaml::UIElement) -> GuiResult<bool> {
    let interface = map_winui(
        "failed to inspect WinUI element focus interface",
        element.cast::<xaml::IUIElement>(),
    )?;
    let vtable = windows_core::Interface::vtable(&interface);
    let start_bring_into_view_slot =
        core::ptr::addr_of!(vtable.StartBringIntoView).cast::<*const c_void>();

    // SAFETY: `interface` was queried for the fixed IUIElement IID. The
    // generated vtable includes pointer-sized placeholders for methods whose
    // signatures it does not expose, and the WinRT ABI fixes Focus at the
    // offset documented above. The official signature is
    // `bool Focus(FocusState)`, where FocusState has an i32 ABI.
    let focus_slot =
        unsafe { *start_bring_into_view_slot.add(FOCUS_SLOT_AFTER_START_BRING_INTO_VIEW) };
    if focus_slot.is_null() {
        return Err(GuiError::host(
            "WinUI IUIElement focus method is unavailable",
        ));
    }
    // SAFETY: The slot is the IUIElement::Focus ABI entry established above.
    let focus: FocusMethod = unsafe { core::mem::transmute(focus_slot) };
    let mut focused = false;
    // SAFETY: `interface` owns a live COM reference for the duration of the
    // call, and `focused` is a valid writable WinRT Boolean out parameter.
    let result = unsafe {
        focus(
            windows_core::Interface::as_raw(&interface),
            PROGRAMMATIC_FOCUS_STATE,
            &mut focused,
        )
        .ok()
    };
    map_winui("failed to focus WinUI element", result)?;
    Ok(focused)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winui_focus_abi_constants_match_the_platform_contract() {
        assert_eq!(PROGRAMMATIC_FOCUS_STATE, 3);
        assert_eq!(FOCUS_SLOT_AFTER_START_BRING_INTO_VIEW, 3);
    }
}
