use super::{AriaComponent, AriaProps};

#[derive(Debug, Clone, PartialEq)]
pub struct AriaElement {
    pub key: String,
    pub component: AriaComponent,
    pub props: AriaProps,
    pub children: Vec<AriaElement>,
}

impl AriaElement {
    pub fn new(key: impl Into<String>, component: AriaComponent) -> Self {
        Self {
            key: key.into(),
            component,
            props: AriaProps::default(),
            children: Vec::new(),
        }
    }

    pub fn text(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(key, AriaComponent::Text).with_props(AriaProps::new().text_value(text))
    }

    pub fn with_props(mut self, props: AriaProps) -> Self {
        self.props = props;
        self
    }

    pub fn child(mut self, child: AriaElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = AriaElement>) -> Self {
        self.children.extend(children);
        self
    }
}
