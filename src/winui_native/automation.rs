use core::ffi::c_void;

use windows_core::{IInspectable_Vtbl, Interface, RuntimeName, HRESULT, HSTRING};

use super::xaml;

struct AutomationProperties;

impl RuntimeName for AutomationProperties {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
}

#[repr(C)]
pub struct IAutomationPropertiesStaticsVtable {
    base__: IInspectable_Vtbl,
}

windows_core::imp::define_interface!(
    IAutomationPropertiesStatics,
    IAutomationPropertiesStaticsVtable,
    0xb1e3e0f3_112f_5966_87dc_7862d4ad50e5
);

// The vendored WinUI binding omits Microsoft.UI.Xaml.Automation. These are the
// stable IAutomationPropertiesStatics ABI slots in Windows App SDK metadata,
// counted after IInspectable's methods.
const SET_ACCELERATOR_KEY_SLOT: usize = 2;
const SET_HELP_TEXT_SLOT: usize = 11;
const SET_NAME_SLOT: usize = 26;

pub(super) fn set_name(element: &xaml::UIElement, value: Option<&str>) -> windows_core::Result<()> {
    set_string_property(element, value, SET_NAME_SLOT)
}

pub(super) fn set_help_text(
    element: &xaml::UIElement,
    value: Option<&str>,
) -> windows_core::Result<()> {
    set_string_property(element, value, SET_HELP_TEXT_SLOT)
}

pub(super) fn set_accelerator_key(
    element: &xaml::UIElement,
    value: Option<&str>,
) -> windows_core::Result<()> {
    set_string_property(element, value, SET_ACCELERATOR_KEY_SLOT)
}

fn set_string_property(
    element: &xaml::UIElement,
    value: Option<&str>,
    method_slot: usize,
) -> windows_core::Result<()> {
    let dependency_object: xaml::DependencyObject = element.cast()?;
    let value = HSTRING::from(value.unwrap_or(""));
    static SHARED: windows_core::imp::FactoryCache<
        AutomationProperties,
        IAutomationPropertiesStatics,
    > = windows_core::imp::FactoryCache::new();

    SHARED.call(|statics| unsafe {
        let slots = Interface::vtable(statics) as *const _ as *const *const c_void;
        let first_method_slot =
            core::mem::size_of::<IInspectable_Vtbl>() / core::mem::size_of::<usize>();
        let method = *slots.add(first_method_slot + method_slot);
        if method.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(
                0x8000_4005_u32 as i32,
            )));
        }
        type SetStringPropertyMethod =
            unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT;
        let set_property: SetStringPropertyMethod = core::mem::transmute(method);
        set_property(
            Interface::as_raw(statics),
            Interface::as_raw(&dependency_object),
            core::mem::transmute_copy(&value),
        )
        .ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_property_setter_slots_match_windows_app_sdk_metadata() {
        assert_eq!(SET_ACCELERATOR_KEY_SLOT, 2);
        assert_eq!(SET_HELP_TEXT_SLOT, 11);
        assert_eq!(SET_NAME_SLOT, 26);
    }
}
