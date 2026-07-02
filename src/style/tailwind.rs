#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TailwindClass<'a> {
    pub(crate) variants: Vec<String>,
    pub(crate) utility: &'a str,
    pub(crate) important: bool,
}

pub(crate) fn parse_class(class: &str) -> Option<TailwindClass<'_>> {
    let class = class.trim();
    if class.is_empty() {
        return None;
    }

    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut start = 0usize;
    let mut variants = Vec::new();
    for (index, ch) in class.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' if bracket_depth == 0 => paren_depth += 1,
            ')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            ':' if bracket_depth == 0 && paren_depth == 0 => {
                variants.push(class[start..index].to_string());
                start = index + 1;
            }
            _ => {}
        }
    }

    let utility = &class[start..];
    let (important, utility) = match utility.strip_prefix('!') {
        Some(utility) if !utility.is_empty() => (true, utility),
        Some(utility) => (false, utility),
        None => (false, utility),
    };

    Some(TailwindClass {
        variants,
        utility,
        important,
    })
}

pub(crate) fn ordered_class_tokens(class_name: &str) -> Vec<&str> {
    let mut normal = Vec::new();
    let mut important = Vec::new();

    for class in class_name.split_whitespace() {
        if parse_class(class).is_some_and(|parsed| parsed.important) {
            important.push(class);
        } else {
            normal.push(class);
        }
    }

    normal.extend(important);
    normal
}

pub(crate) fn variant_key(variants: &[String]) -> String {
    variants
        .iter()
        .map(|variant| decode_variant_token(variant))
        .collect::<Vec<_>>()
        .join(":")
}

fn decode_variant_token(variant: &str) -> String {
    let mut output = String::with_capacity(variant.len());
    let mut chars = variant.char_indices().peekable();

    while let Some((_, ch)) = chars.next() {
        if ch != '[' {
            output.push(ch);
            continue;
        }

        let mut content = String::new();
        let mut bracket_depth = 1usize;
        while let Some((_, inner)) = chars.next() {
            match inner {
                '[' => {
                    bracket_depth += 1;
                    content.push(inner);
                }
                ']' => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                    if bracket_depth == 0 {
                        break;
                    }
                    content.push(inner);
                }
                _ => content.push(inner),
            }
        }

        output.push('[');
        output.push_str(&decode_arbitrary_value(&content));
        output.push(']');
    }

    output
}

pub(crate) fn arbitrary_or_custom_var(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(decode_arbitrary_value(arbitrary));
    }
    custom_var(value)
}

pub(crate) fn custom_var(value: &str) -> Option<String> {
    let variable = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))?
        .trim();
    if variable.is_empty() {
        None
    } else {
        Some(format!("var({variable})"))
    }
}

pub(crate) fn typed_custom_var(value: &str, expected_type: &str) -> Option<String> {
    let value = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))?;
    let (value_type, variable) = value.split_once(':')?;
    if value_type == expected_type && !variable.trim().is_empty() {
        Some(format!("var({})", variable.trim()))
    } else {
        None
    }
}

pub(crate) fn decode_arbitrary_value(value: &str) -> String {
    decode_arbitrary_value_with_options(value, true)
}

pub(crate) fn decode_arbitrary_content_value(value: &str) -> String {
    decode_arbitrary_value_with_options(value, false)
}

fn decode_arbitrary_value_with_options(value: &str, preserve_url_underscores: bool) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.char_indices().peekable();
    let mut function_stack = Vec::new();

    while let Some((index, ch)) = chars.next() {
        if ch == '\\' && chars.peek().is_some_and(|(_, next)| *next == '_') {
            output.push('_');
            chars.next();
            continue;
        }

        match ch {
            '(' => {
                let inside_url = function_stack.iter().any(|state| *state);
                let is_url = preserve_url_underscores
                    && function_name_before(value, index).eq_ignore_ascii_case("url");
                function_stack.push(inside_url || is_url);
                output.push(ch);
            }
            ')' => {
                output.push(ch);
                function_stack.pop();
            }
            '_' if function_stack.iter().any(|state| *state) => output.push('_'),
            '_' => output.push(' '),
            _ => output.push(ch),
        }
    }

    output
}

fn function_name_before(value: &str, open_paren_index: usize) -> &str {
    let before = &value[..open_paren_index];
    let end = before.trim_end().len();
    let start = before[..end]
        .rfind(|ch: char| !(ch.is_ascii_alphabetic() || ch == '-'))
        .map_or(0, |index| index + 1);
    &before[start..end]
}

#[cfg(test)]
mod tests {
    use super::{
        arbitrary_or_custom_var, custom_var, decode_arbitrary_content_value,
        decode_arbitrary_value, ordered_class_tokens, parse_class, typed_custom_var, variant_key,
    };

    #[test]
    fn parses_variants_around_arbitrary_values() {
        let parsed = parse_class("group-hover:[&:focus]:!bg-[color:var(--accent)]").unwrap();

        assert_eq!(parsed.variants, ["group-hover", "[&:focus]"]);
        assert_eq!(parsed.utility, "bg-[color:var(--accent)]");
        assert!(parsed.important);
    }

    #[test]
    fn parses_variants_around_parenthesized_values() {
        let parsed = parse_class("active:shadow-(color:--shadow-color)").unwrap();

        assert_eq!(parsed.variants, ["active"]);
        assert_eq!(parsed.utility, "shadow-(color:--shadow-color)");
        assert!(!parsed.important);

        let parsed = parse_class("focus:shadow-(--elevation)").unwrap();

        assert_eq!(parsed.variants, ["focus"]);
        assert_eq!(parsed.utility, "shadow-(--elevation)");
        assert!(!parsed.important);
    }

    #[test]
    fn orders_important_tokens_after_normal_tokens() {
        assert_eq!(
            ordered_class_tokens("!mt-4 p-2 hover:!bg-black hover:bg-white ![color:red]"),
            [
                "p-2",
                "hover:bg-white",
                "!mt-4",
                "hover:!bg-black",
                "![color:red]"
            ]
        );
    }

    #[test]
    fn decodes_arbitrary_variant_key_segments() {
        assert_eq!(
            variant_key(&["[&_p]".to_string(), "hover".to_string()]),
            "[& p]:hover"
        );
        assert_eq!(
            variant_key(&["group-[.is-open_&]".to_string()]),
            "group-[.is-open &]"
        );
        assert_eq!(
            variant_key(&["[@media(width_>=_48rem)]".to_string()]),
            "[@media(width >= 48rem)]"
        );
        assert_eq!(
            variant_key(&["[&_.nav\\_item]".to_string()]),
            "[& .nav_item]"
        );
    }

    #[test]
    fn decodes_arbitrary_values_with_tailwind_whitespace_rules() {
        assert_eq!(
            decode_arbitrary_value("calc(100%_-_2rem)"),
            "calc(100% - 2rem)"
        );
        assert_eq!(
            decode_arbitrary_value("var(--space\\_lg)"),
            "var(--space_lg)"
        );
        assert_eq!(
            decode_arbitrary_value("url('/what_a_rush.png')"),
            "url('/what_a_rush.png')"
        );
        assert_eq!(
            decode_arbitrary_value("image-set(url('/a_b.png')_1x,_url('/a_b@2x.png')_2x)"),
            "image-set(url('/a_b.png') 1x, url('/a_b@2x.png') 2x)"
        );
    }

    #[test]
    fn decodes_content_values_without_url_special_casing() {
        assert_eq!(
            decode_arbitrary_content_value("'Hello\\_World'"),
            "'Hello_World'"
        );
        assert_eq!(
            decode_arbitrary_content_value("'Hello_World'"),
            "'Hello World'"
        );
    }

    #[test]
    fn parses_custom_property_shorthands() {
        assert_eq!(
            custom_var("(--spacing-lg)").as_deref(),
            Some("var(--spacing-lg)")
        );
        assert_eq!(
            arbitrary_or_custom_var("[calc(100%_-_2rem)]").as_deref(),
            Some("calc(100% - 2rem)")
        );
        assert_eq!(
            typed_custom_var("(color:--brand)", "color").as_deref(),
            Some("var(--brand)")
        );
        assert_eq!(typed_custom_var("(length:--brand)", "color"), None);
    }
}
