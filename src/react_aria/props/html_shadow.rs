use crate::html::HtmlShadowProps;

use super::AriaProps;

impl AriaProps {
    pub fn html_shadow(mut self, html_shadow: HtmlShadowProps) -> Self {
        self.html_shadow = html_shadow;
        self
    }
}
