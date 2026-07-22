use super::*;
use crate::event::{NativeEvent, NativeEventKind};
use crate::input::NativeInputModality;
use crate::native::NativeElement;
use crate::platform::{Gtk4Adapter, PlatformCommand, PlatformPlanningHost};
use crate::runtime::GuiRuntime;
use crate::web::WebProps;

fn record(
    node: u64,
    parent: Option<u64>,
    role: NativeRole,
    props: NativeProps,
) -> MountedNodeSnapshot {
    MountedNodeSnapshot {
        node: HostNodeId::new(node),
        parent: parent.map(HostNodeId::new),
        key: crate::native::ElementKey::new(format!("node-{node}")),
        role,
        props,
    }
}

fn scope_props(contain: bool, auto_focus: bool) -> NativeProps {
    NativeProps::new().auto_focus(auto_focus).web(
        WebProps::new()
            .attribute("data-focus-scope", "true")
            .attribute("data-contain", contain.to_string()),
    )
}

fn restoring_scope_props() -> NativeProps {
    NativeProps::new().web(
        WebProps::new()
            .attribute("data-focus-scope", "true")
            .attribute("data-restore-focus", "true"),
    )
}

#[test]
fn scope_navigation_uses_tab_order_and_wraps() {
    let snapshot = vec![
        record(1, None, NativeRole::View, scope_props(true, false)),
        record(
            2,
            Some(1),
            NativeRole::Button,
            NativeProps::new().tab_index(Some(0)),
        ),
        record(
            3,
            Some(1),
            NativeRole::Button,
            NativeProps::new().tab_index(Some(2)),
        ),
        record(
            4,
            Some(1),
            NativeRole::Button,
            NativeProps::new().tab_index(Some(-1)),
        ),
    ];
    let manager = FocusManager::from_snapshot(&snapshot);

    assert_eq!(
        manager.focusable_nodes(Some(HostNodeId::new(1)), FocusNavigationMode::Tabbable),
        vec![HostNodeId::new(3), HostNodeId::new(2)]
    );
    assert_eq!(
        manager.next(
            HostNodeId::new(2),
            Some(HostNodeId::new(1)),
            FocusNavigationMode::Tabbable,
            true,
        ),
        Some(HostNodeId::new(3))
    );
    assert!(manager.is_focusable(HostNodeId::new(4)));
    assert!(!manager.is_tabbable(HostNodeId::new(4)));
}

#[test]
fn containment_rejects_focus_outside_the_active_scope() {
    let snapshot = vec![
        record(1, None, NativeRole::View, NativeProps::new()),
        record(2, Some(1), NativeRole::View, scope_props(true, false)),
        record(3, Some(2), NativeRole::Button, NativeProps::new()),
        record(4, Some(1), NativeRole::Button, NativeProps::new()),
    ];
    let manager = FocusManager::from_snapshot(&snapshot);

    assert_eq!(
        manager.constrain_focus(HostNodeId::new(3), HostNodeId::new(4)),
        Some(HostNodeId::new(3))
    );
}

#[test]
fn scope_auto_focus_selects_a_descendant_not_the_scope_wrapper() {
    let snapshot = vec![
        record(1, None, NativeRole::View, scope_props(false, true)),
        record(2, Some(1), NativeRole::Text, NativeProps::new()),
        record(3, Some(1), NativeRole::TextField, NativeProps::new()),
    ];
    let manager = FocusManager::from_snapshot(&snapshot);

    assert_eq!(manager.auto_focus_target(), Some(HostNodeId::new(3)));
    assert!(!manager.is_focusable(HostNodeId::new(1)));
}

#[test]
fn removed_restore_scope_returns_the_focus_held_before_mount() {
    let closed = vec![
        record(1, None, NativeRole::View, NativeProps::new()),
        record(2, Some(1), NativeRole::Button, NativeProps::new()),
    ];
    let open = vec![
        record(1, None, NativeRole::View, NativeProps::new()),
        record(2, Some(1), NativeRole::Button, NativeProps::new()),
        record(3, Some(1), NativeRole::View, restoring_scope_props()),
        record(4, Some(3), NativeRole::Button, NativeProps::new()),
    ];
    let mut manager = FocusManager::new();

    assert_eq!(manager.sync_with_focus(&closed, None), None);
    assert_eq!(
        manager.sync_with_focus(&open, Some(HostNodeId::new(2))),
        None
    );
    assert_eq!(
        manager.sync_with_focus(&closed, Some(HostNodeId::new(4))),
        Some(HostNodeId::new(2))
    );
}

#[test]
fn nested_restore_scopes_unwind_to_their_own_prior_targets() {
    let closed = vec![
        record(1, None, NativeRole::View, NativeProps::new()),
        record(2, Some(1), NativeRole::Button, NativeProps::new()),
    ];
    let outer = vec![
        closed[0].clone(),
        closed[1].clone(),
        record(3, Some(1), NativeRole::View, restoring_scope_props()),
        record(4, Some(3), NativeRole::Button, NativeProps::new()),
    ];
    let mut nested = outer.clone();
    nested.extend([
        record(5, Some(3), NativeRole::View, restoring_scope_props()),
        record(6, Some(5), NativeRole::Button, NativeProps::new()),
    ]);
    let mut manager = FocusManager::new();

    manager.sync_with_focus(&closed, None);
    manager.sync_with_focus(&outer, Some(HostNodeId::new(2)));
    manager.sync_with_focus(&nested, Some(HostNodeId::new(4)));

    assert_eq!(
        manager.sync_with_focus(&outer, Some(HostNodeId::new(6))),
        Some(HostNodeId::new(4))
    );
    assert_eq!(
        manager.sync_with_focus(&closed, Some(HostNodeId::new(4))),
        Some(HostNodeId::new(2))
    );
}

#[test]
fn disabled_ancestors_remove_descendants_from_navigation() {
    let snapshot = vec![
        record(1, None, NativeRole::View, NativeProps::new().disabled(true)),
        record(2, Some(1), NativeRole::Button, NativeProps::new()),
    ];
    let manager = FocusManager::from_snapshot(&snapshot);

    assert!(manager
        .focusable_nodes(None, FocusNavigationMode::Focusable)
        .is_empty());
}

#[test]
fn availability_does_not_depend_on_snapshot_order() {
    let snapshot = vec![
        record(2, Some(1), NativeRole::Button, NativeProps::new()),
        record(1, None, NativeRole::View, NativeProps::new().disabled(true)),
    ];
    let manager = FocusManager::from_snapshot(&snapshot);

    assert!(!manager.is_focusable(HostNodeId::new(2)));
}

#[test]
fn runtime_scope_auto_focus_targets_the_first_focusable_descendant() {
    let tree = NativeElement::new("scope", NativeRole::View)
        .with_props(scope_props(true, true))
        .child(NativeElement::new("label", NativeRole::Text))
        .child(NativeElement::new("field", NativeRole::TextField));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let scope = runtime.render_native(&tree).unwrap();
    let field = runtime.host().node(scope).unwrap().children[1];

    assert_eq!(runtime.focus_manager().auto_focus_target(), Some(field));
    assert!(runtime.interactions().node(field).unwrap().focused);
    assert!(runtime
        .interactions()
        .node(scope)
        .is_none_or(|state| !state.focused));
}

#[test]
fn runtime_auto_focus_uses_the_typed_focus_command_after_mount() {
    let tree = NativeElement::new("scope", NativeRole::View)
        .with_props(scope_props(true, true))
        .child(NativeElement::new("label", NativeRole::Text))
        .child(NativeElement::new("field", NativeRole::TextField));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let scope = runtime.render_native(&tree).unwrap();
    let field = runtime.host().node(scope).unwrap().children[1];
    let commands = runtime.host().commands();
    let set_root = commands
        .iter()
        .position(|command| matches!(command, PlatformCommand::SetRoot { id } if *id == scope))
        .unwrap();
    let request_focus = commands
        .iter()
        .position(|command| matches!(command, PlatformCommand::RequestFocus { id } if *id == field))
        .unwrap();

    assert!(set_root < request_focus);
    assert_eq!(runtime.host().focused(), Some(field));
    assert_eq!(
        runtime.interactions().input_modality(),
        NativeInputModality::Virtual
    );
    assert!(runtime.interactions().node(field).unwrap().focus_visible);

    runtime.host_mut().clear_commands();
    runtime.render_native(&tree).unwrap();
    assert!(!runtime
        .host()
        .commands()
        .iter()
        .any(|command| matches!(command, PlatformCommand::RequestFocus { .. })));
}

#[test]
fn runtime_focus_navigation_emits_programmatic_focus_commands() {
    let tree = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("later", NativeRole::Button)
                .with_props(NativeProps::new().tab_index(Some(0))),
        )
        .child(
            NativeElement::new("first", NativeRole::Button)
                .with_props(NativeProps::new().tab_index(Some(2))),
        )
        .child(
            NativeElement::new("programmatic", NativeRole::Button)
                .with_props(NativeProps::new().tab_index(Some(-1))),
        );
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let root = runtime.render_native(&tree).unwrap();
    let children = runtime.host().node(root).unwrap().children.clone();
    let first = runtime
        .focus_first(None, FocusNavigationMode::Tabbable)
        .unwrap();

    assert_eq!(first, Some(children[1]));
    assert_eq!(runtime.host().focused(), first);
    assert!(matches!(
        runtime.host().commands().last(),
        Some(PlatformCommand::RequestFocus { id }) if Some(*id) == first
    ));

    runtime
        .handle_native_event(
            NativeEvent::new(children[1], NativeEventKind::Focus)
                .modality(NativeInputModality::Keyboard),
        )
        .unwrap();
    assert_eq!(
        runtime
            .focus_next(None, FocusNavigationMode::Tabbable, false)
            .unwrap(),
        Some(children[0])
    );
    assert_eq!(runtime.host().focused(), Some(children[0]));
}

#[test]
fn runtime_programmatic_focus_marks_unknown_native_focus_as_virtual() {
    let tree = NativeElement::new("save", NativeRole::Button);
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let button = runtime.render_native(&tree).unwrap();

    runtime.request_focus(button).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(NativeEvent::new(button, NativeEventKind::Focus))
        .unwrap();

    assert_eq!(handled.event.context.modality, NativeInputModality::Virtual);
    assert_eq!(
        runtime.interactions().input_modality(),
        NativeInputModality::Virtual
    );
    let interaction = runtime.interactions().node(button).unwrap();
    assert!(interaction.focused);
    assert!(interaction.focus_visible);
}

#[test]
fn runtime_programmatic_focus_respects_contained_scope() {
    let tree = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("scope", NativeRole::View)
                .with_props(scope_props(true, false))
                .child(NativeElement::new("inside", NativeRole::Button)),
        )
        .child(NativeElement::new("outside", NativeRole::Button));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let root = runtime.render_native(&tree).unwrap();
    let scope = runtime.host().node(root).unwrap().children[0];
    let inside = runtime.host().node(scope).unwrap().children[0];
    let outside = runtime.host().node(root).unwrap().children[1];
    runtime
        .handle_native_event(
            NativeEvent::new(inside, NativeEventKind::Focus)
                .modality(NativeInputModality::Keyboard),
        )
        .unwrap();

    assert_eq!(runtime.request_focus(outside).unwrap(), inside);
    assert_eq!(runtime.host().focused(), Some(inside));
}

#[test]
fn runtime_redirects_native_focus_that_escapes_a_contained_scope() {
    let tree = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("scope", NativeRole::View)
                .with_props(scope_props(true, false))
                .child(NativeElement::new("inside", NativeRole::Button)),
        )
        .child(NativeElement::new("outside", NativeRole::Button));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime.render_native(&tree).unwrap();
    let scope = runtime.host().node(root).unwrap().children[0];
    let inside = runtime.host().node(scope).unwrap().children[0];
    let outside = runtime.host().node(root).unwrap().children[1];

    runtime
        .handle_native_event(
            NativeEvent::new(inside, NativeEventKind::Focus)
                .modality(NativeInputModality::Keyboard),
        )
        .unwrap();
    runtime
        .handle_native_event(NativeEvent::new(inside, NativeEventKind::Blur))
        .unwrap();
    runtime.host_mut().clear_commands();

    let handled = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(outside, NativeEventKind::Focus)
                .modality(NativeInputModality::Keyboard),
        )
        .unwrap();

    assert!(handled.invocations.is_empty());
    assert!(handled.interaction_changes.is_empty());
    assert_eq!(runtime.host().focused(), Some(inside));
    assert!(matches!(
        runtime.host().commands().last(),
        Some(PlatformCommand::RequestFocus { id }) if *id == inside
    ));
    assert!(runtime
        .interactions()
        .node(outside)
        .is_none_or(|state| !state.focused));
}

#[test]
fn runtime_restores_native_focus_when_a_scope_unmounts() {
    fn tree(open: bool) -> NativeElement {
        let root = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("trigger", NativeRole::Button));
        if open {
            root.child(
                NativeElement::new("dialog-scope", NativeRole::View)
                    .with_props(restoring_scope_props())
                    .child(NativeElement::new("dialog-button", NativeRole::Button)),
            )
        } else {
            root
        }
    }

    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));
    let root = runtime.render_native(&tree(false)).unwrap();
    let trigger = runtime.host().node(root).unwrap().children[0];
    runtime
        .handle_native_event(NativeEvent::new(trigger, NativeEventKind::Focus))
        .unwrap();

    runtime.render_native(&tree(true)).unwrap();
    let scope = runtime.host().node(root).unwrap().children[1];
    let dialog_button = runtime.host().node(scope).unwrap().children[0];
    runtime
        .handle_native_event(NativeEvent::new(dialog_button, NativeEventKind::Focus))
        .unwrap();
    runtime.host_mut().clear_commands();

    runtime.render_native(&tree(false)).unwrap();

    assert_eq!(runtime.host().focused(), Some(trigger));
    assert!(matches!(
        runtime.host().commands().last(),
        Some(PlatformCommand::RequestFocus { id }) if *id == trigger
    ));
}

#[test]
fn runtime_rejects_programmatic_focus_for_non_focusable_nodes() {
    let tree = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("label", NativeRole::Text));
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter));

    let root = runtime.render_native(&tree).unwrap();
    let label = runtime.host().node(root).unwrap().children[0];
    let error = runtime.request_focus(label).unwrap_err();

    assert!(error.to_string().contains("not a mounted focusable node"));
    assert!(runtime.host().focused().is_none());
}
