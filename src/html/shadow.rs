use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlShadowProps {
    pub slot_name: Option<String>,
    pub part: Option<String>,
    pub export_parts: Option<String>,
}

impl HtmlShadowProps {
    pub fn slot_name(mut self, slot_name: impl Into<String>) -> Self {
        self.slot_name = Some(slot_name.into());
        self
    }

    pub fn part(mut self, part: impl Into<String>) -> Self {
        self.part = Some(part.into());
        self
    }

    pub fn export_parts(mut self, export_parts: impl Into<String>) -> Self {
        self.export_parts = Some(export_parts.into());
        self
    }
}
