use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlTextAnnotationProps {
    pub cite: Option<String>,
    pub date_time: Option<String>,
}

impl HtmlTextAnnotationProps {
    pub fn cite(mut self, cite: impl Into<String>) -> Self {
        self.cite = Some(cite.into());
        self
    }

    pub fn date_time(mut self, date_time: impl Into<String>) -> Self {
        self.date_time = Some(date_time.into());
        self
    }
}
