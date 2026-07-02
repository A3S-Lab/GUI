use crate::html::HtmlResourcePolicyProps;

use super::NativeProps;

impl NativeProps {
    pub fn html_resource_policy(mut self, html_resource_policy: HtmlResourcePolicyProps) -> Self {
        self.html_resource_policy = html_resource_policy;
        self
    }
}
