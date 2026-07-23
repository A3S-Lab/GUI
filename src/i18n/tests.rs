use super::*;
use crate::native::{ElementKey, NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::web::WebProps;

fn snapshot(node: u64, parent: Option<u64>, props: NativeProps) -> MountedNodeSnapshot {
    MountedNodeSnapshot {
        node: HostNodeId::new(node),
        parent: parent.map(HostNodeId::new),
        key: ElementKey::new(format!("node-{node}")),
        role: NativeRole::View,
        props,
    }
}

#[test]
fn locale_direction_recognizes_language_and_script_subtags() {
    assert_eq!(direction_for_locale("en-US"), TextDirection::Ltr);
    assert_eq!(direction_for_locale("ar-EG"), TextDirection::Rtl);
    assert_eq!(direction_for_locale("az-Arab"), TextDirection::Rtl);
    assert_eq!(direction_for_locale("az-Latn"), TextDirection::Ltr);
}

#[test]
fn mounted_context_inherits_and_allows_nested_overrides() {
    let records = vec![
        snapshot(1, None, NativeProps::new().lang("ar-EG")),
        snapshot(2, Some(1), NativeProps::new()),
        snapshot(3, Some(1), NativeProps::new().lang("en-GB").dir("rtl")),
    ];
    let mut manager = I18nManager::new();
    manager.sync(&records);

    assert_eq!(manager.locale(HostNodeId::new(2)), Some("ar-EG"));
    assert_eq!(manager.direction(HostNodeId::new(2)), TextDirection::Rtl);
    assert_eq!(manager.locale(HostNodeId::new(3)), Some("en-GB"));
    assert_eq!(manager.direction(HostNodeId::new(3)), TextDirection::Rtl);
}

#[test]
fn projection_applies_effective_context_to_native_descendants() {
    let mut root = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().lang("ar-EG"))
        .child(NativeElement::new("child", NativeRole::Button).with_props(
            NativeProps::new().web(WebProps::new().attribute("lang", "").attribute("dir", "")),
        ));
    I18nManager::new().project_native_tree(&mut root);

    let child = &root.children[0].props;
    assert_eq!(child.lang.as_deref(), Some("ar-EG"));
    assert_eq!(child.dir.as_deref(), Some("rtl"));
    assert_eq!(
        child.web.attributes.get("dir").map(String::as_str),
        Some("rtl")
    );
}

#[test]
fn default_locale_can_seed_a_tree_without_an_explicit_provider() {
    let mut manager = I18nManager::new();
    manager.set_default_locale(Some("he-IL"));
    let mut root = NativeElement::new("root", NativeRole::View)
        .child(NativeElement::new("child", NativeRole::Text));

    manager.project_native_tree(&mut root);

    assert_eq!(root.children[0].props.lang.as_deref(), Some("he-IL"));
    assert_eq!(root.children[0].props.dir.as_deref(), Some("rtl"));
}

#[test]
fn collator_supports_search_sensitivity_and_numeric_sorting() {
    let collator = LocaleCollator::try_new(
        "fr-FR",
        CollationOptions::default()
            .usage(CollationUsage::Search)
            .sensitivity(CollationSensitivity::Base)
            .numeric(true),
    )
    .expect("French search collator should load");

    assert!(collator.is_equal("\u{00c9}clair", "eclair"));
    assert_eq!(
        collator.compare("document2", "document10"),
        std::cmp::Ordering::Less
    );
}

#[test]
fn collator_filters_locale_equivalent_unicode_substrings() {
    let collator = LocaleCollator::try_new(
        "fr-FR",
        CollationOptions::default()
            .usage(CollationUsage::Search)
            .sensitivity(CollationSensitivity::Base),
    )
    .expect("French search collator should load");

    assert!(collator.starts_with("\u{00c9}clair", "e"));
    assert!(collator.ends_with("cr\u{00e8}me br\u{00fb}l\u{00e9}e", "BRULEE"));
    assert!(collator.contains("caf\u{00e9} noir", "CAFE"));
    assert!(collator.contains("e\u{301}clair", "\u{00c9}"));
    assert!(collator.starts_with("value", ""));
    assert!(!collator.contains("caf\u{00e9}", "tea"));
}

#[test]
fn number_formatter_localizes_and_applies_intl_fraction_defaults() {
    let formatter = LocaleNumberFormatter::try_new(
        "en-US",
        NumberFormatOptions::default().fraction_digits(2, 2),
    )
    .expect("English number formatter should load");

    assert_eq!(formatter.format_decimal("1234.5").unwrap(), "1,234.50");
    assert_eq!(formatter.format_decimal("1.005").unwrap(), "1.01");

    let french = LocaleNumberFormatter::try_new("fr-FR", NumberFormatOptions::default())
        .expect("French number formatter should load")
        .format_decimal("1234.5")
        .unwrap();
    assert_ne!(french, "1,234.5");
    assert!(french.ends_with(",5"));
}

#[test]
fn number_parser_localizes_symbols_and_detects_supported_digits() {
    let english = LocaleNumberParser::try_new("en-US").expect("English number parser should load");
    assert_eq!(english.parse("1,234.5").unwrap(), 1234.5);
    assert_eq!(english.parse("\u{0661}\u{0662}").unwrap(), 12.0);
    assert!(
        english.parse("\u{ff11}\u{ff12}\u{ff0e}\u{ff15}").is_err(),
        "full-width digits use the locale decimal separator",
    );
    assert_eq!(english.parse("\u{ff11}\u{ff12}.\u{ff15}").unwrap(), 12.5);
    assert_eq!(english.numbering_system("\u{0661}\u{0662}"), "arab");
    assert_eq!(english.numbering_system("\u{4e00}\u{4e8c}"), "hanidec");

    let french = LocaleNumberParser::try_new("fr-FR").expect("French number parser should load");
    assert_eq!(french.parse("1\u{202f}234,5").unwrap(), 1234.5);
    assert_eq!(french.parse("1 234,5").unwrap(), 1234.5);
    assert!(french.parse("1.5").is_err());

    let arabic = LocaleNumberParser::try_new("ar-EG").expect("Arabic number parser should load");
    assert_eq!(
        arabic
            .parse("\u{0661}\u{066c}\u{0662}\u{0663}\u{0664}\u{066b}\u{0665}")
            .unwrap(),
        1234.5
    );
}

#[test]
fn number_parser_validates_partial_input_and_explicit_number_systems() {
    let parser = LocaleNumberParser::try_new("en-US").expect("English number parser should load");
    assert!(parser.is_valid_partial_number("", None, None));
    assert!(parser.is_valid_partial_number("-", Some(-10.0), Some(10.0)));
    assert!(!parser.is_valid_partial_number("-", Some(0.0), Some(10.0)));
    assert!(parser.is_valid_partial_number(".", None, None));
    assert!(parser.is_valid_partial_number("1,", None, None));
    assert!(!parser.is_valid_partial_number("1..", None, None));
    assert!(!parser.is_valid_partial_number("12kg", None, None));

    let devanagari = LocaleNumberParser::try_new("en-US-u-nu-deva")
        .expect("explicit Devanagari parser should load");
    assert_eq!(devanagari.parse("\u{0967}\u{0968}").unwrap(), 12.0);
    assert!(devanagari.parse("12").is_err());
    assert_eq!(devanagari.numbering_system(""), "deva");
}

#[test]
fn date_formatter_localizes_date_time_and_hour_cycle() {
    let value =
        DateTimeValue::date_time(2025, 1, 15, 16, 9, 35).expect("fixture date should be valid");
    let date = LocaleDateFormatter::try_new("en-US", DateFormatOptions::default())
        .expect("English date formatter should load");
    assert_eq!(date.format(value).unwrap(), "Jan 15, 2025");

    let full = LocaleDateFormatter::try_new(
        "en-US",
        DateFormatOptions::default().style(DateFormatStyle::Full),
    )
    .expect("English full date formatter should load");
    assert_eq!(full.format(value).unwrap(), "Wednesday, January 15, 2025");

    let time = LocaleDateFormatter::try_new(
        "en-US-u-hc-h23",
        DateFormatOptions::default()
            .kind(DateFormatKind::Time)
            .style(DateFormatStyle::Short)
            .include_seconds(true),
    )
    .expect("24-hour time formatter should load");
    assert_eq!(time.format(value).unwrap(), "16:09:35");
}

#[test]
fn manager_factories_follow_inherited_locale_and_default_context() {
    let records = vec![
        snapshot(1, None, NativeProps::new().lang("fr-FR")),
        snapshot(2, Some(1), NativeProps::new()),
    ];
    let mut manager = I18nManager::new();
    manager.sync(&records);

    let formatter = manager
        .number_formatter(HostNodeId::new(2), NumberFormatOptions::default())
        .expect("inherited formatter should load");
    assert_eq!(formatter.locale(), "fr-FR");
    let parser = manager
        .number_parser(HostNodeId::new(2))
        .expect("inherited parser should load");
    assert_eq!(parser.locale(), "fr-FR");
    assert_eq!(parser.parse("1,5").unwrap(), 1.5);

    manager.set_default_locale(Some("de-DE"));
    let formatter = manager
        .date_formatter(HostNodeId::new(99), DateFormatOptions::default())
        .expect("default formatter should load");
    assert_eq!(formatter.locale(), "de-DE");
}

#[test]
fn formatter_inputs_fail_with_context_instead_of_panicking() {
    let locale_error =
        LocaleCollator::try_new("not a locale", CollationOptions::default()).unwrap_err();
    assert!(locale_error.to_string().contains("invalid BCP 47 locale"));

    let options_error = LocaleNumberFormatter::try_new(
        "en-US",
        NumberFormatOptions::default().fraction_digits(4, 2),
    )
    .unwrap_err();
    assert!(options_error
        .to_string()
        .contains("minimum fraction digits"));

    assert!(DateTimeValue::date(2025, 2, 29).is_err());
}

#[test]
fn reusable_i18n_formatters_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<LocaleCollator>();
    assert_send_sync::<LocaleNumberFormatter>();
    assert_send_sync::<LocaleNumberParser>();
    assert_send_sync::<LocaleDateFormatter>();
}
