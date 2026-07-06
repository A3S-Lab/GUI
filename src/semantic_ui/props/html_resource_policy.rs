use crate::html::HtmlResourcePolicyProps;

use super::SemanticProps;

impl SemanticProps {
    pub fn html_resource_policy(mut self, html_resource_policy: HtmlResourcePolicyProps) -> Self {
        self.html_resource_policy = html_resource_policy;
        self
    }
}
