use std::fmt::{Debug, Formatter};

use icu_datetime::fieldsets::{T, YMD, YMDE, YMDET, YMDT};
use icu_datetime::input::{Date, DateTime, Time};
use icu_datetime::options::{Length, TimePrecision};
use icu_datetime::{DateTimeFormatter, NoCalendarFormatter};
use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};

use super::parse_locale;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DateFormatKind {
    #[default]
    Date,
    DateTime,
    Time,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DateFormatStyle {
    Short,
    #[default]
    Medium,
    Long,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateFormatOptions {
    pub kind: DateFormatKind,
    pub style: DateFormatStyle,
    pub include_seconds: bool,
}

impl Default for DateFormatOptions {
    fn default() -> Self {
        Self {
            kind: DateFormatKind::Date,
            style: DateFormatStyle::Medium,
            include_seconds: false,
        }
    }
}

impl DateFormatOptions {
    pub fn kind(mut self, kind: DateFormatKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn style(mut self, style: DateFormatStyle) -> Self {
        self.style = style;
        self
    }

    pub fn include_seconds(mut self, include_seconds: bool) -> Self {
        self.include_seconds = include_seconds;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeValue {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl DateTimeValue {
    pub fn date(year: i32, month: u8, day: u8) -> GuiResult<Self> {
        Self::try_new(year, month, day, 0, 0, 0, 0)
    }

    pub fn date_time(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> GuiResult<Self> {
        Self::try_new(year, month, day, hour, minute, second, 0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> GuiResult<Self> {
        validate_date(year, month, day)?;
        validate_time(hour, minute, second, nanosecond)?;
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    pub fn year(self) -> i32 {
        self.year
    }

    pub fn month(self) -> u8 {
        self.month
    }

    pub fn day(self) -> u8 {
        self.day
    }

    pub fn hour(self) -> u8 {
        self.hour
    }

    pub fn minute(self) -> u8 {
        self.minute
    }

    pub fn second(self) -> u8 {
        self.second
    }

    pub fn nanosecond(self) -> u32 {
        self.nanosecond
    }
}

enum DateFormatterInner {
    Date(DateTimeFormatter<YMD>),
    FullDate(DateTimeFormatter<YMDE>),
    DateTime(DateTimeFormatter<YMDT>),
    FullDateTime(DateTimeFormatter<YMDET>),
    Time(NoCalendarFormatter<T>),
}

/// A reusable locale-aware Gregorian date and time formatter.
///
/// Locale Unicode extensions select the calendar, numbering system, and hour
/// cycle. `Full` date styles include the weekday.
pub struct LocaleDateFormatter {
    locale: String,
    options: DateFormatOptions,
    inner: DateFormatterInner,
}

impl LocaleDateFormatter {
    pub fn try_new(locale: &str, options: DateFormatOptions) -> GuiResult<Self> {
        let locale = parse_locale(locale)?;
        let canonical_locale = locale.to_string();
        let length = style_length(options.style);
        let inner = match (options.kind, options.style) {
            (DateFormatKind::Date, DateFormatStyle::Full) => {
                DateTimeFormatter::try_new(locale.into(), YMDE::long())
                    .map(DateFormatterInner::FullDate)
            }
            (DateFormatKind::Date, _) => {
                DateTimeFormatter::try_new(locale.into(), YMD::for_length(length))
                    .map(DateFormatterInner::Date)
            }
            (DateFormatKind::DateTime, DateFormatStyle::Full) => {
                let mut fields = YMDE::long().with_time_hm();
                fields.time_precision = Some(time_precision(options.include_seconds));
                DateTimeFormatter::try_new(locale.into(), fields)
                    .map(DateFormatterInner::FullDateTime)
            }
            (DateFormatKind::DateTime, _) => {
                let mut fields = YMD::for_length(length).with_time_hm();
                fields.time_precision = Some(time_precision(options.include_seconds));
                DateTimeFormatter::try_new(locale.into(), fields).map(DateFormatterInner::DateTime)
            }
            (DateFormatKind::Time, _) => {
                let mut fields = T::for_length(length);
                fields.time_precision = Some(time_precision(options.include_seconds));
                NoCalendarFormatter::try_new(locale.into(), fields).map(DateFormatterInner::Time)
            }
        }
        .map_err(|error| {
            GuiError::internationalization(format!(
                "failed to create date formatter for locale {canonical_locale:?}: {error}"
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

    pub fn options(&self) -> DateFormatOptions {
        self.options
    }

    pub fn format(&self, value: DateTimeValue) -> GuiResult<String> {
        let date = Date::try_new_iso(value.year, value.month, value.day).map_err(|error| {
            GuiError::internationalization(format!("invalid date passed to formatter: {error}"))
        })?;
        let time = Time::try_new(value.hour, value.minute, value.second, value.nanosecond)
            .map_err(|error| {
                GuiError::internationalization(format!("invalid time passed to formatter: {error}"))
            })?;

        Ok(match &self.inner {
            DateFormatterInner::Date(formatter) => formatter.format(&date).to_string(),
            DateFormatterInner::FullDate(formatter) => formatter.format(&date).to_string(),
            DateFormatterInner::DateTime(formatter) => {
                formatter.format(&DateTime { date, time }).to_string()
            }
            DateFormatterInner::FullDateTime(formatter) => {
                formatter.format(&DateTime { date, time }).to_string()
            }
            DateFormatterInner::Time(formatter) => formatter.format(&time).to_string(),
        })
    }
}

impl Debug for LocaleDateFormatter {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocaleDateFormatter")
            .field("locale", &self.locale)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

fn style_length(style: DateFormatStyle) -> Length {
    match style {
        DateFormatStyle::Short => Length::Short,
        DateFormatStyle::Medium => Length::Medium,
        DateFormatStyle::Long | DateFormatStyle::Full => Length::Long,
    }
}

fn time_precision(include_seconds: bool) -> TimePrecision {
    if include_seconds {
        TimePrecision::Second
    } else {
        TimePrecision::Minute
    }
}

fn validate_date(year: i32, month: u8, day: u8) -> GuiResult<()> {
    Date::try_new_iso(year, month, day)
        .map(|_| ())
        .map_err(|error| {
            GuiError::internationalization(format!(
                "invalid ISO date {year:04}-{month:02}-{day:02}: {error}"
            ))
        })
}

fn validate_time(hour: u8, minute: u8, second: u8, nanosecond: u32) -> GuiResult<()> {
    Time::try_new(hour, minute, second, nanosecond)
        .map(|_| ())
        .map_err(|error| {
            GuiError::internationalization(format!(
                "invalid time {hour:02}:{minute:02}:{second:02}.{nanosecond:09}: {error}"
            ))
        })
}
