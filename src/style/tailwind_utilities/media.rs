use super::*;

pub(in crate::style) fn tailwind_media_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("bg-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((hint, hinted_value)) = value.split_once(':') {
            if let Some(property) = match hint {
                "color" => Some("background-color"),
                "image" | "url" => Some("background-image"),
                "length" | "size" => Some("background-size"),
                "position" => Some("background-position"),
                _ => None,
            } {
                return Some((property.to_string(), tailwind_arbitrary_value(hinted_value)));
            }
        }
        let value = tailwind_arbitrary_value(value);
        let property = if is_css_background_image_value(&value) {
            "background-image"
        } else if parse_color(&value).is_some() {
            "background-color"
        } else if is_background_position_value(&value) {
            "background-position"
        } else {
            "background-size"
        };
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("object-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "object-position".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if class == "list-image-none" {
        return Some(("list-style-image".to_string(), "none".to_string()));
    }
    if let Some(value) = class
        .strip_prefix("list-image-")
        .and_then(tailwind_arbitrary_or_custom_var)
    {
        return Some(("list-style-image".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("list-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "list-style-type".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("columns-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("columns".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("columns-") {
        if let Some(value) = tailwind_columns_value(value) {
            return Some(("columns".to_string(), value));
        }
    }
    None
}

pub(in crate::style) fn is_css_background_image_value(value: &str) -> bool {
    matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some(
            "url"
                | "image"
                | "image-set"
                | "linear-gradient"
                | "radial-gradient"
                | "conic-gradient"
                | "repeating-linear-gradient"
                | "repeating-radial-gradient"
                | "repeating-conic-gradient"
        )
    ) && value.ends_with(')')
}

pub(in crate::style) fn is_background_position_value(value: &str) -> bool {
    let parts = value.split_whitespace().collect::<Vec<_>>();
    !parts.is_empty()
        && parts.iter().all(|part| {
            matches!(*part, "top" | "right" | "bottom" | "left" | "center")
                || parse_length(part).is_some()
        })
}

pub(in crate::style) fn tailwind_columns_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| value.parse::<u16>().ok().map(|value| value.to_string()))
}

pub(in crate::style) fn tailwind_flex_value(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "initial" => Some("0 auto".to_string()),
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| tailwind_fraction(value).map(|value| format!("calc({value} * 100%)")))
            .or_else(|| value.parse::<f64>().ok().map(trim_float)),
    }
}

pub(in crate::style) fn tailwind_basis_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| tailwind_length(value).map(style_length_css))
}

pub(in crate::style) fn tailwind_number_token(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| value.parse::<f64>().ok().map(trim_float))
}

pub(in crate::style) fn tailwind_order_value(class: &str) -> Option<String> {
    let negative = class.starts_with("-order-");
    let value = if negative {
        class.strip_prefix("-order-")?
    } else {
        class.strip_prefix("order-")?
    };
    let value = match value {
        "first" if !negative => "-9999".to_string(),
        "last" if !negative => "9999".to_string(),
        "none" if !negative => "0".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| value.parse::<i32>().ok().map(|value| value.to_string()))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

pub(in crate::style) fn tailwind_fraction(value: &str) -> Option<String> {
    let (numerator, denominator) = value.split_once('/')?;
    let numerator = numerator.parse::<f64>().ok()?;
    let denominator = denominator.parse::<f64>().ok()?;
    if denominator == 0.0 {
        None
    } else {
        Some(trim_float(numerator / denominator))
    }
}

pub(in crate::style) fn tailwind_container_width(value: &str) -> Option<&'static str> {
    match value {
        "3xs" => Some("16rem"),
        "2xs" => Some("18rem"),
        "xs" => Some("20rem"),
        "sm" => Some("24rem"),
        "md" => Some("28rem"),
        "lg" => Some("32rem"),
        "xl" => Some("36rem"),
        "2xl" => Some("42rem"),
        "3xl" => Some("48rem"),
        "4xl" => Some("56rem"),
        "5xl" => Some("64rem"),
        "6xl" => Some("72rem"),
        "7xl" => Some("80rem"),
        _ => None,
    }
}

pub(in crate::style) fn tailwind_font_family(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
}

pub(in crate::style) fn tailwind_letter_spacing(class: &str) -> Option<String> {
    let negative = class.starts_with("-tracking-");
    let value = if negative {
        class.strip_prefix("-tracking-")?
    } else {
        class.strip_prefix("tracking-")?
    };
    let value = match value {
        "tighter" => "-0.05em".to_string(),
        "tight" => "-0.025em".to_string(),
        "normal" => "0em".to_string(),
        "wide" => "0.025em".to_string(),
        "wider" => "0.05em".to_string(),
        "widest" => "0.1em".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| parse_length(value).map(style_length_css))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

pub(in crate::style) fn tailwind_decoration_declaration(class: &str) -> Option<(String, String)> {
    let value = class.strip_prefix("decoration-")?;
    if let Some(value) = tailwind_decoration_thickness(value) {
        return Some(("text-decoration-thickness".to_string(), value));
    }
    tailwind_color_css(value).map(|value| ("text-decoration-color".to_string(), value))
}

pub(in crate::style) fn tailwind_decoration_thickness(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "from-font" => Some("from-font".to_string()),
        _ => tailwind_border_width(value).map(style_length_css),
    }
}

pub(in crate::style) fn tailwind_underline_offset(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_border_width(value).map(style_length_css))
}

pub(in crate::style) fn tailwind_visual_effect_declaration(
    class: &str,
) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("--tw-shadow".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("outline".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "outline-offset".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("cursor-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("cursor".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("aspect-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("aspect-ratio".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("transform".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("filter".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-")
        .and_then(tailwind_length)
    {
        return Some(("outline-offset".to_string(), style_length_css(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-")
        .and_then(tailwind_border_width)
    {
        return Some(("outline-width".to_string(), style_length_css(value)));
    }
    if let Some(value) = class.strip_prefix("outline-").and_then(tailwind_color_css) {
        return Some(("outline-color".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("cursor-") {
        if is_tailwind_cursor(value) {
            return Some(("cursor".to_string(), value.to_string()));
        }
    }
    if let Some(value) = class.strip_prefix("aspect-") {
        if let Some((width, height)) = value.split_once('/') {
            if width.parse::<f64>().is_ok() && height.parse::<f64>().is_ok() {
                return Some(("aspect-ratio".to_string(), format!("{width} / {height}")));
            }
        }
    }
    if let Some(value) = tailwind_transform_declaration(class) {
        return Some(("transform".to_string(), value));
    }
    None
}
