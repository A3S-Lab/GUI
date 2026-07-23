use crate::error::GuiResult;

use super::parse_locale;

/// Locale set mirrored by React Aria's NumberField and SpinButton message
/// packages. Message text is synchronized with Adobe React Spectrum's
/// Apache-2.0-licensed locale data.
pub const NUMBER_FIELD_MESSAGE_LOCALES: &[&str] = &[
    "ar-AE", "bg-BG", "cs-CZ", "da-DK", "de-DE", "el-GR", "en-US", "es-ES", "et-EE", "fi-FI",
    "fr-FR", "he-IL", "hr-HR", "hu-HU", "it-IT", "ja-JP", "ko-KR", "lt-LT", "lv-LV", "nb-NO",
    "nl-NL", "pl-PL", "pt-BR", "pt-PT", "ro-RO", "ru-RU", "sk-SK", "sl-SI", "sr-SP", "sv-SE",
    "tr-TR", "uk-UA", "zh-CN", "zh-TW",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberFieldStepAction {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleMessageFormatter {
    locale: String,
    resolved_locale: &'static str,
    messages: NumberFieldMessages,
}

impl LocaleMessageFormatter {
    pub fn try_new(locale: &str) -> GuiResult<Self> {
        let locale = parse_locale(locale)?.to_string();
        let (resolved_locale, messages) = number_field_messages(&locale);
        Ok(Self {
            locale,
            resolved_locale,
            messages,
        })
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn resolved_locale(&self) -> &str {
        self.resolved_locale
    }

    pub fn number_field_step_label(
        &self,
        action: NumberFieldStepAction,
        field_label: Option<&str>,
    ) -> String {
        let pattern = match action {
            NumberFieldStepAction::Increment => self.messages.increase,
            NumberFieldStepAction::Decrement => self.messages.decrease,
        };
        pattern
            .replace("{fieldLabel}", field_label.unwrap_or_default().trim())
            .trim()
            .to_string()
    }

    pub fn number_field_role_description(&self) -> &'static str {
        self.messages.number_field
    }

    pub fn spin_button_empty(&self) -> &'static str {
        self.messages.empty
    }

    pub(crate) fn for_locale_lossy(locale: &str) -> Self {
        Self::try_new(locale).unwrap_or_default()
    }
}

impl Default for LocaleMessageFormatter {
    fn default() -> Self {
        let (resolved_locale, messages) = number_field_messages("en-US");
        Self {
            locale: "en-US".to_string(),
            resolved_locale,
            messages,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NumberFieldMessages {
    decrease: &'static str,
    increase: &'static str,
    number_field: &'static str,
    empty: &'static str,
}

fn number_field_messages(locale: &str) -> (&'static str, NumberFieldMessages) {
    let language = locale
        .split('-')
        .next()
        .unwrap_or("en")
        .to_ascii_lowercase();
    let subtags = locale
        .split('-')
        .skip(1)
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    let has_subtag = |expected: &str| {
        subtags
            .iter()
            .any(|subtag| subtag.eq_ignore_ascii_case(expected))
    };

    match language.as_str() {
        "ar" => (
            "ar-AE",
            messages("خفض {fieldLabel}", "زيادة {fieldLabel}", "حقل رقمي", "فارغ"),
        ),
        "bg" => (
            "bg-BG",
            messages(
                "Намаляване {fieldLabel}",
                "Усилване {fieldLabel}",
                "Номер на полето",
                "Изпразни",
            ),
        ),
        "cs" => (
            "cs-CZ",
            messages(
                "Snížit {fieldLabel}",
                "Zvýšit {fieldLabel}",
                "Číselné pole",
                "Prázdné",
            ),
        ),
        "da" => (
            "da-DK",
            messages("Reducer {fieldLabel}", "Øg {fieldLabel}", "Talfelt", "Tom"),
        ),
        "de" => (
            "de-DE",
            messages(
                "{fieldLabel} verringern",
                "{fieldLabel} erhöhen",
                "Nummernfeld",
                "Leer",
            ),
        ),
        "el" => (
            "el-GR",
            messages(
                "Μείωση {fieldLabel}",
                "Αύξηση {fieldLabel}",
                "Πεδίο αριθμού",
                "Άδειο",
            ),
        ),
        "es" => (
            "es-ES",
            messages(
                "Reducir {fieldLabel}",
                "Aumentar {fieldLabel}",
                "Campo de número",
                "Vacío",
            ),
        ),
        "et" => (
            "et-EE",
            messages(
                "Vähenda {fieldLabel}",
                "Suurenda {fieldLabel}",
                "Numbri väli",
                "Tühjenda",
            ),
        ),
        "fi" => (
            "fi-FI",
            messages(
                "Vähennä {fieldLabel}",
                "Lisää {fieldLabel}",
                "Numerokenttä",
                "Tyhjä",
            ),
        ),
        "fr" => (
            "fr-FR",
            messages(
                "Diminuer {fieldLabel}",
                "Augmenter {fieldLabel}",
                "Champ de nombre",
                "Vide",
            ),
        ),
        "he" | "iw" => (
            "he-IL",
            messages("הקטן {fieldLabel}", "הגדל {fieldLabel}", "שדה מספר", "ריק"),
        ),
        "hr" => (
            "hr-HR",
            messages(
                "Smanji {fieldLabel}",
                "Povećaj {fieldLabel}",
                "Polje broja",
                "Prazno",
            ),
        ),
        "hu" => (
            "hu-HU",
            messages(
                "{fieldLabel} csökkentése",
                "{fieldLabel} növelése",
                "Számmező",
                "Üres",
            ),
        ),
        "it" => (
            "it-IT",
            messages(
                "Riduci {fieldLabel}",
                "Aumenta {fieldLabel}",
                "Campo numero",
                "Vuoto",
            ),
        ),
        "ja" => (
            "ja-JP",
            messages(
                "{fieldLabel}を縮小",
                "{fieldLabel}を拡大",
                "数値フィールド",
                "空",
            ),
        ),
        "ko" => (
            "ko-KR",
            messages(
                "{fieldLabel} 감소",
                "{fieldLabel} 증가",
                "번호 필드",
                "비어 있음",
            ),
        ),
        "lt" => (
            "lt-LT",
            messages(
                "Sumažinti {fieldLabel}",
                "Padidinti {fieldLabel}",
                "Numerio laukas",
                "Tuščias",
            ),
        ),
        "lv" => (
            "lv-LV",
            messages(
                "Samazināšana {fieldLabel}",
                "Palielināšana {fieldLabel}",
                "Skaitļu lauks",
                "Tukšs",
            ),
        ),
        "nb" | "no" => (
            "nb-NO",
            messages("Reduser {fieldLabel}", "Øk {fieldLabel}", "Tallfelt", "Tom"),
        ),
        "nl" => (
            "nl-NL",
            messages(
                "{fieldLabel} verlagen",
                "{fieldLabel} verhogen",
                "Getalveld",
                "Leeg",
            ),
        ),
        "pl" => (
            "pl-PL",
            messages(
                "Zmniejsz {fieldLabel}",
                "Zwiększ {fieldLabel}",
                "Pole numeru",
                "Pusty",
            ),
        ),
        "pt" if has_subtag("br") => (
            "pt-BR",
            messages(
                "Diminuir {fieldLabel}",
                "Aumentar {fieldLabel}",
                "Campo de número",
                "Vazio",
            ),
        ),
        "pt" => (
            "pt-PT",
            messages(
                "Diminuir {fieldLabel}",
                "Aumentar {fieldLabel}",
                "Campo numérico",
                "Vazio",
            ),
        ),
        "ro" => (
            "ro-RO",
            messages(
                "Scădere {fieldLabel}",
                "Creștere {fieldLabel}",
                "Câmp numeric",
                "Gol",
            ),
        ),
        "ru" => (
            "ru-RU",
            messages(
                "Уменьшение {fieldLabel}",
                "Увеличение {fieldLabel}",
                "Числовое поле",
                "Не заполнено",
            ),
        ),
        "sk" => (
            "sk-SK",
            messages(
                "Znížiť {fieldLabel}",
                "Zvýšiť {fieldLabel}",
                "Číselné pole",
                "Prázdne",
            ),
        ),
        "sl" => (
            "sl-SI",
            messages(
                "Upadati {fieldLabel}",
                "Povečajte {fieldLabel}",
                "Številčno polje",
                "Prazen",
            ),
        ),
        "sr" => (
            "sr-SP",
            messages(
                "Smanji {fieldLabel}",
                "Povećaj {fieldLabel}",
                "Polje broja",
                "Prazno",
            ),
        ),
        "sv" => (
            "sv-SE",
            messages(
                "Minska {fieldLabel}",
                "Öka {fieldLabel}",
                "Nummerfält",
                "Tomt",
            ),
        ),
        "tr" => (
            "tr-TR",
            messages(
                "{fieldLabel} azalt",
                "{fieldLabel} arttır",
                "Sayı alanı",
                "Boş",
            ),
        ),
        "uk" => (
            "uk-UA",
            messages(
                "Зменшити {fieldLabel}",
                "Збільшити {fieldLabel}",
                "Поле номера",
                "Пусто",
            ),
        ),
        "zh" if has_subtag("hant") || has_subtag("tw") || has_subtag("hk") || has_subtag("mo") => (
            "zh-TW",
            messages("縮小 {fieldLabel}", "放大 {fieldLabel}", "數字欄位", "空白"),
        ),
        "zh" => (
            "zh-CN",
            messages("降低 {fieldLabel}", "提高 {fieldLabel}", "数字字段", "空"),
        ),
        _ => (
            "en-US",
            messages(
                "Decrease {fieldLabel}",
                "Increase {fieldLabel}",
                "Number field",
                "Empty",
            ),
        ),
    }
}

const fn messages(
    decrease: &'static str,
    increase: &'static str,
    number_field: &'static str,
    empty: &'static str,
) -> NumberFieldMessages {
    NumberFieldMessages {
        decrease,
        increase,
        number_field,
        empty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_field_messages_follow_locale_order_and_fallbacks() {
        let german = LocaleMessageFormatter::try_new("de-AT").unwrap();
        assert_eq!(german.resolved_locale(), "de-DE");
        assert_eq!(
            german.number_field_step_label(NumberFieldStepAction::Increment, Some("Menge")),
            "Menge erhöhen"
        );
        assert_eq!(german.number_field_role_description(), "Nummernfeld");

        let japanese = LocaleMessageFormatter::try_new("ja").unwrap();
        assert_eq!(
            japanese.number_field_step_label(NumberFieldStepAction::Decrement, Some("数量")),
            "数量を縮小"
        );

        let traditional_chinese = LocaleMessageFormatter::try_new("zh-Hant-HK").unwrap();
        assert_eq!(traditional_chinese.resolved_locale(), "zh-TW");
        assert_eq!(traditional_chinese.spin_button_empty(), "空白");

        let fallback = LocaleMessageFormatter::try_new("is-IS").unwrap();
        assert_eq!(fallback.resolved_locale(), "en-US");
        assert_eq!(
            fallback.number_field_step_label(NumberFieldStepAction::Increment, None),
            "Increase"
        );
    }

    #[test]
    fn every_declared_number_field_locale_resolves_to_its_catalog() {
        for locale in NUMBER_FIELD_MESSAGE_LOCALES {
            let formatter = LocaleMessageFormatter::try_new(locale).unwrap();
            assert_eq!(formatter.resolved_locale(), *locale);
            assert!(!formatter.number_field_role_description().is_empty());
            assert!(!formatter.spin_button_empty().is_empty());
            assert!(!formatter
                .number_field_step_label(NumberFieldStepAction::Increment, Some("Field"))
                .is_empty());
            assert!(!formatter
                .number_field_step_label(NumberFieldStepAction::Decrement, Some("Field"))
                .is_empty());
        }
    }
}
