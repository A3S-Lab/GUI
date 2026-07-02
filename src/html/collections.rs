use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlCollectionProps {
    pub column_span: Option<u32>,
    pub row_span: Option<u32>,
    pub headers: Option<String>,
    pub scope: Option<String>,
    pub cell_abbr: Option<String>,
    pub list_start: Option<i32>,
    pub list_reversed: bool,
    pub list_type: Option<String>,
    pub list_item_value: Option<i32>,
}

impl HtmlCollectionProps {
    pub fn column_span(mut self, column_span: Option<u32>) -> Self {
        self.column_span = column_span;
        self
    }

    pub fn row_span(mut self, row_span: Option<u32>) -> Self {
        self.row_span = row_span;
        self
    }

    pub fn headers(mut self, headers: impl Into<String>) -> Self {
        self.headers = Some(headers.into());
        self
    }

    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    pub fn cell_abbr(mut self, cell_abbr: impl Into<String>) -> Self {
        self.cell_abbr = Some(cell_abbr.into());
        self
    }

    pub fn list_start(mut self, list_start: Option<i32>) -> Self {
        self.list_start = list_start;
        self
    }

    pub fn list_reversed(mut self, list_reversed: bool) -> Self {
        self.list_reversed = list_reversed;
        self
    }

    pub fn list_type(mut self, list_type: impl Into<String>) -> Self {
        self.list_type = Some(list_type.into());
        self
    }

    pub fn list_item_value(mut self, list_item_value: Option<i32>) -> Self {
        self.list_item_value = list_item_value;
        self
    }
}
