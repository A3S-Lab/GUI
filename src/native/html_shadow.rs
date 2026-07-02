use crate::html::HtmlShadowProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_shadow(mut self, html_shadow: HtmlShadowProps) -> Self {
        self.html_shadow = html_shadow;
        self
    }
}
