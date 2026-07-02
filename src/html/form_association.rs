use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlFormAssociationProps {
    pub label_for: Option<String>,
    pub output_for: Option<String>,
    pub meter_low: Option<f64>,
    pub meter_high: Option<f64>,
    pub meter_optimum: Option<f64>,
}

impl HtmlFormAssociationProps {
    pub fn label_for(mut self, label_for: impl Into<String>) -> Self {
        self.label_for = Some(label_for.into());
        self
    }

    pub fn output_for(mut self, output_for: impl Into<String>) -> Self {
        self.output_for = Some(output_for.into());
        self
    }

    pub fn meter_low(mut self, meter_low: Option<f64>) -> Self {
        self.meter_low = meter_low;
        self
    }

    pub fn meter_high(mut self, meter_high: Option<f64>) -> Self {
        self.meter_high = meter_high;
        self
    }

    pub fn meter_optimum(mut self, meter_optimum: Option<f64>) -> Self {
        self.meter_optimum = meter_optimum;
        self
    }
}
