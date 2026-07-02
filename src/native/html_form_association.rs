use crate::html::HtmlFormAssociationProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_form_association(
        mut self,
        html_form_association: HtmlFormAssociationProps,
    ) -> Self {
        self.html_form_association = html_form_association;
        self
    }
}
