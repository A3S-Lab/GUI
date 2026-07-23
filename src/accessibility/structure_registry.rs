use std::collections::BTreeMap;

use super::structure::{
    normalize_grid_count, normalize_grid_index, normalize_grid_span, normalize_position_in_set,
    normalize_positive_u32, normalize_set_size, normalize_sort, AccessibilitySortValue,
};
use super::AccessibilityStructureProps;
use crate::host::HostNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessibilityStructureField {
    Level,
    PositionInSet,
    SetSize,
    RowCount,
    RowIndex,
    RowSpan,
    ColumnCount,
    ColumnIndex,
    ColumnSpan,
    RowIndexText,
    ColumnIndexText,
    Sort,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NormalizedAccessibilityStructure {
    pub(crate) level: Option<i32>,
    pub(crate) position_in_set: Option<i32>,
    pub(crate) set_size: Option<i32>,
    pub(crate) row_count: Option<i32>,
    pub(crate) row_index: Option<i32>,
    pub(crate) row_span: Option<i32>,
    pub(crate) column_count: Option<i32>,
    pub(crate) column_index: Option<i32>,
    pub(crate) column_span: Option<i32>,
    pub(crate) row_index_text: Option<String>,
    pub(crate) column_index_text: Option<String>,
    pub(crate) sort: Option<AccessibilitySortValue>,
}

impl NormalizedAccessibilityStructure {
    fn from_props(structure: &AccessibilityStructureProps) -> Self {
        let set_size = normalize_set_size(structure.set_size);
        let row_count = normalize_grid_count(structure.row_count);
        let row_index = normalize_grid_index(structure.row_index, row_count);
        let column_count = normalize_grid_count(structure.column_count);
        let column_index = normalize_grid_index(structure.column_index, column_count);
        Self {
            level: normalize_positive_u32(structure.level),
            position_in_set: normalize_position_in_set(structure.position_in_set, set_size),
            set_size,
            row_count,
            row_index,
            row_span: normalize_grid_span(structure.row_span, row_index, row_count),
            column_count,
            column_index,
            column_span: normalize_grid_span(structure.column_span, column_index, column_count),
            row_index_text: structure.row_index_text.clone(),
            column_index_text: structure.column_index_text.clone(),
            sort: structure.sort.as_deref().and_then(normalize_sort),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct AccessibilityStructureUpdate {
    pub(crate) value: NormalizedAccessibilityStructure,
    pub(crate) changed: Vec<AccessibilityStructureField>,
}

#[derive(Debug, Default)]
pub(crate) struct AccessibilityStructureRegistry {
    structures: BTreeMap<HostNodeId, NormalizedAccessibilityStructure>,
}

impl AccessibilityStructureRegistry {
    pub(crate) fn update(
        &mut self,
        node: HostNodeId,
        structure: &AccessibilityStructureProps,
    ) -> AccessibilityStructureUpdate {
        let value = NormalizedAccessibilityStructure::from_props(structure);
        let previous = self.structures.get(&node).cloned().unwrap_or_default();
        let changed = changed_fields(&previous, &value);
        if value == NormalizedAccessibilityStructure::default() {
            self.structures.remove(&node);
        } else {
            self.structures.insert(node, value.clone());
        }
        AccessibilityStructureUpdate { value, changed }
    }

    pub(crate) fn remove_node(&mut self, node: HostNodeId) {
        self.structures.remove(&node);
    }
}

fn changed_fields(
    previous: &NormalizedAccessibilityStructure,
    current: &NormalizedAccessibilityStructure,
) -> Vec<AccessibilityStructureField> {
    [
        (
            AccessibilityStructureField::Level,
            previous.level != current.level,
        ),
        (
            AccessibilityStructureField::PositionInSet,
            previous.position_in_set != current.position_in_set,
        ),
        (
            AccessibilityStructureField::SetSize,
            previous.set_size != current.set_size,
        ),
        (
            AccessibilityStructureField::RowCount,
            previous.row_count != current.row_count,
        ),
        (
            AccessibilityStructureField::RowIndex,
            previous.row_index != current.row_index,
        ),
        (
            AccessibilityStructureField::RowSpan,
            previous.row_span != current.row_span,
        ),
        (
            AccessibilityStructureField::ColumnCount,
            previous.column_count != current.column_count,
        ),
        (
            AccessibilityStructureField::ColumnIndex,
            previous.column_index != current.column_index,
        ),
        (
            AccessibilityStructureField::ColumnSpan,
            previous.column_span != current.column_span,
        ),
        (
            AccessibilityStructureField::RowIndexText,
            previous.row_index_text != current.row_index_text,
        ),
        (
            AccessibilityStructureField::ColumnIndexText,
            previous.column_index_text != current.column_index_text,
        ),
        (
            AccessibilityStructureField::Sort,
            previous.sort != current.sort,
        ),
    ]
    .into_iter()
    .filter_map(|(field, changed)| changed.then_some(field))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_reports_all_explicit_structure_fields_once() {
        let node = HostNodeId::new(1);
        let structure = AccessibilityStructureProps::default()
            .level(Some(2))
            .position_in_set(Some(3))
            .set_size(Some(10))
            .row_count(Some(100))
            .row_index(Some(4))
            .row_span(Some(2))
            .column_count(Some(8))
            .column_index(Some(3))
            .column_span(Some(2))
            .row_index_text("row four")
            .column_index_text("column three")
            .sort("ascending");
        let mut registry = AccessibilityStructureRegistry::default();

        let first = registry.update(node, &structure);
        assert_eq!(first.changed.len(), 12);
        assert_eq!(first.value.level, Some(2));
        assert_eq!(first.value.sort, Some(AccessibilitySortValue::Ascending));
        assert!(registry.update(node, &structure).changed.is_empty());
    }

    #[test]
    fn invalid_or_removed_values_clear_only_the_affected_native_fields() {
        let node = HostNodeId::new(1);
        let mut registry = AccessibilityStructureRegistry::default();
        registry.update(
            node,
            &AccessibilityStructureProps::default()
                .level(Some(2))
                .position_in_set(Some(2))
                .set_size(Some(4))
                .sort("descending"),
        );

        let update = registry.update(
            node,
            &AccessibilityStructureProps::default()
                .level(Some(0))
                .set_size(Some(4))
                .sort("sideways"),
        );

        assert_eq!(
            update.changed,
            [
                AccessibilityStructureField::Level,
                AccessibilityStructureField::PositionInSet,
                AccessibilityStructureField::Sort,
            ]
        );
        assert_eq!(update.value.set_size, Some(4));
        assert_eq!(update.value.level, None);
        assert_eq!(update.value.sort, None);
    }

    #[test]
    fn removing_a_node_discards_its_structure_snapshot() {
        let node = HostNodeId::new(1);
        let structure = AccessibilityStructureProps::default().level(Some(2));
        let mut registry = AccessibilityStructureRegistry::default();
        registry.update(node, &structure);

        registry.remove_node(node);

        assert_eq!(registry.update(node, &structure).changed.len(), 1);
    }
}
