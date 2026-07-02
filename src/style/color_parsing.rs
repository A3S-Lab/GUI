use super::StyleColor;

pub(super) fn parse_color(value: &str) -> Option<StyleColor> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    if let Some(color) = parse_rgb_function(value) {
        return Some(color);
    }
    if let Some(color) = parse_hsl_function(value) {
        return Some(color);
    }
    if is_css_color_function(value) {
        return Some(StyleColor::Function(value.to_string()));
    }
    if value.is_empty() {
        None
    } else {
        Some(StyleColor::Keyword(value.to_string()))
    }
}

pub(super) fn parse_background_shorthand_color(value: &str) -> Option<StyleColor> {
    let color = parse_color(value)?;
    match &color {
        StyleColor::Rgba { .. } => Some(color),
        StyleColor::Function(_) => Some(color),
        StyleColor::Keyword(keyword) if is_background_color_keyword_candidate(keyword) => {
            Some(color)
        }
        StyleColor::Keyword(_) => None,
    }
}

fn is_background_color_keyword_candidate(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty()
        || value.contains('/')
        || value.contains(',')
        || value.contains('(')
        || value.chars().any(char::is_whitespace)
    {
        return false;
    }

    let lower = value.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "auto"
            | "border-box"
            | "bottom"
            | "center"
            | "contain"
            | "content-box"
            | "cover"
            | "fixed"
            | "inherit"
            | "initial"
            | "left"
            | "local"
            | "none"
            | "no-repeat"
            | "padding-box"
            | "repeat"
            | "repeat-x"
            | "repeat-y"
            | "revert"
            | "revert-layer"
            | "right"
            | "round"
            | "scroll"
            | "space"
            | "top"
            | "unset"
    ) {
        return false;
    }

    value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

fn parse_hex_color(hex: &str) -> Option<StyleColor> {
    match hex.len() {
        3 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: 255,
        }),
        4 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: expand_hex_digit(&hex[3..4])?,
        }),
        6 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: 255,
        }),
        8 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: u8::from_str_radix(&hex[6..8], 16).ok()?,
        }),
        _ => None,
    }
}

fn expand_hex_digit(hex: &str) -> Option<u8> {
    let value = u8::from_str_radix(hex, 16).ok()?;
    Some((value << 4) | value)
}

fn parse_rgb_function(value: &str) -> Option<StyleColor> {
    let content = css_function_content(value, &["rgb", "rgba"])?;
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let red = parse_rgb_channel(&channels[0])?;
    let green = parse_rgb_channel(&channels[1])?;
    let blue = parse_rgb_channel(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_rgb_channel(value: &str) -> Option<u8> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    value.trim().parse::<u8>().ok()
}

fn parse_hsl_function(value: &str) -> Option<StyleColor> {
    let content = css_function_content(value, &["hsl", "hsla"])?;
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let hue = parse_hue_degrees(&channels[0])?;
    let saturation = parse_percent_fraction(&channels[1])?;
    let lightness = parse_percent_fraction(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn is_css_color_function(value: &str) -> bool {
    css_function_name(value).is_some_and(|name| {
        matches!(
            name.as_str(),
            "rgb"
                | "rgba"
                | "hsl"
                | "hsla"
                | "hwb"
                | "lab"
                | "lch"
                | "oklab"
                | "oklch"
                | "color"
                | "color-mix"
                | "light-dark"
                | "contrast-color"
                | "alpha"
                | "device-cmyk"
        )
    })
}

fn css_function_content<'a>(value: &'a str, names: &[&str]) -> Option<&'a str> {
    let value = value.trim();
    let open = value.find('(')?;
    if !value.ends_with(')') {
        return None;
    }
    let name = value[..open].trim();
    if !names
        .iter()
        .any(|expected| name.eq_ignore_ascii_case(expected))
    {
        return None;
    }
    Some(&value[open + 1..value.len() - 1])
}

fn css_function_name(value: &str) -> Option<String> {
    let value = value.trim();
    let open = value.find('(')?;
    if open == 0 || !value.ends_with(')') {
        return None;
    }
    let name = value[..open].trim();
    if !name.chars().all(|ch| ch.is_ascii_alphabetic() || ch == '-') {
        return None;
    }
    Some(name.to_ascii_lowercase())
}

fn parse_color_function_parts(content: &str) -> (Vec<String>, Option<String>) {
    let content = content.replace(',', " ");
    let mut channels = Vec::new();
    let mut alpha = None;
    let mut alpha_next = false;
    for part in content.split_whitespace() {
        if part == "/" {
            alpha_next = true;
        } else if let Some((before, after)) = part.split_once('/') {
            if !before.is_empty() {
                channels.push(before.to_string());
            }
            if !after.is_empty() {
                alpha = Some(after.to_string());
            }
            alpha_next = false;
        } else if alpha_next {
            alpha = Some(part.to_string());
            alpha_next = false;
        } else {
            channels.push(part.to_string());
        }
    }
    if alpha.is_none() && channels.len() > 3 {
        alpha = channels.pop();
    }
    (channels, alpha)
}

fn parse_hue_degrees(value: &str) -> Option<f64> {
    let value = value.trim();
    let degrees = if let Some(degrees) = value.strip_suffix("deg") {
        degrees.trim().parse::<f64>().ok()?
    } else if let Some(turns) = value.strip_suffix("turn") {
        turns.trim().parse::<f64>().ok()? * 360.0
    } else if let Some(radians) = value.strip_suffix("rad") {
        radians.trim().parse::<f64>().ok()?.to_degrees()
    } else if let Some(gradians) = value.strip_suffix("grad") {
        gradians.trim().parse::<f64>().ok()? * 0.9
    } else {
        value.parse::<f64>().ok()?
    };
    Some(degrees.rem_euclid(360.0))
}

fn parse_percent_fraction(value: &str) -> Option<f64> {
    let value = value.trim().strip_suffix('%')?.trim();
    Some((value.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
}

fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_prime = hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
    let (red1, green1, blue1) = if (0.0..1.0).contains(&hue_prime) {
        (chroma, x, 0.0)
    } else if (1.0..2.0).contains(&hue_prime) {
        (x, chroma, 0.0)
    } else if (2.0..3.0).contains(&hue_prime) {
        (0.0, chroma, x)
    } else if (3.0..4.0).contains(&hue_prime) {
        (0.0, x, chroma)
    } else if (4.0..5.0).contains(&hue_prime) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };
    let m = lightness - chroma / 2.0;
    (
        ((red1 + m) * 255.0).round() as u8,
        ((green1 + m) * 255.0).round() as u8,
        ((blue1 + m) * 255.0).round() as u8,
    )
}

fn parse_alpha_channel(value: &str) -> Option<u8> {
    let value = value.trim().trim_start_matches('/');
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    let alpha = value.parse::<f64>().ok()?;
    Some((alpha.clamp(0.0, 1.0) * 255.0).round() as u8)
}
