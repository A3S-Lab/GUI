use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;
use crate::platform::NativeWidgetBlueprint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeEventKind {
    Press,
    Change,
    SelectionChange,
    Toggle,
    Focus,
    Blur,
    KeyDown,
    KeyUp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeEvent {
    pub node: HostNodeId,
    pub kind: NativeEventKind,
    pub value: Option<String>,
}

impl NativeEvent {
    pub fn new(node: HostNodeId, kind: NativeEventKind) -> Self {
        Self {
            node,
            kind,
            value: None,
        }
    }

    pub fn validate(&self) -> GuiResult<()> {
        if self.node.get() == 0 {
            return Err(GuiError::host(
                "a3s-gui native events need a non-zero node id",
            ));
        }
        Ok(())
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub node: HostNodeId,
    pub action: String,
    pub event: NativeEventKind,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EventRouter;

impl EventRouter {
    pub fn new() -> Self {
        Self
    }

    pub fn route(
        &self,
        blueprint: &NativeWidgetBlueprint,
        event: &NativeEvent,
    ) -> Option<ActionInvocation> {
        let action = action_for_event(blueprint, event)?;
        Some(ActionInvocation {
            node: event.node,
            action: action.to_string(),
            event: event.kind,
            value: event.value.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredAction {
    pub id: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ActionRegistry {
    actions: BTreeMap<String, RegisteredAction>,
    invocations: Vec<ActionInvocation>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: impl Into<String>) {
        let id = id.into();
        self.actions
            .entry(id.clone())
            .or_insert(RegisteredAction { id, label: None });
    }

    pub fn register_labeled(&mut self, id: impl Into<String>, label: impl Into<String>) {
        let id = id.into();
        self.actions.insert(
            id.clone(),
            RegisteredAction {
                id,
                label: Some(label.into()),
            },
        );
    }

    pub fn replace_registered<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = RegisteredAction>,
    {
        self.actions.clear();
        for action in actions {
            self.actions.insert(action.id.clone(), action);
        }
    }

    pub fn contains(&self, id: &str) -> bool {
        self.actions.contains_key(id)
    }

    pub fn registered(&self, id: &str) -> Option<&RegisteredAction> {
        self.actions.get(id)
    }

    pub fn invocations(&self) -> &[ActionInvocation] {
        &self.invocations
    }

    pub fn invoke(&mut self, invocation: ActionInvocation) -> GuiResult<()> {
        if self.contains(&invocation.action) {
            self.invocations.push(invocation);
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "unregistered action {}",
                invocation.action
            )))
        }
    }
}

fn action_for_event<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Option<&'a str> {
    let events = &blueprint.events;
    match event.kind {
        NativeEventKind::Press => press_action(blueprint),
        NativeEventKind::Change => events
            .get("onChange")
            .or(blueprint.action.as_ref())
            .map(String::as_str),
        NativeEventKind::SelectionChange => events
            .get("onSelectionChange")
            .or_else(|| events.get("onChange"))
            .or(blueprint.action.as_ref())
            .map(String::as_str),
        NativeEventKind::Toggle if is_expansion_toggle(blueprint) => events
            .get("onExpandedChange")
            .or_else(|| events.get("onToggle"))
            .or_else(|| events.get("onChange"))
            .or(blueprint.action.as_ref())
            .map(String::as_str),
        NativeEventKind::Toggle => events
            .get("onChange")
            .or_else(|| events.get("onToggle"))
            .or_else(|| events.get("onClick"))
            .or(blueprint.action.as_ref())
            .map(String::as_str),
        NativeEventKind::Focus => events
            .get("onFocus")
            .or_else(|| events.get("onFocusChange"))
            .map(String::as_str),
        NativeEventKind::Blur => events
            .get("onBlur")
            .or_else(|| events.get("onFocusChange"))
            .map(String::as_str),
        NativeEventKind::KeyDown => events
            .get("onKeyDown")
            .map(String::as_str)
            .or_else(|| activation_key_action(blueprint, event)),
        NativeEventKind::KeyUp => events.get("onKeyUp").map(String::as_str),
    }
}

fn press_action(blueprint: &NativeWidgetBlueprint) -> Option<&str> {
    blueprint
        .events
        .get("onPress")
        .or_else(|| blueprint.events.get("onClick"))
        .or(blueprint.action.as_ref())
        .map(String::as_str)
}

fn activation_key_action<'a>(
    blueprint: &'a NativeWidgetBlueprint,
    event: &NativeEvent,
) -> Option<&'a str> {
    if !is_keyboard_activatable(blueprint) || !is_activation_key(event.value.as_deref()) {
        return None;
    }

    press_action(blueprint)
}

fn is_keyboard_activatable(blueprint: &NativeWidgetBlueprint) -> bool {
    matches!(
        blueprint.role,
        crate::native::NativeRole::Button
            | crate::native::NativeRole::Link
            | crate::native::NativeRole::ImageMapArea
            | crate::native::NativeRole::MenuItem
    )
}

pub(crate) fn is_activation_key(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = value.trim();
    normalized.eq_ignore_ascii_case("enter")
        || value == " "
        || normalized.eq_ignore_ascii_case("space")
        || normalized.eq_ignore_ascii_case("spacebar")
}

fn is_expansion_toggle(blueprint: &NativeWidgetBlueprint) -> bool {
    matches!(
        blueprint.role,
        crate::native::NativeRole::Disclosure
            | crate::native::NativeRole::DisclosureSummary
            | crate::native::NativeRole::Popover
    ) || blueprint.control_state.expanded.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{AppKitAdapter, PlatformAdapter};
    use crate::web::WebProps;

    #[test]
    fn routes_native_press_to_web_click_action() {
        let element = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().web(WebProps::new().on_click("saveDocument")));
        let blueprint = AppKitAdapter.blueprint(&element);
        let event = NativeEvent::new(HostNodeId::new(7), NativeEventKind::Press);

        let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

        assert_eq!(invocation.node, HostNodeId::new(7));
        assert_eq!(invocation.action, "saveDocument");
        assert_eq!(invocation.event, NativeEventKind::Press);
    }

    #[test]
    fn routes_native_change_with_value() {
        let element = NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().web(WebProps::new().on_change("setEmail")));
        let blueprint = AppKitAdapter.blueprint(&element);
        let event = NativeEvent::new(HostNodeId::new(9), NativeEventKind::Change).value("a@b.c");

        let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

        assert_eq!(invocation.action, "setEmail");
        assert_eq!(invocation.value.as_deref(), Some("a@b.c"));
    }

    #[test]
    fn routes_native_toggle_with_checked_value_to_web_change_action() {
        let element = NativeElement::new("notifications", NativeRole::Switch)
            .with_props(NativeProps::new().web(WebProps::new().on_change("setNotifications")));
        let blueprint = AppKitAdapter.blueprint(&element);
        let event = NativeEvent::new(HostNodeId::new(10), NativeEventKind::Toggle).value("true");

        let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

        assert_eq!(invocation.action, "setNotifications");
        assert_eq!(invocation.event, NativeEventKind::Toggle);
        assert_eq!(invocation.value.as_deref(), Some("true"));
    }

    #[test]
    fn routes_native_toggle_to_expanded_change_for_disclosure_controls() {
        let element = NativeElement::new("summary", NativeRole::DisclosureSummary).with_props(
            NativeProps::new()
                .expanded(false)
                .web(WebProps::new().event("onExpandedChange", "setOpen")),
        );
        let blueprint = AppKitAdapter.blueprint(&element);
        let event = NativeEvent::new(HostNodeId::new(12), NativeEventKind::Toggle).value("true");

        let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

        assert_eq!(invocation.action, "setOpen");
        assert_eq!(invocation.event, NativeEventKind::Toggle);
        assert_eq!(invocation.value.as_deref(), Some("true"));
    }

    #[test]
    fn routes_native_focus_and_blur_to_focus_change_alias() {
        let element = NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().web(WebProps::new().event("onFocusChange", "setFocus")));
        let blueprint = AppKitAdapter.blueprint(&element);

        let focus = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(13), NativeEventKind::Focus).value("true"),
            )
            .unwrap();
        let blur = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(13), NativeEventKind::Blur).value("false"),
            )
            .unwrap();

        assert_eq!(focus.action, "setFocus");
        assert_eq!(focus.value.as_deref(), Some("true"));
        assert_eq!(blur.action, "setFocus");
        assert_eq!(blur.value.as_deref(), Some("false"));
    }

    #[test]
    fn routes_native_selection_change_to_react_aria_selection_action() {
        let element = NativeElement::new("project", NativeRole::Select)
            .with_props(NativeProps::new().web(WebProps::new().on_selection_change("setProject")));
        let blueprint = AppKitAdapter.blueprint(&element);
        let event =
            NativeEvent::new(HostNodeId::new(11), NativeEventKind::SelectionChange).value("A3S");

        let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

        assert_eq!(invocation.action, "setProject");
        assert_eq!(invocation.event, NativeEventKind::SelectionChange);
        assert_eq!(invocation.value.as_deref(), Some("A3S"));
    }

    #[test]
    fn routes_native_keyboard_events_to_key_actions() {
        let element = NativeElement::new("search", NativeRole::TextField).with_props(
            NativeProps::new().web(
                WebProps::new()
                    .on_key_down("handleKeyDown")
                    .on_key_up("handleKeyUp"),
            ),
        );
        let blueprint = AppKitAdapter.blueprint(&element);

        let key_down = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(14), NativeEventKind::KeyDown).value("Enter"),
            )
            .unwrap();
        let key_up = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(14), NativeEventKind::KeyUp).value("Enter"),
            )
            .unwrap();

        assert_eq!(key_down.action, "handleKeyDown");
        assert_eq!(key_down.event, NativeEventKind::KeyDown);
        assert_eq!(key_down.value.as_deref(), Some("Enter"));
        assert_eq!(key_up.action, "handleKeyUp");
        assert_eq!(key_up.event, NativeEventKind::KeyUp);
        assert_eq!(key_up.value.as_deref(), Some("Enter"));
    }

    #[test]
    fn routes_button_activation_keys_to_primary_action() {
        let element = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().web(WebProps::new().on_press("saveDocument")));
        let blueprint = AppKitAdapter.blueprint(&element);

        let enter = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(15), NativeEventKind::KeyDown).value("Enter"),
            )
            .unwrap();
        let space = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(15), NativeEventKind::KeyDown).value(" "),
            )
            .unwrap();

        assert_eq!(enter.action, "saveDocument");
        assert_eq!(enter.event, NativeEventKind::KeyDown);
        assert_eq!(enter.value.as_deref(), Some("Enter"));
        assert_eq!(space.action, "saveDocument");
        assert_eq!(space.value.as_deref(), Some(" "));
    }

    #[test]
    fn explicit_key_down_action_wins_over_activation_fallback() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new().web(
                WebProps::new()
                    .on_press("saveDocument")
                    .on_key_down("handleShortcut"),
            ),
        );
        let blueprint = AppKitAdapter.blueprint(&element);

        let invocation = EventRouter::new()
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(16), NativeEventKind::KeyDown).value("Enter"),
            )
            .unwrap();

        assert_eq!(invocation.action, "handleShortcut");
        assert_eq!(invocation.event, NativeEventKind::KeyDown);
    }

    #[test]
    fn ignores_non_activation_keys_and_stateful_toggle_keydowns() {
        let button = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().web(WebProps::new().on_press("saveDocument")));
        let switch = NativeElement::new("enabled", NativeRole::Switch)
            .with_props(NativeProps::new().web(WebProps::new().on_change("setEnabled")));
        let button_blueprint = AppKitAdapter.blueprint(&button);
        let switch_blueprint = AppKitAdapter.blueprint(&switch);

        assert!(EventRouter::new()
            .route(
                &button_blueprint,
                &NativeEvent::new(HostNodeId::new(17), NativeEventKind::KeyDown).value("A"),
            )
            .is_none());
        assert!(EventRouter::new()
            .route(
                &switch_blueprint,
                &NativeEvent::new(HostNodeId::new(18), NativeEventKind::KeyDown).value(" "),
            )
            .is_none());
    }

    #[test]
    fn action_registry_records_registered_invocations() {
        let mut registry = ActionRegistry::new();
        registry.register("saveDocument");

        registry
            .invoke(ActionInvocation {
                node: HostNodeId::new(1),
                action: "saveDocument".to_string(),
                event: NativeEventKind::Press,
                value: None,
            })
            .unwrap();

        assert_eq!(registry.invocations().len(), 1);
        assert!(registry
            .invoke(ActionInvocation {
                node: HostNodeId::new(1),
                action: "missingAction".to_string(),
                event: NativeEventKind::Press,
                value: None,
            })
            .is_err());
    }

    #[test]
    fn action_registry_replaces_registered_action_scope() {
        let mut registry = ActionRegistry::new();
        registry.register("saveDocument");
        registry
            .invoke(ActionInvocation {
                node: HostNodeId::new(1),
                action: "saveDocument".to_string(),
                event: NativeEventKind::Press,
                value: None,
            })
            .unwrap();

        registry.replace_registered([RegisteredAction {
            id: "deleteDocument".to_string(),
            label: Some("Delete document".to_string()),
        }]);

        assert!(!registry.contains("saveDocument"));
        assert!(registry.contains("deleteDocument"));
        assert_eq!(registry.invocations().len(), 1);
    }
}
