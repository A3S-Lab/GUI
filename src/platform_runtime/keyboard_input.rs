use crate::event::{native_key_value, NativeEventKind};
use crate::native::NativeRole;
use crate::platform_host::{
    PlatformHostRevision, PlatformKeyEvent, PlatformKeyState, PlatformWheelEvent,
};
use crate::semantic_event::is_press_activation_key;

use super::input::RoutedInput;
use super::interaction::{
    KeyboardPress, KeyboardPressKey, SelfDrawnEventContext, SelfDrawnInteractionSession,
};
use super::interaction_tree::SelfDrawnInteractionTree;

impl SelfDrawnInteractionSession {
    pub(super) fn route_key(
        &mut self,
        event: &PlatformKeyEvent,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        tree: &SelfDrawnInteractionTree,
    ) -> RoutedInput {
        let key = native_key_value(&event.logical_key);
        let mut context = SelfDrawnEventContext::keyboard(
            event.device,
            event.modifiers,
            event.repeat,
            event.timestamp_micros,
        );
        let press_key = KeyboardPressKey {
            device: event.device,
            physical_key: event.physical_key.clone(),
        };
        let mut routed = RoutedInput::default();

        match event.state {
            PlatformKeyState::Pressed => {
                let focused = self.focused.clone();
                routed.target = focused.clone();
                let drag_handled = self.route_keyboard_drag(
                    &key,
                    event.modifiers.shift,
                    event.repeat,
                    tree,
                    frame_revision,
                    event_sequence,
                    &mut context,
                    &mut routed,
                );
                if !drag_handled {
                    if let Some(target) = focused {
                        self.route_keyboard_move(
                            &target,
                            &key,
                            tree,
                            frame_revision,
                            event_sequence,
                            &mut context,
                            &mut routed,
                        );
                        let tracks_press = tree
                            .source(&target)
                            .is_some_and(|source| source.tracks_press());
                        let role = tree.node(&target).map(|node| node.role);
                        let activates = tracks_press
                            && role.is_some_and(|role| self_drawn_activation_key(role, &key));
                        if activates
                            && !event.repeat
                            && !self.keyboard_presses.contains_key(&press_key)
                        {
                            context.handled_activation = true;
                            self.keyboard_presses.insert(
                                press_key.clone(),
                                KeyboardPress {
                                    target: target.clone(),
                                },
                            );
                            self.begin_press(&target, &mut routed.changes);
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &target,
                                NativeEventKind::PressStart,
                                context.clone(),
                                Some("true".to_string()),
                                &mut routed.invocations,
                            );
                        } else if activates {
                            context.handled_activation = true;
                        }
                        self.emit(
                            tree,
                            frame_revision,
                            event_sequence,
                            &target,
                            NativeEventKind::KeyDown,
                            context.clone(),
                            Some(key.clone()),
                            &mut routed.invocations,
                        );
                    }
                }
                if !drag_handled && key == "Tab" && !event.repeat {
                    let next = tree.tab_target(self.focused.as_ref(), event.modifiers.shift);
                    self.transition_focus(
                        tree,
                        frame_revision,
                        event_sequence,
                        next.clone(),
                        &context,
                        &mut routed.invocations,
                        &mut routed.changes,
                    );
                    routed.target = next;
                }
            }
            PlatformKeyState::Released => {
                let active = self.keyboard_presses.remove(&press_key);
                let target = active
                    .as_ref()
                    .map(|press| press.target.clone())
                    .or_else(|| self.focused.clone());
                routed.target = target.clone();
                if let Some(target) = target {
                    if active.is_some() {
                        context.handled_activation = true;
                        for (kind, value) in [
                            (NativeEventKind::PressUp, None),
                            (NativeEventKind::PressEnd, Some("false".to_string())),
                            (NativeEventKind::Press, None),
                        ] {
                            if kind == NativeEventKind::PressEnd {
                                self.end_press(&target, &mut routed.changes);
                            }
                            self.emit(
                                tree,
                                frame_revision,
                                event_sequence,
                                &target,
                                kind,
                                context.clone(),
                                value,
                                &mut routed.invocations,
                            );
                        }
                    }
                    self.emit(
                        tree,
                        frame_revision,
                        event_sequence,
                        &target,
                        NativeEventKind::KeyUp,
                        context,
                        Some(key),
                        &mut routed.invocations,
                    );
                }
            }
        }
        routed
    }

    pub(super) fn route_wheel(
        &self,
        event: &PlatformWheelEvent,
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        tree: &SelfDrawnInteractionTree,
    ) -> RoutedInput {
        let target = tree.hit_test(event.position);
        let mut routed = RoutedInput {
            target: target.clone(),
            ..RoutedInput::default()
        };
        if let Some(target) = target {
            let context = SelfDrawnEventContext::wheel(
                event.device,
                event.modifiers,
                event.position,
                event.delta,
                event.delta_mode,
                event.timestamp_micros,
            );
            self.emit(
                tree,
                frame_revision,
                event_sequence,
                &target,
                NativeEventKind::Wheel,
                context,
                None,
                &mut routed.invocations,
            );
        }
        routed
    }
}

fn self_drawn_activation_key(role: NativeRole, key: &str) -> bool {
    if is_press_activation_key(role, Some(key)) {
        return true;
    }
    matches!(
        role,
        NativeRole::Checkbox | NativeRole::Switch | NativeRole::Radio | NativeRole::Tab
    ) && key == " "
}
