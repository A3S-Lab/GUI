use crate::html::HtmlMicrodataProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_microdata(mut self, html_microdata: HtmlMicrodataProps) -> Self {
        self.html_microdata = html_microdata;
        self
    }
}
