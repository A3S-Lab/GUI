pub(crate) fn parse_style_declarations(value: &str) -> Vec<(String, String)> {
    split_css_declarations(value)
        .into_iter()
        .filter_map(|declaration| {
            let separator = find_css_declaration_separator(&declaration)?;
            let property = declaration[..separator].trim();
            let value = declaration[separator + 1..].trim();
            if property.is_empty() || value.is_empty() {
                None
            } else {
                Some((property.to_string(), value.to_string()))
            }
        })
        .collect()
}

fn split_css_declarations(value: &str) -> Vec<String> {
    let chars = value.chars().collect::<Vec<_>>();
    let mut declarations = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut index = 0usize;

    while index < chars.len() {
        let ch = chars[index];
        let next = chars.get(index + 1).copied();

        if let Some(active_quote) = quote {
            current.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        if ch == '/' && next == Some('*') {
            let Some(relative_end) = chars[index + 2..]
                .windows(2)
                .position(|window| window == ['*', '/'])
            else {
                break;
            };
            index += relative_end + 4;
            continue;
        }

        match ch {
            '"' | '\'' => {
                quote = Some(ch);
                current.push(ch);
            }
            '(' => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                current.push(ch);
            }
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                current.push(ch);
            }
            ';' if paren_depth == 0 && bracket_depth == 0 => {
                declarations.push(current);
                current = String::new();
            }
            _ => current.push(ch),
        }
        index += 1;
    }

    if !current.trim().is_empty() {
        declarations.push(current);
    }
    declarations
}

fn find_css_declaration_separator(declaration: &str) -> Option<usize> {
    let mut quote = None;
    let mut escaped = false;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    for (index, ch) in declaration.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active_quote {
                quote = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if paren_depth == 0 && bracket_depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_css_text_without_splitting_inside_functions_or_strings() {
        let declarations = parse_style_declarations(
            r#"
            color: rgb(10 20 30 / 50%);
            background-image: url("https://example.com/a:b;c.svg");
            content: "label: value; still text";
            --accent: color-mix(in srgb, rebeccapurple 40%, white);
            /* ignored comment: with delimiter; */
            padding-inline: 1rem 2rem;
            "#,
        );

        assert_eq!(
            declarations,
            vec![
                ("color".to_string(), "rgb(10 20 30 / 50%)".to_string()),
                (
                    "background-image".to_string(),
                    r#"url("https://example.com/a:b;c.svg")"#.to_string()
                ),
                (
                    "content".to_string(),
                    r#""label: value; still text""#.to_string()
                ),
                (
                    "--accent".to_string(),
                    "color-mix(in srgb, rebeccapurple 40%, white)".to_string()
                ),
                ("padding-inline".to_string(), "1rem 2rem".to_string()),
            ]
        );
    }
}
