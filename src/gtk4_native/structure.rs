use super::*;
use crate::accessibility::structure::AccessibilitySortValue;
use crate::accessibility::structure_registry::AccessibilityStructureField;
use crate::accessibility::AccessibilityStructureProps;

impl Gtk4NativeSurface {
    pub(super) fn set_accessibility_structure(
        &mut self,
        node: HostNodeId,
        structure: &AccessibilityStructureProps,
    ) {
        let update = self.accessibility_structures.update(node, structure);
        let Some(widget) = self.widgets.get(&node) else {
            return;
        };
        for field in update.changed {
            match field {
                AccessibilityStructureField::Level => {
                    if let Some(value) = update.value.level {
                        widget.update_property(&[gtk::accessible::Property::Level(value)]);
                    } else {
                        widget.reset_property(gtk::AccessibleProperty::Level);
                    }
                }
                AccessibilityStructureField::PositionInSet => update_relation(
                    widget,
                    update.value.position_in_set,
                    gtk::accessible::Relation::PosInSet,
                    gtk::AccessibleRelation::PosInSet,
                ),
                AccessibilityStructureField::SetSize => update_relation(
                    widget,
                    update.value.set_size,
                    gtk::accessible::Relation::SetSize,
                    gtk::AccessibleRelation::SetSize,
                ),
                AccessibilityStructureField::RowCount => update_relation(
                    widget,
                    update.value.row_count,
                    gtk::accessible::Relation::RowCount,
                    gtk::AccessibleRelation::RowCount,
                ),
                AccessibilityStructureField::RowIndex => update_relation(
                    widget,
                    update.value.row_index,
                    gtk::accessible::Relation::RowIndex,
                    gtk::AccessibleRelation::RowIndex,
                ),
                AccessibilityStructureField::RowSpan => update_relation(
                    widget,
                    update.value.row_span,
                    gtk::accessible::Relation::RowSpan,
                    gtk::AccessibleRelation::RowSpan,
                ),
                AccessibilityStructureField::ColumnCount => update_relation(
                    widget,
                    update.value.column_count,
                    gtk::accessible::Relation::ColCount,
                    gtk::AccessibleRelation::ColCount,
                ),
                AccessibilityStructureField::ColumnIndex => update_relation(
                    widget,
                    update.value.column_index,
                    gtk::accessible::Relation::ColIndex,
                    gtk::AccessibleRelation::ColIndex,
                ),
                AccessibilityStructureField::ColumnSpan => update_relation(
                    widget,
                    update.value.column_span,
                    gtk::accessible::Relation::ColSpan,
                    gtk::AccessibleRelation::ColSpan,
                ),
                AccessibilityStructureField::RowIndexText => {
                    if let Some(value) = update.value.row_index_text.as_deref() {
                        widget.update_relation(&[gtk::accessible::Relation::RowIndexText(value)]);
                    } else {
                        widget.reset_relation(gtk::AccessibleRelation::RowIndexText);
                    }
                }
                AccessibilityStructureField::ColumnIndexText => {
                    if let Some(value) = update.value.column_index_text.as_deref() {
                        widget.update_relation(&[gtk::accessible::Relation::ColIndexText(value)]);
                    } else {
                        widget.reset_relation(gtk::AccessibleRelation::ColIndexText);
                    }
                }
                AccessibilityStructureField::Sort => {
                    if let Some(value) = update.value.sort {
                        widget.update_property(&[gtk::accessible::Property::Sort(
                            gtk_accessible_sort(value),
                        )]);
                    } else {
                        widget.reset_property(gtk::AccessibleProperty::Sort);
                    }
                }
            }
        }
    }

    pub(super) fn forget_accessibility_structure_node(&mut self, node: HostNodeId) {
        self.accessibility_structures.remove_node(node);
    }
}

fn update_relation(
    widget: &gtk::Widget,
    value: Option<i32>,
    relation: impl FnOnce(i32) -> gtk::accessible::Relation<'static>,
    kind: gtk::AccessibleRelation,
) {
    if let Some(value) = value {
        widget.update_relation(&[relation(value)]);
    } else {
        widget.reset_relation(kind);
    }
}

fn gtk_accessible_sort(value: AccessibilitySortValue) -> gtk::AccessibleSort {
    match value {
        AccessibilitySortValue::None => gtk::AccessibleSort::None,
        AccessibilitySortValue::Ascending => gtk::AccessibleSort::Ascending,
        AccessibilitySortValue::Descending => gtk::AccessibleSort::Descending,
        AccessibilitySortValue::Other => gtk::AccessibleSort::Other,
    }
}
