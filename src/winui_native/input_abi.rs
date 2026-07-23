use core::ffi::c_void;
use core::ptr;

use windows_core::{IInspectable, IUnknown, Interface, GUID, HRESULT};
use winui3::Microsoft::UI::Xaml as xaml;
use winui3::Microsoft::UI::Xaml::Input::PointerEventHandler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WinUiPointerRoutedEvent {
    Pressed,
    Moved,
    Released,
    WheelChanged,
    CaptureLost,
    Canceled,
}

impl WinUiPointerRoutedEvent {
    const fn statics_offset(self) -> usize {
        match self {
            Self::Pressed => 3,
            Self::Moved => 4,
            Self::Released => 5,
            Self::WheelChanged => 6,
            Self::CaptureLost => 7,
            Self::Canceled => 8,
        }
    }
}

pub(super) fn add_handled_pointer_event_handler(
    element: &xaml::UIElement,
    event: WinUiPointerRoutedEvent,
    handler: &PointerEventHandler,
) -> windows_core::Result<()> {
    let routed_event = ui_element_routed_event(event.statics_offset())?;
    let handler = WinUiInspectablePointerHandler::new(handler.clone());
    let interface = element.cast::<xaml::IUIElement>()?;
    let vtable = Interface::vtable(&interface);
    let release_pointer_captures_slot =
        ptr::addr_of!(vtable.ReleasePointerCaptures).cast::<*const c_void>();
    // IUIElement::AddHandler immediately follows ReleasePointerCaptures in the
    // fixed WinRT ABI. The published binding leaves this method ungenerated.
    let method = unsafe { *release_pointer_captures_slot.add(1) };
    if method.is_null() {
        return Err(windows_core::Error::from_hresult(HRESULT(
            0x8000_4005_u32 as i32,
        )));
    }
    type AddHandlerMethod =
        unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void, bool) -> HRESULT;
    let add_handler: AddHandlerMethod = unsafe { core::mem::transmute(method) };
    unsafe {
        add_handler(
            Interface::as_raw(&interface),
            Interface::as_raw(&routed_event),
            Interface::as_raw(&handler),
            true,
        )
        .ok()
    }
}

#[repr(transparent)]
#[derive(Clone)]
struct WinUiInspectablePointerHandler(IInspectable);

#[repr(C)]
struct WinUiInspectablePointerHandlerBox {
    identity: *const windows_core::IInspectable_Vtbl,
    pointer_handler: *const WinUiPointerEventHandlerVtable,
    pointer_handler_reference: *const WinUiPointerEventHandlerReferenceVtable,
    handler: PointerEventHandler,
    count: windows_core::imp::RefCount,
}

#[repr(C)]
struct WinUiPointerEventHandlerVtable {
    base__: windows_core::IUnknown_Vtbl,
    invoke: unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT,
}

#[repr(C)]
struct WinUiPointerEventHandlerReferenceVtable {
    base__: windows_core::IInspectable_Vtbl,
    value: unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}

const POINTER_EVENT_HANDLER_REFERENCE_IID: GUID =
    GUID::from_u128(0xa136a7c3_b398_5438_95ce_2b47b9052db6);

unsafe impl Interface for WinUiInspectablePointerHandler {
    type Vtable = windows_core::IInspectable_Vtbl;
    const IID: GUID = IInspectable::IID;
}

impl WinUiInspectablePointerHandler {
    fn new(handler: PointerEventHandler) -> Self {
        let boxed = Box::new(WinUiInspectablePointerHandlerBox {
            identity: &WinUiInspectablePointerHandlerBox::IDENTITY_VTABLE,
            pointer_handler: &WinUiInspectablePointerHandlerBox::POINTER_VTABLE,
            pointer_handler_reference: &WinUiInspectablePointerHandlerBox::POINTER_REFERENCE_VTABLE,
            handler,
            count: windows_core::imp::RefCount::new(1),
        });
        unsafe { Self::from_raw(Box::into_raw(boxed).cast()) }
    }
}

impl WinUiInspectablePointerHandlerBox {
    const IDENTITY_VTABLE: windows_core::IInspectable_Vtbl = windows_core::IInspectable_Vtbl {
        base: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::identity_query_interface,
            AddRef: Self::identity_add_ref,
            Release: Self::identity_release,
        },
        GetIids: Self::get_iids,
        GetRuntimeClassName: Self::get_runtime_class_name,
        GetTrustLevel: Self::get_trust_level,
    };
    const POINTER_VTABLE: WinUiPointerEventHandlerVtable = WinUiPointerEventHandlerVtable {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::pointer_query_interface,
            AddRef: Self::pointer_add_ref,
            Release: Self::pointer_release,
        },
        invoke: Self::pointer_invoke,
    };
    const POINTER_REFERENCE_VTABLE: WinUiPointerEventHandlerReferenceVtable =
        WinUiPointerEventHandlerReferenceVtable {
            base__: windows_core::IInspectable_Vtbl {
                base: windows_core::IUnknown_Vtbl {
                    QueryInterface: Self::reference_query_interface,
                    AddRef: Self::reference_add_ref,
                    Release: Self::reference_release,
                },
                GetIids: Self::get_iids,
                GetRuntimeClassName: Self::get_runtime_class_name,
                GetTrustLevel: Self::get_trust_level,
            },
            value: Self::reference_value,
        };

    unsafe extern "system" fn identity_query_interface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe { Self::query_interface(this.cast(), iid, interface) }
    }

    unsafe extern "system" fn pointer_query_interface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        let owner = unsafe { this.cast::<*const c_void>().sub(1) }.cast();
        unsafe { Self::query_interface(owner, iid, interface) }
    }

    unsafe extern "system" fn reference_query_interface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        let owner = unsafe { this.cast::<*const c_void>().sub(2) }.cast();
        unsafe { Self::query_interface(owner, iid, interface) }
    }

    unsafe fn query_interface(
        this: *mut Self,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        if iid.is_null() || interface.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        unsafe {
            *interface = if *iid == IUnknown::IID
                || *iid == IInspectable::IID
                || *iid == windows_core::imp::IAgileObject::IID
            {
                ptr::addr_of_mut!((*this).identity).cast()
            } else if *iid == PointerEventHandler::IID {
                ptr::addr_of_mut!((*this).pointer_handler).cast()
            } else if *iid == POINTER_EVENT_HANDLER_REFERENCE_IID {
                ptr::addr_of_mut!((*this).pointer_handler_reference).cast()
            } else if *iid == windows_core::imp::IMarshal::IID {
                (*this).count.add_ref();
                let identity: IUnknown =
                    core::mem::transmute(ptr::addr_of_mut!((*this).identity).cast::<c_void>());
                return windows_core::imp::marshaler(identity, interface);
            } else {
                ptr::null_mut()
            };
            if (*interface).is_null() {
                HRESULT(0x8000_4002_u32 as i32)
            } else {
                (*this).count.add_ref();
                HRESULT(0)
            }
        }
    }

    unsafe extern "system" fn identity_add_ref(this: *mut c_void) -> u32 {
        unsafe { (*this.cast::<Self>()).count.add_ref() }
    }

    unsafe extern "system" fn pointer_add_ref(this: *mut c_void) -> u32 {
        let owner = unsafe { this.cast::<*const c_void>().sub(1) }.cast::<Self>();
        unsafe { (*owner).count.add_ref() }
    }

    unsafe extern "system" fn reference_add_ref(this: *mut c_void) -> u32 {
        let owner = unsafe { this.cast::<*const c_void>().sub(2) }.cast::<Self>();
        unsafe { (*owner).count.add_ref() }
    }

    unsafe extern "system" fn identity_release(this: *mut c_void) -> u32 {
        unsafe { Self::release(this.cast()) }
    }

    unsafe extern "system" fn pointer_release(this: *mut c_void) -> u32 {
        let owner = unsafe { this.cast::<*const c_void>().sub(1) }.cast::<Self>();
        unsafe { Self::release(owner) }
    }

    unsafe extern "system" fn reference_release(this: *mut c_void) -> u32 {
        let owner = unsafe { this.cast::<*const c_void>().sub(2) }.cast::<Self>();
        unsafe { Self::release(owner) }
    }

    unsafe fn release(this: *mut Self) -> u32 {
        let remaining = unsafe { (*this).count.release() };
        if remaining == 0 {
            unsafe { drop(Box::from_raw(this)) };
        }
        remaining
    }

    unsafe extern "system" fn get_iids(
        _this: *mut c_void,
        count: *mut u32,
        values: *mut *mut GUID,
    ) -> HRESULT {
        if count.is_null() || values.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        unsafe {
            *count = 0;
            *values = ptr::null_mut();
        }
        HRESULT(0)
    }

    unsafe extern "system" fn get_runtime_class_name(
        _this: *mut c_void,
        value: *mut *mut c_void,
    ) -> HRESULT {
        if value.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        unsafe { *value = ptr::null_mut() };
        HRESULT(0)
    }

    unsafe extern "system" fn get_trust_level(_this: *mut c_void, value: *mut i32) -> HRESULT {
        if value.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        unsafe { *value = 0 };
        HRESULT(0)
    }

    unsafe extern "system" fn pointer_invoke(
        this: *mut c_void,
        sender: *mut c_void,
        args: *mut c_void,
    ) -> HRESULT {
        let owner = unsafe { this.cast::<*const c_void>().sub(1) }.cast::<Self>();
        let handler = unsafe { &(*owner).handler };
        #[repr(C)]
        struct HandlerVtable {
            base__: windows_core::IUnknown_Vtbl,
            invoke: unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT,
        }
        let vtable = unsafe { &**Interface::as_raw(handler).cast::<*const HandlerVtable>() };
        unsafe { (vtable.invoke)(Interface::as_raw(handler), sender, args) }
    }

    unsafe extern "system" fn reference_value(
        this: *mut c_void,
        value: *mut *mut c_void,
    ) -> HRESULT {
        if value.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        let owner = unsafe { this.cast::<*const c_void>().sub(2) }.cast::<Self>();
        unsafe {
            *value = Interface::into_raw((*owner).handler.clone());
        }
        HRESULT(0)
    }
}

pub(super) fn add_preview_key_event_handler(
    element: &xaml::UIElement,
    key_down: bool,
    handler: impl FnMut(usize) -> windows_core::Result<bool> + Send + 'static,
) -> windows_core::Result<()> {
    let handler = WinUiKeyEventHandler::new(handler);
    let interface = element.cast::<xaml::IUIElement>()?;
    let vtable = Interface::vtable(&interface);
    let remove_slot = if key_down {
        ptr::addr_of!(vtable.RemovePreviewKeyDown).cast::<*const c_void>()
    } else {
        ptr::addr_of!(vtable.RemovePreviewKeyUp).cast::<*const c_void>()
    };
    // Each generated remove accessor immediately follows its omitted add
    // accessor in IUIElement's fixed event ABI.
    let method = unsafe { *remove_slot.sub(1) };
    if method.is_null() {
        return Err(windows_core::Error::from_hresult(HRESULT(
            0x8000_4005_u32 as i32,
        )));
    }
    type AddKeyHandlerMethod =
        unsafe extern "system" fn(*mut c_void, *mut c_void, *mut i64) -> HRESULT;
    let add_handler: AddKeyHandlerMethod = unsafe { core::mem::transmute(method) };
    let mut token = 0;
    unsafe {
        add_handler(
            Interface::as_raw(&interface),
            Interface::as_raw(&handler),
            &mut token,
        )
        .ok()
    }
}

fn ui_element_routed_event(statics_offset: usize) -> windows_core::Result<IInspectable> {
    static SHARED: windows_core::imp::FactoryCache<xaml::UIElement, xaml::IUIElementStatics> =
        windows_core::imp::FactoryCache::new();
    SHARED.call(|statics| unsafe {
        let slots = Interface::vtable(statics) as *const _ as *const *const c_void;
        let first_event_slot =
            core::mem::size_of::<windows_core::IInspectable_Vtbl>() / core::mem::size_of::<usize>();
        let method = *slots.add(first_event_slot + statics_offset);
        if method.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(
                0x8000_4005_u32 as i32,
            )));
        }
        type GetRoutedEventMethod =
            unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
        let get_event: GetRoutedEventMethod = core::mem::transmute(method);
        let mut event = ptr::null_mut();
        get_event(Interface::as_raw(statics), &mut event).ok()?;
        if event.is_null() {
            return Err(windows_core::Error::from_hresult(HRESULT(
                0x8000_4005_u32 as i32,
            )));
        }
        Ok(IInspectable::from_raw(event))
    })
}

#[repr(transparent)]
#[derive(Clone)]
struct WinUiKeyEventHandler(IUnknown);

unsafe impl Interface for WinUiKeyEventHandler {
    type Vtable = WinUiKeyEventHandlerVtable;
    const IID: GUID = GUID::from_u128(0xdb68e7cc_9a2b_527d_9989_25284daccc03);
}

#[repr(C)]
struct WinUiKeyEventHandlerVtable {
    base__: windows_core::IUnknown_Vtbl,
    invoke: unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT,
}

#[repr(C)]
struct WinUiKeyEventHandlerBox<F>
where
    F: FnMut(usize) -> windows_core::Result<bool> + Send + 'static,
{
    vtable: *const WinUiKeyEventHandlerVtable,
    handler: F,
    count: windows_core::imp::RefCount,
}

impl WinUiKeyEventHandler {
    fn new<F>(handler: F) -> Self
    where
        F: FnMut(usize) -> windows_core::Result<bool> + Send + 'static,
    {
        let boxed = Box::new(WinUiKeyEventHandlerBox {
            vtable: &WinUiKeyEventHandlerBox::<F>::VTABLE,
            handler,
            count: windows_core::imp::RefCount::new(1),
        });
        unsafe { Self::from_raw(Box::into_raw(boxed).cast()) }
    }
}

impl<F> WinUiKeyEventHandlerBox<F>
where
    F: FnMut(usize) -> windows_core::Result<bool> + Send + 'static,
{
    const VTABLE: WinUiKeyEventHandlerVtable = WinUiKeyEventHandlerVtable {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::query_interface,
            AddRef: Self::add_ref,
            Release: Self::release,
        },
        invoke: Self::invoke,
    };

    unsafe extern "system" fn query_interface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        let this = this.cast::<Self>();
        if iid.is_null() || interface.is_null() {
            return HRESULT(0x8000_4003_u32 as i32);
        }
        unsafe {
            *interface = if *iid == WinUiKeyEventHandler::IID
                || *iid == IUnknown::IID
                || *iid == windows_core::imp::IAgileObject::IID
            {
                ptr::addr_of_mut!((*this).vtable).cast()
            } else if *iid == windows_core::imp::IMarshal::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute::<*mut c_void, IUnknown>(
                        ptr::addr_of_mut!((*this).vtable).cast(),
                    ),
                    interface,
                );
            } else {
                ptr::null_mut()
            };
            if (*interface).is_null() {
                HRESULT(0x8000_4002_u32 as i32)
            } else {
                (*this).count.add_ref();
                HRESULT(0)
            }
        }
    }

    unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
        unsafe { (*this.cast::<Self>()).count.add_ref() }
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        let this = this.cast::<Self>();
        let remaining = unsafe { (*this).count.release() };
        if remaining == 0 {
            unsafe { drop(Box::from_raw(this)) };
        }
        remaining
    }

    unsafe extern "system" fn invoke(
        this: *mut c_void,
        _sender: *mut c_void,
        args: *mut c_void,
    ) -> HRESULT {
        let this = unsafe { &mut *this.cast::<Self>() };
        let result = unsafe { key_event_args_key(args) }.and_then(|key| {
            (this.handler)(key).and_then(|handled| {
                if handled {
                    unsafe { key_event_args_set_handled(args, true) }
                } else {
                    Ok(())
                }
            })
        });
        result.into()
    }
}

#[repr(C)]
struct WinUiKeyEventArgsVtable {
    base__: windows_core::IInspectable_Vtbl,
    key: unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT,
    key_status: usize,
    handled: usize,
    set_handled: unsafe extern "system" fn(*mut c_void, bool) -> HRESULT,
    original_key: usize,
    device_id: usize,
}

unsafe fn key_event_args_key(args: *mut c_void) -> windows_core::Result<usize> {
    if args.is_null() {
        return Err(windows_core::Error::from_hresult(HRESULT(
            0x8000_4003_u32 as i32,
        )));
    }
    let vtable = unsafe { &**args.cast::<*const WinUiKeyEventArgsVtable>() };
    let mut key = 0;
    unsafe { (vtable.key)(args, &mut key).ok()? };
    Ok(key as usize)
}

unsafe fn key_event_args_set_handled(args: *mut c_void, handled: bool) -> windows_core::Result<()> {
    let vtable = unsafe { &**args.cast::<*const WinUiKeyEventArgsVtable>() };
    unsafe { (vtable.set_handled)(args, handled).ok() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routed_event_offsets_match_iui_element_statics() {
        assert_eq!(WinUiPointerRoutedEvent::Pressed.statics_offset(), 3);
        assert_eq!(WinUiPointerRoutedEvent::Released.statics_offset(), 5);
        assert_eq!(WinUiPointerRoutedEvent::WheelChanged.statics_offset(), 6);
        assert_eq!(WinUiPointerRoutedEvent::Canceled.statics_offset(), 8);
    }
}
