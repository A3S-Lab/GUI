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
    let mut start = 0usize;
    let mut variants = Vec::new();
    for (index, ch) in class.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => {
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

#[cfg(test)]
mod tests {
    use super::{ordered_class_tokens, parse_class};

    #[test]
    fn parses_variants_around_arbitrary_values() {
        let parsed = parse_class("group-hover:[&:focus]:!bg-[color:var(--accent)]").unwrap();

        assert_eq!(parsed.variants, ["group-hover", "[&:focus]"]);
        assert_eq!(parsed.utility, "bg-[color:var(--accent)]");
        assert!(parsed.important);
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
}
