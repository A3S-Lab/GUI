use crate::html::HtmlFormAssociationProps;

use super::SemanticProps;

impl SemanticProps {
    pub fn html_form_association(
        mut self,
        html_form_association: HtmlFormAssociationProps,
    ) -> Self {
        self.html_form_association = html_form_association;
        self
    }
}
