#[cfg(any(
    all(feature = "appkit", target_os = "macos"),
    feature = "gtk4",
    feature = "winui"
))]
use crate::backend::NativeHandleAdapter;
#[cfg(any(
    all(feature = "appkit", target_os = "macos"),
    feature = "gtk4",
    feature = "winui"
))]
use crate::host::HostNodeId;
#[cfg(any(
    all(feature = "appkit", target_os = "macos"),
    feature = "gtk4",
    feature = "winui"
))]
use crate::native::{NativeElement, NativeProps, NativeRole};
#[cfg(any(
    all(feature = "appkit", target_os = "macos"),
    feature = "gtk4",
    feature = "winui"
))]
use crate::platform::{NativeWidgetSetter, PlatformAdapter, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT};

#[cfg(any(
    all(feature = "appkit", target_os = "macos"),
    feature = "gtk4",
    feature = "winui"
))]
fn password_blueprint(adapter: &dyn PlatformAdapter) -> crate::platform::NativeWidgetBlueprint {
    let mut blueprint = adapter.blueprint(
        &NativeElement::new("password", NativeRole::TextField).with_props(
            NativeProps::new()
                .metadata("type", "password")
                .metadata("value", "adapter-password")
                .value("adapter-password"),
        ),
    );
    // Exercise the defensive metadata fallback rather than relying on the
    // adapter-populated marker or typed password kind.
    blueprint.value_sensitivity = crate::ValueSensitivity::Public;
    blueprint.widget_kind =
        crate::NativeWidgetKind::TextInput(crate::NativeTextInputKind::SingleLine);
    blueprint.control_state.input_type = None;
    blueprint
}

#[cfg(all(feature = "appkit", target_os = "macos"))]
#[test]
fn appkit_handle_setter_history_is_bounded_redacted_and_drainable() {
    let blueprint = password_blueprint(&crate::platform::AppKitAdapter);
    let mut adapter = crate::appkit::AppKitHandleAdapter::default();
    let handle = adapter
        .create_handle(HostNodeId::new(1), &blueprint)
        .unwrap();
    for _ in 0..10 {
        adapter
            .update_handle(HostNodeId::new(1), &handle, &blueprint)
            .unwrap();
    }
    let state = handle.state();
    assert_eq!(
        state.applied_setters.len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetValue(None)));
    assert!(!format!("{state:?}").contains("adapter-password"));
    assert!(!format!("{:?}", state.applied_setters).contains("adapter-password"));
    drop(state);
    assert_eq!(
        handle.take_applied_setters().len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(handle.state().applied_setters.is_empty());
}

#[cfg(feature = "gtk4")]
#[test]
fn gtk_handle_setter_history_is_bounded_redacted_and_drainable() {
    let blueprint = password_blueprint(&crate::platform::Gtk4Adapter);
    let mut adapter = crate::gtk4::Gtk4HandleAdapter::default();
    let handle = adapter
        .create_handle(HostNodeId::new(1), &blueprint)
        .unwrap();
    for _ in 0..10 {
        adapter
            .update_handle(HostNodeId::new(1), &handle, &blueprint)
            .unwrap();
    }
    let state = handle.state();
    assert_eq!(
        state.applied_setters.len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetValue(None)));
    assert!(!format!("{state:?}").contains("adapter-password"));
    assert!(!format!("{:?}", state.applied_setters).contains("adapter-password"));
    drop(state);
    assert_eq!(
        handle.take_applied_setters().len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(handle.state().applied_setters.is_empty());
}

#[cfg(feature = "winui")]
#[test]
fn winui_handle_setter_history_is_bounded_redacted_and_drainable() {
    let blueprint = password_blueprint(&crate::platform::WinUiAdapter);
    let mut adapter = crate::winui::WinUiHandleAdapter::default();
    let handle = adapter
        .create_handle(HostNodeId::new(1), &blueprint)
        .unwrap();
    for _ in 0..10 {
        adapter
            .update_handle(HostNodeId::new(1), &handle, &blueprint)
            .unwrap();
    }
    let state = handle.state();
    assert_eq!(
        state.applied_setters.len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(state
        .applied_setters
        .contains(&NativeWidgetSetter::SetValue(None)));
    assert!(!format!("{state:?}").contains("adapter-password"));
    assert!(!format!("{:?}", state.applied_setters).contains("adapter-password"));
    drop(state);
    assert_eq!(
        handle.take_applied_setters().len(),
        DEFAULT_NATIVE_SETTER_HISTORY_LIMIT
    );
    assert!(handle.state().applied_setters.is_empty());
}
