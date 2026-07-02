use crate::html::HtmlActivationProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_activation(mut self, html_activation: HtmlActivationProps) -> Self {
        self.html_activation = html_activation;
        self
    }
}
