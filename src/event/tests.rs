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
fn routes_each_long_press_phase_to_its_distinct_callback() {
    let element = NativeElement::new("menu", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .event("onLongPressStart", "startMenuPress")
                .event("onLongPress", "openMenu")
                .event("onLongPressEnd", "endMenuPress"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();
    let node = HostNodeId::new(70);

    for (kind, action) in [
        (NativeEventKind::LongPressStart, "startMenuPress"),
        (NativeEventKind::LongPress, "openMenu"),
        (NativeEventKind::LongPressEnd, "endMenuPress"),
    ] {
        let invocation = router
            .route(&blueprint, &NativeEvent::new(node, kind))
            .unwrap();
        assert_eq!(invocation.action, action);
        assert_eq!(invocation.event, kind);
    }
}

#[test]
fn routes_each_move_phase_and_preserves_incremental_delta() {
    let element = NativeElement::new("thumb", NativeRole::View).with_props(
        NativeProps::new().web(
            WebProps::new()
                .event("onMoveStart", "startMove")
                .event("onMove", "moveThumb")
                .event("onMoveEnd", "endMove"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();
    let node = HostNodeId::new(71);

    for (kind, action) in [
        (NativeEventKind::MoveStart, "startMove"),
        (NativeEventKind::Move, "moveThumb"),
        (NativeEventKind::MoveEnd, "endMove"),
    ] {
        let event = NativeEvent::new(node, kind)
            .modality(NativeInputModality::Touch)
            .delta(2.5, -1.0);
        let invocation = router.route(&blueprint, &event).unwrap();
        assert_eq!(invocation.action, action);
        assert_eq!(invocation.event, kind);
        assert_eq!(
            invocation.context.delta,
            Some(crate::input::NativeEventPosition::new(2.5, -1.0))
        );
    }
}

#[test]
fn ignores_empty_action_ids_and_uses_non_empty_fallbacks() {
    let empty = NativeElement::new("empty", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().on_press("")));
    let fallback = NativeElement::new("fallback", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().on_press("").on_click("saveDocument")));
    let empty_blueprint = AppKitAdapter.blueprint(&empty);
    let fallback_blueprint = AppKitAdapter.blueprint(&fallback);
    let event = NativeEvent::new(HostNodeId::new(8), NativeEventKind::Press);

    assert!(EventRouter::new().route(&empty_blueprint, &event).is_none());
    let invocation = EventRouter::new()
        .route(&fallback_blueprint, &event)
        .unwrap();

    assert_eq!(invocation.action, "saveDocument");
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
fn routes_static_action_value_when_native_event_has_no_value() {
    let element = NativeElement::new("alpha", NativeRole::Button).with_props(
        NativeProps::new()
            .metadata("actionValue", "alpha")
            .web(WebProps::new().on_press("selectItem")),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let event = NativeEvent::new(HostNodeId::new(17), NativeEventKind::Press);

    let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

    assert_eq!(invocation.action, "selectItem");
    assert_eq!(invocation.value.as_deref(), Some("alpha"));
}

#[test]
fn native_event_value_wins_over_static_action_value() {
    let element = NativeElement::new("email", NativeRole::TextField).with_props(
        NativeProps::new()
            .metadata("actionValue", "fallback")
            .web(WebProps::new().on_input("setEmail")),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let event =
        NativeEvent::new(HostNodeId::new(18), NativeEventKind::Change).value("current@example.com");

    let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

    assert_eq!(invocation.action, "setEmail");
    assert_eq!(invocation.value.as_deref(), Some("current@example.com"));
}

#[derive(Debug, PartialEq, Deserialize)]
struct ItemPayload {
    id: String,
    title: String,
}

#[test]
fn action_invocation_decodes_json_and_string_payloads() {
    let json = ActionInvocation {
        node: HostNodeId::new(19),
        current_target: None,
        action: "selectItem".to_string(),
        event: NativeEventKind::Press,
        context: Default::default(),
        value: Some(r#"{"id":"alpha","title":"Alpha"}"#.to_string()),
    };
    let string = ActionInvocation {
        node: HostNodeId::new(20),
        current_target: None,
        action: "selectItem".to_string(),
        event: NativeEventKind::Press,
        context: Default::default(),
        value: Some("alpha".to_string()),
    };

    assert_eq!(
        json.payload::<ItemPayload>().unwrap(),
        Some(ItemPayload {
            id: "alpha".to_string(),
            title: "Alpha".to_string()
        })
    );
    assert_eq!(
        string.payload::<String>().unwrap().as_deref(),
        Some("alpha")
    );
    assert_eq!(
        string.payload_json().unwrap(),
        Some(JsonValue::String("alpha".to_string()))
    );
}

#[test]
fn routes_native_change_to_input_event_alias() {
    let input_only = NativeElement::new("email", NativeRole::TextField)
        .with_props(NativeProps::new().web(WebProps::new().on_input("setEmailInput")));
    let change_wins = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_input("setNameInput")
                .on_change("setName"),
        ),
    );
    let event = NativeEvent::new(HostNodeId::new(19), NativeEventKind::Change).value("Ada");

    let input_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&input_only), &event)
        .unwrap();
    let change_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&change_wins), &event)
        .unwrap();

    assert_eq!(input_invocation.action, "setEmailInput");
    assert_eq!(input_invocation.value.as_deref(), Some("Ada"));
    assert_eq!(change_invocation.action, "setName");
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
fn routes_native_toggle_to_input_event_alias_for_value_toggles() {
    let input_only = NativeElement::new("notifications", NativeRole::Switch)
        .with_props(NativeProps::new().web(WebProps::new().on_input("setNotificationsInput")));
    let change_wins = NativeElement::new("accepted", NativeRole::Checkbox).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_input("setAcceptedInput")
                .on_change("setAccepted"),
        ),
    );
    let event = NativeEvent::new(HostNodeId::new(20), NativeEventKind::Toggle).value("true");

    let input_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&input_only), &event)
        .unwrap();
    let change_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&change_wins), &event)
        .unwrap();

    assert_eq!(input_invocation.action, "setNotificationsInput");
    assert_eq!(input_invocation.value.as_deref(), Some("true"));
    assert_eq!(change_invocation.action, "setAccepted");
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
fn routes_native_selection_change_to_semantic_ui_selection_action() {
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
fn routes_native_selection_change_to_input_event_alias() {
    let input_only = NativeElement::new("project", NativeRole::Select)
        .with_props(NativeProps::new().web(WebProps::new().on_input("setProjectInput")));
    let change_wins = NativeElement::new("assignee", NativeRole::Select).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_input("setAssigneeInput")
                .on_change("setAssignee"),
        ),
    );
    let event =
        NativeEvent::new(HostNodeId::new(21), NativeEventKind::SelectionChange).value("A3S");

    let input_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&input_only), &event)
        .unwrap();
    let change_invocation = EventRouter::new()
        .route(&AppKitAdapter.blueprint(&change_wins), &event)
        .unwrap();

    assert_eq!(input_invocation.action, "setProjectInput");
    assert_eq!(input_invocation.value.as_deref(), Some("A3S"));
    assert_eq!(change_invocation.action, "setAssignee");
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
fn routes_native_clipboard_events_to_clipboard_actions() {
    let element = NativeElement::new("clipboard", NativeRole::TextField).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_copy("copySelection")
                .on_cut("cutSelection")
                .on_paste("pasteSelection"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);

    let copy = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(24), NativeEventKind::Copy),
        )
        .unwrap();
    let cut = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(24), NativeEventKind::Cut),
        )
        .unwrap();
    let paste = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(24), NativeEventKind::Paste).value("Hello world"),
        )
        .unwrap();

    assert_eq!(copy.action, "copySelection");
    assert_eq!(copy.event, NativeEventKind::Copy);
    assert_eq!(cut.action, "cutSelection");
    assert_eq!(cut.event, NativeEventKind::Cut);
    assert_eq!(paste.action, "pasteSelection");
    assert_eq!(paste.event, NativeEventKind::Paste);
    assert_eq!(paste.value.as_deref(), Some("Hello world"));
}

#[test]
fn routes_native_close_to_window_close_action() {
    let element = NativeElement::new("window", NativeRole::Window)
        .with_props(NativeProps::new().web(WebProps::new().event("onClose", "closeApp")));
    let blueprint = AppKitAdapter.blueprint(&element);

    let invocation = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(23), NativeEventKind::Close),
        )
        .unwrap();

    assert_eq!(invocation.node, HostNodeId::new(23));
    assert_eq!(invocation.action, "closeApp");
    assert_eq!(invocation.event, NativeEventKind::Close);
}

#[test]
fn normalizes_common_native_key_values() {
    assert_eq!(native_key_value("Return"), "Enter");
    assert_eq!(native_key_value("KP_Enter"), "Enter");
    assert_eq!(native_key_value(" "), " ");
    assert_eq!(native_key_value("space"), " ");
    assert_eq!(native_key_value("BackSpace"), "Backspace");
    assert_eq!(native_key_value("Esc"), "Escape");
    assert_eq!(native_key_value("Left"), "ArrowLeft");
    assert_eq!(native_key_value("Page_Down"), "PageDown");
    assert_eq!(native_key_value("a"), "a");
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
fn link_keyboard_activation_uses_enter_but_not_space() {
    let element = NativeElement::new("docs", NativeRole::Link)
        .with_props(NativeProps::new().web(WebProps::new().on_press("openDocs")));
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();

    assert!(router
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(18), NativeEventKind::KeyDown).value("Enter"),
        )
        .is_some());
    assert!(router
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(18), NativeEventKind::KeyDown).value(" "),
        )
        .is_none());
}

#[test]
fn routes_native_platform_activation_key_names_to_primary_action() {
    let element = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().on_press("saveDocument")));
    let blueprint = AppKitAdapter.blueprint(&element);

    let return_key = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(22), NativeEventKind::KeyDown).value("Return"),
        )
        .unwrap();
    let gtk_space = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(22), NativeEventKind::KeyDown).value("space"),
        )
        .unwrap();

    assert_eq!(return_key.action, "saveDocument");
    assert_eq!(gtk_space.action, "saveDocument");
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
fn handled_native_activation_keeps_key_handler_without_duplicate_press() {
    let element = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_press("saveDocument")
                .on_key_down("handleShortcut"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let event = NativeEvent::new(HostNodeId::new(26), NativeEventKind::KeyDown)
        .value("Enter")
        .context(
            NativeEventContext::new()
                .modality(NativeInputModality::Keyboard)
                .handled_activation(true),
        );

    let invocations = EventRouter::new().route_all(&blueprint, &event);
    assert_eq!(invocations.len(), 1);
    assert_eq!(invocations[0].action, "handleShortcut");
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
            current_target: None,
            action: "saveDocument".to_string(),
            event: NativeEventKind::Press,
            context: Default::default(),
            value: None,
        })
        .unwrap();

    assert_eq!(registry.invocations().len(), 1);
    assert!(registry
        .invoke(ActionInvocation {
            node: HostNodeId::new(1),
            current_target: None,
            action: "missingAction".to_string(),
            event: NativeEventKind::Press,
            context: Default::default(),
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
            current_target: None,
            action: "saveDocument".to_string(),
            event: NativeEventKind::Press,
            context: Default::default(),
            value: None,
        })
        .unwrap();

    registry.replace_registered([RegisteredAction {
        id: "deleteDocument".to_string(),
        disabled: false,
        label: Some("Delete document".to_string()),
    }]);

    assert!(!registry.contains("saveDocument"));
    assert!(registry.contains("deleteDocument"));
    assert_eq!(registry.invocations().len(), 1);
}

#[test]
fn action_registry_rejects_disabled_actions() {
    let mut registry = ActionRegistry::new();
    registry.replace_registered([RegisteredAction {
        id: "saveDocument".to_string(),
        disabled: true,
        label: Some("Save document".to_string()),
    }]);

    let error = registry
        .invoke(ActionInvocation {
            node: HostNodeId::new(1),
            current_target: None,
            action: "saveDocument".to_string(),
            event: NativeEventKind::Press,
            context: Default::default(),
            value: None,
        })
        .unwrap_err();

    assert!(registry.contains("saveDocument"));
    assert!(registry.is_disabled("saveDocument"));
    assert!(error.to_string().contains("disabled action saveDocument"));
    assert!(registry.invocations().is_empty());
}
