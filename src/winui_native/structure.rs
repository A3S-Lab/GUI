use super::*;
use crate::accessibility::structure_registry::AccessibilityStructureField;
use crate::accessibility::AccessibilityStructureProps;

impl WinUiNativeSurface {
    pub(super) fn set_accessibility_structure(
        &mut self,
        node: HostNodeId,
        structure: &AccessibilityStructureProps,
    ) -> GuiResult<()> {
        let update = self.accessibility_structures.update(node, structure);
        let Some(element) = self.widgets.get(&node).and_then(WinUiOsWidget::ui_element) else {
            return Ok(());
        };
        for field in update.changed {
            match field {
                AccessibilityStructureField::Level => map_winui(
                    "failed to set WinUI accessibility level",
                    automation::set_level(&element, update.value.level),
                )?,
                AccessibilityStructureField::PositionInSet => map_winui(
                    "failed to set WinUI accessibility position in set",
                    automation::set_position_in_set(&element, update.value.position_in_set),
                )?,
                AccessibilityStructureField::SetSize => map_winui(
                    "failed to set WinUI accessibility set size",
                    automation::set_size_of_set(&element, update.value.set_size),
                )?,
                AccessibilityStructureField::RowCount
                | AccessibilityStructureField::RowIndex
                | AccessibilityStructureField::RowSpan
                | AccessibilityStructureField::ColumnCount
                | AccessibilityStructureField::ColumnIndex
                | AccessibilityStructureField::ColumnSpan
                | AccessibilityStructureField::RowIndexText
                | AccessibilityStructureField::ColumnIndexText
                | AccessibilityStructureField::Sort => {}
            }
        }
        Ok(())
    }

    pub(super) fn forget_accessibility_structure_node(&mut self, node: HostNodeId) {
        self.accessibility_structures.remove_node(node);
    }
}
