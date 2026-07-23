use super::event_loop::winui_key_value_from_virtual_key;
use super::*;

use crate::event::{
    keyboard_move_events, virtual_press_events, NativeInteractionProfile, NativeLongPressTimer,
    PointerMoveState, PointerPressState,
};
use crate::native::NativeRole;
use winui3::Microsoft::UI::Dispatching::DispatcherQueueTimer;
use winui3::Microsoft::UI::Input::PointerDeviceType;
use xaml::Input::{PointerEventHandler, PointerRoutedEventArgs};

#[derive(Debug, Clone, Copy)]
pub(super) struct WinUiInteractionRegistration {
    pub(super) profile: NativeInteractionProfile,
    native_button: bool,
    native_selection_item: bool,
}

impl WinUiInteractionRegistration {
    pub(super) fn new(widget: &WinUiOsWidget, blueprint: &NativeWidgetBlueprint) -> Self {
        Self {
            profile: NativeInteractionProfile::from_blueprint(blueprint),
            native_button: matches!(widget, WinUiOsWidget::Button(_)),
            native_selection_item: matches!(
                (widget, blueprint.role),
                (
                    WinUiOsWidget::ListBoxItem(_) | WinUiOsWidget::ComboBoxItem(_),
                    NativeRole::ListBoxItem | NativeRole::TreeItem
                )
            ),
        }
    }

    pub(super) fn apply_setter(&mut self, setter: &NativeWidgetSetter) {
        self.profile.apply_setter(setter);
    }

    pub(super) fn awaits_native_activation(self) -> bool {
        self.native_button || self.native_selection_item
    }
}

pub(super) fn register_interaction_events(
    id: HostNodeId,
    widget: &WinUiOsWidget,
    events: &WinUiEventQueue,
    focused_node: WinUiFocusedNode,
    key_modifiers: WinUiKeyModifiers,
    activation_contexts: WinUiActivationContexts,
    pending_activation_cleanup: WinUiPendingActivationCleanup,
    forced_pointer_cancellations: Arc<Mutex<BTreeSet<HostNodeId>>>,
    registration: Arc<Mutex<WinUiInteractionRegistration>>,
    keyboard_presses: Arc<Mutex<KeyboardPressState>>,
) -> GuiResult<()> {
    let Some(element) = widget.ui_element() else {
        return Ok(());
    };
    let is_button = registration
        .lock()
        .map(|registration| registration.native_button)
        .unwrap_or(false);
    let press_state = Arc::new(Mutex::new(PointerPressState::default()));
    let move_state = Arc::new(Mutex::new(PointerMoveState::default()));

    if is_button {
        register_button_click(id, widget, events, Arc::clone(&activation_contexts))?;
    }
    register_number_field_wheel(
        id,
        &element,
        events,
        focused_node,
        Arc::clone(&registration),
    )?;
    register_pointer_press_lifecycle(
        id,
        &element,
        events,
        Arc::clone(&activation_contexts),
        pending_activation_cleanup,
        Arc::clone(&press_state),
        Arc::clone(&move_state),
        Arc::clone(&registration),
        forced_pointer_cancellations,
    )?;
    register_pointer_boundary_events(
        id,
        &element,
        events,
        Arc::clone(&activation_contexts),
        press_state,
        move_state,
        Arc::clone(&registration),
    )?;
    register_preview_keyboard_events(
        id,
        &element,
        events,
        activation_contexts,
        registration,
        keyboard_presses,
        key_modifiers,
    )?;
    Ok(())
}

fn register_preview_keyboard_events(
    id: HostNodeId,
    element: &xaml::UIElement,
    events: &WinUiEventQueue,
    activation_contexts: WinUiActivationContexts,
    registration: Arc<Mutex<WinUiInteractionRegistration>>,
    keyboard_presses: Arc<Mutex<KeyboardPressState>>,
    key_modifiers: WinUiKeyModifiers,
) -> GuiResult<()> {
    let down_events = Arc::clone(events);
    let down_contexts = Arc::clone(&activation_contexts);
    let down_registration = Arc::clone(&registration);
    let down_presses = Arc::clone(&keyboard_presses);
    let down_modifiers = Arc::clone(&key_modifiers);
    map_winui(
        "failed to register WinUI preview key-down handler",
        input_abi::add_preview_key_event_handler(element, true, move |virtual_key| {
            let key = winui_key_value_from_virtual_key(virtual_key);
            let modifiers = down_modifiers
                .lock()
                .map(|modifiers| *modifiers)
                .unwrap_or_default();
            let context = NativeEventContext::new()
                .modality(NativeInputModality::Keyboard)
                .modifiers(modifiers);
            let registration = down_registration
                .lock()
                .ok()
                .map(|registration| *registration);
            let number_field_step_handled = registration.is_some_and(|registration| {
                registration.profile.handles_number_field_step_key(
                    NativeEventKind::KeyDown,
                    &key,
                    context.modifiers,
                )
            });
            if registration.is_some_and(|registration| registration.awaits_native_activation()) {
                remember_activation_context(&down_contexts, id, context);
            }
            let pending = registration
                .map(|registration| {
                    keyboard_events(
                        id,
                        key.clone(),
                        NativeEventKind::KeyDown,
                        context,
                        registration,
                        &down_presses,
                    )
                })
                .unwrap_or_else(|| {
                    vec![NativeEvent::new(id, NativeEventKind::KeyDown)
                        .value(key)
                        .context(context)]
                });
            let prevent_default = number_field_step_handled
                || pending
                    .iter()
                    .any(|event| event.kind == NativeEventKind::MoveStart);
            push_events(&down_events, pending);
            Ok(prevent_default)
        }),
    )?;

    let up_events = Arc::clone(events);
    let up_modifiers = key_modifiers;
    map_winui(
        "failed to register WinUI preview key-up handler",
        input_abi::add_preview_key_event_handler(element, false, move |virtual_key| {
            let key = winui_key_value_from_virtual_key(virtual_key);
            let modifiers = up_modifiers
                .lock()
                .map(|modifiers| *modifiers)
                .unwrap_or_default();
            let context = NativeEventContext::new()
                .modality(NativeInputModality::Keyboard)
                .modifiers(modifiers);
            let registration = registration.lock().ok().map(|registration| *registration);
            if registration.is_some_and(|registration| registration.awaits_native_activation()) {
                remember_activation_context(&activation_contexts, id, context);
            }
            let pending = registration
                .map(|registration| {
                    keyboard_events(
                        id,
                        key.clone(),
                        NativeEventKind::KeyUp,
                        context,
                        registration,
                        &keyboard_presses,
                    )
                })
                .unwrap_or_else(|| {
                    vec![NativeEvent::new(id, NativeEventKind::KeyUp)
                        .value(key)
                        .context(context)]
                });
            let prevent_default = pending
                .iter()
                .any(|event| event.kind == NativeEventKind::MoveStart);
            push_events(&up_events, pending);
            Ok(prevent_default)
        }),
    )
}

fn register_number_field_wheel(
    id: HostNodeId,
    element: &xaml::UIElement,
    events: &WinUiEventQueue,
    focused_node: WinUiFocusedNode,
    registration: Arc<Mutex<WinUiInteractionRegistration>>,
) -> GuiResult<()> {
    let wheel_events = Arc::clone(events);
    let wheel_element = element.clone();
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let properties = args
            .GetCurrentPoint(&wheel_element)
            .and_then(|point| point.Properties())?;
        let delta = f64::from(properties.MouseWheelDelta()?);
        let (delta_x, delta_y) = if properties.IsHorizontalMouseWheel()? {
            (-delta, 0.0)
        } else {
            // WinUI reports positive deltas for an upward wheel gesture. The
            // portable wheel contract follows DOM signs.
            (0.0, -delta)
        };
        let context = pointer_event_context(args, &wheel_element).delta(delta_x, delta_y);
        let focused = focused_node
            .lock()
            .map(|focused| *focused == Some(id))
            .unwrap_or(false);
        let handled = registration
            .lock()
            .map(|registration| {
                registration.profile.handles_number_field_wheel(
                    focused,
                    delta_x,
                    delta_y,
                    context.modifiers,
                )
            })
            .unwrap_or(false);
        if handled {
            push_event(
                &wheel_events,
                NativeEvent::new(id, NativeEventKind::Wheel).context(context),
            );
            args.SetHandled(true)?;
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI number-field wheel handler",
        input_abi::add_handled_pointer_event_handler(
            element,
            input_abi::WinUiPointerRoutedEvent::WheelChanged,
            &handler,
        ),
    )
}

fn register_button_click(
    id: HostNodeId,
    widget: &WinUiOsWidget,
    events: &WinUiEventQueue,
    activation_contexts: WinUiActivationContexts,
) -> GuiResult<()> {
    let WinUiOsWidget::Button(button) = widget else {
        return Ok(());
    };
    let events = Arc::clone(events);
    let handler = RoutedEventHandler::new(move |_, _| {
        match take_activation_context(&activation_contexts, id) {
            Some(context) if context.modality != NativeInputModality::Unknown => {}
            Some(_) | None => push_events(&events, virtual_press_events(id)),
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI button press handler",
        button.Click(&handler),
    )?;
    Ok(())
}

pub(super) fn active_keyboard_target(
    active: &Mutex<KeyboardPressState>,
    key: &str,
) -> Option<HostNodeId> {
    active.lock().ok()?.target_for_key(key)
}

pub(super) fn keyboard_events(
    id: HostNodeId,
    key: String,
    kind: NativeEventKind,
    context: NativeEventContext,
    registration: WinUiInteractionRegistration,
    active: &Mutex<KeyboardPressState>,
) -> Vec<NativeEvent> {
    let mut events = if kind == NativeEventKind::KeyDown && registration.profile.tracks_movement() {
        keyboard_move_events(id, &key, context)
    } else {
        Vec::new()
    };
    match active.lock() {
        Ok(mut active) => events.extend(active.events(
            id,
            key,
            kind,
            context,
            registration.profile.role,
            registration.profile.subscriptions.tracks_press(),
        )),
        Err(_) => events.push(NativeEvent::new(id, kind).value(key).context(context)),
    }
    events
}

fn register_pointer_press_lifecycle(
    id: HostNodeId,
    element: &xaml::UIElement,
    events: &WinUiEventQueue,
    activation_contexts: WinUiActivationContexts,
    pending_activation_cleanup: WinUiPendingActivationCleanup,
    pressed: Arc<Mutex<PointerPressState>>,
    movement: Arc<Mutex<PointerMoveState>>,
    registration: Arc<Mutex<WinUiInteractionRegistration>>,
    forced_pointer_cancellations: Arc<Mutex<BTreeSet<HostNodeId>>>,
) -> GuiResult<()> {
    let press_events = Arc::clone(events);
    let press_element = element.clone();
    let press_state = Arc::clone(&pressed);
    let press_move_state = Arc::clone(&movement);
    let press_registration = Arc::clone(&registration);
    let press_contexts = Arc::clone(&activation_contexts);
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        if !is_primary_pointer_press(args, &press_element) {
            return Ok(());
        }
        let Ok(current) = press_registration.lock().map(|registration| *registration) else {
            return Ok(());
        };
        if !current.native_button
            && !current.profile.subscriptions.tracks_press()
            && !current.profile.tracks_movement()
        {
            return Ok(());
        }
        let context = pointer_event_context(args, &press_element);
        if current.awaits_native_activation() {
            remember_activation_context(&press_contexts, id, context);
        }
        if current.native_button || current.profile.subscriptions.tracks_press() {
            let Ok(mut state) = press_state.lock() else {
                return Ok(());
            };
            let pending =
                state.begin_with_long_press(id, context, current.profile.long_press_config());
            let timer = state.take_long_press_timer(id, context);
            push_events(&press_events, pending);
            drop(state);
            if let Some(timer) = timer {
                let _ = schedule_long_press(
                    timer,
                    &press_element,
                    Arc::clone(&press_events),
                    Arc::clone(&press_move_state),
                );
            }
        }
        if current.profile.tracks_movement() {
            if let Ok(mut state) = press_move_state.lock() {
                state.begin_pointer(pointer_id(args), context);
            }
        }
        if let Ok(pointer) = args.Pointer() {
            let _ = press_element.CapturePointer(&pointer);
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI pointer press handler",
        input_abi::add_handled_pointer_event_handler(
            element,
            input_abi::WinUiPointerRoutedEvent::Pressed,
            &handler,
        ),
    )?;

    let move_events = Arc::clone(events);
    let move_element = element.clone();
    let move_state = Arc::clone(&movement);
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let context = pointer_event_context(args, &move_element);
        if let Ok(mut state) = move_state.lock() {
            push_events(
                &move_events,
                state.update_pointer(pointer_id(args), id, context),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI pointer move handler",
        input_abi::add_handled_pointer_event_handler(
            element,
            input_abi::WinUiPointerRoutedEvent::Moved,
            &handler,
        ),
    )?;

    let release_events = Arc::clone(events);
    let release_element = element.clone();
    let release_state = Arc::clone(&pressed);
    let release_move_state = Arc::clone(&movement);
    let release_contexts = Arc::clone(&activation_contexts);
    let release_pending_cleanup = pending_activation_cleanup;
    let release_registration = registration;
    let release_forced_cancellations = Arc::clone(&forced_pointer_cancellations);
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let context = pointer_event_context(args, &release_element);
        let forced = release_forced_cancellations
            .lock()
            .map(|mut cancellations| cancellations.remove(&id))
            .unwrap_or(false);
        let canceled = forced || pointer_is_canceled(args, &release_element);
        if let Ok(mut state) = release_move_state.lock() {
            let pending = if canceled {
                state.cancel(id, context)
            } else {
                state.end_pointer(pointer_id(args), id, context)
            };
            push_events(&release_events, pending);
        }
        if canceled {
            forget_activation_context(&release_contexts, id);
            if let Ok(mut state) = release_state.lock() {
                push_events(&release_events, state.cancel(id, context));
            }
            if let Ok(pointer) = args.Pointer() {
                let _ = release_element.ReleasePointerCapture(&pointer);
            }
            return Ok(());
        }
        let over_target = pointer_is_over_target(args, &release_element);
        let current = release_registration
            .lock()
            .ok()
            .map(|registration| *registration);
        let emit_press_on_release =
            current.is_some_and(|registration| registration.profile.subscriptions.terminal_press);
        let mut long_pressed = false;
        if let Ok(mut state) = release_state.lock() {
            let boundary_events = if over_target {
                state.enter(id, context)
            } else {
                state.leave(id, context)
            };
            push_events(&release_events, boundary_events);
            let recognized = state.long_press_recognized();
            let pending = state.release(id, context, emit_press_on_release);
            long_pressed = recognized
                || pending
                    .iter()
                    .any(|event| event.kind == NativeEventKind::LongPress);
            push_events(&release_events, pending);
        }
        if over_target && current.is_some_and(|registration| registration.native_button) {
            update_activation_context_if_present(
                &release_contexts,
                id,
                context.handled_activation(long_pressed),
            );
            if let Ok(mut pending) = release_pending_cleanup.lock() {
                pending.insert(id);
            }
        } else {
            forget_activation_context(&release_contexts, id);
        }
        if let Ok(pointer) = args.Pointer() {
            let _ = release_element.ReleasePointerCapture(&pointer);
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI pointer release handler",
        input_abi::add_handled_pointer_event_handler(
            element,
            input_abi::WinUiPointerRoutedEvent::Released,
            &handler,
        ),
    )?;

    register_pointer_cancel(
        id,
        element,
        events,
        Arc::clone(&pressed),
        Arc::clone(&movement),
        Arc::clone(&activation_contexts),
        Arc::clone(&forced_pointer_cancellations),
        false,
    )?;
    register_pointer_cancel(
        id,
        element,
        events,
        pressed,
        movement,
        activation_contexts,
        forced_pointer_cancellations,
        true,
    )?;
    Ok(())
}

fn schedule_long_press(
    timer: NativeLongPressTimer,
    element: &xaml::UIElement,
    events: WinUiEventQueue,
    movement: Arc<Mutex<PointerMoveState>>,
) -> windows_core::Result<()> {
    let dispatcher_timer = element.DispatcherQueue()?.CreateTimer()?;
    let ticks = timer
        .threshold()
        .as_nanos()
        .div_ceil(100)
        .min(i64::MAX as u128) as i64;
    dispatcher_timer.SetInterval(windows::Foundation::TimeSpan { Duration: ticks })?;
    dispatcher_timer.SetIsRepeating(false)?;
    let tick_token = Arc::new(Mutex::new(None));
    let handler_token = Arc::clone(&tick_token);
    let handler_timer = dispatcher_timer.clone();
    let handler = windows::Foundation::TypedEventHandler::<
        DispatcherQueueTimer,
        windows_core::IInspectable,
    >::new(move |_, _| {
        if let Some(recognition) = timer.try_fire() {
            let pending = match movement.lock() {
                Ok(mut movement) => recognition.into_events_with_movement(&mut movement),
                Err(_) => recognition.into_events(),
            };
            push_events(&events, pending);
        }
        handler_timer.Stop()?;
        if let Ok(mut token) = handler_token.lock() {
            if let Some(token) = token.take() {
                handler_timer.RemoveTick(token)?;
            }
        }
        Ok(())
    });
    let token = dispatcher_timer.Tick(&handler)?;
    if let Ok(mut tick_token) = tick_token.lock() {
        *tick_token = Some(token);
    }
    if let Err(error) = dispatcher_timer.Start() {
        let _ = dispatcher_timer.RemoveTick(token);
        return Err(error);
    }
    Ok(())
}

fn register_pointer_cancel(
    id: HostNodeId,
    element: &xaml::UIElement,
    events: &WinUiEventQueue,
    pressed: Arc<Mutex<PointerPressState>>,
    movement: Arc<Mutex<PointerMoveState>>,
    activation_contexts: WinUiActivationContexts,
    forced_pointer_cancellations: Arc<Mutex<BTreeSet<HostNodeId>>>,
    capture_lost: bool,
) -> GuiResult<()> {
    let cancel_events = Arc::clone(events);
    let cancel_element = element.clone();
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let forced = forced_pointer_cancellations
            .lock()
            .map(|mut cancellations| cancellations.remove(&id))
            .unwrap_or(false);
        if capture_lost && !forced && !pointer_is_in_contact(args, &cancel_element) {
            return Ok(());
        }
        forget_activation_context(&activation_contexts, id);
        let context = pointer_event_context(args, &cancel_element);
        if let Ok(mut state) = movement.lock() {
            push_events(&cancel_events, state.cancel(id, context));
        }
        if let Ok(mut state) = pressed.lock() {
            push_events(&cancel_events, state.cancel(id, context));
        }
        Ok(())
    });
    if capture_lost {
        map_winui(
            "failed to register WinUI pointer capture-lost handler",
            input_abi::add_handled_pointer_event_handler(
                element,
                input_abi::WinUiPointerRoutedEvent::CaptureLost,
                &handler,
            ),
        )?;
    } else {
        map_winui(
            "failed to register WinUI pointer cancel handler",
            input_abi::add_handled_pointer_event_handler(
                element,
                input_abi::WinUiPointerRoutedEvent::Canceled,
                &handler,
            ),
        )?;
    }
    Ok(())
}

fn register_pointer_boundary_events(
    id: HostNodeId,
    element: &xaml::UIElement,
    events: &WinUiEventQueue,
    activation_contexts: WinUiActivationContexts,
    pressed: Arc<Mutex<PointerPressState>>,
    movement: Arc<Mutex<PointerMoveState>>,
    registration: Arc<Mutex<WinUiInteractionRegistration>>,
) -> GuiResult<()> {
    let hover_active = Arc::new(AtomicBool::new(false));
    let enter_events = Arc::clone(events);
    let enter_element = element.clone();
    let enter_pressed = Arc::clone(&pressed);
    let enter_registration = Arc::clone(&registration);
    let enter_hover_active = Arc::clone(&hover_active);
    let enter_movement = Arc::clone(&movement);
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let context = pointer_event_context(args, &enter_element);
        let current = enter_registration
            .lock()
            .ok()
            .map(|registration| *registration);
        if current.is_some_and(|registration| {
            registration.native_button || registration.profile.subscriptions.tracks_press()
        }) {
            if let Ok(mut state) = enter_pressed.lock() {
                let pending = state.enter(id, context);
                let timer = state.take_long_press_timer(id, context);
                drop(state);
                push_events(&enter_events, pending);
                if let Some(timer) = timer {
                    let _ = schedule_long_press(
                        timer,
                        &enter_element,
                        Arc::clone(&enter_events),
                        Arc::clone(&enter_movement),
                    );
                }
            }
        }
        if current.is_some_and(|registration| registration.profile.subscriptions.hover)
            && context.modality.supports_hover()
        {
            enter_hover_active.store(true, Ordering::Relaxed);
            push_event(
                &enter_events,
                NativeEvent::new(id, NativeEventKind::HoverStart).context(context),
            );
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI pointer enter handler",
        element.PointerEntered(&handler),
    )?;

    let exit_events = Arc::clone(events);
    let exit_element = element.clone();
    let exit_pressed = pressed;
    let exit_registration = registration;
    let handler = PointerEventHandler::new(move |_, args| {
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let context = pointer_event_context(args, &exit_element);
        let current = exit_registration
            .lock()
            .ok()
            .map(|registration| *registration);
        if current.is_some_and(|registration| {
            registration.native_button || registration.profile.subscriptions.tracks_press()
        }) {
            if let Ok(mut state) = exit_pressed.lock() {
                push_events(&exit_events, state.leave(id, context));
            }
        }
        if hover_active.swap(false, Ordering::Relaxed) {
            push_event(
                &exit_events,
                NativeEvent::new(id, NativeEventKind::HoverEnd).context(context),
            );
        }
        if args
            .Pointer()
            .is_ok_and(|pointer| !pointer.IsInContact().unwrap_or(false))
        {
            forget_activation_context(&activation_contexts, id);
        }
        Ok(())
    });
    map_winui(
        "failed to register WinUI pointer exit handler",
        element.PointerExited(&handler),
    )?;
    Ok(())
}

fn push_events(events: &WinUiEventQueue, pending: Vec<NativeEvent>) {
    for event in pending {
        push_event(events, event);
    }
}

fn is_primary_pointer_press(args: &PointerRoutedEventArgs, element: &xaml::UIElement) -> bool {
    args.GetCurrentPoint(element)
        .and_then(|point| point.Properties())
        .and_then(|properties| properties.IsLeftButtonPressed())
        .unwrap_or_else(|_| {
            args.Pointer()
                .is_ok_and(|pointer| pointer.IsInContact().unwrap_or(false))
        })
}

fn pointer_is_over_target(args: &PointerRoutedEventArgs, element: &xaml::UIElement) -> bool {
    let Ok(position) = args
        .GetCurrentPoint(element)
        .and_then(|point| point.Position())
    else {
        return false;
    };
    let Ok(framework_element) = element.cast::<xaml::FrameworkElement>() else {
        return false;
    };
    let (Ok(width), Ok(height)) = (
        framework_element.ActualWidth(),
        framework_element.ActualHeight(),
    ) else {
        return false;
    };
    let x = f64::from(position.X);
    let y = f64::from(position.Y);
    x >= 0.0 && x <= width && y >= 0.0 && y <= height
}

fn pointer_is_canceled(args: &PointerRoutedEventArgs, element: &xaml::UIElement) -> bool {
    args.GetCurrentPoint(element)
        .and_then(|point| point.Properties())
        .and_then(|properties| properties.IsCanceled())
        .unwrap_or(false)
}

fn pointer_is_in_contact(args: &PointerRoutedEventArgs, element: &xaml::UIElement) -> bool {
    args.Pointer()
        .and_then(|pointer| pointer.IsInContact())
        .or_else(|_| {
            args.GetCurrentPoint(element)
                .and_then(|point| point.IsInContact())
        })
        .unwrap_or(false)
}

fn pointer_event_context(
    args: &PointerRoutedEventArgs,
    element: &xaml::UIElement,
) -> NativeEventContext {
    let modality = args
        .Pointer()
        .and_then(|pointer| pointer.PointerDeviceType())
        .map(pointer_modality)
        .unwrap_or_default();
    let modifiers = args
        .KeyModifiers()
        .map(|modifiers| modifiers_from_bits(modifiers.0))
        .unwrap_or_default();
    let position = args
        .GetCurrentPoint(element)
        .and_then(|point| point.Position())
        .ok();
    let mut context = NativeEventContext::new()
        .modality(modality)
        .modifiers(modifiers);
    if let Some(position) = position {
        context = context.position(f64::from(position.X), f64::from(position.Y));
    }
    context
}

fn pointer_modality(device_type: PointerDeviceType) -> NativeInputModality {
    match device_type {
        PointerDeviceType::Touch => NativeInputModality::Touch,
        PointerDeviceType::Pen => NativeInputModality::Pen,
        PointerDeviceType::Mouse | PointerDeviceType::Touchpad => NativeInputModality::Mouse,
        _ => NativeInputModality::Unknown,
    }
}

fn pointer_id(args: &PointerRoutedEventArgs) -> u64 {
    args.Pointer()
        .and_then(|pointer| pointer.PointerId())
        .map(u64::from)
        .unwrap_or_default()
}

fn modifiers_from_bits(bits: u32) -> NativeKeyModifiers {
    NativeKeyModifiers::new()
        .control(bits & 1 != 0)
        .alt(bits & 2 != 0)
        .shift(bits & 4 != 0)
        .meta(bits & 8 != 0)
}

pub(super) fn keyboard_event_context(
    message: &MSG,
    kind: NativeEventKind,
    modifiers: &Mutex<NativeKeyModifiers>,
) -> NativeEventContext {
    let mut current = modifiers.lock().map(|value| *value).unwrap_or_default();
    let pressed = kind == NativeEventKind::KeyDown;
    match message.wParam.0 {
        0x10 => current.shift = pressed,
        0x11 => current.control = pressed,
        0x12 => current.alt = pressed,
        0x5B | 0x5C => current.meta = pressed,
        _ => {}
    }
    if let Ok(mut stored) = modifiers.lock() {
        *stored = current;
    }
    NativeEventContext::new()
        .modality(NativeInputModality::Keyboard)
        .modifiers(current)
        .repeat(kind == NativeEventKind::KeyDown && message.lParam.0 & (1 << 30) != 0)
}

pub(super) fn remember_activation_context(
    contexts: &WinUiActivationContexts,
    id: HostNodeId,
    context: NativeEventContext,
) {
    if let Ok(mut contexts) = contexts.lock() {
        contexts.insert(id, context);
    }
}

pub(super) fn take_activation_context(
    contexts: &WinUiActivationContexts,
    id: HostNodeId,
) -> Option<NativeEventContext> {
    contexts.lock().ok()?.remove(&id)
}

pub(super) fn forget_activation_context(contexts: &WinUiActivationContexts, id: HostNodeId) {
    if let Ok(mut contexts) = contexts.lock() {
        contexts.remove(&id);
    }
}

fn update_activation_context_if_present(
    contexts: &WinUiActivationContexts,
    id: HostNodeId,
    context: NativeEventContext,
) {
    if let Ok(mut contexts) = contexts.lock() {
        if let Some(stored) = contexts.get_mut(&id) {
            *stored = context;
        }
    }
}

pub(super) fn clear_pending_activation_contexts(
    contexts: &WinUiActivationContexts,
    pending: &WinUiPendingActivationCleanup,
) {
    let pending = pending
        .lock()
        .map(|mut pending| std::mem::take(&mut *pending))
        .unwrap_or_default();
    if let Ok(mut contexts) = contexts.lock() {
        for id in pending {
            contexts.remove(&id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{PlatformAdapter, WinUiAdapter};
    use crate::web::WebProps;

    fn test_registration(press_lifecycle: bool) -> WinUiInteractionRegistration {
        let mut web = WebProps::new().on_press("activate");
        if press_lifecycle {
            web = web.on_press_start("start");
        }
        let blueprint = WinUiAdapter.blueprint(
            &NativeElement::new("button", NativeRole::Button)
                .with_props(NativeProps::new().web(web)),
        );
        WinUiInteractionRegistration {
            profile: NativeInteractionProfile::from_blueprint(&blueprint),
            native_button: true,
            native_selection_item: false,
        }
    }

    #[test]
    fn pointer_devices_map_to_portable_modalities() {
        assert_eq!(
            pointer_modality(PointerDeviceType::Touch),
            NativeInputModality::Touch
        );
        assert_eq!(
            pointer_modality(PointerDeviceType::Pen),
            NativeInputModality::Pen
        );
        assert_eq!(
            pointer_modality(PointerDeviceType::Mouse),
            NativeInputModality::Mouse
        );
        assert_eq!(
            pointer_modality(PointerDeviceType::Touchpad),
            NativeInputModality::Mouse
        );
    }

    #[test]
    fn virtual_key_modifier_bits_map_to_portable_modifiers() {
        let modifiers = modifiers_from_bits(1 | 2 | 4 | 8);
        assert!(modifiers.control);
        assert!(modifiers.alt);
        assert!(modifiers.shift);
        assert!(modifiers.meta);
    }

    #[test]
    fn keyboard_context_tracks_modifier_release_and_repeat() {
        let modifiers = Mutex::new(NativeKeyModifiers::new());
        let mut message = MSG::default();
        message.wParam.0 = 0x10;
        message.lParam.0 = 1 << 30;

        let down = keyboard_event_context(&message, NativeEventKind::KeyDown, &modifiers);
        assert_eq!(down.modality, NativeInputModality::Keyboard);
        assert!(down.modifiers.shift);
        assert!(down.repeat);

        let up = keyboard_event_context(&message, NativeEventKind::KeyUp, &modifiers);
        assert!(!up.modifiers.shift);
        assert!(!up.repeat);
    }

    #[test]
    fn keyboard_press_source_emits_one_semantic_lifecycle() {
        let node = HostNodeId::new(11);
        let registration = test_registration(true);
        let active = Mutex::new(KeyboardPressState::default());
        let context = NativeEventContext::new().modality(NativeInputModality::Keyboard);

        let down = keyboard_events(
            node,
            "Enter".to_string(),
            NativeEventKind::KeyDown,
            context,
            registration,
            &active,
        );
        assert_eq!(
            down.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![NativeEventKind::PressStart, NativeEventKind::KeyDown]
        );
        assert!(down.iter().all(|event| event.context.handled_activation));
        assert_eq!(active_keyboard_target(&active, "Enter"), Some(node));

        let up = keyboard_events(
            node,
            "Enter".to_string(),
            NativeEventKind::KeyUp,
            context,
            registration,
            &active,
        );
        assert_eq!(
            up.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
                NativeEventKind::KeyUp
            ]
        );
        assert_eq!(active_keyboard_target(&active, "Enter"), None);
    }

    #[test]
    fn keyboard_repeat_does_not_restart_a_press() {
        let node = HostNodeId::new(13);
        let registration = test_registration(false);
        let active = Mutex::new(KeyboardPressState::default());
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Keyboard)
            .repeat(true);

        let events = keyboard_events(
            node,
            " ".to_string(),
            NativeEventKind::KeyDown,
            context,
            registration,
            &active,
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, NativeEventKind::KeyDown);
        assert_eq!(active_keyboard_target(&active, " "), None);
    }

    #[test]
    fn arrow_key_emits_move_lifecycle_before_the_raw_key_event() {
        let node = HostNodeId::new(14);
        let blueprint = WinUiAdapter.blueprint(
            &NativeElement::new("thumb", NativeRole::View).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .event("onMoveStart", "start")
                        .event("onMove", "move")
                        .event("onMoveEnd", "end"),
                ),
            ),
        );
        let registration = WinUiInteractionRegistration {
            profile: NativeInteractionProfile::from_blueprint(&blueprint),
            native_button: false,
            native_selection_item: false,
        };
        let active = Mutex::new(KeyboardPressState::default());
        let context = NativeEventContext::new().modality(NativeInputModality::Keyboard);

        let events = keyboard_events(
            node,
            "ArrowUp".to_string(),
            NativeEventKind::KeyDown,
            context,
            registration,
            &active,
        );

        assert_eq!(
            events.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![
                NativeEventKind::MoveStart,
                NativeEventKind::Move,
                NativeEventKind::MoveEnd,
                NativeEventKind::KeyDown,
            ]
        );
        assert_eq!(
            events[1].context.delta,
            Some(crate::input::NativeEventPosition::new(0.0, -1.0))
        );
    }

    #[test]
    fn pointer_release_updates_only_an_unconsumed_button_context() {
        let node = HostNodeId::new(17);
        let contexts = Arc::new(Mutex::new(BTreeMap::new()));
        let press = NativeEventContext::new()
            .modality(NativeInputModality::Mouse)
            .position(1.0, 2.0);
        let release = NativeEventContext::new()
            .modality(NativeInputModality::Mouse)
            .position(3.0, 4.0);

        remember_activation_context(&contexts, node, press);
        update_activation_context_if_present(&contexts, node, release);
        assert_eq!(take_activation_context(&contexts, node), Some(release));

        update_activation_context_if_present(&contexts, node, press);
        assert_eq!(take_activation_context(&contexts, node), None);
    }

    #[test]
    fn message_boundary_clears_unconsumed_button_contexts() {
        let node = HostNodeId::new(19);
        let contexts = Arc::new(Mutex::new(BTreeMap::new()));
        let pending = Arc::new(Mutex::new(BTreeSet::from([node])));
        remember_activation_context(
            &contexts,
            node,
            NativeEventContext::new().modality(NativeInputModality::Mouse),
        );

        clear_pending_activation_contexts(&contexts, &pending);

        assert_eq!(take_activation_context(&contexts, node), None);
        assert!(pending.lock().is_ok_and(|pending| pending.is_empty()));
    }
}
