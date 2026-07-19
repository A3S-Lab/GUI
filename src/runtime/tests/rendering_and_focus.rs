use super::super::*;
use crate::accessibility::AccessibilityRole;
use crate::host::HeadlessHost;
use crate::html::HtmlDialogProps;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

#[test]
fn runtime_renders_compiled_rsx_to_platform_host() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
    )
    .unwrap();
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();

    let root = runtime.host().node(root_id).unwrap();
    assert_eq!(root.blueprint.widget_class, "gtk::Button");
    assert_eq!(root.blueprint.action.as_deref(), Some("saveDocument"));
}

#[test]
fn runtime_exports_headless_accessibility_tree() {
    let root = NativeElement::new("dialog", NativeRole::Dialog)
        .with_props(
            NativeProps::new()
                .label("Preferences")
                .accessibility_description_text("Keyboard and display settings")
                .accessibility_level(Some(1))
                .modal(Some(true)),
        )
        .child(
            NativeElement::new("close", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Close")
                    .accessibility_controls("dialog")
                    .pressed("false"),
            ),
        );
    let mut runtime = GuiRuntime::new(HeadlessHost::default());

    let root_id = runtime.render_native(&root).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.node, Some(root_id));
    assert_eq!(accessibility.role, AccessibilityRole::Dialog);
    assert_eq!(accessibility.label.as_deref(), Some("Preferences"));
    assert_eq!(
        accessibility.description.description.as_deref(),
        Some("Keyboard and display settings")
    );
    assert_eq!(accessibility.structure.level, Some(1));
    assert_eq!(accessibility.state.modal, Some(true));
    assert_eq!(accessibility.children.len(), 1);
    assert!(accessibility.children[0].node.is_some());
    assert_eq!(accessibility.children[0].role, AccessibilityRole::Button);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Close"));
    assert_eq!(
        accessibility.children[0].relationships.controls.as_deref(),
        Some("dialog")
    );
    assert_eq!(
        accessibility.children[0].state.pressed.as_deref(),
        Some("false")
    );
}

#[test]
fn runtime_exports_platform_accessibility_tree_from_compiled_rsx() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "preferences",
              "tag": "Dialog",
              "props": {
                "attributes": {
                  "aria-label": "Preferences",
                  "aria-describedby": "preferences-help",
                  "aria-description": "Keyboard and display settings",
                  "aria-roledescription": "settings dialog",
                  "aria-level": "2",
                  "aria-posinset": "1",
                  "aria-setsize": "3",
                  "aria-hidden": "false",
                  "aria-modal": "true",
                  "aria-live": "polite"
                }
              },
              "children": [
                {
                  "kind": "element",
                  "key": "close",
                  "tag": "Button",
                  "props": {
                    "attributes": {
                      "aria-label": "Close",
                      "aria-controls": "preferences",
                      "aria-pressed": "false"
                    }
                  }
                }
              ]
            }
            "#,
    )
    .unwrap();
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_compiled(&compiled).unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert_eq!(accessibility.node, Some(root_id));
    assert_eq!(accessibility.role, AccessibilityRole::Dialog);
    assert_eq!(accessibility.label.as_deref(), Some("Preferences"));
    assert_eq!(
        accessibility.relationships.described_by.as_deref(),
        Some("preferences-help")
    );
    assert_eq!(
        accessibility.description.description.as_deref(),
        Some("Keyboard and display settings")
    );
    assert_eq!(
        accessibility.description.role_description.as_deref(),
        Some("settings dialog")
    );
    assert_eq!(accessibility.structure.level, Some(2));
    assert_eq!(accessibility.structure.position_in_set, Some(1));
    assert_eq!(accessibility.structure.set_size, Some(3));
    assert_eq!(accessibility.state.hidden, Some(false));
    assert_eq!(accessibility.state.modal, Some(true));
    assert_eq!(accessibility.state.live.as_deref(), Some("polite"));
    assert_eq!(accessibility.children.len(), 1);
    assert!(accessibility.children[0].node.is_some());
    assert_eq!(accessibility.children[0].role, AccessibilityRole::Button);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Close"));
    assert_eq!(
        accessibility.children[0].relationships.controls.as_deref(),
        Some("preferences")
    );
    assert_eq!(
        accessibility.children[0].state.pressed.as_deref(),
        Some("false")
    );
}

#[test]
fn runtime_dispatches_native_event_to_registered_rsx_action() {
    let compiled: CompiledRsxNode = serde_json::from_str(
        r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
    )
    .unwrap();
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_compiled(&compiled).unwrap();
    let blueprint = runtime.host().node(root_id).unwrap().blueprint.clone();
    let invocation = runtime
        .dispatch_event(
            &blueprint,
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Press),
        )
        .unwrap();

    assert_eq!(invocation.action, "saveDocument");
    assert_eq!(runtime.actions().invocations().len(), 1);
}

#[test]
fn runtime_keeps_diagnostics_bounded_across_many_events() {
    let element = NativeElement::new("enabled", NativeRole::Switch).with_props(
        NativeProps::new()
            .checked(false)
            .web(WebProps::new().on_change("setEnabled")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEnabled");
    let root_id = runtime.render_native(&element).unwrap();

    for _ in 0..10_000 {
        let handled = runtime
            .handle_native_event_with_changes(crate::event::NativeEvent::new(
                root_id,
                crate::event::NativeEventKind::Toggle,
            ))
            .unwrap();

        assert!(handled.invocation.is_some());
        assert_eq!(handled.interaction_changes.len(), 1);
    }

    assert_eq!(
        runtime.actions().invocations().len(),
        runtime.actions().invocation_history_limit()
    );
    assert_eq!(
        runtime.interactions().changes().len(),
        runtime.interactions().change_history_limit()
    );
    assert_eq!(
        runtime.interactions().node(root_id).unwrap().checked,
        Some(false)
    );

    let invocations = runtime.actions_mut().take_invocations();
    let changes = runtime.interactions_mut().take_changes();
    assert_eq!(invocations.len(), 256);
    assert_eq!(changes.len(), 256);
    assert!(runtime.actions().invocations().is_empty());
    assert!(runtime.interactions().changes().is_empty());
    assert_eq!(
        runtime.interactions().node(root_id).unwrap().checked,
        Some(false)
    );
}

#[test]
fn runtime_routes_button_activation_key_to_primary_action() {
    let element = NativeElement::new("save", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().on_press("saveDocument")));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("saveDocument");

    let root_id = runtime.render_native(&element).unwrap();
    let invocation = runtime
        .dispatch_native_event(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value(" Return "),
        )
        .unwrap();

    assert_eq!(invocation.action, "saveDocument");
    assert_eq!(invocation.event, crate::event::NativeEventKind::KeyDown);
    assert_eq!(invocation.value.as_deref(), Some("Enter"));
    assert_eq!(runtime.actions().invocations().len(), 1);
    assert!(runtime.interactions().changes().is_empty());
}

#[test]
fn runtime_handles_state_event_without_registered_action() {
    let element =
        NativeElement::new("save", NativeRole::Button).with_props(NativeProps::new().label("Save"));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let invocation = runtime
        .handle_native_event(crate::event::NativeEvent::new(
            root_id,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();

    assert!(invocation.is_none());
    assert!(runtime.interactions().node(root_id).unwrap().focused);
    assert!(runtime.accessibility_tree().unwrap().focused);
}

#[test]
fn runtime_initializes_first_renderable_auto_focus_node() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("hidden", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Hidden")
                    .auto_focus(true)
                    .hidden(true),
            ),
        )
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save").auto_focus(true)),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel").auto_focus(true)),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    let accessibility = runtime.accessibility_tree().unwrap();

    assert!(runtime.interactions().changes().is_empty());
    assert!(runtime.interactions().node(children[1]).unwrap().focused);
    assert_eq!(accessibility.children.len(), 2);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
    assert!(accessibility.children[0].focused);
    assert_eq!(accessibility.children[1].label.as_deref(), Some("Cancel"));
    assert!(!accessibility.children[1].focused);
}

#[test]
fn runtime_auto_focus_skips_hidden_and_inert_ancestor_subtrees() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("hidden-group", NativeRole::View)
                .with_props(NativeProps::new().hidden(true))
                .child(
                    NativeElement::new("hidden-save", NativeRole::Button)
                        .with_props(NativeProps::new().label("Hidden save").auto_focus(true)),
                ),
        )
        .child(
            NativeElement::new("inert-group", NativeRole::View)
                .with_props(NativeProps::new().inert(true))
                .child(
                    NativeElement::new("inert-save", NativeRole::Button)
                        .with_props(NativeProps::new().label("Inert save").auto_focus(true)),
                ),
        )
        .child(
            NativeElement::new("css-hidden-group", NativeRole::View)
                .with_props(NativeProps::new().web(WebProps::new().style("display", "none")))
                .child(
                    NativeElement::new("css-hidden-save", NativeRole::Button)
                        .with_props(NativeProps::new().label("CSS hidden save").auto_focus(true)),
                ),
        )
        .child(
            NativeElement::new("closed-dialog", NativeRole::Dialog)
                .with_props(NativeProps::new().html_dialog(HtmlDialogProps::default().open(false)))
                .child(
                    NativeElement::new("dialog-save", NativeRole::Button)
                        .with_props(NativeProps::new().label("Dialog save").auto_focus(true)),
                ),
        )
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save").auto_focus(true)),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    let hidden_save = runtime.host().node(children[0]).unwrap().children[0];
    let inert_save = runtime.host().node(children[1]).unwrap().children[0];
    let css_hidden_save = runtime.host().node(children[2]).unwrap().children[0];
    let dialog_save = runtime.host().node(children[3]).unwrap().children[0];
    let save = children[4];
    let accessibility = runtime.accessibility_tree().unwrap();

    assert!(runtime.interactions().node(hidden_save).is_none());
    assert!(runtime.interactions().node(inert_save).is_none());
    assert!(runtime.interactions().node(css_hidden_save).is_none());
    assert!(runtime.interactions().node(dialog_save).is_none());
    assert!(runtime.interactions().node(save).unwrap().focused);
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(accessibility.children[0].label.as_deref(), Some("Save"));
    assert!(accessibility.children[0].focused);
}

#[test]
fn runtime_auto_focus_skips_disabled_ancestor_subtrees() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("review-gate", NativeRole::FieldSet)
                .with_props(NativeProps::new().label("Review gate").disabled(true))
                .child(
                    NativeElement::new("finish-review", NativeRole::Button)
                        .with_props(NativeProps::new().label("Complete review").auto_focus(true)),
                ),
        )
        .child(
            NativeElement::new("title", NativeRole::TextField)
                .with_props(NativeProps::new().label("Task title").auto_focus(true)),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    let finish_review = runtime.host().node(children[0]).unwrap().children[0];
    let title = children[1];
    let accessibility = runtime.accessibility_tree().unwrap();

    assert!(runtime.interactions().node(finish_review).is_none());
    assert!(runtime.interactions().node(title).unwrap().focused);
    assert_eq!(
        accessibility.children[0].children[0].label.as_deref(),
        Some("Complete review")
    );
    assert!(!accessibility.children[0].children[0].focused);
    assert_eq!(
        accessibility.children[1].label.as_deref(),
        Some("Task title")
    );
    assert!(accessibility.children[1].focused);
}

#[test]
fn runtime_auto_focus_yields_to_native_focus_history() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save").auto_focus(true)),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    assert!(runtime.accessibility_tree().unwrap().children[0].focused);

    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            children[1],
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    runtime.render_native(&element).unwrap();
    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].focused);
    assert!(accessibility.children[1].focused);

    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            children[1],
            crate::event::NativeEventKind::Blur,
        ))
        .unwrap();
    runtime.render_native(&element).unwrap();
    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].focused);
    assert!(!accessibility.children[1].focused);
}

#[test]
fn runtime_auto_focus_yields_after_focused_node_is_removed() {
    let first = NativeElement::new("tools", NativeRole::Toolbar).child(
        NativeElement::new("temporary", NativeRole::TextField)
            .with_props(NativeProps::new().label("Temporary field")),
    );
    let second = NativeElement::new("tools", NativeRole::Toolbar).child(
        NativeElement::new("next", NativeRole::TextField)
            .with_props(NativeProps::new().label("Next field").auto_focus(true)),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&first).unwrap();
    let temporary = runtime.host().node(root_id).unwrap().children[0];
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            temporary,
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();

    runtime.render_native(&second).unwrap();
    let accessibility = runtime.accessibility_tree().unwrap();

    assert!(runtime.interactions().has_focus_history());
    assert_eq!(accessibility.children.len(), 1);
    assert_eq!(
        accessibility.children[0].label.as_deref(),
        Some("Next field")
    );
    assert!(!accessibility.children[0].focused);
}

#[test]
fn runtime_routes_focus_change_with_boolean_payloads() {
    let element = NativeElement::new("email", NativeRole::TextField)
        .with_props(NativeProps::new().web(WebProps::new().on_focus_change("setFocus")));
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setFocus");

    let root_id = runtime.render_native(&element).unwrap();
    let focus = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Focus)
                .value("maybe"),
        )
        .unwrap();
    let blur = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Blur)
                .value("true"),
        )
        .unwrap();

    assert_eq!(focus.event.value.as_deref(), Some("true"));
    assert_eq!(
        focus
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("true")
    );
    assert_eq!(blur.event.value.as_deref(), Some("false"));
    assert_eq!(
        blur.invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("false")
    );
}

#[test]
fn runtime_accessibility_tree_exposes_single_focused_node() {
    let element = NativeElement::new("tools", NativeRole::Toolbar)
        .child(
            NativeElement::new("save", NativeRole::Button)
                .with_props(NativeProps::new().label("Save")),
        )
        .child(
            NativeElement::new("cancel", NativeRole::Button)
                .with_props(NativeProps::new().label("Cancel")),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let children = runtime.host().node(root_id).unwrap().children.clone();
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            children[0],
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();
    runtime
        .handle_native_event(crate::event::NativeEvent::new(
            children[1],
            crate::event::NativeEventKind::Focus,
        ))
        .unwrap();

    let accessibility = runtime.accessibility_tree().unwrap();
    assert!(!accessibility.children[0].focused);
    assert!(accessibility.children[1].focused);
}
