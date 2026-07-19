use crate::accessibility::AccessibilityTreeHost;
use crate::backend::{
    CommandExecutingHost, DriverCommandExecutor, PlatformCommandExecutor, RecordingBackend,
};
use crate::host::HostNodeId;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, NativeBackendKind, PlatformAdapter, PlatformCommand};

use super::{FailingCommandExecutor, TestWidgetDriver};

const PASSWORD: &str = "correct horse battery staple";
const CHANGED_PASSWORD: &str = "changed correct horse battery staple";

#[derive(Debug, Clone, Copy)]
struct MetadataOnlyPasswordAdapter;

impl PlatformAdapter for MetadataOnlyPasswordAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }

    fn blueprint(&self, element: &NativeElement) -> crate::NativeWidgetBlueprint {
        let mut blueprint = Gtk4Adapter.blueprint(element);
        blueprint.value_sensitivity = crate::ValueSensitivity::Public;
        blueprint.widget_kind =
            crate::NativeWidgetKind::TextInput(crate::NativeTextInputKind::SingleLine);
        blueprint.control_state.input_type = None;
        blueprint
    }
}

fn password_command() -> PlatformCommand {
    PlatformCommand::Create {
        id: HostNodeId::new(1),
        blueprint: Gtk4Adapter.blueprint(
            &NativeElement::new("password", NativeRole::TextField).with_props(
                NativeProps::new()
                    .metadata("type", "password")
                    .metadata("value", PASSWORD)
                    .metadata("defaultValue", PASSWORD)
                    .metadata("aria-valuetext", PASSWORD)
                    .value(PASSWORD)
                    .accessibility_description(
                        crate::accessibility::AccessibilityDescriptionProps::default()
                            .value_text(PASSWORD),
                    ),
            ),
        ),
    }
}

#[test]
fn metadata_password_type_drives_kind_sensitivity_wire_and_accessibility_redaction() {
    let element = NativeElement::new("password", NativeRole::TextField).with_props(
        NativeProps::new()
            .input_type("  ")
            .metadata("type", "password")
            .value(PASSWORD)
            .accessibility_description(
                crate::accessibility::AccessibilityDescriptionProps::default().value_text(PASSWORD),
            ),
    );
    let blueprint = Gtk4Adapter.blueprint(&element);

    assert_eq!(
        blueprint.widget_kind,
        crate::platform::NativeWidgetKind::TextInput(
            crate::platform::NativeTextInputKind::Password
        )
    );
    assert_eq!(
        blueprint.control_state.input_type.as_deref(),
        Some("password")
    );
    assert!(blueprint.value_sensitivity.is_sensitive());
    let wire = serde_json::to_string(&blueprint).unwrap();
    assert!(!wire.contains(PASSWORD));
    assert!(!format!("{blueprint:?}").contains(PASSWORD));

    let mut low_level_blueprint = blueprint.clone();
    low_level_blueprint.value_sensitivity = crate::ValueSensitivity::Public;
    low_level_blueprint.widget_kind =
        crate::NativeWidgetKind::TextInput(crate::NativeTextInputKind::SingleLine);
    low_level_blueprint.control_state.input_type = None;
    let low_level_wire = serde_json::to_string(&low_level_blueprint).unwrap();
    assert!(!low_level_wire.contains(PASSWORD));
    assert!(!format!("{low_level_blueprint:?}").contains(PASSWORD));
    assert!(!format!("{:?}", low_level_blueprint.config()).contains(PASSWORD));

    let mut interactions = crate::InteractionState::new();
    let change = interactions
        .apply_event(
            &low_level_blueprint,
            &crate::NativeEvent::new(HostNodeId::new(1), crate::NativeEventKind::Change)
                .value(CHANGED_PASSWORD),
        )
        .unwrap();
    assert_eq!(change.after.value.as_deref(), Some(CHANGED_PASSWORD));
    assert!(interactions.changes()[0].before.value.is_none());
    assert!(interactions.changes()[0].after.value.is_none());
    let interaction_debug = format!("{interactions:?} {change:?}");
    assert!(!interaction_debug.contains(PASSWORD));
    assert!(!interaction_debug.contains(CHANGED_PASSWORD));

    let direct_accessibility = crate::AccessibilityNode::from_native(&element);
    assert!(direct_accessibility.value.is_none());
    assert!(direct_accessibility.description.value_text.is_none());

    let mut renderer = crate::renderer::Renderer::new();
    let mut host = crate::platform::PlatformPlanningHost::new(MetadataOnlyPasswordAdapter);
    renderer.render(&element, &mut host).unwrap();
    let accessibility = host.accessibility_tree().unwrap();
    assert!(accessibility.value.is_none());
    assert!(accessibility.description.value_text.is_none());
}

fn assert_redacted(command: &PlatformCommand) {
    let blueprint = match command {
        PlatformCommand::Create { blueprint, .. } | PlatformCommand::Update { blueprint, .. } => {
            blueprint
        }
        other => panic!("expected blueprint command, got {other:?}"),
    };
    assert!(blueprint.value.is_none());
    assert!(blueprint
        .control_state
        .accessibility_description
        .value_text
        .is_none());
    assert!(!blueprint.metadata.values().any(|value| value == PASSWORD));
    assert!(!format!("{command:?}").contains(PASSWORD));
}

#[test]
fn driver_command_history_redacts_password_values() {
    let mut executor = DriverCommandExecutor::new(TestWidgetDriver::default());
    executor.execute(&password_command()).unwrap();

    assert_redacted(&executor.commands()[0]);
}

#[test]
fn recording_backend_state_and_history_redact_password_values() {
    let mut backend = RecordingBackend::default();
    backend.execute(&password_command()).unwrap();

    assert!(backend.object(HostNodeId::new(1)).unwrap().value.is_none());
    assert!(!format!("{:?}", backend.object(HostNodeId::new(1)).unwrap()).contains(PASSWORD));
    assert_redacted(&backend.commands()[0]);
}

#[test]
fn degraded_batch_diagnostics_redact_password_values() {
    let host = CommandExecutingHost::new(
        Gtk4Adapter,
        FailingCommandExecutor {
            fail_creates: true,
            ..FailingCommandExecutor::default()
        },
    );
    let mut renderer = crate::renderer::Renderer::new();
    let mut host = host;
    let password = NativeElement::new("password", NativeRole::TextField).with_props(
        NativeProps::new()
            .metadata("type", "password")
            .value(PASSWORD),
    );

    renderer.render(&password, &mut host).unwrap_err();

    let degraded = host.degraded_state().unwrap();
    assert_redacted(&degraded.batch.commands[0]);
    assert!(!format!("{degraded:?}").contains(PASSWORD));
    assert!(host.planning().commands().is_empty());
}
