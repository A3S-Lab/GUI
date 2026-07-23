use super::super::*;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};
use crate::web::WebProps;

#[test]
fn runtime_clamps_text_change_values_to_max_length() {
    let element = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Name")
            .value("Ada")
            .max_length(Some(3))
            .web(WebProps::new().on_change("setName")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setName");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("aé日b"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("aé日"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("aé日")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("aé日")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("aé日")
    );
}

#[test]
fn runtime_clamps_initial_text_value_to_max_length_before_rendering() {
    let element = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Name")
            .value("aé日b")
            .max_length(Some(3)),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.max_length, Some(3));
    assert_eq!(blueprint.value.as_deref(), Some("aé日"));
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("aé日")
    );

    let updated = NativeElement::new("name", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Name")
            .value("Ada Lovelace")
            .max_length(Some(3)),
    );
    runtime.render_native(&updated).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.value.as_deref(), Some("Ada"));
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("Ada")
    );
}

#[test]
fn runtime_clamps_slider_change_values_to_range_bounds() {
    let element = NativeElement::new("estimate", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Estimate")
            .range(Some(1.0), Some(12.0), Some(6.0))
            .web(WebProps::new().on_change("setEstimate")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEstimate");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value(" 99 "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("12"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("12")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("12")
    );

    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value(" 0 "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("1"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("1")
    );
    assert_eq!(
        runtime.actions().invocations()[1].value.as_deref(),
        Some("1")
    );
}

#[test]
fn runtime_clamps_number_input_change_values_to_range_bounds() {
    let element = NativeElement::new("estimate", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Estimate")
            .input_type("number")
            .range(Some(1.0), Some(12.0), Some(6.0))
            .web(WebProps::new().on_change("setEstimate")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEstimate");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value(" 99 "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("12"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("12")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("12")
    );

    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value(" 0 "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("1"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("1")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("1")
    );
    assert_eq!(
        runtime.actions().invocations()[1].value.as_deref(),
        Some("1")
    );
}

#[test]
fn runtime_normalizes_inherited_locale_number_input_values() {
    let element = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().lang("fr-FR"))
        .child(
            NativeElement::new("estimate", NativeRole::TextField).with_props(
                NativeProps::new()
                    .label("Estimate")
                    .value("1,5")
                    .input_type("number")
                    .range(Some(1.0), Some(12.0), None)
                    .web(WebProps::new().on_change("setEstimate")),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEstimate");

    let root_id = runtime.render_native(&element).unwrap();
    let field_id = runtime.host().node(root_id).unwrap().children[0];
    let blueprint = &runtime.host().node(field_id).unwrap().blueprint;
    assert_eq!(blueprint.control_state.lang.as_deref(), Some("fr-FR"));
    assert_eq!(blueprint.control_state.current, Some(1.5));
    assert_eq!(blueprint.value.as_deref(), Some("1,5"));

    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::Change)
                .value("12,5"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("12"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("12")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().children[0]
            .value
            .as_deref(),
        Some("12")
    );
}

#[test]
fn runtime_parses_percent_number_input_changes_in_model_space() {
    let element = NativeElement::new("tax", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Tax")
            .lang("tr-TR")
            .input_type("number")
            .range(Some(0.0), Some(1.0), Some(0.25))
            .step(Some(0.01))
            .metadata("data-number-style", "percent")
            .web(WebProps::new().on_change("setTax")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setTax");

    let field_id = runtime.render_native(&element).unwrap();
    let blueprint = &runtime.host().node(field_id).unwrap().blueprint;
    assert_eq!(blueprint.control_state.current, Some(0.25));
    assert_eq!(blueprint.value.as_deref(), Some("%25"));
    assert_eq!(
        blueprint
            .control_state
            .accessibility_description
            .value_text
            .as_deref(),
        Some("%25")
    );

    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::Change)
                .value("%45"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("0.45"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("0.45")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("0.45")
    );
}

#[test]
fn runtime_steps_number_fields_with_arrow_keys_on_the_minimum_anchored_grid() {
    let element = NativeElement::new("quantity", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Quantity")
            .input_type("number")
            .range(Some(2.0), Some(11.0), Some(8.0))
            .step(Some(3.0))
            .metadata(crate::native::NUMBER_FIELD_INPUT_METADATA_KEY, "true")
            .web(WebProps::new().on_change("setQuantity")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setQuantity");

    let field_id = runtime.render_native(&element).unwrap();
    let incremented = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::KeyDown)
                .value("Up"),
        )
        .unwrap();
    assert_eq!(
        incremented.event.kind,
        crate::event::NativeEventKind::Change
    );
    assert_eq!(incremented.event.value.as_deref(), Some("11"));
    assert_eq!(
        incremented
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("11")
    );

    let decremented = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::KeyDown)
                .value("ArrowDown"),
        )
        .unwrap();
    assert_eq!(
        decremented.event.kind,
        crate::event::NativeEventKind::Change
    );
    assert_eq!(decremented.event.value.as_deref(), Some("5"));
}

#[test]
fn runtime_number_field_arrow_steps_avoid_float_noise_and_respect_bounds() {
    let element = NativeElement::new("ratio", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Ratio")
            .input_type("number")
            .range(Some(0.0), Some(0.3), Some(0.2))
            .step(Some(0.1))
            .metadata(crate::native::NUMBER_FIELD_INPUT_METADATA_KEY, "true")
            .web(WebProps::new().on_change("setRatio")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setRatio");

    let field_id = runtime.render_native(&element).unwrap();
    let incremented = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::KeyDown)
                .value("ArrowUp"),
        )
        .unwrap();
    assert_eq!(incremented.event.value.as_deref(), Some("0.3"));

    let at_max = NativeElement::new("ratio", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Ratio")
            .input_type("number")
            .range(Some(0.0), Some(0.3), Some(0.3))
            .step(Some(0.1))
            .metadata(crate::native::NUMBER_FIELD_INPUT_METADATA_KEY, "true")
            .web(WebProps::new().on_change("setRatio")),
    );
    runtime.render_native(&at_max).unwrap();
    let unchanged = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::KeyDown)
                .value("ArrowUp"),
        )
        .unwrap();
    assert_eq!(unchanged.event.kind, crate::event::NativeEventKind::KeyDown);
    assert!(unchanged.invocation.is_none());
}

#[test]
fn runtime_number_field_arrow_steps_are_suppressed_when_read_only() {
    let element = NativeElement::new("quantity", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Quantity")
            .input_type("number")
            .range(Some(0.0), Some(10.0), Some(5.0))
            .step(Some(1.0))
            .read_only(true)
            .metadata(crate::native::NUMBER_FIELD_INPUT_METADATA_KEY, "true")
            .web(WebProps::new().on_change("setQuantity")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setQuantity");

    let field_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(field_id, crate::event::NativeEventKind::KeyDown)
                .value("ArrowUp"),
        )
        .unwrap();

    assert_eq!(handled.event.kind, crate::event::NativeEventKind::KeyDown);
    assert!(handled.invocation.is_none());
    assert!(runtime.actions().invocations().is_empty());
}

#[test]
fn runtime_snaps_ranged_change_values_to_step() {
    let element = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .range(Some(0.0), Some(100.0), Some(50.0))
            .step(Some(5.0))
            .web(WebProps::new().on_change("setVolume")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setVolume");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("43"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("45"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("45")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("45")
    );
    assert_eq!(
        runtime.actions().invocations()[0].value.as_deref(),
        Some("45")
    );

    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("42"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("40"));
    assert_eq!(
        handled
            .invocation
            .as_ref()
            .and_then(|invocation| invocation.value.as_deref()),
        Some("40")
    );
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("40")
    );
    assert_eq!(
        runtime.actions().invocations()[1].value.as_deref(),
        Some("40")
    );
}

#[test]
fn runtime_suppresses_invalid_numeric_change_values() {
    let slider = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .range(Some(0.0), Some(100.0), Some(6.0))
            .web(WebProps::new().on_change("setVolume")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setVolume");

    let root_id = runtime.render_native(&slider).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value("not-a-number"),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some("not-a-number"));
    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert!(runtime.actions().invocations().is_empty());
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("6")
    );

    let number_input = NativeElement::new("estimate", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Estimate")
            .input_type("number")
            .range(Some(1.0), Some(12.0), Some(6.0))
            .web(WebProps::new().on_change("setEstimate")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setEstimate");

    let root_id = runtime.render_native(&number_input).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                .value(" "),
        )
        .unwrap();

    assert_eq!(handled.event.value.as_deref(), Some(" "));
    assert!(handled.invocation.is_none());
    assert!(handled.interaction_changes.is_empty());
    assert!(runtime.actions().invocations().is_empty());
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("6")
    );
}

#[test]
fn runtime_normalizes_initial_ranged_values_before_rendering() {
    let element = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .range(Some(0.0), Some(100.0), Some(43.0))
            .step(Some(5.0)),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.current, Some(45.0));
    assert_eq!(blueprint.value.as_deref(), Some("45"));
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("45")
    );

    let updated = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .range(Some(0.0), Some(100.0), Some(17.0))
            .step(Some(5.0)),
    );
    runtime.render_native(&updated).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.current, Some(15.0));
    assert_eq!(blueprint.value.as_deref(), Some("15"));
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("15")
    );
}

#[test]
fn runtime_normalizes_initial_number_input_values_before_rendering() {
    let element = NativeElement::new("estimate", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Estimate")
            .input_type("number")
            .range(Some(1.0), Some(12.0), Some(99.0)),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&element).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.current, Some(12.0));
    assert_eq!(blueprint.value.as_deref(), Some("12"));
    assert_eq!(
        runtime.accessibility_tree().unwrap().value.as_deref(),
        Some("12")
    );
}

#[test]
fn runtime_omits_invalid_initial_numeric_values_before_rendering() {
    let slider = NativeElement::new("volume", NativeRole::Slider).with_props(
        NativeProps::new()
            .label("Volume")
            .value("not-a-number")
            .range(Some(0.0), Some(100.0), None),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    let root_id = runtime.render_native(&slider).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.current, None);
    assert_eq!(blueprint.value, None);
    assert_eq!(runtime.accessibility_tree().unwrap().value, None);

    let number_input = NativeElement::new("estimate", NativeRole::TextField).with_props(
        NativeProps::new()
            .label("Estimate")
            .value(" ")
            .input_type("number")
            .range(Some(1.0), Some(12.0), None),
    );
    let root_id = runtime.render_native(&number_input).unwrap();
    let blueprint = &runtime.host().node(root_id).unwrap().blueprint;

    assert_eq!(blueprint.control_state.current, None);
    assert_eq!(blueprint.value, None);
    assert_eq!(runtime.accessibility_tree().unwrap().value, None);
}

#[test]
fn runtime_event_number_parser_trims_values_without_coercing_empty_input() {
    assert_eq!(parse_event_number(" 42 "), Some(42.0));
    assert_eq!(parse_event_number("\t0.5\n"), Some(0.5));
    assert_eq!(parse_event_number(" "), None);
    assert_eq!(parse_event_number("not-a-number"), None);
}

#[test]
fn runtime_event_bool_parser_canonicalizes_common_native_payloads() {
    assert_eq!(parse_event_bool(Some(" true ")), Some(true));
    assert_eq!(parse_event_bool(Some("ON")), Some(true));
    assert_eq!(parse_event_bool(Some("1")), Some(true));
    assert_eq!(parse_event_bool(Some(" false ")), Some(false));
    assert_eq!(parse_event_bool(Some("OFF")), Some(false));
    assert_eq!(parse_event_bool(Some("0")), Some(false));
    assert_eq!(parse_event_bool(Some("maybe")), None);
    assert_eq!(parse_event_bool(None), None);
}

#[test]
fn runtime_suppresses_read_only_keyboard_toggle_normalization() {
    let element = NativeElement::new("notifications", NativeRole::Switch).with_props(
        NativeProps::new()
            .label("Notifications")
            .read_only(true)
            .checked(false)
            .web(WebProps::new().on_change("setNotifications")),
    );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);
    runtime.actions_mut().register("setNotifications");

    let root_id = runtime.render_native(&element).unwrap();
    let handled = runtime
        .handle_native_event_with_changes(
            crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::KeyDown)
                .value(" "),
        )
        .unwrap();

    assert!(handled.invocation.is_none());
    assert_eq!(handled.event.kind, crate::event::NativeEventKind::Toggle);
    assert!(handled.interaction_changes.is_empty());
    assert_eq!(runtime.accessibility_tree().unwrap().checked, Some(false));
    assert!(runtime.actions().invocations().is_empty());
}
