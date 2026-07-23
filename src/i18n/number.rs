use std::fmt::{Debug, Formatter};

use fixed_decimal::{
    Decimal, FloatPrecision, SignDisplay as IcuSignDisplay, SignedRoundingMode,
    UnsignedRoundingMode,
};
use icu_decimal::options::{DecimalFormatterOptions, GroupingStrategy as IcuGroupingStrategy};
use icu_decimal::{DecimalFormatter, DecimalFormatterPreferences};
use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};

use super::parse_locale;

const MAX_FRACTION_DIGITS: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum NumberGrouping {
    #[default]
    Auto,
    Never,
    Always,
    Min2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum NumberSignDisplay {
    #[default]
    Auto,
    Never,
    Always,
    ExceptZero,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormatOptions {
    pub grouping: NumberGrouping,
    pub minimum_fraction_digits: u8,
    pub maximum_fraction_digits: u8,
    pub sign_display: NumberSignDisplay,
}

impl Default for NumberFormatOptions {
    fn default() -> Self {
        Self {
            grouping: NumberGrouping::Auto,
            minimum_fraction_digits: 0,
            maximum_fraction_digits: 3,
            sign_display: NumberSignDisplay::Auto,
        }
    }
}

impl NumberFormatOptions {
    pub fn grouping(mut self, grouping: NumberGrouping) -> Self {
        self.grouping = grouping;
        self
    }

    pub fn fraction_digits(mut self, minimum: u8, maximum: u8) -> Self {
        self.minimum_fraction_digits = minimum;
        self.maximum_fraction_digits = maximum;
        self
    }

    pub fn sign_display(mut self, sign_display: NumberSignDisplay) -> Self {
        self.sign_display = sign_display;
        self
    }
}

/// A reusable locale-aware decimal formatter.
///
/// Values use ECMA-402 half-expand rounding. The default zero-to-three
/// fraction digit range matches the decimal style of `Intl.NumberFormat`.
pub struct LocaleNumberFormatter {
    locale: String,
    options: NumberFormatOptions,
    inner: DecimalFormatter,
}

impl LocaleNumberFormatter {
    pub fn try_new(locale: &str, options: NumberFormatOptions) -> GuiResult<Self> {
        validate_options(options)?;
        let locale = parse_locale(locale)?;
        let canonical_locale = locale.to_string();
        let mut formatter_options = DecimalFormatterOptions::default();
        formatter_options.grouping_strategy = Some(match options.grouping {
            NumberGrouping::Auto => IcuGroupingStrategy::Auto,
            NumberGrouping::Never => IcuGroupingStrategy::Never,
            NumberGrouping::Always => IcuGroupingStrategy::Always,
            NumberGrouping::Min2 => IcuGroupingStrategy::Min2,
        });
        let preferences = DecimalFormatterPreferences::from(locale);
        let inner = DecimalFormatter::try_new(preferences, formatter_options).map_err(|error| {
            GuiError::internationalization(format!(
                "failed to create number formatter for locale {canonical_locale:?}: {error}"
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
        value.round_with_mode(
            -(i16::from(self.options.maximum_fraction_digits)),
            SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
        );
        value.trim_end();
        value.pad_end(-(i16::from(self.options.minimum_fraction_digits)));
        let value = value.with_sign_display(match self.options.sign_display {
            NumberSignDisplay::Auto => IcuSignDisplay::Auto,
            NumberSignDisplay::Never => IcuSignDisplay::Never,
            NumberSignDisplay::Always => IcuSignDisplay::Always,
            NumberSignDisplay::ExceptZero => IcuSignDisplay::ExceptZero,
            NumberSignDisplay::Negative => IcuSignDisplay::Negative,
        });
        self.inner.format_to_string(&value)
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

fn validate_options(options: NumberFormatOptions) -> GuiResult<()> {
    if options.minimum_fraction_digits > options.maximum_fraction_digits {
        return Err(GuiError::internationalization(format!(
            "minimum fraction digits ({}) exceed maximum fraction digits ({})",
            options.minimum_fraction_digits, options.maximum_fraction_digits
        )));
    }
    if options.maximum_fraction_digits > MAX_FRACTION_DIGITS {
        return Err(GuiError::internationalization(format!(
            "maximum fraction digits ({}) exceed the supported limit ({MAX_FRACTION_DIGITS})",
            options.maximum_fraction_digits
        )));
    }
    Ok(())
}
