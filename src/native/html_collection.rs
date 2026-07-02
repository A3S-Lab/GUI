use crate::html::HtmlCollectionProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_collection(mut self, html_collection: HtmlCollectionProps) -> Self {
        self.html_collection = html_collection;
        self
    }

    pub fn column_span(mut self, column_span: Option<u32>) -> Self {
        self.html_collection.column_span = column_span;
        self
    }

    pub fn row_span(mut self, row_span: Option<u32>) -> Self {
        self.html_collection.row_span = row_span;
        self
    }

    pub fn headers(mut self, headers: impl Into<String>) -> Self {
        self.html_collection.headers = Some(headers.into());
        self
    }

    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.html_collection.scope = Some(scope.into());
        self
    }

    pub fn cell_abbr(mut self, cell_abbr: impl Into<String>) -> Self {
        self.html_collection.cell_abbr = Some(cell_abbr.into());
        self
    }

    pub fn list_start(mut self, list_start: Option<i32>) -> Self {
        self.html_collection.list_start = list_start;
        self
    }

    pub fn list_reversed(mut self, list_reversed: bool) -> Self {
        self.html_collection.list_reversed = list_reversed;
        self
    }

    pub fn list_type(mut self, list_type: impl Into<String>) -> Self {
        self.html_collection.list_type = Some(list_type.into());
        self
    }

    pub fn list_item_value(mut self, list_item_value: Option<i32>) -> Self {
        self.html_collection.list_item_value = list_item_value;
        self
    }
}
