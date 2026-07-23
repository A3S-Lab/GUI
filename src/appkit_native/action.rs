use super::*;

#[derive(Debug, Clone)]
pub(super) struct AppKitActionTargetIvars {
    node: HostNodeId,
    events: Rc<RefCell<Vec<NativeEvent>>>,
    activation_contexts: interaction::AppKitActivationContexts,
    focused_node: Rc<Cell<Option<HostNodeId>>>,
    selection_value: Option<String>,
    terminal_press_only: bool,
    max_length: Cell<Option<u32>>,
    suppress_text_change: Cell<bool>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = AppKitActionTargetIvars]
    #[derive(Debug)]
    pub(super) struct AppKitActionTarget;

    impl AppKitActionTarget {
        #[unsafe(method(a3sGuiPress:))]
        fn press(&self, _sender: &AnyObject) {
            if let Some(value) = &self.ivars().selection_value {
                let context = interaction::take_activation_context(
                    &self.ivars().activation_contexts,
                    self.ivars().node,
                );
                if context.is_some_and(|context| context.handled_activation) {
                    return;
                }
                let mut event =
                    NativeEvent::new(self.ivars().node, NativeEventKind::SelectionChange)
                        .value(value.clone());
                if let Some(context) = context {
                    event.context = context;
                }
                self.ivars().events.borrow_mut().push(event);
                return;
            }

            if self.ivars().terminal_press_only {
                self.ivars()
                    .events
                    .borrow_mut()
                    .extend(uncontextualized_press_events(self.ivars().node, true));
                return;
            }

            match interaction::take_activation_context(
                &self.ivars().activation_contexts,
                self.ivars().node,
            ) {
                Some(context) if context.handled_activation => {}
                Some(context)
                    if context.modality != crate::input::NativeInputModality::Unknown =>
                {
                    self.ivars().events.borrow_mut().push(
                        NativeEvent::new(self.ivars().node, NativeEventKind::Press)
                            .context(context),
                    );
                }
                Some(_) | None => self
                    .ivars()
                    .events
                    .borrow_mut()
                    .extend(uncontextualized_press_events(self.ivars().node, false)),
            }
        }

        #[unsafe(method(a3sGuiToggle:))]
        fn toggle(&self, sender: &AnyObject) {
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Toggle)
                    .value(control_checked_value(sender).to_string()),
            );
        }

        #[unsafe(method(a3sGuiChange:))]
        fn change(&self, sender: &AnyObject) {
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Change)
                    .value(control_double_value(sender).to_string()),
            );
        }
    }

    unsafe impl NSObjectProtocol for AppKitActionTarget {}

    unsafe impl NSControlTextEditingDelegate for AppKitActionTarget {
        #[unsafe(method(controlTextDidBeginEditing:))]
        fn control_text_did_begin_editing(&self, _notification: &NSNotification) {
            self.ivars().focused_node.set(Some(self.ivars().node));
        }

        #[unsafe(method(controlTextDidChange:))]
        fn control_text_did_change(&self, notification: &NSNotification) {
            if self.ivars().suppress_text_change.get() {
                return;
            }

            let value = notification
                .object()
                .and_then(|object| object.downcast::<NSControl>().ok())
                .map(|control| {
                    let raw_value = control.stringValue().to_string();
                    let value = truncate_to_max_length(&raw_value, self.max_length());
                    if value != raw_value {
                        self.ivars().suppress_text_change.set(true);
                        control.setStringValue(&ns_string(&value));
                        self.ivars().suppress_text_change.set(false);
                    }
                    value
                })
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::Change).value(value),
            );
        }

        #[unsafe(method(controlTextDidEndEditing:))]
        fn control_text_did_end_editing(&self, _notification: &NSNotification) {
            if self.ivars().focused_node.get() == Some(self.ivars().node) {
                self.ivars().focused_node.set(None);
            }
        }
    }

    unsafe impl NSTextFieldDelegate for AppKitActionTarget {}

    unsafe impl NSSearchFieldDelegate for AppKitActionTarget {}

    unsafe impl NSComboBoxDelegate for AppKitActionTarget {
        #[unsafe(method(comboBoxSelectionDidChange:))]
        fn combo_box_selection_did_change(&self, notification: &NSNotification) {
            let value = notification
                .object()
                .and_then(|object| object.downcast::<NSComboBox>().ok())
                .map(|combo_box| combo_box_selected_value(&combo_box))
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::SelectionChange)
                    .value(value),
            );
        }
    }

    unsafe impl NSTabViewDelegate for AppKitActionTarget {
        #[unsafe(method(tabView:didSelectTabViewItem:))]
        fn tab_view_did_select_tab_view_item(
            &self,
            _tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        ) {
            let value = tab_view_item
                .map(|item| item.label().to_string())
                .unwrap_or_default();
            self.ivars().events.borrow_mut().push(
                NativeEvent::new(self.ivars().node, NativeEventKind::SelectionChange)
                    .value(value),
            );
        }
    }
);

impl AppKitActionTarget {
    pub(super) fn new(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        activation_contexts: interaction::AppKitActivationContexts,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            activation_contexts,
            focused_node,
            selection_value: None,
            terminal_press_only: false,
            max_length: Cell::new(None),
            suppress_text_change: Cell::new(false),
        });
        unsafe { msg_send![super(this), init] }
    }

    pub(super) fn new_terminal_press(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        activation_contexts: interaction::AppKitActivationContexts,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            activation_contexts,
            focused_node,
            selection_value: None,
            terminal_press_only: true,
            max_length: Cell::new(None),
            suppress_text_change: Cell::new(false),
        });
        unsafe { msg_send![super(this), init] }
    }

    pub(super) fn new_selection(
        node: HostNodeId,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        activation_contexts: interaction::AppKitActivationContexts,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        selection_value: String,
        mtm: MainThreadMarker,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AppKitActionTargetIvars {
            node,
            events,
            activation_contexts,
            focused_node,
            selection_value: Some(selection_value),
            terminal_press_only: false,
            max_length: Cell::new(None),
            suppress_text_change: Cell::new(false),
        });
        unsafe { msg_send![super(this), init] }
    }

    pub(super) fn as_any_object(&self) -> &AnyObject {
        self.as_super().as_super()
    }

    pub(super) fn max_length(&self) -> Option<u32> {
        self.ivars().max_length.get()
    }

    pub(super) fn set_max_length(&self, max_length: Option<u32>) {
        self.ivars().max_length.set(max_length);
    }
}

fn uncontextualized_press_events(node: HostNodeId, terminal_press_only: bool) -> Vec<NativeEvent> {
    if terminal_press_only {
        vec![NativeEvent::new(node, NativeEventKind::Press)]
    } else {
        virtual_press_events(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_fallback_does_not_invent_a_virtual_press_lifecycle() {
        let events = uncontextualized_press_events(HostNodeId::new(7), true);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, NativeEventKind::Press);
        assert_eq!(
            events[0].context.modality,
            crate::input::NativeInputModality::Unknown
        );
    }

    #[test]
    fn ordinary_control_fallback_remains_a_virtual_press_lifecycle() {
        let events = uncontextualized_press_events(HostNodeId::new(8), false);
        assert_eq!(
            events.iter().map(|event| event.kind).collect::<Vec<_>>(),
            vec![
                NativeEventKind::PressStart,
                NativeEventKind::PressUp,
                NativeEventKind::PressEnd,
                NativeEventKind::Press,
            ]
        );
        assert!(events
            .iter()
            .all(|event| { event.context.modality == crate::input::NativeInputModality::Virtual }));
    }
}
