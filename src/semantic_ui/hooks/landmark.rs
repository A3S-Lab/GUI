use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LandmarkKind {
    #[default]
    Region,
    Main,
    Navigation,
    Header,
    Footer,
    Article,
    Section,
    Aside,
    Search,
}

impl LandmarkKind {
    fn from_option(value: Option<impl Into<String>>) -> Self {
        match value
            .map(Into::into)
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref()
        {
            Some("main") => Self::Main,
            Some("navigation") | Some("nav") => Self::Navigation,
            Some("header") | Some("banner") => Self::Header,
            Some("footer") | Some("contentinfo") => Self::Footer,
            Some("article") => Self::Article,
            Some("section") => Self::Section,
            Some("aside") | Some("complementary") => Self::Aside,
            Some("search") => Self::Search,
            _ => Self::Region,
        }
    }

    fn kind_name(self) -> &'static str {
        match self {
            Self::Region => "region",
            Self::Main => "main",
            Self::Navigation => "navigation",
            Self::Header => "header",
            Self::Footer => "footer",
            Self::Article => "article",
            Self::Section => "section",
            Self::Aside => "aside",
            Self::Search => "search",
        }
    }

    fn role_name(self) -> &'static str {
        match self {
            Self::Region | Self::Section => "region",
            Self::Main => "main",
            Self::Navigation => "navigation",
            Self::Header => "banner",
            Self::Footer => "contentinfo",
            Self::Article => "article",
            Self::Aside => "complementary",
            Self::Search => "search",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseLandmarkProps {
    label: Option<String>,
    kind: LandmarkKind,
}

impl UseLandmarkProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: Option<impl Into<String>>) -> Self {
        self.label = label.map(Into::into).filter(|label| !label.is_empty());
        self
    }

    pub fn kind(mut self, kind: Option<impl Into<String>>) -> Self {
        self.kind = LandmarkKind::from_option(kind);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseLandmarkResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub landmark_kind: &'static str,
    pub landmark_props: LandmarkProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LandmarkProps {
    pub role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "data-landmark")]
    pub data_landmark: bool,
    #[serde(rename = "data-landmark-kind")]
    pub data_landmark_kind: &'static str,
}

pub fn use_landmark(props: UseLandmarkProps) -> UseLandmarkResult {
    let kind = props.kind;

    UseLandmarkResult {
        label: props.label.clone(),
        landmark_kind: kind.kind_name(),
        landmark_props: LandmarkProps {
            role: kind.role_name(),
            label: props.label,
            data_landmark: true,
            data_landmark_kind: kind.kind_name(),
        },
    }
}

pub fn use_landmark_value(props: UseLandmarkProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_landmark(props)).map_err(|error| {
        GuiError::invalid_tree(format!(
            "semantic use_landmark hook did not serialize: {error}"
        ))
    })
}
