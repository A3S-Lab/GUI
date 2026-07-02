use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlMicrodataProps {
    pub item_scope: bool,
    pub item_prop: Option<String>,
    pub item_type: Option<String>,
    pub item_id: Option<String>,
    pub item_ref: Option<String>,
}

impl HtmlMicrodataProps {
    pub fn item_scope(mut self, item_scope: bool) -> Self {
        self.item_scope = item_scope;
        self
    }

    pub fn item_prop(mut self, item_prop: impl Into<String>) -> Self {
        self.item_prop = Some(item_prop.into());
        self
    }

    pub fn item_type(mut self, item_type: impl Into<String>) -> Self {
        self.item_type = Some(item_type.into());
        self
    }

    pub fn item_id(mut self, item_id: impl Into<String>) -> Self {
        self.item_id = Some(item_id.into());
        self
    }

    pub fn item_ref(mut self, item_ref: impl Into<String>) -> Self {
        self.item_ref = Some(item_ref.into());
        self
    }
}
