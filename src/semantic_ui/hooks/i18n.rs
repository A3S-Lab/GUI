use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::i18n::{direction_for_locale, direction_name};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UseI18nProps {
    locale: Option<String>,
    direction: Option<String>,
}

impl UseI18nProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn locale(mut self, locale: Option<impl Into<String>>) -> Self {
        self.locale = locale.map(Into::into).filter(|locale| !locale.is_empty());
        self
    }

    pub fn direction(mut self, direction: Option<impl Into<String>>) -> Self {
        self.direction = direction
            .map(Into::into)
            .map(|direction| direction.to_ascii_lowercase())
            .filter(|direction| matches!(direction.as_str(), "ltr" | "rtl"));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseI18nResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub is_rtl: bool,
    pub i18n_props: I18nProps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    #[serde(rename = "data-locale", skip_serializing_if = "Option::is_none")]
    pub data_locale: Option<String>,
    #[serde(rename = "data-rtl")]
    pub data_rtl: bool,
}

pub fn use_i18n(props: UseI18nProps) -> UseI18nResult {
    let locale = props.locale;
    let direction = props.direction.or_else(|| {
        locale
            .as_deref()
            .map(direction_for_locale)
            .map(direction_name)
            .map(str::to_string)
    });
    let is_rtl = direction.as_deref() == Some("rtl");

    UseI18nResult {
        i18n_props: I18nProps {
            lang: locale.clone(),
            dir: direction.clone(),
            data_locale: locale.clone(),
            data_rtl: is_rtl,
        },
        locale,
        direction,
        is_rtl,
    }
}

pub fn use_i18n_value(props: UseI18nProps) -> GuiResult<JsonValue> {
    serde_json::to_value(use_i18n(props)).map_err(|error| {
        GuiError::invalid_tree(format!("semantic use_i18n hook did not serialize: {error}"))
    })
}
