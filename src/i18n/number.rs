use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::{Arc, OnceLock, RwLock};

use fixed_decimal::{
    Decimal, FloatPrecision, Sign, SignDisplay as IcuSignDisplay, SignedRoundingMode,
    UnsignedRoundingMode,
};
use icu_decimal::options::{DecimalFormatterOptions, GroupingStrategy as IcuGroupingStrategy};
use icu_decimal::{DecimalFormatter, DecimalFormatterPreferences};
use icu_experimental::dimension::percent::formatter::{
    PercentFormatter, PercentFormatterPreferences,
};
use icu_experimental::dimension::percent::options::{Display, PercentFormatterOptions};
use icu_locale_core::Locale;
use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};

use super::parse_locale;

const MAX_FRACTION_DIGITS: u8 = 100;
const NUMBER_FORMATTER_CACHE_CAPACITY: usize = 32;

pub(crate) const NUMBER_STYLE_METADATA_KEY: &str = "data-number-style";
pub(crate) const NUMBER_GROUPING_METADATA_KEY: &str = "data-number-grouping";
pub(crate) const NUMBER_MIN_FRACTION_DIGITS_METADATA_KEY: &str =
    "data-number-minimum-fraction-digits";
pub(crate) const NUMBER_MAX_FRACTION_DIGITS_METADATA_KEY: &str =
    "data-number-maximum-fraction-digits";
pub(crate) const NUMBER_SIGN_DISPLAY_METADATA_KEY: &str = "data-number-sign-display";

static NUMBER_FORMATTER_CACHE: OnceLock<
    RwLock<BTreeMap<(String, NumberFormatOptions), Arc<LocaleNumberFormatter>>>,
> = OnceLock::new();

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "camelCase")]
pub enum NumberFormatStyle {
    #[default]
    Decimal,
    Percent,
}

impl NumberFormatStyle {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Decimal => "decimal",
            Self::Percent => "percent",
        }
    }
}

impl FromStr for NumberFormatStyle {
    type Err = GuiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "decimal" => Ok(Self::Decimal),
            "percent" => Ok(Self::Percent),
            _ => Err(GuiError::internationalization(format!(
                "unsupported number format style {value:?}"
            ))),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "camelCase")]
pub enum NumberGrouping {
    #[default]
    Auto,
    Never,
    Always,
    Min2,
}

impl FromStr for NumberGrouping {
    type Err = GuiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "auto" => Ok(Self::Auto),
            "never" | "false" => Ok(Self::Never),
            "always" | "true" => Ok(Self::Always),
            "min2" => Ok(Self::Min2),
            _ => Err(GuiError::internationalization(format!(
                "unsupported number grouping strategy {value:?}"
            ))),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "camelCase")]
pub enum NumberSignDisplay {
    #[default]
    Auto,
    Never,
    Always,
    ExceptZero,
    Negative,
}

impl FromStr for NumberSignDisplay {
    type Err = GuiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "auto" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            "always" => Ok(Self::Always),
            "exceptzero" | "except-zero" => Ok(Self::ExceptZero),
            "negative" => Ok(Self::Negative),
            _ => Err(GuiError::internationalization(format!(
                "unsupported number sign display {value:?}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormatOptions {
    #[serde(default)]
    pub style: NumberFormatStyle,
    #[serde(default)]
    pub grouping: NumberGrouping,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_fraction_digits: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_fraction_digits: Option<u8>,
    #[serde(default)]
    pub sign_display: NumberSignDisplay,
}

impl Default for NumberFormatOptions {
    fn default() -> Self {
        Self {
            style: NumberFormatStyle::Decimal,
            grouping: NumberGrouping::Auto,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
            sign_display: NumberSignDisplay::Auto,
        }
    }
}

impl NumberFormatOptions {
    pub fn style(mut self, style: NumberFormatStyle) -> Self {
        self.style = style;
        self
    }

    pub fn grouping(mut self, grouping: NumberGrouping) -> Self {
        self.grouping = grouping;
        self
    }

    pub fn fraction_digits(mut self, minimum: u8, maximum: u8) -> Self {
        self.minimum_fraction_digits = Some(minimum);
        self.maximum_fraction_digits = Some(maximum);
        self
    }

    pub fn minimum_fraction_digits(mut self, minimum: u8) -> Self {
        self.minimum_fraction_digits = Some(minimum);
        self
    }

    pub fn maximum_fraction_digits(mut self, maximum: u8) -> Self {
        self.maximum_fraction_digits = Some(maximum);
        self
    }

    pub fn sign_display(mut self, sign_display: NumberSignDisplay) -> Self {
        self.sign_display = sign_display;
        self
    }

    pub fn resolved_minimum_fraction_digits(self) -> u8 {
        self.minimum_fraction_digits.unwrap_or(0)
    }

    pub fn resolved_maximum_fraction_digits(self) -> u8 {
        self.maximum_fraction_digits.unwrap_or_else(|| {
            let style_default = match self.style {
                NumberFormatStyle::Decimal => 3,
                NumberFormatStyle::Percent => 0,
            };
            style_default.max(self.resolved_minimum_fraction_digits())
        })
    }

    pub(crate) fn from_metadata(metadata: &BTreeMap<String, String>) -> Self {
        let mut options = Self::default();
        if let Some(style) = parse_metadata(metadata, NUMBER_STYLE_METADATA_KEY) {
            options.style = style;
        }
        if let Some(grouping) = parse_metadata(metadata, NUMBER_GROUPING_METADATA_KEY) {
            options.grouping = grouping;
        }
        options.minimum_fraction_digits =
            parse_metadata(metadata, NUMBER_MIN_FRACTION_DIGITS_METADATA_KEY);
        options.maximum_fraction_digits =
            parse_metadata(metadata, NUMBER_MAX_FRACTION_DIGITS_METADATA_KEY);
        if let Some(sign_display) = parse_metadata(metadata, NUMBER_SIGN_DISPLAY_METADATA_KEY) {
            options.sign_display = sign_display;
        }
        options
    }
}

/// A reusable locale-aware decimal or percentage formatter.
///
/// Values use ECMA-402 half-expand rounding. The default zero-to-three
/// fraction digit range for decimal values and zero fraction digits for
/// percentages match `Intl.NumberFormat`.
pub struct LocaleNumberFormatter {
    locale: String,
    options: NumberFormatOptions,
    inner: NumberFormatterKind,
}

enum NumberFormatterKind {
    Decimal(DecimalFormatter),
    Percent {
        standard: PercentFormatter<DecimalFormatter>,
        explicit_sign: PercentFormatter<DecimalFormatter>,
    },
}

impl LocaleNumberFormatter {
    pub fn try_new(locale: &str, options: NumberFormatOptions) -> GuiResult<Self> {
        validate_options(options)?;
        let locale = parse_locale(locale)?;
        let canonical_locale = locale.to_string();
        let inner = match options.style {
            NumberFormatStyle::Decimal => {
                NumberFormatterKind::Decimal(decimal_formatter(&locale, options)?)
            }
            NumberFormatStyle::Percent => {
                let preferences = PercentFormatterPreferences::from(&locale);
                let standard = PercentFormatter::try_new_with_decimal_formatter(
                    preferences,
                    decimal_formatter(&locale, options)?,
                    PercentFormatterOptions::from(Display::Standard),
                )
                .map_err(|error| {
                    GuiError::internationalization(format!(
                        "failed to create percent formatter for locale {canonical_locale:?}: {error}"
                    ))
                })?;
                let explicit_sign = PercentFormatter::try_new_with_decimal_formatter(
                    preferences,
                    decimal_formatter(&locale, options)?,
                    PercentFormatterOptions::from(Display::ExplicitSign),
                )
                .map_err(|error| {
                    GuiError::internationalization(format!(
                        "failed to create signed percent formatter for locale {canonical_locale:?}: {error}"
                    ))
                })?;
                NumberFormatterKind::Percent {
                    standard,
                    explicit_sign,
                }
            }
        };

        Ok(Self {
            locale: canonical_locale,
            options,
            inner,
        })
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn options(&self) -> NumberFormatOptions {
        self.options
    }

    pub fn format_decimal(&self, value: &str) -> GuiResult<String> {
        let decimal = value.parse::<Decimal>().map_err(|error| {
            GuiError::internationalization(format!("invalid decimal value {value:?}: {error}"))
        })?;
        Ok(self.format_value(decimal))
    }

    pub fn format_i64(&self, value: i64) -> String {
        self.format_value(Decimal::from(value))
    }

    pub fn format_u64(&self, value: u64) -> String {
        self.format_value(Decimal::from(value))
    }

    pub fn format_f64(&self, value: f64) -> GuiResult<String> {
        let decimal = Decimal::try_from_f64(value, FloatPrecision::RoundTrip).map_err(|error| {
            GuiError::internationalization(format!(
                "number formatter requires a finite f64 value: {error}"
            ))
        })?;
        Ok(self.format_value(decimal))
    }

    fn format_value(&self, mut value: Decimal) -> String {
        if self.options.style == NumberFormatStyle::Percent {
            value.multiply_pow10(2);
            value.trim_start();
        }
        value.round_with_mode(
            -(i16::from(self.options.resolved_maximum_fraction_digits())),
            SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
        );
        value.trim_end();
        value.pad_end(-(i16::from(self.options.resolved_minimum_fraction_digits())));
        let value = value.with_sign_display(match self.options.sign_display {
            NumberSignDisplay::Auto => IcuSignDisplay::Auto,
            NumberSignDisplay::Never => IcuSignDisplay::Never,
            NumberSignDisplay::Always => IcuSignDisplay::Always,
            NumberSignDisplay::ExceptZero => IcuSignDisplay::ExceptZero,
            NumberSignDisplay::Negative => IcuSignDisplay::Negative,
        });
        match &self.inner {
            NumberFormatterKind::Decimal(formatter) => formatter.format_to_string(&value),
            NumberFormatterKind::Percent {
                standard,
                explicit_sign,
            } => {
                if value.sign() == Sign::Positive {
                    explicit_sign
                        .format(&value.with_sign(Sign::None))
                        .to_string()
                } else {
                    standard.format(&value).to_string()
                }
            }
        }
    }
}

impl Debug for LocaleNumberFormatter {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocaleNumberFormatter")
            .field("locale", &self.locale)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

pub(crate) fn cached_number_formatter(
    locale: &str,
    options: NumberFormatOptions,
) -> GuiResult<Arc<LocaleNumberFormatter>> {
    let canonical_locale = parse_locale(locale)?.to_string();
    let key = (canonical_locale.clone(), options);
    let cache = NUMBER_FORMATTER_CACHE.get_or_init(|| RwLock::new(BTreeMap::new()));
    let cached = match cache.read() {
        Ok(cache) => cache.get(&key).cloned(),
        Err(poisoned) => poisoned.into_inner().get(&key).cloned(),
    };
    if let Some(formatter) = cached {
        return Ok(formatter);
    }

    let formatter = Arc::new(LocaleNumberFormatter::try_new(&canonical_locale, options)?);
    let mut cache = match cache.write() {
        Ok(cache) => cache,
        Err(poisoned) => poisoned.into_inner(),
    };
    if cache.len() >= NUMBER_FORMATTER_CACHE_CAPACITY && !cache.contains_key(&key) {
        cache.clear();
    }
    Ok(cache
        .entry(key)
        .or_insert_with(|| Arc::clone(&formatter))
        .clone())
}

fn decimal_formatter(locale: &Locale, options: NumberFormatOptions) -> GuiResult<DecimalFormatter> {
    let mut formatter_options = DecimalFormatterOptions::default();
    formatter_options.grouping_strategy = Some(match options.grouping {
        NumberGrouping::Auto => IcuGroupingStrategy::Auto,
        NumberGrouping::Never => IcuGroupingStrategy::Never,
        NumberGrouping::Always => IcuGroupingStrategy::Always,
        NumberGrouping::Min2 => IcuGroupingStrategy::Min2,
    });
    let preferences = DecimalFormatterPreferences::from(locale);
    DecimalFormatter::try_new(preferences, formatter_options).map_err(|error| {
        GuiError::internationalization(format!(
            "failed to create number formatter for locale {:?}: {error}",
            locale.to_string()
        ))
    })
}

fn validate_options(options: NumberFormatOptions) -> GuiResult<()> {
    let minimum_fraction_digits = options.resolved_minimum_fraction_digits();
    let maximum_fraction_digits = options.resolved_maximum_fraction_digits();
    if minimum_fraction_digits > maximum_fraction_digits {
        return Err(GuiError::internationalization(format!(
            "minimum fraction digits ({minimum_fraction_digits}) exceed maximum fraction digits ({maximum_fraction_digits})"
        )));
    }
    if maximum_fraction_digits > MAX_FRACTION_DIGITS {
        return Err(GuiError::internationalization(format!(
            "maximum fraction digits ({maximum_fraction_digits}) exceed the supported limit ({MAX_FRACTION_DIGITS})"
        )));
    }
    Ok(())
}

fn parse_metadata<T: FromStr>(metadata: &BTreeMap<String, String>, name: &str) -> Option<T> {
    metadata.get(name)?.parse().ok()
}
