use crate::accessibility::AccessibilityDescriptionProps;
use crate::html::HtmlResourcePolicyProps;
use crate::native::{NativeElement, NativeProps, NativeRole, ValueSensitivity};
use crate::platform::{
    push_widget_setter_history, Gtk4Adapter, NativeWidgetSetter, NativeWidgetSetterBatch,
    PlatformAdapter, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
};
use std::collections::BTreeMap;

#[test]
fn setter_diagnostic_history_is_bounded_over_long_updates() {
    let mut history = Vec::new();
    for revision in 0..2_000 {
        push_widget_setter_history(
            &mut history,
            &[NativeWidgetSetter::SetLabel(Some(format!(
                "revision-{revision}"
            )))],
            ValueSensitivity::Public,
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
    }

    assert_eq!(history.len(), DEFAULT_NATIVE_SETTER_HISTORY_LIMIT);
    assert_eq!(
        history.last(),
        Some(&NativeWidgetSetter::SetLabel(Some(
            "revision-1999".to_string()
        )))
    );
}

#[test]
fn setter_diagnostic_history_redacts_sensitive_value_channels() {
    const PASSWORD: &str = "setter history password";
    let setters = [
        NativeWidgetSetter::SetValue(Some(PASSWORD.to_string())),
        NativeWidgetSetter::SetAccessibilityDescription(
            AccessibilityDescriptionProps::default().value_text(PASSWORD),
        ),
        NativeWidgetSetter::SetMetadata(BTreeMap::from([
            ("type".to_string(), "password".to_string()),
            ("value".to_string(), PASSWORD.to_string()),
            ("defaultValue".to_string(), PASSWORD.to_string()),
        ])),
    ];
    let mut history = Vec::new();

    push_widget_setter_history(
        &mut history,
        &setters,
        ValueSensitivity::Sensitive,
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
    );

    assert_eq!(history[0], NativeWidgetSetter::SetValue(None));
    assert!(matches!(
        &history[1],
        NativeWidgetSetter::SetAccessibilityDescription(description)
            if description.value_text.is_none()
    ));
    assert!(matches!(
        &history[2],
        NativeWidgetSetter::SetMetadata(metadata)
            if metadata.get("type").map(String::as_str) == Some("password")
                && metadata.len() == 1
    ));
    assert!(!format!("{history:?}").contains(PASSWORD));
}

#[test]
fn live_config_setter_and_batch_debug_omit_sensitive_value_channels() {
    const PASSWORD: &str = "live setter password";
    let before = Gtk4Adapter
        .blueprint(&NativeElement::new("password", NativeRole::TextField))
        .config();
    let mut blueprint = Gtk4Adapter.blueprint(
        &NativeElement::new("password", NativeRole::TextField).with_props(
            NativeProps::new()
                .value(PASSWORD)
                .metadata("value", PASSWORD)
                .metadata("defaultValue", PASSWORD)
                .metadata("aria-valuetext", PASSWORD)
                .accessibility_description(
                    AccessibilityDescriptionProps::default().value_text(PASSWORD),
                ),
        ),
    );
    blueprint.value_sensitivity = ValueSensitivity::Sensitive;
    let after = blueprint.config();
    let batch = NativeWidgetSetterBatch::between(&before, &after);
    let setters = [
        NativeWidgetSetter::SetValue(Some(PASSWORD.to_string())),
        NativeWidgetSetter::SetAccessibilityDescription(
            AccessibilityDescriptionProps::default().value_text(PASSWORD),
        ),
        NativeWidgetSetter::SetMetadata(BTreeMap::from([
            ("value".to_string(), PASSWORD.to_string()),
            ("defaultValue".to_string(), PASSWORD.to_string()),
        ])),
        NativeWidgetSetter::SetNonce(Some(PASSWORD.to_string())),
    ];

    assert_eq!(after.value.as_deref(), Some(PASSWORD));
    assert_eq!(
        after.metadata.get("value").map(String::as_str),
        Some(PASSWORD)
    );
    let diagnostics = format!("{after:?} {batch:?} {setters:?}");
    assert!(!diagnostics.contains(PASSWORD));
    assert!(diagnostics.contains("has_value: true"));
    assert!(diagnostics.contains("has_value_text: true"));
    assert!(diagnostics.contains("metadata_keys"));
}

#[test]
fn diagnostic_projections_remove_csp_nonce_from_duplicate_channels() {
    const NONCE: &str = "csp-nonce-credential";
    let before = Gtk4Adapter
        .blueprint(&NativeElement::new("script", NativeRole::View))
        .config();
    let element = NativeElement::new("script", NativeRole::View).with_props(
        NativeProps::new()
            .nonce(NONCE)
            .metadata("nonce", NONCE)
            .html_resource_policy(
                HtmlResourcePolicyProps::default()
                    .nonce(NONCE)
                    .frame_srcdoc(NONCE),
            ),
    );
    let blueprint = Gtk4Adapter.blueprint(&element);
    let after = blueprint.config();
    let batch = NativeWidgetSetterBatch::between(&before, &after);
    let mut history = Vec::new();
    push_widget_setter_history(
        &mut history,
        batch.as_setters(),
        ValueSensitivity::Public,
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
    );

    assert_eq!(blueprint.control_state.nonce.as_deref(), Some(NONCE));
    assert_eq!(
        blueprint.metadata.get("nonce").map(String::as_str),
        Some(NONCE)
    );
    assert_eq!(
        blueprint
            .control_state
            .html_resource_policy
            .nonce
            .as_deref(),
        Some(NONCE)
    );
    let redacted = blueprint.redacted_for_diagnostics();
    assert!(redacted.control_state.nonce.is_none());
    assert!(redacted.control_state.html_resource_policy.nonce.is_none());
    assert!(redacted
        .control_state
        .html_resource_policy
        .frame_srcdoc
        .is_none());
    assert!(!redacted.metadata.contains_key("nonce"));
    assert!(history
        .iter()
        .any(|setter| matches!(setter, NativeWidgetSetter::SetNonce(None))));
    assert!(history.iter().any(|setter| matches!(
        setter,
        NativeWidgetSetter::SetHtmlResourcePolicy(policy)
            if policy.nonce.is_none() && policy.frame_srcdoc.is_none()
    )));
    assert!(history.iter().any(|setter| matches!(
        setter,
        NativeWidgetSetter::SetMetadata(metadata) if !metadata.contains_key("nonce")
    )));

    let diagnostics = format!(
        "{element:?} {blueprint:?} {after:?} {batch:?} {history:?} {:?}",
        blueprint.control_state.html_resource_policy
    );
    assert!(!diagnostics.contains(NONCE));
    assert!(diagnostics.contains("has_nonce: true"));
}
