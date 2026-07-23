use super::*;
use crate::event::{
    keyboard_move_events, NativeInteractionProfile, NativeLongPressTimer, PointerMoveState,
    PointerPressState,
};
use crate::input::{NativeEventContext, NativeInputModality, NativeKeyModifiers};

pub(super) type AppKitActivationContexts = Rc<RefCell<BTreeMap<HostNodeId, NativeEventContext>>>;

#[derive(Debug, Clone)]
struct AppKitLongPressTargetIvars {
    timer: NativeLongPressTimer,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    pointer_press: Rc<RefCell<Option<AppKitPointerPress>>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppKitLongPressTargetIvars]
    #[derive(Debug)]
    struct AppKitLongPressTarget;

    impl AppKitLongPressTarget {
        #[unsafe(method(a3sGuiLongPress:))]
        fn fire(&self, _timer: &NSTimer) {
            if let Some(recognition) = self.ivars().timer.try_fire() {
                let node = recognition.node();
                let pending = match self
                    .ivars()
                    .pointer_press
                    .borrow_mut()
                    .as_mut()
                    .filter(|active| active.node == node)
                {
                    Some(active) => {
                        recognition.into_events_with_movement(&mut active.move_state)
                    }
                    None => recognition.into_events(),
                };
                self.ivars().events.borrow_mut().extend(pending);
            }
        }
    }

    unsafe impl NSObjectProtocol for AppKitLongPressTarget {}
);

impl AppKitLongPressTarget {
    fn new(
        timer: NativeLongPressTimer,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        pointer_press: Rc<RefCell<Option<AppKitPointerPress>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitLongPressTargetIvars {
            timer,
            events,
            pointer_press,
        });
        unsafe { msg_send![super(this), init] }
    }

    fn as_any_object(&self) -> &AnyObject {
        self.as_super().as_super()
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct AppKitInteractionRegistration {
    profile: NativeInteractionProfile,
    native_press_action: bool,
}

impl AppKitInteractionRegistration {
    pub(super) fn new(widget: &AppKitOsWidget, blueprint: &NativeWidgetBlueprint) -> Self {
        Self {
            profile: NativeInteractionProfile::from_blueprint(blueprint),
            native_press_action: native_press_action(widget, blueprint.role),
        }
    }

    pub(super) fn apply_setter(&mut self, setter: &NativeWidgetSetter) {
        self.profile.apply_setter(setter);
    }
}

#[derive(Debug)]
pub(super) struct AppKitPointerPress {
    pub(super) node: HostNodeId,
    view: Retained<NSView>,
    press_state: Option<PointerPressState>,
    move_state: PointerMoveState,
}

#[derive(Debug)]
pub(super) struct AppKitHoverTarget {
    pub(super) node: HostNodeId,
    view: Retained<NSView>,
}

#[derive(Debug)]
struct AppKitHitTarget {
    node: HostNodeId,
    view: Retained<NSView>,
    registration: AppKitInteractionRegistration,
}

impl AppKitNativeSurface {
    pub(super) fn enqueue_interaction_event(&self, event: &NSEvent) -> bool {
        let event_type = event.r#type();
        if event_type == NSEventType::KeyDown || event_type == NSEventType::KeyUp {
            return self.enqueue_key_event(event);
        }
        if event_type == NSEventType::ScrollWheel {
            return self.enqueue_number_field_wheel(event);
        }

        if event_type == NSEventType::MouseExited {
            self.clear_hover(event);
            return false;
        }
        if event_type == NSEventType::MouseCancelled {
            self.cancel_pointer_press(event);
            self.clear_hover(event);
            return false;
        }

        if event_type == NSEventType::MouseEntered
            || event_type == NSEventType::MouseMoved
            || event_type == NSEventType::LeftMouseDragged
            || event_type == NSEventType::LeftMouseDown
            || event_type == NSEventType::LeftMouseUp
        {
            self.update_hover(event);
        }

        if event_type == NSEventType::LeftMouseDown {
            self.begin_pointer_press(event);
        } else if event_type == NSEventType::LeftMouseDragged {
            self.update_pointer_press_boundary(event);
        } else if event_type == NSEventType::LeftMouseUp {
            self.end_pointer_press(event);
        }
        false
    }

    pub(super) fn focus_before_event(&self, event: &NSEvent) -> Option<HostNodeId> {
        self.focus_node_in_event_window(event)
            .or_else(|| self.focused_node.get())
    }

    pub(super) fn finish_interaction_event(
        &self,
        event: &NSEvent,
        previous_focus: Option<HostNodeId>,
    ) {
        self.activation_contexts.borrow_mut().clear();
        let current_focus = self
            .focus_node_in_event_window(event)
            .or_else(|| self.focused_node.get());
        self.focused_node.set(current_focus);
        if previous_focus == current_focus {
            return;
        }

        let context = focus_context(event);
        let mut events = self.events.borrow_mut();
        if let Some(node) = previous_focus {
            events.push(NativeEvent::new(node, NativeEventKind::Blur).context(context));
        }
        if let Some(node) = current_focus {
            events.push(NativeEvent::new(node, NativeEventKind::Focus).context(context));
        }
    }

    fn enqueue_key_event(&self, event: &NSEvent) -> bool {
        let kind = if event.r#type() == NSEventType::KeyDown {
            NativeEventKind::KeyDown
        } else {
            NativeEventKind::KeyUp
        };
        let key = appkit_key_value(event);
        let target = if kind == NativeEventKind::KeyUp {
            self.keyboard_presses
                .borrow()
                .target_for_key(&key)
                .or_else(|| self.node_for_key_event(event))
        } else {
            self.node_for_key_event(event)
        };
        let Some(node) = target else {
            return false;
        };
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Keyboard)
            .modifiers(appkit_modifiers(event.modifierFlags()))
            .repeat(kind == NativeEventKind::KeyDown && event.isARepeat());
        let Some(registration) = self.interaction_nodes.get(&node).copied() else {
            self.events
                .borrow_mut()
                .push(NativeEvent::new(node, kind).value(key).context(context));
            return false;
        };
        let number_field_step_handled =
            registration
                .profile
                .handles_number_field_step_key(kind, &key, context.modifiers);
        let mut pending =
            if kind == NativeEventKind::KeyDown && registration.profile.tracks_movement() {
                keyboard_move_events(node, &key, context)
            } else {
                Vec::new()
            };
        pending.extend(self.keyboard_presses.borrow_mut().events(
            node,
            key.clone(),
            kind,
            context,
            registration.profile.role,
            registration.profile.subscriptions.tracks_press(),
        ));
        let movement_handled = pending
            .iter()
            .any(|event| event.kind == NativeEventKind::MoveStart);
        self.events.borrow_mut().extend(pending);

        if registration.native_press_action
            && crate::event::is_press_activation_key(registration.profile.role, Some(&key))
        {
            let activation_context =
                context.handled_activation(registration.profile.normalizes_keyboard_press());
            self.activation_contexts
                .borrow_mut()
                .insert(node, activation_context);
        }
        movement_handled || number_field_step_handled
    }

    fn enqueue_number_field_wheel(&self, event: &NSEvent) -> bool {
        // AppKit reports positive vertical deltas for an upward wheel gesture,
        // while the portable event contract follows DOM wheel signs.
        let delta_x = event.scrollingDeltaX();
        let delta_y = -event.scrollingDeltaY();
        let modifiers = appkit_modifiers(event.modifierFlags());
        let Some(target) = self.interaction_target_at_event(event, |registration| {
            registration
                .profile
                .handles_number_field_wheel(true, delta_x, delta_y, modifiers)
        }) else {
            return false;
        };
        let focused = self.focus_node_in_event_window(event) == Some(target.node);
        if !target
            .registration
            .profile
            .handles_number_field_wheel(focused, delta_x, delta_y, modifiers)
        {
            return false;
        }

        let context = pointer_context(event, &target.view).delta(delta_x, delta_y);
        self.events
            .borrow_mut()
            .push(NativeEvent::new(target.node, NativeEventKind::Wheel).context(context));
        true
    }

    fn begin_pointer_press(&self, event: &NSEvent) {
        let Some(target) = self.interaction_target_at_event(event, |registration| {
            registration.native_press_action
                || registration.profile.subscriptions.tracks_press()
                || registration.profile.tracks_movement()
        }) else {
            return;
        };

        if let Some(mut previous) = self.pointer_press.borrow_mut().take() {
            let context = pointer_context(event, &previous.view);
            let mut events = self.events.borrow_mut();
            events.extend(previous.move_state.end(previous.node, context));
            if let Some(state) = previous.press_state.as_mut() {
                events.extend(state.cancel(previous.node, context));
            }
        }

        let context = pointer_context(event, &target.view);
        let mut press_state = (target.registration.native_press_action
            || target.registration.profile.subscriptions.tracks_press())
        .then(PointerPressState::default);
        let (pending, timer) = if let Some(state) = press_state.as_mut() {
            let pending = state.begin_with_long_press(
                target.node,
                context,
                target.registration.profile.long_press_config(),
            );
            let timer = state.take_long_press_timer(target.node, context);
            (pending, timer)
        } else {
            (Vec::new(), None)
        };
        let mut move_state = PointerMoveState::default();
        if target.registration.profile.tracks_movement() {
            move_state.begin(context);
        }
        self.events.borrow_mut().extend(pending);
        if let Some(timer) = timer {
            self.schedule_long_press(timer);
        }
        *self.pointer_press.borrow_mut() = Some(AppKitPointerPress {
            node: target.node,
            view: target.view,
            press_state,
            move_state,
        });
    }

    fn update_pointer_press_boundary(&self, event: &NSEvent) {
        let mut active = self.pointer_press.borrow_mut();
        let Some(active) = active.as_mut() else {
            return;
        };
        let context = pointer_context(event, &active.view);
        let mut events = self.events.borrow_mut();
        events.extend(active.move_state.update(active.node, context));
        let mut timer = None;
        if let Some(state) = active.press_state.as_mut() {
            let pending = if point_is_inside(&active.view, context) {
                let pending = state.enter(active.node, context);
                timer = state.take_long_press_timer(active.node, context);
                pending
            } else {
                state.leave(active.node, context)
            };
            events.extend(pending);
        }
        drop(events);
        if let Some(timer) = timer {
            self.schedule_long_press(timer);
        }
    }

    fn end_pointer_press(&self, event: &NSEvent) {
        let Some(mut active) = self.pointer_press.borrow_mut().take() else {
            return;
        };
        let context = pointer_context(event, &active.view);
        self.events
            .borrow_mut()
            .extend(active.move_state.end(active.node, context));
        let over_target = point_is_inside(&active.view, context);

        let registration = self.interaction_nodes.get(&active.node).copied();
        let mut long_pressed = false;
        if let Some(state) = active.press_state.as_mut() {
            let boundary = if over_target {
                state.enter(active.node, context)
            } else {
                state.leave(active.node, context)
            };
            self.events.borrow_mut().extend(boundary);
            let emit_press = registration.is_some_and(|registration| {
                !registration.native_press_action
                    && registration.profile.subscriptions.terminal_press
            });
            let recognized = state.long_press_recognized();
            let pending = state.release(active.node, context, emit_press);
            long_pressed = recognized
                || pending
                    .iter()
                    .any(|event| event.kind == NativeEventKind::LongPress);
            self.events.borrow_mut().extend(pending);
        }

        if over_target && registration.is_some_and(|registration| registration.native_press_action)
        {
            self.activation_contexts
                .borrow_mut()
                .insert(active.node, context.handled_activation(long_pressed));
        } else {
            self.activation_contexts.borrow_mut().remove(&active.node);
        }
    }

    fn cancel_pointer_press(&self, event: &NSEvent) {
        let Some(mut active) = self.pointer_press.borrow_mut().take() else {
            return;
        };
        self.activation_contexts.borrow_mut().remove(&active.node);
        let context = pointer_context(event, &active.view);
        let mut events = self.events.borrow_mut();
        events.extend(active.move_state.end(active.node, context));
        if let Some(state) = active.press_state.as_mut() {
            events.extend(state.cancel(active.node, context));
        }
    }

    fn schedule_long_press(&self, timer: NativeLongPressTimer) {
        let interval = timer.threshold().as_secs_f64();
        let target = AppKitLongPressTarget::new(
            timer,
            Rc::clone(&self.events),
            Rc::clone(&self.pointer_press),
            self.mtm,
        );
        let timer = unsafe {
            NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                interval,
                target.as_any_object(),
                sel!(a3sGuiLongPress:),
                None,
                false,
            )
        };
        unsafe {
            NSRunLoop::mainRunLoop().addTimer_forMode(&timer, NSRunLoopCommonModes);
        }
    }

    fn update_hover(&self, event: &NSEvent) {
        let target = self.interaction_target_at_event(event, |registration| {
            registration.profile.subscriptions.hover
        });
        let mut hovered = self.hovered_pointer.borrow_mut();
        if hovered.as_ref().map(|target| target.node) == target.as_ref().map(|target| target.node) {
            return;
        }

        if let Some(previous) = hovered.take() {
            let context = pointer_context(event, &previous.view);
            self.events
                .borrow_mut()
                .push(NativeEvent::new(previous.node, NativeEventKind::HoverEnd).context(context));
        }

        if let Some(target) = target {
            let context = pointer_context(event, &target.view);
            if context.modality.supports_hover() {
                self.events.borrow_mut().push(
                    NativeEvent::new(target.node, NativeEventKind::HoverStart).context(context),
                );
                *hovered = Some(AppKitHoverTarget {
                    node: target.node,
                    view: target.view,
                });
            }
        }
    }

    fn clear_hover(&self, event: &NSEvent) {
        let Some(previous) = self.hovered_pointer.borrow_mut().take() else {
            return;
        };
        let context = pointer_context(event, &previous.view);
        self.events
            .borrow_mut()
            .push(NativeEvent::new(previous.node, NativeEventKind::HoverEnd).context(context));
    }

    fn interaction_target_at_event(
        &self,
        event: &NSEvent,
        accepts: impl Fn(AppKitInteractionRegistration) -> bool,
    ) -> Option<AppKitHitTarget> {
        let window = event.window(self.mtm)?;
        let content = window.contentView()?;
        let point = content.convertPoint_fromView(event.locationInWindow(), None);
        let mut view = content.hitTest(point)?;
        loop {
            if let Some(node) = self
                .responder_nodes
                .get(&responder_key(view.as_super()))
                .copied()
            {
                if let Some(registration) = self.interaction_nodes.get(&node).copied() {
                    if accepts(registration) {
                        return Some(AppKitHitTarget {
                            node,
                            view,
                            registration,
                        });
                    }
                }
            }
            view = unsafe { view.superview()? };
        }
    }

    fn focus_node_in_event_window(&self, event: &NSEvent) -> Option<HostNodeId> {
        let window = event.window(self.mtm)?;
        let responder = window.firstResponder()?;
        self.node_for_responder(&responder)
    }

    pub(super) fn node_for_responder(&self, responder: &NSResponder) -> Option<HostNodeId> {
        self.responder_nodes
            .get(&responder_key(responder))
            .copied()
            .or_else(|| {
                responder
                    .downcast_ref::<NSView>()
                    .and_then(|view| self.node_for_view(view))
            })
    }

    fn node_for_view(&self, view: &NSView) -> Option<HostNodeId> {
        if let Some(node) = self
            .responder_nodes
            .get(&responder_key(view.as_super()))
            .copied()
        {
            return Some(node);
        }
        let mut view = unsafe { view.superview() };
        while let Some(current) = view {
            if let Some(node) = self
                .responder_nodes
                .get(&responder_key(current.as_super()))
                .copied()
            {
                return Some(node);
            }
            view = unsafe { current.superview() };
        }
        None
    }
}

pub(super) fn install_pointer_tracking_area(window: &NSWindow, view: &NSView) {
    window.setAcceptsMouseMovedEvents(true);
    let options = NSTrackingAreaOptions::MouseEnteredAndExited
        | NSTrackingAreaOptions::MouseMoved
        | NSTrackingAreaOptions::ActiveInKeyWindow
        | NSTrackingAreaOptions::InVisibleRect
        | NSTrackingAreaOptions::EnabledDuringMouseDrag;
    let tracking_area = unsafe {
        NSTrackingArea::initWithRect_options_owner_userInfo(
            NSTrackingArea::alloc(),
            view.bounds(),
            options,
            Some(view.as_super().as_super().as_super()),
            None,
        )
    };
    view.addTrackingArea(&tracking_area);
}

pub(super) fn take_activation_context(
    contexts: &AppKitActivationContexts,
    node: HostNodeId,
) -> Option<NativeEventContext> {
    contexts.borrow_mut().remove(&node)
}

fn native_press_action(widget: &AppKitOsWidget, role: crate::native::NativeRole) -> bool {
    matches!(widget, AppKitOsWidget::MenuItem(_))
        || matches!(
            role,
            crate::native::NativeRole::Button
                | crate::native::NativeRole::Link
                | crate::native::NativeRole::ImageMapArea
                | crate::native::NativeRole::DisclosureSummary
        )
}

fn pointer_context(event: &NSEvent, view: &NSView) -> NativeEventContext {
    let point = view.convertPoint_fromView(event.locationInWindow(), None);
    NativeEventContext::new()
        .modality(pointer_modality(event))
        .modifiers(appkit_modifiers(event.modifierFlags()))
        .position(point.x, point.y)
        .click_count(u8::try_from(event.clickCount()).unwrap_or(u8::MAX))
}

fn focus_context(event: &NSEvent) -> NativeEventContext {
    let event_type = event.r#type();
    let modality = if event_type == NSEventType::KeyDown
        || event_type == NSEventType::KeyUp
        || event_type == NSEventType::FlagsChanged
    {
        NativeInputModality::Keyboard
    } else if event_type == NSEventType::LeftMouseDown
        || event_type == NSEventType::LeftMouseUp
        || event_type == NSEventType::LeftMouseDragged
        || event_type == NSEventType::MouseMoved
    {
        pointer_modality(event)
    } else {
        NativeInputModality::Unknown
    };
    NativeEventContext::new()
        .modality(modality)
        .modifiers(appkit_modifiers(event.modifierFlags()))
}

fn pointer_modality(event: &NSEvent) -> NativeInputModality {
    let device = event.pointingDeviceType();
    if device == NSPointingDeviceType::Pen || device == NSPointingDeviceType::Eraser {
        NativeInputModality::Pen
    } else {
        NativeInputModality::Mouse
    }
}

fn appkit_modifiers(flags: NSEventModifierFlags) -> NativeKeyModifiers {
    NativeKeyModifiers::new()
        .alt(flags.contains(NSEventModifierFlags::Option))
        .control(flags.contains(NSEventModifierFlags::Control))
        .meta(flags.contains(NSEventModifierFlags::Command))
        .shift(flags.contains(NSEventModifierFlags::Shift))
}

fn point_is_inside(view: &NSView, context: NativeEventContext) -> bool {
    let Some(point) = context.position else {
        return false;
    };
    let bounds = view.bounds();
    point.x >= bounds.origin.x
        && point.y >= bounds.origin.y
        && point.x <= bounds.origin.x + bounds.size.width
        && point.y <= bounds.origin.y + bounds.size.height
}
