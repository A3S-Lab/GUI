use super::*;
use crate::backend::{CommandExecutingHost, RecordingBackend};
use crate::event::NativeEventKind;
use crate::input::{NativeEventContext, NativeInputModality};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{
    AppKitAdapter, Gtk4Adapter, PlatformAdapter, PlatformCommand, PlatformPlanningHost,
    WinUiAdapter,
};
use crate::web::WebProps;

fn runtime() -> GuiRuntime<PlatformPlanningHost<Gtk4Adapter>> {
    GuiRuntime::new(PlatformPlanningHost::new(Gtk4Adapter))
}

fn projected_opacity(
    runtime: &GuiRuntime<PlatformPlanningHost<Gtk4Adapter>>,
    node: HostNodeId,
) -> Option<f64> {
    runtime
        .host()
        .node(node)
        .and_then(|node| node.blueprint.portable_style.opacity)
}

fn assert_adapter_projects_hover_style<A: PlatformAdapter>(adapter: A) {
    let mut runtime = GuiRuntime::new(PlatformPlanningHost::new(adapter));
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new().web(WebProps::new().class_name("opacity-50 hover:opacity-100")),
            ),
        )
        .unwrap();

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();

    assert_eq!(
        runtime
            .host()
            .node(button)
            .unwrap()
            .blueprint
            .portable_style
            .opacity,
        Some(1.0)
    );
}

#[test]
fn every_platform_planning_adapter_receives_interaction_style_updates() {
    assert_adapter_projects_hover_style(AppKitAdapter);
    assert_adapter_projects_hover_style(Gtk4Adapter);
    assert_adapter_projects_hover_style(WinUiAdapter);
}

#[test]
fn command_executor_receives_the_resolved_portable_style_update() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut runtime = GuiRuntime::new(host);
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new().web(WebProps::new().class_name("opacity-50 hover:opacity-100")),
            ),
        )
        .unwrap();

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();

    let Some(PlatformCommand::Update { id, blueprint }) =
        runtime.host().executor().commands().last()
    else {
        panic!("resolved style update command");
    };
    assert_eq!(*id, button);
    assert_eq!(blueprint.portable_style.opacity, Some(1.0));
    assert!(runtime.host().planning().commands().is_empty());
}

#[test]
fn hover_and_press_variants_update_native_style_without_action_handlers() {
    let mut runtime = runtime();
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new().web(
                    WebProps::new().class_name("opacity-50 hover:opacity-75 active:opacity-25"),
                ),
            ),
        )
        .unwrap();

    assert_eq!(projected_opacity(&runtime, button), Some(0.5));

    let hovered = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert!(hovered.invocations.is_empty());
    assert_eq!(projected_opacity(&runtime, button), Some(0.75));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::PressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, button), Some(0.25));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::PressEnd)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, button), Some(0.75));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverEnd)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, button), Some(0.5));
}

#[test]
fn react_aria_data_pressed_variant_tracks_runtime_press_state() {
    let mut runtime = runtime();
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new()
                    .web(WebProps::new().class_name("opacity-50 data-[pressed=true]:opacity-100")),
            ),
        )
        .unwrap();

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::PressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, button), Some(1.0));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::PressCancel)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, button), Some(0.5));
}

#[test]
fn long_press_and_move_data_variants_track_runtime_lifecycles() {
    let mut runtime = runtime();
    let target = runtime
        .render_native(&NativeElement::new("target", NativeRole::View).with_props(
            NativeProps::new().web(WebProps::new().class_name(
                "opacity-25 data-[long-pressed=true]:opacity-50 \
                 data-[moving=true]:opacity-75",
            )),
        ))
        .unwrap();

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(target, NativeEventKind::LongPressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Touch)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, target), Some(0.5));
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(target, NativeEventKind::LongPressEnd)
                .context(NativeEventContext::new().modality(NativeInputModality::Touch)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, target), Some(0.25));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(target, NativeEventKind::MoveStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Touch)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, target), Some(0.75));
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(target, NativeEventKind::MoveEnd)
                .context(NativeEventContext::new().modality(NativeInputModality::Touch)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, target), Some(0.25));
}

#[test]
fn focus_visible_style_follows_global_input_modality() {
    let mut runtime = runtime();
    let root =
        runtime
            .render_native(
                &NativeElement::new("root", NativeRole::View)
                    .child(NativeElement::new("first", NativeRole::Button).with_props(
                        NativeProps::new().web(
                            WebProps::new().class_name("opacity-50 focus-visible:opacity-100"),
                        ),
                    ))
                    .child(NativeElement::new("second", NativeRole::Button)),
            )
            .unwrap();
    let children = runtime.host().node(root).unwrap().children.clone();
    let first = children[0];
    let second = children[1];

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(first, NativeEventKind::Focus)
                .context(NativeEventContext::new().modality(NativeInputModality::Keyboard)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, first), Some(1.0));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(second, NativeEventKind::PressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, first), Some(0.5));
}

#[test]
fn focus_within_variant_tracks_descendants_without_focus_callbacks() {
    let mut runtime = runtime();
    let root = runtime
        .render_native(
            &NativeElement::new("group", NativeRole::View)
                .with_props(
                    NativeProps::new()
                        .web(WebProps::new().class_name("opacity-50 focus-within:opacity-100")),
                )
                .child(NativeElement::new("field", NativeRole::TextField)),
        )
        .unwrap();
    let field = runtime.host().node(root).unwrap().children[0];

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(field, NativeEventKind::Focus)
                .context(NativeEventContext::new().modality(NativeInputModality::Keyboard)),
        )
        .unwrap();

    assert!(runtime.interactions().node(root).unwrap().focus_within);
    assert_eq!(projected_opacity(&runtime, root), Some(1.0));

    runtime
        .handle_native_event_with_changes(NativeEvent::new(field, NativeEventKind::Blur))
        .unwrap();
    assert_eq!(projected_opacity(&runtime, root), Some(0.5));
}

#[test]
fn focus_visible_within_requires_descendant_focus_and_visible_modality() {
    let mut runtime = runtime();
    let root = runtime
        .render_native(
            &NativeElement::new("group", NativeRole::View)
                .with_props(
                    NativeProps::new().web(
                        WebProps::new()
                            .class_name("opacity-50 data-[focus-visible-within=true]:opacity-100"),
                    ),
                )
                .child(NativeElement::new("field", NativeRole::TextField))
                .child(NativeElement::new("pointer-target", NativeRole::Button)),
        )
        .unwrap();
    let children = runtime.host().node(root).unwrap().children.clone();
    let field = children[0];
    let pointer_target = children[1];

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(field, NativeEventKind::Focus)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, root), Some(0.5));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(field, NativeEventKind::KeyDown)
                .value("Tab")
                .context(NativeEventContext::new().modality(NativeInputModality::Keyboard)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, root), Some(1.0));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(pointer_target, NativeEventKind::PressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, root), Some(0.5));
}

#[test]
fn static_disabled_and_data_state_variants_are_projected_on_render() {
    let mut runtime = runtime();
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new().disabled(true).web(
                    WebProps::new().class_name(
                        "opacity-100 disabled:opacity-50 data-[disabled=true]:opacity-25",
                    ),
                ),
            ),
        )
        .unwrap();

    assert_eq!(projected_opacity(&runtime, button), Some(0.25));
}

#[test]
fn checked_variant_tracks_native_toggle_state() {
    let mut runtime = runtime();
    let checkbox = runtime
        .render_native(
            &NativeElement::new("checkbox", NativeRole::Checkbox).with_props(
                NativeProps::new()
                    .checked(false)
                    .web(WebProps::new().class_name("opacity-50 checked:opacity-100")),
            ),
        )
        .unwrap();

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(checkbox, NativeEventKind::Toggle).value("true"),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, checkbox), Some(1.0));

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(checkbox, NativeEventKind::Toggle).value("false"),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, checkbox), Some(0.5));
}

#[test]
fn selection_projection_updates_selected_styles_and_preserves_hover_styles() {
    let mut runtime = runtime();
    let root = runtime
        .render_native(
            &NativeElement::new("people", NativeRole::ListBox)
                .with_props(
                    NativeProps::new()
                        .web(WebProps::new().attribute("data-selection-mode", "single")),
                )
                .child(
                    NativeElement::new("ada", NativeRole::ListBoxItem).with_props(
                        NativeProps::new()
                            .value("Ada")
                            .web(WebProps::new().class_name("opacity-25 hover:opacity-50")),
                    ),
                )
                .child(
                    NativeElement::new("linus", NativeRole::ListBoxItem).with_props(
                        NativeProps::new()
                            .value("Linus")
                            .web(WebProps::new().class_name("opacity-25 selected:opacity-75")),
                    ),
                ),
        )
        .unwrap();
    let items = runtime.renderer.child_ids(root);

    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(items[0], NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            items[0],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();
    assert_eq!(projected_opacity(&runtime, items[0]), Some(0.5));

    runtime
        .handle_native_event_with_changes(NativeEvent::new(
            items[1],
            NativeEventKind::SelectionChange,
        ))
        .unwrap();
    assert_eq!(projected_opacity(&runtime, items[0]), Some(0.5));
    assert_eq!(projected_opacity(&runtime, items[1]), Some(0.75));
}

#[test]
fn rerender_reapplies_active_interaction_style_after_declarative_update() {
    let mut runtime = runtime();
    let first = NativeElement::new("button", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Before")
            .web(WebProps::new().class_name("opacity-50 hover:opacity-100")),
    );
    let second = NativeElement::new("button", NativeRole::Button).with_props(
        NativeProps::new()
            .label("After")
            .web(WebProps::new().class_name("opacity-50 hover:opacity-100")),
    );
    let button = runtime.render_native(&first).unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();

    let rerendered = runtime.render_native(&second).unwrap();

    assert_eq!(rerendered, button);
    assert_eq!(
        runtime
            .host()
            .node(button)
            .unwrap()
            .blueprint
            .label
            .as_deref(),
        Some("After")
    );
    assert_eq!(projected_opacity(&runtime, button), Some(1.0));
}

#[test]
fn identical_rerender_reapplies_active_style_after_base_reconciliation() {
    let mut runtime = runtime();
    let element = NativeElement::new("button", NativeRole::Button).with_props(
        NativeProps::new().web(WebProps::new().class_name("opacity-50 hover:opacity-100")),
    );
    let button = runtime.render_native(&element).unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();

    assert_eq!(runtime.render_native(&element).unwrap(), button);
    assert_eq!(projected_opacity(&runtime, button), Some(1.0));
}

#[test]
fn rerender_refreshes_declarative_state_without_losing_transient_state() {
    let mut runtime = runtime();
    let first = NativeElement::new("checkbox", NativeRole::Checkbox).with_props(
        NativeProps::new()
            .checked(false)
            .web(WebProps::new().class_name("opacity-25 checked:opacity-75 hover:cursor-pointer")),
    );
    let checkbox = runtime.render_native(&first).unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(checkbox, NativeEventKind::Toggle).value("true"),
        )
        .unwrap();
    runtime
        .handle_native_event_with_changes(
            NativeEvent::new(checkbox, NativeEventKind::HoverStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap();
    assert_eq!(projected_opacity(&runtime, checkbox), Some(0.75));

    let second = NativeElement::new("checkbox", NativeRole::Checkbox).with_props(
        NativeProps::new()
            .checked(false)
            .web(WebProps::new().class_name("opacity-25 checked:opacity-75 hover:cursor-pointer")),
    );
    runtime.render_native(&second).unwrap();

    assert_eq!(projected_opacity(&runtime, checkbox), Some(0.25));
    assert!(runtime.interactions().node(checkbox).unwrap().hovered);
}

#[test]
fn rejected_action_rolls_back_projected_interaction_style() {
    let mut runtime = runtime();
    let button = runtime
        .render_native(
            &NativeElement::new("button", NativeRole::Button).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .class_name("opacity-50 active:opacity-100")
                        .on_press_start("missingAction"),
                ),
            ),
        )
        .unwrap();

    let error = runtime
        .handle_native_event_with_changes(
            NativeEvent::new(button, NativeEventKind::PressStart)
                .context(NativeEventContext::new().modality(NativeInputModality::Mouse)),
        )
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("unregistered action missingAction"));
    assert!(runtime
        .interactions()
        .node(button)
        .is_none_or(|state| !state.pressed));
    assert_eq!(projected_opacity(&runtime, button), Some(0.5));
}
