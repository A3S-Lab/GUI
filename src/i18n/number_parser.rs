use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter, Write};
use std::sync::{Arc, OnceLock, RwLock};

use fixed_decimal::{Decimal, SignDisplay};
use icu_decimal::options::{DecimalFormatterOptions, GroupingStrategy};
use icu_decimal::{parts, DecimalFormatter, DecimalFormatterPreferences, FormattedDecimal};
use icu_locale_core::Locale;
use icu_properties::props::GeneralCategory;
use icu_properties::CodePointMapData;
use writeable::{Part, PartsWrite, Writeable};

use crate::error::{GuiError, GuiResult};

use super::{parse_locale, NumberFormatOptions, NumberFormatStyle};

const AUTO_NUMBERING_SYSTEMS: [&str; 6] = ["latn", "arab", "hanidec", "deva", "beng", "fullwide"];
const NUMBER_PARSER_CACHE_CAPACITY: usize = 32;

static NUMBER_PARSER_CACHE: OnceLock<RwLock<BTreeMap<String, Arc<LocaleNumberParser>>>> =
    OnceLock::new();

/// A reusable locale-aware decimal parser.
///
/// The parser accepts the locale's decimal, grouping, and sign symbols and
/// automatically detects the same six positional numbering systems as React
/// Aria's `NumberParser`: Latin, Arabic, Han decimal, Devanagari, Bengali, and
/// full-width digits. Locale identifiers with an explicit `-u-nu-` extension
/// restrict input to that numbering system.
#[derive(Clone)]
pub struct LocaleNumberParser {
    locale: String,
    default_profile: NumberProfile,
    alternate_profiles: Vec<NumberProfile>,
}

impl LocaleNumberParser {
    pub fn try_new(locale: &str) -> GuiResult<Self> {
        let locale = parse_locale(locale)?;
        let canonical_locale = locale.to_string();
        let explicit_numbering_system = explicit_numbering_system(&canonical_locale);
        let default_profile = NumberProfile::try_new(&locale, explicit_numbering_system)?;
        let mut alternate_profiles: Vec<NumberProfile> = Vec::new();

        if explicit_numbering_system.is_none() {
            for numbering_system in AUTO_NUMBERING_SYSTEMS {
                let profile = NumberProfile::try_new(&locale, Some(numbering_system))?;
                if profile.digits != default_profile.digits
                    && alternate_profiles
                        .iter()
                        .all(|existing| existing.digits != profile.digits)
                {
                    alternate_profiles.push(profile);
                }
            }
        }

        Ok(Self {
            locale: canonical_locale,
            default_profile,
            alternate_profiles,
        })
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Parses a localized decimal into a finite `f64`.
    pub fn parse(&self, value: &str) -> GuiResult<f64> {
        self.parse_with_options(value, NumberFormatOptions::default())
    }

    /// Parses a localized number using the supplied display options.
    ///
    /// Percentage input is divided by 100 so the returned value remains in
    /// model space, matching `Intl.NumberFormat` and React Aria `NumberField`.
    pub fn parse_with_options(&self, value: &str, options: NumberFormatOptions) -> GuiResult<f64> {
        let normalized = normalize_style_input(value, options.style).ok_or_else(|| {
            GuiError::internationalization(format!(
                "invalid {:?} value {value:?} for locale {:?}",
                options.style, self.locale
            ))
        })?;
        let profile = self
            .profile_for_partial_number(&normalized, None, None)
            .unwrap_or(&self.default_profile);
        let parsed = profile.parse(&normalized).ok_or_else(|| {
            GuiError::internationalization(format!(
                "invalid {:?} value {value:?} for locale {:?}",
                options.style, self.locale
            ))
        })?;
        Ok(match options.style {
            NumberFormatStyle::Decimal => parsed,
            NumberFormatStyle::Percent => parsed / 100.0,
        })
    }

    /// Returns whether `value` can become a localized decimal while editing.
    ///
    /// Empty input, a permitted leading sign, a decimal separator, and
    /// grouping separators are accepted as partial input. `min` and `max`
    /// constrain whether negative and positive signs are permitted.
    pub fn is_valid_partial_number(&self, value: &str, min: Option<f64>, max: Option<f64>) -> bool {
        self.is_valid_partial_number_with_options(value, min, max, NumberFormatOptions::default())
    }

    /// Returns whether `value` can become a localized number for `options`
    /// while editing.
    pub fn is_valid_partial_number_with_options(
        &self,
        value: &str,
        min: Option<f64>,
        max: Option<f64>,
        options: NumberFormatOptions,
    ) -> bool {
        normalize_style_input(value, options.style).is_some_and(|normalized| {
            self.profile_for_partial_number(&normalized, min, max)
                .is_some()
        })
    }

    /// Returns the detected numbering system, or the locale default when the
    /// input does not identify a supported system.
    pub fn numbering_system(&self, value: &str) -> &str {
        self.numbering_system_with_options(value, NumberFormatOptions::default())
    }

    /// Returns the detected numbering system after removing style affixes.
    pub fn numbering_system_with_options(&self, value: &str, options: NumberFormatOptions) -> &str {
        let normalized = normalize_style_input(value, options.style);
        normalized
            .as_deref()
            .and_then(|value| self.profile_for_partial_number(value, None, None))
            .unwrap_or(&self.default_profile)
            .numbering_system
            .as_str()
    }

    fn profile_for_partial_number(
        &self,
        value: &str,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Option<&NumberProfile> {
        std::iter::once(&self.default_profile)
            .chain(&self.alternate_profiles)
            .find(|profile| profile.is_valid_partial_number(value, min, max))
    }
}

fn normalize_style_input(value: &str, style: NumberFormatStyle) -> Option<String> {
    if style == NumberFormatStyle::Decimal {
        return Some(value.to_string());
    }

    let mut percent_signs = 0usize;
    let normalized = value
        .chars()
        .filter(|character| {
            if matches!(*character, '%' | '\u{066a}' | '\u{fe6a}' | '\u{ff05}') {
                percent_signs += 1;
                false
            } else {
                true
            }
        })
        .collect::<String>();
    (percent_signs <= 1).then_some(normalized)
}

impl Debug for LocaleNumberParser {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocaleNumberParser")
            .field("locale", &self.locale)
            .field(
                "numbering_systems",
                &std::iter::once(&self.default_profile)
                    .chain(&self.alternate_profiles)
                    .map(|profile| profile.numbering_system.as_str())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

pub(crate) fn cached_number_parser(locale: &str) -> GuiResult<Arc<LocaleNumberParser>> {
    let canonical_locale = parse_locale(locale)?.to_string();
    let cache = NUMBER_PARSER_CACHE.get_or_init(|| RwLock::new(BTreeMap::new()));
    let cached = match cache.read() {
        Ok(cache) => cache.get(&canonical_locale).cloned(),
        Err(poisoned) => poisoned.into_inner().get(&canonical_locale).cloned(),
    };
    if let Some(parser) = cached {
        return Ok(parser);
    }

    let parser = Arc::new(LocaleNumberParser::try_new(&canonical_locale)?);
    let mut cache = match cache.write() {
        Ok(cache) => cache,
        Err(poisoned) => poisoned.into_inner(),
    };
    if cache.len() >= NUMBER_PARSER_CACHE_CAPACITY && !cache.contains_key(&canonical_locale) {
        cache.clear();
    }
    Ok(cache
        .entry(canonical_locale)
        .or_insert_with(|| Arc::clone(&parser))
        .clone())
}

#[derive(Debug, Clone)]
struct NumberProfile {
    numbering_system: String,
    digits: [char; 10],
    minus_sign: String,
    plus_sign: String,
    decimal_separator: String,
    grouping_separator: String,
}

impl NumberProfile {
    fn try_new(locale: &Locale, requested_numbering_system: Option<&str>) -> GuiResult<Self> {
        let mut preferences = DecimalFormatterPreferences::from(locale);
        if let Some(numbering_system) = requested_numbering_system {
            preferences.numbering_system = numbering_system_preference(numbering_system)?;
        }

        let mut options = DecimalFormatterOptions::default();
        options.grouping_strategy = Some(GroupingStrategy::Always);
        let formatter = DecimalFormatter::try_new(preferences, options).map_err(|error| {
            GuiError::internationalization(format!(
                "failed to create number parser symbols for locale {:?}: {error}",
                locale.to_string()
            ))
        })?;

        let mut digits = formatted_digits(&formatter)?;
        let mut numbering_system = numbering_system_for_digits(&digits)
            .map(str::to_string)
            .unwrap_or_else(|| requested_numbering_system.unwrap_or("latn").to_string());
        if let Some(requested) = requested_numbering_system {
            if let Some(requested_digits) = digits_for_numbering_system(requested) {
                if numbering_system != requested {
                    digits = requested_digits;
                    numbering_system = requested.to_string();
                }
            }
        }

        let sample = decimal("1234567890.5")?;
        let sample_parts = collect_parts(&formatter.format(&sample))?;
        let negative_parts = collect_parts(&formatter.format(&decimal("-1")?))?;
        let positive = Decimal::from(1).with_sign_display(SignDisplay::Always);
        let positive_parts = collect_parts(&formatter.format(&positive))?;

        Ok(Self {
            numbering_system,
            digits,
            minus_sign: negative_parts.text_for_part(parts::MINUS_SIGN),
            plus_sign: positive_parts.text_for_part(parts::PLUS_SIGN),
            decimal_separator: sample_parts.text_for_part(parts::DECIMAL),
            grouping_separator: sample_parts.text_for_part(parts::GROUP),
        })
    }

    fn parse(&self, value: &str) -> Option<f64> {
        if !self.is_valid_partial_number(value, None, None) {
            return None;
        }

        let mut value = self.sanitize(value);
        normalize_leading_sign(&mut value, &self.minus_sign, "-", true);
        normalize_leading_sign(&mut value, &self.plus_sign, "+", false);
        if !self.grouping_separator.is_empty() {
            value = value.replace(&self.grouping_separator, "");
        }
        replace_first(&mut value, &self.decimal_separator, ".");

        let mut normalized = String::with_capacity(value.len());
        for character in value.chars() {
            if let Some(index) = self.digits.iter().position(|digit| *digit == character) {
                normalized.push(char::from(b'0' + index as u8));
            } else {
                normalized.push(character);
            }
        }

        normalized
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
    }

    fn is_valid_partial_number(&self, value: &str, min: Option<f64>, max: Option<f64>) -> bool {
        let mut value = self.sanitize(value);
        let min = min
            .filter(|value| value.is_finite())
            .unwrap_or(f64::NEG_INFINITY);
        let max = max
            .filter(|value| value.is_finite())
            .unwrap_or(f64::INFINITY);

        let negative_sign = min < 0.0 && strip_leading_sign(&mut value, &self.minus_sign, true);
        if !negative_sign && max > 0.0 {
            strip_leading_sign(&mut value, &self.plus_sign, false);
        }

        if !self.grouping_separator.is_empty() {
            value = value.replace(&self.grouping_separator, "");
        }
        for digit in self.digits {
            value = value.replace(digit, "");
        }
        replace_first(&mut value, &self.decimal_separator, "");
        value.is_empty()
    }

    fn sanitize(&self, value: &str) -> String {
        let mut value = value
            .chars()
            .filter(|character| {
                !character.is_whitespace()
                    && CodePointMapData::<GeneralCategory>::new().get(*character)
                        != GeneralCategory::Format
            })
            .collect::<String>();

        if self.numbering_system == "arab" {
            if !self.decimal_separator.is_empty() {
                value = value.replace([',', '\u{060c}'], &self.decimal_separator);
            }
            if !self.grouping_separator.is_empty() {
                value = value.replace('.', &self.grouping_separator);
            }
        }
        if self.grouping_separator == "\u{2019}" {
            value = value.replace('\'', &self.grouping_separator);
        } else if self.grouping_separator == "'" {
            value = value.replace('\u{2019}', &self.grouping_separator);
        }
        value
    }
}

#[derive(Default)]
struct CollectedParts {
    text: String,
    parts: Vec<(usize, usize, Part)>,
}

impl CollectedParts {
    fn text_for_part(&self, target: Part) -> String {
        self.parts
            .iter()
            .filter(|(_, _, part)| *part == target)
            .filter_map(|(start, end, _)| self.text.get(*start..*end))
            .map(|value| {
                value
                    .chars()
                    .filter(|character| {
                        !character.is_whitespace()
                            && CodePointMapData::<GeneralCategory>::new().get(*character)
                                != GeneralCategory::Format
                    })
                    .collect()
            })
            .next()
            .unwrap_or_default()
    }
}

impl Write for CollectedParts {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        self.text.write_str(value)
    }

    fn write_char(&mut self, value: char) -> std::fmt::Result {
        self.text.write_char(value)
    }
}

impl PartsWrite for CollectedParts {
    type SubPartsWrite = Self;

    fn with_part(
        &mut self,
        part: Part,
        mut write: impl FnMut(&mut Self::SubPartsWrite) -> std::fmt::Result,
    ) -> std::fmt::Result {
        let start = self.text.len();
        write(self)?;
        let end = self.text.len();
        if start < end {
            self.parts.push((start, end, part));
        }
        Ok(())
    }
}

fn collect_parts(value: &FormattedDecimal<'_>) -> GuiResult<CollectedParts> {
    let mut collected = CollectedParts::default();
    value.write_to_parts(&mut collected).map_err(|error| {
        GuiError::internationalization(format!(
            "failed to inspect localized number symbols: {error}"
        ))
    })?;
    Ok(collected)
}

fn formatted_digits(formatter: &DecimalFormatter) -> GuiResult<[char; 10]> {
    let mut digits = ['0'; 10];
    for (index, slot) in digits.iter_mut().enumerate() {
        let formatted = formatter.format_to_string(&Decimal::from(index as u64));
        let mut characters = formatted.chars().filter(|character| {
            !character.is_whitespace()
                && CodePointMapData::<GeneralCategory>::new().get(*character)
                    != GeneralCategory::Format
        });
        let Some(digit) = characters.next() else {
            return Err(GuiError::internationalization(format!(
                "numbering system emitted no digit for {index}"
            )));
        };
        if characters.next().is_some() {
            return Err(GuiError::internationalization(format!(
                "numbering system emitted multiple characters for digit {index}"
            )));
        }
        *slot = digit;
    }
    Ok(digits)
}

fn decimal(value: &str) -> GuiResult<Decimal> {
    value.parse::<Decimal>().map_err(|error| {
        GuiError::internationalization(format!(
            "failed to prepare number parser symbol fixture {value:?}: {error}"
        ))
    })
}

fn numbering_system_preference(
    numbering_system: &str,
) -> GuiResult<Option<icu_decimal::preferences::NumberingSystem>> {
    let locale = format!("und-u-nu-{numbering_system}")
        .parse::<Locale>()
        .map_err(|error| {
            GuiError::internationalization(format!(
                "invalid numbering system {numbering_system:?}: {error}"
            ))
        })?;
    Ok(DecimalFormatterPreferences::from(locale).numbering_system)
}

fn explicit_numbering_system(locale: &str) -> Option<&str> {
    let subtags = locale.split('-').collect::<Vec<_>>();
    let unicode_extension = subtags
        .iter()
        .position(|subtag| subtag.eq_ignore_ascii_case("u"))?;
    let mut index = unicode_extension + 1;
    while index + 1 < subtags.len() {
        if subtags[index].len() == 1 {
            break;
        }
        if subtags[index].eq_ignore_ascii_case("nu") {
            return Some(subtags[index + 1]);
        }
        index += 1;
    }
    None
}

fn normalize_leading_sign(value: &mut String, localized: &str, normalized: &str, minus: bool) {
    if strip_leading_sign(value, localized, minus) {
        value.insert_str(0, normalized);
    }
}

fn strip_leading_sign(value: &mut String, localized: &str, minus: bool) -> bool {
    let ascii = if minus { "-" } else { "+" };
    let unicode_minus = minus.then_some("\u{2212}");
    for sign in [Some(localized), Some(ascii), unicode_minus]
        .into_iter()
        .flatten()
        .filter(|sign| !sign.is_empty())
    {
        if value.starts_with(sign) {
            value.drain(..sign.len());
            return true;
        }
    }
    false
}

fn replace_first(value: &mut String, from: &str, to: &str) {
    if from.is_empty() {
        return;
    }
    if let Some(start) = value.find(from) {
        value.replace_range(start..start + from.len(), to);
    }
}

fn numbering_system_for_digits(digits: &[char; 10]) -> Option<&'static str> {
    NUMBERING_SYSTEM_DIGITS
        .iter()
        .find(|(_, candidate)| candidate.chars().eq(digits.iter().copied()))
        .map(|(name, _)| *name)
}

fn digits_for_numbering_system(numbering_system: &str) -> Option<[char; 10]> {
    let digits = NUMBERING_SYSTEM_DIGITS
        .iter()
        .find(|(name, _)| *name == numbering_system)
        .map(|(_, digits)| *digits)?;
    let mut parsed = ['0'; 10];
    let mut characters = digits.chars();
    for digit in &mut parsed {
        *digit = characters.next()?;
    }
    characters.next().is_none().then_some(parsed)
}

const NUMBERING_SYSTEM_DIGITS: [(&str, &str); 28] = [
    ("adlm", "\u{1e950}\u{1e951}\u{1e952}\u{1e953}\u{1e954}\u{1e955}\u{1e956}\u{1e957}\u{1e958}\u{1e959}"),
    ("arab", "\u{0660}\u{0661}\u{0662}\u{0663}\u{0664}\u{0665}\u{0666}\u{0667}\u{0668}\u{0669}"),
    ("arabext", "\u{06f0}\u{06f1}\u{06f2}\u{06f3}\u{06f4}\u{06f5}\u{06f6}\u{06f7}\u{06f8}\u{06f9}"),
    ("beng", "\u{09e6}\u{09e7}\u{09e8}\u{09e9}\u{09ea}\u{09eb}\u{09ec}\u{09ed}\u{09ee}\u{09ef}"),
    ("cakm", "\u{11136}\u{11137}\u{11138}\u{11139}\u{1113a}\u{1113b}\u{1113c}\u{1113d}\u{1113e}\u{1113f}"),
    ("deva", "\u{0966}\u{0967}\u{0968}\u{0969}\u{096a}\u{096b}\u{096c}\u{096d}\u{096e}\u{096f}"),
    ("fullwide", "\u{ff10}\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{ff15}\u{ff16}\u{ff17}\u{ff18}\u{ff19}"),
    ("gujr", "\u{0ae6}\u{0ae7}\u{0ae8}\u{0ae9}\u{0aea}\u{0aeb}\u{0aec}\u{0aed}\u{0aee}\u{0aef}"),
    ("guru", "\u{0a66}\u{0a67}\u{0a68}\u{0a69}\u{0a6a}\u{0a6b}\u{0a6c}\u{0a6d}\u{0a6e}\u{0a6f}"),
    ("hanidec", "\u{3007}\u{4e00}\u{4e8c}\u{4e09}\u{56db}\u{4e94}\u{516d}\u{4e03}\u{516b}\u{4e5d}"),
    ("hmnp", "\u{1e140}\u{1e141}\u{1e142}\u{1e143}\u{1e144}\u{1e145}\u{1e146}\u{1e147}\u{1e148}\u{1e149}"),
    ("java", "\u{a9d0}\u{a9d1}\u{a9d2}\u{a9d3}\u{a9d4}\u{a9d5}\u{a9d6}\u{a9d7}\u{a9d8}\u{a9d9}"),
    ("khmr", "\u{17e0}\u{17e1}\u{17e2}\u{17e3}\u{17e4}\u{17e5}\u{17e6}\u{17e7}\u{17e8}\u{17e9}"),
    ("knda", "\u{0ce6}\u{0ce7}\u{0ce8}\u{0ce9}\u{0cea}\u{0ceb}\u{0cec}\u{0ced}\u{0cee}\u{0cef}"),
    ("laoo", "\u{0ed0}\u{0ed1}\u{0ed2}\u{0ed3}\u{0ed4}\u{0ed5}\u{0ed6}\u{0ed7}\u{0ed8}\u{0ed9}"),
    ("latn", "0123456789"),
    ("mlym", "\u{0d66}\u{0d67}\u{0d68}\u{0d69}\u{0d6a}\u{0d6b}\u{0d6c}\u{0d6d}\u{0d6e}\u{0d6f}"),
    ("mong", "\u{1810}\u{1811}\u{1812}\u{1813}\u{1814}\u{1815}\u{1816}\u{1817}\u{1818}\u{1819}"),
    ("mymr", "\u{1040}\u{1041}\u{1042}\u{1043}\u{1044}\u{1045}\u{1046}\u{1047}\u{1048}\u{1049}"),
    ("nkoo", "\u{07c0}\u{07c1}\u{07c2}\u{07c3}\u{07c4}\u{07c5}\u{07c6}\u{07c7}\u{07c8}\u{07c9}"),
    ("olck", "\u{1c50}\u{1c51}\u{1c52}\u{1c53}\u{1c54}\u{1c55}\u{1c56}\u{1c57}\u{1c58}\u{1c59}"),
    ("orya", "\u{0b66}\u{0b67}\u{0b68}\u{0b69}\u{0b6a}\u{0b6b}\u{0b6c}\u{0b6d}\u{0b6e}\u{0b6f}"),
    ("tamldec", "\u{0be6}\u{0be7}\u{0be8}\u{0be9}\u{0bea}\u{0beb}\u{0bec}\u{0bed}\u{0bee}\u{0bef}"),
    ("telu", "\u{0c66}\u{0c67}\u{0c68}\u{0c69}\u{0c6a}\u{0c6b}\u{0c6c}\u{0c6d}\u{0c6e}\u{0c6f}"),
    ("thai", "\u{0e50}\u{0e51}\u{0e52}\u{0e53}\u{0e54}\u{0e55}\u{0e56}\u{0e57}\u{0e58}\u{0e59}"),
    ("tibt", "\u{0f20}\u{0f21}\u{0f22}\u{0f23}\u{0f24}\u{0f25}\u{0f26}\u{0f27}\u{0f28}\u{0f29}"),
    ("vaii", "\u{a620}\u{a621}\u{a622}\u{a623}\u{a624}\u{a625}\u{a626}\u{a627}\u{a628}\u{a629}"),
    ("wara", "\u{118e0}\u{118e1}\u{118e2}\u{118e3}\u{118e4}\u{118e5}\u{118e6}\u{118e7}\u{118e8}\u{118e9}"),
];
