use super::*;
use crate::accessibility::AccessibilityTreeHost;
use crate::event::NativeEventKind;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::overlay_position::{
    OverlayPlacement, OVERLAY_PLACEMENT_ATTRIBUTE, OVERLAY_POSITION_MARKER,
    OVERLAY_SHOULD_UPDATE_POSITION_ATTRIBUTE,
};
use crate::platform::{
    AppKitAdapter, Gtk4Adapter, PlatformAdapter, PlatformPlanningHost, WinUiAdapter,
};
use crate::web::WebProps;

fn runtime() -> GuiRuntime<PlatformPlanningHost<Gtk4Adapter>> {
    GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter))
}

fn overlay_props(
    close_action: &str,
    modal: bool,
    dismissable: bool,
    keyboard_dismiss_disabled: bool,
) -> NativeProps {
    NativeProps::new()
        .metadata("data-overlay", "true")
        .metadata("data-overlay-modal", modal.to_string())
        .metadata("data-overlay-dismissable", dismissable.to_string())
        .metadata(
            "data-overlay-keyboard-dismiss-disabled",
            keyboard_dismiss_disabled.to_string(),
        )
        .metadata("data-focus-scope", "true")
        .metadata("data-contain", modal.to_string())
        .metadata("data-restore-focus", "true")
        .metadata("data-auto-focus", "true")
        .modal(Some(modal))
        .web(WebProps::new().event("onClose", close_action))
}

fn inactive_overlay_props(close_action: &str) -> NativeProps {
    NativeProps::new().web(WebProps::new().event("onClose", close_action))
}

fn positioned_popover(should_update_position: bool) -> NativeElement {
    NativeElement::new("popover", NativeRole::Popover).with_props(
        NativeProps::new()
            .anchor("#trigger")
            .metadata("data-open", "true")
            .metadata(OVERLAY_POSITION_MARKER, "true")
            .metadata(OVERLAY_PLACEMENT_ATTRIBUTE, "top end")
            .metadata(
                OVERLAY_SHOULD_UPDATE_POSITION_ATTRIBUTE,
                should_update_position.to_string(),
            ),
    )
}

#[test]
fn runtime_projects_typed_anchor_position_and_honors_static_positioning() {
    let element = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("trigger", NativeRole::Button)
                .with_props(NativeProps::new().web(WebProps::new().id("trigger"))),
        )
        .child(positioned_popover(false));
    let mut runtime = runtime();

    let root = runtime.render_native(&element).unwrap();
    let trigger = runtime.host().node(root).unwrap().children[0];
    let popover = runtime.host().node(root).unwrap().children[1];
    let (anchor, request) = runtime.host().overlay_positions().get(&popover).unwrap();
    assert_eq!(*anchor, trigger);
    assert_eq!(request.options.placement, OverlayPlacement::TopEnd);
    assert!(!request.options.should_update_position);
    assert!(runtime.host().commands().iter().any(|command| matches!(
        command,
        crate::platform::PlatformCommand::PositionOverlay {
            overlay,
            anchor,
            ..
        } if *overlay == popover && *anchor == trigger
    )));

    runtime.host_mut().clear_commands();
    runtime.render_native(&element).unwrap();
    assert!(!runtime.host().commands().iter().any(|command| matches!(
        command,
        crate::platform::PlatformCommand::PositionOverlay { .. }
    )));
}

fn assert_adapter_applies_overlay_policy<A: PlatformAdapter>(adapter: A) {
    let element = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("background", NativeRole::Button))
        .child(
            NativeElement::new("modal", NativeRole::Dialog)
                .with_props(overlay_props("closeModal", true, false, false))
                .child(NativeElement::new("confirm", NativeRole::Button)),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    runtime.actions_mut().register("closeModal");

    let root = runtime.render_native(&element).unwrap();
    let background = runtime.host().node(root).unwrap().children[0];
    let modal = runtime.host().node(root).unwrap().children[1];
    let confirm = runtime.host().node(modal).unwrap().children[0];

    assert!(
        runtime
            .host()
            .node(background)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert_eq!(
        runtime
            .host()
            .node(background)
            .unwrap()
            .blueprint
            .metadata
            .get(crate::overlay::OVERLAY_CAPTURE_METADATA_KEY)
            .map(String::as_str),
        Some("true")
    );
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(confirm, NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();
    assert_eq!(handled.event.kind, NativeEventKind::Close);
    assert_eq!(handled.event.node, modal);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("closeModal")
    );
}

#[test]
fn every_platform_planning_adapter_applies_the_same_overlay_policy() {
    assert_adapter_applies_overlay_policy(AppKitAdapter);
    assert_adapter_applies_overlay_policy(Gtk4Adapter);
    assert_adapter_applies_overlay_policy(WinUiAdapter);
}

#[test]
fn escape_dismisses_only_the_topmost_overlay() {
    let element = NativeElement::new("root", NativeRole::View).child(
        NativeElement::new("outer", NativeRole::Dialog)
            .with_props(overlay_props("closeOuter", true, true, false))
            .child(NativeElement::new("outer-action", NativeRole::Button))
            .child(
                NativeElement::new("inner", NativeRole::Popover)
                    .with_props(overlay_props("closeInner", false, true, false))
                    .child(NativeElement::new("inner-action", NativeRole::Button)),
            ),
    );
    let mut runtime = runtime();
    runtime.actions_mut().register("closeOuter");
    runtime.actions_mut().register("closeInner");

    let root = runtime.render_native(&element).unwrap();
    let outer = runtime.host().node(root).unwrap().children[0];
    let inner = runtime.host().node(outer).unwrap().children[1];
    let inner_action = runtime.host().node(inner).unwrap().children[0];
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(inner_action, NativeEventKind::KeyDown).value("Esc"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::Close);
    assert_eq!(handled.event.node, inner);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("closeInner")
    );
    assert_eq!(runtime.actions().invocations().len(), 1);
}

#[test]
fn topmost_overlay_follows_activation_order_instead_of_tree_order() {
    let first = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("first", NativeRole::Popover)
                .with_props(inactive_overlay_props("closeFirst"))
                .child(NativeElement::new("first-action", NativeRole::Button)),
        )
        .child(
            NativeElement::new("second", NativeRole::Popover)
                .with_props(overlay_props("closeSecond", false, true, false))
                .child(NativeElement::new("second-action", NativeRole::Button)),
        );
    let second = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("first", NativeRole::Popover)
                .with_props(overlay_props("closeFirst", false, true, false))
                .child(NativeElement::new("first-action", NativeRole::Button)),
        )
        .child(
            NativeElement::new("second", NativeRole::Popover)
                .with_props(overlay_props("closeSecond", false, true, false))
                .child(NativeElement::new("second-action", NativeRole::Button)),
        );
    let mut runtime = runtime();
    runtime.actions_mut().register("closeFirst");
    runtime.actions_mut().register("closeSecond");

    let root = runtime.render_native(&first).unwrap();
    let first_overlay = runtime.host().node(root).unwrap().children[0];
    runtime.render_native(&second).unwrap();
    let first_action = runtime.host().node(first_overlay).unwrap().children[0];
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(first_action, NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();

    assert_eq!(handled.event.node, first_overlay);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("closeFirst")
    );
}

#[test]
fn keyboard_dismiss_disabled_leaves_escape_for_the_focused_control() {
    let element = NativeElement::new("root", NativeRole::View).child(
        NativeElement::new("modal", NativeRole::Dialog)
            .with_props(overlay_props("closeModal", true, true, true))
            .child(
                NativeElement::new("editor", NativeRole::TextField)
                    .with_props(NativeProps::new().web(WebProps::new().on_key_down("editKey"))),
            ),
    );
    let mut runtime = runtime();
    runtime.actions_mut().register("closeModal");
    runtime.actions_mut().register("editKey");

    let root = runtime.render_native(&element).unwrap();
    let modal = runtime.host().node(root).unwrap().children[0];
    let editor = runtime.host().node(modal).unwrap().children[0];
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(editor, NativeEventKind::KeyDown).value("Escape"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, NativeEventKind::KeyDown);
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("editKey")
    );
    assert!(runtime
        .actions()
        .invocations()
        .iter()
        .all(|invocation| invocation.action != "closeModal"));
}

#[test]
fn close_on_blur_dismisses_only_when_focus_moves_outside() {
    let popover_props = overlay_props("closePopover", false, false, false)
        .metadata("data-overlay-close-on-blur", "true");
    let element = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("outside", NativeRole::Button))
        .child(
            NativeElement::new("popover", NativeRole::Popover)
                .with_props(popover_props)
                .child(
                    NativeElement::new("field", NativeRole::TextField)
                        .with_props(NativeProps::new().web(WebProps::new().on_blur("fieldBlur"))),
                )
                .child(NativeElement::new("next-field", NativeRole::TextField)),
        );
    let mut runtime = runtime();
    runtime.actions_mut().register("closePopover");
    runtime.actions_mut().register("fieldBlur");

    let root = runtime.render_native(&element).unwrap();
    let outside = runtime.host().node(root).unwrap().children[0];
    let popover = runtime.host().node(root).unwrap().children[1];
    let field = runtime.host().node(popover).unwrap().children[0];
    let next_field = runtime.host().node(popover).unwrap().children[1];
    runtime
        .handle_native_event(NativeEvent::new(field, NativeEventKind::Focus))
        .unwrap();
    let inside = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(field, NativeEventKind::Blur)
                .context(crate::input::NativeEventContext::new().related_target(next_field)),
        )
        .unwrap();
    runtime
        .handle_native_event(NativeEvent::new(field, NativeEventKind::Focus))
        .unwrap();
    let unknown = runtime
        .handle_native_event_with_changes(NativeEvent::new(field, NativeEventKind::Blur))
        .unwrap();
    runtime
        .handle_native_event(NativeEvent::new(field, NativeEventKind::Focus))
        .unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(field, NativeEventKind::Blur)
                .context(crate::input::NativeEventContext::new().related_target(outside)),
        )
        .unwrap();

    assert_eq!(inside.event.kind, NativeEventKind::Blur);
    assert_eq!(
        inside
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["fieldBlur"]
    );
    assert_eq!(unknown.event.kind, NativeEventKind::Blur);
    assert_eq!(
        unknown
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["fieldBlur"]
    );
    assert_eq!(handled.event.kind, NativeEventKind::Blur);
    assert_eq!(handled.event.node, field);
    assert_eq!(
        handled
            .invocations
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["fieldBlur", "closePopover"]
    );
    assert!(runtime
        .interactions()
        .node(field)
        .is_some_and(|state| !state.focused));
}

#[test]
fn outside_press_sequence_dismisses_without_activating_the_background() {
    let element = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("background", NativeRole::Button).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .on_press_start("backgroundStart")
                        .on_press_up("backgroundUp")
                        .on_press("backgroundPress"),
                ),
            ),
        )
        .child(
            NativeElement::new("underlay", NativeRole::View).child(
                NativeElement::new("modal", NativeRole::Dialog)
                    .with_props(overlay_props("closeModal", true, true, false))
                    .child(NativeElement::new("confirm", NativeRole::Button)),
            ),
        );
    let mut runtime = runtime();
    for action in [
        "backgroundStart",
        "backgroundUp",
        "backgroundPress",
        "closeModal",
    ] {
        runtime.actions_mut().register(action);
    }

    let root = runtime.render_native(&element).unwrap();
    let background = runtime.host().node(root).unwrap().children[0];
    let start = runtime
        .handle_native_event_with_changes(NativeEvent::new(background, NativeEventKind::PressStart))
        .unwrap();
    let end = runtime
        .handle_native_event_with_changes(NativeEvent::new(background, NativeEventKind::PressUp))
        .unwrap();

    assert!(start.invocations.is_empty());
    assert_eq!(end.event.kind, NativeEventKind::Close);
    assert_eq!(
        end.invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("closeModal")
    );
    assert_eq!(
        runtime
            .actions()
            .invocations()
            .iter()
            .map(|invocation| invocation.action.as_str())
            .collect::<Vec<_>>(),
        vec!["closeModal"]
    );
}

#[test]
fn dismissable_modal_treats_its_underlay_root_as_outside_content() {
    let props =
        overlay_props("closeModal", true, true, false).metadata("data-overlay-underlay", "true");
    let element = NativeElement::new("modal", NativeRole::Dialog)
        .with_props(props)
        .child(NativeElement::new("confirm", NativeRole::Button));
    let mut runtime = runtime();
    runtime.actions_mut().register("closeModal");

    let modal = runtime.render_native(&element).unwrap();
    let start = runtime
        .handle_native_event_with_changes(NativeEvent::new(modal, NativeEventKind::PressStart))
        .unwrap();
    let end = runtime
        .handle_native_event_with_changes(NativeEvent::new(modal, NativeEventKind::PressUp))
        .unwrap();

    assert!(start.invocations.is_empty());
    assert_eq!(end.event.kind, NativeEventKind::Close);
    assert_eq!(
        end.invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("closeModal")
    );
}

#[test]
fn outside_press_must_end_outside_the_same_topmost_overlay() {
    let element = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("background", NativeRole::Button))
        .child(
            NativeElement::new("modal", NativeRole::Dialog)
                .with_props(overlay_props("closeModal", true, true, false))
                .child(
                    NativeElement::new("confirm", NativeRole::Button).with_props(
                        NativeProps::new().web(WebProps::new().on_press_up("confirmUp")),
                    ),
                ),
        );
    let mut runtime = runtime();
    runtime.actions_mut().register("closeModal");
    runtime.actions_mut().register("confirmUp");

    let root = runtime.render_native(&element).unwrap();
    let background = runtime.host().node(root).unwrap().children[0];
    let modal = runtime.host().node(root).unwrap().children[1];
    let confirm = runtime.host().node(modal).unwrap().children[0];
    runtime
        .handle_native_event_with_changes(NativeEvent::new(background, NativeEventKind::PressStart))
        .unwrap();
    let end = runtime
        .handle_native_event_with_changes(NativeEvent::new(confirm, NativeEventKind::PressUp))
        .unwrap();

    assert_eq!(
        end.invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("confirmUp")
    );
    assert!(runtime
        .actions()
        .invocations()
        .iter()
        .all(|invocation| invocation.action != "closeModal"));
}

#[test]
fn modal_projects_background_inertness_and_restores_it_after_unmount() {
    let open = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("background", NativeRole::Button)
                .with_props(NativeProps::new().label("Background")),
        )
        .child(
            NativeElement::new("underlay", NativeRole::View).child(
                NativeElement::new("modal", NativeRole::Dialog)
                    .with_props(overlay_props("closeModal", true, false, false))
                    .child(
                        NativeElement::new("confirm", NativeRole::Button)
                            .with_props(NativeProps::new().label("Confirm")),
                    ),
            ),
        );
    let closed = NativeElement::new("root", NativeRole::View).child(
        NativeElement::new("background", NativeRole::Button)
            .with_props(NativeProps::new().label("Background")),
    );
    let mut runtime = runtime();

    let root = runtime.render_native(&open).unwrap();
    let background = runtime.host().node(root).unwrap().children[0];
    let underlay = runtime.host().node(root).unwrap().children[1];
    let modal = runtime.host().node(underlay).unwrap().children[0];
    assert!(
        runtime
            .host()
            .node(background)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert!(
        !runtime
            .host()
            .node(underlay)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert_eq!(
        runtime
            .host()
            .node(underlay)
            .unwrap()
            .blueprint
            .metadata
            .get(crate::overlay::OVERLAY_CAPTURE_METADATA_KEY)
            .map(String::as_str),
        Some("true")
    );
    assert!(
        !runtime
            .host()
            .node(modal)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    let accessibility = runtime.host().accessibility_tree().unwrap();
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(
        accessibility.children[0].role,
        crate::accessibility::AccessibilityRole::Group
    );

    runtime.render_native(&closed).unwrap();
    assert!(
        !runtime
            .host()
            .node(background)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert!(!runtime
        .host()
        .node(background)
        .unwrap()
        .blueprint
        .metadata
        .contains_key(crate::overlay::OVERLAY_CAPTURE_METADATA_KEY));
    let accessibility = runtime.host().accessibility_tree().unwrap();
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(
        accessibility.children[0].label.as_deref(),
        Some("Background")
    );
}

#[test]
fn modal_keeps_later_portaled_overlays_interactive() {
    let element = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("background", NativeRole::Button)
                .with_props(NativeProps::new().label("Background")),
        )
        .child(
            NativeElement::new("modal", NativeRole::Dialog)
                .with_props(overlay_props("closeModal", true, false, false))
                .child(NativeElement::new("modal-action", NativeRole::Button)),
        )
        .child(
            NativeElement::new("portaled-popover", NativeRole::Popover)
                .with_props(overlay_props("closePopover", false, true, false))
                .child(
                    NativeElement::new("popover-action", NativeRole::Button).with_props(
                        NativeProps::new().web(WebProps::new().on_press("activatePopover")),
                    ),
                ),
        );
    let mut runtime = runtime();
    runtime.actions_mut().register("activatePopover");

    let root = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root).unwrap().children.clone();
    let modal_action = runtime.host().node(children[1]).unwrap().children[0];
    let popover_action = runtime.host().node(children[2]).unwrap().children[0];

    assert!(
        runtime
            .host()
            .node(children[0])
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert!(
        !runtime
            .host()
            .node(children[2])
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    assert!(
        !runtime
            .host()
            .node(popover_action)
            .unwrap()
            .blueprint
            .control_state
            .inert
    );
    let handled = runtime
        .handle_native_event_with_changes(NativeEvent::new(popover_action, NativeEventKind::Press))
        .unwrap();
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .map(|invocation| invocation.action.as_str()),
        Some("activatePopover")
    );
    runtime
        .handle_native_event(NativeEvent::new(modal_action, NativeEventKind::Focus))
        .unwrap();
    runtime
        .handle_native_event(NativeEvent::new(popover_action, NativeEventKind::Focus))
        .unwrap();
    assert!(runtime
        .interactions()
        .node(popover_action)
        .is_some_and(|state| state.focused));
}

#[test]
fn opening_and_closing_overlay_moves_focus_in_and_restores_the_trigger() {
    let closed = NativeElement::new("root", NativeRole::View).child(
        NativeElement::new("trigger", NativeRole::Button)
            .with_props(NativeProps::new().label("Open")),
    );
    let open = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("trigger", NativeRole::Button)
                .with_props(NativeProps::new().label("Open")),
        )
        .child(
            NativeElement::new("modal", NativeRole::Dialog)
                .with_props(overlay_props("closeModal", true, false, false))
                .child(
                    NativeElement::new("confirm", NativeRole::Button)
                        .with_props(NativeProps::new().label("Confirm")),
                ),
        );
    let mut runtime = runtime();

    let root = runtime.render_native(&closed).unwrap();
    let trigger = runtime.host().node(root).unwrap().children[0];
    runtime
        .handle_native_event(NativeEvent::new(trigger, NativeEventKind::Focus))
        .unwrap();

    runtime.render_native(&open).unwrap();
    let modal = runtime.host().node(root).unwrap().children[1];
    let confirm = runtime.host().node(modal).unwrap().children[0];
    assert_eq!(runtime.host().focused(), Some(confirm));
    runtime
        .handle_native_event(NativeEvent::new(confirm, NativeEventKind::Focus))
        .unwrap();

    runtime.render_native(&closed).unwrap();
    assert_eq!(runtime.host().focused(), Some(trigger));
}

#[test]
fn newly_opened_overlay_autofocus_precedes_background_tree_focus_fallback() {
    let tree = |expanded_keys: &str| {
        NativeElement::new("files", NativeRole::Tree)
            .with_props(
                NativeProps::new().web(WebProps::new().attribute("expandedKeys", expanded_keys)),
            )
            .child(
                NativeElement::new("documents", NativeRole::TreeItem).with_props(
                    NativeProps::new()
                        .label("Documents")
                        .web(WebProps::new().attribute("data-has-child-items", "true")),
                ),
            )
            .child(
                NativeElement::new("resume", NativeRole::TreeItem).with_props(
                    NativeProps::new()
                        .label("Resume")
                        .web(WebProps::new().attribute("data-tree-parent-key", "documents")),
                ),
            )
    };
    let closed = NativeElement::new("root", NativeRole::View).child(tree(r#"["documents"]"#));
    let open = NativeElement::new("root", NativeRole::View)
        .child(tree("[]"))
        .child(
            NativeElement::new("modal", NativeRole::Dialog)
                .with_props(overlay_props("closeModal", true, false, false))
                .child(NativeElement::new("confirm", NativeRole::Button)),
        );
    let mut runtime = runtime();

    let root = runtime.render_native(&closed).unwrap();
    let tree_node = runtime.host().node(root).unwrap().children[0];
    let resume = runtime.host().node(tree_node).unwrap().children[1];
    runtime
        .handle_native_event(NativeEvent::new(resume, NativeEventKind::Focus))
        .unwrap();

    runtime.render_native(&open).unwrap();
    let modal = runtime.host().node(root).unwrap().children[1];
    let confirm = runtime.host().node(modal).unwrap().children[0];

    assert_eq!(runtime.host().focused(), Some(confirm));
}
