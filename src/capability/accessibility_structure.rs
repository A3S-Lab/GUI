use super::*;
use crate::accessibility::AccessibilityStructureProps;

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
