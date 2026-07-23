use super::*;
use crate::accessibility::structure::AccessibilitySortValue;
use crate::accessibility::structure_registry::AccessibilityStructureField;
use crate::accessibility::AccessibilityStructureProps;

impl AppKitNativeSurface {
    pub(super) fn set_accessibility_structure(
        &mut self,
        node: HostNodeId,
        structure: &AccessibilityStructureProps,
    ) {
        let update = self.accessibility_structures.update(node, structure);
        let Some(view) = self.widgets.get(&node).and_then(AppKitOsWidget::as_view) else {
            return;
        };
        for field in &update.changed {
            match field {
                AccessibilityStructureField::Level => {
                    let level = update
                        .value
                        .level
                        .map(|value| value.saturating_sub(1) as NSInteger)
                        .unwrap_or_default();
                    view.setAccessibilityDisclosureLevel(level);
                }
                AccessibilityStructureField::RowCount => {
                    view.setAccessibilityRowCount(native_count(update.value.row_count));
                }
                AccessibilityStructureField::ColumnCount => {
                    view.setAccessibilityColumnCount(native_count(update.value.column_count));
                }
                AccessibilityStructureField::RowIndex | AccessibilityStructureField::RowSpan => {
                    view.setAccessibilityRowIndexRange(native_index_range(
                        update.value.row_index,
                        update.value.row_span,
                    ));
                }
                AccessibilityStructureField::ColumnIndex
                | AccessibilityStructureField::ColumnSpan => {
                    view.setAccessibilityColumnIndexRange(native_index_range(
                        update.value.column_index,
                        update.value.column_span,
                    ));
                }
                AccessibilityStructureField::Sort => {
                    view.setAccessibilitySortDirection(native_sort(update.value.sort));
                }
                AccessibilityStructureField::PositionInSet
                | AccessibilityStructureField::SetSize
                | AccessibilityStructureField::RowIndexText
                | AccessibilityStructureField::ColumnIndexText => {}
            }
        }
    }

    pub(super) fn forget_accessibility_structure_node(&mut self, node: HostNodeId) {
        self.accessibility_structures.remove_node(node);
    }
}

fn native_count(value: Option<i32>) -> NSInteger {
    value.filter(|value| *value >= 0).unwrap_or_default() as NSInteger
}

fn native_index_range(index: Option<i32>, span: Option<i32>) -> NSRange {
    let Some(index) = index.and_then(|index| usize::try_from(index - 1).ok()) else {
        return NSRange::new(0, 0);
    };
    let span = span
        .and_then(|span| usize::try_from(span).ok())
        .unwrap_or(1);
    NSRange::new(index, span)
}

fn native_sort(value: Option<AccessibilitySortValue>) -> NSAccessibilitySortDirection {
    match value {
        Some(AccessibilitySortValue::Ascending) => NSAccessibilitySortDirection::Ascending,
        Some(AccessibilitySortValue::Descending) => NSAccessibilitySortDirection::Descending,
        Some(AccessibilitySortValue::None | AccessibilitySortValue::Other) | None => {
            NSAccessibilitySortDirection::Unknown
        }
    }
}
