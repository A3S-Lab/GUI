use super::*;
use crate::accessibility::AccessibilityRelationshipProps;

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
