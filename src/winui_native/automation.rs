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

// The vendored WinUI binding omits Microsoft.UI.Xaml.Automation. This is the
// stable IAutomationPropertiesStatics ABI slot for SetName in Windows App SDK
// metadata, counted after IInspectable's methods.
const SET_NAME_SLOT: usize = 26;

pub(super) fn set_name(element: &xaml::UIElement, value: Option<&str>) -> windows_core::Result<()> {
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
        let method = *slots.add(first_method_slot + SET_NAME_SLOT);
        if method.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(
                0x8000_4005_u32 as i32,
            )));
        }
        type SetNameMethod =
            unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT;
        let set_name: SetNameMethod = core::mem::transmute(method);
        set_name(
            Interface::as_raw(statics),
            Interface::as_raw(&dependency_object),
            core::mem::transmute_copy(&value),
        )
        .ok()
    })
}
