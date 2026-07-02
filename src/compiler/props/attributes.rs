use std::collections::BTreeMap;

pub(super) fn string_attribute<'a>(
    attributes: &'a BTreeMap<String, String>,
    names: &[&str],
) -> Option<&'a str> {
    names
        .iter()
        .find_map(|name| attributes.get(*name).map(String::as_str))
}

pub(super) fn non_empty_string_attribute<'a>(
    attributes: &'a BTreeMap<String, String>,
    names: &[&str],
) -> Option<&'a str> {
    string_attribute(attributes, names)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

pub(super) fn non_empty_string_value(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

pub(super) fn html_string_attribute(
    attributes: &BTreeMap<String, String>,
    names: &[&str],
) -> Option<String> {
    non_empty_string_attribute(attributes, names).map(str::to_string)
}

pub(super) fn html_present_string_attribute(
    attributes: &BTreeMap<String, String>,
    names: &[&str],
) -> Option<String> {
    string_attribute(attributes, names).map(|value| value.trim().to_string())
}

pub(super) fn bool_attribute(
    attributes: &BTreeMap<String, String>,
    names: &[&str],
) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_bool_attribute)
}

pub(super) fn invalid_attribute(
    attributes: &BTreeMap<String, String>,
    names: &[&str],
) -> Option<bool> {
    string_attribute(attributes, names).and_then(parse_invalid_attribute)
}

pub(super) fn number_attribute(
    attributes: &BTreeMap<String, String>,
    names: &[&str],
) -> Option<f64> {
    string_attribute(attributes, names).and_then(parse_number_attribute)
}

pub(super) fn u32_attribute(attributes: &BTreeMap<String, String>, names: &[&str]) -> Option<u32> {
    string_attribute(attributes, names).and_then(parse_u32_attribute)
}

pub(super) fn parse_number_attribute(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
}

fn parse_u32_attribute(value: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

fn parse_bool_attribute(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn parse_invalid_attribute(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "" | "true" | "grammar" | "spelling" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
