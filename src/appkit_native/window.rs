use super::*;

#[derive(Debug, Clone)]
pub(super) struct AppKitWindowDelegateIvars {
    node: HostNodeId,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
}

define_class!(
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug)]
    struct AppKitFlippedView;

    impl AppKitFlippedView {
        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }
    }
);

define_class!(
    #[unsafe(super(NSStackView))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug)]
    struct AppKitFlippedStackView;

    impl AppKitFlippedStackView {
        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }
    }
);

pub(super) fn flipped_view(mtm: MainThreadMarker, frame: NSRect) -> Retained<NSView> {
    let view = AppKitFlippedView::alloc(mtm).set_ivars(());
    let view: Retained<AppKitFlippedView> = unsafe { msg_send![super(view), initWithFrame: frame] };
    view.into_super()
}

pub(super) fn flipped_stack_view(mtm: MainThreadMarker, frame: NSRect) -> Retained<NSStackView> {
    let stack_view = AppKitFlippedStackView::alloc(mtm).set_ivars(());
    let stack_view: Retained<AppKitFlippedStackView> =
        unsafe { msg_send![super(stack_view), initWithFrame: frame] };
    stack_view.into_super()
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppKitWindowDelegateIvars]
    #[derive(Debug)]
    pub(super) struct AppKitWindowDelegate;

    unsafe impl NSObjectProtocol for AppKitWindowDelegate {}

    unsafe impl NSWindowDelegate for AppKitWindowDelegate {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            true
        }

        #[unsafe(method(windowWillClose:))]
        fn window_will_close(&self, _notification: &NSNotification) {
            push_window_close_event_once(
                self.ivars().node,
                &self.ivars().events,
                &self.ivars().closed_windows,
            );
        }
    }
);

impl AppKitWindowDelegate {
    pub(super) fn new(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        closed_windows: Rc<RefCell<BTreeSet<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitWindowDelegateIvars {
            node,
            events,
            closed_windows,
        });
        unsafe { msg_send![super(this), init] }
    }
}

pub(super) fn push_window_close_event_once(
    node: HostNodeId,
    events: &Rc<RefCell<Vec<NativeEvent>>>,
    closed_windows: &Rc<RefCell<BTreeSet<HostNodeId>>>,
) {
    if closed_windows.borrow_mut().insert(node) {
        events
            .borrow_mut()
            .push(NativeEvent::new(node, NativeEventKind::Close));
    }
}
