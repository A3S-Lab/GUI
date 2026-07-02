use crate::html::HtmlDialogProps;

use super::AriaProps;

impl AriaProps {
    pub fn html_dialog(mut self, html_dialog: HtmlDialogProps) -> Self {
        self.html_dialog = html_dialog;
        self
    }
}
