use super::*;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::web::WebProps;

#[test]
fn headless_capabilities_are_portable_and_complete() {
    let capabilities = NativeCapabilities::default();

    assert_eq!(capabilities.backend, NativeBackendKind::Headless);
    assert!(capabilities.role_overrides.is_empty());
    assert_eq!(capabilities.features.len(), PORTABLE_FEATURES.len());
    for feature in PORTABLE_FEATURES {
        assert_eq!(
            capabilities.support(*feature, None),
            CapabilitySupport::Portable
        );
    }
}

#[test]
fn requested_semantics_are_reported_as_portable() {
    let target = NativeElement::new("target", NativeRole::Button)
        .with_props(NativeProps::new().web(WebProps::new().event("onPress", "pressTarget")));

    let issues = NativeCapabilities::default().audit_tree(&target);

    assert!(issues.iter().any(|issue| {
        issue.feature == NativeCapabilityFeature::Press
            && issue.support == CapabilitySupport::Portable
            && issue.path == "target"
    }));
}

#[test]
fn portable_capabilities_round_trip_through_json() {
    let capabilities = NativeCapabilities::default();
    let json = serde_json::to_string(&capabilities).expect("capabilities serialize");
    let decoded: NativeCapabilities =
        serde_json::from_str(&json).expect("capabilities deserialize");

    assert_eq!(decoded, capabilities);
}
