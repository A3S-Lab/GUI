use core::ffi::c_void;

use windows_collections::IVector;
use windows_core::{IInspectable_Vtbl, Interface, RuntimeName, RuntimeType, HRESULT, HSTRING};

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
const SET_LABELED_BY_SLOT: usize = 23;
const SET_NAME_SLOT: usize = 26;
const GET_CONTROLLED_PEERS_SLOT: usize = 34;
const GET_DESCRIBED_BY_SLOT: usize = 65;
const GET_FLOWS_TO_SLOT: usize = 67;

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

pub(super) fn set_labeled_by(
    element: &xaml::UIElement,
    target: Option<&xaml::UIElement>,
) -> windows_core::Result<()> {
    let dependency_object: xaml::DependencyObject = element.cast()?;
    static SHARED: windows_core::imp::FactoryCache<
        AutomationProperties,
        IAutomationPropertiesStatics,
    > = windows_core::imp::FactoryCache::new();

    SHARED.call(|statics| unsafe {
        let method = statics_method(statics, SET_LABELED_BY_SLOT)?;
        type SetObjectPropertyMethod =
            unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT;
        let set_property: SetObjectPropertyMethod = core::mem::transmute(method);
        let target = target
            .map(Interface::as_raw)
            .unwrap_or(core::ptr::null_mut());
        set_property(
            Interface::as_raw(statics),
            Interface::as_raw(&dependency_object),
            target,
        )
        .ok()
    })
}

pub(super) fn replace_controlled_peers(
    element: &xaml::UIElement,
    targets: &[xaml::UIElement],
) -> windows_core::Result<()> {
    let vector = get_vector_property(element, GET_CONTROLLED_PEERS_SLOT)?;
    replace_vector(&vector, targets)
}

pub(super) fn replace_described_by(
    element: &xaml::UIElement,
    targets: &[xaml::DependencyObject],
) -> windows_core::Result<()> {
    let vector = get_vector_property(element, GET_DESCRIBED_BY_SLOT)?;
    replace_vector(&vector, targets)
}

pub(super) fn replace_flows_to(
    element: &xaml::UIElement,
    targets: &[xaml::DependencyObject],
) -> windows_core::Result<()> {
    let vector = get_vector_property(element, GET_FLOWS_TO_SLOT)?;
    replace_vector(&vector, targets)
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
        let method = statics_method(statics, method_slot)?;
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

fn get_vector_property<T>(
    element: &xaml::UIElement,
    method_slot: usize,
) -> windows_core::Result<IVector<T>>
where
    T: RuntimeType + 'static,
{
    let dependency_object: xaml::DependencyObject = element.cast()?;
    static SHARED: windows_core::imp::FactoryCache<
        AutomationProperties,
        IAutomationPropertiesStatics,
    > = windows_core::imp::FactoryCache::new();

    SHARED.call(|statics| unsafe {
        let method = statics_method(statics, method_slot)?;
        type GetVectorPropertyMethod =
            unsafe extern "system" fn(*mut c_void, *mut c_void, *mut *mut c_void) -> HRESULT;
        let get_property: GetVectorPropertyMethod = core::mem::transmute(method);
        let mut result = core::ptr::null_mut();
        get_property(
            Interface::as_raw(statics),
            Interface::as_raw(&dependency_object),
            &mut result,
        )
        .ok()?;
        windows_core::Type::from_abi(result)
    })
}

fn replace_vector<T>(vector: &IVector<T>, values: &[T]) -> windows_core::Result<()>
where
    T: RuntimeType + 'static,
    for<'a> &'a T: windows_core::Param<T>,
{
    vector.Clear()?;
    for value in values {
        vector.Append(value)?;
    }
    Ok(())
}

unsafe fn statics_method(
    statics: &IAutomationPropertiesStatics,
    method_slot: usize,
) -> windows_core::Result<*const c_void> {
    let slots = Interface::vtable(statics) as *const _ as *const *const c_void;
    let first_method_slot =
        core::mem::size_of::<IInspectable_Vtbl>() / core::mem::size_of::<usize>();
    let method = unsafe { *slots.add(first_method_slot + method_slot) };
    if method.is_null() {
        Err(windows_core::Error::from_hresult(HRESULT(
            0x8000_4005_u32 as i32,
        )))
    } else {
        Ok(method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automation_property_setter_slots_match_windows_app_sdk_metadata() {
        assert_eq!(SET_ACCELERATOR_KEY_SLOT, 2);
        assert_eq!(SET_HELP_TEXT_SLOT, 11);
        assert_eq!(SET_LABELED_BY_SLOT, 23);
        assert_eq!(SET_NAME_SLOT, 26);
        assert_eq!(GET_CONTROLLED_PEERS_SLOT, 34);
        assert_eq!(GET_DESCRIBED_BY_SLOT, 65);
        assert_eq!(GET_FLOWS_TO_SLOT, 67);
    }
}
