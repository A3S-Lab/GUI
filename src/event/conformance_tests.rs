use super::*;
use crate::input::{NativeInputModality, NativeKeyModifiers};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{AppKitAdapter, PlatformAdapter};
use crate::web::WebProps;

#[test]
fn native_event_context_is_optional_and_backward_compatible() {
    let legacy = serde_json::json!({
        "node": 7,
        "kind": "press",
        "value": null
    });
    let decoded: NativeEvent = serde_json::from_value(legacy).unwrap();
    assert!(decoded.context.is_empty());

    let enriched = NativeEvent::new(HostNodeId::new(7), NativeEventKind::PressStart)
        .modality(NativeInputModality::Touch)
        .modifiers(NativeKeyModifiers::new().shift(true))
        .position(4.0, 8.0);
    let json = serde_json::to_value(enriched).unwrap();
    assert_eq!(json["context"]["modality"], "touch");
    assert_eq!(json["context"]["modifiers"]["shift"], true);
    assert_eq!(json["context"]["position"]["y"], 8.0);
}

#[test]
fn infers_only_unambiguous_legacy_modalities() {
    assert_eq!(
        NativeEvent::new(HostNodeId::new(1), NativeEventKind::KeyDown).effective_modality(),
        NativeInputModality::Keyboard
    );
    assert_eq!(
        NativeEvent::new(HostNodeId::new(1), NativeEventKind::HoverStart).effective_modality(),
        NativeInputModality::Mouse
    );
    assert_eq!(
        NativeEvent::new(HostNodeId::new(1), NativeEventKind::Press).effective_modality(),
        NativeInputModality::Unknown
    );
}

#[test]
fn routes_press_up_and_hover_lifecycle_actions() {
    let element = NativeElement::new("target", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_press_up("releaseTarget")
                .on_hover_start("enterTarget")
                .on_hover_end("leaveTarget"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();

    assert_eq!(
        router
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(1), NativeEventKind::PressUp),
            )
            .unwrap()
            .action,
        "releaseTarget"
    );
    assert_eq!(
        router
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(1), NativeEventKind::HoverStart),
            )
            .unwrap()
            .action,
        "enterTarget"
    );
    assert_eq!(
        router
            .route(
                &blueprint,
                &NativeEvent::new(HostNodeId::new(1), NativeEventKind::HoverEnd),
            )
            .unwrap()
            .action,
        "leaveTarget"
    );
    assert!(router
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(1), NativeEventKind::HoverStart)
                .modality(NativeInputModality::Touch),
        )
        .is_none());
}

#[test]
fn routes_specific_and_change_callbacks_in_semantic_order() {
    let element = NativeElement::new("target", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_hover_start("enterTarget")
                .on_hover_change("changeHover")
                .on_focus("focusTarget")
                .on_focus_change("changeFocus"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();

    let hover = router.route_all(
        &blueprint,
        &NativeEvent::new(HostNodeId::new(1), NativeEventKind::HoverStart),
    );
    assert_eq!(
        hover
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["enterTarget", "changeHover"]
    );

    let focus = router.route_all(
        &blueprint,
        &NativeEvent::new(HostNodeId::new(1), NativeEventKind::Focus),
    );
    assert_eq!(
        focus
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["focusTarget", "changeFocus"]
    );
}

#[test]
fn routes_press_lifecycle_before_press_change() {
    let element = NativeElement::new("target", NativeRole::Button).with_props(
        NativeProps::new().web(
            WebProps::new()
                .on_press_start("startTarget")
                .on_press_end("endTarget")
                .on_press_change("changePress"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let router = EventRouter::new();

    let start = router.route_all(
        &blueprint,
        &NativeEvent::new(HostNodeId::new(1), NativeEventKind::PressStart).value("true"),
    );
    assert_eq!(
        start
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["startTarget", "changePress"]
    );
    assert!(start
        .iter()
        .all(|invocation| invocation.value.as_deref() == Some("true")));

    let end = router.route_all(
        &blueprint,
        &NativeEvent::new(HostNodeId::new(1), NativeEventKind::PressCancel).value("false"),
    );
    assert_eq!(
        end.iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["endTarget", "changePress"]
    );
    assert!(end
        .iter()
        .all(|invocation| invocation.value.as_deref() == Some("false")));
}

#[test]
fn collection_action_has_a_distinct_callback_channel() {
    let element = NativeElement::new("people", NativeRole::ListBox).with_props(
        NativeProps::new().web(
            WebProps::new()
                .event("onAction", "openPerson")
                .on_press("pressList")
                .on_selection_change("selectPerson"),
        ),
    );
    let blueprint = AppKitAdapter.blueprint(&element);
    let invocation = EventRouter::new()
        .route(
            &blueprint,
            &NativeEvent::new(HostNodeId::new(1), NativeEventKind::Action).value("ada-key"),
        )
        .unwrap();

    assert_eq!(invocation.action, "openPerson");
    assert_eq!(invocation.event, NativeEventKind::Action);
    assert_eq!(invocation.value.as_deref(), Some("ada-key"));
}

#[test]
fn action_registry_rejects_a_batch_without_recording_a_prefix() {
    let mut registry = ActionRegistry::new();
    registry.register("first");
    let invocations = vec![
        ActionInvocation::new(HostNodeId::new(1), "first", NativeEventKind::HoverStart),
        ActionInvocation::new(HostNodeId::new(1), "missing", NativeEventKind::HoverStart),
    ];

    assert!(registry.invoke_all(&invocations).is_err());
    assert!(registry.invocations().is_empty());
}

#[test]
fn action_invocation_preserves_typed_event_context() {
    let element = NativeElement::new("target", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().on_press("activate")));
    let blueprint = AppKitAdapter.blueprint(&element);
    let event = NativeEvent::new(HostNodeId::new(9), NativeEventKind::Press)
        .modality(NativeInputModality::Virtual)
        .modifiers(NativeKeyModifiers::new().control(true))
        .position(3.0, 5.0);

    let invocation = EventRouter::new().route(&blueprint, &event).unwrap();

    assert_eq!(invocation.context, event.context);
    assert_eq!(invocation.modality(), NativeInputModality::Virtual);
    assert_eq!(invocation.context.position.unwrap().x, 3.0);
}

#[test]
fn action_invocation_context_is_backward_compatible() {
    let legacy = serde_json::json!({
        "node": 7,
        "action": "activate",
        "event": "press",
        "value": null
    });

    let invocation: ActionInvocation = serde_json::from_value(legacy).unwrap();

    assert!(invocation.context.is_empty());
    assert_eq!(invocation.current_target(), HostNodeId::new(7));
}

#[test]
fn bubbled_action_serializes_its_current_target() {
    let invocation = ActionInvocation::new(
        HostNodeId::new(7),
        "parentHandler",
        NativeEventKind::HoverStart,
    )
    .with_current_target(HostNodeId::new(3));

    let json = serde_json::to_value(invocation).unwrap();

    assert_eq!(json["node"], 7);
    assert_eq!(json["currentTarget"], 3);
}

#[test]
fn action_invocation_decodes_legacy_and_set_selection_payloads() {
    let legacy = ActionInvocation::new(
        HostNodeId::new(1),
        "select",
        NativeEventKind::SelectionChange,
    )
    .with_value("alpha");
    let multiple = ActionInvocation::new(
        HostNodeId::new(1),
        "select",
        NativeEventKind::SelectionChange,
    )
    .with_value(r#"["alpha","beta"]"#);

    assert_eq!(
        legacy.selection().unwrap(),
        Some(crate::selection::Selection::keys([
            crate::selection::CollectionKey::from("alpha")
        ]))
    );
    assert_eq!(
        multiple.selection().unwrap(),
        Some(crate::selection::Selection::keys([
            crate::selection::CollectionKey::from("alpha"),
            crate::selection::CollectionKey::from("beta")
        ]))
    );
}

#[test]
fn native_event_rejects_non_finite_positions() {
    let event =
        NativeEvent::new(HostNodeId::new(1), NativeEventKind::PressStart).position(f64::NAN, 0.0);
    assert!(event.validate().is_err());
}

#[test]
fn native_event_rejects_non_finite_movement_deltas() {
    let event =
        NativeEvent::new(HostNodeId::new(1), NativeEventKind::Move).delta(f64::INFINITY, 0.0);
    assert!(event.validate().is_err());
}
