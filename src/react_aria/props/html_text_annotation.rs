use crate::html::HtmlTextAnnotationProps;

use super::AriaProps;

impl AriaProps {
    pub fn html_text_annotation(mut self, html_text_annotation: HtmlTextAnnotationProps) -> Self {
        self.html_text_annotation = html_text_annotation;
        self
    }
}
