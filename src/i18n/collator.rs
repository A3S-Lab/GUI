use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use icu_collator::options::{CaseLevel, CollatorOptions as IcuCollatorOptions, Strength};
use icu_collator::preferences::{
    CollationCaseFirst as IcuCaseFirst, CollationNumericOrdering, CollationType,
};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};

use super::parse_locale;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CollationUsage {
    #[default]
    Sort,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CollationSensitivity {
    Base,
    Accent,
    Case,
    #[default]
    Variant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CollationCaseFirst {
    #[default]
    Auto,
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollationOptions {
    pub usage: CollationUsage,
    pub sensitivity: CollationSensitivity,
    pub case_first: CollationCaseFirst,
    pub numeric: bool,
}

impl Default for CollationOptions {
    fn default() -> Self {
        Self {
            usage: CollationUsage::Sort,
            sensitivity: CollationSensitivity::Variant,
            case_first: CollationCaseFirst::Auto,
            numeric: false,
        }
    }
}

impl CollationOptions {
    pub fn usage(mut self, usage: CollationUsage) -> Self {
        self.usage = usage;
        self
    }

    pub fn sensitivity(mut self, sensitivity: CollationSensitivity) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn case_first(mut self, case_first: CollationCaseFirst) -> Self {
        self.case_first = case_first;
        self
    }

    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }
}

/// A reusable locale-aware string comparator equivalent to React Aria's
/// `useCollator` result.
pub struct LocaleCollator {
    locale: String,
    options: CollationOptions,
    inner: CollatorBorrowed<'static>,
}

impl LocaleCollator {
    pub fn try_new(locale: &str, options: CollationOptions) -> GuiResult<Self> {
        let locale = parse_locale(locale)?;
        let canonical_locale = locale.to_string();
        let mut preferences = CollatorPreferences::from(locale);
        preferences.collation_type = match options.usage {
            CollationUsage::Sort => None,
            CollationUsage::Search => Some(CollationType::Search),
        };
        preferences.case_first = match options.case_first {
            CollationCaseFirst::Auto => None,
            CollationCaseFirst::Upper => Some(IcuCaseFirst::Upper),
            CollationCaseFirst::Lower => Some(IcuCaseFirst::Lower),
        };
        preferences.numeric_ordering = Some(if options.numeric {
            CollationNumericOrdering::True
        } else {
            CollationNumericOrdering::False
        });

        let mut collator_options = IcuCollatorOptions::default();
        match options.sensitivity {
            CollationSensitivity::Base => {
                collator_options.strength = Some(Strength::Primary);
            }
            CollationSensitivity::Accent => {
                collator_options.strength = Some(Strength::Secondary);
            }
            CollationSensitivity::Case => {
                collator_options.strength = Some(Strength::Primary);
                collator_options.case_level = Some(CaseLevel::On);
            }
            CollationSensitivity::Variant => {
                collator_options.strength = Some(Strength::Tertiary);
            }
        }

        let inner = Collator::try_new(preferences, collator_options).map_err(|error| {
            GuiError::internationalization(format!(
                "failed to create collator for locale {canonical_locale:?}: {error}"
            ))
        })?;
        Ok(Self {
            locale: canonical_locale,
            options,
            inner,
        })
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn options(&self) -> CollationOptions {
        self.options
    }

    pub fn compare(&self, left: &str, right: &str) -> Ordering {
        self.inner.compare(left, right)
    }

    pub fn is_equal(&self, left: &str, right: &str) -> bool {
        self.compare(left, right) == Ordering::Equal
    }

    /// Returns whether `value` starts with a locale-equivalent `query`.
    pub fn starts_with(&self, value: &str, query: &str) -> bool {
        substring_boundaries(value).any(|end| self.is_equal(&value[..end], query))
    }

    /// Returns whether `value` ends with a locale-equivalent `query`.
    pub fn ends_with(&self, value: &str, query: &str) -> bool {
        substring_boundaries(value).any(|start| self.is_equal(&value[start..], query))
    }

    /// Returns whether `value` contains a locale-equivalent `query`.
    pub fn contains(&self, value: &str, query: &str) -> bool {
        let boundaries = substring_boundaries(value).collect::<Vec<_>>();
        boundaries.iter().enumerate().any(|(start_index, start)| {
            boundaries[start_index..]
                .iter()
                .any(|end| self.is_equal(&value[*start..*end], query))
        })
    }
}

impl Debug for LocaleCollator {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocaleCollator")
            .field("locale", &self.locale)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

fn substring_boundaries(value: &str) -> impl Iterator<Item = usize> + '_ {
    std::iter::once(0)
        .chain(value.char_indices().skip(1).map(|(index, _)| index))
        .chain(std::iter::once(value.len()))
}
