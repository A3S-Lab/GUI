use super::*;
use crate::accessibility::AccessibilityRelationshipProps;

pub(super) const RELATIONSHIP_FEATURES: [NativeCapabilityFeature; 8] = [
    NativeCapabilityFeature::AccessibilityLabelledBy,
    NativeCapabilityFeature::AccessibilityDescribedBy,
    NativeCapabilityFeature::AccessibilityDetails,
    NativeCapabilityFeature::AccessibilityControls,
    NativeCapabilityFeature::AccessibilityOwns,
    NativeCapabilityFeature::AccessibilityFlowTo,
    NativeCapabilityFeature::AccessibilityErrorMessage,
    NativeCapabilityFeature::AccessibilityActiveDescendant,
];

pub(super) fn requested_features(
    relationships: &AccessibilityRelationshipProps,
) -> Vec<NativeCapabilityFeature> {
    [
        (
            NativeCapabilityFeature::AccessibilityLabelledBy,
            relationships.labelled_by.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityDescribedBy,
            relationships.described_by.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityDetails,
            relationships.details.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityControls,
            relationships.controls.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityOwns,
            relationships.owns.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityFlowTo,
            relationships.flow_to.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityErrorMessage,
            relationships.error_message.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityActiveDescendant,
            relationships.active_descendant.is_some(),
        ),
    ]
    .into_iter()
    .filter_map(|(feature, requested)| requested.then_some(feature))
    .collect()
}

pub(super) fn add_wrapper_overrides(
    backend: NativeBackendKind,
    role_overrides: &mut Vec<NativeRoleCapabilities>,
) {
    let (role, note) = match backend {
        NativeBackendKind::Gtk4 => (
            NativeRole::MenuItem,
            "GTK4 gio::MenuItem retains accessibility relationships in portable output but has no independent GtkAccessible target",
        ),
        NativeBackendKind::WinUI => (
            NativeRole::Window,
            "the WinUI Window wrapper retains accessibility relationships in portable output but is not a UIElement AutomationProperties target",
        ),
        NativeBackendKind::AppKit | NativeBackendKind::Headless => return,
    };
    for feature in RELATIONSHIP_FEATURES {
        set_role_capability(
            role_overrides,
            role,
            feature,
            CapabilitySupport::Portable,
            Some(note),
        );
    }
}

pub(super) fn capabilities(backend: NativeBackendKind) -> Vec<NativeFeatureCapability> {
    use CapabilitySupport::{Native, Portable};
    use NativeCapabilityFeature as Feature;

    let headless = backend == NativeBackendKind::Headless;
    vec![
        NativeFeatureCapability::new(
            Feature::AccessibilityHidden,
            match backend {
                NativeBackendKind::AppKit | NativeBackendKind::Gtk4 => Native,
                NativeBackendKind::WinUI | NativeBackendKind::Headless => Portable,
            },
            Some(match backend {
                NativeBackendKind::AppKit => {
                    "aria-hidden uses the native NSAccessibility hidden state"
                }
                NativeBackendKind::Gtk4 => {
                    "aria-hidden uses the native GtkAccessible hidden state"
                }
                NativeBackendKind::WinUI => {
                    "aria-hidden is removed from the portable accessibility tree but has no exact generic WinUI attached-property setter"
                }
                NativeBackendKind::Headless => {
                    "headless mode filters aria-hidden content without an OS accessibility object"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityAutocomplete,
            if backend == NativeBackendKind::Gtk4 {
                Native
            } else {
                Portable
            },
            Some(match backend {
                NativeBackendKind::Gtk4 => {
                    "aria-autocomplete uses the native GtkAccessible autocomplete property"
                }
                NativeBackendKind::AppKit | NativeBackendKind::WinUI => {
                    "aria-autocomplete remains in portable accessibility output because this backend has no exact generic setter"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-autocomplete without an OS accessibility object"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityMultiline,
            if backend == NativeBackendKind::Gtk4 {
                Native
            } else {
                Portable
            },
            Some(match backend {
                NativeBackendKind::Gtk4 => {
                    "aria-multiline uses the native GtkAccessible multiline property"
                }
                NativeBackendKind::AppKit | NativeBackendKind::WinUI => {
                    "aria-multiline remains in portable accessibility output because this backend derives multiline behavior from the concrete control"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-multiline without an OS accessibility object"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityCurrent,
            Portable,
            Some(if headless {
                "headless mode retains aria-current without an OS accessibility object"
            } else {
                "aria-current remains in portable accessibility output because the backend has no exact generic setter"
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityHasPopup,
            Portable,
            Some(match backend {
                NativeBackendKind::Gtk4 => {
                    "GTK4 projects popup presence natively but retains the ARIA popup subtype only in portable output"
                }
                NativeBackendKind::AppKit | NativeBackendKind::WinUI => {
                    "aria-haspopup remains in portable accessibility output because popup semantics are concrete-control-specific"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-haspopup without an OS accessibility object"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityPressed,
            if backend == NativeBackendKind::Gtk4 {
                Native
            } else {
                Portable
            },
            Some(match backend {
                NativeBackendKind::Gtk4 => {
                    "aria-pressed uses the native GtkAccessible tristate pressed state"
                }
                NativeBackendKind::AppKit | NativeBackendKind::WinUI => {
                    "aria-pressed remains in portable accessibility output because the backend requires a concrete toggle control"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-pressed without an OS accessibility object"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityLiveRegion,
            if headless { Portable } else { Native },
            Some(if headless {
                "headless mode evaluates live-region policy without an OS assistive-technology channel"
            } else {
                "live, atomic, and relevant policy is evaluated by the runtime and delivered through the native announcement channel"
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityBusy,
            if backend == NativeBackendKind::Gtk4 {
                Native
            } else {
                Portable
            },
            Some(match backend {
                NativeBackendKind::Gtk4 => {
                    "aria-busy uses the native GtkAccessible busy state and gates runtime live-region announcements"
                }
                NativeBackendKind::AppKit | NativeBackendKind::WinUI => {
                    "aria-busy gates runtime live-region announcements but has no exact generic native state setter"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-busy and applies it to portable live-region policy"
                }
            }),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityModal,
            match backend {
                NativeBackendKind::AppKit | NativeBackendKind::Gtk4 => Native,
                NativeBackendKind::WinUI | NativeBackendKind::Headless => Portable,
            },
            Some(match backend {
                NativeBackendKind::AppKit => {
                    "aria-modal uses the native NSAccessibility modal state"
                }
                NativeBackendKind::Gtk4 => {
                    "aria-modal uses the native GtkAccessible modal property"
                }
                NativeBackendKind::WinUI => {
                    "generic aria-modal state remains portable; native ContentDialog roles expose modality through their automation peer"
                }
                NativeBackendKind::Headless => {
                    "headless mode retains aria-modal and applies modal background filtering without an OS accessibility object"
                }
            }),
        ),
        relationship_capability(
            Feature::AccessibilityLabelledBy,
            backend,
            true,
            false,
            "aria-labelledby uses the backend's native label relationship",
        ),
        relationship_capability(
            Feature::AccessibilityDescribedBy,
            backend,
            true,
            true,
            "aria-describedby uses native GtkAccessible or UI Automation reference collections",
        ),
        relationship_capability(
            Feature::AccessibilityDetails,
            backend,
            true,
            false,
            "aria-details has an exact generic relationship only in GtkAccessible",
        ),
        relationship_capability(
            Feature::AccessibilityControls,
            backend,
            true,
            true,
            "aria-controls uses native GtkAccessible or UI Automation controlled-peer collections",
        ),
        relationship_capability(
            Feature::AccessibilityOwns,
            backend,
            true,
            false,
            "aria-owns has an exact generic relationship only in GtkAccessible",
        ),
        relationship_capability(
            Feature::AccessibilityFlowTo,
            backend,
            true,
            true,
            "aria-flowto uses native GtkAccessible or UI Automation reading-order collections",
        ),
        relationship_capability(
            Feature::AccessibilityErrorMessage,
            backend,
            true,
            false,
            "aria-errormessage has an exact generic relationship only in GtkAccessible",
        ),
        relationship_capability(
            Feature::AccessibilityActiveDescendant,
            backend,
            true,
            false,
            "aria-activedescendant has an exact generic relationship only in GtkAccessible",
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityRelationships,
            Portable,
            Some(
                "aggregate compatibility entry; capability audits use field-level accessibility relationship features",
            ),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityState,
            Portable,
            Some(
                "aggregate compatibility entry; capability audits use field-level accessibility state features",
            ),
        ),
        NativeFeatureCapability::new(
            Feature::AccessibilityAnnouncements,
            if headless { Portable } else { Native },
            Some(if headless {
                "headless mode records announcements without an OS assistive-technology channel"
            } else {
                "announcements use the backend's native assistive-technology notification API"
            }),
        ),
    ]
}

fn relationship_capability(
    feature: NativeCapabilityFeature,
    backend: NativeBackendKind,
    gtk_native: bool,
    winui_native: bool,
    native_note: &'static str,
) -> NativeFeatureCapability {
    use CapabilitySupport::{Native, Portable};

    let support = match backend {
        NativeBackendKind::Gtk4 if gtk_native => Native,
        NativeBackendKind::WinUI if winui_native => Native,
        _ => Portable,
    };
    let note = match backend {
        NativeBackendKind::Headless => {
            "headless mode resolves relationship IDREFs without an OS accessibility object"
        }
        NativeBackendKind::AppKit
            if feature == NativeCapabilityFeature::AccessibilityLabelledBy =>
        {
            "AppKit projects one resolved static-text label through accessibilityTitleUIElement; multi-label and non-text relationships remain portable"
        }
        NativeBackendKind::AppKit => {
            "the relationship remains in portable accessibility output because AppKit has no exact generic equivalent"
        }
        NativeBackendKind::Gtk4
            if feature == NativeCapabilityFeature::AccessibilityLabelledBy =>
        {
            "GTK4 projects the complete labelled-by IDREF list through GtkAccessible"
        }
        NativeBackendKind::Gtk4 | NativeBackendKind::WinUI if support == Native => native_note,
        NativeBackendKind::Gtk4 => {
            "the relationship remains in portable accessibility output because GtkAccessible has no exact generic equivalent"
        }
        NativeBackendKind::WinUI
            if feature == NativeCapabilityFeature::AccessibilityLabelledBy =>
        {
            "WinUI projects one resolved label through AutomationProperties.LabeledBy; multi-label relationships remain portable"
        }
        NativeBackendKind::WinUI => {
            "the relationship remains in portable accessibility output because UI Automation has no exact generic equivalent"
        }
    };
    NativeFeatureCapability::new(feature, support, Some(note))
}
