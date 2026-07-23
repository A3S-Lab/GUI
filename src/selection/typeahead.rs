use std::time::{Duration, Instant};

use crate::i18n::{
    CollationOptions, CollationSensitivity, CollationUsage, LocaleCollator,
    DEFAULT_FORMATTING_LOCALE,
};

use super::CollectionKey;

const TYPEAHEAD_TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct TypeaheadState {
    search: String,
    last_input: Option<Instant>,
}

impl TypeaheadState {
    pub(super) fn push(&mut self, input: &str, now: Instant) -> Option<&str> {
        if !is_printable_character(input) {
            return None;
        }
        if self.last_input.is_some_and(|last_input| {
            now.saturating_duration_since(last_input) >= TYPEAHEAD_TIMEOUT
        }) {
            self.search.clear();
        }
        if self.search.is_empty() && input.chars().all(char::is_whitespace) {
            self.last_input = None;
            return None;
        }

        self.search.push_str(input);
        self.last_input = Some(now);
        Some(&self.search)
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TypeaheadCandidate<'a> {
    pub key: &'a CollectionKey,
    pub text: &'a str,
    pub disabled: bool,
}

pub(super) fn find_match(
    candidates: &[TypeaheadCandidate<'_>],
    search: &str,
    current: Option<&CollectionKey>,
    locale: Option<&str>,
) -> Option<CollectionKey> {
    if search.is_empty() || candidates.is_empty() {
        return None;
    }
    let collator = search_collator(locale)?;
    let start = current
        .and_then(|current| {
            candidates
                .iter()
                .position(|candidate| candidate.key == current)
        })
        .unwrap_or(0);

    candidates[start..]
        .iter()
        .chain(candidates[..start].iter())
        .find(|candidate| {
            !candidate.disabled
                && !candidate.text.is_empty()
                && collator.starts_with(candidate.text, search)
        })
        .map(|candidate| candidate.key.clone())
}

fn is_printable_character(input: &str) -> bool {
    let mut characters = input.chars();
    characters
        .next()
        .is_some_and(|character| !character.is_control())
        && characters.next().is_none()
}

fn search_collator(locale: Option<&str>) -> Option<LocaleCollator> {
    let options = CollationOptions::default()
        .usage(CollationUsage::Search)
        .sensitivity(CollationSensitivity::Base);
    LocaleCollator::try_new(locale.unwrap_or(DEFAULT_FORMATTING_LOCALE), options)
        .or_else(|_| LocaleCollator::try_new(DEFAULT_FORMATTING_LOCALE, options))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_expires_after_the_react_aria_interval() {
        let start = Instant::now();
        let mut state = TypeaheadState::default();

        assert_eq!(state.push("g", start), Some("g"));
        assert_eq!(
            state.push("r", start + Duration::from_millis(499)),
            Some("gr")
        );
        assert_eq!(
            state.push("a", start + Duration::from_millis(999)),
            Some("a")
        );
    }

    #[test]
    fn initial_space_and_non_character_keys_do_not_start_a_search() {
        let now = Instant::now();
        let mut state = TypeaheadState::default();

        assert_eq!(state.push(" ", now), None);
        assert_eq!(state.push("ArrowDown", now), None);
        assert_eq!(state.push("g", now), Some("g"));
        assert_eq!(state.push(" ", now), Some("g "));
    }

    #[test]
    fn search_collation_ignores_case_and_accents_and_wraps() {
        let alpha = CollectionKey::from("alpha");
        let eclair = CollectionKey::from("eclair");
        let zulu = CollectionKey::from("zulu");
        let candidates = [
            TypeaheadCandidate {
                key: &alpha,
                text: "Alpha",
                disabled: false,
            },
            TypeaheadCandidate {
                key: &eclair,
                text: "Éclair",
                disabled: false,
            },
            TypeaheadCandidate {
                key: &zulu,
                text: "Zulu",
                disabled: false,
            },
        ];

        assert_eq!(
            find_match(&candidates, "e", Some(&zulu), Some("fr-FR")),
            Some(eclair.clone())
        );
        assert_eq!(
            find_match(&candidates, "a", Some(&zulu), Some("en-US")),
            Some(alpha.clone())
        );
    }

    #[test]
    fn search_skips_fully_disabled_candidates() {
        let first = CollectionKey::from("first");
        let second = CollectionKey::from("second");
        let candidates = [
            TypeaheadCandidate {
                key: &first,
                text: "Gamma",
                disabled: true,
            },
            TypeaheadCandidate {
                key: &second,
                text: "Garden",
                disabled: false,
            },
        ];

        assert_eq!(
            find_match(&candidates, "ga", None, Some("en-US")),
            Some(second)
        );
    }

    #[test]
    fn invalid_inherited_locale_falls_back_without_disabling_typeahead() {
        let key = CollectionKey::from("alpha");
        let candidates = [TypeaheadCandidate {
            key: &key,
            text: "Alpha",
            disabled: false,
        }];

        assert_eq!(
            find_match(&candidates, "a", None, Some("not a locale")),
            Some(key)
        );
    }
}
