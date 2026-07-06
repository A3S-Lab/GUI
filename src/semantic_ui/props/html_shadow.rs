use crate::html::HtmlShadowProps;

use super::SemanticProps;

impl SemanticProps {
    pub fn html_shadow(mut self, html_shadow: HtmlShadowProps) -> Self {
        self.html_shadow = html_shadow;
        self
    }
}
