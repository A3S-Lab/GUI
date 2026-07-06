use crate::html::HtmlActivationProps;

use super::SemanticProps;

impl SemanticProps {
    pub fn html_activation(mut self, html_activation: HtmlActivationProps) -> Self {
        self.html_activation = html_activation;
        self
    }
}
