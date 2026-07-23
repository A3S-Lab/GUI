use std::collections::BTreeMap;

use icu_locale_core::Locale;
use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;
use crate::native::{NativeElement, NativeProps};
use crate::renderer::MountedNodeSnapshot;
use crate::style::TextDirection;

mod collator;
mod datetime;
mod number;

pub use collator::{
    CollationCaseFirst, CollationOptions, CollationSensitivity, CollationUsage, LocaleCollator,
};
pub use datetime::{
    DateFormatKind, DateFormatOptions, DateFormatStyle, DateTimeValue, LocaleDateFormatter,
};
pub use number::{LocaleNumberFormatter, NumberFormatOptions, NumberGrouping, NumberSignDisplay};

pub const DEFAULT_FORMATTING_LOCALE: &str = "und";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocaleContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub direction: TextDirection,
}

impl Default for LocaleContext {
    fn default() -> Self {
        Self {
            locale: None,
            direction: TextDirection::Ltr,
        }
    }
}

/// Resolves inherited locale and writing direction for a mounted native tree.
#[derive(Debug, Clone, Default)]
pub struct I18nManager {
    default_context: LocaleContext,
    nodes: BTreeMap<HostNodeId, LocaleContext>,
}

impl I18nManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_context(&self) -> &LocaleContext {
        &self.default_context
    }

    pub fn set_default_locale(&mut self, locale: Option<impl Into<String>>) {
        self.default_context.locale = locale
            .map(Into::into)
            .map(|locale| locale.trim().to_string())
            .filter(|locale| !locale.is_empty());
        if let Some(locale) = self.default_context.locale.as_deref() {
            self.default_context.direction = direction_for_locale(locale);
        }
    }

    pub fn set_default_direction(&mut self, direction: TextDirection) {
        self.default_context.direction = direction;
    }

    pub fn context(&self, node: HostNodeId) -> Option<&LocaleContext> {
        self.nodes.get(&node)
    }

    pub fn locale(&self, node: HostNodeId) -> Option<&str> {
        self.context(node)
            .and_then(|context| context.locale.as_deref())
    }

    pub fn direction(&self, node: HostNodeId) -> TextDirection {
        self.context(node)
            .map(|context| context.direction)
            .unwrap_or(self.default_context.direction)
    }

    pub fn formatting_locale(&self, node: HostNodeId) -> &str {
        self.locale(node)
            .or(self.default_context.locale.as_deref())
            .unwrap_or(DEFAULT_FORMATTING_LOCALE)
    }

    pub fn collator(
        &self,
        node: HostNodeId,
        options: CollationOptions,
    ) -> GuiResult<LocaleCollator> {
        LocaleCollator::try_new(self.formatting_locale(node), options)
    }

    pub fn number_formatter(
        &self,
        node: HostNodeId,
        options: NumberFormatOptions,
    ) -> GuiResult<LocaleNumberFormatter> {
        LocaleNumberFormatter::try_new(self.formatting_locale(node), options)
    }

    pub fn date_formatter(
        &self,
        node: HostNodeId,
        options: DateFormatOptions,
    ) -> GuiResult<LocaleDateFormatter> {
        LocaleDateFormatter::try_new(self.formatting_locale(node), options)
    }

    pub fn sync(&mut self, snapshot: &[MountedNodeSnapshot]) {
        let mut contexts = BTreeMap::new();
        for node in snapshot {
            let parent = node
                .parent
                .and_then(|parent| contexts.get(&parent))
                .unwrap_or(&self.default_context);
            contexts.insert(node.node, resolve_context(&node.props, parent));
        }
        self.nodes = contexts;
    }

    pub(crate) fn project_native_tree(&self, root: &mut NativeElement) {
        let active = self.default_context.locale.is_some();
        project_node(root, &self.default_context, active);
    }
}

pub fn direction_for_locale(locale: &str) -> TextDirection {
    let subtags = locale
        .split(['-', '_'])
        .filter(|subtag| !subtag.is_empty())
        .collect::<Vec<_>>();
    if subtags.iter().any(|subtag| {
        matches!(
            subtag.to_ascii_lowercase().as_str(),
            "adlm" | "arab" | "hebr" | "mand" | "mend" | "nkoo" | "rohg" | "samr" | "syrc" | "thaa"
        )
    }) {
        return TextDirection::Rtl;
    }

    let language = subtags
        .first()
        .map(|language| language.to_ascii_lowercase())
        .unwrap_or_default();
    if matches!(
        language.as_str(),
        "ar" | "arc"
            | "ckb"
            | "dv"
            | "fa"
            | "he"
            | "iw"
            | "ks"
            | "ku"
            | "nqo"
            | "ps"
            | "sd"
            | "syr"
            | "ug"
            | "ur"
            | "yi"
    ) {
        TextDirection::Rtl
    } else {
        TextDirection::Ltr
    }
}

pub fn direction_name(direction: TextDirection) -> &'static str {
    match direction {
        TextDirection::Ltr => "ltr",
        TextDirection::Rtl => "rtl",
    }
}

fn parse_locale(locale: &str) -> GuiResult<Locale> {
    locale.trim().parse::<Locale>().map_err(|error| {
        GuiError::internationalization(format!("invalid BCP 47 locale {locale:?}: {error}"))
    })
}

fn resolve_context(props: &NativeProps, parent: &LocaleContext) -> LocaleContext {
    let locale = explicit_locale(props).or_else(|| parent.locale.clone());
    let direction = explicit_direction(props).unwrap_or_else(|| {
        if explicit_locale(props).is_some() {
            locale
                .as_deref()
                .map(direction_for_locale)
                .unwrap_or(parent.direction)
        } else {
            parent.direction
        }
    });
    LocaleContext { locale, direction }
}

fn project_node(node: &mut NativeElement, parent: &LocaleContext, parent_active: bool) {
    let has_locale = explicit_locale(&node.props).is_some();
    let has_direction = explicit_direction(&node.props).is_some();
    let active = parent_active || has_locale || has_direction;
    let context = resolve_context(&node.props, parent);

    if active {
        node.props.lang = context.locale.clone();
        node.props.dir = Some(direction_name(context.direction).to_string());
        if node.props.web.attributes.contains_key("lang") {
            if let Some(locale) = &context.locale {
                node.props
                    .web
                    .attributes
                    .insert("lang".to_string(), locale.clone());
            }
        }
        if node.props.web.attributes.contains_key("dir") {
            node.props.web.attributes.insert(
                "dir".to_string(),
                direction_name(context.direction).to_string(),
            );
        }
        if node.props.metadata.contains_key("lang") {
            if let Some(locale) = &context.locale {
                node.props
                    .metadata
                    .insert("lang".to_string(), locale.clone());
            }
        }
        if node.props.metadata.contains_key("dir") {
            node.props.metadata.insert(
                "dir".to_string(),
                direction_name(context.direction).to_string(),
            );
        }
    }

    for child in &mut node.children {
        project_node(child, &context, active);
    }
}

fn explicit_locale(props: &NativeProps) -> Option<String> {
    props
        .lang
        .as_deref()
        .or_else(|| props.web.attributes.get("lang").map(String::as_str))
        .map(str::trim)
        .filter(|locale| !locale.is_empty())
        .map(str::to_string)
}

fn explicit_direction(props: &NativeProps) -> Option<TextDirection> {
    props
        .dir
        .as_deref()
        .or_else(|| props.web.attributes.get("dir").map(String::as_str))
        .and_then(
            |direction| match direction.trim().to_ascii_lowercase().as_str() {
                "ltr" => Some(TextDirection::Ltr),
                "rtl" => Some(TextDirection::Rtl),
                _ => None,
            },
        )
}

#[cfg(test)]
mod tests;
