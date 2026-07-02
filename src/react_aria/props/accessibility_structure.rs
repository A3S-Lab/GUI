use crate::accessibility::AccessibilityStructureProps;

use super::AriaProps;

impl AriaProps {
    pub fn accessibility_structure(
        mut self,
        accessibility_structure: AccessibilityStructureProps,
    ) -> Self {
        self.accessibility_structure = accessibility_structure;
        self
    }

    pub fn accessibility_level(mut self, level: Option<u32>) -> Self {
        self.accessibility_structure.level = level;
        self
    }

    pub fn accessibility_position_in_set(mut self, position_in_set: Option<i32>) -> Self {
        self.accessibility_structure.position_in_set = position_in_set;
        self
    }

    pub fn accessibility_set_size(mut self, set_size: Option<i32>) -> Self {
        self.accessibility_structure.set_size = set_size;
        self
    }

    pub fn accessibility_row_count(mut self, row_count: Option<i32>) -> Self {
        self.accessibility_structure.row_count = row_count;
        self
    }

    pub fn accessibility_row_index(mut self, row_index: Option<i32>) -> Self {
        self.accessibility_structure.row_index = row_index;
        self
    }

    pub fn accessibility_row_span(mut self, row_span: Option<u32>) -> Self {
        self.accessibility_structure.row_span = row_span;
        self
    }

    pub fn accessibility_column_count(mut self, column_count: Option<i32>) -> Self {
        self.accessibility_structure.column_count = column_count;
        self
    }

    pub fn accessibility_column_index(mut self, column_index: Option<i32>) -> Self {
        self.accessibility_structure.column_index = column_index;
        self
    }

    pub fn accessibility_column_span(mut self, column_span: Option<u32>) -> Self {
        self.accessibility_structure.column_span = column_span;
        self
    }

    pub fn accessibility_row_index_text(mut self, row_index_text: impl Into<String>) -> Self {
        self.accessibility_structure.row_index_text = Some(row_index_text.into());
        self
    }

    pub fn accessibility_column_index_text(mut self, column_index_text: impl Into<String>) -> Self {
        self.accessibility_structure.column_index_text = Some(column_index_text.into());
        self
    }

    pub fn accessibility_sort(mut self, sort: impl Into<String>) -> Self {
        self.accessibility_structure.sort = Some(sort.into());
        self
    }
}
