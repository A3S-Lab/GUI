use super::{SemanticComponent, SemanticProps};

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticElement {
    pub key: String,
    pub component: SemanticComponent,
    pub props: SemanticProps,
    pub children: Vec<SemanticElement>,
}

impl SemanticElement {
    pub fn new(key: impl Into<String>, component: SemanticComponent) -> Self {
        Self {
            key: key.into(),
            component,
            props: SemanticProps::default(),
            children: Vec::new(),
        }
    }

    pub fn text(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(key, SemanticComponent::Text).with_props(SemanticProps::new().text_value(text))
    }

    pub fn with_props(mut self, props: SemanticProps) -> Self {
        self.props = props;
        self
    }

    pub fn child(mut self, child: SemanticElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = SemanticElement>) -> Self {
        self.children.extend(children);
        self
    }
}
