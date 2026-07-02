use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlDialogProps {
    pub open: Option<bool>,
}

impl HtmlDialogProps {
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }
}
