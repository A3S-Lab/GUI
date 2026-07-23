use super::*;
use crate::accessibility::AccessibilityDescriptionProps;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::web::WebProps;

#[test]
fn activation_roles_report_the_shared_native_press_contract() {
    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let capabilities = NativeCapabilities::for_backend(backend);
        for role in [
            NativeRole::Button,
            NativeRole::DisclosureSummary,
            NativeRole::ListBoxItem,
            NativeRole::TreeItem,
        ] {
            assert_eq!(
                capabilities.support(NativeCapabilityFeature::Press, Some(role)),
                CapabilitySupport::Native
            );
            assert_eq!(
                capabilities.support(NativeCapabilityFeature::PressLifecycle, Some(role)),
                CapabilitySupport::Native
            );
            assert_eq!(
                capabilities.support(NativeCapabilityFeature::LongPress, Some(role)),
                CapabilitySupport::Native
            );
        }
    }
}

#[test]
fn native_menu_capabilities_match_each_backend_event_source() {
    for backend in [NativeBackendKind::AppKit, NativeBackendKind::Gtk4] {
        let capabilities = NativeCapabilities::for_backend(backend);
        assert_eq!(
            capabilities.support(NativeCapabilityFeature::Press, Some(NativeRole::MenuItem)),
            CapabilitySupport::Native
        );
        for feature in [
            NativeCapabilityFeature::PressLifecycle,
            NativeCapabilityFeature::LongPress,
            NativeCapabilityFeature::Move,
            NativeCapabilityFeature::InputModality,
            NativeCapabilityFeature::Hover,
        ] {
            assert_eq!(
                capabilities.support(feature, Some(NativeRole::MenuItem)),
                CapabilitySupport::Unsupported
            );
        }
    }

    let winui = NativeCapabilities::for_backend(NativeBackendKind::WinUI);
    assert_eq!(
        winui.support(
            NativeCapabilityFeature::PressLifecycle,
            Some(NativeRole::MenuItem)
        ),
        CapabilitySupport::Native
    );
}

#[test]
fn move_capability_tracks_generic_native_event_sources() {
    let target = NativeElement::new("target", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().event("onMove", "moveTarget")));
    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let capabilities = NativeCapabilities::for_backend(backend);
        assert_eq!(
            capabilities.support(NativeCapabilityFeature::Move, Some(NativeRole::View)),
            CapabilitySupport::Native
        );
        assert!(capabilities.audit_tree(&target).is_empty());
    }

    let appkit = NativeCapabilities::for_backend(NativeBackendKind::AppKit);
    assert_eq!(
        appkit.support(NativeCapabilityFeature::Move, Some(NativeRole::Popover)),
        CapabilitySupport::Unsupported
    );
    let headless = NativeCapabilities::default();
    assert_eq!(
        headless.support(NativeCapabilityFeature::Move, Some(NativeRole::View)),
        CapabilitySupport::Portable
    );
}

#[test]
fn audit_reports_portable_and_unsupported_requested_behavior() {
    let tree = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().lang("ar-EG"))
        .child(
            NativeElement::new("item", NativeRole::Tab).with_props(
                NativeProps::new().web(
                    WebProps::new()
                        .on_press_start("start")
                        .on_hover_change("hover"),
                ),
            ),
        );
    let capabilities = NativeCapabilities::for_backend(NativeBackendKind::Gtk4);

    let issues = capabilities.audit_tree(&tree);

    assert!(issues.iter().any(|issue| {
        issue.path == "root"
            && issue.feature == NativeCapabilityFeature::Locale
            && issue.support == CapabilitySupport::Portable
    }));
    assert!(issues.iter().any(|issue| {
        issue.path == "root/item"
            && issue.feature == NativeCapabilityFeature::PressLifecycle
            && issue.support == CapabilitySupport::Unsupported
    }));
    assert!(!issues
        .iter()
        .any(|issue| issue.feature == NativeCapabilityFeature::Hover));
}

#[test]
fn collection_action_items_have_no_native_capability_gap() {
    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let item = NativeElement::new("item", NativeRole::ListBoxItem).with_props(
            NativeProps::new().metadata(crate::selection::COLLECTION_ACTION_METADATA_KEY, "true"),
        );
        assert!(NativeCapabilities::for_backend(backend)
            .audit_tree(&item)
            .is_empty());
    }
}

#[test]
fn native_selection_and_focus_are_reported_only_for_supported_roles() {
    let capabilities = NativeCapabilities::for_backend(NativeBackendKind::AppKit);

    assert_eq!(
        capabilities.support(
            NativeCapabilityFeature::Selection,
            Some(NativeRole::ListBox)
        ),
        CapabilitySupport::Native
    );
    assert_eq!(
        capabilities.support(NativeCapabilityFeature::Selection, Some(NativeRole::View)),
        CapabilitySupport::Unsupported
    );
    assert_eq!(
        capabilities.support(
            NativeCapabilityFeature::FocusEvents,
            Some(NativeRole::TextField)
        ),
        CapabilitySupport::Native
    );
    assert_eq!(
        capabilities.support(NativeCapabilityFeature::FocusEvents, Some(NativeRole::Tab)),
        CapabilitySupport::Unsupported
    );
}

#[test]
fn programmatic_focus_capability_matches_the_native_binding() {
    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let capabilities = NativeCapabilities::for_backend(backend);
        assert_eq!(
            capabilities.support(
                NativeCapabilityFeature::ProgrammaticFocus,
                Some(NativeRole::Button)
            ),
            CapabilitySupport::Native
        );
        assert_eq!(
            capabilities.support(
                NativeCapabilityFeature::ProgrammaticFocus,
                Some(NativeRole::View)
            ),
            CapabilitySupport::Unsupported
        );
        for role in [NativeRole::ListBoxItem, NativeRole::TreeItem] {
            assert_eq!(
                capabilities.support(NativeCapabilityFeature::ProgrammaticFocus, Some(role)),
                CapabilitySupport::Native
            );
            assert_eq!(
                capabilities.support(NativeCapabilityFeature::FocusEvents, Some(role)),
                CapabilitySupport::Native
            );
        }
    }

    let winui = NativeCapabilities::for_backend(NativeBackendKind::WinUI);
    assert_eq!(
        winui.support(
            NativeCapabilityFeature::ProgrammaticFocus,
            Some(NativeRole::Button)
        ),
        CapabilitySupport::Native
    );
    assert_eq!(
        winui.support(
            NativeCapabilityFeature::ProgrammaticFocus,
            Some(NativeRole::Tab)
        ),
        CapabilitySupport::Native
    );
}

#[test]
fn number_field_announcements_report_the_native_os_channel() {
    let number_field = NativeElement::new("quantity", NativeRole::TextField).with_props(
        NativeProps::new()
            .input_type("number")
            .metadata(crate::native::NUMBER_FIELD_INPUT_METADATA_KEY, "true")
            .metadata(crate::native::NUMBER_FIELD_ANNOUNCE_METADATA_KEY, "true"),
    );

    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let capabilities = NativeCapabilities::for_backend(backend);
        assert_eq!(
            capabilities.support(
                NativeCapabilityFeature::AccessibilityAnnouncements,
                Some(NativeRole::TextField),
            ),
            CapabilitySupport::Native
        );
        assert!(capabilities.audit_tree(&number_field).is_empty());
    }

    assert!(NativeCapabilities::default()
        .audit_tree(&number_field)
        .iter()
        .any(|issue| {
            issue.feature == NativeCapabilityFeature::AccessibilityAnnouncements
                && issue.support == CapabilitySupport::Portable
        }));
}

#[test]
fn live_regions_request_the_native_announcement_channel() {
    let explicit = NativeElement::new("status", NativeRole::View)
        .with_props(NativeProps::new().live("polite"));
    let implicit = NativeElement::new("output", NativeRole::Output);

    for element in [&explicit, &implicit] {
        assert!(NativeCapabilities::default()
            .audit_tree(element)
            .iter()
            .any(|issue| {
                issue.feature == NativeCapabilityFeature::AccessibilityAnnouncements
                    && issue.support == CapabilitySupport::Portable
            }));
        for backend in [
            NativeBackendKind::AppKit,
            NativeBackendKind::Gtk4,
            NativeBackendKind::WinUI,
        ] {
            assert!(!NativeCapabilities::for_backend(backend)
                .audit_tree(element)
                .iter()
                .any(|issue| {
                    issue.feature == NativeCapabilityFeature::AccessibilityAnnouncements
                }));
        }
    }
}

#[test]
fn explicit_accessibility_names_report_native_projection_and_role_exceptions() {
    let named_button = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new()
            .label("Save")
            .accessibility_label("Save document"),
    );

    for backend in [
        NativeBackendKind::AppKit,
        NativeBackendKind::Gtk4,
        NativeBackendKind::WinUI,
    ] {
        let capabilities = NativeCapabilities::for_backend(backend);
        assert_eq!(
            capabilities.support(
                NativeCapabilityFeature::AccessibilityName,
                Some(NativeRole::Button),
            ),
            CapabilitySupport::Native
        );
        assert!(capabilities.audit_tree(&named_button).is_empty());
    }

    assert!(NativeCapabilities::default()
        .audit_tree(&named_button)
        .iter()
        .any(|issue| {
            issue.feature == NativeCapabilityFeature::AccessibilityName
                && issue.support == CapabilitySupport::Portable
        }));

    for (backend, role) in [
        (NativeBackendKind::AppKit, NativeRole::ListBoxItem),
        (NativeBackendKind::AppKit, NativeRole::TreeItem),
        (NativeBackendKind::AppKit, NativeRole::Tab),
        (NativeBackendKind::Gtk4, NativeRole::MenuItem),
        (NativeBackendKind::WinUI, NativeRole::Window),
    ] {
        let named = NativeElement::new("named", role)
            .with_props(NativeProps::new().accessibility_label("Accessible name"));
        let capabilities = NativeCapabilities::for_backend(backend);

        assert_eq!(
            capabilities.support(NativeCapabilityFeature::AccessibilityName, Some(role)),
            CapabilitySupport::Portable
        );
        assert!(capabilities.audit_tree(&named).iter().any(|issue| {
            issue.feature == NativeCapabilityFeature::AccessibilityName
                && issue.support == CapabilitySupport::Portable
        }));
    }
}

#[test]
fn accessibility_description_fields_report_precise_native_projection() {
    let described_button = NativeElement::new("save", NativeRole::Button).with_props(
        NativeProps::new().accessibility_description(
            AccessibilityDescriptionProps::default()
                .description("Saves the current document")
                .role_description("primary action")
                .key_shortcuts("Control+S")
                .value_text("Ready"),
        ),
    );

    let gtk4 = NativeCapabilities::for_backend(NativeBackendKind::Gtk4);
    assert!(gtk4.audit_tree(&described_button).is_empty());
    for feature in [
        NativeCapabilityFeature::AccessibilityDescription,
        NativeCapabilityFeature::AccessibilityRoleDescription,
        NativeCapabilityFeature::AccessibilityKeyShortcuts,
        NativeCapabilityFeature::AccessibilityValueText,
    ] {
        assert_eq!(
            gtk4.support(feature, Some(NativeRole::Button)),
            CapabilitySupport::Native
        );
    }

    let appkit_issues =
        NativeCapabilities::for_backend(NativeBackendKind::AppKit).audit_tree(&described_button);
    assert_eq!(appkit_issues.len(), 1);
    assert_eq!(
        appkit_issues[0].feature,
        NativeCapabilityFeature::AccessibilityKeyShortcuts
    );
    assert_eq!(appkit_issues[0].support, CapabilitySupport::Portable);

    let winui_issues =
        NativeCapabilities::for_backend(NativeBackendKind::WinUI).audit_tree(&described_button);
    assert_eq!(winui_issues.len(), 2);
    assert!(winui_issues.iter().any(|issue| {
        issue.feature == NativeCapabilityFeature::AccessibilityRoleDescription
            && issue.support == CapabilitySupport::Portable
    }));
    assert!(winui_issues.iter().any(|issue| {
        issue.feature == NativeCapabilityFeature::AccessibilityValueText
            && issue.support == CapabilitySupport::Portable
    }));

    let headless_issues = NativeCapabilities::default().audit_tree(&described_button);
    assert_eq!(headless_issues.len(), 4);
    assert!(headless_issues
        .iter()
        .all(|issue| issue.support == CapabilitySupport::Portable));
}

#[test]
fn accessibility_description_projection_reports_non_accessible_native_wrappers() {
    let description = AccessibilityDescriptionProps::default()
        .description("Description")
        .role_description("role")
        .key_shortcuts("Control+K")
        .value_text("value");

    for (backend, role) in [
        (NativeBackendKind::AppKit, NativeRole::ListBoxItem),
        (NativeBackendKind::AppKit, NativeRole::TreeItem),
        (NativeBackendKind::AppKit, NativeRole::Tab),
        (NativeBackendKind::Gtk4, NativeRole::MenuItem),
        (NativeBackendKind::WinUI, NativeRole::Window),
    ] {
        let described = NativeElement::new("described", role)
            .with_props(NativeProps::new().accessibility_description(description.clone()));
        let issues = NativeCapabilities::for_backend(backend).audit_tree(&described);

        assert_eq!(issues.len(), 4);
        assert!(issues
            .iter()
            .all(|issue| issue.support == CapabilitySupport::Portable));
    }
}

#[test]
fn manifest_round_trips_with_an_explicit_ir_version() {
    let capabilities = NativeCapabilities::for_backend(NativeBackendKind::AppKit);
    let json = serde_json::to_value(&capabilities).unwrap();

    assert_eq!(json["irVersion"], NATIVE_IR_VERSION);
    assert_eq!(json["backend"], "appKit");
    assert_eq!(
        serde_json::from_value::<NativeCapabilities>(json).unwrap(),
        capabilities
    );
}
