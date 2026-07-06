use std::error::Error;
use std::fmt::{Display, Formatter};

pub type GuiResult<T> = Result<T, GuiError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuiError {
    UnsupportedSemanticComponent {
        component: String,
    },
    UnsupportedDomAttribute {
        component: String,
        attribute: String,
    },
    MissingRequiredProp {
        component: &'static str,
        prop: &'static str,
    },
    InvalidTree {
        message: String,
    },
    Host {
        message: String,
    },
}

impl Display for GuiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GuiError::UnsupportedSemanticComponent { component } => {
                write!(f, "unsupported semantic UI component: {component}")
            }
            GuiError::UnsupportedDomAttribute {
                component,
                attribute,
            } => write!(
                f,
                "unsupported DOM-only attribute {attribute:?} on semantic UI component {component}"
            ),
            GuiError::MissingRequiredProp { component, prop } => {
                write!(f, "missing required prop {prop:?} on component {component}")
            }
            GuiError::InvalidTree { message } => write!(f, "invalid native UI tree: {message}"),
            GuiError::Host { message } => write!(f, "native host error: {message}"),
        }
    }
}

impl Error for GuiError {}

impl GuiError {
    pub fn host(message: impl Into<String>) -> Self {
        GuiError::Host {
            message: message.into(),
        }
    }

    pub fn invalid_tree(message: impl Into<String>) -> Self {
        GuiError::InvalidTree {
            message: message.into(),
        }
    }
}
