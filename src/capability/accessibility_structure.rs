use super::*;
use crate::accessibility::AccessibilityStructureProps;

pub(super) const STRUCTURE_FEATURES: [NativeCapabilityFeature; 12] = [
    NativeCapabilityFeature::AccessibilityLevel,
    NativeCapabilityFeature::AccessibilityPositionInSet,
    NativeCapabilityFeature::AccessibilitySetSize,
    NativeCapabilityFeature::AccessibilityRowCount,
    NativeCapabilityFeature::AccessibilityRowIndex,
    NativeCapabilityFeature::AccessibilityRowSpan,
    NativeCapabilityFeature::AccessibilityColumnCount,
    NativeCapabilityFeature::AccessibilityColumnIndex,
    NativeCapabilityFeature::AccessibilityColumnSpan,
    NativeCapabilityFeature::AccessibilityRowIndexText,
    NativeCapabilityFeature::AccessibilityColumnIndexText,
    NativeCapabilityFeature::AccessibilitySort,
];

pub(super) fn requested_features(
    structure: &AccessibilityStructureProps,
) -> Vec<NativeCapabilityFeature> {
    [
        (
            NativeCapabilityFeature::AccessibilityLevel,
            structure.level.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityPositionInSet,
            structure.position_in_set.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilitySetSize,
            structure.set_size.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityRowCount,
            structure.row_count.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityRowIndex,
            structure.row_index.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityRowSpan,
            structure.row_span.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityColumnCount,
            structure.column_count.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityColumnIndex,
            structure.column_index.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityColumnSpan,
            structure.column_span.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityRowIndexText,
            structure.row_index_text.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilityColumnIndexText,
            structure.column_index_text.is_some(),
        ),
        (
            NativeCapabilityFeature::AccessibilitySort,
            structure.sort.is_some(),
        ),
    ]
    .into_iter()
    .filter_map(|(feature, requested)| requested.then_some(feature))
    .collect()
}

pub(super) fn capabilities(backend: NativeBackendKind) -> Vec<NativeFeatureCapability> {
    use CapabilitySupport::{Native, Portable};
    use NativeCapabilityFeature as Feature;

    let mut capabilities = STRUCTURE_FEATURES
        .into_iter()
        .map(|feature| {
            let support = match backend {
                NativeBackendKind::Gtk4 => Native,
                NativeBackendKind::WinUI
                    if matches!(
                        feature,
                        Feature::AccessibilityLevel
                            | Feature::AccessibilityPositionInSet
                            | Feature::AccessibilitySetSize
                    ) =>
                {
                    Native
                }
                NativeBackendKind::AppKit
                | NativeBackendKind::WinUI
                | NativeBackendKind::Headless => Portable,
            };
            NativeFeatureCapability::new(
                feature,
                support,
                Some(structure_capability_note(backend, support)),
            )
        })
        .collect::<Vec<_>>();
    capabilities.push(NativeFeatureCapability::new(
        Feature::AccessibilityStructure,
        Portable,
        Some(
            "aggregate compatibility entry; capability audits use field-level accessibility structure features",
        ),
    ));
    capabilities
}

pub(super) fn add_wrapper_overrides(
    backend: NativeBackendKind,
    role_overrides: &mut Vec<NativeRoleCapabilities>,
) {
    let (role, note) = match backend {
        NativeBackendKind::Gtk4 => (
            NativeRole::MenuItem,
            "GTK4 gio::MenuItem retains accessibility structure in portable output but has no independent GtkAccessible target",
        ),
        NativeBackendKind::WinUI => (
            NativeRole::Window,
            "the WinUI Window wrapper retains accessibility structure in portable output but is not a UIElement AutomationProperties target",
        ),
        NativeBackendKind::AppKit | NativeBackendKind::Headless => return,
    };
    for feature in STRUCTURE_FEATURES {
        set_role_capability(
            role_overrides,
            role,
            feature,
            CapabilitySupport::Portable,
            Some(note),
        );
    }
}

fn structure_capability_note(
    backend: NativeBackendKind,
    support: CapabilitySupport,
) -> &'static str {
    match backend {
        NativeBackendKind::AppKit => {
            "AppKit receives conservative structural hints where exact NSAccessibility semantics exist; the complete ARIA field remains in portable output"
        }
        NativeBackendKind::Gtk4 => {
            "the field uses its exact GtkAccessible structural property or relation"
        }
        NativeBackendKind::WinUI if support == CapabilitySupport::Native => {
            "the field uses its exact AutomationProperties attached property"
        }
        NativeBackendKind::WinUI => {
            "the field remains in portable accessibility output because WinUI has no exact generic AutomationProperties setter"
        }
        NativeBackendKind::Headless => {
            "headless mode retains accessibility structure without an OS accessibility object"
        }
    }
}
