use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use gtk::prelude::*;

use super::{gtk, gtk_key_value, push_event, Gtk4OsWidget};
use crate::event::{
    keyboard_move_events, virtual_press_events, KeyboardPressState, NativeEvent, NativeEventKind,
    NativeInteractionProfile, NativeLongPressTimer, PointerMoveState, PointerPressState,
};
use crate::host::HostNodeId;
use crate::input::{NativeEventContext, NativeInputModality, NativeKeyModifiers};
use crate::platform::{NativeWidgetBlueprint, NativeWidgetSetter};

type Gtk4EventQueue = Rc<RefCell<Vec<NativeEvent>>>;
type Gtk4EventsSuppressed = Rc<RefCell<bool>>;
type Gtk4ActivationContexts = Rc<RefCell<BTreeMap<HostNodeId, NativeEventContext>>>;
type Gtk4InteractionNode = Rc<Cell<Gtk4InteractionRegistration>>;
pub(super) type Gtk4InteractionNodes = Rc<RefCell<BTreeMap<HostNodeId, Gtk4InteractionNode>>>;
pub(super) type Gtk4KeyboardPresses = Rc<RefCell<KeyboardPressState>>;

#[derive(Debug, Clone, Copy)]
pub(super) struct Gtk4InteractionRegistration {
    profile: NativeInteractionProfile,
    native_button: bool,
}

impl Gtk4InteractionRegistration {
    pub(super) fn new(widget: &Gtk4OsWidget, blueprint: &NativeWidgetBlueprint) -> Self {
        Self {
            profile: NativeInteractionProfile::from_blueprint(blueprint),
            native_button: matches!(widget, Gtk4OsWidget::Button(_)),
        }
    }

    pub(super) fn apply_setter(&mut self, setter: &NativeWidgetSetter) {
        self.profile.apply_setter(setter);
    }

    fn normalizes_keyboard_press(self) -> bool {
        self.profile.normalizes_keyboard_press()
    }
}

pub(super) fn register_interaction_events(
    id: HostNodeId,
    widget: &Gtk4OsWidget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    activation_contexts: &Gtk4ActivationContexts,
    interaction_nodes: &Gtk4InteractionNodes,
    keyboard_presses: &Gtk4KeyboardPresses,
    registration: Gtk4InteractionNode,
) {
    let Some(gtk_widget) = widget.as_widget() else {
        return;
    };

    register_focus_events(id, &gtk_widget, events, events_suppressed);
    register_key_events(
        id,
        &gtk_widget,
        events,
        events_suppressed,
        activation_contexts,
        interaction_nodes,
        keyboard_presses,
        Rc::clone(&registration),
    );
    register_number_field_wheel(
        id,
        &gtk_widget,
        events,
        events_suppressed,
        Rc::clone(&registration),
    );
    register_pointer_events(
        id,
        &gtk_widget,
        events,
        events_suppressed,
        activation_contexts,
        Rc::clone(&registration),
    );
    if let Gtk4OsWidget::Button(button) = widget {
        register_button_click(
            id,
            button,
            events,
            events_suppressed,
            activation_contexts,
            registration,
        );
    }
}

fn register_focus_events(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
) {
    let controller = gtk::EventControllerFocus::new();
    let focus_events = Rc::clone(events);
    let focus_suppressed = Rc::clone(events_suppressed);
    controller.connect_is_focus_notify(move |controller| {
        let kind = if controller.is_focus() {
            NativeEventKind::Focus
        } else {
            NativeEventKind::Blur
        };
        push_event(&focus_events, &focus_suppressed, NativeEvent::new(id, kind));
    });
    widget.add_controller(controller);
}

#[allow(clippy::too_many_arguments)]
fn register_key_events(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    activation_contexts: &Gtk4ActivationContexts,
    interaction_nodes: &Gtk4InteractionNodes,
    keyboard_presses: &Gtk4KeyboardPresses,
    registration: Gtk4InteractionNode,
) {
    let controller = gtk::EventControllerKey::new();
    controller.set_propagation_phase(gtk::PropagationPhase::Capture);

    let down_events = Rc::clone(events);
    let down_suppressed = Rc::clone(events_suppressed);
    let down_contexts = Rc::clone(activation_contexts);
    let down_presses = Rc::clone(keyboard_presses);
    let down_registration = Rc::clone(&registration);
    controller.connect_key_pressed(move |controller, key, keycode, modifiers| {
        if !controller_targets_focused_widget(controller) || *down_suppressed.borrow() {
            return gtk::glib::Propagation::Proceed;
        }

        let key = gtk_key_value(key, keycode);
        let repeat = down_presses.borrow().target_for_key(&key).is_some();
        let context = keyboard_context(modifiers, repeat);
        let current = down_registration.get();
        let number_field_step_handled = current.profile.handles_number_field_step_key(
            NativeEventKind::KeyDown,
            &key,
            context.modifiers,
        );
        if current.native_button {
            remember_activation_context(&down_contexts, id, context);
        }
        let mut pending = if current.profile.tracks_movement() {
            keyboard_move_events(id, &key, context)
        } else {
            Vec::new()
        };
        let movement_handled = !pending.is_empty();
        pending.extend(down_presses.borrow_mut().events(
            id,
            key,
            NativeEventKind::KeyDown,
            context,
            current.profile.role,
            current.profile.subscriptions.tracks_press(),
        ));
        push_events(&down_events, &down_suppressed, pending);
        if movement_handled || number_field_step_handled {
            gtk::glib::Propagation::Stop
        } else {
            gtk::glib::Propagation::Proceed
        }
    });

    let up_events = Rc::clone(events);
    let up_suppressed = Rc::clone(events_suppressed);
    let up_contexts = Rc::clone(activation_contexts);
    let up_nodes = Rc::clone(interaction_nodes);
    let up_presses = Rc::clone(keyboard_presses);
    controller.connect_key_released(move |controller, key, keycode, modifiers| {
        if !controller_targets_focused_widget(controller) || *up_suppressed.borrow() {
            return;
        }

        let key = gtk_key_value(key, keycode);
        let target = up_presses.borrow().target_for_key(&key).unwrap_or(id);
        let target_registration = up_nodes
            .borrow()
            .get(&target)
            .map(|registration| registration.get())
            .unwrap_or_else(|| registration.get());
        let context = keyboard_context(modifiers, false);
        if target_registration.native_button {
            remember_activation_context(&up_contexts, target, context);
        }
        let pending = up_presses.borrow_mut().events(
            target,
            key,
            NativeEventKind::KeyUp,
            context,
            target_registration.profile.role,
            target_registration.profile.subscriptions.tracks_press(),
        );
        push_events(&up_events, &up_suppressed, pending);
    });
    widget.add_controller(controller);
}

fn register_number_field_wheel(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    registration: Gtk4InteractionNode,
) {
    let controller = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::BOTH_AXES);
    controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    let wheel_events = Rc::clone(events);
    let wheel_suppressed = Rc::clone(events_suppressed);
    controller.connect_scroll(move |controller, delta_x, delta_y| {
        if *wheel_suppressed.borrow() {
            return gtk::glib::Propagation::Proceed;
        }
        let modifiers = gtk_modifiers(controller.current_event_state());
        let focused = controller.widget().is_some_and(|widget| widget.has_focus());
        let profile = registration.get().profile;
        if !profile.handles_number_field_wheel(focused, delta_x, delta_y, modifiers) {
            return gtk::glib::Propagation::Proceed;
        }

        let context = pointer_context(controller, None).delta(delta_x, delta_y);
        push_event(
            &wheel_events,
            &wheel_suppressed,
            NativeEvent::new(id, NativeEventKind::Wheel).context(context),
        );
        gtk::glib::Propagation::Stop
    });
    widget.add_controller(controller);
}

fn controller_targets_focused_widget(controller: &gtk::EventControllerKey) -> bool {
    controller.widget().is_some_and(|widget| widget.has_focus())
}

fn register_pointer_events(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    activation_contexts: &Gtk4ActivationContexts,
    registration: Gtk4InteractionNode,
) {
    let state = Rc::new(RefCell::new(PointerPressState::default()));
    let move_state = Rc::new(RefCell::new(PointerMoveState::default()));
    let gesture = gtk::GestureClick::new();
    gesture.set_button(1);
    gesture.set_propagation_phase(gtk::PropagationPhase::Capture);

    let press_events = Rc::clone(events);
    let press_suppressed = Rc::clone(events_suppressed);
    let press_contexts = Rc::clone(activation_contexts);
    let press_state = Rc::clone(&state);
    let press_move_state = Rc::clone(&move_state);
    let press_registration = Rc::clone(&registration);
    gesture.connect_pressed(move |gesture, n_press, x, y| {
        if *press_suppressed.borrow() {
            return;
        }
        let current = press_registration.get();
        if !current.native_button && !current.profile.subscriptions.tracks_press() {
            return;
        }
        let context = pointer_context(gesture, Some((x, y)))
            .click_count(u8::try_from(n_press).unwrap_or(u8::MAX));
        if current.native_button {
            remember_activation_context(&press_contexts, id, context);
        }
        let (pending, timer) =
            if current.native_button || current.profile.subscriptions.tracks_press() {
                let mut state = press_state.borrow_mut();
                let pending =
                    state.begin_with_long_press(id, context, current.profile.long_press_config());
                let timer = state.take_long_press_timer(id, context);
                (pending, timer)
            } else {
                (Vec::new(), None)
            };
        push_events(&press_events, &press_suppressed, pending);
        if let Some(timer) = timer {
            schedule_long_press(
                timer,
                Rc::clone(&press_events),
                Rc::clone(&press_suppressed),
                Rc::clone(&press_move_state),
            );
        }
    });

    let release_events = Rc::clone(events);
    let release_suppressed = Rc::clone(events_suppressed);
    let release_contexts = Rc::clone(activation_contexts);
    let release_state = Rc::clone(&state);
    let release_registration = Rc::clone(&registration);
    let release_widget = widget.clone();
    gesture.connect_released(move |gesture, n_press, x, y| {
        if *release_suppressed.borrow() {
            return;
        }
        let current = release_registration.get();
        let context = pointer_context(gesture, Some((x, y)))
            .click_count(u8::try_from(n_press).unwrap_or(u8::MAX));
        let over_target = point_is_inside(&release_widget, x, y);
        let mut state = release_state.borrow_mut();
        let boundary = if over_target {
            state.enter(id, context)
        } else {
            state.leave(id, context)
        };
        push_events(&release_events, &release_suppressed, boundary);
        let emit_press = !current.native_button && current.profile.subscriptions.terminal_press;
        let recognized = state.long_press_recognized();
        let pending = state.release(id, context, emit_press);
        let long_pressed = recognized
            || pending
                .iter()
                .any(|event| event.kind == NativeEventKind::LongPress);
        drop(state);
        push_events(&release_events, &release_suppressed, pending);

        if current.native_button && over_target {
            remember_activation_context(
                &release_contexts,
                id,
                context.handled_activation(long_pressed),
            );
        } else {
            forget_activation_context(&release_contexts, id);
        }
    });

    let cancel_events = Rc::clone(events);
    let cancel_suppressed = Rc::clone(events_suppressed);
    let cancel_contexts = Rc::clone(activation_contexts);
    let cancel_state = Rc::clone(&state);
    gesture.connect_cancel(move |gesture, _| {
        cancel_pointer_press(
            id,
            gesture,
            &cancel_events,
            &cancel_suppressed,
            &cancel_contexts,
            &cancel_state,
        );
    });

    let stopped_events = Rc::clone(events);
    let stopped_suppressed = Rc::clone(events_suppressed);
    let stopped_contexts = Rc::clone(activation_contexts);
    let stopped_state = Rc::clone(&state);
    gesture.connect_stopped(move |gesture| {
        cancel_pointer_press(
            id,
            gesture,
            &stopped_events,
            &stopped_suppressed,
            &stopped_contexts,
            &stopped_state,
        );
    });
    register_pointer_move_events(
        id,
        widget,
        events,
        events_suppressed,
        Rc::clone(&registration),
        Rc::clone(&move_state),
        &gesture,
    );
    widget.add_controller(gesture);

    register_pointer_boundary_events(
        id,
        widget,
        events,
        events_suppressed,
        state,
        move_state,
        registration,
    );
}

fn register_pointer_move_events(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    registration: Gtk4InteractionNode,
    state: Rc<RefCell<PointerMoveState>>,
    click: &gtk::GestureClick,
) {
    let origin = Rc::new(Cell::new(None::<(f64, f64)>));
    let drag = gtk::GestureDrag::new();
    drag.set_button(1);
    drag.set_propagation_phase(gtk::PropagationPhase::Capture);
    drag.group_with(click);

    let begin_state = Rc::clone(&state);
    let begin_origin = Rc::clone(&origin);
    let begin_registration = Rc::clone(&registration);
    drag.connect_drag_begin(move |drag, x, y| {
        let context = pointer_context(drag, Some((x, y)));
        if begin_registration.get().profile.tracks_movement() {
            begin_origin.set(Some((x, y)));
            begin_state.borrow_mut().begin(context);
        } else {
            begin_origin.set(None);
            begin_state.borrow_mut().cancel(id, context);
        }
    });

    let update_events = Rc::clone(events);
    let update_suppressed = Rc::clone(events_suppressed);
    let update_state = Rc::clone(&state);
    let update_origin = Rc::clone(&origin);
    drag.connect_drag_update(move |drag, offset_x, offset_y| {
        let Some((start_x, start_y)) = update_origin.get() else {
            return;
        };
        let context = pointer_context(drag, Some((start_x + offset_x, start_y + offset_y)));
        let pending = update_state.borrow_mut().update(id, context);
        push_events(&update_events, &update_suppressed, pending);
    });

    let end_events = Rc::clone(events);
    let end_suppressed = Rc::clone(events_suppressed);
    let end_state = Rc::clone(&state);
    let end_origin = Rc::clone(&origin);
    drag.connect_drag_end(move |drag, offset_x, offset_y| {
        let Some((start_x, start_y)) = end_origin.take() else {
            return;
        };
        let context = pointer_context(drag, Some((start_x + offset_x, start_y + offset_y)));
        let pending = end_state.borrow_mut().end(id, context);
        push_events(&end_events, &end_suppressed, pending);
    });

    let cancel_events = Rc::clone(events);
    let cancel_suppressed = Rc::clone(events_suppressed);
    drag.connect_cancel(move |drag, _| {
        origin.set(None);
        let context = pointer_context(drag, None);
        let pending = state.borrow_mut().cancel(id, context);
        push_events(&cancel_events, &cancel_suppressed, pending);
    });

    widget.add_controller(drag);
}

fn register_pointer_boundary_events(
    id: HostNodeId,
    widget: &gtk::Widget,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    state: Rc<RefCell<PointerPressState>>,
    move_state: Rc<RefCell<PointerMoveState>>,
    registration: Gtk4InteractionNode,
) {
    let controller = gtk::EventControllerMotion::new();
    controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    let hover_context = Rc::new(Cell::new(None::<NativeEventContext>));

    let enter_events = Rc::clone(events);
    let enter_suppressed = Rc::clone(events_suppressed);
    let enter_state = Rc::clone(&state);
    let enter_registration = Rc::clone(&registration);
    let enter_hover_context = Rc::clone(&hover_context);
    let enter_move_state = Rc::clone(&move_state);
    controller.connect_enter(move |controller, x, y| {
        if *enter_suppressed.borrow() {
            return;
        }
        let current = enter_registration.get();
        let context = pointer_context(controller, Some((x, y)));
        if current.profile.subscriptions.tracks_press() || current.native_button {
            let (pending, timer) = {
                let mut state = enter_state.borrow_mut();
                let pending = state.enter(id, context);
                let timer = state.take_long_press_timer(id, context);
                (pending, timer)
            };
            push_events(&enter_events, &enter_suppressed, pending);
            if let Some(timer) = timer {
                schedule_long_press(
                    timer,
                    Rc::clone(&enter_events),
                    Rc::clone(&enter_suppressed),
                    Rc::clone(&enter_move_state),
                );
            }
        }
        if current.profile.subscriptions.hover && context.modality.supports_hover() {
            enter_hover_context.set(Some(context));
            push_event(
                &enter_events,
                &enter_suppressed,
                NativeEvent::new(id, NativeEventKind::HoverStart).context(context),
            );
        }
    });

    let leave_events = Rc::clone(events);
    let leave_suppressed = Rc::clone(events_suppressed);
    let leave_state = state;
    let leave_registration = registration;
    controller.connect_leave(move |controller| {
        if *leave_suppressed.borrow() {
            return;
        }
        let current = leave_registration.get();
        let observed = pointer_context(controller, None);
        let context = if observed.modality.supports_hover() {
            observed
        } else {
            hover_context.get().unwrap_or(observed)
        };
        if current.profile.subscriptions.tracks_press() || current.native_button {
            let pending = leave_state.borrow_mut().leave(id, context);
            push_events(&leave_events, &leave_suppressed, pending);
        }
        if current.profile.subscriptions.hover && hover_context.take().is_some() {
            push_event(
                &leave_events,
                &leave_suppressed,
                NativeEvent::new(id, NativeEventKind::HoverEnd).context(context),
            );
        }
    });
    widget.add_controller(controller);
}

fn register_button_click(
    id: HostNodeId,
    button: &gtk::Button,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    activation_contexts: &Gtk4ActivationContexts,
    registration: Gtk4InteractionNode,
) {
    let events = Rc::clone(events);
    let events_suppressed = Rc::clone(events_suppressed);
    let activation_contexts = Rc::clone(activation_contexts);
    button.connect_clicked(move |_| {
        if *events_suppressed.borrow() {
            return;
        }
        let current = registration.get();
        match take_activation_context(&activation_contexts, id) {
            Some(context)
                if context.handled_activation
                    || (context.modality == NativeInputModality::Keyboard
                        && current.normalizes_keyboard_press()) => {}
            Some(context) if context.modality != NativeInputModality::Unknown => push_event(
                &events,
                &events_suppressed,
                NativeEvent::new(id, NativeEventKind::Press).context(context),
            ),
            Some(_) | None => push_events(&events, &events_suppressed, virtual_press_events(id)),
        }
    });
}

fn cancel_pointer_press<C: IsA<gtk::EventController>>(
    id: HostNodeId,
    controller: &C,
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    activation_contexts: &Gtk4ActivationContexts,
    state: &Rc<RefCell<PointerPressState>>,
) {
    forget_activation_context(activation_contexts, id);
    let context = pointer_context(controller, None);
    let pending = state.borrow_mut().cancel(id, context);
    push_events(events, events_suppressed, pending);
}

fn schedule_long_press(
    timer: NativeLongPressTimer,
    events: Gtk4EventQueue,
    events_suppressed: Gtk4EventsSuppressed,
    movement: Rc<RefCell<PointerMoveState>>,
) {
    gtk::glib::timeout_add_local_once(timer.threshold(), move || {
        if let Some(recognition) = timer.try_fire() {
            let pending = recognition.into_events_with_movement(&mut movement.borrow_mut());
            push_events(&events, &events_suppressed, pending);
        }
    });
}

fn push_events(
    events: &Gtk4EventQueue,
    events_suppressed: &Gtk4EventsSuppressed,
    pending: Vec<NativeEvent>,
) {
    for event in pending {
        push_event(events, events_suppressed, event);
    }
}

fn pointer_context<C: IsA<gtk::EventController>>(
    controller: &C,
    position: Option<(f64, f64)>,
) -> NativeEventContext {
    let modality = controller
        .current_event_device()
        .map(|device| input_source_modality(device.source()))
        .unwrap_or_default();
    let mut context = NativeEventContext::new()
        .modality(modality)
        .modifiers(gtk_modifiers(controller.current_event_state()));
    if let Some((x, y)) = position {
        context = context.position(x, y);
    }
    context
}

fn keyboard_context(modifiers: gtk::gdk::ModifierType, repeat: bool) -> NativeEventContext {
    NativeEventContext::new()
        .modality(NativeInputModality::Keyboard)
        .modifiers(gtk_modifiers(modifiers))
        .repeat(repeat)
}

fn input_source_modality(source: gtk::gdk::InputSource) -> NativeInputModality {
    match source {
        gtk::gdk::InputSource::Mouse
        | gtk::gdk::InputSource::Touchpad
        | gtk::gdk::InputSource::Trackpoint => NativeInputModality::Mouse,
        gtk::gdk::InputSource::Pen | gtk::gdk::InputSource::TabletPad => NativeInputModality::Pen,
        gtk::gdk::InputSource::Touchscreen => NativeInputModality::Touch,
        gtk::gdk::InputSource::Keyboard => NativeInputModality::Keyboard,
        _ => NativeInputModality::Unknown,
    }
}

fn gtk_modifiers(modifiers: gtk::gdk::ModifierType) -> NativeKeyModifiers {
    NativeKeyModifiers::new()
        .alt(modifiers.contains(gtk::gdk::ModifierType::ALT_MASK))
        .control(modifiers.contains(gtk::gdk::ModifierType::CONTROL_MASK))
        .meta(
            modifiers
                .intersects(gtk::gdk::ModifierType::META_MASK | gtk::gdk::ModifierType::SUPER_MASK),
        )
        .shift(modifiers.contains(gtk::gdk::ModifierType::SHIFT_MASK))
}

fn point_is_inside(widget: &gtk::Widget, x: f64, y: f64) -> bool {
    x >= 0.0 && y >= 0.0 && x <= f64::from(widget.width()) && y <= f64::from(widget.height())
}

fn remember_activation_context(
    contexts: &Gtk4ActivationContexts,
    id: HostNodeId,
    context: NativeEventContext,
) {
    contexts.borrow_mut().insert(id, context);
    let contexts = Rc::downgrade(contexts);
    gtk::glib::idle_add_local_once(move || {
        if let Some(contexts) = contexts.upgrade() {
            contexts.borrow_mut().remove(&id);
        }
    });
}

fn take_activation_context(
    contexts: &Gtk4ActivationContexts,
    id: HostNodeId,
) -> Option<NativeEventContext> {
    contexts.borrow_mut().remove(&id)
}

pub(super) fn forget_activation_context(contexts: &Gtk4ActivationContexts, id: HostNodeId) {
    contexts.borrow_mut().remove(&id);
}
